
use crate::ex03::boolean_evaluation::check_eval_formula;
use anyhow::Result;
use std::collections::HashSet;

pub fn generate_truth_table(formula: &str) -> Result<String, Box<dyn std::error::Error>> {
    let variables: HashSet<char> = formula.chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect();
    let mut var_vec: Vec<char> = variables.into_iter().collect();
    var_vec.sort();
    
    let mut output = String::new();

    print_header(&mut output, &var_vec);
    truth_table(&mut output, formula, &var_vec)?;

    Ok(output)
}

fn print_header(output: &mut String, var_vec: &[char]) {
    for &var in var_vec {
        output.push_str(&format!("| {} ", var));
    }
    output.push_str("| = |\n");
    for _ in var_vec {
        output.push_str("|---");
    }
    output.push_str("|---|\n");
}

fn truth_table(output: &mut String, formula: &str, var_vec: &[char]) -> Result<(), Box<dyn std::error::Error>> {
    let var_count = var_vec.len();
    let row_count = 1 << var_count;  // 2^var_count

    for i in 0..row_count {
        let mut expr = formula.to_string();

        for (j, &var) in var_vec.iter().enumerate() {
            let value = (i & (1 << (var_count - j - 1))) != 0;
            output.push_str(&format!("| {} ", if value { 1 } else { 0 }));

            unsafe {
                for c in expr.as_bytes_mut().iter_mut() {
                    if *c == var as u8 {
                        *c = if value { b'1' } else { b'0' };
                    }
                }
            }
        }

        let result = check_eval_formula(&expr)?;
        output.push_str(&format!("| {} |\n", if result { 1 } else { 0 }));
    }

    Ok(())
}

pub fn print_truth_table(formula: &str) {
    print!("{}", generate_truth_table(formula).unwrap());
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
