use anyhow::{Result, anyhow};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Operand(char),  // 피연산자는 문자로 표현 (e.g., 'A', 'B', '0', '1')
    Operator(char), // 연산자는 문자로 표현 (e.g., '&', '|', '!')
}


// AST Node Definition
#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Operand(char),
    Operator(char, Box<ASTNode>, Option<Box<ASTNode>>),
}

pub fn get_ast(expression: &str) -> Result<ASTNode> {
    let tokens = tokenize(expression);
    postfix_to_ast(&tokens)
}

fn postfix_to_ast(tokens: &[Token]) -> Result<ASTNode> {
    let mut stack: Vec<ASTNode> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(value) => stack.push(ASTNode::Operand(*value)),
            Token::Operator(op) => {
                match *op {
                    '!' => {
                        let operand = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for NOT operand"))?;
                        stack.push(ASTNode::Operator('!', Box::new(operand), None));
                    }
                    '&' | '|' => {
                        let right = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for right operand"))?;
                        let left = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for left operand"))?;
                        stack.push(ASTNode::Operator(*op, Box::new(left), Some(Box::new(right))));
                    }
                    '>' => {
                        // A > B -> !A | B
                        let right = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for right operand"))?;
                        let left = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for left operand"))?;
                        let not_left = ASTNode::Operator('!', Box::new(left), None);
                        stack.push(ASTNode::Operator('|', Box::new(not_left), Some(Box::new(right))));
                    }
                    '=' => {
                        // A = B -> (A & B) | (!A & !B)
                        let right = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for right operand"))?;
                        let left = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for left operand"))?;
                        let left_and_right = ASTNode::Operator('&', Box::new(left.clone()), Some(Box::new(right.clone())));
                        let not_left = ASTNode::Operator('!', Box::new(left), None);
                        let not_right = ASTNode::Operator('!', Box::new(right), None);
                        let not_left_and_not_right = ASTNode::Operator('&', Box::new(not_left), Some(Box::new(not_right)));
                        stack.push(ASTNode::Operator('|', Box::new(left_and_right), Some(Box::new(not_left_and_not_right))));
                    }
                    '^' => {
                        // A ^ B -> (A & !B) | (!A & B)
                        let right = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for right operand"))?;
                        let left = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for left operand"))?;
                        let not_right = ASTNode::Operator('!', Box::new(right.clone()), None);
                        let not_left = ASTNode::Operator('!', Box::new(left.clone()), None);
                        let left_and_not_right = ASTNode::Operator('&', Box::new(left), Some(Box::new(not_right)));
                        let not_left_and_right = ASTNode::Operator('&', Box::new(not_left), Some(Box::new(right)));
                        stack.push(ASTNode::Operator('|', Box::new(left_and_not_right), Some(Box::new(not_left_and_right))));
                    }
                    _ => return Err(anyhow!("Unexpected operator {}", op)),
                }
            }
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(anyhow!("Failed to generate AST from postfix expression"))
    }
}

// Convert the input string to a list of tokens
fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for ch in expression.chars() {
        if ch.is_alphabetic() || ch == '0' || ch == '1' {
            tokens.push(Token::Operand(ch));
        } else if "!&|^>=".contains(ch) {
            tokens.push(Token::Operator(ch));
        } else {
            panic!("Unexpected character in expression: {}", ch);
        }
    }
    tokens
}

// Implement fmt::Display to convert AST to a postfix string
impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Operand(c) => write!(f, "{}", c),
            ASTNode::Operator(op, left, right) => {
                let left_str = format!("{}", left);
                let right_str = right.as_ref().map(|r| format!("{}", r)).unwrap_or_default();
                write!(f, "{}{}{}", left_str, right_str, op)
            }
        }
    }
}

// Converts the AST back into a postfix expression string
pub fn ast_to_postfix_string(ast: &ASTNode) -> String {
    format!("{}", ast)
}

// Converts the AST to an infix string, taking operator precedence into account
pub fn ast_to_infix_string(ast: &ASTNode) -> String {
    match ast {
        ASTNode::Operand(c) => c.to_string(),
        ASTNode::Operator(op, left, right) => {
            let current_precedence = operator_precedence(*op);

            let left_str = match **left {
                ASTNode::Operator(left_op, _, _) if operator_precedence(left_op) < current_precedence => {
                    format!("({})", ast_to_infix_string(left))
                }
                _ => ast_to_infix_string(left),
            };

            let right_str = match right {
                Some(r) => match **r {
                    ASTNode::Operator(right_op, _, _) if operator_precedence(right_op) <= current_precedence => {
                        format!("({})", ast_to_infix_string(r))
                    }
                    _ => ast_to_infix_string(r),
                },
                None => String::new(),
            };

            if right.is_some() {
                format!("{} {} {}", left_str, op, right_str)
            } else {
                format!("{}{}", op, left_str)
            }
        }
    }
}

fn operator_precedence(op: char) -> u8 {
    match op {
        '!' => 3,  // NOT has the highest precedence
        '&' => 2,  // AND
        '|' => 1,  // OR
        '^' => 1,  // XOR
        '>' | '=' => 0, // Implication and equivalence have the lowest precedence
        _ => 0,
    }
}