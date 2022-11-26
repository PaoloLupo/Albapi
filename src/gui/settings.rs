use eframe::egui;
// Se escoge el modo reactivo para reducir el uso de la CPU
#[derive(Default)]
enum RunMode {
    #[default]
    Reactive,
    Continuous,
}

pub struct Settings {
    pub open: bool,
    run_mode: RunMode,
    repaint_after_seconds: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            open: false,
            run_mode: RunMode::default(),
            repaint_after_seconds: 1.0,
        }
    }
}

impl Settings {
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.run_mode {
            RunMode::Continuous => {
                ctx.request_repaint();
            }
            RunMode:: Reactive => {
                ctx.request_repaint_after(std::time::Duration::from_secs_f32(
                    self.repaint_after_seconds,
                ));
            }
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {

    }
}
