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
    let mut flattened_ors = vec![];

    // OR 연산자들을 플랫하게 만든다
    flatten_or(&cnf(left), &mut flattened_ors);
    flatten_or(&cnf(right), &mut flattened_ors);

    // 플랫한 OR 연산자를 하나의 트리로 재구성
    let mut current_ast = flattened_ors.pop().unwrap();  // 마지막 OR 노드로 시작
    while let Some(next) = flattened_ors.pop() {
        current_ast = ASTNode::Operator('|', Box::new(next), Some(Box::new(current_ast)));  // 트리를 재구성
    }
    current_ast
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


// OR 연산자를 플랫하게 만드는 함수
fn flatten_or(ast: &ASTNode, nodes: &mut Vec<ASTNode>) {
    match ast {
        // 중첩된 OR 연산자를 플랫하게 만든다
        ASTNode::Operator('|', left, Some(right)) => {
            flatten_or(left, nodes);  // 왼쪽 서브트리에서 재귀적으로 플랫
            flatten_or(right, nodes); // 오른쪽 서브트리에서 재귀적으로 플랫
        }
        // OR 연산자가 아닌 노드(피연산자 등)는 그대로 추가
        _ => nodes.push(ast.clone()),
    }
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
    println!("NNF: {}", ast_to_postfix_string(&nnf_ast));
    let cnf_ast = cnf(&nnf_ast);  // CNF로 변환
    println!("CNF: {}", ast_to_postfix_string(&cnf_ast));
    ast_to_postfix_string(&cnf_ast)  // 결과를 후위 표기법 문자열로 반환
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cnf_conversion() {
        // 기본적인 CNF 변환
        assert_eq!(conjunctive_normal_form("A"), "A");               // 단일 변수는 그대로 유지
        assert_eq!(conjunctive_normal_form("A!"), "A!");             // 부정된 단일 변수는 그대로 유지
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");        // !(A & B) -> !A | !B (De Morgan's Law)
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");        // !(A | B) -> !A & !B (De Morgan's Law)
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");       // A | (B & C)는 이미 CNF 형태
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");   // A | B | C | D도 그대로 유지
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");   // A & B & C & D도 CNF 형태
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");  // 부정 포함 복합식 변환
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");  // 또 다른 복합식 변환

        // 추가된 CNF 변환 테스트
        assert_eq!(conjunctive_normal_form("ABC||"), "ABC||");       // 세 항 논리합은 그대로 유지
        assert_eq!(conjunctive_normal_form("ABC||!"), "A!B!C!&&");   // !(A | B | C) -> !A & !B & !C
        assert_eq!(conjunctive_normal_form("ABC|&"), "ABC|&");     // CNF 변환 결과 그대로
        assert_eq!(conjunctive_normal_form("ABC&|"), "ABC&|");     // CNF 변환 결과 그대로
        assert_eq!(conjunctive_normal_form("ABC&|!"), "A!B!|C!|");   // !(A | (B & C)) -> !A | !B | !C
        assert_eq!(conjunctive_normal_form("ABC^^"), "A^(B^C)");     // XOR은 그대로 유지
        assert_eq!(conjunctive_normal_form("ABC>>"), "A>>(B>>C)");   // 조건부 연산은 그대로 유지
    }
}
