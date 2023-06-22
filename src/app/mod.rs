
use std::cell::RefCell;
use std::borrow::BorrowMut;

mod tabs;

use tabs::types::Tab;

use tabs::first_tab::FirstTab;
use tabs::second_tab::SecondTab;
use tabs::third_tab::ThirdTab;


macro_rules! define_tabs {
    ( $anchors:ident,
      $instances:ident,
        $(
            { anchor = $anchor:ident, name = $name:expr, target = $target:ty, instance_name = $instance:ident },
        )*
    ) => {
        #[derive(Copy,Clone,PartialEq,Eq)]
        enum $anchors{
            $($anchor,)*
        }

        impl $anchors{
            fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$anchor => $name,
                    )*
                }
            }

            // https://stackoverflow.com/questions/21371534/in-rust-is-there-a-way-to-iterate-through-the-values-of-an-enum
            fn iter() -> impl Iterator<Item = Self> {
                [$(Self::$anchor,)*].iter().copied()
            }
        }

        /////////////////////////

        struct $instances {
            $($instance: $target,)*
        }

        impl Default for $instances {
            fn default() -> Self {
                Self {
                    $($instance: <$target>::default(),)*
                }
            }
        }

        impl $instances {
            fn get(&self, anchor: $anchors) -> &dyn Tab {
                match anchor {
                    $(
                        $anchors::$anchor => &self.$instance,
                    )*
                }
            }

            fn get_mut(&mut self, anchor: $anchors) -> &mut dyn Tab {
                match anchor {
                    $(
                        $anchors::$anchor => &mut self.$instance,
                    )*
                }
            }
        }
    }
}

define_tabs!(Anchors, Tabs,
    { anchor = First,  name = "First tab" , target = FirstTab,  instance_name = first  },
    { anchor = Second, name = "Second tab", target = SecondTab, instance_name = second },
    { anchor = Third,  name = "Third tab" , target = ThirdTab , instance_name = third  },
);

// // // // // // // // // // // 

pub struct TemplateApp {
    tabs:        Tabs,
    selected: Anchors,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tabs: Tabs::default(),
            selected: Anchors::First,
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
            });

        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Current pane");
            ui.separator();
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                for tab in Anchors::iter() {
                    if ui.selectable_label(self.selected == tab, tab.name()).clicked() {
                        self.selected = tab
                    }
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.tabs.get_mut(self.selected).update(ctx, ui);
        });
    }
}
