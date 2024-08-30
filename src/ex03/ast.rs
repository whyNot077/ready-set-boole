use std::fmt;
use anyhow::Result;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Operand(char),  // 피연산자는 문자로 표현 (e.g., 'A', 'B', '0', '1')
    Operator(char), // 연산자는 문자로 표현 (e.g., '&', '|', '!')
}

// 연산자 우선순위를 반환하는 함수
fn precedence(op: char) -> i32 {
    match op {
        '!' => 5,   // 논리 NOT 연산자
        '&' => 4,   // 논리 AND 연산자
        '|' => 3,   // 논리 OR 연산자
        '^' => 2,   // XOR 연산자
        '>' => 1,   // 시프트 연산자 (혹은 더 큰 의미에서의 비교)
        '=' => 0,   // 대입 연산자
        _ => -1,    // 기타 연산자는 우선순위가 없음
    }
}

// 중위 표기식을 후위 표기식으로 변환하는 Shunting-yard 알고리즘
fn infix_to_postfix(tokens: &[Token]) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(_) => output.push(token.clone()),

            Token::Operator(op) => {
                while let Some(top_op) = operators.last() {
                    if let Token::Operator(top) = top_op {
                        if precedence(*top) >= precedence(*op) {
                            output.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                operators.push(token.clone());
            }
        }
    }

    // 남아 있는 연산자를 출력에 추가
    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

// AST 노드 정의
#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Operand(char),
    Operator(char, Box<ASTNode>, Box<ASTNode>),
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
                    stack.push(ASTNode::Operator(*op, Box::new(operand), Box::new(ASTNode::Operand('\0')))); // 오른쪽 피연산자 없이 트리 구성
                } else {
                    let right = stack.pop()?;
                    let left = stack.pop()?;
                    stack.push(ASTNode::Operator(*op, Box::new(left), Box::new(right)));
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

// fmt::Display 트레이트 구현을 통해 AST를 문자열로 변환
impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Operand(c) => write!(f, "{}", c),
            ASTNode::Operator(op, left, right) => {
                if *op == '!' {
                    // NOT 연산자는 단항이므로 오른쪽 자식이 없음
                    write!(f, "{}{}", op, left)
                } else {
                    write!(f, "{}{}{}", left, op, right)
                }
            }
        }
    }
}

// AST를 문자열로 변환하는 함수
fn ast_to_string(ast: &ASTNode) -> String {
    format!("{}", ast)
}

pub fn get_ast(expression: &str) -> Result<ASTNode> {
    let tokens = tokenize(expression);
    postfix_to_ast(&tokens).ok_or_else(|| anyhow::anyhow!("Failed to generate AST"))
}

// 입력 문자열과 원래 표현식을 비교하는 함수
pub fn check_ast_string(expression: &str) {
    let tokens = tokenize(expression);
    let postfix_tokens: Vec<Token> = infix_to_postfix(&tokens);
    let ast = postfix_to_ast(&postfix_tokens).unwrap();
    let ast_string = ast_to_string(&ast);

    assert_eq!(ast_string, expression);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expressions() {
        check_ast_string("A&B|C");
        check_ast_string("A&0|1");
    }

    #[test]
    fn test_complex_expressions() {
        check_ast_string("A&!B|C");
        check_ast_string("A&B|C");
        check_ast_string("A&!B");
        check_ast_string("A&!B|C");
    }

    #[test]
    fn test_with_parentheses() {
        check_ast_string("A|B&C");
        check_ast_string("A&B|C");
    }
}
