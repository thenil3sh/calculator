use slint::{self, SharedString};
use std::cell::RefCell;
use std::rc::Rc;
slint::include_modules!();
use dark_light::Mode;

fn main() {
    let window = AppWindow::new().unwrap();
    let oreo = Rc::new(RefCell::new(String::new()));
    window.global::<theme>().set_dark(    match dark_light::detect() {
        Mode::Dark => true,
        Mode::Light => false,
        _ => true,
    });


    
    
    let weak = window.as_weak();
    let pika = Rc::clone(&oreo);
    window.global::<elements>().on_touch(move |str|{
        let app = weak.upgrade().unwrap();
        let mut oreo = pika.borrow_mut();
        oreo.push(str.to_string().chars().nth(0).unwrap());
        app.global::<elements>().set_text(SharedString::from(&*oreo));
    });
    
    
    
    
    
    window.run().unwrap();
    
}