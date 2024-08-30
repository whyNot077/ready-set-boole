// use crate::ex03::ast::Node; // Import the Node struct from ast.rs
// use anyhow::{Result, anyhow};
// use std::rc::Rc;
// use std::cell::RefCell;

// fn parse(mut s: &[u8]) -> Result<(Rc<RefCell<Node>>, &[u8])> {
//     let mut val = *s.last().unwrap();
//     s = &s[..s.len() - 1];
//     let mut neg = false;

//     while val == b'!' {
//         val = *s
//             .last()
//             .ok_or_else(|| anyhow!("invalid input (invalid negation)"))?;
//         s = &s[..s.len() - 1];
//         neg = !neg;
//     }

//     match val {
//         b'&' | b'^' | b'|' | b'>' | b'=' => {
//             let (right, s) = parse(s)?;
//             let (left, s) = parse(s)?;

//             let node = Node::new_operator(val, left, Some(right));
//             if neg {
//                 let neg_node = Node::new_operator(b'!', Rc::new(RefCell::new(node)), None);
//                 return Ok((Rc::new(RefCell::new(neg_node)), s));
//             }
//             Ok((Rc::new(RefCell::new(node)), s))
//         }
//         b'A'..=b'Z' => {
//             let node = Node::new_variable(val);
//             if neg {
//                 let neg_node = Node::new_operator(b'!', Rc::new(RefCell::new(node)), None);
//                 return Ok((Rc::new(RefCell::new(neg_node)), s));
//             }
//             Ok((Rc::new(RefCell::new(node)), s))
//         }
//         _ => Err(anyhow!("invalid input (invalid char `{}`)", val as char)),
//     }
// }

// pub fn build_tree(s: &str) -> Result<Rc<RefCell<Node>>> {
//     let res = parse(s.as_bytes())?;
//     assert!(res.1.is_empty());
//     Ok(res.0)
// }

// fn to_nnf_string(node: &Rc<RefCell<Node>>) -> Result<String> {
//     let node_ref = node.borrow();
//     if let Some(b'!') = node_ref.operator {
//         let child = node_ref.children.as_ref().unwrap()[0].clone();
//         let child_op = child.borrow().operator; // Borrow operator
//         match child_op {
//             Some(b'&') => {
//                 let left_nnf = to_nnf_string(&Rc::new(RefCell::new(Node::new_operator(
//                     b'!', Rc::clone(&child.borrow().children.as_ref().unwrap()[0]), None,
//                 ))))?;
//                 let right_nnf = to_nnf_string(&Rc::new(RefCell::new(Node::new_operator(
//                     b'!', Rc::clone(&child.borrow().children.as_ref().unwrap()[1]), None,
//                 ))))?;
//                 Ok(format!("{}{}|", left_nnf, right_nnf))
//             }
//             Some(b'|') => {
//                 let left_nnf = to_nnf_string(&Rc::new(RefCell::new(Node::new_operator(
//                     b'!', Rc::clone(&child.borrow().children.as_ref().unwrap()[0]), None,
//                 ))))?;
//                 let right_nnf = to_nnf_string(&Rc::new(RefCell::new(Node::new_operator(
//                     b'!', Rc::clone(&child.borrow().children.as_ref().unwrap()[1]), None,
//                 ))))?;
//                 Ok(format!("{}{}&", left_nnf, right_nnf))
//             }
//             Some(b'!') => to_nnf_string(&child.borrow().children.as_ref().unwrap()[0]),
//             _ => Ok(format!("{}!", to_nnf_string(&child)?)),
//         }
//     } else if let Some(b'>') = node_ref.operator {
//         let left_nnf = to_nnf_string(&Rc::new(RefCell::new(Node::new_operator(
//             b'!', Rc::clone(&node_ref.children.as_ref().unwrap()[0]), None,
//         ))))?;
//         let right_nnf = to_nnf_string(&node_ref.children.as_ref().unwrap()[1])?;
//         Ok(format!("{}{}|", left_nnf, right_nnf))
//     } else if let Some(b'=') = node_ref.operator {
//         let left_and_right = format!("{}{}&", to_nnf_string(&node_ref.children.as_ref().unwrap()[0])?, to_nnf_string(&node_ref.children.as_ref().unwrap()[1])?);
//         let not_left = to_nnf_string(&Rc::new(RefCell::new(Node::new_operator(
//             b'!', Rc::clone(&node_ref.children.as_ref().unwrap()[0]), None,
//         ))))?;
//         let not_right = to_nnf_string(&Rc::new(RefCell::new(Node::new_operator(
//             b'!', Rc::clone(&node_ref.children.as_ref().unwrap()[1]), None,
//         ))))?;
//         let not_left_and_right = format!("{}{}&", not_left, not_right);
//         Ok(format!("{}{}|", left_and_right, not_left_and_right))
//     } else {
//         if let Some(children) = &node_ref.children {
//             let left_nnf = to_nnf_string(&children[0])?;
//             let right_nnf = to_nnf_string(&children[1])?;
//             Ok(format!("{}{}{}", left_nnf, right_nnf, node_ref.operator.unwrap() as char))
//         } else {
//             Ok((node_ref.variable.unwrap() as char).to_string())
//         }
//     }
// }

// pub fn negation_normal_form(formula: &str) -> Result<String> {
//     let tree = build_tree(formula)?;
//     to_nnf_string(&tree)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_negation_normal_form() {
//         assert_eq!(negation_normal_form("AB&!").unwrap(), "A!B!|");
//         assert_eq!(negation_normal_form("AB|!").unwrap(), "A!B!&");
//         assert_eq!(negation_normal_form("AB>").unwrap(), "A!B|");
//         assert_eq!(negation_normal_form("AB=").unwrap(), "AB&A!B!&|");
//         assert_eq!(negation_normal_form("AB|C&!").unwrap(), "A!B!&C!|");
//         assert_eq!(negation_normal_form("A!B!|C!&").unwrap(), "A!B!|C!&");
//     }
// }
