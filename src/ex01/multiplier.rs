use crate::ex00::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    if b == 0 {
        return 0;
    }
    if b & 1 == 0 {
        multiplier(a, b >> 1) << 1
    } else {
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
    #[test]
    fn eval_multiplier() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(1, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(1, 2), 2);
        assert_eq!(multiplier(2, 2), 4);
    }
}
