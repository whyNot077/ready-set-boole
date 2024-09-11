use crate::ex03::ast::{ASTNode, get_ast};
use crate::ex05::negation_normal_form::nnf;
use crate::ex06::conjunctive_normal_form::cnf;
use std::collections::HashSet;

// eval_set 함수: 주어진 CNF 논리식을 집합에 적용하여 결과 반환
pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let ast = match get_ast(formula) {
        Ok(ast) => ast,
        Err(_) => {
            eprintln!("Invalid formula");
            return vec![];
        }
    };

    let nnf_ast = nnf(&ast);
    let cnf_ast = cnf(&nnf_ast);

    // 전체 집합을 구하기 위해 모든 집합의 합집합을 구함
    let universal_set: HashSet<i32> = sets.iter().flat_map(|set| set.iter().cloned()).collect();

    // print the universal set
    println!("Universal set: {:?}", universal_set);

    // 집합 연산을 수행하는 함수를 호출하여 결과 반환
    eval_cnf_set(&cnf_ast, sets, &universal_set)
}

fn eval_cnf_set(ast: &ASTNode, sets: &Vec<Vec<i32>>, universal_set: &HashSet<i32>) -> Vec<i32> {
    let result = match ast {
        ASTNode::Operand(var) => {
            // 'A', 'B', 'C' 같은 변수는 sets의 인덱스로 해석됨
            let idx = (*var as usize) - ('A' as usize);
            if idx < sets.len() {
                sets[idx].clone()
            } else {
                vec![]
            }
        }
        ASTNode::Operator('&', left, Some(right)) => {
            let left_set = eval_cnf_set(left, sets, universal_set);
            let right_set = eval_cnf_set(right, sets, universal_set);
            intersection(&left_set, &right_set)
        }
        ASTNode::Operator('|', left, Some(right)) => {
            let left_set = eval_cnf_set(left, sets, universal_set);
            let right_set = eval_cnf_set(right, sets, universal_set);
            union(&left_set, &right_set)
        }
        ASTNode::Operator('!', operand, None) => {
            let set = eval_cnf_set(operand, sets, universal_set);
            complement(&set, universal_set)
        }
        _ => vec![],
    };
    
    let mut sorted_result = result.clone(); // 결과 복사
    sorted_result.sort();  // 결과 집합을 정렬
    sorted_result
}


// 집합의 교집합
fn intersection(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    let set1: HashSet<_> = set1.iter().collect();
    let set2: HashSet<_> = set2.iter().collect();
    set1.intersection(&set2).cloned().copied().collect()
}

// 집합의 합집합
fn union(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    let set1: HashSet<_> = set1.iter().collect();
    let set2: HashSet<_> = set2.iter().collect();
    set1.union(&set2).cloned().copied().collect()
}

