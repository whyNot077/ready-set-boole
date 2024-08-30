use anyhow::{Result, Context};
use crate::ex03::boolean_evaluation::{eval_formula, apply_operator};

/// 모든 변수 조합에 대해 수식을 평가하여 진리표를 생성하는 함수
fn generate_truth_table(formula: &str) -> Result<String> {
    if eval_formula(formula) {
        return Err(anyhow::anyhow!("Invalid formula"));
    }

    Ok(output)
}


/// 진리표를 출력하는 함수
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
        let res = generate_truth_table("AB&C|").unwrap();
        assert_eq!(res, "| A | B | C | = |\n|---|---|---|---|\n| 0 | 0 | 0 | 0 |\n| 0 | 0 | 1 | 1 |\n| 0 | 1 | 0 | 0 |\n| 0 | 1 | 1 | 1 |\n| 1 | 0 | 0 | 0 |\n| 1 | 0 | 1 | 1 |\n| 1 | 1 | 0 | 1 |\n| 1 | 1 | 1 | 1 |\n");

        assert!(generate_truth_table("AB&C|&").is_err());

        let res = generate_truth_table("A!").unwrap();
        assert_eq!(res, "| A | = |\n|---|---|\n| 0 | 1 |\n| 1 | 0 |\n");
    }
}
