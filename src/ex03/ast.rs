use std::fmt;
use anyhow::Result;

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

// 후위 표기식에서 AST를 생성하는 함수
fn postfix_to_ast(tokens: &[Token]) -> Option<ASTNode> {
    let mut stack: Vec<ASTNode> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(value) => stack.push(ASTNode::Operand(*value)),
            Token::Operator(op) => {
                if *op == '!' {
                    // 논리 NOT 연산자는 단항 연산자이므로 하나의 피연산자만 필요
                    let operand = stack.pop()?;
                    stack.push(ASTNode::Operator(*op, Box::new(operand), None)); // 오른쪽 피연산자는 None으로 설정
                } else {
                    let right = stack.pop()?;
                    let left = stack.pop()?;
                    stack.push(ASTNode::Operator(*op, Box::new(left), Some(Box::new(right)))); // Some으로 오른쪽 피연산자 설정
                }
            }
        }
    }

    stack.pop()
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

pub fn get_ast_full_string(expression: &str) -> String {
    let ast = get_ast(expression).unwrap();
    format!("{}", ast)
}

pub fn print_ast_full_string(ast: &ASTNode) {
    println!("ast = {}", ast);
}

pub fn get_ast(expression: &str) -> Result<ASTNode> {
    let tokens = tokenize(expression);
    postfix_to_ast(&tokens).ok_or_else(|| anyhow::anyhow!("Failed to generate AST"))
}

// fmt::Display 트레이트 구현을 통해 AST를 문자열로 변환 : infix 표기법
impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Operand(c) => write!(f, "{}", c),
            ASTNode::Operator(op, left, right) => {
                match right {
                    Some(r) => write!(f, "({} {} {})", left, op, r),
                    None => write!(f, "{}{}", op, left),
                }
            }
        }
    }
}

// AST를 infix 문자열로 변환하는 함수
pub fn ast_to_infix_string(ast: &ASTNode) -> String {
    match ast {
        ASTNode::Operand(c) => c.to_string(),
        ASTNode::Operator(op, left, right) => {
            match op {
                '!' => format!("{}{}", op, ast_to_infix_string(left)),
                _ => format!("{}{}{}", ast_to_infix_string(left), op, ast_to_infix_string(right.as_ref().unwrap())),
            }
        }
    }
}

// AST를 postfix 문자열로 변환하는 함수
pub fn ast_to_postfix_string(ast: &ASTNode) -> String {
    match ast {
        ASTNode::Operand(c) => c.to_string(),
        ASTNode::Operator(op, left, right) => {
            match op {
                '!' => format!("{}{}", ast_to_postfix_string(left), op),
                _ => format!("{}{}{}", ast_to_postfix_string(left), ast_to_postfix_string(right.as_ref().unwrap()), op),
            }
        }
    }
}
