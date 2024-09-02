use std::collections::HashSet;


pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut stack: Vec<HashSet<i32>> = Vec::new();

    for symbol in formula.chars() {
        match symbol {
            'A'..='Z' => {
                let index = (symbol as usize) - ('A' as usize);
                if index >= sets.len() {
                    panic!("Invalid input: more symbols than provided sets");
                }
                let set: HashSet<i32> = sets[index].iter().cloned().collect();
                stack.push(set);
            }
            '!' => {
                let set = stack.pop().expect("Invalid formula: missing operand for '!'");
                let universe: HashSet<i32> = sets.iter().flat_map(|s| s.iter()).cloned().collect();
                let complement: HashSet<i32> = universe.difference(&set).cloned().collect();
                stack.push(complement);
            }
            '&' => {
                let right = stack.pop().expect("Invalid formula: missing operand for '&'");
                let left = stack.pop().expect("Invalid formula: missing operand for '&'");
                let intersection: HashSet<i32> = left.intersection(&right).cloned().collect();
                stack.push(intersection);
            }
            '|' => {
                let right = stack.pop().expect("Invalid formula: missing operand for '|'");
                let left = stack.pop().expect("Invalid formula: missing operand for '|'");
                let union: HashSet<i32> = left.union(&right).cloned().collect();
                stack.push(union);
            }
            '^' => {
                let right = stack.pop().expect("Invalid formula: missing operand for '^'");
                let left = stack.pop().expect("Invalid formula: missing operand for '^'");
                let sym_diff: HashSet<i32> = left.symmetric_difference(&right).cloned().collect();
                stack.push(sym_diff);
            }
            '>' => {
                let right = stack.pop().expect("Invalid formula: missing operand for '>'");
                let left = stack.pop().expect("Invalid formula: missing operand for '>'");
                let implication: HashSet<i32> = left.difference(&right).cloned().collect();
                let union: HashSet<i32> = implication.union(&right).cloned().collect();
                stack.push(union);
            }
            '=' => {
                let right = stack.pop().expect("Invalid formula: missing operand for '='");
                let left = stack.pop().expect("Invalid formula: missing operand for '='");
                let left_imp_right: HashSet<i32> = left.difference(&right).cloned().collect();
                let right_imp_left: HashSet<i32> = right.difference(&left).cloned().collect();
                let eq_set: HashSet<i32> = left_imp_right
                    .union(&right_imp_left)
                    .cloned()
                    .collect();
                stack.push(eq_set);
            }
            _ => panic!("Invalid symbol in formula: {}", symbol),
        }
    }

    // The final result should be a single set on the stack
    let result = stack.pop().expect("Invalid formula: no result");
    if !stack.is_empty() {
        panic!("Invalid formula: extra operands");
    }

    let mut result_vec: Vec<i32> = result.into_iter().collect();
    result_vec.sort(); // Sort the result for consistency
    result_vec
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn eval_set_tests() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];

        assert_eq!(eval_set("AB&", &sets), [0]);

        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];

        let mut result: Vec<i32> = eval_set("AB|", &sets);
        result.sort();
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);

        let sets = vec![vec![0, 1, 2]];

        assert_eq!(eval_set("A!", &sets), []);
    }

    #[test]
    fn eval_set_stress_test() {
        let sets = vec![vec![0], vec![0], vec![0]];

        assert_eq!(eval_set("ABC||", &sets), [0]);
        assert_eq!(eval_set("ABC&&", &sets), [0]);
        assert_eq!(eval_set("ABC^^", &sets), [0]);
        assert_eq!(eval_set("ABC>>", &sets), [0]);

        let sets = vec![vec![0], vec![0], vec![]];

        assert_eq!(eval_set("ABC&&", &sets), []);
    }
}
