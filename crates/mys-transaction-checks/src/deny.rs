// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto_zkp::bn254::zk_login::OIDCProvider;
use mys_config::transaction_deny_config::TransactionDenyConfig;
use mys_types::{
    base_types::{MysAddress, ObjectRef},
    error::{MysError, MysResult, UserInputError},
    object::Owner,
    signature::GenericSignature,
    storage::{BackingPackageStore, ObjectStore},
    transaction::{Command, InputObjectKind, TransactionData, TransactionDataAPI},
};
macro_rules! deny_if_true {
    ($cond:expr, $msg:expr) => {
        if ($cond) {
            return Err(MysError::UserInputError {
                error: UserInputError::TransactionDenied {
                    error: $msg.to_string(),
                },
            });
        }
    };
}

/// Check that the provided transaction is allowed to be signed according to the
/// deny config.
pub fn check_transaction_for_signing(
    tx_data: &TransactionData,
    tx_signatures: &[GenericSignature],
    input_object_kinds: &[InputObjectKind],
    receiving_objects: &[ObjectRef],
    filter_config: &TransactionDenyConfig,
    package_store: &dyn BackingPackageStore,
    object_store: &dyn ObjectStore,
) -> MysResult {
    check_disabled_features(filter_config, tx_data, tx_signatures, input_object_kinds, package_store, object_store)?;

    check_signers(filter_config, tx_data)?;

    check_input_objects(filter_config, input_object_kinds)?;

    check_package_dependencies(filter_config, tx_data, package_store)?;

    check_receiving_objects(filter_config, receiving_objects)?;

    Ok(())
}

fn check_receiving_objects(
    filter_config: &TransactionDenyConfig,
    receiving_objects: &[ObjectRef],
) -> MysResult {
    deny_if_true!(
        filter_config.receiving_objects_disabled() && !receiving_objects.is_empty(),
        "Receiving objects is temporarily disabled".to_string()
    );
    for (id, _, _) in receiving_objects {
        deny_if_true!(
            filter_config.get_object_deny_set().contains(id),
            format!("Access to object {:?} is temporarily disabled", id)
        );
    }
    Ok(())
}

fn check_disabled_features(
    filter_config: &TransactionDenyConfig,
    tx_data: &TransactionData,
    tx_signatures: &[GenericSignature],
    input_object_kinds: &[InputObjectKind],
    _package_store: &dyn BackingPackageStore,
    object_store: &dyn ObjectStore,
) -> MysResult {
    deny_if_true!(
        filter_config.user_transaction_disabled(),
        "Transaction signing is temporarily disabled"
    );

    tx_signatures.iter().try_for_each(|s| {
        if let GenericSignature::ZkLoginAuthenticator(z) = s {
            deny_if_true!(
                filter_config.zklogin_sig_disabled(),
                "zkLogin authenticator is temporarily disabled"
            );
            deny_if_true!(
                filter_config.zklogin_disabled_providers().contains(
                    &OIDCProvider::from_iss(z.get_iss())
                        .map_err(|_| MysError::UnexpectedMessage(z.get_iss().to_string()))?
                        .to_string()
                ),
                "zkLogin OAuth provider is temporarily disabled"
            )
        }
        Ok(())
    })?;

    // Check if publish/upgrade restrictions apply
    let publish_disabled = filter_config.package_publish_disabled();
    let upgrade_disabled = filter_config.package_upgrade_disabled();

    if !publish_disabled && !upgrade_disabled {
        return Ok(());
    }

    // Check if sender owns an UpgradeAdminCap or PackagePublishingAdminCap (admin bypass)
    let sender = tx_data.sender();
    let sender_has_upgrade_admin_cap = has_upgrade_admin_cap(sender, input_object_kinds, object_store);
    let sender_has_publish_admin_cap = has_package_publishing_admin_cap(sender, input_object_kinds, object_store);

    for command in tx_data.kind().iter_commands() {
        // Allow publish if sender owns UpgradeAdminCap or PackagePublishingAdminCap, otherwise check the disable flag
        deny_if_true!(
            publish_disabled && !sender_has_upgrade_admin_cap && !sender_has_publish_admin_cap && matches!(command, Command::Publish(..)),
            "Package publish is temporarily disabled"
        );
        deny_if_true!(
            upgrade_disabled && !sender_has_upgrade_admin_cap && matches!(command, Command::Upgrade(..)),
            "Package upgrade is temporarily disabled"
        );
    }
    Ok(())
}

