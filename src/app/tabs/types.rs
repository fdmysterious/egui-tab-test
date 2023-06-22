pub trait Tab {
    /// This function shall return the name of the tab
    fn name(&self) -> &'static str;
    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}