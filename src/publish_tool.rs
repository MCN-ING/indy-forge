use crate::app::{MyRoles, NymInfo};
use crate::helpers::ledgers::{IndyLedger, Ledgers};
use crate::helpers::wallet::IndyWallet;
use derive_more::Display;
use egui::{Button, ComboBox, Label, TextEdit, Ui};
use futures_executor::block_on;
use indy_data_types::did::DidValue;
use indy_data_types::Validatable;
use indy_vdr::common::error::VdrResult;
use indy_vdr::ledger::constants::{LedgerRole, UpdateRole};

#[derive(PartialEq, Eq, Debug, Display)]
enum PublishEntities {
    CredDef,
    Nym,
    Attrib,
    Schema,
    Custom,
}

pub fn publish_tool_ui(
    ui: &mut Ui,
    wallet: &mut Option<IndyWallet>,
    publish_option: &mut String,
    nym_role: &mut MyRoles,
    nym_info: &mut NymInfo,
    picked_path: &mut Option<String>,
    ledgers: &mut Option<IndyLedger>,
    txn_result: &mut String,
) -> anyhow::Result<()> {
    ui.label("Publish something on a ledger");

    let options = vec![
        PublishEntities::CredDef,
        PublishEntities::Nym,
        PublishEntities::Attrib,
        PublishEntities::Schema,
        PublishEntities::Custom,
    ];
    ComboBox::from_id_source("publish_option")
        .selected_text(publish_option.as_str())
        .show_ui(ui, |ui| {
            for option in &options {
                ui.selectable_value(publish_option, option.to_string(), option.to_string());
            }
        });

    if *publish_option == PublishEntities::Nym.to_string() {
        ui.vertical(|ui| {
            ui.heading("NYM Registration");
            ui.label("Enter the NYM DID and Verkey that you want to register");
            ui.label("NYM DID: ");
            ui.add(
                egui::TextEdit::singleline(&mut nym_info.did)
                    .char_limit(32)
                    .hint_text("NYM DID"),
            );
            ui.label("NYM Verkey: ");
            ui.add(egui::TextEdit::singleline(&mut nym_info.verkey).hint_text("NYM Verkey"));
        });
        let is_valid_did = &DidValue((*nym_info.did.to_string()).parse()?).validate();
        match is_valid_did {
            Ok(_) => ui.label("The entered NYM DID seems valid."),
            Err(e) => ui.label(format!("Invalid NYM DID: {} ", e)),
        };
        let is_valid_verkey = &DidValue((*nym_info.verkey.to_string()).parse()?).validate();
        match is_valid_verkey {
            Ok(_) => ui.label("The entered NYM Verkey seems valid."),
            Err(e) => ui.label(format!("Invalid NYM Verkey: {} ", e)),
        };

        ui.label("Select the role for the NYM");
        egui::ComboBox::from_id_source("my_role_nym")
            .selected_text(format!("{:?}", nym_role))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut *nym_role, MyRoles::Endorser, "Endorser");
                ui.selectable_value(&mut *nym_role, MyRoles::NetworkMonitor, "Network Monitor");
                ui.selectable_value(&mut *nym_role, MyRoles::Steward, "Steward");
            });

        // Check each field and add the name of the missing fields to a vector
        let mut missing_fields = Vec::new();
        if nym_info.did.is_empty() || !is_valid_did.is_ok() {
            missing_fields.push("NYM DID");
        }
        if nym_info.verkey.is_empty() || !is_valid_verkey.is_ok() {
            missing_fields.push("NYM Verkey");
        }
        if picked_path.is_none() {
            missing_fields.push("genesis file");
        }

        // Create a string from the vector of missing fields
        let missing_fields_str = missing_fields.join(", ");

        // Display the missing fields in the label
        if !missing_fields.is_empty() {
            ui.colored_label(
                egui::Color32::LIGHT_RED,
                format!(
                    "Please fill the following fields and make sure they are valid: {}",
                    missing_fields_str
                ),
            );
        } else {
            // If both DID and Verkey seems valid, show the "Register Nym" button
            if ui.button("Register Nym").clicked()
                && is_valid_did.is_ok()
                && is_valid_verkey.is_ok()
                && picked_path.is_some()
            {
                ui.label("Registering NYM...");

                let wallet_ref = wallet.as_ref().unwrap();
                let role = match nym_role {
                    MyRoles::Endorser => UpdateRole::Set(LedgerRole::Endorser),
                    MyRoles::NetworkMonitor => UpdateRole::Set(LedgerRole::NetworkMonitor),
                    MyRoles::Steward => UpdateRole::Set(LedgerRole::Steward),
                };
                if let Some(ledger) = ledgers {
                    match block_on(IndyLedger::publish_nym(
                        ledger,
                        wallet_ref,
                        &wallet_ref.did,
                        &nym_info.did,
                        &nym_info.verkey,
                        role,
                    )) {
                        Ok(result) => {
                            *txn_result = result;
                            // Clear the fields after the transaction is successful
                            nym_info.did.clear();
                            nym_info.verkey.clear();
                        }
                        Err(e) => {
                            *txn_result = e.to_string();
                        }
                    }
                }
            }
        }

        ui.separator();
        ui.label("Result:");
        ui.monospace(format!("{:?}", txn_result));
    }

    Ok(())
}
