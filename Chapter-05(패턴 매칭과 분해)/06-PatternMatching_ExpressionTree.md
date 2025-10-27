# 패턴 매칭 Expression Tree

**재귀적인 수식 트리(Expression Tree)** 를 정의하고 평가하는 구조입니다.  
아래에 전체 흐름을 단계별로 체계적으로 설명.

## 전체 소스
```rust

/// 두 개의 하위 표현식에서 실행할 연산입니다.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}
```

```rust
/// 트리 형식의 표현식입니다.
#[derive(Debug)]
enum Expression {
    /// 두 개의 하위 표현식에 관한 연산입니다.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },
    /// 리터럴 값
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right } => {
            let left = match eval(*left) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            let right = match eval(*right) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            Ok(match op {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => {
                    if right == 0 {
                        return Err(String::from("0으로 나누기"));
                    } else {
                        left / right
                    }
                }
            })
        }
        Expression::Value(v) => Ok(v),
    }
}
```
```rust
#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}
```rust
#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}
```
```rust
#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}
```
```rust
#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("0으로 나누기"))
    );
}
```
```rust
fn main() {
    let expr = Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    };
    println!("expr: {:?}", expr);
    println!("결과: {:?}", eval(expr));
}
```

## 🧠 1. 핵심 구조 정의
### 🔹 Operation 열거형
```rust
#[derive(Debug)]
enum Operation {
    Add, // 덧셈
    Sub, // 뺄셈
    Mul, // 곱셈
    Div, // 나눗셈
}
```

- 수식에서 사용할 연산 종류를 정의
- Add, Sub, Mul, Div는 각각 +, −, ×, ÷에 해당
- `#[derive(Debug)]` → 디버깅 시 출력 가능

### 🔹 Expression 열거형
```rust
#[derive(Debug)]
enum Expression {
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },
    Value(i64),
}
```

- 수식을 트리 형태로 표현하는 구조
- Value(i64) → 리터럴 숫자
- Op { ... } → 연산 노드 (왼쪽/오른쪽 하위 표현식 포함)
- Box<Expression> → 재귀 구조를 위해 힙에 저장 (Rust는 크기 고정이 필요하므로 Box 사용)

## 🔧 2. 평가 함수 eval

```rust
fn eval(e: Expression) -> Result<i64, String>
```

- Expression을 받아서 계산 결과를 Result<i64, String>으로 반환
- 성공 시 Ok(값), 실패 시 Err(문자열) (예: 0으로 나누기)

### 🔹 내부 로직
```rust
match e {
    Expression::Op { op, left, right } => {
        let left = match eval(*left) { ... };
        let right = match eval(*right) { ... };
        Ok(match op { ... })
    }
    Expression::Value(v) => Ok(v),
}
```

- 재귀적으로 하위 표현식을 평가함
- Box<Expression>을 *left, *right로 꺼내서 다시 eval 호출
- Operation에 따라 계산 수행
- Div일 경우 right == 0이면 오류 반환

## 🧪 3. 테스트 함수들
### ✅ test_value
```rust
assert_eq!(eval(Expression::Value(19)), Ok(19));
```

- 단일 숫자 표현식 평가 → 19 반환

### ✅ test_sum
```rust
eval(Expression::Op {
    op: Operation::Add,
    left: Box::new(Expression::Value(10)),
    right: Box::new(Expression::Value(20)),
})
```

- 10 + 20 → 30 반환

### ✅ test_recursion
```rust
let term1 = 10 * 9 = 90  
let term2 = (3 - 4) * 5 = (-1) * 5 = -5
```  
→ 전체: 90 + (-5) = 85


- 복잡한 트리 구조를 재귀적으로 평가

### ✅ test_error
```rust
eval(99 / 0) → Err("0으로 나누기")
```

- 나눗셈에서 0으로 나누면 오류 반환

## ▶️ 4. main() 함수
```rust
let expr = Expression::Op {
    op: Operation::Sub,
    left: Box::new(Expression::Value(20)),
    right: Box::new(Expression::Value(10)),
};
```

- 20 - 10 → 10 출력
- println!("{:?}", expr) → 트리 구조 출력
- println!("{:?}", eval(expr)) → 결과 출력

