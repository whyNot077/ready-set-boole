use anyhow::{anyhow, Result};
use std::rc::Rc;
use std::cell::RefCell;

// 트리의 노드를 정의
#[derive(Debug, Clone)]
pub enum Node {
    Operand(bool), // 피연산자 (0 또는 1)
    Operator(u8, Rc<RefCell<Node>>, Rc<RefCell<Node>>), // 연산자 및 자식 노드들
}

// 트리 구조로부터 논리식을 평가하는 함수
pub fn eval_tree(node: &Rc<RefCell<Node>>) -> bool {
    match &*node.borrow() {
        Node::Operand(value) => *value,
        Node::Operator(op, left, right) => {
            let a = eval_tree(left);
            let b = eval_tree(right);
            match op {
                b'&' => a && b,  // Conjunction ∧
                b'|' => a || b,  // Disjunction ∨
                b'^' => a ^ b,   // Exclusive disjunction ⊕
                b'>' => !a || b, // Material condition ⇒
                b'=' => a == b,  // Logical equivalence ⇔
                _ => unreachable!("Invalid operator: {op}"),
            }
        }
    }
}

// 후위 표기법에서 AST 트리 생성
pub fn build_tree(formula: &str) -> Result<Rc<RefCell<Node>>> {
    let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();

    for &token in formula.as_bytes() {
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

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err(anyhow!("Formula evaluation resulted in multiple values on stack"))
    }
}

// AST 트리에서 논리식을 평가
pub fn eval_formula(formula: &str) -> bool {
    let tree = build_tree(formula).unwrap();
    eval_tree(&tree)
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

        assert!(build_tree("1&").is_err());
    }
}
