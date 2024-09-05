pub fn adder(a: u32, b: u32) -> u32 {
    let res = a ^ b;
    let carry = (a & b) << 1;
    if carry == 0 {
        res
    } else {
        adder(res, carry)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adder() {
        for i in 0u32..10 {
            for j in 0u32..10 {
                assert_eq!(adder(i, j), i + j);
            }
        }
        assert_eq!(
            adder(std::u32::MAX, std::u32::MAX),
            std::u32::MAX.wrapping_add(std::u32::MAX)
        );
    }


    #[test]
    fn eval_adder() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(1, 0), 1);
        assert_eq!(adder(0, 1), 1);
        assert_eq!(adder(1, 1), 2);
        assert_eq!(adder(1, 2), 3);
        assert_eq!(adder(2, 2), 4);
    }
}
