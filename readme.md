# Ready Set Boole!

# adder 
```rs
pub fn adder(mut a: u32, mut b: u32) -> Result<u32, String> {
    while b != 0 {
        let carry = a & b;
        a = a ^ b;
        b = carry << 1;
    }
    Ok(a)
}
```
a + b를 비트 연산으로 구현하는 문제이다. 
이 때, 둘 다 