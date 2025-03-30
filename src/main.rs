use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use my_lib::Notation;
use slint::{self, SharedString};
use std::cell::RefCell;
use std::rc::Rc;

use dark_light::Mode;
slint::include_modules!();

fn remove_first_paren(s: &mut String) -> String {
    let mut rev: String = s.chars().rev().collect();
    println!("{rev:?}");
    if let Some(pos) = rev.find('(') {
        rev.remove(pos);
        rev = rev.chars().rev().collect();
        rev.push(' ');
        rev
    } else {
        s.to_string()
    }
}
trait TypeOfchar {
    fn is_operator(&self) -> bool;
    fn is_parenthesis(&self) -> bool;
}
impl TypeOfchar for char {
    fn is_operator(&self) -> bool {
        matches!(*self, '+' | '-' | '÷' | '×')
    }
    fn is_parenthesis(&self) -> bool {
        matches!(*self, '(' | ')')
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let window = AppWindow::new().unwrap();
    let oreo = Rc::new(RefCell::new(String::new()));
    let paren_count = Rc::new(RefCell::new(0));
    let paren_str = Rc::new(RefCell::new(String::new()));
    let points = Rc::new(RefCell::new(false));
    // let expression_vec: Vec<Expression<f64>> = Vec::new();

    window
        .global::<theme>()
        .set_dark(match dark_light::detect() {
            Mode::Dark => true,
            Mode::Light => false,
            _ => true,
        });

    let weak = window.as_weak();
    let pika = Rc::clone(&oreo);
    let paren_clone = Rc::clone(&paren_count);
    let paren_str_clone = Rc::clone(&paren_str);
    let point_clone = Rc::clone(&points);

    window.global::<elements>().on_touch(move |char| {
        let app = weak.upgrade().unwrap();
        let char = char.to_string();
        let char = char.chars().next().unwrap();
        let mut paren_str = paren_str_clone.borrow_mut();
        let mut paren_count = paren_clone.borrow_mut();
        let mut points = point_clone.borrow_mut();
        let mut base_expression = pika.borrow_mut();
        let last_char = base_expression.chars().last().unwrap_or(' ');
        if char.is_numeric() || char == '.' {
            if last_char == ')' {
                base_expression.push('×');
                paren_str.push(' ');
            }

            if char == '.' {
                if !*points {
                    if (matches!(last_char, '+' | '-' | '×' | '÷' | '(' | ')')
                        || base_expression.is_empty())
                        && !*points
                    {
                        base_expression.push('0');
                        paren_str.push(' ');
                    }
                    *points = true;
                    base_expression.push('.');
                    paren_str.push(' ');
                }
            } else {
                base_expression.push(char);
                paren_str.push(' ');
            }
        } else if char.is_operator() {
            if last_char == char
                || (matches!(last_char, |'+'| '-' | '×' | '÷') && matches!(char, '+' | '÷' | '×'))
            {
                base_expression.pop();
                paren_str.pop();
            } else if matches!(last_char, '×' | '÷') && char == '-' {
                base_expression.push('(');
                paren_str.push('(');
                *paren_count += 1;
            } else if last_char == '+' && char == '-' || last_char == '-' && char == '+' {
                base_expression.pop();
                paren_str.pop();
            } else if last_char == '.' {
            }

            if base_expression.is_empty() {
                //else if last_char == {
            } else if matches!(last_char, '(' | '.') && matches!(char, '+' | '×' | '÷') {
            } else {
                base_expression.push(char);
                *points = false;
                paren_str.push(' ');
            }
        } else if char == 'C' {
            base_expression.clear();
            *points = false;
            *paren_count = 0;
            paren_str.clear();
        } else if char.is_parenthesis() {
            if last_char.is_numeric() || last_char == ')' {
                if *paren_count == 0 {
                    base_expression.push('×');
                    base_expression.push('(');
                    paren_str.push(' ');
                    paren_str.push('(');
                    *paren_count += 1;
                } else {
                    base_expression.push(')');
                    *paren_str = remove_first_paren(&mut paren_str);
                    paren_str.push(' ');
                    *paren_count -= 1;
                }
            } else if base_expression.is_empty() || last_char.is_operator() || last_char == '(' {
                base_expression.push('(');
                paren_str.push('(');
                *paren_count += 1;
            }
        } else if char == '<' {
            let popped = base_expression.pop().unwrap_or(' ');
            *paren_count += if popped == '(' {
                -1
            } else if popped == ')' {
                1
            } else {
                0
            };
            paren_str.pop();
        } else if char == '=' {
        }
        app.global::<elements>()
            .set_text(SharedString::from(&*base_expression));
        app.global::<elements>().set_result(SharedString::from(
            base_expression.to_infix().solve().to_string(),
        ));
        println!("{}", base_expression.to_infix());
        app.global::<elements>()
            .set_paren(SharedString::from(&*paren_str));

        app.global::<elements>()
            .set_result_state(char == '=' && !app.global::<elements>().get_result_state());
        app.set_scroll_offset_x(0.0); //);
        app.global::<elements>()
            .set_unclosed_paren(*paren_count > 0);
    });

    // let copy = window.as_weak();
    window.global::<elements>().on_copy(move |string| {

        // If you want to compile it for wayland // // // // // //
        // use wl_clipboard_rs::copy::{MimeType, Options, Source};

        // let opts = Options::new();
        // opts.copy(
        //     Source::Bytes(string.to_string().into_bytes().into()),
        //     MimeType::Autodetect,
        // ).unwrap();

        // For Compiling it on Other Platforms // // // /// // // //
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(string.to_string()).unwrap();
    });

    window.run().unwrap();
}


