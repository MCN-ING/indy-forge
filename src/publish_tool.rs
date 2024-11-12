use crate::app::{MyRoles, NymInfo, SchemaInfo};
use crate::helpers::genesis::GenesisSource;
use crate::helpers::ledgers::IndyLedger;
use crate::helpers::wallet::IndyWallet;
use derive_more::Display;
use egui::{ComboBox, Ui};
use futures_executor::block_on;
use indy_data_types::anoncreds::schema::{
    AttributeNames, Schema as IndySchema, SchemaV1 as IndySchemaV1,
};
use indy_data_types::did::DidValue;
use indy_data_types::{SchemaId, Validatable};
use indy_vdr::ledger::constants::{LedgerRole, UpdateRole};

#[derive(PartialEq, Eq, Debug, Display)]
enum PublishEntities {
    CredDef,
    Nym,
    Attrib,
    Schema,
    Custom,
}

// FIXME: This function has too many arguments. Consider grouping them into a struct.
#[allow(clippy::too_many_arguments)]
pub fn publish_tool_ui(
    ui: &mut Ui,
    wallet: &mut Option<IndyWallet>,
    publish_option: &mut String,
    nym_role: &mut MyRoles,
    nym_info: &mut NymInfo,
    genesis_source: &mut Option<GenesisSource>,
    ledgers: &mut Option<IndyLedger>,
    txn_result: &mut String,
    schema_info: &mut SchemaInfo,
    txn: &mut String,
    signed_txn_result: &mut Option<String>,
) -> anyhow::Result<()> {
    ui.label("Publish something on a ledger");

    let options = vec![
        //PublishEntities::Attrib,
        //PublishEntities::CredDef,
        PublishEntities::Nym,
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

    if *publish_option == PublishEntities::CredDef.to_string() {
        ui.heading("Cred Def registration");
    } else if *publish_option == PublishEntities::Attrib.to_string() {
        ui.heading("Attrib registration");
    } else if *publish_option == PublishEntities::Schema.to_string() {
        //region SCHEMA REGISTRATION
        ui.heading("Schema registration");
        //build a form that ask for the schema name, version, and attributes.  You can add extra attributes by clicking a+ sign
        //and remove them by clicking a - sign.  The form should have a submit button that will send the schema to the ledger.

        // Initialize a vector to hold the attributes
        ui.add(
            egui::TextEdit::singleline(&mut schema_info.schema_name).hint_text("Enter schema name"),
        );
        ui.add(
            egui::TextEdit::singleline(&mut schema_info.schema_version)
                .hint_text("Enter schema version"),
        );
        let version_parts: Vec<&str> = schema_info.schema_version.split('.').collect();
        if version_parts.len() != 3
            || version_parts
                .iter()
                .any(|part| part.parse::<u32>().is_err())
        {
            // Handle the error, e.g., by displaying an error message
            ui.label("Error: The version must have three parts, separated by dots, and each part must be a number.");
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut schema_info.new_attribute)
                    .hint_text("Enter new attribute"),
            );
            if ui.button("+").clicked() && !schema_info.new_attribute.is_empty() {
                {
                    schema_info
                        .attributes
                        .push(schema_info.new_attribute.clone());
                    schema_info.new_attribute.clear();
                }
            }
        });

        let mut to_remove = Vec::new();
        for (index, attribute) in schema_info.attributes.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(attribute);
                if ui.button("-").clicked() {
                    to_remove.push(index);
                }
            });
        }
        for index in to_remove.iter().rev() {
            schema_info.attributes.remove(*index);
        }
        if ui.button("Schema Done").clicked() {
            schema_info.schema_done_clicked = true;
        }

        // Check the variable to decide whether to display the schema information or not
        if schema_info.schema_done_clicked {
            // Display the full schema struct (minus "new_attribute") on the right side
            ui.colored_label(egui::Color32::KHAKI, "Schema:");
            ui.label(format!("Name: {}", schema_info.schema_name));
            ui.label(format!("Version: {}", schema_info.schema_version));
            ui.colored_label(egui::Color32::KHAKI, "Attributes:");
            for attribute in &schema_info.attributes {
                ui.label(attribute);
            }
            let wallet_ref = wallet.as_ref().unwrap();
            let schema_to_publish: IndySchema = IndySchema::SchemaV1(IndySchemaV1 {
                id: SchemaId::new(
                    &DidValue(wallet_ref.did.clone()),
                    &schema_info.schema_name.clone(),
                    &schema_info.schema_version.clone(),
                ),
                name: schema_info.schema_name.clone(),
                version: schema_info.schema_version.clone(),
                attr_names: AttributeNames::from(schema_info.attributes.clone()),
                seq_no: None,
            });
            ui.label(format!("Schema to publish: {:?}", schema_to_publish));
            let is_schema_valid = schema_to_publish.validate();

            match is_schema_valid {
                Ok(_) => ui.label("The schema seems valid."),
                Err(e) => ui.label(format!("Invalid schema: {} ", e)),
            };
            if ui.button("Register Schema").clicked() && genesis_source.is_some() {
                ui.label("Registering Schema...");

                let wallet_ref = wallet.as_ref().unwrap();
                if let Some(ledger) = ledgers {
                    match block_on(IndyLedger::publish_schema(
                        ledger,
                        wallet_ref,
                        &wallet_ref.did,
                        &schema_to_publish,
                    )) {
                        Ok(result) => {
                            *txn_result = result;
                        }
                        Err(e) => {
                            *txn_result = e.to_string();
                        }
                    }
                }
            }
        }
    //endregion
    } else if *publish_option == PublishEntities::Custom.to_string() {
        //region CUSTOM REGISTRATION
        ui.heading("Custom txn registration");
        ui.separator();

        ui.add(
            egui::TextEdit::multiline(txn)
                .hint_text("Input Signed Transaction")
                .desired_width(f32::INFINITY),
        );
        ui.separator();
        if ui.button("Register Custom Txn").clicked() {
            match block_on(IndyLedger::write_signed_transaction_to_ledger(
                ledgers.as_ref().unwrap(),
                wallet.as_ref().unwrap(),
                txn,
            )) {
                Ok(result) => {
                    *txn_result = result;
                }
                Err(e) => {
                    *txn_result = e.to_string();
                }
            }
        }

        ui.vertical(|ui| {
            ui.label("Signed Transaction:");
            if let Some(result) = &signed_txn_result {
                // ui.colored_label(egui::Color32::GREEN, "Signed Transaction:");
                ui.colored_label(egui::Color32::GREEN, result.clone());

                ui.separator();
                // Add a button to copy the unescaped_json content
                if ui.button("Copy output").clicked() {
                    let r = result.clone();
                    ui.output_mut(|o| o.copied_text = r);
                };
            }
        });
        ui.separator();
        //endregion
    } else if *publish_option == PublishEntities::Nym.to_string() {
        ui.vertical(|ui| {
            ui.heading("NYM Registration");
            ui.colored_label(
                egui::Color32::from_rgb(144, 238, 144),
                "Enter the NYM DID and Verkey that you want to register",
            );
            ui.colored_label(egui::Color32::from_rgb(144, 238, 144), "NYM DID: ");
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
        ui.colored_label(egui::Color32::from_rgb(144, 238, 144), "NYM Alias:");
        if let Some(alias) = &mut nym_info.alias {
            ui.add(egui::TextEdit::singleline(alias).hint_text("NYM Alias"));
            if alias.trim().is_empty() {
                nym_info.alias = None;
            }
        } else {
            let mut new_alias = String::new();
            ui.add(egui::TextEdit::singleline(&mut new_alias).hint_text("NYM Alias"));
            if !new_alias.trim().is_empty() {
                nym_info.alias = Some(new_alias);
            }
        }

        ui.label("Select the role for the NYM");
        egui::ComboBox::from_id_source("my_role_nym")
            .selected_text(format!("{:?}", nym_role))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut *nym_role, MyRoles::Author, "Author");
                ui.selectable_value(&mut *nym_role, MyRoles::Endorser, "Endorser");
                ui.selectable_value(&mut *nym_role, MyRoles::NetworkMonitor, "Network Monitor");
                ui.selectable_value(&mut *nym_role, MyRoles::Steward, "Steward");
                ui.selectable_value(&mut *nym_role, MyRoles::Trustee, "Trustee");
            });

        // Check each field and add the name of the missing fields to a vector
        let mut missing_fields = Vec::new();
        if nym_info.did.is_empty() || !is_valid_did.is_ok() {
            missing_fields.push("NYM DID");
        }
        if nym_info.verkey.is_empty() || !is_valid_verkey.is_ok() {
            missing_fields.push("NYM Verkey");
        }
        if genesis_source.is_none() {
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
                && genesis_source.is_some()
            {
                ui.label("Registering NYM...");

                let wallet_ref = wallet.as_ref().unwrap();
                let role = match nym_role {
                    MyRoles::Author => UpdateRole::Reset,
                    MyRoles::Endorser => UpdateRole::Set(LedgerRole::Endorser),
                    MyRoles::NetworkMonitor => UpdateRole::Set(LedgerRole::NetworkMonitor),
                    MyRoles::Steward => UpdateRole::Set(LedgerRole::Steward),
                    MyRoles::Trustee => UpdateRole::Set(LedgerRole::Trustee),
                };
                if let Some(ledger) = ledgers {
                    match block_on(IndyLedger::publish_nym(
                        ledger,
                        wallet_ref,
                        &wallet_ref.did,
                        nym_info,
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
    }
    ui.separator();
    ui.label("Result:");
    ui.monospace(format!("{:?}", txn_result));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::MyRoles;
    use indy_vdr::ledger::{constants, RequestBuilder};
    use indy_vdr::pool::ProtocolVersion;
    use serde_json::json;

    #[test]
    fn test_author_role_mapping() -> anyhow::Result<()> {
        // First test the enum mapping
        let role = match MyRoles::Author {
            MyRoles::Author => UpdateRole::Reset,
            _ => panic!("Wrong role mapping"),
        };
        assert_eq!(role, UpdateRole::Reset);

        // Create request builder directly
        let request_builder = RequestBuilder::new(ProtocolVersion::default());
        let submitter_did = DidValue("V4SGRU86Z58d6TV7PBUe6f".to_string());
        let target_did = DidValue("7RR5ZhPkxRnNFsV6uhNDfq".to_string());

        let request = request_builder.build_nym_request(
            &submitter_did,
            &target_did,
            None, // no verkey
            None, // no alias
            Some(UpdateRole::Reset),
            None,
            None,
        )?;

        // Check that the operation contains null role as shown in the indy-vdr test
        let expected_operation = json!({
            "type": constants::NYM,
            "dest": target_did.to_string(),
            "role": serde_json::Value::Null,
        });

        let request_json: serde_json::Value = serde_json::from_str(&request.req_json.to_string())?;

        // Test the role specifically
        assert_eq!(
            request_json["operation"]["role"],
            serde_json::Value::Null,
            "Role should be null for Author"
        );

        // Test the complete operation matches expected format
        let operation = &request_json["operation"];
        assert_eq!(
            operation["type"],
            constants::NYM,
            "Transaction type should be NYM"
        );
        assert_eq!(
            operation["dest"],
            target_did.to_string(),
            "Destination DID should match"
        );

        Ok(())
    }
}