## ✅ 전체 흐름 요약: 수식 트리 평가 구조
| 단계        | 설명                                                                 |
|-------------|----------------------------------------------------------------------|
| Operation   | 덧셈, 뺄셈, 곱셈, 나눗셈을 나타내는 열거형 → 수식의 연산 종류 정의         |
| Expression  | 리터럴 값(Value) 또는 연산 노드(Op)로 구성된 재귀적 수식 트리 구조         |
| eval()      | Expression을 재귀적으로 평가하여 i64 결과 반환 → 오류 시 Err 처리          |
| Err         | 0으로 나누기 등 예외 상황 발생 시 문자열 기반 오류 반환                   |
| main()      | 수식을 생성하고 평가 결과를 출력 → 디버깅 및 실행 확인용                   |


## 💡 확장 아이디어
- Operation::Pow 추가 → 거듭제곱 연산
- Expression::Neg(Box<Expression>) → 음수 표현
- eval()을 &Expression 참조로 바꾸면 성능 향상 가능
- Display 구현 → 사람이 읽기 쉬운 수식 출력

## 🧠 enum Expression에 Op와 Value를 넣는 이유와 장점

### 1. 재귀적 수식 표현이 가능해짐
```rust
Expression::Op {
    op: Operation::Add,
    left: Box::new(Expression::Value(3)),
    right: Box::new(Expression::Value(4)),
}
```

- Expression 안에 또 다른 Expression을 넣을 수 있음 → 트리 구조 형성
- 복잡한 수식도 하나의 구조로 표현 가능: ((3 + 4) * 5) - 2

### 2. 패턴 매칭으로 명확한 처리 가능
```rust
match expr {
    Expression::Value(v) => ...,
    Expression::Op { op, left, right } => ...,
}
```

- match를 통해 수식의 종류에 따라 분기 처리가 쉬움
- Value는 그대로 반환, Op는 재귀적으로 평가

### 3. 확장성과 유지보수에 유리
- 새로운 연산 추가 시 Operation에 variant만 추가하면 됨
- 새로운 표현식 형태 추가 시 Expression에 variant만 추가하면 됨
- 예: Neg(Box<Expression>), Pow, FuncCall(String, Vec<Expression>)

### 4. 디버깅과 시각화에 유리한 구조
- #[derive(Debug)] 덕분에 println!("{:?}", expr)로 구조 확인 가능
- 트리 구조를 그대로 시각화하거나 직렬화하기 쉬움

### 5. 타입 안정성과 안전한 표현 보장
- Rust의 enum은 타입 안정성이 뛰어나서 잘못된 표현식을 컴파일 타임에 방지
- 예: Expression::Op에는 반드시 Operation, left, right가 있어야 함

## ✅ 요약: 수식 평가 흐름에서 match의 역할
| 항목     | 설명                                                              |
|----------|-------------------------------------------------------------------|
| match    | Expression의 variant(Value 또는 Op)을 분기 처리하는 핵심 로직       |
| Value    | 리터럴 값을 그대로 반환 → match에서 Ok(v)로 처리                   |
| Op       | 연산 노드 → match에서 재귀적으로 left/right 평가 후 연산 수행       |
| 오류 처리 | match 내부에서 0 나누기 등 예외 상황을 감지하고 Err로 반환         |
| 재귀 구조 | match를 통해 Expression 트리를 깊이 따라가며 평가 가능             |

이 구조는 단순한 enum이 아니라  
수식 해석기, 컴파일러, 계산기, 시뮬레이션 엔진 등에서 핵심적인 표현 방식.  


e @ Err(_) => return e는 Rust의 패턴 매칭에서 "패턴 바인딩"을 사용하는 문법입니다.  
아래에 이 문법을 정확하게, 단계별로 설명.

## 🧠 핵심 문법: e @ Err(_) => return e
이 문장은 match 표현식 안에서 사용되며, 의미는 다음과 같습니다:
- Err(_) → Result 타입에서 오류인 경우를 매칭
- e @ Err(_) → 오류를 e라는 이름으로 바인딩
- return e → 바인딩된 e를 그대로 반환
즉, 오류가 발생하면 그 오류를 그대로 반환하는 구조입니다.

## 🔍 예제 분석
```rust
let left = match eval(*left) {
    Ok(v) => v,
    e @ Err(_) => return e,
};
```

