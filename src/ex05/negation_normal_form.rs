use std::collections::VecDeque;

pub fn negation_normal_form(formula: &str) -> String {
    let mut stack: VecDeque<String> = VecDeque::new();
    
    for token in formula.chars() {
        match token {
            '0' | '1' | 'A'..='Z' => stack.push_back(token.to_string()), // 변수는 그대로 스택에 넣음
            '!' => {
                let operand = stack.pop_back().unwrap();
                // 드모르간 법칙 적용
                if operand.len() == 1 {
                    stack.push_back(format!("{}!", operand)); // 단일 변수에 대한 부정
                } else {
                    // 드모르간 법칙을 적용하여 부정을 분배
                    let mut negated = String::new();
                    for ch in operand.chars() {
                        if ch == '&' {
                            negated.push('|');
                        } else if ch == '|' {
                            negated.push('&');
                        } else if ch == '!' {
                            continue; // 이미 부정된 부분은 무시
                        } else {
                            negated.push(ch);
                            negated.push('!');
                        }
                    }
                    stack.push_back(negated);
                }
            }
            '&' | '|' => {
                let right = stack.pop_back().unwrap();
                let left = stack.pop_back().unwrap();
                stack.push_back(format!("{}{}{}", left, right, token));
            }
            '>' => {
                let right = stack.pop_back().unwrap();
                let left = stack.pop_back().unwrap();
                // A > B는 A! | B로 변환
                stack.push_back(format!("{}!{}|", left, right));
            }
            '=' => {
                let right = stack.pop_back().unwrap();
                let left = stack.pop_back().unwrap();
                // A = B는 (A & B) | (!A & !B)로 변환
                let equiv = format!("{}{}&{}!{}!&|", left, right, left, right);
                stack.push_back(equiv);
            }
            _ => unreachable!("Invalid character in formula: {}", token),
        }
    }

    stack.pop_back().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation_normal_form() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
        assert_eq!(negation_normal_form("A!B!|C!&"), "A!B!|C!&");
    }
}
