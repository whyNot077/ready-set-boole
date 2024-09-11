use crate::ex03::ast::{ASTNode, get_ast};
use crate::ex05::negation_normal_form::nnf;
use crate::ex06::conjunctive_normal_form::cnf;
use crate::ex04::truth_table::extract_variables;
use std::collections::{HashMap, HashSet};

// SAT 문제를 해결하는 함수
pub fn sat(formula: &str) -> bool {
    let ast = match get_ast(formula) {
        Ok(ast) => ast,
        Err(_) => {
            eprintln!("Invalid formula");
            return false;
        }
    };

    let nnf_ast = nnf(&ast);
    let cnf_ast = cnf(&nnf_ast);

    // 변수 추출
    let mut variables = HashSet::new();
    extract_variables(&cnf_ast, &mut variables);
    
    // 가능한 모든 변수 할당 조합을 테스트
    let var_list: Vec<char> = variables.into_iter().collect();
    let num_combinations = 1 << var_list.len();  // 2^n 조합

    for i in 0..num_combinations {
        let mut assignments = HashMap::new();
        
        // i 값에 따라 변수들의 참/거짓 값을 설정
        for (j, &var) in var_list.iter().enumerate() {
            let value = (i & (1 << j)) != 0;
            assignments.insert(var, value);
        }

        // 할당된 변수들로 CNF 평가
        if solve_cnf(&cnf_ast, &assignments) {
            return true; // 참인 조합을 찾으면 true 반환
        }
    }

    false // 모든 조합을 시도해도 참인 조합이 없으면 false 반환
}

// CNF 논리식을 주어진 변수 할당에 따라 평가하는 함수
fn solve_cnf(ast: &ASTNode, assignments: &HashMap<char, bool>) -> bool {
    match ast {
        ASTNode::Operand(var) => {
            *assignments.get(var).unwrap_or(&false)  // 변수 값 참조, 기본은 false
        }
        ASTNode::Operator('&', left, Some(right)) => {
            solve_cnf(left, assignments) && solve_cnf(right, assignments)
        }
        ASTNode::Operator('|', left, Some(right)) => {
            solve_cnf(left, assignments) || solve_cnf(right, assignments)
        }
        ASTNode::Operator('!', operand, None) => {
            !solve_cnf(operand, assignments)
        }
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sat() {
        // 주어진 논리식에 대한 테스트 케이스
        assert_eq!(sat("A"), true);                 // A는 참
        assert_eq!(sat("A!"), true);                // A!는 참
        assert_eq!(sat("AA|"), true);               // A | A는 참
        assert_eq!(sat("AA&"), true);               // A & A는 참
        assert_eq!(sat("AA!&"), false);             // A & !A는 거짓
        assert_eq!(sat("AA^"), false);              // A ^ A는 거짓 (동일한 값의 XOR은 0)
        assert_eq!(sat("AB^"), true);               // A ^ B는 참 (A와 B가 다를 경우)
        assert_eq!(sat("AB="), true);               // A = B는 참 (A와 B가 같을 경우)
        assert_eq!(sat("AA>"), true);               // A -> A는 참 (자명한 진리)
        assert_eq!(sat("AA!>"), true);              // !A -> A는 참 (자명한 진리)
        assert_eq!(sat("ABC||"), true);             // A | B | C는 참 (하나라도 참일 경우)
        assert_eq!(sat("AB&A!B!&&"), false);        // (A & B) & (!A & !B)는 거짓 (모순)
        assert_eq!(sat("ABCDE&&&&"), true);         // A & B & C & D & E는 참 (모두 참일 경우)
        assert_eq!(sat("AAA^^"), true);             // A ^ A ^ A는 참 (세 개 XOR)
        assert_eq!(sat("ABCDE^^^^"), true);         // A ^ B ^ C ^ D ^ E는 참 (홀수 개의 참은 XOR 결과가 참)

        // 추가 테스트 케이스
        assert_eq!(sat("AB|"), true);               // A | B는 참 (하나라도 참일 경우)
    }
}

