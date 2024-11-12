use crate::app::DIDVersion;
use crate::helpers::genesis::GenesisSource;
use crate::helpers::wallet::IndyWallet;
use egui::{Button, TextEdit, Ui};
use futures_executor::block_on;
use rfd::FileDialog;

pub fn create_wallet_ui(
    ui: &mut Ui,
    seed: &mut String,
    wallet: &mut Option<IndyWallet>,
    genesis_source: &mut Option<GenesisSource>,
    did_version: &mut DIDVersion,
    genesis_url_input: &mut String,
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
    ui.horizontal(|ui| {
        if ui.button("Select Local Genesis File").clicked() {
            if let Some(path) = FileDialog::new().pick_file() {
                *genesis_source = GenesisSource::from_str(&path.display().to_string()).ok();
            }
        }

        ui.separator();

        ui.label("Or enter genesis URL:");
        let response = ui.add(
            egui::TextEdit::singleline(genesis_url_input)
                .hint_text("https://example.com/genesis.txt")
                .desired_width(300.0),
        );

        let submit_clicked = ui.button("Load URL").clicked();
        let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

        if submit_clicked || enter_pressed {
            if !genesis_url_input.is_empty() {
                ui.spinner(); // Show a loading indicator
                match GenesisSource::from_str(genesis_url_input) {
                    Ok(source) => {
                        *genesis_source = Some(source);
                    }
                    Err(e) => {
                        ui.colored_label(egui::Color32::RED, format!("Invalid URL: {}", e));
                    }
                }
            }
        }
    });

    // Add helper text
    ui.small("Press Enter or click 'Load URL' to load genesis file from URL");

    if let Some(source) = genesis_source {
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Active genesis source:");
            match source {
                GenesisSource::LocalFile(path) => {
                    ui.colored_label(egui::Color32::GREEN, format!("📁 Local file: {}", path));
                }
                GenesisSource::Url(url) => {
                    ui.colored_label(egui::Color32::GREEN, format!("🌐 URL: {}", url));
                }
            }
        });
    }

    Ok(())
}
