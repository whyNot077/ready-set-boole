use anyhow::{Result, anyhow};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Operand(char),  // 피연산자는 문자로 표현 (e.g., 'A', 'B', '0', '1')
    Operator(char), // 연산자는 문자로 표현 (e.g., '&', '|', '!')
}

// AST 노드 정의
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
                if *op == '!' {
                    // Handle unary NOT operator
                    let operand = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for NOT operand"))?;
                    stack.push(ASTNode::Operator(*op, Box::new(operand), None));
                } else {
                    // Handle binary operators by ensuring precedence
                    let right = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for right operand"))?;
                    let left = stack.pop().ok_or_else(|| anyhow!("Failed to pop from stack for left operand"))?;
                    
                    // Constructing a new AST node with the operator
                    stack.push(ASTNode::Operator(*op, Box::new(left), Some(Box::new(right))));
                }
            }
        }
    }

    // At the end, there should be exactly one element in the stack, the root of the AST
    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(anyhow!("Failed to generate AST from postfix expression"))
    }
}


// 문자열을 토큰 리스트로 변환하는 함수
fn tokenize(expression: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = expression.chars().collect();

    for &ch in &chars {
        if ch.is_alphabetic() || ch == '0' || ch == '1' {
            tokens.push(Token::Operand(ch));
        } else if "!&|^>=".contains(ch) {
            tokens.push(Token::Operator(ch));
        } else {
            panic!("Unexpected character in expression");
        }
    }

    tokens
}

// fmt::Display 트레이트 구현을 통해 AST를 문자열로 변환 : postfix 표기법
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

fn operator_precedence(op: char) -> u8 {
    match op {
        '!' => 3,  // 논리 부정 (NOT) 연산자
        '&' => 2,  // 논리 AND 연산자
        '|' => 1,  // 논리 OR 연산자
        '^' => 1,  // 논리 XOR 연산자
        '>' | '=' => 0, // 임플리케이션과 동치 연산자는 낮은 우선순위
        _ => 0,
    }
}

// AST를 infix 문자열로 변환하는 함수
pub fn ast_to_infix_string(ast: &ASTNode) -> String {
    match ast {
        ASTNode::Operand(c) => c.to_string(),
        ASTNode::Operator(op, left, right) => {
            let current_precedence = operator_precedence(*op);

            let left_str = match **left {
                ASTNode::Operator(left_op, _, _) if operator_precedence(left_op) < current_precedence => {
                    format!("{}", ast_to_infix_string(left))
                }
                _ => ast_to_infix_string(left),
            };

            let right_str = match right {
                Some(r) => match **r {
                    ASTNode::Operator(right_op, _, _) if operator_precedence(right_op) <= current_precedence => {
                        format!("{}", ast_to_infix_string(r))
                    }
                    _ => ast_to_infix_string(r),
                },
                None => String::new(),
            };

            if right.is_some() {
                format!("{}{}{}", left_str, op, right_str)
            } else {
                format!("{}{}", op, left_str)
            }
        }
    }
}

// AST를 postfix 문자열로 변환하는 함수
pub fn ast_to_postfix_string(ast: &ASTNode) -> String {
    format!("{}", ast)
}
