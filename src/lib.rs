mod operator;

mod calculation;

mod expression;

pub mod notation;

use clipboard::{ClipboardContext, ClipboardProvider};
use slint::SharedString;

#[cfg(not(feature = "wayland"))]
pub fn copy_to_clipboard(string: SharedString) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(string.to_string()).unwrap();
}

#[cfg(feature = "wayland")]
use std::process::Command;

#[cfg(feature = "wayland")]
pub fn copy_to_clipboard(string: SharedString) {
    let mut copy_command = Command::new("wl-copy");
    copy_command.arg(format!("'{}'", string.to_string()));

    if let Err(reason) = copy_command.status() {
        eprintln!("Error copying : {reason}");
    }
}
