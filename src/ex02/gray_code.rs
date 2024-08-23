pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gray_code_examples() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
    }

    #[test]
    fn test_gray_code_next_value_different_by_one_bit() {
        // 연속된 두 값의 gray_code 결과가 1비트만 차이나는지 확인
        for n in 0..1_000 {
            let gray_n = gray_code(n);
            let gray_n_plus_one = gray_code(n + 1);
            let differing_bits = (gray_n ^ gray_n_plus_one).count_ones(); // XOR 후 1비트만 차이나야 함
            assert_eq!(differing_bits, 1);
        }
    }

}
