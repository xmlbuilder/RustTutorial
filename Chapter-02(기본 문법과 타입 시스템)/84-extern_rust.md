# extern "Rust"

## 📌 기본 개념
- extern "C": C 언어와의 ABI 호환을 위해 사용. 함수 호출 규약을 C 스타일로 맞춥니다.
- extern "Rust": Rust 고유의 ABI를 의미합니다.
- Rust 컴파일러가 내부적으로 함수 호출을 어떻게 처리하는지 나타내는 태그
- 외부에서 Rust 함수/변수를 가져올 때, Rust ABI를 그대로 사용하겠다는 뜻

## 📌 실제 쓰임새
```rust
extern "Rust" {
    static ETIP: [ElemTypeInfoParam; 32];
    fn some_internal_function(x: i32) -> i32;
}
```
- 의미: 이 블록 안의 심볼(ETIP, some_internal_function)은 다른 Rust 모듈/크레이트에서 정의된 것이고, Rust ABI로 연결됩니다.
- 차이점:
  - extern "C"는 C와 연결할 때 필요
  - extern "Rust"는 Rust끼리 연결할 때 필요 (보통은 pub static이나 pub fn으로 충분하지만, FFI 레벨에서 ABI를 명시할 수 있음)

## 📌 왜 쓰는가?
- 안전한 모듈 경계: Rust끼리도 ABI를 명시적으로 지정할 수 있어, 링커가 심볼을 정확히 연결합니다.
- FFI 통합: 어떤 경우에는 Rust 라이브러리를 다른 언어에서 호출할 때, Rust ABI를 그대로 노출하기도 합니다.
- 실험적/내부적: 사실 대부분의 경우 extern "Rust"는 잘 쓰이지 않고, extern "C"가 일반적입니다.
- "Rust"는 주로 컴파일러 내부나 특정 상황에서만 필요합니다.

## 📌 정리
- extern "Rust" = Rust ABI를 사용해서 외부 심볼을 가져오겠다는 선언
- 보통은 extern "C"를 많이 쓰고, "Rust"는 Rust 내부 모듈 간 연결이나 특수한 경우에만 사용
- 실제 프로젝트에서는 pub static / pub fn으로 충분한 경우가 많고, extern "Rust"는 ABI 레벨에서 더 명시적으로 쓰는 방법
- 👉 FEM/RTree 프로젝트에서는 사실 extern "Rust" 대신 pub static으로 두는 게 더 자연스러움.
- extern "Rust"는 **다른 곳에서 정의된 심볼을 가져온다** 는 의미라서, 같은 크레이트 안에서라면 필요 없음.

---

