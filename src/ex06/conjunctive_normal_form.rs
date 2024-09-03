use crate::ex03::ast::{ASTNode, get_ast, ast_to_postfix_string};
use crate::ex05::negation_normal_form::nnf;

// CNF 변환의 메인 함수
pub fn cnf(ast: &ASTNode) -> ASTNode {
    match ast {
        // 기본적인 피연산자는 그대로 유지
        ASTNode::Operand(_) => ast.clone(),

        // OR 연산자에 대한 처리
        ASTNode::Operator('|', left, Some(right)) => cnf_or(left, right),

        // AND 연산자에 대한 처리
        ASTNode::Operator('&', left, right_opt) => cnf_and(left, right_opt),

        // 나머지 연산자에 대해 CNF를 적용하여 재귀적으로 변환
        ASTNode::Operator(op, left, right_opt) => {
            ASTNode::Operator(*op, Box::new(cnf(left)), right_opt.as_ref().map(|r| Box::new(cnf(r))))
        }
    }
}

// OR 연산자에 대한 CNF 처리 함수
fn cnf_or(left: &ASTNode, right: &ASTNode) -> ASTNode {
    let left_cnf = cnf(left);
    let right_cnf = cnf(right);

    match (&left_cnf, &right_cnf) {
        // A | (B & C) => (A | B) & (A | C)
        (ASTNode::Operand(_), ASTNode::Operator('&', rl, rr)) => {
            ASTNode::Operator('&',
                Box::new(ASTNode::Operator('|', Box::new(left_cnf.clone()), Some(rl.clone()))),
                Some(Box::new(ASTNode::Operator('|', Box::new(left_cnf), rr.clone()))))
        }
        // (A & B) | C => (A | C) & (B | C)
        (ASTNode::Operator('&', ll, lr), _) => {
            ASTNode::Operator('&',
                Box::new(ASTNode::Operator('|', ll.clone(), Some(Box::new(right_cnf.clone())))),
                Some(Box::new(ASTNode::Operator('|', lr.clone().unwrap(), Some(Box::new(right_cnf))))))
        }
        // Flatten nested OR chains
        (ASTNode::Operator('|', ll, lr), _) => {
            ASTNode::Operator('|',
                Box::new(cnf(ll)),
                Some(Box::new(cnf(&ASTNode::Operator('|', lr.clone().unwrap(), Some(Box::new(right_cnf)))))))
        }
        (_, ASTNode::Operator('|', rl, rr)) => {
            ASTNode::Operator('|',
                Box::new(cnf(&ASTNode::Operator('|', Box::new(left_cnf.clone()), Some(rl.clone())))),
                Some(Box::new(cnf(rr.as_deref().unwrap()))))
        }
        // 나머지 경우는 OR 연산자 그대로 유지
        _ => ASTNode::Operator('|', Box::new(left_cnf), Some(Box::new(right_cnf))),
    }
}

// AND 연산자에 대한 CNF 처리 함수
fn cnf_and(left: &ASTNode, right_opt: &Option<Box<ASTNode>>) -> ASTNode {
    let mut flattened_ands = vec![];
    flatten_and(&cnf(left), &mut flattened_ands);
    if let Some(right_node) = right_opt {
        flatten_and(&cnf(right_node), &mut flattened_ands);
    }

    let mut current_ast = flattened_ands.pop().unwrap();
    while let Some(next) = flattened_ands.pop() {
        current_ast = ASTNode::Operator('&', Box::new(next), Some(Box::new(current_ast)));
    }
    current_ast
}

// AND 연산자를 가진 노드들을 플랫하게 만드는 함수
fn flatten_and(ast: &ASTNode, nodes: &mut Vec<ASTNode>) {
    match ast {
        ASTNode::Operator('&', left, Some(right)) => {
            flatten_and(left, nodes);
            flatten_and(right, nodes);
        }
        _ => nodes.push(ast.clone()),
    }
}


/// 주어진 논리식을 CNF로 변환하는 함수
pub fn conjunctive_normal_form(formula: &str) -> String {
    let ast = get_ast(formula).expect("Failed to parse formula");  // AST를 생성
    let nnf_ast = nnf(&ast);  // NNF로 변환
    let cnf_ast = cnf(&nnf_ast);  // CNF로 변환
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
}