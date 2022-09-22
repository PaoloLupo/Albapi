use crate::Alba;
use eframe::egui;

impl Alba{
    pub fn top_bar(&mut self, ctx:&egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
            egui::menu::bar(ui, |ui|{
                ui.menu_button("File", |ui|{
                    if ui.button("Quit").clicked(){
                        frame.close();
                    }
                })
            });
        });
    }
}