# Option / Result
Rust에서 Option<T>과 Result<T, E>는 에러 없는 안전한 프로그래밍의 핵심.
오늘은 이 두 타입의 값 추출 방법, 좋은 사용법, 그리고 서로 변환하는 방법까지 깔끔하게 정리.

## ✅ 1. Option<T>에서 값 추출하는 방법
| 메서드               | 동작 조건      | 반환 타입                 | 설명                                      |
|-------------------------------|----------------|-------------------------|-------------------------------------------|
| .unwrap()             | Some             | T                      | None이면 panic 발생 (❌ 위험)              |
| .expect("msg")        | Some            | T                      | None이면 panic + 메시지 출력 (❌ 위험)     |
| .unwrap_or(default)   | 항상            | T                      | None이면 기본값 반환 (✅ 안전)             |
| .unwrap_or_else(||)  | 항상            | T                      | None이면 함수 실행해서 값 반환 (✅ 안전)   |
| .map(|v| ...)        | Some            | Option<U>              | 값에 함수 적용 (✅ 안전)                  |
| .and_then(|v| ...)   | Some            | Option<U>             | 체이닝 처리 (Option → Option) (✅ 안전)   |
| .ok_or(err)          | Some/None       | Result<T, E>         | None → Err(err)로 변환 (✅ 안전)           |
| .ok_or_else(|| err)  | Some/None       | Result<T, E>         | None → Err(err) (지연 생성) (✅ 안전)      |


## ✅ 2. Result<T, E>에서 값 추출하는 방법
| 메서드                  | 동작 조건     | 반환 타입         | 설명                                      |
|-------------------------|----------------|--------------------|-------------------------------------------|
| .unwrap()               | Ok             | T                  | Err이면 panic 발생 (❌ 위험)               |
| .expect("msg")          | Ok             | T                  | Err이면 panic + 메시지 출력 (❌ 위험)      |
| .unwrap_or(default)     | 항상           | T                  | Err이면 기본값 반환 (✅ 안전)              |
| .unwrap_or_else(|e| ...) | 항상          | T                  | Err이면 함수 실행해서 값 반환 (✅ 안전)    |
| .map(|v| ...)           | Ok             | Result<U, E>       | 값에 함수 적용 (✅ 안전)                  |
| .and_then(|v| ...)      | Ok             | Result<U, E>       | 체이닝 처리 (Result → Result) (✅ 안전)   |
| .ok()                   | Ok/Err         | Option<T>          | Err → None, Ok → Some (✅ 안전)            |
| .err()                  | Ok/Err         | Option<E>          | Ok → None, Err → Some (✅ 안전)            |


## 🔄 3. Option ↔ Result 변환 방법
| 변환 방향              | 메서드             | 설명                                 |
|------------------------|--------------------|--------------------------------------|
| Option<T> → Result<T,E> | .ok_or(err)        | None → Err(err)                      |
| Option<T> → Result<T,E> | .ok_or_else(||err) | None → Err(err) (지연 생성)          |
| Result<T,E> → Option<T> | .ok()              | Ok → Some, Err → None                |
| Result<T,E> → Option<E> | .err()             | Err → Some, Ok → None                |



## 💡 어떤 방식이 좋은가요?
| 방식                          | 안전성     | 추천 상황                          | 설명                                               |
|------------------------------|------------|------------------------------------|----------------------------------------------------|
| .expect("msg")               | ❌ 위험    | 테스트 중 빠르게 실패 확인         | None/Err이면 panic 발생, 디버깅에 유용              |
| .unwrap_or(...).map(...).and_then(...) | ✅ 안전 | 실전 코드에서 안전하게 처리       | 기본값, 체이닝, 함수 적용 등 유연한 흐름 제어 가능 |
| Result<T, String>            | ✅ 안전    | 오류 메시지를 명확히 전달할 때     | 실패 시 에러 메시지 포함, 로깅/UI에 적합            |
| Result<T, String>? (with ?)  | ✅ 매우 안전 | 함수 내부에서 오류 전파할 때       | `?` 연산자로 깔끔하게 상위로 오류 전달 가능        |


