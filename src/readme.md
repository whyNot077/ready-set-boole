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
- 부정연산을 변환함
- NNF 변환의 목표는 NOT 연산자가 개별 변수에만 적용되고, AND와 OR 연산자가 전체 식을 구성하도록 변환하는 것입니다. 이를 위해 드모르간 법칙을 사용하여 NOT이 AND와 OR 바깥으로 이동하고, 복잡한 논리식을 재구성합니다.
- NNF에서 부정(!) 연산자는 항상 피연산자 바로 앞에 위치하며, AND (&)와 OR (|) 연산자만 사용됩니다. 
- 즉, 부정이 결합된 형태는 단일 변수나 리터럴 앞에만 존재해야 합니다.

### ex06 Conjunctive Normal
- Conjunctive Normal Form (CNF)는 논리식을 AND 연산자와 OR 연산자만을 사용하여 변환한 논리식입니다. CNF로 변환하려면 다음 단계가 필요합니다:
    1. Negation Normal Form (NNF)으로 변환: 이미 구현된 to_nnf 함수를 사용하여 논리식을 NNF로 변환합니다.
    2. 분배 법칙 적용: OR 연산자가 AND 연산자 안으로 분배되도록 논리식을 변환합니다.
        - 변수와 부정만 있는 경우: 그대로 유지 예: !A, B 등.  
        - AND 연산자: 각각의 피연산자에 대해 재귀적으로 CNF를 적용.   
            예: A & B는 A & B로 유지.  
        - OR 연산자: 분배 법칙을 적용하여 OR 연산자가 AND 연산자 안으로 들어가도록 함  
            예: A | (B & C)는 (A | B) & (A | C)로 변환.  

### ex07 SAT

### ex08 powerset

### ex09 Set evaluations

### ex10 curve

### ex11 Inverse function 