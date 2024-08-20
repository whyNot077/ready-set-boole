use crate::adder::adder;

pub fn multiplier(mut a: u32, mut b: u32) -> Result<u32, String> {
    let mut result = 0;
    while b != 0 {
        if b & 1 != 0 {
            let new_result = adder(result, a)?;
            result = new_result;
        }
        a <<= 1;
        b >>= 1;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(5, 7), Ok(35));
        assert_eq!(multiplier(15, 27), Ok(405));
        assert_eq!(multiplier(0, 42), Ok(0));
    }

    #[test]
    fn test_multiplier_edge_cases() {
        assert_eq!(multiplier(1, u32::MAX), Ok(u32::MAX));
        assert_eq!(multiplier(u32::MAX, 1), Ok(u32::MAX));
    }

    #[test]
    fn test_multiplier_large_numbers() {
        assert_eq!(multiplier(10000, 10000), Ok(100_000_000));
        assert_eq!(multiplier(u32::MAX / 2, 2), Ok(u32::MAX - 1));
    }
}
