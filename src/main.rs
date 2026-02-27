use calc::{handle, theme, AppWindow};
use slint::ComponentHandle;

use dark_light::Mode;

fn main() {
    // Build a window instance
    let window = AppWindow::new().unwrap();
    window
        .global::<theme>()
        .set_dark(match dark_light::detect() {
            Mode::Dark => true,
            Mode::Light => false,
            _ => true,
        });

    handle(&window);

    window.run().unwrap();
}