- eval(*left)가 Ok(v)이면 v를 꺼냄
- eval(*left)가 Err(...)이면 e에 바인딩하고 return e로 즉시 반환
- e는 Err(...) 전체 값을 그대로 갖고 있음

## 🔧 왜 e @ Err(_)를 쓰는가?
| 표현            | 설명                                                              |
|-----------------|-------------------------------------------------------------------|
| Err(_)          | 오류를 감지하지만 내부 값은 직접 사용하지 않음 (`_`는 무시 패턴)     |
| e @ Err(_)      | 오류 전체를 변수 `e`에 바인딩함 → 이후에 그대로 반환하거나 로깅 가능  |
| return e        | 바인딩된 `Err(...)` 값을 그대로 반환함 → 오류 메시지 포함됨           |


이 방식은 오류를 무시하지 않고 그대로 전달할 수 있게 해줍니다.

## ✅ 예시 비교
### ❌ 잘못된 방식 (값을 버림)
```rust
match eval(expr) {
    Err(_) => return Err("실패".to_string()), // 원래 오류 정보 손실
    Ok(v) => v,
}
```

## ✅ 올바른 방식 (오류 전달)
```rust
match eval(expr) {
    e @ Err(_) => return e, // 원래 오류 그대로 전달
    Ok(v) => v,
}
```


## ✅ 패턴 바인딩 문법 요약: e @ Err(_) => return e
| 항목         | 설명                                                              |
|--------------|-------------------------------------------------------------------|
| Err(_)       | 오류 패턴을 매칭하지만 내부 값을 무시함 (`_`)                        |
| e @ ...      | 전체 패턴을 변수 `e`에 바인딩함 → 이후에 `e`를 그대로 사용할 수 있음   |
| return e     | 바인딩된 오류 값을 그대로 반환함 → 오류 전파에 유용                   |

이 문법은 에러 전파를 깔끔하게 처리할 수 있는 Rust의 강력한 패턴 매칭 기능입니다.  
e @ Err(_) => return e에서 _는 오류 메시지를 버리는 게 아니라 무시하는 패턴이고,  
e는 그 전체 Err(...) 값을 그대로 바인딩합니다.
즉, Rust는 내부적으로 Err("0으로 나누기") 같은 값을 갖고 있고,  
우리는 그걸 e라는 이름으로 받아서 return e로 그대로 반환 것임.

## 🔍 핵심 이해 포인트: e @ Err(_) => return e;
| 표현            | 의미                                               |
|-----------------|----------------------------------------------------|
| Err(_)          | Result가 Err일 때 매칭 → 내부 값은 무시하지만 존재함 |
| e @ Err(_)      | Err(...) 전체를 변수 e에 바인딩함                   |
| return e        | 바인딩된 e를 그대로 반환 → 예: Err("0으로 나누기")   |

## 🧠 왜 메시지를 받지 않았는데 e에 들어가는가?
- Rust의 패턴 매칭은 값을 구조적으로 매칭하면서 동시에 바인딩할 수 있음
- _는 “이 값은 필요 없지만 매칭은 하겠다”는 의미
- e @ Err(_)는 “이 Err 전체를 e에 저장하면서 내부 값은 무시하겠다”는 의미
즉, _는 버리는 게 아니라 무시하는 것이고,  
e는 그 무시된 값을 포함한 전체 Err(...)를 갖고 있음.

## ✅ 예시로 확인
```rust
let result = Err("실패 메시지");

match result {
    e @ Err(_) => println!("전체 오류: {:?}", e),
    Ok(_) => {}
}
```

### 출력:
```
전체 오류: Err("실패 메시지")
```
→ _로 내부 메시지를 무시했지만, e는 전체 Err("실패 메시지")를 갖고 있음.


## 🔁 무한 확장성: 표현식 트리의 깊이와 다양성
- enum Expression은 Box<Expression>을 통해 자기 자신을 포함할 수 있음.
- 이 구조는 트리 형태로 무한히 깊은 수식을 표현할 수 있게 해줍니다.  
예:
```rust
Expression::Op {
    op: Add,
    left: Box::new(Expression::Op {
        op: Mul,
        left: Box::new(Expression::Value(2)),
        right: Box::new(Expression::Value(3)),
    }),
    right: Box::new(Expression::Value(4)),
}
```
→ ((2 * 3) + 4)처럼 중첩된 수식을 자유롭게 표현 가능

