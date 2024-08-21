use std::collections::HashSet;

const A_ASCII: usize = 65;

#[derive(Clone)]
struct AstNode {
    item: char,
    left_leaf: Option<Box<AstNode>>,
    right_leaf: Option<Box<AstNode>>,
}

impl AstNode {
    fn new(item: char) -> AstNode {
        return AstNode {
            item: (item),
            left_leaf: (None),
            right_leaf: (None),
        };
    }

    fn parse_formula(&mut self, formula: &mut Vec<char>) {
        let operand: Vec<char> = vec!['!', '&', '|', '^', '>', '='];
        self.item = formula.last().copied().unwrap();
        let c: char = formula.pop().unwrap();
        if operand.iter().any(|&i| i == c) {
            self.left_leaf = Some(Box::new(AstNode::new('0')));
            if c != '!' {
                self.right_leaf = Some(Box::new(AstNode::new('0')));
                self.right_leaf.as_mut().unwrap().parse_formula(formula);
            }
            self.left_leaf.as_mut().unwrap().parse_formula(formula);
        }
    }

    fn is_in(&self, haystack: &str) -> bool {
        for c in haystack.chars() {
            if self.item == c {
                return true;
            }
        }
        false
    }

    fn negation_normal_form(&mut self) {
        if self.left_leaf.is_some() {
            self.left_leaf.as_mut().unwrap().negation_normal_form();
        }

        if self.right_leaf.is_some() {
            self.right_leaf.as_mut().unwrap().negation_normal_form();
        }

        if self.item == '!' && self.left_leaf.as_ref().unwrap().is_in("&|") {
            let right_cpy = self.left_leaf.as_mut().unwrap().right_leaf.take();

            if self.left_leaf.as_ref().unwrap().item == '|' {
                self.item = '&';
            } else {
                self.item = '|';
            }

            self.left_leaf.as_mut().unwrap().item = '!';
            self.left_leaf.as_mut().unwrap().right_leaf = None;

            self.right_leaf = Some(Box::new(AstNode::new('!')));
            self.right_leaf.as_mut().unwrap().left_leaf = right_cpy;

            self.negation_normal_form();
        }

        if self.item == '=' {
            self.item = '&';
            let a_cpy = self.left_leaf.take();
            let b_cpy = self.right_leaf.take();

            self.left_leaf = Some(Box::new(AstNode::new('>')));
            self.right_leaf = Some(Box::new(AstNode::new('>')));

            self.left_leaf.as_mut().unwrap().left_leaf = a_cpy.clone();
            self.left_leaf.as_mut().unwrap().right_leaf = b_cpy.clone();

            self.right_leaf.as_mut().unwrap().left_leaf = b_cpy.clone();
            self.right_leaf.as_mut().unwrap().right_leaf = a_cpy.clone();

            self.negation_normal_form();
        }

        if self.item == '^' {
            self.item = '|';
            let a_cpy = self.left_leaf.take();
            let b_cpy = self.right_leaf.take();

            self.left_leaf = Some(Box::new(AstNode::new('&')));
            self.right_leaf = Some(Box::new(AstNode::new('&')));

            self.left_leaf.as_mut().unwrap().right_leaf = Some(Box::new(AstNode::new('!')));
            self.left_leaf
                .as_mut()
                .unwrap()
                .right_leaf
                .as_mut()
                .unwrap()
                .left_leaf = b_cpy.clone();
            self.left_leaf.as_mut().unwrap().left_leaf = a_cpy.clone();

            self.right_leaf.as_mut().unwrap().left_leaf = Some(Box::new(AstNode::new('!')));
            self.right_leaf
                .as_mut()
                .unwrap()
                .left_leaf
                .as_mut()
                .unwrap()
                .left_leaf = a_cpy.clone();
            self.right_leaf.as_mut().unwrap().right_leaf = b_cpy.clone();
            self.negation_normal_form();
        }

        if self.item == '>' {
            let left_cpy = self.left_leaf.take();
            self.item = '|';
            self.left_leaf = Some(Box::new(AstNode::new('!')));
            self.left_leaf.as_mut().unwrap().left_leaf = left_cpy;
            self.negation_normal_form();
        }
    }

    fn compute(&mut self, sets: &Vec<Vec<i32>>, superset: &mut Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        if !self.is_in("&|!") {
            result = sets[self.item as usize - A_ASCII].clone();
        } else if self.item == '!' {
            for element in superset.clone().iter() {
                if !self
                    .left_leaf
                    .as_mut()
                    .unwrap()
                    .compute(sets, superset)
                    .contains(element)
                {
                    result.push(*element);
                }
            }
        } else if self.item == '|' {
            let mut set: HashSet<i32> = HashSet::new();
            set.extend(self.left_leaf.as_mut().unwrap().compute(sets, superset));
            set.extend(self.right_leaf.as_mut().unwrap().compute(sets, superset));
            for x in set.iter() {
                result.push(*x);
            }
        } else if self.item == '&' {
            for x in self.left_leaf.as_mut().unwrap().compute(sets, superset) {
                if self
                    .right_leaf
                    .as_mut()
                    .unwrap()
                    .compute(sets, superset)
                    .contains(&x)
                {
                    result.push(x);
                }
            }
        }
        result
    }
}

fn is_valid_formula(formula: &str, sets_size: usize) -> bool {
    formula
        .chars()
        .collect::<HashSet<_>>()
        .iter()
        .filter(|c| c.is_alphabetic())
        .count()
        == sets_size
}

fn get_superset(sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    for elements in sets.iter() {
        set.extend(elements.clone());
    }
    let result: Vec<i32> = set.into_iter().collect();
    return result;
}

pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    if !is_valid_formula(formula, sets.len()) {
        panic!("The formula and sets provided are not compatible");
    } else {
        let mut formula_stack: Vec<char> = formula.chars().collect();
        let mut root = AstNode::new('0');
        root.parse_formula(&mut formula_stack);
        root.negation_normal_form();
        root.compute(sets, &mut get_superset(sets))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn eval_set_tests() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];

        assert_eq!(eval_set("AB&", &sets), [0]);

        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];

        let mut result: Vec<i32> = eval_set("AB|", &sets);
        result.sort();
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);

        let sets = vec![vec![0, 1, 2]];

        assert_eq!(eval_set("A!", &sets), []);
    }

    #[test]
    fn eval_set_stress_test() {
        let sets = vec![vec![0], vec![0], vec![0]];

        assert_eq!(eval_set("ABC||", &sets), [0]);
        assert_eq!(eval_set("ABC&&", &sets), [0]);
        assert_eq!(eval_set("ABC^^", &sets), [0]);
        assert_eq!(eval_set("ABC>>", &sets), [0]);

        let sets = vec![vec![0], vec![0], vec![]];

        assert_eq!(eval_set("ABC&&", &sets), []);
    }
}