## ✨ 예시로 정리
```rust
fn get_value(opt: Option<f64>) -> Result<f64, String> {
    opt.ok_or("값이 없습니다".to_string())
}


fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}


let result: Result<f64, String> = safe_divide(10.0, 2.0).ok_or("0으로 나눌 수 없습니다".to_string());
```


## 💬 내 생각
Rust는 Option과 Result를 통해 panic 없는 안전한 흐름을 만들 수 있게 해줌.  
Result<T, String> 기반으로 처리하고, ? 연산자와 .map_err(...)를 활용하면  
에러 메시지까지 깔끔하게 전달할 수 있음.

# f64 -> Ok / Result

f64 값을 Result<f64, String>에 담아서 넘기는 방법은 아주 간단하지만,
언제 어떤 방식으로 감싸는지가 더 중요

## ✅ 기본 문법
```rust
let value: f64 = 10.0;
let result: Result<f64, String> = Ok(value);
```

→ Ok(...)로 감싸면 성공 결과
→ Err("에러 메시지".to_string())로 감싸면 실패 결과

## ✅ 함수에서 반환할 때
```rust
fn compute(x: f64, y: f64) -> Result<f64, String> {
    if x.is_nan() || y.is_nan() {
        return Err("입력값이 유효하지 않습니다".to_string());
    }
    Ok(x * 2.0 + y)
}
```

→ Result<f64, String>을 반환하는 함수
→ ? 연산자와 함께 쓰면 체이닝도 가능

## ✅ Option에서 f64를 Result로 변환할 때
```rust
let x_opt: Option<f64> = Some(3.0);
let x: Result<f64, String> = x_opt.ok_or("x가 없습니다".to_string());
```

→ Option<f64> → Result<f64, String> 변환
→ None이면 Err(...)로 처리

## ✅ DSL 클로저에서 넘길 때
```rust
let f = |vars: &HashMap<String, f64>| -> Result<f64, String> {
    let x = vars.get("x").copied().ok_or("x 누락")?;
    let y = vars.get("y").copied().unwrap_or(0.0);
    Ok(x * 2.0 + y)
};
```

→ Result<f64, String>을 반환하는 클로저
→ DSL registry에 등록 가능

## ✨ 정리
| 사용 방식                          | 설명                                               |
|-----------------------------------|----------------------------------------------------|
| Ok(10.0)                          | 성공 결과로 `f64` 값을 직접 감싸서 반환            |
| Err("에러 메시지".to_string())    | 실패 결과로 에러 메시지를 담아 반환                |
| .ok_or("에러")                    | `Option<f64>` → `Result<f64, String>` 변환          |
| fn → Result<f64, String>         | 함수에서 `Result` 타입으로 안전하게 반환           |

## 💡 예시 코드 스니펫
```rust
let value: Result<f64, String> = Ok(10.0);

let error: Result<f64, String> = Err("계산 실패".to_string());

let x_opt: Option<f64> = Some(3.0);

let x: Result<f64, String> = x_opt.ok_or("x가 없습니다".to_string());

fn compute(x: f64, y: f64) -> Result<f64, String> {
    if x.is_nan() {
        return Err("x가 유효하지 않음".to_string());
    }
    Ok(x * 2.0 + y)
}

let f = |vars: &HashMap<String, f64>| -> Result<f64, String> {
    let x = vars.get("x").copied().ok_or("x 누락")?;
    let y = vars.get("y").copied().unwrap_or(0.0);
    Ok(x * 2.0 + y)
};
```

## 💡 간단하게 다시 정리해볼게요
```
Some(T)  → Option<T>에서 성공
None     → Option<T>에서 실패

Ok(T)    → Result<T, E>에서 성공
Err(E)   → Result<T, E>에서 실패
```

- Option은 실패 이유를 말하지 않음 → 그냥 값이 없을 뿐
- Result는 실패 이유를 명확히 전달함 → Err("이유")

---



