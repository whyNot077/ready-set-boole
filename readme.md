# 러스트 문법 알아보기!

# 윈도우와 mac에서 동시에 작업하기
### Windows에서:
git config --global core.autocrlf true

### Unix/Linux/macOS에서:
git config --global core.autocrlf input

# rust 모듈화하기
### mod.rs 파일
동일한 폴더 내의 폴더 또는 파일 이름을 mod.rs 파일에 넣는다.
pub을 붙이면 다른 파일에서 사용할 수 있게 된다.
파일을 풀러올 때레는 super, crate를 쓴다.
```rs
use super::ast::Node;  // 동일 폴더의 다른 파일에서 가져올 때
use crate::ast::Node; // 다른 폴더의 다른 파일에서 가져올 때
```

# enum과 option
```rs
#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Operand(char), // siye = 8
    Operator(char, Box<ASTNode>, Option<Box<ASTNode>>), // size = 24
}
```

### some
```rs
// 후위 표기식에서 AST를 생성하는 함수
fn postfix_to_ast(tokens: &[Token]) -> Option<ASTNode> {
    let mut stack: Vec<ASTNode> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(value) => stack.push(ASTNode::Operand(*value)),
            Token::Operator(op) => {
                if *op == '!' {
                    // 논리 NOT 연산자는 단항 연산자이므로 하나의 피연산자만 필요
                    let operand = stack.pop()?;
                    stack.push(ASTNode::Operator(*op, Box::new(operand), None)); // 오른쪽 피연산자는 None으로 설정
                } else {
                    let right = stack.pop()?;
                    let left = stack.pop()?;
                    stack.push(ASTNode::Operator(*op, Box::new(left), Some(Box::new(right)))); // Some으로 오른쪽 피연산자 설정
                }
            }
        }
    }

    stack.pop()
}
```
option 부분의 type은 Some 또는 None이다. ASTNode의 옵션에 해당하는 위치에 값을 넣을 때에는 Some으로 wrapping 해주어야 함.

```rs
enum Option<T> {
    Some(T),  // 값이 있을 때 사용
    None,     // 값이 없을 때 사용
}
```

# ? 와 unwrap()의 차이 : 에러 전파 vs. 프로그램 종료
결론: 함수의 리턴값이 result일 때에는 ? 사용, 구체적인 값일 때에는 unwrap() 사용

- ?는 에러가 발생할 가능성이 있는 함수 호출에서 에러를 적절히 처리하고, 이를 호출한 함수에 전달해야 할 때 사용한다. 
Result<T, E>의 경우 Err(E)가 반환되면 그 오류를 호출한 함수로 반환한다.
Option<T>의 경우, Some(T)를 반환하면 그 값을 사용하고, None이 반환되면 호출한 함수에서 None을 반환한다. 

```rs
// option = 값 vs none
enum Option<T> {
    Some(T),
    None,
}
// result = 값 vs 에러
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### unwrap
- unwrap() 함수는 Result나 Option에서 값을 추출하는 데 사용되며, 
오류 처리가 아닌 프로그램이 무조건적으로 성공할 것이라고 가정하는 상황에서 사용된다.
unwrap()은 Result나 Option을 호출하여 Ok(T) 또는 Some(T)인 경우 값을 반환하지만, 
Err(E)나 None인 경우에는 패닉(panic)을 발생시킨다. 즉, 오류가 발생하면 프로그램이 종료됩니다.
- 프로그램이 정상 동작할 것으로 확신하는 상황에서만 사용해야 합니다. 즉, 에러가 발생할 가능성이 없다고 확신할 때 사용됩니다. 예를 들어, 프로토타입 코드나 테스트 코드에서 사용될 수 있지만, 실제 프로덕션 코드에서는 남용을 피해야 합니다.

### as_ref, unwrap_or
```rs
impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Operand(c) => write!(f, "{}", c),
            ASTNode::Operator(op, left, right) => {
                // `right`가 `None`이면 기본 값을 사용 (예: 빈 노드)
                let right_node = right.as_ref().map(|r| &**r).unwrap_or(&ASTNode::Operand('\0'));
                write!(f, "{}{}{}", left, op, right_node)
            }
        }
    }
}
```
- 소유권을 유지한 채로 참조를 얻기: as_ref는 원래 데이터를 소유한 상태에서 참조만을 얻을 수 있습니다.
- Option<Box<ASTNode>>의 구조: Option<Box<ASTNode>>는 Some(Box<ASTNode>) 또는 None의 두 가지 상태를 가질 수 있습니다. as_ref()는 Option의 내부 참조를 가져오고, 이를 Option<&Box<ASTNode>>로 변환합니다. 이렇게 하면 Option 자체를 소비하지 않고도 값에 접근할 수 있습니다.
- as_ref()를 사용하여 Option<&Box<ASTNode>>로 만든 다음, map(|r| &**r)로 Option<&ASTNode>를 반환합니다.
- as_ref()의 필요성: right가 Option<Box<ASTNode>>일 때, 이 값을 그대로 사용하면 Option을 소유권 이동(consume)하게 되어, 이후에는 해당 값을 사용할 수 없습니다. as_ref()를 사용하면 Option<&Box<ASTNode>>를 얻게 되어, 값의 소유권을 이동하지 않고도 참조를 통해 값에 접근할 수 있습니다.  
   
- `map(|r| &**r)`의 역할: 이 부분은 Option<&Box<ASTNode>>를 Option<&ASTNode>로 변환하는 과정입니다. &**r은 Box<ASTNode>를 참조 해제하고, 그 다음에 ASTNode에 대한 참조를 반환합니다.
- unwrap_or(&ASTNode::Operand('\0'))로 None인 경우 빈 노드를 기본값으로 사용합니다.


### ?, context : 함수의 반환형이 Result인 경우 사용
- ? 연산자는 함수가 Result 또는 Option을 반환할 때만 사용할 수 있습니다.
- 에러를 적절하게 처리하고 싶다면 ?를 사용하는 것이 더 좋습니다. 
?는 에러 전파를 자동으로 처리하고, 호출 스택 전체에서 에러를 추적할 수 있습니다.
에러가 절대로 발생하지 않을 것으로 확신한다면 unwrap()을 사용해도 되지만, 프로덕션 코드에서는 가능한 피하는 것이 좋습니다.

```rs
/// 수식을 평가하는 함수
pub fn check_eval_formula(formula: &str) -> Result<bool> {
    let ast = get_ast(formula).context("Failed to create AST from formula")?;
    Ok(evaluate_ast(&ast))
}

