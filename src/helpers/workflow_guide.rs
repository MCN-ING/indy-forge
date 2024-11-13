// In src/helpers/workflow_guide.rs

use egui::{Color32, RichText, Ui};

pub struct WorkflowGuide {
    pub has_wallet: bool,
    pub has_genesis: bool,
}

impl WorkflowGuide {
    pub fn new(has_wallet: bool, has_genesis: bool) -> Self {
        Self {
            has_wallet,
            has_genesis,
        }
    }

    pub fn show(&self, ctx: &egui::Context) {
        let current_step = if !self.has_wallet {
            1
        } else if !self.has_genesis {
            2
        } else {
            3
        };

        egui::Window::new("Getting Started Guide")
            .default_size([400.0, 500.0])
            .show(ctx, |ui| {
                // Header section
                ui.add_space(8.0);
                let header_text = RichText::new("Getting Started with IndyForge")
                    .size(18.0)
                    .strong()
                    .color(Color32::WHITE);
                ui.colored_label(Color32::from_rgb(50, 50, 50), header_text);

                ui.add_space(4.0);
                ui.colored_label(
                    Color32::LIGHT_GRAY,
                    "Follow these steps to start working with the Indy ledger. Complete each step in order."
                );
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Step 1: Create Wallet
                    self.show_step(
                        ui,
                        1,
                        current_step,
                        self.has_wallet,
                        "Create Wallet",
                        "Enter a 32-byte seed and select DID version to create your wallet",
                        &[
                            "Enter a 32-character seed in the input field",
                            "Choose between DID:SOV (v1) or DID:INDY (v2)",
                            "Click \"Create Wallet\" to generate your DID",
                        ],
                    );

                    // Step 2: Configure Genesis
                    self.show_step(
                        ui,
                        2,
                        current_step,
                        self.has_genesis,
                        "Configure Genesis",
                        "Select a local genesis file or enter a genesis URL",
                        &[
                            "Choose between local file or URL input",
                            "For local file: Click \"Select Local Genesis File\"",
                            "For URL: Enter the genesis URL and click \"Load URL\"",
                        ],
                    );

                    // Step 3: Publish Transactions
                    self.show_step(
                        ui,
                        3,
                        current_step,
                        false,
                        "Publish Transactions",
                        "Create and submit transactions to the ledger",
                        &[
                            "Select transaction type (NYM, Schema, or Custom)",
                            "Fill in required transaction details",
                            "Choose signing and submission options",
                            "Review and submit your transaction",
                        ],
                    );

                    ui.add_space(8.0);
                });
            });
    }

    #[allow(clippy::too_many_arguments)]
    fn show_step(
        &self,
        ui: &mut Ui,
        step_number: u8,
        current_step: u8,
        is_complete: bool,
        title: &str,
        description: &str,
        instructions: &[&str],
    ) {
        ui.horizontal(|ui| {
            // Step indicator
            let indicator_color = if is_complete {
                Color32::from_rgb(34, 197, 94) // Green
            } else if step_number == current_step {
                Color32::from_rgb(59, 130, 246) // Blue
            } else {
                Color32::LIGHT_GRAY // Light gray for better visibility
            };

            let indicator_text = if is_complete {
                "✓"
            } else {
                &step_number.to_string()
            };

            ui.add_space(4.0);
            ui.label(
                RichText::new(indicator_text)
                    .color(if is_complete {
                        Color32::WHITE
                    } else {
                        indicator_color
                    })
                    .size(16.0)
                    .strong(),
            );
            ui.add_space(8.0);

            ui.vertical(|ui| {
                // Title
                let title_text = RichText::new(title)
                    .size(16.0)
                    .color(if step_number == current_step {
                        Color32::from_rgb(59, 130, 246) // Blue for current step
                    } else if is_complete {
                        Color32::from_rgb(34, 197, 94) // Green for completed
                    } else {
                        Color32::LIGHT_GRAY // Light gray for pending
                    })
                    .strong();
                ui.label(title_text);

                // Description
                ui.label(
                    RichText::new(description)
                        .size(14.0)
                        .color(Color32::LIGHT_GRAY),
                );

                // Show detailed instructions for current step
                if step_number == current_step {
                    ui.add_space(4.0);
                    let frame = egui::Frame::none()
                        .fill(Color32::from_rgb(20, 40, 80))
                        .inner_margin(8.0)
                        .rounding(4.0);

                    frame.show(ui, |ui| {
                        for instruction in instructions {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("→").color(Color32::from_rgb(59, 130, 246)));
                                ui.label(
                                    RichText::new(*instruction)
                                        .size(14.0)
                                        .color(Color32::LIGHT_GRAY),
                                );
                            });
                        }
                    });
                }
            });
        });
        ui.add_space(8.0);
    }
}
