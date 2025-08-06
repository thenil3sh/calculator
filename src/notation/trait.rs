use crate::{
    expression::Expression::{self, Number, Op, Parenthesis},
    notation::r#type::NotationType,
    operator::{
        Operator::{Add, Divide, Multiply, RaisedTo, Subtract},
        ParenType::{Left, Right},
    },
};

pub trait Notation {
    fn to_infix(&self) -> NotationType;
}

impl Notation for String {
    fn to_infix(&self) -> NotationType {
        let mut char_set = String::from(self);
        char_set.push(' ');
        let char_set = char_set.chars();
        let mut expression: Vec<Expression<f64>> = vec![Parenthesis(Left)];

        let mut paren_count: u16 = 1;
        let mut num = String::new();
        let mut last_i = '_';

        for i in char_set {
            //let peri = i != ' ' && i != '\'';
            if i.is_numeric() || i == '.' {
                num.push(i);
            } else if last_i.is_numeric() && !num.is_empty() {
                expression.push(Number(num.parse().unwrap()));
                num.clear();
            } else if last_i == '(' && i == '-' {
                num.push(i);
                continue;
            }
            if !i.is_numeric() {
                expression.push(match i {
                    '(' => {
                        paren_count += 1;
                        Parenthesis(Left)
                    }
                    ')' => {
                        paren_count -= 1;
                        Parenthesis(Right)
                    }
                    '+' => Op(Add),
                    '-' => Op(Subtract),
                    '×' => Op(Multiply),
                    '÷' => Op(Divide),
                    '^' => Op(RaisedTo),
                    ' ' => continue,
                    '.' => continue,
                    _ => panic!("Bro wtf"),
                });
            }
            last_i = i;
        }

        loop {
            if let Op(_) = expression.last().unwrap_or(&Expression::Number(0.0)) {
                expression.pop();
            } else if let Parenthesis(Left) = expression.last().unwrap_or(&Expression::Number(0.0))
            {
                expression.pop();
                paren_count -= 1;
            } else {
                break;
            }
        }
        while paren_count > 0 {
            expression.push(Parenthesis(Right));
            paren_count -= 1;
        }
        NotationType::Infix(expression)
    }
}
