use anyhow::{Result, Context};
use super::ast::{get_ast, ASTNode};

pub fn eval_formula(formula: &str) -> bool {
    check_eval_formula(formula).unwrap()
}

/// 수식을 평가하는 함수
fn check_eval_formula(formula: &str) -> Result<bool> {
    let ast = get_ast(formula).context("Failed to create AST from formula")?;
    Ok(evaluate_ast(&ast))
}

pub fn evaluate_ast(node: &ASTNode) -> bool {
    match node {
        ASTNode::Operand(c) => match c {
            '0' => false,  // '0'은 false를 나타냄
            '1' => true,   // '1'은 true를 나타냄
            _ => panic!("Unexpected operand: {}", c),  // 예상하지 못한 피연산자일 경우 패닉 발생
        },
        ASTNode::Operator('!', left, _) => {
            // 논리 NOT 연산자는 단항 연산자이므로 왼쪽만 평가
            let val = evaluate_ast(left);
            !val
        }
        ASTNode::Operator(op, left, right_opt) => {
            let left_val = evaluate_ast(left);

            // `right_opt`이 `Some`일 경우에는 `right`를 평가하고, 그렇지 않으면 오류 발생
            let right_val = match right_opt {
                Some(right) => evaluate_ast(right), // `Some`일 때 `right`를 평가
                None => panic!("Missing right operand for operator '{}'", op), // `None`일 때 패닉 발생
            };

            calculate(*op, left_val, right_val)
        }
    }
}

/// 주어진 연산자와 두 피연산자 값을 사용해 논리 연산을 수행하는 함수
pub fn calculate(op: char, left_val: bool, right_val: bool) -> bool {
    match op {
        '&' => left_val && right_val, // AND 연산자
        '|' => left_val || right_val, // OR 연산자
        '^' => left_val ^ right_val,  // XOR 연산자
        '!' => !left_val,             // NOT 연산자 (단항 연산자)
        '>' => !left_val || right_val, // IMPL 연산자 (논리적 함의) : A->B = !A | B
        '=' => left_val == right_val, // EQV 연산자
        _ => panic!("Unexpected operator: {}", op),  // 예상하지 못한 연산자는 패닉을 발생시킴
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_formula() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }
}
