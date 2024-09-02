use crate::ex03::ast::{ASTNode, get_ast, ast_to_postfix_string};

pub fn negation_normal_form(formula: &str) -> String {
    let mut ast = get_ast(formula).expect("Failed to parse formula");  // AST를 생성
    nnf(&mut ast);  // NNF로 변환
    ast_to_postfix_string(&ast)  // 결과를 중위 표기법 문자열로 반환
}

pub fn nnf(ast: &mut ASTNode) {
    match ast {
        ASTNode::Operand(_) => {}, // 그대로 둠

        ASTNode::Operator('!', operand, _) => {
            let inner = operand.as_mut();
            match inner {
                ASTNode::Operator('!', inner_operand, _) => {
                    // !!A => A
                    *ast = nnf(inner_operand);
                }
                ASTNode::Operator('&', left, right_opt) => {
                    // !(A & B) => !A | !B
                    let new_left = ASTNode::Operator('!', left.clone(), None);
                    let new_right = right_opt.as_ref().map(|r| ASTNode::Operator('!', r.clone(), None));
                    *ast = ASTNode::Operator('|', Box::new(new_left), new_right.map(Box::new));
                }
                ASTNode::Operator('|', left, right_opt) => {
                    // !(A | B) => !A & !B
                    let new_left = ASTNode::Operator('!', left.clone(), None);
                    let new_right = right_opt.as_ref().map(|r| ASTNode::Operator('!', r.clone(), None));
                    *ast = ASTNode::Operator('&', Box::new(new_left), new_right.map(Box::new));
                }
                _ => nnf(inner),
            }
        }

        ASTNode::Operator('>', left, right_opt) => {
            let right = right_opt.as_mut().expect("Expected right operand for '>' operator");
            *ast = ASTNode::Operator('|',
                Box::new(ASTNode::Operator('!', Box::new(left.clone()), None)),
                Some(Box::new(nnf(right)))
            );
        }

        ASTNode::Operator('=', left, right_opt) => {
            let right = right_opt.as_mut().expect("Expected right operand for '=' operator");
            *ast = apply_equivalence(left, &Some(Box::new(right.clone())));
        }

        ASTNode::Operator(op, left, right_opt) if *op == '&' || *op == '|' || *op == '^' => {
            nnf(left);
            if let Some(right) = right_opt {
                nnf(right);
            }
        }

        _ => {},
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nnf_conversion_postfix() {
        // 기본적인 NNF 변환
        assert_eq!(negation_normal_form("AB&!"), "A!B!|"); // !(A & B) -> !A | !B
        assert_eq!(negation_normal_form("AB|!"), "A!B!&"); // !(A | B) -> !A & !B
        assert_eq!(negation_normal_form("AB!!|"), "AB|");  // !(!(A | B)) -> A | B
        assert_eq!(negation_normal_form("AB>"), "A!B|"); // A > B -> !A | B
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|"); // A = B -> (A & B) | (!A & !B)
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|"); // !(A | B & C) -> !A | !B & !C
        assert_eq!(negation_normal_form("A!B!|C!&"), "A!B!|C!&"); // !(!A | !B & !C) -> !A | !B & !C
        assert_eq!(negation_normal_form("A!!"), "A"); // !!A -> A
        assert_eq!(negation_normal_form("AB>"), "A!B|"); // A > B -> !A | B
        assert_eq!(negation_normal_form("A!!!"), "A!"); // !!!A -> !A

        // 추가적인 테스트 케이스
        // 이중 부정
        assert_eq!(negation_normal_form("A!!"), "A"); // !!A -> A
        assert_eq!(negation_normal_form("A!!!!"), "A"); // !!!!A -> A
        assert_eq!(negation_normal_form("AB!!&"), "AB&"); // !!(A & B) -> A & B

        // // 복잡한 논리식
        // assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|"); // !(A | B & C) -> !A | !B & !C
        // assert_eq!(negation_normal_form("AB|C&!D|!"), "AB|C&D!&"); // !((A | B) & (C | D)) -> !A | !B & !C | !D
        // assert_eq!(negation_normal_form("AB&CD|!|"), "AB&CD!|!|"); // (A & B) | !(C | D) -> (A & B) | (!C | !D)
        // assert_eq!(negation_normal_form("AB!|CD&"), "A!B|CD&"); // (A | !B) & (C & D) -> !A | !B & C & D
        // assert_eq!(negation_normal_form("AB!|C!D!&|"), "A!B|C!D!&|"); // (A | !B) | (C & !D) -> !A | !B | C | !D

        // // 임플리케이션과 동치 연산자
        // assert_eq!(negation_normal_form("ABC>="), "AB>C="); // (A > B) = C -> (!A | B) = C
        // assert_eq!(negation_normal_form("AB>C="), "A!B|C="); // (A > B) = C -> (!A | B) = C
        // assert_eq!(negation_normal_form("A!!B>!"), "A!B!|"); // !!A > !B -> A > !B
        // assert_eq!(negation_normal_form("AB>C!|!"), "A!B|C!|!"); // !(A > B | !C) -> !A & B | !C

        // XOR 연산자
        // assert_eq!(negation_normal_form("AB^"), "A!B&A!B&|"); // A XOR B -> (A & !B) | (!A & B)
        // assert_eq!(negation_normal_form("A!B^C&"), "A!B&A!C!&|A!B&A&C!&|"); // !((A XOR B) & C) -> ((!A & !B & C) | (!A & B & !C) | (A & !B & !C) | (A & B & C))
    }

    #[test]
    fn negation_normal_form_with_negation() {
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
    }

    #[test]
    fn negation_normal_form_with_material() {
        assert_eq!(negation_normal_form("AB>"), "A!B|");
    }

    #[test]
    fn negation_normal_form_with_complex() {
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }

    #[test]
    fn negation_normal_form_with_exclusive() {
        assert_eq!(negation_normal_form("AB^"), "AB!&A!B&|");
    }

    #[test]
    fn negation_normal_form_unique() {
        assert_eq!(negation_normal_form("A"), "A");
        assert_eq!(negation_normal_form("A!"), "A!");
    }

    #[test]
    fn negation_normal_form_already_valid() {
        assert_eq!(negation_normal_form("AB|C&"), "AB|C&");
        assert_eq!(negation_normal_form("A!B|"), "A!B|");
        assert_eq!(negation_normal_form("AB!&"), "AB!&");
    }
}
