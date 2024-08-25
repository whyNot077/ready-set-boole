use anyhow::{anyhow, Result};

fn eval(a: bool, b: bool, op: u8) -> bool {
    match op {
        b'&' => a && b,  // Conjunction ∧
        b'|' => a || b,  // Disjunction ∨
        b'^' => a ^ b,   // Exclusive disjunction ⊕
        b'>' => !a || b, // Material condition ⇒
        b'=' => a == b,  // Logical equivalence ⇔
        _ => unreachable!("Invalid operator: {op}"),
    }
}

pub fn check_eval_formula(formula: &str) -> Result<bool> {
    let mut stack = Vec::new();

    for &token in formula.as_bytes() {
        match token {
            b'0' | b'1' => stack.push((token - b'0') != 0),
            b'!' => {
                let value = stack
                    .pop()
                    .ok_or_else(|| anyhow!("Missing operand for '!' operator"))?;
                stack.push(!value);
            }
            b'&' | b'|' | b'^' | b'>' | b'=' => {
                let b = stack
                    .pop()
                    .ok_or_else(|| anyhow!("Missing second operand for operator {token}"))?;
                let a = stack
                    .pop()
                    .ok_or_else(|| anyhow!("Missing first operand for operator {token}"))?;
                stack.push(eval(a, b, token));
            }
            _ => return Err(anyhow!("Invalid character in formula: {}", token as char)), // 잘못된 문자 처리
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(anyhow!(
            "Formula evaluation resulted in multiple values on stack"
        ))
    }
}

pub fn eval_formula(formula: &str) -> bool {
    check_eval_formula(formula).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_formula() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("101|&"));
        assert!(eval_formula("1011||="));
        assert!(!eval_formula("010&1|&"));
        assert!(!eval_formula("1!"));

        assert!(!eval_formula("10>"));
        assert!(eval_formula("11>"));
        assert!(eval_formula("01>"));

        assert!(eval_formula("11="));
        assert!(!eval_formula("01="));

        assert!(!eval_formula("11^"));
        assert!(eval_formula("10^"));

        assert!(check_eval_formula("1&").is_err());
    }
}