## ✅ 확정성(정형성): 타입 시스템이 보장하는 안정성
- Rust의 enum은 모든 경우의 수를 컴파일 타임에 명확히 정의합니다.
- match를 통해 모든 variant를 처리해야 하므로 누락된 경우가 없도록 강제됩니다.
- Expression은 Value 또는 Op 중 하나만 가질 수 있으므로 불확실성이 없음
예:
```rust
match expr {
    Expression::Value(v) => ...,
    Expression::Op { op, left, right } => ...,
}
```

→ 이 구조는 확장 가능하면서도 안정적인 수식 표현을 가능하게 해줌.

## 🧠 결론
Rust의 enum을 재귀적으로 설계하면
무한히 복잡한 구조를 표현할 수 있는 유연성과
컴파일 타임에 모든 경우를 명확히 처리할 수 있는 확정성을 동시에 얻을 수 있음.

이건 Rust가 함수형 언어의 표현력과 시스템 언어의 안전성을
아주 세련되게 결합한 예라고 볼 수 있음.

---

# 병렬 처리

Box 대신 Arc<Mutex<...>>로 감싸면 **동시성(병렬 처리)**이 가능한 구조로 바뀝니다.
아래에 그 의미와 구조적 변화, 그리고 주의할 점까지 체계적으로 설명.

## 🧠 기본 개념 비교: Rust의 포인터 및 동기화 타입
| 타입             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| Box<T>           | 힙에 저장된 단일 소유자 포인터 → 재귀 구조나 큰 데이터를 다룰 때 사용 |
| Arc<T>           | 참조 카운트 기반 공유 포인터 → 여러 스레드에서 읽기 전용으로 공유 가능 |
| Mutex<T>         | 내부 데이터에 대한 동기화된 접근 제어 → 단일 스레드에서도 lock 필요     |
| Arc<Mutex<T>>    | 여러 스레드에서 안전하게 공유 및 수정 가능 → 병렬 처리에 적합            |

## 🔧 구조 변경 예시
### 기존:
```rust
Expression::Op {
    op: Operation::Add,
    left: Box::new(Expression::Value(1)),
    right: Box::new(Expression::Value(2)),
}
```

## 변경 후:
```rust
use std::sync::{Arc, Mutex};

Expression::Op {
    op: Operation::Add,
    left: Arc::new(Mutex::new(Expression::Value(1))),
    right: Arc::new(Mutex::new(Expression::Value(2))),
}
```
→ 이제 left와 right는 다른 스레드에서도 안전하게 접근 가능

## ✅ 병렬 처리 가능해지는 이유
- Arc는 참조 카운트를 통해 여러 스레드에서 안전하게 공유
- Mutex는 내부 데이터를 lock/unlock으로 보호
- 따라서 Arc<Mutex<Expression>> 구조는 스레드 간 공유 + 수정이 모두 가능

### 예:
```rust
let expr = Arc::new(Mutex::new(Expression::Value(42)));

let cloned = Arc::clone(&expr);
std::thread::spawn(move || {
    let mut locked = cloned.lock().unwrap();
    *locked = Expression::Value(99);
});
```


## ⚠️ 주의할 점: Arc<Mutex<Expression>> 사용 시
| 항목                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| Mutex                         | lock/unlock 비용이 있으므로 과도한 사용 시 성능 저하 가능               |
| Mutex                         | 여러 개의 Mutex를 동시에 사용할 경우 데드락 위험 존재                  |
| eval() + Arc<Mutex<Expression>> | 평가 시 반드시 lock() 후 *expr로 접근해야 하며, 재귀적으로 처리 시 주의 필요 |


## 💡 결론
Box는 단일 스레드에서 재귀 구조 표현에 적합하고,
Arc<Mutex<...>>는 병렬 처리와 공유 수정이 필요한 경우에 적합한 구조입니다.

---
# Heap Pointer (Box)

Rust에서 Box<T>는 단순한 포인터가 아니라, 안전성과 표현력을 동시에 갖춘 힙 포인터입니다.  
아래에 Box가 어떻게 C++의 포인터 역할을 대체하면서도 더 나은 구조를 제공하는지 설명.

