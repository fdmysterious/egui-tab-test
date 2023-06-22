use egui;
use eframe;

pub struct SecondTab {
    value: f32,
}

impl Default for SecondTab {
    fn default() -> Self {
        Self {
            value: 0.0,
        }
    }
}

impl SecondTab {
    fn name() -> &'static str {
        "Test tab 2"
    }
}

impl eframe::App for SecondTab {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Second tab!");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Test number");
                ui.add(egui::Slider::new(&mut self.value, 0.0..=360.0).suffix("°C"));
            })
        });
    }
}

