use crate::ex03::ast::{ASTNode, get_ast, ast_to_postfix_string};

pub fn negation_normal_form(formula: &str) -> String {
    let mut ast = get_ast(formula).expect("Failed to parse formula");  // AST를 생성
    modify_ast(&mut ast);
    let nnf_ast = nnf(&ast);  // NNF로 변환
    ast_to_postfix_string(&nnf_ast)  // 결과를 중위 표기법 문자열로 반환
}

fn modify_ast(ast: &mut ASTNode) {
    match ast {
        // 임플리케이션 변환 A > B -> !A | B
        ASTNode::Operator('>', _, _) => {
            // In-place로 AST를 수정
            apply_implication(ast);
        }

        // 동치 변환 A = B -> (A & B) | (!A & !B)
        ASTNode::Operator('=', _, _) => {
            apply_equivalence(ast);
        }

        // XOR 변환 A ^ B -> (A & !B) | (!A & B)
        ASTNode::Operator('^', _, _) => {
            apply_xor(ast);
        }

        // AND, OR 등의 다른 연산자는 하위 트리부터 재귀적으로 탐색
        ASTNode::Operator(_, left, Some(right)) => {
            modify_ast(left);
            modify_ast(right);
        }

        // 피연산자는 아무것도 하지 않음 (변경 없음)
        ASTNode::Operand(_) => {}
        
        _ => {},
    }
}


fn apply_de_morgan(left: &ASTNode) -> ASTNode {
    match left {
        ASTNode::Operator('!', inner_operand, _) => nnf(inner_operand), // !!A -> A
        ASTNode::Operator('&', left_inner, Some(right_inner)) => {
            // 드모르간 법칙: !(A & B) => !A | !B
            ASTNode::Operator('|',
                Box::new(nnf(&ASTNode::Operator('!', Box::new(nnf(left_inner)), None))),
                Some(Box::new(nnf(&ASTNode::Operator('!', Box::new(nnf(right_inner)), None)))))
        }
        ASTNode::Operator('|', left_inner, Some(right_inner)) => {
            // 드모르간 법칙: !(A | B) => !A & !B
            println!("left_inner: {:?}", left_inner);
            println!("right_inner: {:?}", right_inner);
            ASTNode::Operator('&',
                Box::new(nnf(&ASTNode::Operator('!', Box::new(nnf(left_inner)), None))),
                Some(Box::new(nnf(&ASTNode::Operator('!', Box::new(nnf(right_inner)), None)))))
        }
        _ => ASTNode::Operator('!', Box::new(nnf(left)), None),
    }
}


fn apply_operator(op: char, left: &ASTNode, right: &ASTNode) -> ASTNode {
    ASTNode::Operator(op, Box::new(nnf(left)), Some(Box::new(nnf(right))))
}

fn apply_implication(ast: &mut ASTNode) {
    if let ASTNode::Operator('>', left, Some(right)) = ast {
        // A > B -> !A | B
        let mut not_left = ASTNode::Operator('!', left.clone(), None);
        modify_ast(&mut not_left);
        
        let mut right = right.clone();
        modify_ast(&mut right);

        *ast = ASTNode::Operator('|', Box::new(not_left), Some(Box::new(*right)));
    }
}

fn apply_equivalence(ast: &mut ASTNode) {
    if let ASTNode::Operator('=', left, Some(right)) = ast {
        // (A & B)
        let mut left_and_right = ASTNode::Operator('&', left.clone(), Some(right.clone()));
        modify_ast(&mut left_and_right);

        // (!A & !B)
        let mut not_left_and_not_right = ASTNode::Operator('&',
            Box::new(ASTNode::Operator('!', left.clone(), None)),
            Some(Box::new(ASTNode::Operator('!', right.clone(), None)))
        );
        modify_ast(&mut not_left_and_not_right);

        *ast = ASTNode::Operator('|', Box::new(left_and_right), Some(Box::new(not_left_and_not_right)));
    }
}

