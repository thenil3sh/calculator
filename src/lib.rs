use std::{
    collections::LinkedList,
    fmt::{Display, Formatter},
    fmt
};


// Implemented Operators, Parentheses and Expression with their Respected enums
// This makes the further calculations easier to carry
#[derive(Copy, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    RaisedTo,
}

#[derive(Copy, Clone)]
pub enum Paren {
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub enum Expression<T: Into<f64>> {
    Number(T),
    Parenthesis(Paren),
    Op(Operator),
    // Something,
}

// Enum to specify the sort of result an expression generates
pub enum ResultState {
    Success(f64),
    Infinite,
    Indeterminate,
    Invalid,
    //
    Heart,
    Hate,
    NotFound,
    //
}

// This implements to_string() method (which is was I wanted) 
// + allows Result to be used in other cases too
impl Display for ResultState {
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        write!(f, "{}",
            match self {
                Self::Success(num) => num.approximate().to_string(),
                Self::Infinite => String::from("Infinite"),
                Self::Indeterminate => String::from("Indeterminate"),
                Self::Hate => String::from(":( "),
                Self::Heart => String::from("❤ "),
                Self::Invalid => String::from("Format Error"),
                Self::NotFound => String::from("Not found"),
            }
        )
    }
}

// Maybe useless but denotes the type of expression
pub enum NotationType {
    Infix(Vec<Expression<f64>>),
}

// Again, maybe useless but y'can use it to print the infix for debugging purpose
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

// Re Export enum variants for cleaner expression
use Expression::{Number, Op, Parenthesis};
use Operator::{Add, Divide, Multiply, RaisedTo, Subtract};
use Paren::{Left, Right};

// Trait to implement a method to convert a type to String
pub trait Notation {
    fn to_infix(&self) -> NotationType;
}


// Yeilds an Infix Notation out of String
// However, it expects the String to store expression in desirable format
// 
impl Notation for String {
    
    fn to_infix(&self) -> NotationType {
        
        // the actual vector we, want our expression to parse in
        let mut expression: Vec<Expression<f64>> = vec![Parenthesis(Left)];

        // More prerequisities we may need
        let mut unclosed_paren_count: u16 = 1;
        let mut num = String::new();
        let mut last_i = '_';

        // Iterating over base_expression to develop an infix_notation vector
        for i in self.chars() {

            // push numric or floating point character to `num` string
            if i.is_numeric() || i == '.' {
                num.push(i);

            // we're ready to parse a `num` into `f64` if we don't have a numeric character
            } else if last_i.is_numeric() && !num.is_empty() {
                expression.push(Number(num.parse::<f64>().unwrap()));
                num.clear();
            } 

            // pushes unrary minus operator to num string if any expression starts 
            // with the same operator
            else if last_i == '(' && i == '-' {
                num.push(i);
                continue;
            } 
            
            // if we have any other characters, the operators
            if !i.is_numeric() {
                expression.push(match i {
                    '(' => {
                        unclosed_paren_count += 1;
                        Parenthesis(Left)
                    }
                    ')' => {
                        unclosed_paren_count -= 1;
                        Parenthesis(Right)
                    }
                    '+' => Op(Add),
                    '-' => Op(Subtract),
                    '×' => Op(Multiply),
                    '÷' => Op(Divide),
                    '^' => Op(RaisedTo),
                    ' ' => continue,
                    '.' => continue,
                    _ => panic!("No way this can happen"),
                });
            }

            last_i = i;
        }

        // parse and push the `num` string if not done yet
        if !num.is_empty() {
            expression.push(Number(num.parse::<f64>().unwrap()));
            num.clear();
        }


        // Remove unnecessary Operators and Opening Parentheses
        // This helps us to calculate the expressions that arn't completed yet
        // However this change only appears to us, user cannot see it
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
                unclosed_paren_count -= 1;
            } else {
                break;
            }
        }

        // Push closing parentheses to complete the expression
        // Again, this also helps in calculating expressions that arn't completed yet
        while unclosed_paren_count > 0 {
            expression.push(Parenthesis(Right));
            unclosed_paren_count -= 1;
        }

        // And finally, we return our infix expression
        NotationType::Infix(expression)
    }
}

