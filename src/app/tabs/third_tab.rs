use egui;
use crate::app::tabs::types::Tab;

pub struct ThirdTab {
}

impl Default for ThirdTab {
    fn default() -> Self {
        Self {
        }
    }
}

impl Tab for ThirdTab {
    fn name(&self) -> &'static str {
        "Third tab"
    }

    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Incredible third tab");
        ui.separator();
    }
}

