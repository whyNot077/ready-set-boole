// use std::fmt::Write;
// use anyhow::Result;
// use crate::ex03::boolean_evaluation::checked_eval_formula;

// // Function to generate a truth table for a given formula
// pub fn generate_truth_table(formula: &str) -> Result<String> {
//     let mut output = String::new();

//     // Extract and sort variables (A-Z)
//     let mut vars = formula
//         .bytes()
//         .filter(|token| token.is_ascii_uppercase())
//         .collect::<Vec<_>>();
//     vars.sort_unstable();
//     vars.dedup();

//     // Generate the table header
//     for &v in &vars {
//         write!(output, "| {} ", v as char)?;
//     }
//     writeln!(output, "| = |")?;
//     for _ in &vars {
//         write!(output, "|---")?;
//     }
//     writeln!(output, "|---|")?;

//     // Generate rows for each possible combination of truth values
//     for i in 0..(2_u32.pow(vars.len() as u32)) {
//         // Create a bitmask for variable values
//         let var_values = format!("{:0width$b}", i, width = vars.len()).into_bytes();
//         assert!(var_values.len() == vars.len());

//         // Make a copy of the formula and replace variables with their truth values
//         let mut formula_copy = formula.to_string();
//         unsafe {
//             for c in formula_copy.as_bytes_mut().iter_mut() {
//                 for (var, &val) in vars.iter().zip(var_values.iter()) {
//                     if *c == *var {
//                         *c = val;
//                     }
//                 }
//             }
//         }

//         // Evaluate the modified formula
//         let result = checked_eval_formula(&formula_copy)?;

//         // Write the row for the current combination
//         for &c in &var_values {
//             write!(output, "| {} ", c as char)?;
//         }
//         writeln!(output, "| {} |", result as u32)?;
//     }

//     Ok(output)
// }

// // Function to print the truth table
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
