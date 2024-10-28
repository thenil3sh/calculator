
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
pub enum Expression<T: Into<f64>> {
    Number(T),
    Parenthesis(Paren),
    Op(Operator),
}

use Expression::{Number, Op, Parenthesis};
use Operator::{Add, Divide, Multiply, Subtract};
use Paren::{Left, Right};

pub fn expression(expression_str : &str) -> Vec<Expression <f64>> {
    let mut expression_string = format!("{expression_str} ");
    expression_string.push(' ');
    let mut expression: Vec<Expression<f64>> = Vec::new();

    let mut num = String::new();
    let mut last_i = ' ';

    for i in expression_string.chars() {
        //let peri = i != ' ' && i != '\'';
        if i.is_numeric() || i == '.' {
            num.push(i);
        } else if last_i.is_numeric() && !num.is_empty(){
            expression.push(Number(num.parse().unwrap()));
            num.clear();

        }
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
                '×' => Op(Multiply),
                '÷' => Op(Divide),
                ' ' => continue,
                '.' => continue,
                _ => panic!("Bro wtf"),
            });
        }
    }
    expression
}
