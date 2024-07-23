mod app;
mod keyboard;
mod input_handler;
mod output_handler;

fn main() {
    let mut app:app::App = app::App::new();
    app.run();
    return;
}
