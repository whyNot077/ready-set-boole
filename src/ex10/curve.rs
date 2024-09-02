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
