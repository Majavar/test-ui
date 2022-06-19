slint::include_modules!();

pub fn main() {
    let app = App::new();

    app.on_button_pressed(slint::quit_event_loop);
    app.run();
}