/// Check if the sender owns an UpgradeAdminCap from the social contracts package
fn has_upgrade_admin_cap(
    sender: MysAddress,
    input_object_kinds: &[InputObjectKind],
    object_store: &dyn ObjectStore,
) -> bool {
    // For each input object, check if it's an owned object that could be an UpgradeAdminCap
    for input_kind in input_object_kinds {
        if let InputObjectKind::ImmOrOwnedMoveObject((object_id, _, _)) = input_kind {
            // Try to get the object to check its type and ownership
            if let Some(object) = object_store.get_object(object_id) {
                // Check if this object is owned by the sender (must be Address owner)
                if let Owner::AddressOwner(owner_addr) = object.owner {
                    if owner_addr == sender {
                        // Check the type - we need to find the social contracts package first
                        // The UpgradeAdminCap has type: social_contracts::upgrade::UpgradeAdminCap
                        if let Some(move_object) = object.data.try_as_move() {
                            let type_tag = move_object.type_();
                            // Check if the type matches the expected UpgradeAdminCap structure
                            // For now, we'll check if it contains "upgrade" and "UpgradeAdminCap" in the type name
                            let type_name = format!("{}", type_tag);
                            if type_name.contains("upgrade") && type_name.contains("UpgradeAdminCap") {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Check if the sender owns a PackagePublishingAdminCap from the mys framework package
fn has_package_publishing_admin_cap(
    sender: MysAddress,
    input_object_kinds: &[InputObjectKind],
    object_store: &dyn ObjectStore,
) -> bool {
    // For each input object, check if it's an owned object that could be a PackagePublishingAdminCap
    for input_kind in input_object_kinds {
        if let InputObjectKind::ImmOrOwnedMoveObject((object_id, _, _)) = input_kind {
            // Try to get the object to check its type and ownership
            if let Some(object) = object_store.get_object(object_id) {
                // Check if this object is owned by the sender (must be Address owner)
                if let Owner::AddressOwner(owner_addr) = object.owner {
                    if owner_addr == sender {
                        // Check the type - PackagePublishingAdminCap has type: mys::package::PackagePublishingAdminCap
                        if let Some(move_object) = object.data.try_as_move() {
                            let type_tag = move_object.type_();
                            // Check if the type matches the expected PackagePublishingAdminCap structure
                            let type_name = format!("{}", type_tag);
                            if type_name.contains("package") && type_name.contains("PackagePublishingAdminCap") {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn check_signers(filter_config: &TransactionDenyConfig, tx_data: &TransactionData) -> MysResult {
    let deny_map = filter_config.get_address_deny_set();
    if deny_map.is_empty() {
        return Ok(());
    }
    for signer in tx_data.signers() {
        deny_if_true!(
            deny_map.contains(&signer),
            format!(
                "Access to account address {:?} is temporarily disabled",
                signer
            )
        );
    }
    Ok(())
}

fn check_input_objects(
    filter_config: &TransactionDenyConfig,
    input_object_kinds: &[InputObjectKind],
) -> MysResult {
    let deny_map = filter_config.get_object_deny_set();
    let shared_object_disabled = filter_config.shared_object_disabled();
    if deny_map.is_empty() && !shared_object_disabled {
        // No need to iterate through the input objects if no relevant policy is set.
        return Ok(());
    }
    for input_object_kind in input_object_kinds {
        let id = input_object_kind.object_id();
        deny_if_true!(
            deny_map.contains(&id),
            format!("Access to input object {:?} is temporarily disabled", id)
        );
        deny_if_true!(
            shared_object_disabled && input_object_kind.is_shared_object(),
            "Usage of shared object in transactions is temporarily disabled"
        );
    }
    Ok(())
}

fn check_package_dependencies(
    filter_config: &TransactionDenyConfig,
    tx_data: &TransactionData,
    package_store: &dyn BackingPackageStore,
) -> MysResult {
    let deny_map = filter_config.get_package_deny_set();
    if deny_map.is_empty() {
        return Ok(());
    }
    let mut dependencies = vec![];
    for command in tx_data.kind().iter_commands() {
        match command {
            Command::Publish(_, deps) => {
                // It is possible that the deps list is inaccurate since it's provided
                // by the user. But that's OK because this publish transaction will fail
                // to execute in the end. Similar reasoning for Upgrade.
                dependencies.extend(deps.iter().copied());
            }
            Command::Upgrade(_, deps, package_id, _) => {
                dependencies.extend(deps.iter().copied());
                // It's crucial that we don't allow upgrading a package in the deny list,
                // otherwise one can bypass the deny list by upgrading a package.
                dependencies.push(*package_id);
            }
            Command::MoveCall(call) => {
                let package = package_store.get_package_object(&call.package)?.ok_or(
                    MysError::UserInputError {
                        error: UserInputError::ObjectNotFound {
                            object_id: call.package,
                            version: None,
                        },
                    },
                )?;
                // linkage_table maps from the original package ID to the upgraded ID for each
                // dependency. Here we only check the upgraded (i.e. the latest) ID against the
                // deny list. This means that we only make sure that the denied package is not
                // currently used as a dependency. This allows us to deny an older version of
                // package but permits the use of a newer version.
                dependencies.extend(
                    package
                        .move_package()
                        .linkage_table()
                        .values()
                        .map(|upgrade_info| upgrade_info.upgraded_id),
                );
                dependencies.push(package.move_package().id());
            }
            Command::TransferObjects(..)
            | &Command::SplitCoins(..)
            | &Command::MergeCoins(..)
            | &Command::MakeMoveVec(..) => {}
        }
    }
    for dep in dependencies {
        deny_if_true!(
            deny_map.contains(&dep),
            format!("Access to package {:?} is temporarily disabled", dep)
        );
    }
    Ok(())
}
