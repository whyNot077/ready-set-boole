use anyhow::Result;
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

/// 연산자 우선순위를 정의하는 함수
fn operator_precedence(op: char) -> u8 {
    match op {
        '!' => 3,    // 논리 부정 (NOT) 연산자
        '&' => 2,    // 논리 AND 연산자
        '|' => 1,    // 논리 OR 연산자
        '^' => 1,    // 논리 XOR 연산자
        '>' | '=' => 0, // 임플리케이션과 동치 연산자는 낮은 우선순위
        _ => 0,
    }
}
fn postfix_to_ast(tokens: &[Token]) -> Option<ASTNode> {
    let mut stack: Vec<ASTNode> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(value) => stack.push(ASTNode::Operand(*value)),
            Token::Operator(op) => {
                let precedence = operator_precedence(*op);

                // 연산자가 나오면 스택의 상단에 있는 노드들과 우선순위를 비교하여 재정렬
                while let Some(ASTNode::Operator(prev_op, _, _)) = stack.last() {
                    if operator_precedence(*prev_op) < precedence {
                        break;
                    }
                    let right = stack.pop()?;
                    let left = stack.pop()?;
                    stack.push(ASTNode::Operator(*op, Box::new(left), Some(Box::new(right))));
                }

                if *op == '!' {
                    // 논리 NOT 연산자는 단항 연산자이므로 하나의 피연산자만 필요
                    let operand = stack.pop()?;
                    stack.push(ASTNode::Operator(*op, Box::new(operand), None));
                } else {
                    let right = stack.pop()?;
                    let left = stack.pop()?;
                    stack.push(ASTNode::Operator(
                        *op,
                        Box::new(left),
                        Some(Box::new(right)),
                    ));
                }
            }
        }
    }

    // 남아 있는 연산자들을 스택에서 처리
    while stack.len() > 1 {
        let right = stack.pop()?;
        let left = stack.pop()?;

        // 여기서 중요한 점은, 마지막 남은 연산자가 무엇이냐에 따라 제대로 처리되는지 확인하는 것입니다.
        if let ASTNode::Operator(op, _, _) = left {
            stack.push(ASTNode::Operator(op, Box::new(left), Some(Box::new(right))));
        } else {
            // 만약 남아 있는 것이 연산자가 아니라면 다시 스택에 넣습니다.
            stack.push(left);
            stack.push(right);
            break;
        }
    }

    // 최종적으로 스택에 남은 하나의 요소를 반환
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

pub fn get_ast(expression: &str) -> Result<ASTNode> {
    let tokens = tokenize(expression);
    postfix_to_ast(&tokens).ok_or_else(|| anyhow::anyhow!("Failed to generate AST"))
}

// fmt::Display 트레이트 구현을 통해 AST를 문자열로 변환 : infix 표기법
impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Operand(c) => write!(f, "{}", c),
            ASTNode::Operator(op, left, right) => match right {
                Some(r) => write!(f, "({} {} {})", left, op, r),
                None => write!(f, "{}{}", op, left),
            },
        }
    }
}
pub fn ast_full_infix(expression: &str) -> String {
    let ast = get_ast(expression).unwrap();
    ast_full_infix_string(&ast)
}

pub fn ast_full_infix_string(ast: &ASTNode) -> String {
    match ast {
        ASTNode::Operand(c) => c.to_string(),
        ASTNode::Operator(op, left, right) => {
            let left_str = ast_full_infix_string(left);
            match right {
                Some(r) => {
                    let right_str = ast_full_infix_string(r);
                    format!("({} {} {})", left_str, op, right_str)
                }
                None => format!("{}({})", op, left_str), // 단항 연산자 (!)의 경우
            }
        }
    }
}

// AST를 infix 문자열로 변환하는 함수
pub fn ast_to_infix_string(ast: &ASTNode) -> String {
    match ast {
        ASTNode::Operand(c) => c.to_string(),
        ASTNode::Operator(op, left, right) => match op {
            '!' => format!("{}{}", op, ast_to_infix_string(left)),
            _ => format!(
                "{}{}{}",
                ast_to_infix_string(left),
                op,
                ast_to_infix_string(right.as_ref().unwrap())
            ),
        },
    }
}

// AST를 postfix 문자열로 변환하는 함수
pub fn ast_to_postfix_string(ast: &ASTNode) -> String {
    match ast {
        ASTNode::Operand(c) => c.to_string(),
        ASTNode::Operator(op, left, right) => match op {
            '!' => format!("{}{}", ast_to_postfix_string(left), op),
            _ => format!(
                "{}{}{}",
                ast_to_postfix_string(left),
                ast_to_postfix_string(right.as_ref().unwrap()),
                op
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_full_infix_conversion() {
        // 기본적인 ast_full_infix 변환
        assert_eq!(ast_full_infix("AB&!"), "!((A & B))"); // !(A & B)
        assert_eq!(ast_full_infix("AB|!"), "!((A | B))"); // !(A | B)
        assert_eq!(ast_full_infix("AB|!!"), "!(!((A | B)))");  // !(!(A | B))
        assert_eq!(ast_full_infix("AB>"), "(A > B)"); // A > B
        assert_eq!(ast_full_infix("AB="), "(A = B)"); // A = B
        assert_eq!(ast_full_infix("AB|C&!"), "!(A | (B & C))"); // ! (A | (B & C))
        assert_eq!(ast_full_infix("A!B!|C!&"), "(!A | !(B) & !(C)))"); // !(!A | (!B & !C))
        assert_eq!(ast_full_infix("A!!"), "!!A"); // !!A -> !!A 그대로 유지
        assert_eq!(ast_full_infix("AB>"), "!(A > B)"); // A > B
        assert_eq!(ast_full_infix("A!!!"), "!!!A"); // !!!A

        // 추가적인 테스트 케이스
        // 이중 부정
        assert_eq!(ast_full_infix("A!!"), "!!A"); // !!A
        assert_eq!(ast_full_infix("A!!!!"), "!!!!A"); // !!!!A
        assert_eq!(ast_full_infix("AB!!&"), "!!(A & B)"); // !!(A & B)

        // 복잡한 논리식
        assert_eq!(ast_full_infix("AB|C&!D|!"), "!(A | (B & !C) | D)"); // !((A | (B & !C)) | D)
        assert_eq!(ast_full_infix("AB&CD|!|"), "!(A & B | C & D)"); // !(A & B | C & D)
        assert_eq!(ast_full_infix("AB!|CD&"), "(A | !B) & (C & D)"); // (A | !B) & (C & D)
        assert_eq!(ast_full_infix("AB!|C!D!&|"), "(A | !B) | (C & !D)"); // (A | !B) | (C & !D)

        // 임플리케이션과 동치 연산자
        assert_eq!(ast_full_infix("ABC>="), "((!(A > B)) = C)"); // A > B = C -> (!(A > B)) = C
        assert_eq!(ast_full_infix("AB>C="), "((A > B) = C)"); // (A > B) = C -> (A > B) = C
        assert_eq!(ast_full_infix("A!!B>!"), "!!(A > !B)"); // !!A > !B
        assert_eq!(ast_full_infix("AB>C!|!"), "!(A > B) | !C"); // !(A > B | !C) -> !(A > B) | !C

        // XOR 연산자
        assert_eq!(ast_full_infix("AB^"), "(A XOR B)"); // A XOR B
        assert_eq!(ast_full_infix("A!B^C&"), "!(A XOR B) & C"); // !((A XOR B) & C)
    }
}
