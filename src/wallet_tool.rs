use crate::app::DIDVersion;
use crate::helpers::genesis::GenesisSource;
use crate::helpers::wallet::IndyWallet;
use egui::{Button, TextEdit, Ui};
use futures_executor::block_on;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RecentUrls {
    urls: VecDeque<String>,
    max_urls: usize,
}

impl RecentUrls {
    pub fn new(max_urls: usize) -> Self {
        Self {
            urls: VecDeque::with_capacity(max_urls),
            max_urls,
        }
    }

    pub fn add(&mut self, url: String) {
        // Remove if already exists to avoid duplicates
        if let Some(pos) = self.urls.iter().position(|x| x == &url) {
            self.urls.remove(pos);
        }

        // Add to front of queue
        self.urls.push_front(url);

        // Remove oldest if over capacity
        while self.urls.len() > self.max_urls {
            self.urls.pop_back();
        }
    }

    pub fn get_recent(&self) -> impl Iterator<Item = &String> {
        self.urls.iter()
    }

    pub fn clear(&mut self) {
        self.urls.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.urls.is_empty()
    }
}

pub fn create_wallet_ui(
    ui: &mut Ui,
    seed: &mut String,
    wallet: &mut Option<IndyWallet>,
    genesis_source: &mut Option<GenesisSource>,
    did_version: &mut DIDVersion,
    genesis_url_input: &mut String,
    recent_urls: &mut RecentUrls,
) -> anyhow::Result<()> {
    // Wallet Creation Section
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
        match block_on(IndyWallet::new(Some(&seed), did_version_value)) {
            Ok(new_wallet) => {
                *wallet = Some(new_wallet);
            }
            Err(e) => {
                ui.colored_label(
                    egui::Color32::RED,
                    format!("Failed to create wallet: {}", e),
                );
            }
        }
    }

    if let Some(wallet) = wallet {
        ui.label(format!(
            "Wallet created with DID: {} and Verkey: {}",
            wallet.did, wallet.verkey
        ));
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    // Genesis Source Section
    ui.colored_label(
        egui::Color32::from_rgb(135, 206, 250),
        "The genesis file is required to publish something on a ledger",
    );
    ui.add_space(8.0);

    // Option 1: Local File
    ui.horizontal(|ui| {
        ui.colored_label(egui::Color32::from_rgb(144, 238, 144), "Option 1:");
        if ui.button("Select Local Genesis File").clicked() {
            if let Some(path) = FileDialog::new().pick_file() {
                *genesis_source = GenesisSource::from_str(&path.display().to_string()).ok();
            }
        }
    });

    ui.add_space(8.0);

    // Option 2: URL
    ui.colored_label(
        egui::Color32::from_rgb(144, 238, 144),
        "Option 2: Use a genesis URL",
    );

    let recent_urls_text = if recent_urls.is_empty() {
        "No recent URLs"
    } else {
        "Select from recent URLs"
    };

    // Recent URLs dropdown
    egui::ComboBox::from_id_source("recent_urls")
        .selected_text(recent_urls_text)
        .width(500.0)
        .show_ui(ui, |ui| {
            for url in recent_urls.get_recent() {
                if ui
                    .selectable_value(genesis_url_input, url.clone(), url)
                    .clicked()
                {
                    if let Ok(source) = GenesisSource::from_str(genesis_url_input) {
                        *genesis_source = Some(source);
                    }
                }
            }
        });

    ui.horizontal(|ui| {
        let response = ui.add(
            TextEdit::singleline(genesis_url_input)
                .hint_text("https://example.com/genesis.txt")
                .desired_width(500.0)
                .min_size(egui::vec2(500.0, 0.0)),
        );

        let submit_clicked = ui.button("Load URL").clicked();
        let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

        if (submit_clicked || enter_pressed) && !genesis_url_input.is_empty() {
            match GenesisSource::from_str(genesis_url_input) {
                Ok(source) => {
                    recent_urls.add(genesis_url_input.clone());
                    *genesis_source = Some(source);
                    ui.memory_mut(|mem| mem.close_popup());
                }
                Err(e) => {
                    ui.colored_label(egui::Color32::RED, format!("Invalid URL: {}", e));
                }
            }
        }
    });

    // Only show clear history if we have URLs
    if !recent_urls.is_empty() {
        ui.horizontal(|ui| {
            if ui.small_button("🗑 Clear History").clicked() {
                recent_urls.clear();
            }
            ui.small("Remove all saved URLs");
        });
    }

    // Show current genesis source status
    if let Some(source) = genesis_source {
        ui.add_space(5.0);
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

    // Helper text at the bottom with enhanced visibility
    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    // Using horizontal layout for the tip icon and text
    ui.horizontal(|ui| {
        // Add a tip icon
        ui.label("💡");

        // Use colored label with larger text
        ui.colored_label(
            egui::Color32::from_rgb(130, 190, 255),
            egui::RichText::new(
                "Tip: Press Enter or click 'Load URL' to load genesis file from URL",
            )
            .size(16.0) // Larger text size
            .strong(), // Make it bold
        );
    });
    ui.add_space(10.0); // Add some space after the tip

    Ok(())
}
