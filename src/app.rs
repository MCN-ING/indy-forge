use crate::helpers::genesis::GenesisSource;
use crate::helpers::ledgers::IndyLedger;
use crate::helpers::wallet::IndyWallet;
use crate::indorser::endorser_tool;
use crate::publish_tool::publish_tool_ui;
use crate::wallet_tool::create_wallet_ui;
use egui::TextBuffer;
use futures_executor::block_on;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
pub enum MyRoles {
    Author = 999,
    Endorser = 101,
    NetworkMonitor = 201,
    Steward = 2,
    Trustee = 0,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TransactionOptions {
    pub sign: bool,
    pub send: bool,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            sign: true,
            send: true,
        }
    }
}

pub struct SchemaInfo {
    pub schema_name: String,
    pub schema_version: String,
    pub attributes: Vec<String>,
    pub new_attribute: String,
    pub schema_done_clicked: bool,
}

pub struct ToolVisibility {
    show_endorser: bool,
    show_publish_tool: bool,
    show_wallet_tool: bool,
}

#[derive(Debug)]
pub struct NymInfo {
    pub(crate) did: String,
    pub(crate) verkey: String,
    pub(crate) alias: Option<String>,
}
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
pub enum DIDVersion {
    Sov,
    Indy,
}
impl DIDVersion {
    pub fn to_usize(&self) -> usize {
        match self {
            DIDVersion::Sov => 1,
            DIDVersion::Indy => 2,
        }
    }
}
impl Default for MyRoles {
    fn default() -> Self {
        Self::Author
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    // #[serde(skip)] // This how you opt-out of serialization of a field
    trustee_seed: String,
    endorser_seed: String,
    txn: String,
    signed_txn_result: Option<String>,
    tool_visibility: ToolVisibility,
    genesis_source: Option<GenesisSource>,
    nym_role: MyRoles,
    did_version: DIDVersion,
    wallet: Option<IndyWallet>,
    publish_option: String,
    nym_info: NymInfo,
    ledgers: Option<IndyLedger>,
    txn_result: String,
    schema_info: SchemaInfo,
    genesis_url_input: String,
    ledger_connecting: bool,
    ledger_error: Option<String>,
    genesis_content: Option<String>,
    show_genesis_content: bool,
    current_genesis_path: Option<String>,
    connection_start_time: Option<std::time::Instant>,
    transaction_options: TransactionOptions,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            trustee_seed: "".to_owned(),
            endorser_seed: "".to_owned(),
            txn: "".to_owned(),
            signed_txn_result: None,
            tool_visibility: ToolVisibility {
                show_endorser: true,
                show_publish_tool: true,
                show_wallet_tool: true,
            },
            genesis_source: Default::default(),
            nym_role: Default::default(),
            did_version: DIDVersion::Indy,
            wallet: None,
            publish_option: "".to_owned(),
            nym_info: NymInfo {
                did: "".to_owned(),
                verkey: "".to_owned(),
                alias: None,
            },
            ledgers: None,
            txn_result: "".to_owned(),
            schema_info: SchemaInfo {
                schema_name: "".to_owned(),
                schema_version: "".to_owned(),
                attributes: Vec::new(),
                new_attribute: "".to_owned(),
                schema_done_clicked: false,
            },
            genesis_url_input: String::new(),
            ledger_connecting: false,
            ledger_error: None,
            genesis_content: None,
            show_genesis_content: false,
            current_genesis_path: None,
            connection_start_time: None,
            transaction_options: TransactionOptions::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        egui::SidePanel::right("side_panel")
            .min_width(150.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Tools");
                });
                // Add a checkbox to toggle the visibility of the "Indorser" window
                ui.checkbox(&mut self.tool_visibility.show_endorser, "Endorser Tool");
                ui.checkbox(&mut self.tool_visibility.show_publish_tool, "Publish Tool");
                ui.checkbox(&mut self.tool_visibility.show_wallet_tool, "Wallet Tool");
                ui.separator();
                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            // Indorser Tool section
            if self.tool_visibility.show_endorser {
                // Only show the "Indorser" window if `show_indorser` is true
                egui::Window::new("Endorser Tool")
                    // .default_pos(egui::pos2(0.0, 0.0))
                    .show(ui.ctx(), |ui| {
                        endorser_tool(
                            ui,
                            &mut self.endorser_seed,
                            &mut self.txn,
                            &mut self.signed_txn_result,
                            &mut self.did_version,
                        );
                    });
            }



