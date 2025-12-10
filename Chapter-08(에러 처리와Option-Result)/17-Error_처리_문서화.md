## NurbsError 에러 처리 과정 문서화

### 1. 초기 상황

- NurbsError는 단순 enum으로 정의되어 있었음.
- 예: InvalidInput(String) 같은 튜플 스타일 variant.
- #[derive(Debug, Clone)]만 붙어 있었고, 컴파일 및 실행에 문제 없음.

### 2. 새 함수 추가 후 발생한 문제
```rust
pub fn on_gordon_surface_from_curve_network(...) -> Result<NurbsSurface> {
    if !(tu1 > tu0) {
        return Err(NurbsError::InvalidInput(
            "Gordon: no common u domain in v_family".to_string(),
        ));
    }
}
```

- 반환 타입을 Result<NurbsSurface>로만 선언.
- Rust의 Result<T, E>는 E 타입을 명시해야 하는데, 여기서는 생략됨.
- 따라서 Err(NurbsError::InvalidInput(...))와 타입 불일치가 발생하여 컴파일 에러.

### 3. 첫 번째 해결 방법

- 반환 타입을 명시적으로 작성:
```rust
pub fn on_gordon_surface_from_curve_network(...) -> Result<NurbsSurface, NurbsError> {
    // ...
}
```
- 이렇게 하면 Err(NurbsError::InvalidInput(...))와 타입이 일치하여 컴파일 성공.

### 4. 두 번째 문제 (thiserror 도입 후)

- 에러 메시지를 사람이 읽기 좋게 출력하기 위해 thiserror::Error를 도입.
- 기존 튜플 스타일 variant는 #[error("...")]에서 필드 이름을 쓸 수 없음.
- 예: InvalidInput(String)은 {0}으로 접근해야 하지만 thiserror는 지원하지 않음.

### 5. 해결 방법

- 구조체 스타일로 변경:
```rust
#[derive(Debug, thiserror::Error)]
pub enum NurbsError {
    #[error("invalid input: {msg}")]
    InvalidInput { msg: String },
    // ...
}
```
- 호출도 구조체 스타일로 수정:
```rust
return Err(NurbsError::InvalidInput { msg: "...".to_string() });
```

### 6. 최종 정리
- 첫 번째 실수: 함수 반환 타입을 Result<T>로만 선언하여 에러 타입 불일치 발생.
- 첫 번째 해결: 반환 타입을 Result<T, NurbsError>로 명시.
- 두 번째 실수: thiserror 도입 후에도 튜플 스타일 variant를 그대로 사용.
- 두 번째 해결: 구조체 스타일 variant로 변경하고 호출 방식도 수정.

### 7. 교훈
- Result<T>를 쓸 때는 항상 에러 타입 E를 명시해야 한다.
- thiserror를 사용할 때는 필드 이름 기반 포맷을 지원하므로, 구조체 스타일 variant가 필요하다.
- 정의 방식과 호출 방식이 반드시 일치해야 한다.

---

## thiserror / anyhow
- 지금은 thiserror만 사용해서 NurbsError라는 도메인 전용 에러 타입을 정의.
- 여기에 anyhow를 연동하면 라이브러리 내부에서는 NurbsError를 유지하면서, 외부(애플리케이션)에서는  
  anyhow::Result로 편하게 에러를 처리할 수 있습니다.

### 1. 현재 구조 (thiserror만 사용)
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NurbsError {
    #[error("invalid input: {msg}")]
    InvalidInput { msg: String },
    #[error("invalid size")]
    InvalidSize,
    // ...
}
```
```rust
pub fn build_surface(...) -> Result<NurbsSurface, NurbsError> {
    if n_samp_u < 2 {
        return Err(NurbsError::InvalidSize);
    }
    Ok(NurbsSurface {})
}
```
- 라이브러리 함수는 Result<T, NurbsError>를 반환.
- 호출자는 match로 직접 NurbsError를 처리.

### 2. anyhow 연동 방법
- 애플리케이션 코드에서는 anyhow::Result<T>를 반환 타입으로 씁니다.
- ? 연산자를 사용하면 NurbsError가 자동으로 anyhow::Error로 변환됩니다.  
  (thiserror::Error를 derive 했기 때문에 std::error::Error 트레이트가 구현되어 있고, anyhow가 이를 받아줍니다.)

```rust
use anyhow::Result;

