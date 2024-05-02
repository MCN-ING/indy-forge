use crate::app::DIDVersion;
use crate::helpers::wallet::IndyWallet;
use egui::{Button, TextEdit, Ui};
use futures_executor::block_on;
use rfd::FileDialog;

pub fn create_wallet_ui(
    ui: &mut Ui,
    seed: &mut String,
    wallet: &mut Option<IndyWallet>,
    picked_path: &mut Option<String>,
    did_version: &mut DIDVersion,
) -> anyhow::Result<()> {
    ui.colored_label(
        egui::Color32::from_rgb(144, 238, 144),
        "Create a new wallet",
    );
    ui.add(
        TextEdit::singleline(seed)
            .char_limit(32)
            .hint_text("Enter 32 bytes seed"),
    );
    ui.label(format!("Length: {}", seed.len()));
    ui.colored_label(
        egui::Color32::from_rgb(144, 238, 144),
        "Select the version for the DID.  did:Sov is 1, did:Indy is 2",
    );
    egui::ComboBox::from_id_source("did_dropdown")
        .selected_text(format!("{:?}", did_version))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut *did_version, DIDVersion::Sov, "SOV");
            ui.selectable_value(&mut *did_version, DIDVersion::Indy, "Indy");
        });

    if ui.add(Button::new("Create Wallet")).clicked() && seed.len() == 32 {
        let seed = seed.clone();
        let did_version_value = match did_version {
            DIDVersion::Sov => 1,
            DIDVersion::Indy => 2,
        };
        let new_wallet = block_on(IndyWallet::new(Some(&seed), did_version_value));

        *wallet = Some(new_wallet.unwrap());
    }

    if let Some(wallet) = wallet {
        ui.label(format!(
            "Wallet created with DID: {} and Verkey: {} ",
            wallet.did, wallet.verkey
        ));
    }

    ui.colored_label(
        egui::Color32::from_rgb(135, 206, 250),
        "The genesis file is required to publish something on a ledger",
    );
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
    }
    Ok(())
}
