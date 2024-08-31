use crate::ex03::ast::{ASTNode, get_ast, ast_to_postfix_string};
use crate::ex05::negation_normal_form::to_nnf;

/// CNF로 변환하는 함수
pub fn to_cnf(ast: &ASTNode) -> ASTNode {
    match ast {
        // 기본적인 피연산자는 그대로 유지
        ASTNode::Operand(_) => ast.clone(),

        // 분배법칙 적용: `|`에 대해 분배법칙 적용
        ASTNode::Operator('|', left, right) => {
            let left_cnf = to_cnf(left);
            let right_cnf = to_cnf(right);

            match (&left_cnf, &right_cnf) {
                // A | (B & C) => (A | B) & (A | C)
                (ASTNode::Operand(_), ASTNode::Operator('&', rl, rr)) => {
                    ASTNode::Operator('&',
                        Box::new(ASTNode::Operator('|', Box::new(left_cnf.clone()), rl.clone())),
                        Box::new(ASTNode::Operator('|', Box::new(left_cnf), rr.clone())))
                }
                // (A & B) | C => (A | C) & (B | C)
                (ASTNode::Operator('&', ll, lr), ASTNode::Operand(_)) => {
                    ASTNode::Operator('&',
                        Box::new(ASTNode::Operator('|', ll.clone(), Box::new(right_cnf.clone()))),
                        Box::new(ASTNode::Operator('|', lr.clone(), Box::new(right_cnf))))
                }
                // Flatten nested OR chains
                (ASTNode::Operator('|', ll, lr), _) => {
                    ASTNode::Operator('|',
                        Box::new(to_cnf(ll)),
                        Box::new(to_cnf(&ASTNode::Operator('|', lr.clone(), Box::new(right_cnf.clone())))))
                }
                (_, ASTNode::Operator('|', rl, rr)) => {
                    ASTNode::Operator('|',
                        Box::new(to_cnf(&ASTNode::Operator('|', Box::new(left_cnf.clone()), rl.clone()))),
                        Box::new(to_cnf(rr)))
                }
                // 나머지 경우는 OR 연산자 그대로 유지
                _ => ASTNode::Operator('|', Box::new(left_cnf), Box::new(right_cnf)),
            }
        }

        // AND 연산자에 대해 처리
        ASTNode::Operator('&', left, right) => {
            // Flatten nested AND chains: (A & B) & C => A & B & C
            let mut flattened_ands = vec![];
            flatten_and(&to_cnf(left), &mut flattened_ands);
            flatten_and(&to_cnf(right), &mut flattened_ands);

            let mut current_ast = flattened_ands.remove(0);
            for node in flattened_ands {
                current_ast = ASTNode::Operator('&', Box::new(current_ast), Box::new(node));
            }
            current_ast

        }

        // 나머지 연산자에 대해 CNF를 적용하여 재귀적으로 변환
        ASTNode::Operator(op, left, right) => {
            ASTNode::Operator(*op, Box::new(to_cnf(left)), Box::new(to_cnf(right)))
        }
    }
}

/// AND 연산자를 가진 노드들을 플랫하게 만드는 함수
fn flatten_and(ast: &ASTNode, nodes: &mut Vec<ASTNode>) {
    match ast {
        ASTNode::Operator('&', left, right) => {
            flatten_and(left, nodes);
            flatten_and(right, nodes);
        }
        _ => nodes.push(ast.clone()),
    }
}


/// 주어진 논리식을 CNF로 변환하는 함수
pub fn conjunctive_normal_form(formula: &str) -> String {
    let ast = get_ast(formula).expect("Failed to parse formula");  // AST를 생성
    let nnf_ast = to_nnf(&ast);  // NNF로 변환
    let cnf_ast = to_cnf(&nnf_ast);  // CNF로 변환
    ast_to_postfix_string(&cnf_ast)  // 결과를 후위 표기법 문자열로 반환
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cnf_conversion() {
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }

    #[test]
    fn test_str_conversion() {
        // 다양한 논리식의 문자열 변환 테스트
        assert_eq!(conjunctive_normal_form("A!"), "A!");  // 단순 부정
        assert_eq!(conjunctive_normal_form("AB|"), "AB|");  // 단순 OR
        assert_eq!(conjunctive_normal_form("ABC&&"), "AB&C&");  // 다중 AND
        assert_eq!(conjunctive_normal_form("AB&C!|"), "A!B&C|");  // 혼합된 연산자
    }
}