## 🧠 Box<T>는 어떤 포인터인가?
- Box<T>는 값을 힙에 저장하고 그 주소를 스택에 보관하는 스마트 포인터
- C++의 new + raw pointer (T*)와 비슷하지만,
Rust에서는 소유권과 수명을 컴파일 타임에 체크함
```rust
let b = Box::new(42); // 42는 힙에 저장됨
```

---

## 🔧 왜 Box가 필요한가?
### 1. 재귀적 데이터 구조 표현
Rust는 타입 크기를 컴파일 타임에 알아야 하기 때문에
자기 자신을 포함하는 구조는 Box 없이는 만들 수 없음.
```rust
enum List {
    Node(i32, Box<List>),
    Nil,
}
```

→ C++에서는 그냥 List*로 가능하지만, Rust는 명시적으로 안전하게 표현해야 함

### 2. 소유권과 자동 해제
- Box<T>는 소유권을 갖는 포인터 → drop 시 자동으로 메모리 해제
- C++에서는 delete를 직접 호출하거나 스마트 포인터(unique_ptr)를 써야 함

### 3. 표현력과 안전성의 균형
- Box는 단순한 포인터처럼 보이지만,
Rust의 타입 시스템과 함께 쓰면 불변성, 수명, 안전성을 모두 보장함

## ✅ C++ 포인터 vs Rust Box 비교
| 항목           | T* (C++ 포인터)                         | Box<T> (Rust 힙 포인터)                        |
|----------------|------------------------------------------|------------------------------------------------|
| 메모리 위치    | 힙에 할당 (`new T`)                      | 힙에 할당 (`Box::new(T)`)                      |
| 소유권         | 없음 → 수동으로 delete 필요              | 있음 → drop 시 자동 해제                       |
| 안전성         | 위험함 (null, dangling, double free 등)  | 안전함 (컴파일러가 수명과 소유권 체크)         |
| 표현력         | 자유롭지만 실수 가능                     | 제한적이지만 명확하고 안전                     |
| 재귀 구조       | 가능 (단순 포인터로 연결)                | `Box` 없이는 불가능 → enum 내부에 자기 자신 포함 시 필수 |
| Box 역할       | 스마트 포인터가 필요 (`unique_ptr`, etc) | Rust에서는 기본적으로 안전한 힙 포인터 역할 수행 |


## 💬 결론
Rust의 Box<T>는 단순한 포인터가 아니라,  
안전한 힙 기반 표현을 위한 도구이자, 타입 시스템과 함께 작동하는 설계 요소입니다.  
Rust의 Box<T>는 C++의 raw pointer (T*)와 거의 동일한 크기를 갖습니다.  
Rust의 철학답게 불필요한 런타임 오버헤드는 거의 없고, 안전성과 표현력은 훨씬 뛰어난 구조.  

---

## 📏 Box<T>의 크기와 오버헤드
- Box<T>는 내부적으로 단일 포인터만 저장합니다.
- 즉, Box<T>의 크기는 **플랫폼에 따라 4바이트(x86) 또는 8바이트(x86_64)**로 C++의 T*와 동일합니다.
```rust
use std::mem::size_of;
fn main() {
    println!("Box<i32>: {}", size_of::<Box<i32>>()); // 보통 8바이트
    println!("*const i32: {}", size_of::<*const i32>()); // 보통 8바이트
}
```

→ 출력 결과는 대부분 동일합니다.

## ✅ Rust 철학과의 연결
| 철학 개념             | 설명                                                               |
|------------------------|--------------------------------------------------------------------|
| Zero-cost abstraction  | 런타임 오버헤드 없이 안전한 힙 포인터 제공 → `Box`는 단일 포인터 크기 |
| 명시적 소유권          | `Box`는 소유권을 갖고 있어 drop 시 자동 해제 → 메모리 누수 방지       |
| 안전한 메모리 관리      | null, dangling, double free 방지 → 컴파일러가 수명과 접근을 체크       |
| 타입 시스템과 통합      | `Box<T>`는 타입 안정성과 lifetime 추적에 완벽히 통합됨               |
| 재귀적 구조 표현        | `Box` 없이는 자기 자신을 포함하는 enum 불가능 → 표현력의 핵심 도구     |

