
pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut output = Vec::new();
    let n = set.len();
    
    // 부분집합의 크기를 0부터 n까지 증가시키면서 생성
    for k in 0..=n {
        let mut curr = Vec::new();
        backtrack(0, k, &mut curr, set, &mut output);
    }
    
    output
}

// 백트래킹을 통해 부분집합 생성
fn backtrack(first: usize, k: usize, curr: &mut Vec<i32>, set: &[i32], output: &mut Vec<Vec<i32>>) {
    // 현재 부분집합의 크기가 목표 크기(k)와 같으면 결과에 추가
    if curr.len() == k {
        output.push(curr.clone());
        return;
    }

    // 현재 인덱스 이후의 원소들을 탐색하여 부분집합 생성
    for i in first..set.len() {
        // 현재 원소를 부분집합에 추가
        curr.push(set[i]);
        
        // 다음 원소들로 백트래킹
        backtrack(i + 1, k, curr, set, output);
        
        // 백트래킹: 마지막 원소 제거
        curr.pop();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerset() {
        // '[]' gives '[[]]' (1 subset)
        assert_eq!(powerset(&[]), vec![vec![]]);

        // '[0]' gives '[[], [0]]' (2 subset)
        assert_eq!(powerset(&[0]), vec![vec![], vec![0]]);

        // '[0, 1]' gives '[[], [0], [1], [0, 1]]' (4 subset)
        assert_eq!(
            powerset(&[0, 1]),
            vec![vec![], vec![0], vec![1], vec![0, 1]]
        );

        // '[0, 1, 2]' gives '[[], [0], [1], [2], [0, 1], [1, 2], [0, 2], [0, 1, 2]]' (8 subset)
        assert_eq!(
            powerset(&[0, 1, 2]),
            vec![
                vec![],
                vec![0],
                vec![1],
                vec![2],
                vec![0, 1],
                vec![1, 2],
                vec![0, 2],
                vec![0, 1, 2]
            ]
        );
    }
}