pub fn eval_formula(formula: &str) -> bool {
    check_eval_formula(formula).unwrap()
}
```
- get_ast(formula) 호출이 성공하면 ASTNode를 가져오고, 실패하면 context 메서드를 통해 오류 메시지를 덧붙여 처리하려고 합니다.
- unwrap은 Result가 Ok일 때는 그 값을 반환하고, Err일 때는 패닉을 발생시킵니다.
- 오류 처리: check_eval_formula는 Result<bool>을 반환하므로, 오류를 호출자에게 전달할 수 있습니다.
- 간단한 호출: eval_formula는 오류가 발생할 가능성이 없다고 가정하고, 결과만 필요할 때 사용합니다. 만약 오류가 발생하면 패닉이 발생합니다.


### option을 리턴받고 Result로 반환 : ok_or_else
```rs
fn postfix_to_ast(tokens: &[Token]) -> Option<ASTNode>
```

```rs
pub fn get_ast(expression: &str) -> Result<ASTNode> {
    let tokens = tokenize(expression);
    postfix_to_ast(&tokens).ok_or_else(|| anyhow::anyhow!("Failed to generate AST"))
}
```
- ok_or_else는 Option<T> 타입에서 사용할 수 있는 메서드입니다.
- ok_or_else는 Option<T>를 Result<T, E>로 변환합니다. Option이 Some인 경우, 그 값을 Ok로 감싸 Result로 반환합니다. None인 경우, 클로저로 제공된 코드를 실행해 Err를 생성합니다.

### if로 Some인 경우에만 처리하기
```rs
fn extract_vars(node: &ASTNode, vars: &mut HashSet<char>) {
    match node {
        ASTNode::Operand(c) => {
            if c.is_alphabetic() {
                vars.insert(*c);
            }
        }
        ASTNode::Operator(_, left, right_opt) => {
            extract_vars(left, vars);
            if let Some(right) = right_opt {
                extract_vars(right, vars);
            }
        }
    }
}
```

### anyhow
- anyhow는 Rust에서 오류 처리를 간단하게 해주는 라이브러리입니다. 주로 다루기 복잡한 오류를 단일 타입으로 표현하고 싶을 때 유용합니다.
- anyhow::Error는 다양한 오류 타입을 포괄적으로 처리할 수 있는 오류 타입입니다.
- context와 같은 편리한 메서드들을 제공하여, 오류가 발생할 때 더 많은 정보를 추가할 수 있습니다.

### fmt














# 테스트 코드 작성하기
### test drive



# 러스트에서 재귀함수 쓰는 법
### Box<T>
- 스택이 아니라 힙에 데이터를 저장할 수 있도록 해줌
- 박스는 스택 대신 힙에 데이터를 저장한다는 점 외에는, 성능 측면에서의 오버헤드가 없음
하지만 여러 추가 기능도 없습니다. 박스는 아래와 같은 상황에서 가장 자주 쓰이게 됩니다:
    - 컴파일 타임에는 크기를 알 수 없는 타입이 있는데, 정확한 크기를 요구하는 컨텍스트 내에서 그 타입의 값을 사용하고 싶을 때
    - 커다란 데이터를 가지고 있고 소유권을 옮기고 싶지만 그렇게 했을 때 데이터가 복사되지 않을 것을 보장하고 싶을 때
    - 어떤 값을 소유하고 이 값의 구체화된 타입보다는 특정 트레이트를 구현한 타입이라는 점만 신경 쓰고 싶을 때
- Deref을 구현하고 있으므로, 스마트 포인터이다. 
- Rust에서 패턴 매칭 시 box 패턴을 사용할 때 참조를 사용하는 대신 값으로 작업해야 함



# 제너릭 타입
T, U, E
작성한 코드에서 보유하는 값의 타입만 다른 구조체나 열거형이 여러 개 있음을 발견했을 때는 제네릭 타입을 사용해 코드 중복을 제거할 수 있다.
