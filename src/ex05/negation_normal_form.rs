use crate::ex03::ast::{ASTNode, get_ast, ast_to_postfix_string};

pub fn negation_normal_form(formula: &str) -> String {
    let ast = get_ast(formula).expect("Failed to parse formula");
    println!("AST before NNF: {}", ast_to_postfix_string(&ast));
    let nnf_ast = nnf(&ast);
    println!("AST after NNF: {}", ast_to_postfix_string(&nnf_ast));
    ast_to_postfix_string(&nnf_ast)
}

fn apply_de_morgan(left: &ASTNode) -> ASTNode {
    match left {
        // Negating a negation: !!A -> A
        ASTNode::Operator('!', inner, _) => nnf(inner),

        // Negating a conjunction: !(A & B) -> !A | !B
        ASTNode::Operator('&', left_inner, Some(right_inner)) => {
            let neg_left = nnf(&ASTNode::Operator('!', Box::new(nnf(left_inner)), None));
            let neg_right = nnf(&ASTNode::Operator('!', Box::new(nnf(right_inner)), None));
            ASTNode::Operator('|', Box::new(neg_left), Some(Box::new(neg_right)))
        }

        // Negating a disjunction: !(A | B) -> !A & !B
        ASTNode::Operator('|', left_inner, Some(right_inner)) => {
            let neg_left = nnf(&ASTNode::Operator('!', Box::new(nnf(left_inner)), None));
            let neg_right = nnf(&ASTNode::Operator('!', Box::new(nnf(right_inner)), None));
            ASTNode::Operator('&', Box::new(neg_left), Some(Box::new(neg_right)))
        }

        // Negation of a simple operand or an unsupported case
        _ => ASTNode::Operator('!', Box::new(nnf(left)), None),
    }
}

pub fn nnf(ast: &ASTNode) -> ASTNode {
    match ast {
        // Operands are returned as-is
        ASTNode::Operand(_) => ast.clone(),

        // Negation is handled via apply_de_morgan
        ASTNode::Operator('!', left, _) => apply_de_morgan(left),

        // For other operators, we apply NNF recursively to both operands
        ASTNode::Operator(op, left, Some(right)) => {
            let left_nnf = nnf(left);
            let right_nnf = nnf(right);
            ASTNode::Operator(*op, Box::new(left_nnf), Some(Box::new(right_nnf)))
        }

        // Malformed AST or unsupported cases
        _ => ast.clone(),
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
        assert_eq!(negation_normal_form("AB>!"), "AB!&");   // !(A -> B) -> A & !B
        assert_eq!(negation_normal_form("AB=!"), "A!B!|AB|&"); // !(A = B) -> (!A | B) & (A | !B)
        assert_eq!(negation_normal_form("ABC||"), "ABC||");  // A | (B | C)
        assert_eq!(negation_normal_form("ABC||!"), "A!B!C!&&");  // !(A | (B | C)) -> !A & !B & !C
        assert_eq!(negation_normal_form("ABC|&"), "ABC|&");  // A & (B | C)
        assert_eq!(negation_normal_form("ABC&|"), "ABC&|");  // A | (B & C)
        assert_eq!(negation_normal_form("ABC&|!"), "A!B!C!||");  // !(A | (B & C)) -> !A | !B | !C
        assert_eq!(negation_normal_form("ABC^^"), "ABC^^");  // A ^ (B ^ C)
        assert_eq!(negation_normal_form("ABC>>"), "ABC>>");  // A > (B > C)
        
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
