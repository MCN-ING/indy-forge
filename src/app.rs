use serde::{Deserialize, Serialize};

use crate::helpers::DidInfo;
use crate::indorser::endorser_tool;
use crate::nym_registration::nym_registration_tool;

#[derive(PartialEq, Eq, Deserialize, Serialize, Debug)]
pub enum MyRoles {
    Endorser = 101,
    NetworkMonitor = 201,
    Steward = 2,
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
        Self::Endorser
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
    signed_txn_result: std::option::Option<String>,
    show_indorser: bool,
    show_nym_creation: bool,
    picked_path: Option<String>,
    nym_result: String,
    my_role: MyRoles,
    nym_did: DidInfo,
    trustee_did: DidInfo,
    did_version: DIDVersion,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            trustee_seed: "".to_owned(),
            endorser_seed: "".to_owned(),
            txn: "".to_owned(),
            signed_txn_result: None,
            show_indorser: true,
            show_nym_creation: true,
            picked_path: Default::default(),
            nym_result: "".to_owned(),
            my_role: Default::default(),
            nym_did: Default::default(),
            trustee_did: Default::default(),
            did_version: DIDVersion::Indy,
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
                ui.checkbox(&mut self.show_indorser, "Endorser Tool");
                ui.checkbox(&mut self.show_nym_creation, "Nym creation Tool");
                ui.separator();
                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            // Indorser Tool section
            if self.show_indorser {
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

            // Nym Creation Tool section
            if self.show_nym_creation {
                // Only show the "Nym Creation Tool" window if `show_nym_creation` is true

                egui::Window::new("Nym Creation Tool")
                    .default_size([600.0, 300.0])
                    // .default_pos(egui::pos2(800.0, 400.0))
                    .show(ui.ctx(), |ui| {
                        nym_registration_tool(
                            ui,
                            &mut self.trustee_seed,
                            &mut self.nym_result,
                            &mut self.picked_path,
                            &mut self.my_role,
                            &mut self.nym_did,
                            &mut self.trustee_did,
                            &mut self.did_version,
                        );
                    });
            }
        });
    }
}