// 집합의 보수 (차집합)
fn complement(set: &Vec<i32>, universal_set: &HashSet<i32>) -> Vec<i32> {
    let set: HashSet<_> = set.iter().copied().collect();  // 참조에서 값을 복사하여 HashSet<i32> 생성
    universal_set.difference(&set).cloned().collect()     // 차집합 계산
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_set_tests() {
        // 'A' with '[[]]' -> '[]'
        let sets = vec![vec![]];
        assert_eq!(eval_set("A", &sets), vec![]);

        // 'A!' with '[[]]' -> '[]'
        assert_eq!(eval_set("A!", &sets), vec![]);

        // 'A' with '[[42]]' -> '[42]'
        let sets = vec![vec![42]];
        assert_eq!(eval_set("A", &sets), vec![42]);

        // 'A!' with '[[42]]' -> '[]'
        assert_eq!(eval_set("A!", &sets), vec![]);

        // 'A!B&' with '[[1, 2, 3], [2, 3, 4]]' -> '[4]'
        let sets = vec![vec![1, 2, 3], vec![2, 3, 4]];
        assert_eq!(eval_set("A!B&", &sets), vec![4]);

        // 'AB|' with '[[0, 1, 2], []]' -> '[0, 1, 2]'
        let sets = vec![vec![0, 1, 2], vec![]];
        assert_eq!(eval_set("AB|", &sets), vec![0, 1, 2]);

        // 'AB&' with '[[0, 1, 2], []]' -> '[]'
        assert_eq!(eval_set("AB&", &sets), vec![]);

        // 'AB&' with '[[0, 1, 2], [0]]' -> '[0]'
        let sets = vec![vec![0, 1, 2], vec![0]];
        assert_eq!(eval_set("AB&", &sets), vec![0]);

        // 'AB&' with '[[0, 1, 2], [42]]' -> '[]'
        let sets = vec![vec![0, 1, 2], vec![42]];
        assert_eq!(eval_set("AB&", &sets), vec![]);

        // 'AB^' with '[[0, 1, 2], [0]]' -> '[1, 2]'
        let sets = vec![vec![0, 1, 2], vec![0]];
        assert_eq!(eval_set("AB^", &sets), vec![1, 2]);

        // 'AB>' with '[[0], [1, 2]]' -> '[1, 2]'
        let sets = vec![vec![0], vec![1, 2]];
        assert_eq!(eval_set("AB>", &sets), vec![1, 2]);

        // 'AB>' with '[[0], [0, 1, 2]]' -> '[0, 1, 2]'
        let sets = vec![vec![0], vec![0, 1, 2]];
        assert_eq!(eval_set("AB>", &sets), vec![0, 1, 2]);

        // 'ABC||' with '[[], [], []]' -> '[]'
        let sets = vec![vec![], vec![], vec![]];
        assert_eq!(eval_set("ABC||", &sets), vec![]);

        // 'ABC||' with '[[0], [1], [2]]' -> '[0, 1, 2]'
        let sets = vec![vec![0], vec![1], vec![2]];
        assert_eq!(eval_set("ABC||", &sets), vec![0, 1, 2]);

        // 'ABC||' with '[[0], [0], [0]]' -> '[0]'
        let sets = vec![vec![0], vec![0], vec![0]];
        assert_eq!(eval_set("ABC||", &sets), vec![0]);

        // 'ABC&&' with '[[0], [0], []]' -> '[]'
        let sets = vec![vec![0], vec![0], vec![]];
        assert_eq!(eval_set("ABC&&", &sets), vec![]);

        // 'ABC&&' with '[[0], [0], [0]]' -> '[0]'
        let sets = vec![vec![0], vec![0], vec![0]];
        assert_eq!(eval_set("ABC&&", &sets), vec![0]);

        // 'ABC^^' with '[[0], [0], [0]]' -> '[0]'
        assert_eq!(eval_set("ABC^^", &sets), vec![0]);

        // 'ABC>>' with '[[0], [0], [0]]' -> '[0]'
        assert_eq!(eval_set("ABC>>", &sets), vec![0]);
    }

    #[test]
    fn eval_set_stress_test() {
        // 'ABC||' with '[[0], [0], [0]]' -> '[0]'
        let sets = vec![vec![0], vec![0], vec![0]];
        assert_eq!(eval_set("ABC||", &sets), vec![0]);

        // 'ABC&&' with '[[0], [0], [0]]' -> '[0]'
        assert_eq!(eval_set("ABC&&", &sets), vec![0]);

        // 'ABC^^' with '[[0], [0], [0]]' -> '[0]'
        assert_eq!(eval_set("ABC^^", &sets), vec![0]);

        // 'ABC>>' with '[[0], [0], [0]]' -> '[0]'
        assert_eq!(eval_set("ABC>>", &sets), vec![0]);

        // 'ABC&&' with '[[0], [0], []]' -> '[]'
        let sets = vec![vec![0], vec![0], vec![]];
        assert_eq!(eval_set("ABC&&", &sets), vec![]);

        let sets = vec![vec![0], vec![0], vec![]];
        assert_eq!(eval_set("ABC&&", &sets), vec![1]);
    }
}
