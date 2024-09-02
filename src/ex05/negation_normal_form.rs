use crate::ex03::ast::{ASTNode, get_ast, ast_to_infix_string};

pub fn negation_normal_form(formula: &str) -> String {
    let ast = get_ast(formula).expect("Failed to parse formula");  // AST를 생성
    let nnf_ast = to_nnf(&ast);  // NNF로 변환
    ast_to_infix_string(&nnf_ast)  // 결과를 중위 표기법 문자열로 반환
}

pub fn to_nnf(ast: &ASTNode) -> ASTNode {
    match ast {
        ASTNode::Operand(_) => ast.clone(),

        ASTNode::Operator('!', operand, _) => {
            let operand = operand.as_ref();
            match operand {
                ASTNode::Operator('!', inner_operand, _) => to_nnf(inner_operand), // 이중 부정 제거
                ASTNode::Operator('&', left, right_opt) => {
                    // 드모르간 법칙: !(A & B) => !A | !B
                    let right = right_opt.as_ref().map(|r| &**r).unwrap_or(left);
                    ASTNode::Operator('|',
                        Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), None))),
                        Some(Box::new(to_nnf(&ASTNode::Operator('!', Box::new(right.clone()), None)))))
                }
                ASTNode::Operator('|', left, right_opt) => {
                    // 드모르간 법칙: !(A | B) => !A & !B
                    let right = right_opt.as_ref().map(|r| &**r).unwrap_or(left);
                    ASTNode::Operator('&',
                        Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), None))),
                        Some(Box::new(to_nnf(&ASTNode::Operator('!', Box::new(right.clone()), None)))))
                }
                _ => ASTNode::Operator('!', Box::new(to_nnf(operand)), None),
            }
        }

        ASTNode::Operator('>', left, right_opt) => {
            // 임플리케이션 변환: A > B => !A | B
            let right = right_opt.as_ref().expect("Expected right operand for '>' operator");
            ASTNode::Operator('|',
                Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), None))),
                Some(Box::new(to_nnf(right)))
            )
        }

        ASTNode::Operator('=', left, right_opt) => {
            // 동등 연산자 처리: A = B => (A & B) | (!A & !B)
            let right = right_opt.as_ref().expect("Expected right operand for '=' operator");
            let left_and_right = ASTNode::Operator('&', Box::new(to_nnf(left)), Some(Box::new(to_nnf(right))));
            let not_left_and_not_right = ASTNode::Operator('&',
                Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), None))),
                Some(Box::new(to_nnf(&ASTNode::Operator('!', right.clone(), None))))
            );
            ASTNode::Operator('|', Box::new(left_and_right), Some(Box::new(not_left_and_not_right)))
        }

        ASTNode::Operator(op, left, right_opt) => {
            // 추가적인 연산자 우선순위 처리 필요시 여기에 로직 추가
            let right = right_opt.as_ref().map(|r| &**r).unwrap_or(left);
            ASTNode::Operator(*op, Box::new(to_nnf(left)), Some(Box::new(to_nnf(right))))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nnf_conversion() {
        // 기본적인 NNF 변환
        assert_eq!(negation_normal_form("AB&!"), "!A|!B"); // !(A & B) -> !A | !B
        assert_eq!(negation_normal_form("AB|!"), "!A&!B"); // !(A | B) -> !A & !B
        assert_eq!(negation_normal_form("AB|!!"), "A|B");  // !(!(A | B)) -> A | B
        assert_eq!(negation_normal_form("AB>"), "!A|B"); // A > B -> !A | B
        assert_eq!(negation_normal_form("AB="), "A&B|!A&!B"); // A = B -> (A & B) | (!A & !B)
        assert_eq!(negation_normal_form("AB|C&!"), "!A&!B|!C"); // ! (A | B & C) -> !A | !B & !C
        assert_eq!(negation_normal_form("A!B!|C!&"), "!A|!B&!C"); // !(!A | !B & !C) -> !A | !B & !C
        assert_eq!(negation_normal_form("A!!"), "A"); // !!A -> A
        assert_eq!(negation_normal_form("AB>"), "!A|B"); // A > B -> !A | B
        assert_eq!(negation_normal_form("A!!!"), "!A"); // !!!A -> !A

        // 추가적인 테스트 케이스
        // 이중 부정
        assert_eq!(negation_normal_form("A!!"), "A"); // !!A -> A
        assert_eq!(negation_normal_form("A!!!!"), "A"); // !!!!A -> A
        assert_eq!(negation_normal_form("AB!!&"), "A&B"); // !!(A & B) -> A & B

        // 복잡한 논리식
        assert_eq!(negation_normal_form("AB|C&!D|!"), "!A&!B|!C|D"); // !((A | B) & (!C | D)) -> !A & !B | !C | D
        assert_eq!(negation_normal_form("AB&CD|!|"), "(!C|!D)|(A&B)"); // (A & B) | !(C | D) -> (A & B) | (!C | !D)
        assert_eq!(negation_normal_form("AB!|CD&"), "A|!B&C&D"); // (A | !B) & (C & D) -> A | !B & C & D
        assert_eq!(negation_normal_form("AB!|C!D!&|"), "A|!B|C|!D"); // (A | !B) | (C & !D) -> A | !B | C | !D

        // 임플리케이션과 동치 연산자
        assert_eq!(negation_normal_form("ABC>="), "A!|B!|C|A!&B!|C!&A&B|!C"); // A > B = C -> (!A | B) & (A | !B)
        assert_eq!(negation_normal_form("AB>C="), "A!|B!|C|A!&B!|C!&A&B|!C"); // (A > B) = C -> (A > B) = C
        assert_eq!(negation_normal_form("A!!B>!"), "A!|!B"); // !!A > !B -> A > !B
        assert_eq!(negation_normal_form("AB>C!|!"), "!A&B|!C"); // !(A > B | !C) -> !A & B | !C

        // XOR 연산자
        assert_eq!(negation_normal_form("AB^"), "(A&!B)|(!A&B)"); // A XOR B -> (A & !B) | (!A & B)
        assert_eq!(negation_normal_form("A!B^C&"), "(!A&!B&C)|(!A&B&!C)|(A&!B&!C)|(A&B&C)"); // !((A XOR B) & C) -> ((!A & !B & C) | (!A & B & !C) | (A & !B & !C) | (A & B & C))
    }
}
