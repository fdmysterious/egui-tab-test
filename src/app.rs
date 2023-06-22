
use std::cell::RefCell;
use std::borrow::BorrowMut;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    value: f32,
    is_win_open: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            value: 0.0,
            is_win_open: true,
        }
    }
}

impl TemplateApp {
    pub fn name() -> &'static str {
        "Test application"
    }
}

impl TemplateApp {
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

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();

                //ui.menu_button("😵 Test Menu button", |ui| {
                //    ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`
                //});

                //ui.menu_button("👉 Test menu button", |ui| {
                //    ui.set_style(ui.ctx().style());
                //    ui.button("oops");
                //});

                if ui.selectable_label(false, "Test").clicked() {
                    println!("OOps");
                }
            });

        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading(format!("{} - test app", Self::name()));
            ui.label("This is hardworking");
            egui::warn_if_debug_build(ui);

            ui.separator();

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.is_win_open, "Is window opened ?")
                    .on_hover_text("Click to open window!");
            })
        });

        if true {
            egui::Window::new("Window")
                .open(&mut self.is_win_open)
                .show(ctx, |ui| {
                    ui.label("Windows can be moved by dragging them.");
                    ui.label("They are automatically sized based on contents.");
                    ui.label("You can turn on resizing and scrolling if you like.");
                    ui.label("You would normally choose either panels OR windows.");
                })
            ;
        }
    }
}
