use egui;
use eframe;

use crate::app::tabs::types::Tab;

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
    fn name(&self) -> &'static str {
        "Test tab 2"
    }
}

impl Tab for SecondTab {
    fn name(&self) -> &'static str {
        "Second tab"
    }

    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Second tab!");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Test number");
            ui.add(egui::Slider::new(&mut self.value, 0.0..=360.0).suffix("°C"));
        });
    }
}