#[cfg(test)]
mod infix_test {
    use super::Notation;

    #[test]
    fn normal_expression(){
        let expression_a = "2+3-4".to_string();
        let expression_b = "8×(5-2".to_string();

        assert_eq!(expression_a.to_infix().to_string(), 
                 "[ (, 2, +, 3, -, 4, ),]".to_string());

        eprintln!("{}", expression_b.to_infix().to_string());
        assert_eq!(expression_b.to_infix().to_string(),
                "[ (, 8, ×, (, 5, -, 2, ), ),]".to_string())
    }
}


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


fn solve_binary_expression(left: f64, operator: Operator, right: f64) -> ResultState {
    match operator {
        Add => ResultState::Success(left + right),
        Subtract => ResultState::Success(left - right),
        Multiply => ResultState::Success(left * right),
        Divide => {
            if right == 0.0 && left == 0.0 {
                ResultState::Indeterminate
            } else if right == 0.0 {
                ResultState::Infinite
            } else {
                ResultState::Success(left / right)
            }
        }
        // Using powers is implemented but, isn't available for ui level yet
        RaisedTo => {
            if left == 0.0 && right == 0.0 {
                ResultState::Indeterminate
            } else if right == 0.0 {
                ResultState::Success(1.0)
            } else {
                ResultState::Success(left.powf(right))
            }
        }
    }
}

// Now this should make sense, why wrapping Vec<Expression> was a thing
impl NotationType {

    fn unwrap(&self) -> &Vec<Expression<f64>> {
        match self {
            NotationType::Infix(x) => x,
        }
    }

    // Solves the infix expression by using Postfix expression method
    pub fn solve(&self) -> ResultState {
        let mut state = ResultState::Success(0.0);
        let expression_vec = self.unwrap();
        let mut op_list: LinkedList<Expression<f64>> = LinkedList::new();
        let mut p_list: LinkedList<f64> = LinkedList::new();

        // This is useless, until you realise why... 
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
                                        state = ResultState::Invalid;
                                        0.0
                                    });
                                let left_operand = p_list.pop_front().unwrap_or_else(||{
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Left operand\x1b[0m");
                                        state = ResultState::Invalid;
                                        0.0
                                    });

                                if let ResultState::Invalid = state {
                                    return state;
                                } else {
                                    match solve_binary_expression(
                                        left_operand,
                                        operator,
                                        right_operand,
                                    ) {
                                        ResultState::Success(num) => {
                                            p_list.push_front(num);
                                            state = ResultState::Success(num);
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
                                        state = ResultState::Invalid;
                                        0.0
                                    });
                                let left_operand = p_list.pop_front().unwrap_or_else( ||{
                                        eprintln!("\x1b[31mCRITICAL ERROR : \x1b[33mSolution list is empty for Right operand\x1b[0m");
                                        state = ResultState::Invalid;
                                        0.0
                                    });

                                if let ResultState::Invalid = state {
                                    return state;
                                } else {
                                    match solve_binary_expression(left_operand, *op, right_operand)
                                    {
                                        ResultState::Success(num) => {
                                            p_list.push_front(num);
                                            state = ResultState::Success(num);
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

        // println!("\x1b[33mp_list has : {} elements \x1b[0m", p_list.len());

        // When we're done with calculation, here, a result state is thrown back to ui
        match state {
            ResultState::Success(_num) => {
                let value = p_list.pop_front().unwrap_or(0.0);
                if num_count == 1 {
                    if value == 143.0 {
                        ResultState::Heart
                    } else if value == 182.0 {
                        ResultState::Hate
                    } else if value == 404.0 {
                        ResultState::NotFound
                    } else {
                        ResultState::Success(value)
                    }
                } else {
                    ResultState::Success(value)
                }
            }
            x => x,
        }
    }
}


// again, maybe a meaningless trait for approximating large numbers
trait Result {
    fn approximate(&self) -> String;
}

impl Result for f64 {
    
    // If a floating point number is too large to display, 
    // The notation is changed to Scientific
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