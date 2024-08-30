// use crate::ex03::ast::Node;
// use crate::ex05::negation_normal_form::build_tree;
// use anyhow::{Result, anyhow};
// use std::rc::Rc;
// use std::cell::RefCell;

// fn to_cnf_string(node: &Rc<RefCell<Node>>) -> Result<String> {
//     match &*node.borrow() {
//         Node {
//             operator: Some(b'|'),
//             children: Some([left, right]),
//             ..
//         } => {
//             // Distribute over conjunctions (A | (B & C)) -> (A | B) & (A | C)
//             if let Node {
//                 operator: Some(b'&'),
//                 ..
//             } = *left.borrow()
//             {
//                 let a_or_b = to_cnf_string(&Rc::new(RefCell::new(Node::new_operator(
//                     b'|', Rc::clone(&left.borrow().children.as_ref().unwrap()[0]), Some(Rc::clone(right)),
//                 ))))?;
//                 let a_or_c = to_cnf_string(&Rc::new(RefCell::new(Node::new_operator(
//                     b'|', Rc::clone(&left.borrow().children.as_ref().unwrap()[1]), Some(Rc::clone(right)),
//                 ))))?;
//                 return Ok(format!("{}{}&", a_or_b, a_or_c));
//             }

//             if let Node {
//                 operator: Some(b'&'),
//                 ..
//             } = *right.borrow()
//             {
//                 let a_or_b = to_cnf_string(&Rc::new(RefCell::new(Node::new_operator(
//                     b'|', Rc::clone(left), Some(Rc::clone(&right.borrow().children.as_ref().unwrap()[0])),
//                 ))))?;
//                 let a_or_c = to_cnf_string(&Rc::new(RefCell::new(Node::new_operator(
//                     b'|', Rc::clone(left), Some(Rc::clone(&right.borrow().children.as_ref().unwrap()[1])),
//                 ))))?;
//                 return Ok(format!("{}{}&", a_or_b, a_or_c));
//             }

//             let left_str = to_cnf_string(left)?;
//             let right_str = to_cnf_string(right)?;
//             Ok(format!("{}{}|", left_str, right_str))
//         }
//         Node {
//             operator: Some(b'&'),
//             children: Some([left, right]),
//             ..
//         } => {
//             let left_str = to_cnf_string(left)?;
//             let right_str = to_cnf_string(right)?;
//             Ok(format!("{}{}&", left_str, right_str))
//         }
//         Node {
//             operator: Some(b'!'),
//             children: Some([child, ..]),
//             ..
//         } => Ok(format!("{}!", to_cnf_string(child)?)),
//         Node {
//             variable: Some(var), ..
//         } => Ok((*var as char).to_string()),
//         _ => Err(anyhow!("Unexpected node structure")),
//     }
// }

// pub fn conjunctive_normal_form(formula: &str) -> Result<String> {
//     // Build the AST using the formula string
//     let tree = build_tree(formula)?;
//     // Convert the AST to its CNF string representation
//     to_cnf_string(&tree)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_conjunctive_normal_form() {
//         assert_eq!(conjunctive_normal_form("AB&!").unwrap(), "A!B!|");
//         assert_eq!(conjunctive_normal_form("AB|!").unwrap(), "A!B!&");
//         assert_eq!(conjunctive_normal_form("AB|C&").unwrap(), "AB|C&");
//         assert_eq!(conjunctive_normal_form("AB|C|D|").unwrap(), "ABC|D|");
//         assert_eq!(conjunctive_normal_form("AB&C&D&").unwrap(), "ABCD&&&");
//         assert_eq!(conjunctive_normal_form("AB&!C!|").unwrap(), "A!B!C!||");
//         assert_eq!(conjunctive_normal_form("AB|!C!&").unwrap(), "A!B!C!&&");
//     }
// }
