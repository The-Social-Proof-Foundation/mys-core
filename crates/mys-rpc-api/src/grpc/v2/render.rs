// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use mys_rpc::{
    field::FieldMaskTree,
    merge::Merge,
    proto::mys::rpc::v2::{Bcs, Event, Object, TransactionEffects, TransactionEvents},
};

use crate::RpcService;
use async_trait::async_trait;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use mys_package_resolver;
use mys_package_resolver::CleverError;

impl RpcService {
    pub fn render_object_to_proto(
        &self,
        object: &mys_types::object::Object,
        read_mask: &FieldMaskTree,
    ) -> Object {
        let mut message = Object::default();

        if read_mask.contains(Object::JSON_FIELD) {
            message.json = self.render_object_to_json(object).map(Box::new);
        }

        message.merge(object, read_mask);

        message
    }

    fn render_object_to_json(
        &self,
        object: &mys_types::object::Object,
    ) -> Option<prost_types::Value> {
        let move_object = object.data.try_as_move()?;
        self.render_json(&move_object.type_().clone().into(), move_object.contents())
    }

    pub fn render_json(
        &self,
        struct_tag: &move_core_types::language_storage::StructTag,
        contents: &[u8],
    ) -> Option<prost_types::Value> {
        let layout = self
            .reader
            .inner()
            .get_struct_layout(struct_tag)
            .ok()
            .flatten()?;

        mys_types::proto_value::ProtoVisitor::new(self.config.max_json_move_value_size())
            .deserialize_value(contents, &layout)
            .map_err(|e| tracing::debug!("unable to convert move value to JSON: {e}"))
            .ok()
    }

    pub fn render_events_to_proto(
        &self,
        events: &mys_types::effects::TransactionEvents,
        mask: &FieldMaskTree,
    ) -> TransactionEvents {
        let mut message = TransactionEvents::default();

        if mask.contains(TransactionEvents::BCS_FIELD) {
            let mut bcs = Bcs::serialize(&events).unwrap();
            bcs.name = Some("TransactionEvents".to_owned());
            message.bcs = Some(bcs);
        }

        if mask.contains(TransactionEvents::DIGEST_FIELD) {
            message.digest = Some(format!("{:?}", events.digest()));
        }

        if let Some(event_mask) = mask.subtree(TransactionEvents::EVENTS_FIELD) {
            message.events = events
                .data
                .iter()
                .map(|event| self.render_event_to_proto(event, &event_mask))
                .collect();
        }

        message
    }

    pub fn render_event_to_proto(
        &self,
        event: &mys_types::event::Event,
        mask: &FieldMaskTree,
    ) -> Event {
        let mut message = Event::default();

        if mask.contains(Event::PACKAGE_ID_FIELD) {
            message.set_package_id(event.package_id.to_canonical_string(true));
        }

        if mask.contains(Event::MODULE_FIELD) {
            message.set_module(event.transaction_module.to_string());
        }

        if mask.contains(Event::SENDER_FIELD) {
            message.sender = Some(event.sender.to_string());
        }

        if mask.contains(Event::EVENT_TYPE_FIELD) {
            message.event_type = Some(event.type_.to_canonical_string(true));
        }

        if mask.contains(Event::CONTENTS_FIELD) {
            let mut bcs = Bcs::from(event.contents.clone());
            bcs.name = Some(event.type_.to_canonical_string(true));
            message.contents = Some(bcs);
        }

        if mask.contains(Event::JSON_FIELD) {
            message.json = self
                .render_json(&event.type_, &event.contents)
                .map(Box::new);
        }

        message
    }

