# anyhow
Rust의  크레이트는 에러 처리의 복잡함을 단순화해주는 아주 강력한 도구.

## 🧠 What is anyhow?
anyhow는 애플리케이션 전반에서 발생하는 다양한 에러를 하나의 타입으로 통합해 처리할 수 있게 해주는 라이브러리입니다.
- anyhow::Error: 모든 에러를 담을 수 있는 범용 에러 타입
- anyhow::Result<T>: Result<T, anyhow::Error>의 타입 별칭
- ? 연산자와 함께 사용하면 자동으로 에러를 변환해줌
- Context를 통해 에러에 설명을 덧붙일 수 있음
- Backtrace를 지원하여 디버깅에 유용함

## ✅ 기본 사용법
```rust
use anyhow::Result;

fn main() -> Result<()> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    println!("cluster info: {:#?}", map);
    Ok(())
}
```


- main() 함수에서 Result<()>를 반환하면 에러 발생 시 자동으로 메시지 출력
- ? 연산자를 통해 에러를 간결하게 전파
- anyhow::Error는 다양한 에러 타입을 자동으로 래핑함

## 🧩 비교: anyhow::Result vs 일반 Result
| 반환 타입                    | 설명 또는 사용 목적                                 |
|-----------------------------|------------------------------------------------------|
| `Result<T, anyhow::Error>`  | 다양한 에러를 하나의 범용 타입으로 처리. 애플리케이션 전체에 적합 |
| `Result<T, CustomError>`    | 특정 에러 타입만 처리. 라이브러리 개발이나 정밀한 에러 제어에 적합 |

```rust
fn demo1() -> anyhow::Result<T> { ... } // 범용 에러 처리
fn demo2() -> Result<T, MyError> { ... } // 커스텀 에러 처리
```


## 🧠 Context로 에러 메시지 보강
```rust
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file at: {}", path))
}
```

- with_context()를 사용하면 에러 발생 시 추가 설명을 붙일 수 있음
- 디버깅 시 매우 유용: "파일을 못 읽음" → "config.json을 읽는 중 실패"

## 🔍 백트레이스 지원
- anyhow는 Rust 1.65 이상에서 자동으로 백트레이스를 캡처
- 환경 변수 설정으로 활성화 가능:
- RUST_BACKTRACE=1
- RUST_LIB_BACKTRACE=1

## 🛠️ 기타 기능
- anyhow!() 매크로로 즉시 에러 생성
```rust
return Err(anyhow!("Something went wrong"));
```
- bail!() 매크로로 빠른 반환
```rust
bail!("Invalid input");
```
- downcast_ref::<T>()로 원래 에러 타입 확인 가능

## 📦 언제 사용하면 좋을까?

| 사용 상황                  | 추천 여부 | 설명 또는 이유                                      |
|---------------------------|-----------|----------------------------------------------------|
| CLI 애플리케이션 개발      | ✅        | 다양한 에러를 간단하게 처리하고 사용자에게 메시지 출력 가능 |
| 내부 툴, 유틸리티 코드     | ✅        | 빠른 개발과 유지보수에 적합. 에러 타입 통합으로 코드 간결화 |
| 프로토타이핑 / 테스트 코드 | ✅        | 에러 타입 설계 없이 빠르게 기능 구현 가능           |
| 라이브러리 개발            | ❌        | 명확한 에러 타입이 필요하므로 `anyhow`는 부적합     |


---

## 🧠 백트레이스 캡처란?
- 백트레이스(backtrace): 프로그램이 실행되면서 호출한 함수들의 스택 추적 정보
- 캡처(capture): 에러가 발생한 순간의 스택 상태를 저장하는 것
- Rust에서는 anyhow나 std::backtrace를 통해 자동으로 캡처할 수 있음

## 🧩 anyhow에서의 백트레이스 캡처
- anyhow::Error는 내부적으로 백트레이스를 캡처할 수 있음
- Rust 1.65 이상에서는 기본적으로 백트레이스를 자동 캡처함
- 단, 출력하려면 환경 변수 설정이 필요함:
RUST_BACKTRACE=1           # 패닉과 에러 모두 백트레이스 출력
RUST_LIB_BACKTRACE=1       # 에러만 백트레이스 출력


- 에러 메시지 출력 시 다음과 같이 표시됨:
Error: Failed to read config file
Caused by: No such file or directory (os error 2)
Backtrace:
   0: my_app::read_config
   1: my_app::main



## ✅ 왜 중요한가?

| 상황 또는 기능             | 중요성 또는 역할                          |
|---------------------------|-------------------------------------------|
| 에러 발생 위치 추적       | 함수 호출 스택을 통해 원인 파악 가능       |
| 복잡한 호출 구조 분석     | 중첩된 함수 호출 흐름을 시각적으로 확인 가능 |
| 디버깅 효율 향상          | 빠르게 문제 지점을 찾고 수정 가능          |
| `Context`와 함께 사용 시  | 에러 메시지에 설명 + 위치 정보 추가 가능   |



## 🛠️ 실전 팁
- anyhow는 std::error::Error를 구현한 모든 에러 타입과 호환됨
- Context를 추가하면 백트레이스와 함께 의미 있는 메시지를 출력 가능
- Backtrace는 성능에 큰 영향을 주지 않음. 캡처는 빠르고, 심볼 해석만 느릴 수 있음

---
# bail

- bail!(...)은 return Err(anyhow!(...))와 동일한 역할을 합니다
- 함수의 반환 타입이 Result<T, anyhow::Error>일 때 사용 가능
- 에러 메시지를 생성하고 즉시 반환하여 빠르게 탈출할 수 있음

## ✅ 사용 예시
```rust
use anyhow::{Result, bail};

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        bail!("Division by zero");
    }
    Ok(a / b)
}
```

- b == 0일 경우 "Division by zero"라는 메시지를 담은 에러를 즉시 반환
- ? 연산자와 함께 사용하면 상위 함수로 에러가 전파됨

## 🧩 다양한 형태 지원
```rust
bail!("단순 메시지");
bail!("값이 잘못되었습니다: {}", value);
bail!(CustomError::InvalidInput);
```

- 문자열 리터럴
- 포맷 문자열
- 커스텀 에러 타입도 가능

## 🔍 실전 예제: 에러 전파
```rust
fn process_data(data: &str) -> Result<()> {
    if data.is_empty() {
        bail!("입력값이 비어 있습니다");
    }
    // 처리 로직...
    Ok(())
}

fn main() -> Result<()> {
    process_data("")?; // 에러 발생 시 자동 전파
    Ok(())
}
```

- bail!()은 에러 발생 지점에서 즉시 탈출할 수 있게 해줍니다
- ? 연산자와 함께 쓰면 에러 전파가 매우 간결해져요

## 📦 언제 쓰면 좋을까?
| 상황 또는 용도               | `bail!()` 사용 여부 / 설명                     |
|-----------------------------|-----------------------------------------------|
| 조건 검사 후 즉시 에러 반환  | ✅ 조건이 맞지 않을 때 빠르게 탈출 가능         |
| 에러 메시지에 포맷 문자열 사용 | ✅ `format!`처럼 동적 메시지 생성 가능           |
| 복잡한 로직 중 중단 지점 처리 | ✅ 코드 흐름을 깔끔하게 정리할 수 있음          |
| 애플리케이션 전체 에러 처리   | ✅ `anyhow`와 함께 사용하면 범용 에러 처리 가능  |
| 라이브러리 개발              | ❌ `thiserror`를 사용해 명확한 에러 타입 정의 권장 |



