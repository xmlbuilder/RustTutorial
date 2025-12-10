## 에러 처리 문법 혼동 방지 체크리스트
- Rust에서 에러 처리 문법이 겹쳐 들어올 때 혼동을 피하기 위한 단계별 체크리스트입니다.

## 1. Result 반환 타입 확인
- Result<T>만 쓰면 에러 타입이 누락되어 컴파일 에러 발생.
- 항상 Result<T, E> 형태로 에러 타입을 명시.
- 예: Result<NurbsSurface, NurbsError>

## 2. Enum 정의 방식 확인
- 튜플 스타일: InvalidInput(String) → 호출은 InvalidInput("...").
- 구조체 스타일: InvalidInput { msg: String } → 호출은 InvalidInput { msg: "..." }.
- 정의와 호출 방식이 반드시 일치해야 함.

## 3. thiserror 사용 여부 확인
- thiserror를 쓰지 않으면 Debug 출력만 가능.
- thiserror를 쓰면 #[error("...")]로 Display 메시지를 자동 구현.
- 튜플 스타일은 {0} 인덱스 접근 불가 → 구조체 스타일로 바꿔야 함.

## 4. #[error("...")] 메시지 작성
- 각 variant마다 사람이 읽기 좋은 메시지를 작성.
- 구조체 스타일 필드 이름을 그대로 사용.
- 예: `#[error("invalid input: {msg}")] InvalidInput { msg: String }`

## 5. ? 연산자 동작 확인
- ?는 에러를 호출자에게 그대로 전파.
- 메시지는 에러 타입의 Display 구현을 그대로 사용.
- thiserror가 있으면 #[error("...")] 메시지가 출력됨.

## 6. anyhow 연동 여부 확인
- 라이브러리 내부: Result<T, NurbsError> 유지.
- 애플리케이션 외부: anyhow::Result<T> 사용.
- ? 연산자로 NurbsError가 자동 변환되어 메시지는 그대로 유지.

## 7. 최종 교훈
- 반환 타입을 항상 명시한다.
- enum 정의와 호출 방식을 일치시킨다.
- thiserror를 쓰면 구조체 스타일과 #[error("...")] 메시지를 활용한다.
- ?는 메시지를 새로 만드는 게 아니라 Display 구현을 그대로 전파한다.
- anyhow는 선택 사항이며, 외부 애플리케이션에서 에러 처리 편의성을 높여준다.

---
