# Closure에서 lifetime 문제 발생 시

```rust
    let mut A = vec![0.0; rows * cols];
    let a_ij = |r: usize, c: usize, A: &mut [f64]| -> &mut f64 {
        &mut A[r * cols + c]
    };
```
- 지금 에러는 클로저에서 &mut 참조를 반환하려고 해서 발생합니다.  
  Rust에서는 클로저가 반환하는 참조의 lifetime을 제대로 추론하기 어려워서 '1과 '2가 충돌.
- 이런 형태는 A의 lifetime ('1)과, 클로저 안에서 빌려서 리턴하는 참조의 lifetime ('2)을 
  Rust가 맞추지 못해서 에러를 냅니다.

##   🔧 해결 방법
### 1. 함수로 분리하기
- 클로저 대신 일반 함수로 작성하면 lifetime을 명시할 수 있습니다:
```rust
fn a_ij<'a>(r: usize, c: usize, A: &'a mut [f64], cols: usize) -> &'a mut f64 {
    &mut A[r * cols + c]
}
```
```rust
let mut A = vec![0.0; rows * cols];
*a_ij(1, 2, &mut A, cols) = 42.0;
```

- 여기서는 'a lifetime을 명시했기 때문에 컴파일러가 안전하게 추론할 수 있습니다.

### 2. 클로저를 쓰고 싶다면
클로저는 참조 반환에 적합하지 않으므로, 값을 직접 반환하거나 인덱스를 반환하는 방식으로 바꾸는 게 좋습니다:
```rust
let a_ij = |r: usize, c: usize| r * cols + c;

let mut A = vec![0.0; rows * cols];
A[a_ij(1, 2)] = 42.0;
```
- 이렇게 하면 클로저는 단순히 인덱스 계산만 하고, 실제 참조는 벡터에서 직접 얻습니다.

## ✅ 정리
- 클로저에서 &mut 반환 → lifetime 추론 불가 → 에러 발생
- 해결책
  - 일반 함수로 바꿔서 lifetime 명시
  - 클로저는 인덱스만 반환하고 참조는 벡터에서 직접 얻기


