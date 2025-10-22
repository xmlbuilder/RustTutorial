# Statement / Expression

Rust는 표현식 중심(expression-oriented) 언어입니다.  
이 말은 대부분의 코드가 **값을 생성하는 식(expression)** 으로 구성된다는 뜻.  
반면, **명령문(statement)** 은 어떤 동작을 수행하지만 값을 반환하지 않음.

## 🧠 Statement vs Expression

| 구분        | 설명                                                                 | 세미콜론 `;` 여부 |
|-------------|----------------------------------------------------------------------|-------------------|
| Statement   | 어떤 동작을 수행하지만 값을 반환하지 않음. 변수 선언, 함수 호출 등     | 있음              |
| Expression  | 평가되어 값이 생성됨. 다른 식에 포함되거나 반환값으로 사용 가능         | 없음 (마지막 줄) |



## ✅ 예제 비교

### ❌ Statement를 값처럼 사용하면 오류
```rust
fn main() {
    let x = (let y = 6); // ❌ 컴파일 오류
}
```        

- `let y = 6`은 statement이므로 값을 반환하지 않음.
- 따라서 x에 바인딩할 수 있는 값이 없어 오류 발생.

## ✅ Expression 블록 사용
```rust
fn main() {
    let y = {
        let x = 3;     // statement
        5 + x          // expression → y에 8이 바인딩됨
    };

    println!("y = {y}"); // 출력: y = 8
}
```

- 중괄호 {}는 블록 표현식으로 간주됨.
- 마지막 줄 5 + x는 세미콜론 없이 표현식으로 평가되어 y에 저장됨.

## 📦 Statement vs Expression의 장단점

| 항목       | 장점                                                                 | 단점                                                                 |
|------------|----------------------------------------------------------------------|----------------------------------------------------------------------|
| Statement  | - 명령 수행에 명확함<br>- 변수 선언, 제어 흐름 등 구조적 역할        | - 값을 반환하지 않음<br>- 표현식처럼 사용할 수 없음                  |
| Expression | - 대부분의 코드가 값으로 평가됨<br>- 함수, 블록, 조건문 등에서 유연함 | - 세미콜론 실수 시 오류 발생 가능<br>- 초보자에게 개념이 혼동될 수 있음 |


## 🔍 Rust에서 표현식이 쓰이는 곳
- let x = { ... }; → 블록의 마지막 표현식이 x에 바인딩됨
- if, match, loop, while 등도 표현식으로 사용 가능
- 함수의 마지막 줄도 표현식으로 반환 가능
  
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // 표현식 반환
}
```
---


# 표현식 유용론

Rust에서는 let이 단순한 변수 선언을 넘어서 표현식(expression) 안에서도 유용하게 쓰일 수 있음.  
특히 if, match, loop, while 같은 제어 흐름 구조와 함께 쓰면 조건부 바인딩, 패턴 매칭, 스코프 제어 등 다양한 기능을 표현할 수 있음.

## 🧠 기본 개념: let은 표현식이다

Rust에서 let은 단순한 선언이 아니라 **표현식(expression)** 이 될 수 있음.  
즉, let PATTERN = VALUE 자체가 어떤 블록이나 흐름 안에서 조건부로 실행되거나 평가될 수 있다는 뜻.

### 🧩 if let — 조건부 패턴 매칭
```rust
if let Some(x) = maybe_value {
    println!("값은 {}", x);
} else {
    println!("값이 없음");
}
```

- Option, Result 같은 열거형(enum)을 다룰 때 유용.
- match보다 간결하게 특정 패턴만 처리할 수 있음.

### 🎯 while let — 반복 조건으로 패턴 매칭
```rust
let mut iter = vec![1, 2, 3].into_iter();

while let Some(x) = iter.next() {
    println!("다음 값: {}", x);
}
```

- 반복하면서 값이 존재할 때만 실행.
- Option을 반복적으로 처리할 때 깔끔함.

### 🔁 loop { let ... } — 무한 루프 내 바인딩
```rust
loop {
    let x = get_value();
    if x == 0 {
        break;
    }
    println!("받은 값: {}", x);
}
```

- 루프 내부에서 let을 사용해 매 반복마다 새로운 값을 바인딩.
- 스코프가 반복마다 초기화되므로 안전하게 사용 가능.

### 🎭 match let — 사실 `match` 자체가 `let` 을 포함하는 구조
```rust
match maybe_value {
    Some(x) => println!("값: {}", x),
    None => println!("없음"),
}
```

- match는 내부적으로 let PATTERN = VALUE를 평가하는 구조.
- if let은 match의 축약형이라고 볼 수 있어.

### 🧪 고급 예시: let을 표현식으로 사용하는 경우
```rust
let result = if let Some(x) = maybe_value {
    x * 2
} else {
    0
};
```

- if let 블록 자체가 값을 반환하는 표현식이므로 let result = ...처럼 사용할 수 있음.

## ✅ 정리: Rust 제어 흐름과 `let` 표현식 연동

| 제어 구조     | 주요 용도                  | 관련 타입 / 특징         |
|---------------|----------------------------|---------------------------|
| `if let`      | 조건부 패턴 매칭           | `match`의 축약형          |
| `while let`   | 반복 조건 패턴 매칭        | `Option`, `Result` 처리   |
| `loop + let`  | 루프 내 바인딩             | 스코프 분리, 반복 초기화  |
| `match`       | 모든 패턴 처리             | `let` 기반 구조           |
| `let`         | 바인딩 + 표현식 가능       | 조건부, 블록 반환 가능    |

---






