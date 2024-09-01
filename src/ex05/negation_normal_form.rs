use crate::ex03::ast::{ASTNode, get_ast, ast_to_infix_string};

// NNF로 변환하는 함수
pub fn to_nnf(ast: &ASTNode) -> ASTNode {
    match ast {
        ASTNode::Operand(_) => ast.clone(),

        ASTNode::Operator('!', operand, _) => {
            let operand = operand.as_ref();
            match operand {
                ASTNode::Operator('!', inner_operand, _) => to_nnf(inner_operand), // 이중 부정 제거
                ASTNode::Operator('&', left, right) => {
                    // 드모르간 법칙: !(A & B) => !A | !B
                    ASTNode::Operator('|',
                        Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), Box::new(ASTNode::Operand('\0'))))),
                        Box::new(to_nnf(&ASTNode::Operator('!', right.clone(), Box::new(ASTNode::Operand('\0'))))))
                }
                ASTNode::Operator('|', left, right) => {
                    // 드모르간 법칙: !(A | B) => !A & !B
                    ASTNode::Operator('&',
                        Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), Box::new(ASTNode::Operand('\0'))))),
                        Box::new(to_nnf(&ASTNode::Operator('!', right.clone(), Box::new(ASTNode::Operand('\0'))))))
                }
                _ => ASTNode::Operator('!', Box::new(to_nnf(operand)), Box::new(ASTNode::Operand('\0'))),
            }
        }

        ASTNode::Operator('>', left, right) => {
            // 임플리케이션 변환: A > B => !A | B
            ASTNode::Operator('|',
                Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), Box::new(ASTNode::Operand('\0'))))),
                Box::new(to_nnf(right))
            )
        }

        ASTNode::Operator('=', left, right) => {
            // 동등 연산자 처리: A = B => (A & B) | (!A & !B)
            let left_and_right = ASTNode::Operator('&', Box::new(to_nnf(left)), Box::new(to_nnf(right)));
            let not_left_and_not_right = ASTNode::Operator('&',
                Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), Box::new(ASTNode::Operand('\0'))))),
                Box::new(to_nnf(&ASTNode::Operator('!', right.clone(), Box::new(ASTNode::Operand('\0')))))
            );
            ASTNode::Operator('|', Box::new(left_and_right), Box::new(not_left_and_not_right))
        }

        ASTNode::Operator(op, left, right) => {
            // 추가적인 연산자 우선순위 처리 필요시 여기에 로직 추가
            ASTNode::Operator(*op, Box::new(to_nnf(left)), Box::new(to_nnf(right)))
        }
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let ast = get_ast(formula).expect("Failed to parse formula");  // AST를 생성
    let nnf_ast = to_nnf(&ast);  // NNF로 변환
    ast_to_infix_string(&nnf_ast)  // 결과를 중위 표기법 문자열로 반환
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
