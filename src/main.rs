mod func;
use func::{expression, Expression};
use slint::{self, SharedString};
use std::cell::RefCell;
use std::rc::Rc;
slint::include_modules!();
use dark_light::Mode;

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

fn main() {
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
        let char = char.chars().nth(0).unwrap();
        let mut paren_str = paren_str_clone.borrow_mut();
        let mut paren_count = paren_clone.borrow_mut();
        let mut points = point_clone.borrow_mut();
        let mut base_expression = pika.borrow_mut();
        let last_char = match base_expression.chars().last() {
            Some(x) => x,
            None => ' ',
        };
        if char.is_numeric() || char == '.' {
            if last_char == ')' {
                base_expression.push('×');
            }

            if char == '.' {
                if !*points {
                    if (matches!(last_char, '+' | '-' | '×' | '÷' | '(' | ')') || base_expression.is_empty()) && !*points {
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
            if last_char == char {
                base_expression.pop();
                paren_str.pop();
            } else if matches!(last_char, |'+'| '-' | '×' | '÷') && matches!(char, '+' | '÷' | '×')
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

            } else {
                if matches!(last_char, '(' | '.') && matches!(char, '+' | '×' | '÷') {
                } else {
                    base_expression.push(char);
                    *points = false;
                    paren_str.push(' ');
                }
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
            } else if base_expression.is_empty() {
                base_expression.push('(');
                paren_str.push('(');
                *paren_count += 1;
            } else if last_char.is_operator() {
                base_expression.push('(');
                paren_str.push('(');
                *paren_count += 1;
            } else if last_char == '(' {
                base_expression.push('(');
                paren_str.push('(');
                *paren_count += 1;
            }
        } else if char == '<' {
            let popped = match base_expression.pop() {
                Some(x) => x,
                None => ' ',
            };
            *paren_count += if popped == '(' {
                -1
            } else if popped == ')' {
                1
            } else {
                0
            };
            paren_str.pop();
        } else if char == '=' {
            let temp = app.get_big_font();
            app.set_big_font(app.get_smol_font());
            app.set_smol_font(temp);

        }
        app.global::<elements>()
            .set_text(SharedString::from(&*base_expression));
        println!("{:?}", expression(&base_expression));
        app.global::<elements>()
            .set_paren(SharedString::from(&*paren_str));
        app.set_scroll_offset_x(app.get_default_scroll_offset_x());
        app.global::<elements>().set_unclosed_paren(if *paren_count > 0 {
            true
        } else {
            false
        });
    });

    window.run().unwrap();
}
