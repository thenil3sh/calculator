use std::collections::LinkedList;

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    RaisedTo,
}

#[derive(Debug, Copy, Clone)]

pub enum Paren {
    Left,
    Right,
}

pub enum CalculationState {
    Success(f64),
    //Imaginary,
    Infinite,
    Indeterminate,
    Heart,
    Hate,
    Invalid,
    NotFound,
}

impl CalculationState {
    pub fn to_string(&self) -> String {
        match self {
            Self::Success(num) => num.approximate().to_string(),
            //Self::Imaginary => String::from("Not real"),
            Self::Infinite => String::from("Infinite"),
            Self::Indeterminate => String::from("Indeterminate"),
            Self::Hate => String::from(":( "),
            Self::Heart => String::from("❤ "),
            Self::Invalid => String::from("Format Error"),
            Self::NotFound => String::from("Not found"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Expression<T: Into<f64>> {
    Number(T),
    Parenthesis(Paren),
    Op(Operator),
    // Something,
}

#[derive(Debug)]
pub enum NotationType {
    Infix(Vec<Expression<f64>>),
}
impl fmt::Display for NotationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::from("[");
        let x = self.unwrap();
        for i in x {
            string.push(' ');
            match i {
                Number(num) => string.push_str(&format!("{num}")),
                Op(operator) => string.push(match operator {
                    Add => '+',
                    Subtract => '-',
                    Multiply => '×',
                    Divide => '÷',
                    RaisedTo => '^',
                }),
                Parenthesis(paren) => string.push(match paren {
                    Left => '(',
                    Right => ')',
                }),
                // Something => panic!("bruh"),
            }
            string.push(',');
        }
        string.push(']');
        write!(f, "{}", string)
    }
}

use core::fmt;

use Expression::{Number, Op, Parenthesis};
use Operator::{Add, Divide, Multiply, RaisedTo, Subtract};
use Paren::{Left, Right};

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
            if let Op(_) = expression
                .last()
                .unwrap_or(&Expression::Number(0.0))
            {
                expression.pop();
            } else if let Parenthesis(Left) = expression
                .last()
                .unwrap_or(&Expression::Number(0.0))
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

// trait ChangeNotation {
//     // fn to_infix(&self) -> NotationType::Infix();
//     // fn to_prefix(&self) -> NotationType::Prefix();
//     fn to_postfix(&self) -> NotationType;
// }

impl Operator {
    fn has_higher_precedence(&self, compare_with: &Operator) -> bool {
        let self_score: u8 = match self {
            Add => 0,
            Subtract => 1,
            Multiply => 2,
            Divide => 3,
            RaisedTo => 4,
        };
        let other_score: u8 = match compare_with {
            Add => 0,
            Subtract => 1,
            Multiply => 2,
            Divide => 3,
            RaisedTo => 4,
        };

        self_score > other_score
    }
}

fn solve_binary_expression(left: f64, operator: Operator, right: f64) -> CalculationState {
    match operator {
        Add => CalculationState::Success(left + right),
        Subtract => CalculationState::Success(left - right),
        Multiply => CalculationState::Success(left * right),
        Divide => {
            if right == 0.0 && left == 0.0 {
                CalculationState::Indeterminate
            } else if right == 0.0 {
                CalculationState::Infinite
            } else {
                CalculationState::Success(left / right)
            }
        }
        RaisedTo => {
            if left == 0.0 && right == 0.0 {
                CalculationState::Indeterminate
            } else if right == 0.0 {
                CalculationState::Success(1.0)
            } else {
                CalculationState::Success(left.powf(right))
            }
        }
    }
}

impl NotationType {
    fn unwrap(&self) -> &Vec<Expression<f64>> {
        match self {
            NotationType::Infix(x) => x,
            // NotationType::Postfix(x) => x,
        }
    }

    pub fn solve(&self) -> CalculationState {
        let mut state = CalculationState::Success(0.0);
        let expression_vec = self.unwrap();
        let mut op_list: LinkedList<Expression<f64>> = LinkedList::new();
        let mut p_list: LinkedList<f64> = LinkedList::new();
        let mut num_count: u8 = 0;
        'solving_expression: for i in expression_vec {
            match i {
                Parenthesis(x) => match x {
                    Left => {
                        op_list.push_front(Parenthesis(Left));
                    }

                    Right => loop {
                        let last_pop = op_list.pop_front().unwrap();
                        match last_pop {
                            Parenthesis(Left) => break,
                            Op(operator) => {
                                let right_operand = p_list.pop_front().unwrap_or_else(|| {
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Right operand\x1b[0m");
                                        state = CalculationState::Invalid;
                                        0.0
                                    });
                                let left_operand = p_list.pop_front().unwrap_or_else(||{
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Left operand\x1b[0m");
                                        state = CalculationState::Invalid;
                                        0.0
                                    });

                                if let CalculationState::Invalid = state {
                                    return state;
                                } else {
                                    match solve_binary_expression(
                                        left_operand,
                                        operator,
                                        right_operand,
                                    ) {
                                        CalculationState::Success(num) => {
                                            p_list.push_front(num);
                                            state = CalculationState::Success(num);
                                        }
                                        x => {
                                            state = x;
                                            break 'solving_expression;
                                        }
                                    }
                                }
                            }
                            _ => panic!("Bruh what"),
                        }
                    },
                },
                Op(x) => loop {
                    match op_list.front().unwrap() {
                        Op(op) => {
                            if x.has_higher_precedence(op) {
                                op_list.push_front(Op(*x));
                                break;
                            } else {
                                let right_operand = p_list.pop_front().unwrap_or_else(|| {
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Right operand\x1b[0m");
                                        state = CalculationState::Invalid;
                                        0.0
                                    });
                                let left_operand = p_list.pop_front().unwrap_or_else( ||{
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Right operand\x1b[0m");
                                        state = CalculationState::Invalid;
                                        0.0
                                    });

                                if let CalculationState::Invalid = state {
                                    return state;
                                } else {
                                    match solve_binary_expression(left_operand, *op, right_operand)
                                    {
                                        CalculationState::Success(num) => {
                                            p_list.push_front(num);
                                            state = CalculationState::Success(num);
                                        }
                                        x => {
                                            state = x;
                                            break 'solving_expression;
                                        }
                                    };
                                }

                                op_list.pop_front();
                            }
                        }
                        _ => {
                            op_list.push_front(Op(*x));
                            break;
                        }
                    }
                },
                Number(x) => {
                    p_list.push_front(*x);
                    num_count += 1;
                } // Something => todo!("Bro how in the world did'ya even get here"),
            }
        }

        println!("\x1b[33mp_list has : {} elements \x1b[0m", p_list.len());
        match state {
            CalculationState::Success(_num) => {
                let value = p_list.pop_front().unwrap_or(0.0);
                if num_count == 1 {
                    if value == 143.0 {
                        CalculationState::Heart
                    } else if value == 182.0 {
                        CalculationState::Hate
                    } else if value == 404.0 {
                        CalculationState::NotFound
                    } else {
                        CalculationState::Success(value)
                    }
                } else {
                    CalculationState::Success(value)
                }
            }
            x => x,
        }
    }
}
// impl ChangeNotation for NotationType::Infix {
//     fn to_postfix(&self) -> NotationType::Postfix {
//         let infix_vec = if let NotationType::Infix(x) = self {
//             x
//         };

//         for i in infix_vec {
//             match i
//         }

//         NotationType::Postfix(oreo)
//     }
// }

trait Result {
    fn approximate(&self) -> String;
}

impl Result for f64 {
    fn approximate(&self) -> String {
        if self.to_string().len() >= 12 {
            let string = format!("{:1.9e}", self);
            let e_pos = string.find('e').unwrap();
            format!("{}{}", string[..e_pos].trim_end_matches('0'), &string[e_pos..])
        } else {
            self.to_string()
        }
    }
}

#[cfg(target_os="linux")]
fn copy(str : String) {
    
}