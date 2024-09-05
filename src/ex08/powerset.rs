// pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
//     let mut result: Vec<Vec<i32>> = Vec::new();
//     let n = set.len();

//     // 2^n개의 부분집합을 생성
//     for i in 0..(1 << n) {
//         let mut subset: Vec<i32> = Vec::new();
//         for j in 0..n {
//             // i의 j번째 비트가 1인 경우, j번째 원소를 부분집합에 포함
//             if i & (1 << j) != 0 {
//                 subset.push(set[j]);
//             }
//         }
//         result.push(subset);
//     }

//     result
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_powerset() {
//         // '[]' gives '[[]]' (1 subset)
//         assert_eq!(powerset(&[]), vec![vec![]]);

//         // '[0]' gives '[[], [0]]' (2 subset)
//         assert_eq!(powerset(&[0]), vec![vec![], vec![0]]);

//         // '[0, 1]' gives '[[], [0], [1], [0, 1]]' (4 subset)
//         assert_eq!(
//             powerset(&[0, 1]),
//             vec![vec![], vec![0], vec![1], vec![0, 1]]
//         );

//         // '[0, 1, 2]' gives '[[], [0], [1], [2], [0, 1], [1, 2], [0, 2], [0, 1, 2]]' (8 subset)
//         assert_eq!(
//             powerset(&[0, 1, 2]),
//             vec![
//                 vec![],
//                 vec![0],
//                 vec![1],
//                 vec![2],
//                 vec![0, 1],
//                 vec![1, 2],
//                 vec![0, 2],
//                 vec![0, 1, 2]
//             ]
//         );
//     }
// }
