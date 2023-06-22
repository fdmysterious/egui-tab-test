use egui;
use crate::app::tabs::types::Tab;

pub struct FirstTab {
    value: f32,
    is_win_open: bool
}

impl Default for FirstTab {
    fn default() -> Self {
        Self {
            value: 0.0,
            is_win_open: false
        }
    }
}

impl Tab for FirstTab {
    fn name(&self) -> &'static str {
        "First tab"
    }

    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading(format!("{} - test app", self.name()));
        ui.label("This is hardworking, and the first tab");

        ui.separator();

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.is_win_open, "Is window opened ?")
                .on_hover_text("Click to open window!");
        });

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