fn main() -> Result<()> {
    let surface = build_surface(/* params */)?;
    println!("Surface built: {:?}", surface);
    Ok(())
}
```

### 3. 장점
- 라이브러리 내부: NurbsError로 도메인 에러를 명확히 유지 → 매칭, 디버깅, 테스트에 유리.
- 애플리케이션 외부: anyhow::Result로 통합된 에러 처리 → 여러 에러 타입을 한꺼번에 다루기 편리.
- 연동은 자동: 별도 변환 코드 필요 없음, ?만 쓰면 됨.

### 4. 정리
- 지금처럼 thiserror만 써도 충분히 동작합니다.
- anyhow를 쓰면 외부 코드에서 에러를 더 쉽게 전파하고 출력할 수 있습니다.
- 즉, 라이브러리 → NurbsError, **애플리케이션 → anyhow::Result** 라는 역할 분담이 가장 흔한 패턴입니다.

----


### 1. ?의 동작 원리
- ?는 단순히 Result<T, E>에서 Err(e)가 나오면 그 에러를 호출자에게 그대로 전파합니다.
- 이때 e가 어떤 메시지를 가지고 있는지는 에러 타입의 Display 구현에 달려 있습니다.

### 2. thiserror와 메시지
- thiserror를 붙이면 각 variant에 #[error("...")] 속성을 달 수 있습니다.
- 이 속성이 바로 Display 구현을 만들어 줍니다.
- 예:
```rust
#[derive(Debug, thiserror::Error)]
pub enum NurbsError {
    #[error("invalid input: {msg}")]
    InvalidInput { msg: String },
}
```
- 호출:
```rust
return Err(NurbsError::InvalidInput { msg: "샘플 부족".to_string() });
```
- 출력:
```rust
println!("{}", err);
```
  - invalid input: 샘플 부족

### 3. anyhow와 연동했을 때
- 함수가 Result<T, NurbsError>를 반환하면 ?로 전파할 때 그대로 NurbsError가 전달됩니다.
- 함수가 anyhow::Result<T>를 반환하면 ?는 내부적으로 NurbsError를 anyhow::Error로 변환합니다.
- 이때도 변환된 anyhow::Error는 내부적으로 NurbsError의 Display 메시지를 그대로 사용합니다.
- 즉, println!("{}", e) 하면 thiserror에서 정의한 메시지가 출력됩니다.

### 4. 정리
- ?는 에러 메시지를 새로 만드는 게 아니라, 에러 타입이 이미 가지고 있는 Display 구현을 그대로 사용합니다.
- thiserror를 쓰면 #[error("...")]에 적은 문자열이 그대로 메시지가 됩니다.
- anyhow를 쓰면 NurbsError가 anyhow::Error로 감싸지지만, 메시지는 여전히 thiserror에서 정의한 포맷을 따릅니다.
- 👉 ?만 던져도 에러 메시지가 잘 나오는 이유는, thiserror가 자동으로 Display를 구현해주기 때문이에요.

---
## thiserror
- Rust에서 ?로 에러를 전파할 때 메시지가 어떻게 보일지는 에러 타입의 Display 구현에 달려 있습니다.
- thiserror를 쓰면 그걸 자동으로 만들어주니까, 사실상 **에러 메시지를 다 구현해야 한다** 는 말은
  각 variant마다 #[error("...")]를 달아줘야 한다는 뜻.

### 샘플 코드
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NurbsError {
    #[error("invalid input: {msg}")]
    InvalidInput { msg: String },

    #[error("invalid size: expected at least 2, got {got}")]
    InvalidSize { got: usize },

    #[error("dimension mismatch: {msg}")]
    DimensionMismatch { msg: &'static str },
}
```
```rust
pub fn build_surface(n: usize) -> Result<(), NurbsError> {
    if n < 2 {
        return Err(NurbsError::InvalidSize { got: n });
    }
    Ok(())
}
```
```rust
fn main() {
    match build_surface(1) {
        Ok(_) => println!("Surface built"),
        Err(e) => {
            // Debug 출력
            println!("Debug: {:?}", e);
            // Display 출력 (thiserror가 자동 구현)
            println!("Display: {}", e);
        }
    }
}
```
### 출력 예시
```
Debug: InvalidSize { got: 1 }
Display: invalid size: expected at least 2, got 1
```

## 정리
- Debug는 Rust가 기본 제공 → 내부 구조 그대로 보여줌.
- Display는 thiserror가 #[error("...")]를 기반으로 자동 구현 → 사람이 읽기 좋은 메시지.
- 따라서 variant마다 #[error("...")]를 달아주면, ?로 전파된 에러도 자연스럽게 그 메시지를 출력합니다.

- Rust에서 에러 메시지를 사람이 읽기 좋게 만들려면 원래는 std::fmt::Display 트레이트를 직접 구현해야 합니다.
- 예를 들어:
```rust
impl std::fmt::Display for NurbsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NurbsError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
            NurbsError::InvalidSize => write!(f, "invalid size"),
            // ...
        }
    }
}
```

- 이렇게 하면 println!("{}", err)로 출력할 때 원하는 메시지가 나옴.

- 그런데 thiserror를 쓰면?
  - thiserror는 바로 그 Display 구현을 자동으로 만들어주는 매크로입니다.
  - 즉, 직접 impl Display를 작성할 필요 없이 variant마다 #[error("...")]를 붙여주면 됩니다:
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NurbsError {
    #[error("invalid input: {msg}")]
    InvalidInput { msg: String },

    #[error("invalid size")]
    InvalidSize,
}
```
- 이렇게 하면 내부적으로는 위에서 보여드린 impl Display가 자동 생성됩니다.
- 따라서 println!("{}", NurbsError::InvalidInput { msg: "oops".to_string() })라고 하면:
```
invalid input: oops
```
가 출력됩니다.

## 정리
- 직접 구현 방식: impl Display for NurbsError { ... }를 작성해야 함.
- thiserror 방식: 각 variant에 #[error("...")]만 붙이면 자동으로 Display 구현이 생김.
- 즉, #[error("invalid input: {msg}")]는 트레이트 구현을 대신해주는 선언이라고 이해하시면 됩니다.

---


