use crate::{
    transitions::{PracticeSessionState, SessionStates},
    utils::match_states,
};

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
            ui.heading("Jovian Cycles Practice Tool.");
            ui.heading(format!(
                "{:#?}",
                match self.session_data.current_key_data {
                    Some(data) => {
                        format!(
                            "Your current key is: {}",
                            self.note_name_list[data.nid].clone()
                        )
                    }
                    None => {
                        "No current key generated".to_owned()
                    }
                }
            ));

            if ui.button("Request New Key").clicked() {
                self.to_requesting_new_key();
                match_states(self);
            }

            let mut working_button_on = false;
            let mut resting_button_on = false;
            match self.session_state {
                SessionStates::Working => {
                    working_button_on = false;
                    resting_button_on = true;
                }
                SessionStates::Resting => {
                    working_button_on = true;
                    resting_button_on = false;
                }
                SessionStates::Waiting => {
                    working_button_on = false;
                    resting_button_on = false;
                }
                _ => todo!(),
            };

            if ui
                .add_enabled(working_button_on, egui::Button::new("Resume Practice"))
                .clicked()
            {
                if working_button_on {
                    unreachable!();
                } else {
                    self.to_working();
                    match_states(self);
                }
            }

            if ui
                .add_enabled(resting_button_on, egui::Button::new("Rest"))
                .clicked()
            {
                if resting_button_on {
                    unreachable!();
                } else {
                    self.to_resting();
                    match_states(self);
                }
            }

            if ui.button("End Practice Session").clicked() {
                self.to_resting();
                match_states(self);
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/alexei-ozerov/jovian-cycles",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
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
