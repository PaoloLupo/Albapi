use eframe::{egui, App, Frame};
use eframe::egui::Context;

pub struct Alba {
    boolean: bool,
}

impl Default for Alba {
    fn default() -> Self {
        Self { boolean: false }
    }
}

impl Alba {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for Alba {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

pub fn alba_gui_init() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Alba",
        native_options,
        Box::new(|ctx| Box::new(Alba::new(ctx))),
    )
}