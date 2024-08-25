use crate::ex03::boolean_evaluation::{eval_tree, Node};
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

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
        let mut env = Vec::new();

        for (j, &var) in var_vec.iter().enumerate() {
            let value = (i & (1 << (var_count - j - 1))) != 0;
            output.push_str(&format!("| {} ", if value { 1 } else { 0 }));
            env.push((var, value));
        }

        // 트리를 생성하고 환경에 따라 노드를 평가
        let tree = build_tree_with_env(&formula, &env)?;
        let result = eval_tree(&tree);

        output.push_str(&format!("| {} |\n", if result { 1 } else { 0 }));
    }

    Ok(())
}

// 트리를 생성할 때 환경에 따라 변수 값을 바꿔주는 함수
fn build_tree_with_env(formula: &str, env: &[(char, bool)]) -> Result<Rc<RefCell<Node>>> {
    let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();

    for &token in formula.as_bytes() {
        if token.is_ascii_uppercase() {
            // 환경에서 변수 값 찾기
            let value = env.iter().find(|&&(var, _)| var as u8 == token)
                           .map(|&(_, value)| value)
                           .ok_or_else(|| anyhow!("Variable not found in environment"))?;
            let operand = Node::Operand(value);
            stack.push(Rc::new(RefCell::new(operand)));
        } else {
            match token {
                b'0' | b'1' => {
                    // 피연산자를 노드로 변환하여 스택에 푸시
                    let operand = Node::Operand((token - b'0') != 0);
                    stack.push(Rc::new(RefCell::new(operand)));
                }
                b'!' => {
                    // 단항 연산자 (!)
                    let operand = stack.pop().ok_or_else(|| anyhow!("Missing operand for '!' operator"))?;
                    let negated = Node::Operand(!eval_tree(&operand));
                    stack.push(Rc::new(RefCell::new(negated)));
                }
                b'&' | b'|' | b'^' | b'>' | b'=' => {
                    // 이항 연산자 (&, |, ^, >, =)
                    let right = stack.pop().ok_or_else(|| anyhow!("Missing second operand for operator {token}"))?;
                    let left = stack.pop().ok_or_else(|| anyhow!("Missing first operand for operator {token}"))?;
                    let operator = Node::Operator(token, Rc::clone(&left), Rc::clone(&right));
                    stack.push(Rc::new(RefCell::new(operator)));
                }
                _ => return Err(anyhow!("Invalid character in formula: {}", token as char)), // 잘못된 문자 처리
            }
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(anyhow!("Formula evaluation resulted in multiple values on stack"))
    }
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
