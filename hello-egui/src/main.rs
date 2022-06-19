mod app;

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2 { x: 200.0, y: 50.0 }),
        ..Default::default()
    };
    eframe::run_native(
        "Hello egui",
        native_options,
        Box::new(|_| Box::new(app::App::default())),
    );
}
