use anyhow::{anyhow, Result};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Node {
    Operand(bool),  // 피연산자 (0 또는 1)
    Variable(char), // 변수 (A, B, ...)
    Operator(char, Rc<RefCell<Node>>, Option<Rc<RefCell<Node>>>),  // 연산자와 자식 노드들 (두 번째 자식은 이항 연산자일 때만 존재)
}

pub fn eval_tree(node: &Rc<RefCell<Node>>, env: &HashMap<char, bool>) -> bool {
    match &*node.borrow() {
        Node::Operand(value) => *value,
        Node::Variable(var) => *env.get(var).expect("Variable not found in environment"),
        Node::Operator(op, left, right) => {
            let a = eval_tree(left, env);
            match op {
                '!' => !a,
                '&' => {
                    let b = eval_tree(right.as_ref().unwrap(), env);
                    a && b
                },
                '|' => {
                    let b = eval_tree(right.as_ref().unwrap(), env);
                    a || b
                },
                '^' => {
                    let b = eval_tree(right.as_ref().unwrap(), env);
                    a ^ b
                },
                '>' => {
                    let b = eval_tree(right.as_ref().unwrap(), env);
                    !a || b
                },
                '=' => {
                    let b = eval_tree(right.as_ref().unwrap(), env);
                    a == b
                },
                _ => unreachable!("Invalid operator: {op}"),
            }
        }
    }
}

pub fn build_tree(formula: &str) -> Result<Rc<RefCell<Node>>> {
    let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();

    for token in formula.chars() {
        match token {
            '0' | '1' => {
                let operand = Node::Operand(token == '1');
                stack.push(Rc::new(RefCell::new(operand)));
            }
            'A'..='Z' => {
                let variable = Node::Variable(token);
                stack.push(Rc::new(RefCell::new(variable)));
            }
            '!' => {
                let operand = stack.pop().ok_or_else(|| anyhow!("Missing operand for '!' operator"))?;
                let operator = Node::Operator('!', Rc::clone(&operand), None);
                stack.push(Rc::new(RefCell::new(operator)));
            }
            '&' | '|' | '^' | '>' | '=' => {
                let right = stack.pop().ok_or_else(|| anyhow!("Missing second operand for operator {token}"))?;
                let left = stack.pop().ok_or_else(|| anyhow!("Missing first operand for operator {token}"))?;
                let operator = Node::Operator(token, Rc::clone(&left), Some(Rc::clone(&right)));
                stack.push(Rc::new(RefCell::new(operator)));
            }
            _ => return Err(anyhow!("Invalid character in formula: {}", token)),
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
    let env = HashMap::new();
    eval_tree(&tree, &env)
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
