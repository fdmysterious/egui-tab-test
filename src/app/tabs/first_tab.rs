use egui;
use eframe;

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

impl FirstTab {
    fn name() -> &'static str {
        "Test tab"
    }
}

impl eframe::App for FirstTab {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("{} - test app", Self::name()));
            ui.label("This is hardworking, and the first tab");
            egui::warn_if_debug_build(ui);

            ui.separator();

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.is_win_open, "Is window opened ?")
                    .on_hover_text("Click to open window!");
            })
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