fn apply_xor(ast: &mut ASTNode) {
    if let ASTNode::Operator('^', left, Some(right)) = ast {
        // A ^ B => (A & !B) | (!A & B)
        let mut not_a = ASTNode::Operator('!', left.clone(), None);
        modify_ast(&mut not_a);

        let mut not_b = ASTNode::Operator('!', right.clone(), None);
        modify_ast(&mut not_b);

        let mut left_and_not_b = ASTNode::Operator('&', left.clone(), Some(Box::new(not_b)));
        modify_ast(&mut left_and_not_b);

        let mut not_a_and_right = ASTNode::Operator('&', Box::new(not_a), Some(right.clone()));
        modify_ast(&mut not_a_and_right);

        *ast = ASTNode::Operator('|', Box::new(left_and_not_b), Some(Box::new(not_a_and_right)));
    }
}


pub fn nnf(ast: &ASTNode) -> ASTNode {
    match ast {
        ASTNode::Operand(_) => ast.clone(),
        ASTNode::Operator('!', left, _) => apply_de_morgan(left),
        ASTNode::Operator(op, left, Some(right)) => apply_operator(*op, left, right),
        _ => ast.clone(), // This case handles malformed ASTs where an operator has no right operand
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

        // 추가 테스트 케이스
        assert_eq!(negation_normal_form("A"), "A");         // A는 그대로
        assert_eq!(negation_normal_form("A!"), "A!");       // A!는 그대로
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");  // !(A & B) -> !A | !B
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");  // !(A | B) -> !A & !B
        assert_eq!(negation_normal_form("AB>!"), "A&B!");   // !(A -> B) -> A & !B
        assert_eq!(negation_normal_form("AB=!"), "A!B|A!B!|&"); // !(A = B) -> (!A | B) & (A | !B)
        assert_eq!(negation_normal_form("ABC||"), "A|(B|C)"); // A | (B | C)는 그대로
        assert_eq!(negation_normal_form("ABC||!"), "A!B!&C!&"); // !(A | (B | C)) -> !A & !B & !C
        assert_eq!(negation_normal_form("ABC|&"), "A&(B|C)");  // A & (B | C)는 그대로
        assert_eq!(negation_normal_form("ABC&|"), "A|(B&C)");  // A | (B & C)는 그대로
        assert_eq!(negation_normal_form("ABC&|!"), "A!B!|C!|"); // !(A | (B & C)) -> !A | !B | !C
        assert_eq!(negation_normal_form("ABC^^"), "A^(B^C)");  // XOR 연산은 그대로
        assert_eq!(negation_normal_form("ABC>>"), "A>>(B>>C)"); // 조건부 연산은 그대로

        // 추가적인 테스트 케이스
        // 이중 부정
        assert_eq!(negation_normal_form("A!!"), "A"); // !!A -> A
        assert_eq!(negation_normal_form("A!!!!"), "A"); // !!!!A -> A
        assert_eq!(negation_normal_form("AB!!&"), "AB&"); // !!(A & B) -> A & B

        // XOR 연산자 변환
        assert_eq!(negation_normal_form("AB^"), "AB!&A!B&|"); // A ^ B -> (A & !B) | (!A & B)
        assert_eq!(negation_normal_form("A!B^"), "A!B!&A!!B&|"); // !A ^ B -> (!A & !B) | (A & B)
        assert_eq!(negation_normal_form("AB!^"), "AB!!&A!B!&|"); // A ^ !B -> (A & B) | (!A & !B)
        assert_eq!(negation_normal_form("A!B!^"), "A!B!!&A!!B!&|"); // !A ^ !B -> (!A & B) | (A & !B)

        // 이중 부정
        assert_eq!(negation_normal_form("A!!"), "A"); // !!A -> A
        assert_eq!(negation_normal_form("A!!!!"), "A"); // !!!!A -> A
        assert_eq!(negation_normal_form("AB!!&"), "AB&"); // !!(A & B) -> A & B
        assert_eq!(negation_normal_form("AB!!|"), "AB|"); // !!(A | B) -> A | B

        // 복잡한 논리식
        assert_eq!(negation_normal_form("ABC&&"), "ABC&&"); // (A & B) & C -> A & B & C
        assert_eq!(negation_normal_form("ABC||"), "ABC||"); // (A | B) | C -> A | B | C
        assert_eq!(negation_normal_form("AB&C|"), "AB&C|"); // (A & B) | C -> A & B | C
        assert_eq!(negation_normal_form("AB|C&"), "AB|C&"); // A | (B & C) -> A | B & C
        assert_eq!(negation_normal_form("AB|C!&"), "AB|C!&"); // A | (B & !C) -> A | B & !C
    }
}
