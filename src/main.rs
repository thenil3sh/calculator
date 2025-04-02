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

    // A string to store the original inputted expression
    let base_expression = Rc::new(RefCell::new(String::new()));

    let unclosed_paren_count = Rc::new(RefCell::new(0));
    // Counts the number of unclosed parenthesis
    // Helps with warning, deciding next parenthesis, completing expressions

    let paren_str = Rc::new(RefCell::new(String::new()));
    // An additional string that stores only the opening parentheses

    let floating_point = Rc::new(RefCell::new(false));
    // Just a flag, that stores whether floating point is already used

    // // // // The Buttons get their definition, starting from here // // // //
    let button_weak = window.as_weak();

    let pika = Rc::clone(&base_expression);
    let unclosed_paren_clone = Rc::clone(&unclosed_paren_count);
    let paren_str_clone = Rc::clone(&paren_str);
    let fp_clone = Rc::clone(&floating_point);

    // Definition for elements.touch() callback from appwindow.slint
    window.global::<elements>().on_touch(move |char| {
        let app = button_weak.upgrade().unwrap();

        // Convert the pushed button's text to a character
        let char = char.to_string().chars().next().unwrap();

        // Gain access to inner mutable values for reference clones
        let mut paren_str = paren_str_clone.borrow_mut();
        let mut paren_count = unclosed_paren_clone.borrow_mut();
        let mut points = fp_clone.borrow_mut();
        let mut base_expression = pika.borrow_mut();

        let last_char = base_expression.chars().last().unwrap_or(' ');
        // Stores the last character pushed onto base_expression
        // This helps preventing users from pushing mal expressions, that can't be calculated
        // For example this prevents expressions like 3 +-2 or 3 * (2() 3)

        // // The check for pushed character starts here // //

        // if the incoming character is numeric or floating_point
        if char.is_numeric() || char == '.' {
            if last_char == ')' {
                base_expression.push('×');
                paren_str.push(' ');
            }

            // if incoming character is actually floating_point
            if char == '.' {
                if !*points {
                    // if last character was one of the operators
                    if (matches!(last_char, '+' | '-' | '×' | '÷' | '(' | ')')
                    // or the base expression was empty
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

        // if incoming character is an operator
        } else if char.is_operator() {
            if last_char == char
                || (matches!(last_char, |'+'| '-' | '×' | '÷') && matches!(char, '+' | '÷' | '×'))
            {
                base_expression.pop();
                paren_str.pop();

            // allow minus operator if last character was ÷ or ×
            } else if matches!(last_char, '×' | '÷') && char == '-' {
                base_expression.push('(');
                paren_str.push('(');
                *paren_count += 1;

            // the plus and minus operators can replace each other
            } else if last_char == '+' && char == '-' || last_char == '-' && char == '+' {
                base_expression.pop();
                paren_str.pop();

            // ignore if last character is a floating_point
            } else if last_char == '.' {
            }

            // if base expression is empty
            if base_expression.is_empty() {
                // if base expression starts with unrary minus operator
                if char == '-' {
                    base_expression.push('(');
                    base_expression.push(char);
                    *points = false;
                    *paren_count += 1;
                    paren_str.push('(');
                    paren_str.push(' ');
                }

            // avoid pushing +, ×, ÷ to base expression last character is ( or floating_point
            } else if last_char == '(' && matches!(char, '+' | '×' | '÷') {
            } else {
                base_expression.push(char);
                *points = false;
                paren_str.push(' ');
            }

        // if clear button was pushed
        } else if char == 'C' {
            base_expression.clear();
            *points = false;
            *paren_count = 0;
            paren_str.clear();

        // if a parenthesis is being pushed into base_expression
        } else if char.is_parenthesis() {
            // if last character is a number or closing_parenthesis
            if last_char.is_numeric() || last_char == ')' {
                // push opening_parenthesis with multiplication if there's no parenthesis to close
                if *paren_count == 0 {
                    base_expression.push('×');
                    base_expression.push('(');
                    paren_str.push(' ');
                    paren_str.push('(');
                    *paren_count += 1;

                // else push closing_parenthesis if opening_parenthesis are there
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

        // if backspace button was pushed
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
            // it does nothing functional here,
            // just toggles the focus between base_expression and the result
            // so, it's implementation is passed onto frontend
        }

        // update the base_expression on ui
        app.global::<elements>()
            .set_text(SharedString::from(&*base_expression));

        // update the just calculated result
        app.global::<elements>().set_result(SharedString::from(
            base_expression
            .to_infix() // Break down the string into infix expression
            .solve()// Converts the expression to posfix and solves it
            .to_string(), 
        ));

        // Just a debugging output of how and expression is stored
        println!("{base_expression}");
        println!("{}", base_expression.to_infix());

        // Push the parenthesis string to ui
        app.global::<elements>()
            .set_paren(SharedString::from(&*paren_str));

        // what to do if `=` button was pressed
        app.global::<elements>()
            .set_result_state(char == '=' && !app.global::<elements>().get_result_state());
        app.set_scroll_offset_x(0.0); 

        // sets, whether to show warning
        app.global::<elements>()
            .set_unclosed_paren(*paren_count > 0);
    });


    // Definition for elements.copy() callback 
    window.global::<elements>().on_copy(move |string| {

        // Sets up clipboard context, then passes a string to clipboard
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(string.to_string()).unwrap();
        // it works cross platform but, isn't yet implemented for wayland systems and wasm :(
    });

    window.run().unwrap();
}
