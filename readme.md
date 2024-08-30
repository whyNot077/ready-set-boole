# 러스트 문법 알아보기!

# 윈도우와 mac에서 동시에 작업하기
### Windows에서:
git config --global core.autocrlf true

### Unix/Linux/macOS에서:
git config --global core.autocrlf input



# ? 와 unwrap()의 차이 : 에러 전파 vs. 프로그램 종료
결론: 함수의 리턴값이 result일 때에는 ? 사용, 구체적인 값일 때에는 unwrap() 사용

- ?는 에러가 발생할 가능성이 있는 함수 호출에서 에러를 적절히 처리하고, 이를 호출한 함수에 전달해야 할 때 사용한다. 
Result<T, E>의 경우 Err(E)가 반환되면 그 오류를 호출한 함수로 반환한다.
Option<T>의 경우, Some(T)를 반환하면 그 값을 사용하고, None이 반환되면 호출한 함수에서 None을 반환한다. 
```rs
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```


- unwrap() 함수는 Result나 Option에서 값을 추출하는 데 사용되며, 
오류 처리가 아닌 프로그램이 무조건적으로 성공할 것이라고 가정하는 상황에서 사용된다.
unwrap()은 Result나 Option을 호출하여 Ok(T) 또는 Some(T)인 경우 값을 반환하지만, 
Err(E)나 None인 경우에는 패닉(panic)을 발생시킨다. 즉, 오류가 발생하면 프로그램이 종료됩니다.
- 프로그램이 정상 동작할 것으로 확신하는 상황에서만 사용해야 합니다. 즉, 에러가 발생할 가능성이 없다고 확신할 때 사용됩니다. 예를 들어, 프로토타입 코드나 테스트 코드에서 사용될 수 있지만, 실제 프로덕션 코드에서는 남용을 피해야 합니다.

- 에러를 적절하게 처리하고 싶다면 ?를 사용하는 것이 더 좋습니다. 
?는 에러 전파를 자동으로 처리하고, 호출 스택 전체에서 에러를 추적할 수 있습니다.
에러가 절대로 발생하지 않을 것으로 확신한다면 unwrap()을 사용해도 되지만, 프로덕션 코드에서는 가능한 피하는 것이 좋습니다.



# rust 모듈화하기
### mod.rs 파일
동일한 폴더 내의 폴더 또는 파일 이름을 mod.rs 파일에 넣는다.
pub을 붙이면 다른 파일에서 사용할 수 있게 된다.
파일을 풀러올 때레는 super, crate를 쓴다.
```rs
use super::ast::Node;  // 동일 폴더의 다른 파일에서 가져올 때
use crate::ast::Node; // 다른 폴더의 다른 파일에서 가져올 때
```

### class 만들기
```rs
pub struct Node {
    pub operator: Option<u8>,
    pub variable: Option<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new_variable(var: u8) -> Self {
        Node {
            operator: None,
            variable: Some(var),
            left: None,
            right: None,
        }
    }
}      
```
### fmt














# 테스트 코드 작성하기
### test drive



# 러스트에서 재귀함수 쓰는 법
### Box<T>
- 스택이 아니라 힙에 데이터를 저장할 수 있도록 해줌
- 박스는 스택 대신 힙에 데이터를 저장한다는 점 외에는, 성능 측면에서의 오버헤드가 없습니다. 
하지만 여러 추가 기능도 없습니다. 박스는 아래와 같은 상황에서 가장 자주 쓰이게 됩니다:
    - 컴파일 타임에는 크기를 알 수 없는 타입이 있는데, 정확한 크기를 요구하는 컨텍스트 내에서 그 타입의 값을 사용하고 싶을 때
    - 커다란 데이터를 가지고 있고 소유권을 옮기고 싶지만 그렇게 했을 때 데이터가 복사되지 않을 것을 보장하고 싶을 때
    - 어떤 값을 소유하고 이 값의 구체화된 타입보다는 특정 트레이트를 구현한 타입이라는 점만 신경 쓰고 싶을 때
- Deref을 구현하고 있으므로, 스마트 포인터이다. 




# 제너릭 타입
T, U, E
작성한 코드에서 보유하는 값의 타입만 다른 구조체나 열거형이 여러 개 있음을 발견했을 때는 제네릭 타입을 사용해 코드 중복을 제거할 수 있다.
