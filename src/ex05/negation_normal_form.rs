use crate::ex03::ast::{ASTNode, get_ast, ast_to_postfix_string};

pub fn negation_normal_form(formula: &str) -> String {
    let ast = get_ast(formula).expect("Failed to parse formula");  // AST를 생성
    let nnf_ast = nnf(&ast);  // NNF로 변환
    ast_to_postfix_string(&nnf_ast)  // 결과를 중위 표기법 문자열로 반환
}

fn apply_de_morgan(left: &ASTNode) -> ASTNode {
    match left {
        ASTNode::Operator('!', inner_operand, _) => nnf(inner_operand), // !!A -> A
        ASTNode::Operator('&', left_inner, Some(right_inner)) => {
            // 드모르간 법칙: !(A & B) => !A | !B
            ASTNode::Operator('|',
                Box::new(nnf(&ASTNode::Operator('!', left_inner.clone(), None))),
                Some(Box::new(nnf(&ASTNode::Operator('!', right_inner.clone(), None)))))
        }
        ASTNode::Operator('|', left_inner, Some(right_inner)) => {
            // 드모르간 법칙: !(A | B) => !A & !B
            ASTNode::Operator('&',
                Box::new(nnf(&ASTNode::Operator('!', left_inner.clone(), None))),
                Some(Box::new(nnf(&ASTNode::Operator('!', right_inner.clone(), None)))))
        }
        _ => ASTNode::Operator('!', Box::new(nnf(left)), None),
    }
}

fn apply_implication(left: &ASTNode, right: &ASTNode) -> ASTNode {
    // 임플리케이션 변환: A > B => !A | B
    ASTNode::Operator('|',
        Box::new(nnf(&ASTNode::Operator('!', Box::new(left.clone()), None))),
        Some(Box::new(nnf(right)))
    )
}

fn apply_operator(op: char, left: &ASTNode, right: &ASTNode) -> ASTNode {
    ASTNode::Operator(op, Box::new(nnf(left)), Some(Box::new(nnf(right))))
}

fn apply_equivalence(left: &ASTNode, right: &ASTNode) -> ASTNode {
    // (A & B)
    let left_and_right = ASTNode::Operator('&', Box::new(left.clone()), Some(Box::new(right.clone())));

    // (!A & !B)
    let not_left_and_not_right = ASTNode::Operator('&',
        Box::new(ASTNode::Operator('!', Box::new(left.clone()), None)),
        Some(Box::new(ASTNode::Operator('!', Box::new(right.clone()), None)))
    );

    // (A & B) | (!A & !B)
    ASTNode::Operator('|', Box::new(left_and_right), Some(Box::new(not_left_and_not_right)))
}

fn apply_xor(left: &ASTNode, right: &ASTNode) -> ASTNode {    
    // A ^ B => (A & !B) | (!A & B)
    let not_a = ASTNode::Operator('!', Box::new(left.clone()), None);
    let not_b = ASTNode::Operator('!', Box::new(right.clone()), None);

    let left_and_not_b = ASTNode::Operator('&', Box::new(left.clone()), Some(Box::new(not_b)));
    let not_a_and_right = ASTNode::Operator('&', Box::new(not_a), Some(Box::new(right.clone())));

    ASTNode::Operator('|', Box::new(left_and_not_b), Some(Box::new(not_a_and_right)))
}

pub fn nnf(ast: &ASTNode) -> ASTNode {
    match ast {
        ASTNode::Operand(_) => ast.clone(),
        ASTNode::Operator('!', left, _) => apply_de_morgan(left),
        ASTNode::Operator('>', left, Some(right)) => apply_implication(left, right),
        ASTNode::Operator('=', left, Some(right)) => apply_equivalence(left, right),
        ASTNode::Operator('^', left, Some(right)) => apply_xor(left, right),
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
