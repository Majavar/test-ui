use eframe::Frame;
use egui::Context;

#[derive(Default)]
pub struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("Vive les canards !");
                if ui.button("Quit !").clicked() {
                    frame.quit();
                };
            })
        });
    }
}
