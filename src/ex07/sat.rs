use crate::ex03::ast::{ASTNode, get_ast};
use crate::ex05::negation_normal_form::nnf;
use crate::ex06::conjunctive_normal_form::cnf;


pub fn sat(formula: &str) -> bool {
    let ast = match get_ast(formula) {
        Ok(ast) => ast,
        Err(_) => return false, // AST 변환 실패 시 false 반환
    };

    let nnf_ast = nnf(&ast);
    let cnf_ast = cnf(&nnf_ast);
    solve_cnf(&cnf_ast)
}

fn solve_cnf(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Operand(_) => true,
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
    use super::*;

    #[test]
    fn test_sat() {
        // 주어진 논리식에 대한 테스트 케이스
        assert_eq!(sat("A"), true);                 // A는 참
        assert_eq!(sat("A!"), true);                // A!는 참
        assert_eq!(sat("AA|"), true);               // A | A는 참
        assert_eq!(sat("AA&"), true);               // A & A는 참
        assert_eq!(sat("AA!&"), false);             // A & !A는 거짓
        assert_eq!(sat("AA^"), false);              // A ^ A는 거짓 (동일한 값의 XOR은 0)
        assert_eq!(sat("AB^"), true);               // A ^ B는 참 (A와 B가 다를 경우)
        assert_eq!(sat("AB="), true);               // A = B는 참 (A와 B가 같을 경우)
        assert_eq!(sat("AA>"), true);               // A -> A는 참 (자명한 진리)
        assert_eq!(sat("AA!>"), true);              // !A -> A는 참 (자명한 진리)
        assert_eq!(sat("ABC||"), true);             // A | B | C는 참 (하나라도 참일 경우)
        assert_eq!(sat("AB&A!B!&&"), false);        // (A & B) & (!A & !B)는 거짓 (모순)
        assert_eq!(sat("ABCDE&&&&"), true);         // A & B & C & D & E는 참 (모두 참일 경우)
        assert_eq!(sat("AAA^^"), true);             // A ^ A ^ A는 참 (세 개 XOR)
        assert_eq!(sat("ABCDE^^^^"), true);         // A ^ B ^ C ^ D ^ E는 참 (홀수 개의 참은 XOR 결과가 참)

        // 추가 테스트 케이스
        assert_eq!(sat("AB|"), true);               // A | B는 참 (하나라도 참일 경우)
    }
}

