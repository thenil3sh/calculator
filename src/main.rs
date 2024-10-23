// fn sum<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> f64 {
//     x.into() + y.into()
// }

// fn multiply<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> f64 {
//     x.into() + y.into()
// }

// fn divide<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> f64 {
//     x.into() / y.into()
// }

// fn subtract<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> f64 {
//     x.into() - y.into()
// }
slint::include_modules!();
#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]

enum Paren {
    Left,
    Right,
}

#[derive(Debug)]
enum Expression<T: Into<f64>> {
    Number(T),
    Parenthesis(Paren),
    Op(Operator),
}

use Expression::{Number, Op, Parenthesis};
use Operator::{Add, Divide, Multiply, Subtract};
use Paren::{Left, Right};

fn main() {
    let app = AppWindow::new().unwrap();
    app.run().unwrap();
    let expression_string = "56 55* 8 ";
    let mut expression: Vec<Expression<f64>> = Vec::new();

    let mut num = String::new();
    let mut last_i: char = ' ';

    for i in expression_string.chars() {
        //let peri = i != ' ' && i != '\'';
        if i.is_numeric() {
            num.push(i);
        } else if last_i.is_numeric() && !num.is_empty(){
            println!("Pushing {num}!!");
            expression.push(Number(num.parse().unwrap()));
            num.clear();
        }
        println!("{:?}", expression);
        if i != ' ' {
            last_i = i
        } else {
            continue;
        };
        if !i.is_numeric() {
            expression.push(match i {
                '(' => Parenthesis(Left),
                ')' => Parenthesis(Right),
                '+' => Op(Add),
                '-' => Op(Subtract),
                '*' => Op(Multiply),
                '/' => Op(Divide),
                ' ' => continue,
                '.' => continue,
                _ => panic!("Bro wtf"),
            });
        }
    }
    println!("Finally : {:?}", expression);
}
