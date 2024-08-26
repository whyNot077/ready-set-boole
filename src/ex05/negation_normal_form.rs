use anyhow::Result;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ex03::boolean_evaluation::{Node, build_tree};

pub fn negation_normal_form(formula: &str) -> Result<String> {
    let tree = build_tree(formula)?;
    let nnf_tree = to_nnf(&tree);
    Ok(tree_to_string(&nnf_tree))
}

fn to_nnf(node: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    match &*node.borrow() {
        Node::Operand(_) | Node::Variable(_) => Rc::clone(node),
        Node::Operator('!', child, _) => {
            match &*child.borrow() {
                Node::Operand(_) | Node::Variable(_) => Rc::clone(node),
                Node::Operator('&', left, Some(right)) => {
                    // Apply De Morgan's law: !(A & B) -> !A | !B
                    let left_nnf = to_nnf(&Rc::new(RefCell::new(Node::Operator('!', Rc::clone(left), None))));
                    let right_nnf = to_nnf(&Rc::new(RefCell::new(Node::Operator('!', Rc::clone(right), None))));
                    Rc::new(RefCell::new(Node::Operator('|', left_nnf, Some(right_nnf))))
                }
                Node::Operator('|', left, Some(right)) => {
                    // Apply De Morgan's law: !(A | B) -> !A & !B
                    let left_nnf = to_nnf(&Rc::new(RefCell::new(Node::Operator('!', Rc::clone(left), None))));
                    let right_nnf = to_nnf(&Rc::new(RefCell::new(Node::Operator('!', Rc::clone(right), None))));
                    Rc::new(RefCell::new(Node::Operator('&', left_nnf, Some(right_nnf))))
                }
                Node::Operator('!', inner_child, _) => {
                    // Double negation: !!A -> A
                    to_nnf(inner_child)
                }
                Node::Operator('>', left, Some(right)) => {
                    // Implication: !(A > B) -> A & !B
                    let left_nnf = to_nnf(left);
                    let right_nnf = to_nnf(&Rc::new(RefCell::new(Node::Operator('!', Rc::clone(right), None))));
                    Rc::new(RefCell::new(Node::Operator('&', left_nnf, Some(right_nnf))))
                }
                Node::Operator('=', left, Some(right)) => {
                    // Equivalence: !(A = B) -> !(A & B) | !(¬A & ¬B)
                    let a_and_b = Rc::new(RefCell::new(Node::Operator('&', Rc::clone(left), Some(Rc::clone(right)))));
                    let not_a = Rc::new(RefCell::new(Node::Operator('!', Rc::clone(left), None)));
                    let not_b = Rc::new(RefCell::new(Node::Operator('!', Rc::clone(right), None)));
                    // Wrap first argument as Rc<RefCell<Node>> directly, without wrapping in Some
                    let not_a_and_not_b = Rc::new(RefCell::new(Node::Operator('&', not_a, Some(not_b))));
                    Rc::new(RefCell::new(Node::Operator('|', to_nnf(&a_and_b), Some(to_nnf(&not_a_and_not_b)))))
                }
                _ => Rc::clone(node),  // Handle other cases
            }
        }
        Node::Operator('>', left, Some(right)) => {
            // Convert implication: A > B -> !A | B
            let left_nnf = to_nnf(&Rc::new(RefCell::new(Node::Operator('!', Rc::clone(left), None))));
            let right_nnf = to_nnf(right);
            Rc::new(RefCell::new(Node::Operator('|', left_nnf, Some(right_nnf))))
        }
        Node::Operator('=', left, Some(right)) => {
            // Convert equivalence: A = B -> (A & B) | (!A & !B)
            let left_and_right = Rc::new(RefCell::new(Node::Operator('&', Rc::clone(left), Some(Rc::clone(right)))));
            let not_left = Rc::new(RefCell::new(Node::Operator('!', Rc::clone(left), None)));
            let not_right = Rc::new(RefCell::new(Node::Operator('!', Rc::clone(right), None)));
            // Wrap first argument as Rc<RefCell<Node>> directly, without wrapping in Some
            let not_left_and_not_right = Rc::new(RefCell::new(Node::Operator('&', not_left, Some(not_right))));
            Rc::new(RefCell::new(Node::Operator('|', to_nnf(&left_and_right), Some(to_nnf(&not_left_and_not_right)))))
        }
        Node::Operator(op, left, Some(right)) => {
            // Apply NNF to children
            let left_nnf = to_nnf(left);
            let right_nnf = to_nnf(right);
            Rc::new(RefCell::new(Node::Operator(*op, left_nnf, Some(right_nnf))))
        }
        _ => Rc::clone(node),  // Handle other cases
    }
}

// Helper function to convert a tree to a string in reverse polish notation
fn tree_to_string(node: &Rc<RefCell<Node>>) -> String {
    match &*node.borrow() {
        Node::Operand(value) => if *value { "1".to_string() } else { "0".to_string() },
        Node::Variable(var) => var.to_string(),
        Node::Operator(op, left, Some(right)) => {
            let left_str = tree_to_string(left);
            let right_str = tree_to_string(right);
            format!("{}{}{}", left_str, right_str, op)
        }
        Node::Operator(op, left, None) => {
            let left_str = tree_to_string(left);
            format!("{}{}", left_str, op)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation_normal_form() {
        assert_eq!(negation_normal_form("AB&!").unwrap(), "A!B!|");
        assert_eq!(negation_normal_form("AB|!").unwrap(), "A!B!&");
        assert_eq!(negation_normal_form("AB>").unwrap(), "A!B|");
        assert_eq!(negation_normal_form("AB=").unwrap(), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB|C&!").unwrap(), "A!B!&C!|");
        assert_eq!(negation_normal_form("A!B!|C!&").unwrap(), "A!B!|C!&");
    }
}
