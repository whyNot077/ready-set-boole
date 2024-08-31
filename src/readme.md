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
- NNF 변환의 목표는 NOT 연산자가 개별 변수에만 적용되고, AND와 OR 연산자가 전체 식을 구성하도록 변환하는 것입니다. 이를 위해 드모르간 법칙을 사용하여 NOT이 AND와 OR 바깥으로 이동하고, 복잡한 논리식을 재구성.
- NNF에서 부정(!) 연산자는 항상 피연산자 바로 앞에 위치하며, AND (&)와 OR (|) 연산자만 사용됩니다. 
- 즉, 부정이 결합된 형태는 단일 변수나 리터럴 앞에만 존재해야 함.

### ex06 Conjunctive Normal
- Conjunctive Normal Form (CNF)는 논리식을 AND 연산자와 OR 연산자만을 사용하여 변환한 논리식.
    1. Negation Normal Form (NNF)으로 변환: 이미 구현된 to_nnf 함수를 사용하여 논리식을 NNF로 변환.
    2. 분배 법칙 적용: OR 연산자가 AND 연산자 안으로 분배되도록 논리식을 변환.
        - 변수와 부정만 있는 경우: 그대로 유지 예: !A, B 등.  
        - AND 연산자: 각각의 피연산자에 대해 재귀적으로 CNF를 적용.   
            예: A & B는 A & B로 유지.  
        - OR 연산자: 분배 법칙을 적용하여 OR 연산자가 AND 연산자 안으로 들어가도록 함  
            예: A | (B & C)는 (A | B) & (A | C)로 변환.  

#### 왜 cnf로 논리식을 변환하는가?
1. SAT 문제 해결을 위한 표준 형식
- CNF는 SAT(Satisfiability) 문제를 해결하기 위한 표준 형식
- SAT 문제는 주어진 논리식이 참이 되는지(만족 가능한지) 판단하는 문제로, 컴퓨터 과학에서 중요한 문제 중 하나입니다. CNF는 SAT 솔버(SAT solver)에서 사용되는 대표적인 형식이며, 효율적인 알고리즘이 이 형식을 기반으로 작동합니다.
- SAT 문제의 목표는 논리식을 만족시키는 변수 값을 찾는 것

2. 이론적 단순화 및 정리, 연산 최적화
CNF는 이론적으로 단순화된 형태의 논리식으로, 논리식을 단순하게 변환하는 방식 중 하나입니다. 여러 논리식을 동일한 형식으로 변환할 수 있기 때문에, 이 형식으로 변환함으로써 논리적 추론이 더 명확하고 쉬워집니다.
- 논리적 증명과 자동 정리 증명 등의 도구에서 표준 형식으로 많이 사용됩니다.

3. 디지털 회로 설계
디지털 회로 설계에서 논리식을 CNF로 변환하면, NAND 게이트(AND 게이트의 NOT 버전)와 같은 기본적인 논리 게이트들만으로 회로를 구성할 수 있습니다. 이는 하드웨어 설계에서 비용을 줄이고, 회로의 복잡성을 감소시키는 데 유리합니다.

4. 자동화된 논리 검증
프로그램의 모델 검증(model checking) 및 **형식적 검증(formal verification)**에서 논리식을 CNF로 변환하면, 시스템의 동작을 검증할 때 단순화된 형식으로 표현할 수 있습니다. 이는 시스템이 요구사항을 만족하는지, 논리적 오류가 없는지를 효율적으로 검증할 수 있게 합니다.


논리식 (A & B) & C는 다음과 같이 표현된다. 
```less
// 초기 상태 : ast 트리
Operator('&', 
    Operator('&', 
        Operand('A'), 
        Operand('B')
    ), 
    Operand('C')
)

// flatten_and 이후
[Operand('A'), Operand('B'), Operand('C')]
```

```rs
/// AND 연산자를 가진 노드들을 플랫하게 만드는 함수
fn flatten_and(ast: &ASTNode, nodes: &mut Vec<ASTNode>) {
    match ast {
        ASTNode::Operator('&', left, right) => {
            flatten_and(left, nodes);  // 왼쪽 자식을 재귀적으로 평탄화
            flatten_and(right, nodes); // 오른쪽 자식을 재귀적으로 평탄화
        }
        _ => nodes.push(ast.clone()),  // AND 연산자가 아닌 경우 단순히 벡터에 추가
    }
}
```
flatten_and 함수는 이를 [Operand('A'), Operand('B'), Operand('C')]와 같이 단순한 리스트로 변환. 변환된 리스트는 이후의 과정에서 다시 A & B & C 형태의 트리로 재구성됩니다.

### ex07 SAT

### ex08 powerset

### ex09 Set evaluations

### ex10 curve

### ex11 Inverse function 