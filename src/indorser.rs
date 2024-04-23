use crate::helpers::{create_did, sign_transaction};
use egui::Ui;
pub fn endorser_tool(
    ui: &mut Ui,
    endorser_seed: &mut String,
    txn: &mut String,
    signed_txn_result: &mut Option<String>,
) {
    ui.label("Sign Txn with Endorser DID");
    // Add more UI elements inside the nested window
    ui.heading("Endorser");

    ui.horizontal(|ui| {
        ui.label("Endorser seed: ");
        ui.add(
            egui::TextEdit::singleline(endorser_seed)
                .char_limit(32)
                .hint_text("Enter 32 bytes seed"),
        );
        ui.label(format!("Length: {}", endorser_seed.len()));
    });
    ui.separator();
    if endorser_seed.len() == 32 {
        let endorser_did = create_did(endorser_seed.clone()).unwrap();
        ui.colored_label(
            egui::Color32::KHAKI,
            format!("DID: {:?}", endorser_did.did.0),
        );
        ui.colored_label(
            egui::Color32::KHAKI,
            format!("Verkey: {:?}", endorser_did.verkey),
        );
        ui.separator();

        let response_txn = ui.add(
            egui::TextEdit::multiline(txn)
                .hint_text("Input Transaction")
                .desired_width(f32::INFINITY),
        );
        ui.separator();
        if response_txn.changed() {
            let signed_txn = sign_transaction(endorser_did, txn.clone());
            match signed_txn {
                Ok(txn) => {
                    let unescaped_json = serde_json::to_string(&txn).unwrap();
                    *signed_txn_result = Some(unescaped_json.clone().to_string());
                }
                Err(e) => {
                    *signed_txn_result = Some(format!("Error: {:?}", e));
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
        // if ui.button("Apply Transaction").clicked() {
        //     // Apply the signed transaction to the ledger
        //     // apply_transaction(&signed_txn_result);
        //     write_signed_transaction_to_ledger(signed_txn_result.clone());
        // }
        ui.separator();
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    }
}