## 💡 결론
Rust의 Box<T>는 C++ 포인터처럼 가볍지만,  
소유권, 수명, 안전성까지 모두 갖춘 철학적 설계의 결정체입니다.

----

# C++로 재연

아래는 C++ 에서 std::unique_ptr을 사용해 Rust 스타일의 enum + Box 트리 구조를 흉내 낸 예제입니다.
Rust의 enum Expression을 C++ 로 재현하면서 소유권, 재귀 구조, 안전성을 최대한 반영.

## 🦀 Rust 스타일 구조 (참고)
```rust
enum Expression {
    Value(i64),
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}
```


## 🛠️ C++로 흉내 낸 구조
```cpp
#include <iostream>
#include <memory>
#include <variant>

// 연산 종류
enum class Operation { Add, Sub, Mul, Div };

// Expression 트리 노드
struct Expression;

// Value 노드
struct Value {
    int64_t val;
};

// Op 노드
struct Op {
    Operation op;
    std::unique_ptr<Expression> left;
    std::unique_ptr<Expression> right;
};

// Expression은 Value 또는 Op 중 하나
struct Expression {
    std::variant<Value, Op> node;
};
```

## 🔁 평가 함수 (Rust의 match처럼)
```rust
int64_t eval(const Expression& expr) {
    if (std::holds_alternative<Value>(expr.node)) {
        return std::get<Value>(expr.node).val;
    }

    const Op& op = std::get<Op>(expr.node);
    int64_t l = eval(*op.left);
    int64_t r = eval(*op.right);

    switch (op.op) {
        case Operation::Add: return l + r;
        case Operation::Sub: return l - r;
        case Operation::Mul: return l * r;
        case Operation::Div: return r != 0 ? l / r : throw std::runtime_error("Divide by zero");
    }

    throw std::runtime_error("Unknown operation");
}
```

## ✅ 사용 예시
```cpp
int main() {
    auto expr = std::make_unique<Expression>(Expression{
        Op{
            Operation::Add,
            std::make_unique<Expression>(Expression{Value{2}}),
            std::make_unique<Expression>(Expression{Value{3}})
        }
    });

    std::cout << "Result: " << eval(*expr) << std::endl; // 출력: Result: 5
}
```

## 💡 Rust와 C++의 차이 요약

| 개념       | Rust 표현                          | C++ 대응 방식                                 |
|------------|-------------------------------------|-----------------------------------------------|
| 열거형     | `enum Expression`                   | `std::variant<Value, Op>`                     |
| 힙 포인터  | `Box<T>`                            | `std::unique_ptr<T>`                          |
| 패턴 매칭  | `match expr { ... }`                | `std::visit`, `std::holds_alternative`, `std::get` |
| 소유권     | 컴파일러가 강제 체크 (`move`, `borrow`) | 개발자가 직접 관리 (`move semantics`, `delete`) |
| 안전성     | null 불가, 수명 추적, drop 자동      | null 허용, 수동 관리, 실수 가능               |



---

# std::holds_alternative
std::holds_alternative<T>(variant)는 C++17의 std::variant에서 사용하는 함수로,  
variant가 현재 어떤 타입을 담고 있는지 확인하는 역할을 합니다.

## 🔍 핵심 역할
```cpp
std::variant<int, std::string> v = "hello";

if (std::holds_alternative<std::string>(v)) {
    std::cout << "v는 문자열입니다!" << std::endl;
}
```

→ holds_alternative<T>는 variant가 현재 T 타입을 담고 있는지 true/false로 알려줍니다.

## ✅ Rust의 match와 대응되는 부분
Rust에서는 match로 모든 variant를 분기 처리할 수 있음:
```rust
match expr {
    Expression::Value(v) => ...
    Expression::Op { ... } => ...
}
```

C++에서는 std::variant를 쓰면서 std::holds_alternative<T>로 먼저 타입을 확인하고,  
그 다음 std::get<T>(variant)로 값을 꺼내야 해요.
## 🧠 요약 표
| C++ 기능                  | 역할                                       |
|---------------------------|--------------------------------------------|
| std::holds_alternative<T> | 현재 variant가 T 타입인지 확인 (bool 반환) |
| std::get<T>(variant)      | T 타입일 경우 해당 값을 꺼냄               |


----