    // Renders clever error information in-place
    pub fn render_clever_error(&self, effects: &mut TransactionEffects) {
        use mys_rpc::proto::mys::rpc::v2::CleverError;
        use mys_rpc::proto::mys::rpc::v2::MoveAbort;
        use mys_rpc::proto::mys::rpc::v2::clever_error;
        use mys_rpc::proto::mys::rpc::v2::execution_error::ErrorDetails;

        let Some(move_abort) = effects
            .status
            .as_mut()
            .and_then(|status| status.error.as_mut())
            .and_then(|error| match &mut error.error_details {
                Some(ErrorDetails::Abort(move_abort)) => Some(move_abort),
                _ => None,
            })
        else {
            return;
        };

        fn render(service: &RpcService, move_abort: &MoveAbort) -> Option<CleverError> {
            use move_core_types::language_storage::ModuleId;
            use tokio::runtime::Handle;

            let location = move_abort.location.as_ref()?;
            let abort_code = move_abort.abort_code();
            let package_id = location.package().parse::<mys_sdk_types::Address>().ok()?;
            let module_name = location.module();

            let object = service.reader.inner().get_object(&package_id.into())?;
            let package = mys_package_resolver::Package::read_from_object(&object).ok()?;
            
            // Create a simple package store wrapper that can fetch this package
            struct SinglePackageStore {
                package: mys_package_resolver::Package,
            }
            
            #[async_trait]
            impl mys_package_resolver::PackageStore for SinglePackageStore {
                async fn fetch(&self, id: AccountAddress) -> mys_package_resolver::Result<std::sync::Arc<mys_package_resolver::Package>> {
                    if id == self.package.storage_id() {
                        Ok(std::sync::Arc::new(self.package.clone()))
                    } else {
                        Err(mys_package_resolver::Error::PackageNotFound(id))
                    }
                }
            }
            
            let package_store = SinglePackageStore { package };
            let resolver = mys_package_resolver::Resolver::new(package_store);
            
            // Construct ModuleId using storage_id (package_id) since resolve_clever_error fetches using module_id.address()
            let storage_id = AccountAddress::from(package_id.into_inner());
            let module_id = ModuleId::new(storage_id, Identifier::new(module_name).ok()?);
            
            // Use blocking call since we're in a sync context
            let clever_error = Handle::current().block_on(resolver.resolve_clever_error(module_id, abort_code))?;

            let mut clever_error_message = CleverError::default();

            match clever_error.error_info {
                mys_package_resolver::ErrorConstants::None => {}
                mys_package_resolver::ErrorConstants::Rendered {
                    identifier,
                    constant,
                } => {
                    clever_error_message.constant_name = Some(identifier);
                    clever_error_message.value = Some(clever_error::Value::Rendered(constant));
                }
                mys_package_resolver::ErrorConstants::Raw { identifier, bytes } => {
                    clever_error_message.constant_name = Some(identifier);
                    clever_error_message.value = Some(clever_error::Value::Raw(bytes.into()));
                }
            }

            clever_error_message.error_code = clever_error.error_code.map(Into::into);
            clever_error_message.line_number = Some(clever_error.source_line_number.into());

            Some(clever_error_message)
        }

        move_abort.clever_error = render(self, move_abort);
    }

    pub fn render_effects_to_proto<F>(
        &self,
        effects: &mys_types::effects::TransactionEffects,
        unchanged_loaded_runtime_objects: &[mys_types::storage::ObjectKey],
        object_type_lookup: F,
        mask: &FieldMaskTree,
    ) -> TransactionEffects
    where
        F: Fn(&mys_types::base_types::ObjectID) -> Option<mys_types::base_types::ObjectType>,
    {
        // TODO consider inlining this function here to avoid needing to do the extra parsing below
        let mut effects = TransactionEffects::merge_from(effects, mask);

        if mask.contains(TransactionEffects::UNCHANGED_LOADED_RUNTIME_OBJECTS_FIELD) {
            effects.unchanged_loaded_runtime_objects = unchanged_loaded_runtime_objects
                .iter()
                .map(Into::into)
                .collect();
        }

        if mask.contains(TransactionEffects::CHANGED_OBJECTS_FIELD) {
            for changed_object in effects.changed_objects.iter_mut() {
                let Ok(object_id) = changed_object
                    .object_id()
                    .parse::<mys_types::base_types::ObjectID>()
                else {
                    continue;
                };

                if let Some(object_type) = object_type_lookup(&object_id) {
                    changed_object.set_object_type(object_type_to_string(object_type));
                }
            }
        }

        if mask.contains(TransactionEffects::UNCHANGED_CONSENSUS_OBJECTS_FIELD) {
            for unchanged_consensus_object in effects.unchanged_consensus_objects.iter_mut() {
                let Ok(object_id) = unchanged_consensus_object
                    .object_id()
                    .parse::<mys_types::base_types::ObjectID>()
                else {
                    continue;
                };

                if let Some(object_type) = object_type_lookup(&object_id) {
                    unchanged_consensus_object.set_object_type(object_type_to_string(object_type));
                }
            }
        }

        // Try to render clever error info
        self.render_clever_error(&mut effects);

        effects
    }
}

fn object_type_to_string(object_type: mys_types::base_types::ObjectType) -> String {
    match object_type {
        mys_types::base_types::ObjectType::Package => "package".to_owned(),
        mys_types::base_types::ObjectType::Struct(move_object_type) => {
            move_object_type.to_canonical_string(true)
        }
    }
}
