pub fn adder(mut a: u32, mut b: u32) -> Result<u32, String> {
    while b != 0 {
        let carry = a & b;
        a = a ^ b;
        if carry == 0 {
            break;
        }
        b = carry << 1;
    }
    Ok(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        assert_eq!(adder(5, 7), Ok(12));
        assert_eq!(adder(0, 0), Ok(0));
    }
}
