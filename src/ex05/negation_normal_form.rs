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
        // Double negation: !!A -> A
        ASTNode::Operator('!', inner, _) => nnf(inner),
        
        // Negation of conjunction: !(A & B) -> !A | !B
        ASTNode::Operator('&', left_inner, Some(right_inner)) => {
            ASTNode::Operator('|',
                Box::new(nnf(&ASTNode::Operator('!', Box::new(nnf(left_inner)), None))),
                Some(Box::new(nnf(&ASTNode::Operator('!', Box::new(nnf(right_inner)), None)))))
        }
        
        // Negation of disjunction: !(A | B) -> !A & !B
        ASTNode::Operator('|', left_inner, Some(right_inner)) => {
            ASTNode::Operator('&',
                Box::new(nnf(&ASTNode::Operator('!', Box::new(nnf(left_inner)), None))),
                Some(Box::new(nnf(&ASTNode::Operator('!', Box::new(nnf(right_inner)), None)))))
        }

        // This case handles complex structures like !(A | (B & C)) properly
        ASTNode::Operator(op, left_inner, Some(right_inner)) => {
            // Apply De Morgan's law recursively
            let neg_left = ASTNode::Operator('!', Box::new(nnf(left_inner)), None);
            let neg_right = ASTNode::Operator('!', Box::new(nnf(right_inner)), None);
            
            match op {
                '&' => ASTNode::Operator('|', Box::new(neg_left), Some(Box::new(neg_right))),
                '|' => ASTNode::Operator('&', Box::new(neg_left), Some(Box::new(neg_right))),
                _ => ASTNode::Operator('!', Box::new(nnf(left)), None),
            }
        }

        // Negation of a single operand
        _ => ASTNode::Operator('!', Box::new(nnf(left)), None),
    }
}

fn apply_operator(op: char, left: &ASTNode, right: &ASTNode) -> ASTNode {
    ASTNode::Operator(op, Box::new(nnf(left)), Some(Box::new(nnf(right))))
}

pub fn nnf(ast: &ASTNode) -> ASTNode {
    match ast {
        // Operand remains unchanged
        ASTNode::Operand(_) => ast.clone(),
        
        // Negation is handled by De Morgan's laws
        ASTNode::Operator('!', left, _) => apply_de_morgan(left),
        
        // For other binary operators, recursively apply NNF to left and right operands
        ASTNode::Operator(op, left, Some(right)) => apply_operator(*op, left, right),
        
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
