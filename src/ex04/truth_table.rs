use crate::ex03::boolean_evaluation::{eval_tree, build_tree};
use anyhow::Result;
use std::collections::HashSet;
use std::collections::HashMap;
use std::error::Error;

// 변수 추출 함수
fn extract_variables(formula: &str) -> Vec<char> {
    let mut variables = HashSet::new();
    for c in formula.chars() {
        if c.is_ascii_uppercase() {
            variables.insert(c);
        }
    }
    let mut var_vec: Vec<char> = variables.into_iter().collect();
    var_vec.sort(); // 순서를 유지하기 위해 정렬
    var_vec
}

// 진리표 생성 함수
pub fn generate_truth_table(formula: &str) -> Result<String, Box<dyn Error>> {
    let variables = extract_variables(formula);
    let num_vars = variables.len();
    let num_rows = 1 << num_vars; // 2^num_vars

    let mut output = String::new();

    // 헤더 출력
    for var in &variables {
        output.push_str(&format!("| {} ", var));
    }
    output.push_str("| = |\n");
    output.push_str(&"|---".repeat(num_vars + 1));
    output.push_str("|\n");

    // 모든 경우의 수에 대해 평가
    for i in 0..num_rows {
        let mut env = HashMap::new();

        // 각 변수에 대해 값을 할당
        for (j, var) in variables.iter().enumerate() {
            let value = (i >> (num_vars - j - 1)) & 1 == 1;
            env.insert(*var, value);
            output.push_str(&format!("| {} ", if value { 1 } else { 0 }));
        }

        // 식을 평가
        let tree = build_tree(formula).map_err(|e| e.to_string())?; // 오류 처리 추가
        let result = eval_tree(&tree, &env);

        output.push_str(&format!("| {} |\n", if result { 1 } else { 0 }));
    }

    Ok(output)
}

// 진리표를 출력하는 함수
pub fn print_truth_table(formula: &str) {
    match generate_truth_table(formula) {
        Ok(output) => print!("{}", output),
        Err(e) => eprintln!("Error generating truth table: {}", e),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truth_table() {
        let res: String = generate_truth_table("AB&C|").unwrap();
        assert_eq!(res, "| A | B | C | = |\n|---|---|---|---|\n| 0 | 0 | 0 | 0 |\n| 0 | 0 | 1 | 1 |\n| 0 | 1 | 0 | 0 |\n| 0 | 1 | 1 | 1 |\n| 1 | 0 | 0 | 0 |\n| 1 | 0 | 1 | 1 |\n| 1 | 1 | 0 | 1 |\n| 1 | 1 | 1 | 1 |\n");
        
        assert!(generate_truth_table("AB&C|&").is_err());
        
        let res = generate_truth_table("A!").unwrap();
        assert_eq!(res, "| A | = |\n|---|---|\n| 0 | 1 |\n| 1 | 0 |\n");
    }
}
