# 🔍 #define 대안

## 🔁 C의 #define vs Rust의 대안

| 목적             | C 방식                          | Rust 방식                                                   |
|------------------|----------------------------------|-------------------------------------------------------------|
| 상수 정의        | `#define MAX 100`               | `const MAX: u32 = 100;`                                     |
| 조건부 컴파일    | `#ifdef DEBUG`                  | `#[cfg(debug_assertions)]`, `#[cfg(feature = "foo")]`       |
| 매크로 함수 정의 | `#define SQUARE(x) ((x)*(x))`   | `macro_rules! square { ($x:expr) => { $x * $x }; }`         |


## ✅ Rust에서 상수 정의
```rust
const MAX_SIZE: usize = 1024;
```

- 타입 명시가 필수 (usize, u32, 등)
- 컴파일 타임에 결정됨
- 전역 또는 모듈 내에서 선언 가능

## ✅ Rust 매크로 예시 (macro_rules!)
```rust
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    let a = square!(5);
    println!("{}", a); // 출력: 25
}
```


## ✅ 조건부 컴파일 (cfg)
```rust
#[cfg(debug_assertions)]
fn debug_log() {
    println!("디버그 모드입니다.");
}
```

### 또는 feature 기반:
```rust
#[cfg(feature = "fast-math")]
fn compute() {
    // 빠른 계산 로직
}
```

## 💬 결론
Rust는 `#define` 대신 타입 안전한 const, 강력한 매크로 시스템, 조건부 컴파일 속성을 제공합니다.  
전처리기 없이도 더 명확하고 안전하게 같은 기능을 구현할 수 있음.  

---



