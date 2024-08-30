use crate::ex03::ast::{ASTNode, get_ast};
use crate::ex05::negation_normal_form::to_nnf;

/// CNF로 변환하는 함수
pub fn to_cnf(ast: &ASTNode) -> ASTNode {
    match ast {
        // 기본적인 피연산자는 그대로 유지
        ASTNode::Operand(_) => ast.clone(),

        // AND 연산자는 재귀적으로 적용
        ASTNode::Operator('&', left, right) => {
            let left_cnf = to_cnf(left);
            let right_cnf = to_cnf(right);
            ASTNode::Operator('&', Box::new(left_cnf), Box::new(right_cnf))
        }

        // OR 연산자는 분배 법칙 적용
        ASTNode::Operator('|', left, right) => {
            let left_cnf = to_cnf(left);
            let right_cnf = to_cnf(right);

            match (left_cnf, right_cnf) {
                // A | (B & C) => (A | B) & (A | C)
                (ASTNode::Operator('&', ll, lr), rc) => {
                    ASTNode::Operator('&',
                        Box::new(to_cnf(&ASTNode::Operator('|', ll, Box::new(rc.clone())))),
                        Box::new(to_cnf(&ASTNode::Operator('|', lr, Box::new(rc))))
                    )
                }
                // (A & B) | C => (A | C) & (B | C)
                (lc, ASTNode::Operator('&', rl, rr)) => {
                    ASTNode::Operator('&',
                        Box::new(to_cnf(&ASTNode::Operator('|', Box::new(lc.clone()), rl))),
                        Box::new(to_cnf(&ASTNode::Operator('|', Box::new(lc), rr)))
                    )
                }
                // 나머지 경우는 OR 연산자 그대로 유지
                (lc, rc) => ASTNode::Operator('|', Box::new(lc), Box::new(rc)),
            }
        }

        // 나머지 연산자에 대해 재귀적으로 변환
        ASTNode::Operator(op, left, right) => {
            let left_cnf = to_cnf(left);
            let right_cnf = to_cnf(right);
            ASTNode::Operator(*op, Box::new(left_cnf), Box::new(right_cnf))
        }
    }
}

/// 주어진 논리식을 CNF로 변환하는 함수
pub fn conjunctive_normal_form(formula: &str) -> String {
    let ast = get_ast(formula).expect("Failed to parse formula");  // AST를 생성
    let nnf_ast = to_nnf(&ast);  // NNF로 변환
    let cnf_ast = to_cnf(&nnf_ast);  // CNF로 변환
    cnf_to_postfix_string(&cnf_ast)  // 결과를 후위 표기법 문자열로 반환
}

// CNF로 변환된 AST를 후위 표기법 문자열로 변환하는 함수
fn cnf_to_postfix_string(ast: &ASTNode) -> String {
    format!("{}", ast)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_cnf_conversion() {
//         assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
//         assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
//         assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
//         assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
//         assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
//         assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
//         assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
//     }
// }
//     #[test]
//     fn test_basic_nnf() {
//         // 간단한 NNF 변환 테스트
//         assert_eq!(conjunctive_normal_form("AB>"), "A!B|");  // A > B -> !A | B
//         assert_eq!(conjunctive_normal_form("AB="), "AB&A!B!&|");  // A = B -> (A & B) | (!A & !B)
//         assert_eq!(conjunctive_normal_form("A!!!!"), "A");  // 중복된 부정 연산자 처리
//     }

// //     #[test]
// //     fn test_str_conversion() {
// //         // 다양한 논리식의 문자열 변환 테스트
// //         assert_eq!(conjunctive_normal_form("A!"), "A!");  // 단순 부정
// //         assert_eq!(conjunctive_normal_form("AB|"), "AB|");  // 단순 OR
// //         assert_eq!(conjunctive_normal_form("ABC&&"), "AB&C&");  // 다중 AND
// //         assert_eq!(conjunctive_normal_form("AB&C!|"), "A!B&C|");  // 혼합된 연산자
// //     }
// // }
