// use std::collections::HashMap;
// use anyhow::{Result, Context};
// use crate::ex03::ast::{get_ast, ASTNode};
// use crate::ex03::boolean_evaluation::apply_operator;
// use std::collections::HashSet;

// fn eval_formula(formula: &str, var_map: &HashMap<char, bool>) -> Result<bool> {
//     let ast = get_ast(formula).context("Failed to create AST from formula")?;
//     Ok(evaluate_ast(&ast, var_map))
// }

// fn evaluate_ast(node: &ASTNode, var_map: &HashMap<char, bool>) -> bool {
//     match node {
//         ASTNode::Operand(c) => {
//             match c {
//                 '0' => false,  // '0'은 false를 나타냄
//                 '1' => true,   // '1'은 true를 나타냄
//                 _ if c.is_alphabetic() => *var_map.get(c).unwrap_or_else(|| panic!("Variable {} not found in map", c)),
//                 _ => panic!("Unexpected operand: {}", c),  // 예상하지 못한 피연산자는 패닉 발생
//             }
//         }
//         ASTNode::Operator('!', left, _) => {
//             let val = evaluate_ast(left, var_map);
//             !val
//         }
//         ASTNode::Operator(op, left, right) => {
//             let left_val = evaluate_ast(left, var_map);
//             let right_val = evaluate_ast(right, var_map);
//             apply_operator(*op, left_val, right_val)
//         }
//     }
// }

// /// 수식에서 변수를 추출하고 알파벳 순으로 정렬하는 함수
// fn extract_and_sort_vars(formula: &str) -> Vec<char> {
//     let mut vars = HashSet::new();
//     for token in formula.chars() {
//         if token.is_alphabetic() {
//             vars.insert(token);
//         }
//     }
//     let mut vars: Vec<char> = vars.into_iter().collect();
//     vars.sort();
//     vars
// }

// /// 진리표의 헤더와 구분선을 생성하는 함수
// fn create_truth_table_header(vars: &[char]) -> String {
//     let mut header = String::new();
//     header.push('|');
//     for &var in vars {
//         header.push_str(&format!(" {} |", var));
//     }
//     header.push_str(" = |\n|");
//     header.push_str(&format!("{}\n", "---|".repeat(vars.len() + 1)));
//     header
// }

// /// 변수 조합에 따라 수식을 평가하고 결과를 출력하는 함수
// fn evaluate_combinations(formula: &str, vars: &[char]) -> Result<String> {
//     let mut output = String::new();
//     let num_vars = vars.len();

//     for i in 0..(1 << num_vars) {
//         output.push('|');
//         let mut eval_map = HashMap::new();
//         for (j, &var) in vars.iter().enumerate() {
//             let value = (i >> (num_vars - j - 1)) & 1 == 1;
//             eval_map.insert(var, value);
//             output.push_str(&format!(" {} |", if value { 1 } else { 0 }));
//         }
//         let result = eval_formula(formula, &eval_map)?;
//         output.push_str(&format!(" {} |\n", if result { 1 } else { 0 }));
//     }

//     Ok(output)
// }

// /// 모든 변수 조합에 대해 수식을 평가하여 진리표를 생성하는 함수
// fn generate_truth_table(formula: &str) -> Result<String> {
//     let vars = extract_and_sort_vars(formula);
//     let mut output = create_truth_table_header(&vars);
//     output.push_str(&evaluate_combinations(formula, &vars)?);
//     Ok(output)
// }

// /// 진리표를 출력하는 함수
// pub fn print_truth_table(formula: &str) {
//     match generate_truth_table(formula) {
//         Ok(output) => print!("{}", output),
//         Err(e) => eprintln!("Error generating truth table: {}", e),
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_truth_table() {
//         let res = generate_truth_table("AB&C|").unwrap();
//         assert_eq!(res, "| A | B | C | = |\n|---|---|---|---|\n| 0 | 0 | 0 | 0 |\n| 0 | 0 | 1 | 1 |\n| 0 | 1 | 0 | 0 |\n| 0 | 1 | 1 | 1 |\n| 1 | 0 | 0 | 0 |\n| 1 | 0 | 1 | 1 |\n| 1 | 1 | 0 | 1 |\n| 1 | 1 | 1 | 1 |\n");

//         assert!(generate_truth_table("AB&C|&").is_err());

//         let res = generate_truth_table("A!").unwrap();
//         assert_eq!(res, "| A | = |\n|---|---|\n| 0 | 1 |\n| 1 | 0 |\n");
//     }
// }
