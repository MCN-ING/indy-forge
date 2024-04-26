use egui::Ui;
use futures_executor::block_on;
use indy_vdr::config::PoolConfig;
use indy_vdr::ledger::constants::{LedgerRole, UpdateRole};
use indy_vdr::pool::{PoolBuilder, PoolTransactions};
use rfd::FileDialog;

use crate::app::{DIDVersion, MyRoles};
use crate::helper::{create_did, register_nym, DidInfo};

#[allow(clippy::too_many_arguments)]
pub fn nym_registration_tool(
    ui: &mut Ui,
    trustee_seed: &mut String,
    nym_result: &mut String,
    picked_path: &mut Option<String>,
    my_role: &mut MyRoles,
    nym_did: &mut DidInfo,
    trustee_did: &mut DidInfo,
    did_version: &mut DIDVersion,
) {
    ui.heading("Nym Creation Tool");
    ui.separator();

    // Add more UI elements inside the nested window

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading("Trustee Seed");
            ui.label("Enter the Trustee seed that will be used for the registration");
            ui.label("Trustee seed: ");
            ui.add(
                egui::TextEdit::singleline(trustee_seed)
                    .char_limit(32)
                    .hint_text("Enter 32 bytes seed"),
            );
            ui.label(format!("Length: {}", trustee_seed.len()));
            ui.label("Select the version used for the trustee DID.  did:Sov is 1, did:Indy is 2");
            egui::ComboBox::from_id_source("did_version_dropdown")
                .selected_text(format!("{:?}", did_version))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut *did_version, DIDVersion::Sov, "SOV");
                    ui.selectable_value(&mut *did_version, DIDVersion::Indy, "Indy");
                });
        });
        ui.separator();
        ui.vertical(|ui| {
            ui.heading("NYM Registration");
            ui.label("Enter the NYM DID and Verkey that you want to register");
            ui.label("NYM DID: ");
            ui.add(
                egui::TextEdit::singleline(&mut nym_did.did.0)
                    .char_limit(32)
                    .hint_text("NYM DID"),
            );
            ui.label("NYM Verkey: ");
            ui.add(egui::TextEdit::singleline(&mut nym_did.verkey).hint_text("NYM Verkey"));
        });
    });
    ui.label("Genesis file required");
    if ui.button("Select Genesis File").clicked() {
        if let Some(path) = FileDialog::new().pick_file() {
            *picked_path = Some(path.display().to_string());
        }
    }
    if let Some(picked_path) = picked_path {
        ui.horizontal(|ui| {
            ui.label("Picked file:");
            ui.monospace(picked_path.clone());
        });
        ui.separator();
        ui.label("Select the role for the NYM");
        egui::ComboBox::from_id_source("my_role_dropdown")
            .selected_text(format!("{:?}", my_role))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut *my_role, MyRoles::Endorser, "Endorser");
                ui.selectable_value(&mut *my_role, MyRoles::NetworkMonitor, "Network Monitor");
                ui.selectable_value(&mut *my_role, MyRoles::Steward, "Steward");
            });
        ui.separator();
        if !trustee_seed.is_empty()
            && !nym_did.did.0.is_empty()
            && !nym_did.verkey.is_empty()
            && trustee_seed.len() == 32
            && ui.button("Register Nym").clicked()
        {
            let txns = PoolTransactions::from_json_file(picked_path).unwrap();
            // Create a PoolBuilder instance
            let pool_builder = PoolBuilder::new(PoolConfig::default(), txns);
            // Convert into a thread-local Pool instance
            let pool = pool_builder.into_shared().unwrap();
            let role = match my_role {
                MyRoles::Endorser => UpdateRole::Set(LedgerRole::Endorser),
                MyRoles::NetworkMonitor => UpdateRole::Set(LedgerRole::NetworkMonitor),
                MyRoles::Steward => UpdateRole::Set(LedgerRole::Steward),
            };

            match create_did(trustee_seed.clone(), did_version.to_usize()) {
                Ok(did) => *trustee_did = did,
                Err(e) => {
                    eprintln!("Error occurred: {:?}", e);
                    return;
                }
            }

            match block_on(register_nym(trustee_did, nym_did, &role, &pool)) {
                Ok(result) => *nym_result = result,
                Err(e) => {
                    *nym_result = e.to_string();
                }
            }
        }
    }

    ui.separator();
    ui.label("Result:");
    ui.monospace(format!("{:?}", nym_result));
}
