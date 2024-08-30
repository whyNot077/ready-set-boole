use anyhow::Result;
use crate::ex03::ast::{ASTNode, get_ast};

/// NNF로 변환하는 함수
pub fn to_nnf(ast: &ASTNode) -> ASTNode {
    match ast {
        // 기본적인 피연산자는 그대로 유지
        ASTNode::Operand(_) => ast.clone(),
        // 이중 부정 제거: !(!A) => A
        ASTNode::Operator('!', operand, _) => {
            if let ASTNode::Operator('!', inner_operand, _) = &**operand {
                to_nnf(inner_operand)
            } else if let ASTNode::Operator('&', left, right) = &**operand {
                // 드모르간 법칙: !(A & B) => !A | !B
                ASTNode::Operator('|', Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), Box::new(ASTNode::Operand('\0'))))),
                                      Box::new(to_nnf(&ASTNode::Operator('!', right.clone(), Box::new(ASTNode::Operand('\0'))))))
            } else if let ASTNode::Operator('|', left, right) = &**operand {
                // 드모르간 법칙: !(A | B) => !A & !B
                ASTNode::Operator('&', Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), Box::new(ASTNode::Operand('\0'))))),
                                      Box::new(to_nnf(&ASTNode::Operator('!', right.clone(), Box::new(ASTNode::Operand('\0'))))))
            } else {
                ASTNode::Operator('!', Box::new(to_nnf(operand)), Box::new(ASTNode::Operand('\0')))
            }
        }

        // 임플리케이션 변환: A > B => !A | B
        ASTNode::Operator('>', left, right) => {
            ASTNode::Operator('|', Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), Box::new(ASTNode::Operand('\0'))))),
                                   Box::new(to_nnf(right)))
        }

        // 동등 연산자 처리: A = B => (A & B) | (!A & !B)
        ASTNode::Operator('=', left, right) => {
            let left_and_right = ASTNode::Operator('&', Box::new(to_nnf(left)), Box::new(to_nnf(right)));
            let not_left_and_not_right = ASTNode::Operator('&',
                Box::new(to_nnf(&ASTNode::Operator('!', left.clone(), Box::new(ASTNode::Operand('\0'))))),
                Box::new(to_nnf(&ASTNode::Operator('!', right.clone(), Box::new(ASTNode::Operand('\0'))))),
            );
            ASTNode::Operator('|', Box::new(left_and_right), Box::new(not_left_and_not_right))
        }

        // 나머지 연산자에 대해 NNF를 적용하여 재귀적으로 변환
        ASTNode::Operator(op, left, right) => ASTNode::Operator(*op, Box::new(to_nnf(left)), Box::new(to_nnf(right))),
    }
}

/// 수식을 NNF로 변환하는 함수
pub fn negation_normal_form(formula: &str) -> Result<String> {
    let ast = get_ast(formula)?;  // AST를 생성
    let nnf_ast = to_nnf(&ast);   // NNF로 변환
    Ok(nnf_to_postfix_string(&nnf_ast))   // 결과를 문자열로 반환
}

// AST를 문자열로 변환하는 함수 (이미 주어진 코드 활용)
fn nnf_to_postfix_string(ast: &ASTNode) -> String {
    format!("{}", ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nnf_conversion() {
        // !(A & B) -> !A | !B
        assert_eq!(negation_normal_form("AB&!").unwrap(), "!A|!B");

        // !(A | B) -> !A & !B
        assert_eq!(negation_normal_form("AB|!").unwrap(), "!A&!B");

        // !(!(A | B)) -> A | B
        assert_eq!(negation_normal_form("AB|!!").unwrap(), "A|B");

        // A & (B | C) -> A & B | A & C
        assert_eq!(negation_normal_form("ABC|&").unwrap(), "A&B|A&C");

        // A | (B & C) -> A | B & A | C
        assert_eq!(negation_normal_form("ABC&|").unwrap(), "A|B&A|C");
        
        // A > B -> !A | B
        assert_eq!(negation_normal_form("AB>").unwrap(), "!A|B");

        // A = B -> (A & B) | (!A & !B)
        assert_eq!(negation_normal_form("AB=").unwrap(), "A&B|!A&!B");

        // ! (A | B & C) -> !A | !B & !C
        assert_eq!(negation_normal_form("AB|C&!").unwrap(), "!A&!B|!C");

        // !(!A | !B & !C) -> !A | !B & !C
        assert_eq!(negation_normal_form("A!B!|C!&").unwrap(), "!A|!B&!C");

        // !!A -> A
        assert_eq!(negation_normal_form("A!!").unwrap(), "A");

        // A > B -> !A | B
        assert_eq!(negation_normal_form("AB>").unwrap(), "!A|B");
    }


    #[test]
    fn test_tough_nnf() {
        // 좀 더 복잡한 NNF 변환 테스트
        assert_eq!(negation_normal_form("A!!!B!!!!!&").unwrap(), "A!B!&");  // 여러 중복된 부정 연산자 처리
        assert_eq!(negation_normal_form("AB>!").unwrap(), "AB!&!|");  // 부정과 임플리케이션의 조합
        assert_eq!(negation_normal_form("AB&!C|D!&!").unwrap(), "A!B!&C!|D!!&!|");  // 복잡한 조합
    }

}
