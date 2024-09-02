pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let n = set.len();

    // 2^n개의 부분집합을 생성
    for i in 0..(1 << n) {
        let mut subset: Vec<i32> = Vec::new();
        for j in 0..n {
            // i의 j번째 비트가 1인 경우, j번째 원소를 부분집합에 포함
            if i & (1 << j) != 0 {
                subset.push(set[j]);
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
