use crate::ex00::adder::adder;

/// Multiply `a` with each bit of `b` and `add` all the partial products.
pub fn multiplier(a: u32, b: u32) -> u32 {
    if b == 0 {
        return 0;
    }
    if b & 1 == 0 {
        // add a * 0 + multiplier(...)
        multiplier(a, b >> 1) << 1
    } else {
        // add a * 1 + multiplier(...)
        adder(a, multiplier(a, b >> 1) << 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiplier() {
        for i in 0u32..10 {
            for j in 0u32..10 {
                assert_eq!(multiplier(i, j), i * j);
            }
        }
        assert_eq!(
            multiplier(std::u32::MAX, std::u32::MAX),
            std::u32::MAX.wrapping_mul(std::u32::MAX)
        );
    }
}
