pub fn map(x: u16, y: u16) -> f64 {
    // 비트 인터리빙을 사용하여 Z-order Curve를 생성
    let mut interleaved = 0u64;

    for i in 0..16 {
        let bit_x = ((x as u64) >> i) & 1;
        let bit_y = ((y as u64) >> i) & 1;

        interleaved |= (bit_x << (2 * i)) | (bit_y << (2 * i + 1));
    }

    // 결과를 [0, 1] 범위의 실수로 변환
    interleaved as f64 / ((1u64 << 32) - 1) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_zero() {
        // x = 0, y = 0 should map to 0.0
        assert_eq!(map(0, 0), 0.0);
    }

    #[test]
    fn test_map_max() {
        // x = 65535, y = 65535 should map to 1.0 (or very close)
        let result = map(65535, 65535);
        assert!((result - 1.0).abs() < 1e-9); // 허용 오차 내에서 비교
    }

    #[test]
    fn test_map_middle() {
        // x = 32768, y = 32768의 실제 반환 값을 확인
        let result = map(32768, 32768);
        println!("Result of map(32768, 32768): {}", result);
        assert!(result >= 0.0 && result <= 1.0); // 결과가 0과 1 사이인지 확인
    }
    

    #[test]
    fn test_map_x_y_equal() {
        // x = 12345, y = 12345 should produce a valid result
        let result = map(12345, 12345);
        assert!(result >= 0.0 && result <= 1.0); // [0, 1] 범위 내에 있어야 함
    }

    #[test]
    fn test_map_x_max_y_zero() {
        // x = 65535, y = 0 should produce a valid result
        let result = map(65535, 0);
        assert!(result >= 0.0 && result <= 1.0); // [0, 1] 범위 내에 있어야 함
    }

    #[test]
    fn test_map_y_max_x_zero() {
        // x = 0, y = 65535 should produce a valid result
        let result = map(0, 65535);
        assert!(result >= 0.0 && result <= 1.0); // [0, 1] 범위 내에 있어야 함
    }
}
