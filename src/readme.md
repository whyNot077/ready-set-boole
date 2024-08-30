# Code
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

### ex05 Negation Normal Form
- NNF 변환의 목표는 NOT 연산자가 개별 변수에만 적용되고, AND와 OR 연산자가 전체 식을 구성하도록 변환하는 것입니다. 이를 위해 드모르간 법칙을 사용하여 NOT이 AND와 OR 바깥으로 이동하고, 복잡한 논리식을 재구성합니다.
- NNF에서 부정(!) 연산자는 항상 피연산자 바로 앞에 위치하며, AND (&)와 OR (|) 연산자만 사용됩니다. 
- 즉, 부정이 결합된 형태는 단일 변수나 리터럴 앞에만 존재해야 합니다.

### ex06 Conjunctive Normal

### ex07 SAT

### ex08 powerset

### ex09 Set evaluations

### ex10 curve

### ex11 Inverse function 