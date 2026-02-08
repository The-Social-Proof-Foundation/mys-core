// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use move_binary_format::file_format::Ability;
use move_binary_format::file_format::AbilitySet;
use move_binary_format::file_format::Visibility;
use mys_json_rpc_types::MysMoveAbility;
use mys_json_rpc_types::MysMoveAbilitySet;
use mys_json_rpc_types::MysMoveNormalizedFunction;
use mys_json_rpc_types::MysMoveNormalizedType;
use mys_json_rpc_types::MysMoveVisibility;
use mys_package_resolver::FunctionDef;
use mys_package_resolver::OpenSignature;
use mys_package_resolver::OpenSignatureBody;
use mys_package_resolver::Reference;
use mys_types::Identifier;
use mys_types::base_types::ObjectID;

use crate::api::move_utils::error::Error;
use crate::context::Context;
use crate::error::RpcError;
use crate::error::invalid_params;

/// Load information about a function, and convert it into a JSON-RPC response.
pub(super) async fn function(
    ctx: &Context,
    package: ObjectID,
    module: &str,
    name: &str,
) -> Result<MysMoveNormalizedFunction, RpcError<Error>> {
    use Error as E;

    if !Identifier::is_valid(module) {
        return Err(invalid_params(E::BadIdentifier(module.to_owned())));
    }

    if !Identifier::is_valid(name) {
        return Err(invalid_params(E::BadIdentifier(name.to_owned())));
    }

    let sig = ctx
        .package_resolver()
        .function_signature(*package, module, name)
        .await
        .map_err(|e| {
            use mys_package_resolver::error::Error as PRE;
            match &e {
                // These errors can be triggered by passing a type that doesn't exist for the
                // dynamic field name.
                PRE::NotAPackage(_)
                | PRE::PackageNotFound(_)
                | PRE::ModuleNotFound(_, _)
                | PRE::FunctionNotFound(_, _, _) => invalid_params(E::NotFound(e)),

                // These errors can be triggered by requesting a type whose layout is too large
                // (requires too may resources to resolve)
                PRE::TooManyTypeNodes(_, _)
                | PRE::TooManyTypeParams(_, _)
                | PRE::TypeParamNesting(_, _) => invalid_params(E::ResolutionLimit(e)),

                // The other errors are a form of internal error.
                PRE::Bcs(_)
                | PRE::Store { .. }
                | PRE::DatatypeNotFound(_, _, _)
                | PRE::Deserialize(_)
                | PRE::EmptyPackage(_)
                | PRE::InputTypeConflict(_, _, _)
                | PRE::LinkageNotFound(_)
                | PRE::NoTypeOrigin(_, _, _)
                | PRE::NotAnIdentifier(_)
                | PRE::TypeArityMismatch(_, _)
                | PRE::TypeParamOOB(_, _)
                | PRE::UnexpectedReference
                | PRE::UnexpectedSigner
                | PRE::UnexpectedError(_)
                | PRE::ValueNesting(_) => {
                    RpcError::from(anyhow!(e).context("Failed to resolve type layout"))
                }
            }
        })?;

    Ok(normalized_function(&sig))
}

fn normalized_function(sig: &FunctionDef) -> MysMoveNormalizedFunction {
    MysMoveNormalizedFunction {
        visibility: visibility(sig.visibility),
        is_entry: sig.is_entry,
        type_parameters: sig.type_params.iter().map(|a| ability_set(*a)).collect(),
        parameters: sig.parameters.iter().map(normalized_signature).collect(),
        return_: sig.return_.iter().map(normalized_signature).collect(),
    }
}

fn normalized_signature(sig: &OpenSignature) -> MysMoveNormalizedType {
    use MysMoveNormalizedType as T;

    let body = normalized_type(&sig.body);
    match sig.ref_ {
        Some(Reference::Immutable) => T::Reference(Box::new(body)),
        Some(Reference::Mutable) => T::MutableReference(Box::new(body)),
        None => body,
    }
}

fn normalized_type(sig: &OpenSignatureBody) -> MysMoveNormalizedType {
    use OpenSignatureBody as S;
    use MysMoveNormalizedType as T;
    match sig {
        S::Address => T::Address,
        S::Bool => T::Bool,
        S::U8 => T::U8,
        S::U16 => T::U16,
        S::U32 => T::U32,
        S::U64 => T::U64,
        S::U128 => T::U128,
        S::U256 => T::U256,
        S::Vector(sig) => T::Vector(Box::new(normalized_type(sig))),
        S::Datatype(t, params) => T::Struct {
            address: t.package.to_canonical_string(/* with_prefix */ true),
            module: t.module.to_string(),
            name: t.name.to_string(),
            type_arguments: params.iter().map(normalized_type).collect(),
        },
        S::TypeParameter(ix) => T::TypeParameter(*ix),
    }
}

fn visibility(v: Visibility) -> MysMoveVisibility {
    match v {
        Visibility::Public => MysMoveVisibility::Public,
        Visibility::Friend => MysMoveVisibility::Friend,
        Visibility::Private => MysMoveVisibility::Private,
    }
}

fn ability_set(a: AbilitySet) -> MysMoveAbilitySet {
    MysMoveAbilitySet {
        abilities: a.into_iter().map(ability).collect(),
    }
}

fn ability(a: Ability) -> MysMoveAbility {
    match a {
        Ability::Copy => MysMoveAbility::Copy,
        Ability::Drop => MysMoveAbility::Drop,
        Ability::Store => MysMoveAbility::Store,
        Ability::Key => MysMoveAbility::Key,
    }
}
