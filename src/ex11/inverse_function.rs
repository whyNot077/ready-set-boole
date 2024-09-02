pub fn reverse_map(n: f64) -> (u16, u16) {
    // n 값을 [0, 1]에서 [0, 2^32-1]로 스케일링
    let interleaved = (n * (u32::MAX as f64)).round() as u32;

    let mut x: u16 = 0;
    let mut y: u16 = 0;

    // interleaved 비트를 해석하여 x와 y의 비트를 추출
    for i in 0..16 {
        x |= (((interleaved >> (2 * i)) & 1) as u16) << i;
        y |= (((interleaved >> (2 * i + 1)) & 1) as u16) << i;
    }

    (x, y)
}

// 테스트 드라이브
#[cfg(test)]
mod tests {
    use super::*;

    // 이전 exercise에서 사용한 map 함수
    fn map(x: u16, y: u16) -> f64 {
        let mut interleaved = 0u64;

        for i in 0..16 {
            let bit_x = ((x as u64) >> i) & 1;
            let bit_y = ((y as u64) >> i) & 1;

            interleaved |= (bit_x << (2 * i)) | (bit_y << (2 * i + 1));
        }

        interleaved as f64 / ((1u64 << 32) - 1) as f64
    }

    #[test]
    fn test_reverse_map() {
        let original_x = 12345u16;
        let original_y = 54321u16; // 범위 내 값으로 변경

        // map 함수를 통해 좌표를 Z-order Curve 값으로 변환
        let mapped_value = map(original_x, original_y);

        // reverse_map 함수를 통해 좌표 복원
        let (recovered_x, recovered_y) = reverse_map(mapped_value);

        // 원본 좌표와 복원된 좌표가 일치하는지 확인
        assert_eq!((original_x, original_y), (recovered_x, recovered_y));
    }

    #[test]
    fn test_reverse_map_edge_cases() {
        // 좌표가 (0, 0)인 경우
        let mapped_value = map(0, 0);
        let (recovered_x, recovered_y) = reverse_map(mapped_value);
        assert_eq!((recovered_x, recovered_y), (0, 0));

        // 좌표가 최대값인 경우 (65535, 65535)
        let mapped_value = map(65535, 65535);
        let (recovered_x, recovered_y) = reverse_map(mapped_value);
        assert_eq!((recovered_x, recovered_y), (65535, 65535));
    }
}
