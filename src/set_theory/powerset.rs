pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut cardinal: u32 = 2;
    cardinal = cardinal.pow(set.len() as u32);
    for subset_elt in 0..cardinal {
        let mut subset: Vec<i32> = Vec::new();
        for elt in 0..set.len() {
            if subset_elt & (1 << elt) > 0 {
                subset.push(set[elt]);
            }
        }
        result.push(subset);
    }
    result
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn powerset_with_empty_sets() {
        assert_eq!(powerset(&[]), [[]]);
    }

    #[test]
    fn powerset_basic_test() {
        assert_eq!(powerset(&[42]), vec![vec![], vec![42]]);
    }

    #[test]
    fn powerset_with_big_set() {
        assert_eq!(
            powerset(&[1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }
}