            if self.tool_visibility.show_wallet_tool {
            // Wallet Tool section
            egui::Window::new("Wallet Tool")
                .default_size([600.0, 300.0])
                .show(ui.ctx(), |ui| {
                    ui.heading("Wallet Tool");
                    ui.separator();
                    ui.label("Tool that create a temporary wallet and hold the DID used by the other tools");
                    create_wallet_ui(ui, &mut self.trustee_seed, &mut self.wallet, &mut self.genesis_source, &mut self.did_version,  &mut self.genesis_url_input,).expect("Something went wrong with the wallet creation");
                });
            }
            if self.tool_visibility.show_publish_tool {
                egui::Window::new("Publish Tool")
                    .default_size([600.0, 300.0])
                    .show(ui.ctx(), |ui| {
                        ui.heading("Publish Tool");
                        ui.separator();

                        // Check wallet and genesis separately
                        let has_wallet = self.wallet.is_some();
                        let has_genesis = self.genesis_source.is_some();

                        // Check if genesis source has changed
                        if let Some(genesis_source) = &self.genesis_source {
                            let new_path = match genesis_source {
                                GenesisSource::LocalFile(path) => Some(path.clone()),
                                GenesisSource::Url(url) => Some(url.clone()),
                            };

                            if new_path != self.current_genesis_path {
                                // Genesis source has changed, reset everything
                                self.current_genesis_path = new_path;
                                self.ledgers = None;
                                self.ledger_error = None;
                                self.genesis_content = None;
                                self.ledger_connecting = false;
                            }
                        }

                        if !has_wallet {
                            ui.colored_label(
                                egui::Color32::LIGHT_RED,
                                "Please create a wallet first"
                            );
                        }
                        if !has_genesis {
                            ui.colored_label(
                                egui::Color32::LIGHT_RED,
                                "Please select a genesis file first"
                            );
                        }

                        // Add genesis viewer if we have a genesis source
                        if has_genesis {
                            ui.separator();
                            if ui.button(
                                if self.show_genesis_content {
                                    "Hide Genesis Content"
                                } else {
                                    "Show Genesis Content"
                                }
                            ).clicked() {
                                self.show_genesis_content = !self.show_genesis_content;
                            }

                            if self.show_genesis_content {
                                match &self.genesis_source {
                                    Some(GenesisSource::LocalFile(path)) => {
                                        if let Ok(content) = std::fs::read_to_string(path) {
                                            show_genesis_content(ui, &content);
                                        } else {
                                            ui.colored_label(egui::Color32::RED, "Failed to read genesis file");
                                        }
                                    }
                                    Some(GenesisSource::Url(_url)) => {
                                        if self.genesis_content.is_none() {
                                            ui.spinner();
                                            ui.label("Loading genesis content...");

                                            if let Ok(content) = block_on(self.genesis_source.as_ref().unwrap().get_content()) {
                                                self.genesis_content = Some(content);
                                            } else {
                                                ui.colored_label(egui::Color32::RED, "Failed to fetch genesis content");
                                            }
                                        }

                                        if let Some(content) = &self.genesis_content {
                                            show_genesis_content(ui, content);
                                        }
                                    }
                                    None => {}
                                }
                            }
                        }


                        // Only proceed if we have both
                        if has_wallet && has_genesis {
                            // Check if we're already connected and not in an error state
                            if self.ledgers.is_none() && !self.ledger_connecting && self.ledger_error.is_none() {
                                // Set connecting flag to true
                                self.ledger_connecting = true;

                                // Show connecting indicator
                                ui.horizontal(|ui| {
                                    ui.spinner();
                                    ui.label("Connecting to ledger...");
                                    if let Some(start_time) = self.connection_start_time {
                                        let elapsed = start_time.elapsed().as_secs();
                                        if elapsed >= 20 {  // Overall timeout of 20 seconds
                                            self.ledger_connecting = false;
                                            self.ledger_error = Some("Connection attempt timed out after 20 seconds".to_string());
                                            self.connection_start_time = None;
                                        } else {
                                            ui.label(format!("({} seconds)", elapsed));
                                        }
                                    }
                                });

                                if self.ledger_error.is_none() {  // Only try connection if we haven't timed out
                                    // Move connection attempt into a block_on block to ensure completion
                                    let connection_result = block_on(async {
                                        // Wrap the entire connection process in a timeout
                                        timeout(Duration::from_secs(20), async {
                                            match IndyLedger::new(self.genesis_source.clone().unwrap()).await {
                                                Ok(ledger) => {
                                                    match ledger.check_connection().await {
                                                        Ok(true) => {
                                                            log::info!("Successfully connected to ledger");
                                                            Ok(ledger)
                                                        }
                                                        Ok(false) => {
                                                            Err(anyhow::anyhow!("Connected to nodes but ledger is not responding correctly"))
                                                        }
                                                        Err(e) => {
                                                            Err(anyhow::anyhow!("Failed to verify ledger connection: {}", e))
                                                        }
                                                    }
                                                },
                                                Err(e) => {
                                                    Err(anyhow::anyhow!("Failed to connect to ledger: {}. Check your genesis file configuration.", e))
                                                }
                                            }
                                        }).await.unwrap_or_else(|_| Err(anyhow::anyhow!("Connection attempt timed out")))
                                    });

                                    // Handle connection result
                                    match connection_result {
                                        Ok(ledger) => {
                                            self.ledgers = Some(ledger);
                                            self.ledger_error = None;
                                        }
                                        Err(e) => {
                                            log::error!("{}", e);
                                            self.ledger_error = Some(e.to_string());
                                        }
                                    }

                                    self.ledger_connecting = false;
                                    self.connection_start_time = None;
                                }
                            }

                            // Periodically check connection if we're connected
                            if let Some(ledger) = &self.ledgers {
                                if ui.input(|i| i.time).floor() as i64 % 30 == 0 { // Check every 30 seconds
                                    match block_on(ledger.check_connection()) {
                                        Ok(true) => {
                                            // Connection still good
                                            if let Some(error) = &mut self.ledger_error {
                                                if error.contains("Connection lost") {
                                                    self.ledger_error = None;
                                                }
                                            }
                                        }
                                        Ok(false) | Err(_) => {
                                            // Connection lost
                                            self.ledger_error = Some("Connection lost to ledger. Retry?".to_string());
                                            self.ledgers = None;
                                        }
                                    }
                                }
                            }

                            // First check if we need to show error and get retry action
                            let should_retry = if self.ledger_error.is_some() {
                                ui.separator();
                                ui.horizontal(|ui| {
                                    ui.colored_label(
                                        egui::Color32::RED,
                                        "❌ Connection Error"
                                    );
                                    ui.button("🔄 Retry Connection").clicked()
                                }).inner
                            } else {
                                false
                            };

                            // Then show error message if any
                            if let Some(error) = &self.ledger_error {
                                egui::ScrollArea::vertical()
                                    .max_height(100.0)
                                    .show(ui, |ui| {
                                        ui.colored_label(
                                            egui::Color32::LIGHT_RED,
                                            error
                                        );
                                    });
                            }

                            // Handle retry action after releasing the borrow
                            if should_retry {
                                self.ledgers = None;
                                self.ledger_error = None;
                                self.ledger_connecting = false;  // Ensure connecting state is reset
                            }

                            // Continue with UI if connected successfully
                            if self.ledgers.is_some() {
                                publish_tool_ui(
                                    ui,
                                    &mut self.wallet,
                                    &mut self.publish_option,
                                    &mut self.nym_role,
                                    &mut self.nym_info,
                                    &mut self.genesis_source,
                                    &mut self.ledgers,
                                    &mut self.txn_result,
                                    &mut self.schema_info,
                                    &mut self.txn,
                                    &mut self.transaction_options,
                                ).expect("Failed to render publish tool UI");
                            }
                        }
                    });
            }
        });
    }
}

// Add this helper function at the bottom of the file
fn show_genesis_content(ui: &mut egui::Ui, content: &str) {
    egui::ScrollArea::vertical()
        .max_height(200.0)
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut content.as_str())
                    .desired_width(f32::INFINITY)
                    .desired_rows(10)
                    .interactive(false), // This makes it non-interactive/read-only
            );

            if ui.button("Copy Genesis Content").clicked() {
                ui.output_mut(|o| o.copied_text = content.to_string());
            }
        });
}
