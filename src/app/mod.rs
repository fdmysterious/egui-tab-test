
use std::cell::RefCell;
use std::borrow::BorrowMut;

mod tabs;

use tabs::first_tab::FirstTab;
use tabs::second_tab::SecondTab;

#[derive(Copy, Clone, PartialEq, Eq)]
enum CurrentTab {
    First,
    Second,
}

impl CurrentTab {
    fn name(&self) -> &'static str {
        match self {
            CurrentTab::First => "First",
            CurrentTab::Second => "Second",
        }
    }

    // https://stackoverflow.com/questions/21371534/in-rust-is-there-a-way-to-iterate-through-the-values-of-an-enum
    fn iter() -> impl Iterator<Item = Self> {
        [Self::First, Self::Second,].iter().copied()
    }
}



impl Default for CurrentTab {
    fn default() -> Self {
        Self::First
    }
}

struct Tabs {
    first:    FirstTab,
    second:   SecondTab,

    selected: CurrentTab,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            first:    FirstTab::default(),
            second:   SecondTab::default(),

            selected: CurrentTab::default(),
        }
    }
}

impl Tabs {
    fn current(&mut self) -> &mut dyn eframe::App {
        match self.selected {
            CurrentTab::First  => &mut self.first,
            CurrentTab::Second => &mut self.second,
        }
    }

    fn current_selected(&self) -> &CurrentTab {
        &self.selected
    }

    fn select(&mut self, name: CurrentTab) {
        self.selected = name;
    }
}

// // // // // // // // // // // 

pub struct TemplateApp {
    tabs: Tabs,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tabs: Tabs::default(),
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
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    //fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //    eframe::set_value(storage, eframe::APP_KEY, self);
    //}

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                egui::warn_if_debug_build(ui);
                ui.separator();

                //ui.menu_button("😵 Test Menu button", |ui| {
                //    ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`
                //});

                //ui.menu_button("👉 Test menu button", |ui| {
                //    ui.set_style(ui.ctx().style());
                //    ui.button("oops");
                //});


                //if ui.selectable_label(false, "Test").clicked() {
                //    println!("OOps");
                //}
            });

        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                for tab in CurrentTab::iter() {
                    if ui.selectable_label(*self.tabs.current_selected() == tab, tab.name()).clicked() {
                        self.tabs.select(tab);
                    }
                }
            })


            //ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //    ui.horizontal(|ui| {
            //        ui.spacing_mut().item_spacing.x = 0.0;
            //        ui.label("powered by ");
            //        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            //        ui.label(" and ");
            //        ui.hyperlink_to(
            //            "eframe",
            //            "https://github.com/emilk/egui/tree/master/crates/eframe",
            //        );
            //        ui.label(".");
            //    });
            //});
        });

        self.tabs.current().update(ctx, frame);
    }
}
