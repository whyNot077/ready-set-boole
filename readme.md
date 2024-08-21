# Ready Set Boole!

### ex00 adder 
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

### ex01 Multipliers

### ex02 gray code

### ex03 boolean evaluation

### ex04 truth table

### ex05 Negation Normal

### ex06 Conjunctive Normal

### ex07 SAT

### ex08 powerset

### ex09 Set evaluations

### ex10 curve

### ex11 Inverse function 