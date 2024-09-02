use crate::ex03::ast::{ASTNode, get_ast};
use crate::ex05::negation_normal_form::nnf;
use crate::ex06::conjunctive_normal_form::cnf;


pub fn sat(formula: &str) -> bool {
    // 1. AST 생성
    let ast = match get_ast(formula) {
        Ok(ast) => ast,
        Err(_) => return false, // AST 변환 실패 시 false 반환
    };

    // 2. NNF로 변환
    let nnf_ast = nnf(&ast);

    // 3. CNF로 변환
    let cnf_ast = cnf(&nnf_ast);

    // 4. SAT 문제 해결 (단순히 CNF에 리터럴이 있는지 확인하는 기본 솔버)
    solve_cnf(&cnf_ast)
}

fn solve_cnf(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Operand(_) => true, // 피연산자(변수)는 만족 가능성 있음
        ASTNode::Operator('&', left, Some(right)) => {
            // AND 연산자는 양쪽 모두 만족 가능해야 함
            solve_cnf(left) && solve_cnf(right)
        }
        ASTNode::Operator('|', left, Some(right)) => {
            // OR 연산자는 한쪽만 만족 가능하면 됨
            solve_cnf(left) || solve_cnf(right)
        }
        ASTNode::Operator('!', operand, None) => {
            // 부정(NOT)은 피연산자가 거짓일 때 만족 가능함
            !solve_cnf(operand)
        }
        _ => false, // 기타 경우는 지원하지 않음
    }
}

#[cfg(test)]

mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn sat_with_or() {
        assert_eq!(sat("AB|"), true);
    }

    #[test]
    fn sat_with_and() {
        assert_eq!(sat("AB&"), true);
    }

    #[test]
    fn test_with_double_letter() {
        assert_eq!(sat("AA!&"), false);
        assert_eq!(sat("AA^"), false);
    }
}
