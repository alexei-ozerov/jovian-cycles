use crate::{
    transitions::{PracticeSessionState, SessionStates},
    utils::match_states,
};

use egui::{Align, Direction};
use log::{debug, info};

impl PracticeSessionState {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for PracticeSessionState {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut open = true;
            egui::Window::new("Jovian Cycles")
                .open(&mut open)
                .resizable([true, false])
                .default_width(150.0)
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        egui::Grid::new("center_pane")
                            .min_col_width(120.0)
                            .max_col_width(150.0)
                            .show(ui, |ui| {
                                ui.with_layout(
                                    egui::Layout::top_down_justified(Align::LEFT),
                                    |ui| match self.session_data.current_key_data {
                                        Some(data) => {
                                            ui.heading(format!(
                                                "Your current key is: {}",
                                                self.note_name_list[data.nid].clone()
                                            ));
                                        }
                                        None => {
                                            ui.heading("No current key.");
                                        }
                                    },
                                );

                                ui.end_row();

                                let mut working_button_on = false;
                                let mut resting_button_on = false;
                                let mut skip_button_on = false;
                                match self.session_state {
                                    SessionStates::RequestingNewKey => {
                                        working_button_on = true;
                                        resting_button_on = false;
                                        skip_button_on = false;
                                    }
                                    SessionStates::Working => {
                                        working_button_on = false;
                                        resting_button_on = true;
                                        skip_button_on = true;
                                    }
                                    SessionStates::Resting => {
                                        working_button_on = true;
                                        resting_button_on = false;
                                        skip_button_on = true;
                                    }
                                    SessionStates::Waiting => {
                                        working_button_on = false;
                                        resting_button_on = false;
                                        skip_button_on = false;
                                    }
                                    _ => {}
                                };

                                ui.with_layout(
                                    egui::Layout::top_down_justified(Align::LEFT),
                                    |ui| {
                                        if ui.button("Request New Key").clicked() {
                                            self.to_requesting_new_key();
                                            match_states(self);
                                        }
                                    },
                                );

                                ui.with_layout(
                                    egui::Layout::top_down_justified(Align::LEFT),
                                    |ui| {
                                        if ui
                                            .add_enabled(
                                                skip_button_on,
                                                egui::Button::new("Skip Selected Key"),
                                            )
                                            .clicked()
                                        {
                                            if skip_button_on {
                                                info!("Skipping current key!");
                                                self.decrement_key();
                                                self.to_requesting_new_key();
                                                match_states(self);
                                            } else {
                                                debug!(
                                                    "Button not currently functional in this state"
                                                );
                                            }
                                        }
                                    },
                                );

                                ui.end_row();

                                ui.with_layout(
                                    egui::Layout::top_down_justified(Align::LEFT),
                                    |ui| {
                                        if ui
                                            .add_enabled(
                                                resting_button_on,
                                                egui::Button::new("Pause Practice Session"),
                                            )
                                            .clicked()
                                        {
                                            if resting_button_on {
                                                self.to_resting();
                                                match_states(self);
                                            } else {
                                                debug!(
                                                    "Button not currently functional in this state"
                                                );
                                            }
                                        }
                                    },
                                );

                                ui.with_layout(
                                    egui::Layout::top_down_justified(Align::LEFT),
                                    |ui| {
                                        if ui
                                            .add_enabled(
                                                working_button_on,
                                                egui::Button::new("Resume Practice Session"),
                                            )
                                            .clicked()
                                        {
                                            if working_button_on {
                                                self.to_working();
                                                match_states(self);
                                            } else {
                                                debug!(
                                                    "Button not currently functional in this state"
                                                );
                                            }
                                        }
                                    },
                                );

                                ui.end_row();

                                ui.with_layout(
                                    egui::Layout::top_down_justified(Align::LEFT),
                                    |ui| {
                                        if ui.button("End Practice Session").clicked() {
                                            self.to_finishing();
                                            match_states(self);

                                            // Send command to exit
                                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                        }
                                    },
                                );
                            });

                        ui.separator();

                        ui.add(egui::github_link_file!(
                            "https://github.com/alexei-ozerov/jovian-cycles/tree/main/",
                            "Source code."
                        ));

                        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                            powered_by_egui_and_eframe(ui);
                            egui::warn_if_debug_build(ui);
                        });
                    });
                });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
