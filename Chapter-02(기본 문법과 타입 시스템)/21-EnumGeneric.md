# Enum Generic 
Rust의 타입 시스템에서 가장 강력하고 유용한 기능 중 하나.  
특히 Option<T>와 Result<T, E>는 Rust의 안전성 철학을 대표하는 타입.

## 🧠 Enum + Generic이란?
Rust의 enum은 여러 가지 형태의 값을 표현할 수 있고, **Generic(제네릭)**을 사용하면 타입을 매개변수로 받아서 유연하게 동작할 수 있어요.
```rust
enum Option<T> {
    Some(T),
    None,
}
```

여기서 T는 아직 정해지지 않은 타입입니다.
사용할 때 Option<i32>, Option<String>처럼 구체적인 타입을 지정하게 됩니다.

## ✅ Option<T>: 값이 있을 수도, 없을 수도
정의
```rust
enum Option<T> {
    Some(T),
    None,
}
```

### 의미
| Variant     | 포함된 값 | 설명                          |
|-------------|------------|-------------------------------|
| `Some(T)`   | `T`        | 값이 존재함. `T` 타입의 실제 데이터 |
| `None`      | 없음       | 값이 없음. null-safe 표현       |



### 사용 예시
```rust
fn get_name(id: u32) -> Option<String> {
    if id == 1 {
        Some("JungHwan".to_string())
    } else {
        None
    }
}
```

- Option은 null을 대체하는 안전한 방식
- match, if let, unwrap_or 등으로 처리 가능

## ✅ Result<T, E>: 성공 또는 실패
정의
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 의미
| Variant     | 포함된 값 | 설명                          |
|-------------|------------|-------------------------------|
| `Ok(T)`     | `T`        | 성공 결과. `T` 타입의 실제 데이터 |
| `Err(E)`    | `E`        | 실패 원인. `E` 타입의 에러 정보   |


### 사용 예시
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

- Result는 에러 처리를 타입으로 표현함
- match, ?, unwrap_or_else 등으로 처리 가능

## 🔍 제네릭의 장점
| 장점 항목           | 설명                                                                 | 적용 예시         |
|--------------------|----------------------------------------------------------------------|-------------------|
| 타입 유연성         | 다양한 타입에 대해 하나의 구조로 처리 가능                             | `Option<T>`       |
| 코드 재사용성       | 동일한 로직을 여러 타입에 대해 재사용 가능                              | `Result<T, E>`    |
| 타입 안전성         | 컴파일 시 타입 체크로 오류 방지                                         | `Some(String)`    |
| null-safe 처리      | `Option`으로 값의 존재 여부를 안전하게 표현                              | `None` / `Some(T)`|
| 에러 처리 일관성    | `Result`로 성공/실패를 명확하게 구분 가능                               | `Ok(T)` / `Err(E)`|



## 🧪 실전 비교 예시
```rust
let name: Option<String> = Some("JungHwan".to_string());
let age: Option<u8> = None;

let result: Result<i32, String> = divide(10, 0);
match result {
    Ok(val) => println!("Result: {}", val),
    Err(e) => println!("Error: {}", e),
}
```

- Option<T>는 값이 있을 수도 없을 수도
- Result<T, E>는 성공일 수도 실패일 수도

## 🧠 핵심 정리
| 타입             | 정의                                | 목적                          |
|------------------|-------------------------------------|-------------------------------|
| `Option<T>`       | `Some(T)` / `None`                  | 값의 존재 여부 표현 (null-safe) |
| `Result<T, E>`    | `Ok(T)` / `Err(E)`                  | 성공/실패 표현 (에러 처리)     |
| 제네릭 사용 이유   | `T`, `E`로 타입을 유연하게 처리       | 다양한 타입에 대해 재사용 가능  |

---


