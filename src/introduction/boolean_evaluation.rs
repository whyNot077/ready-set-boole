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

pub fn checked_eval_formula(formula: &str) -> Result<bool> {
    let mut stack = Vec::new();  // 값을 저장할 스택

    for &token in formula.as_bytes() {
        match token {
            b'0' | b'1' => stack.push((token - b'0') != 0),  // 0 또는 1을 스택에 추가
            b'!' => {  // 단항 연산자 처리
                let value = stack.pop().ok_or_else(|| anyhow!("Missing operand for '!' operator"))?;
                stack.push(!value);
            }
            b'&' | b'|' | b'^' | b'>' | b'=' => {  // 이항 연산자 처리
                let b = stack.pop().ok_or_else(|| anyhow!("Missing second operand for operator {token}"))?;
                let a = stack.pop().ok_or_else(|| anyhow!("Missing first operand for operator {token}"))?;
                stack.push(eval(a, b, token));
            }
            _ => return Err(anyhow!("Invalid character in formula: {}", token as char)),  // 잘못된 문자 처리
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())  // 스택에 남은 마지막 값을 반환
    } else {
        Err(anyhow!("Formula evaluation resulted in multiple values on stack"))  // 스택에 값이 여러 개 남은 경우 오류 반환
    }
}

// 포뮬러를 평가하는 함수 (unwrap 사용)
pub fn eval_formula(formula: &str) -> bool {
    checked_eval_formula(formula).unwrap()
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

        assert!(checked_eval_formula("1&").is_err());
    }
}
