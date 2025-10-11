# Rust 디버깅
## 🧠 Rust 디버깅에 도움되는 기능 정리
### ✅ 1. 포인터 주소 출력 – {:p}
- {:p}는 포인터의 메모리 주소를 출력하는 포맷입니다.
- 예시:
```rust
let x = 42;
let ptr = &x;
println!("주소: {:p}", ptr); // 주소: 0x7ffee4c2aabc
```
메모리 위치를 직접 확인할 수 있어 참조/소유권 문제 디버깅에 유용


### ✅ 2. #[derive(Debug)] + {:?} 출력
- 구조체나 열거형을 디버깅용으로 출력할 수 있게 해주는 기본 기능
- 예시:
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 7 };
    println!("{:?}", p); // Point { x: 3, y: 7 }
}
```

Debug 트레이트를 자동 구현해주며, 내부 상태를 쉽게 확인 가능


### ✅ 3. dbg!() 매크로
- 디버깅 중 변수 값을 빠르게 출력할 수 있는 매크로
- 출력에는 파일명, 줄 번호, 값까지 포함됨
```rust
fn main() {
    let a = 10;
    let b = dbg!(a * 2); // [src/main.rs:3] a * 2 = 20
}
```

디버깅 중 println보다 더 많은 정보를 자동으로 제공


### ✅ 4. std::mem::size_of::<T>()
- 타입의 메모리 크기를 확인할 수 있음
```rust
println!("i32 크기: {}", std::mem::size_of::<i32>()); // 4
```

구조체나 enum의 메모리 최적화 확인에 유용


### ✅ 5. cargo expand
- 매크로가 확장된 실제 코드를 보여주는 도구
- 설치: `cargo install cargo-expand`
- 사용: `cargo expand`
```
#[derive], macro_rules!, procedural macro 등이 실제로 어떤 코드로 변환되는지 확인 가능
```

### ✅ 6. cargo-asm / cargo llvm-ir
- Rust 함수가 어셈블리나 LLVM IR로 어떻게 컴파일되는지 확인 가능
- 설치: `cargo install cargo-asm`
- 사용: `cargo asm crate_name::function_name`
성능 최적화나 low-level 디버깅에 매우 유용


### ✅ 7. RUST_BACKTRACE=1
- 패닉 발생 시 스택 트레이스를 출력해주는 환경 변수
```
RUST_BACKTRACE=1 cargo run
```

에러 발생 위치와 호출 경로를 추적할 수 있어 디버깅에 필수


### ✅ 8. cargo test -- --nocapture
- 테스트 중 println! 출력이 보이지 않을 때 사용
- --nocapture 옵션으로 테스트 중 출력 확인 가능
```
cargo test -- --nocapture
```
## 🧩 핵심 요약 – Rust 디버깅 도구

| 기능               | 설명                                      | 사용 목적                     |
|--------------------|-------------------------------------------|-------------------------------|
| `{:p}`             | 포인터 주소 출력                          | 메모리 참조 확인              |
| `#[derive(Debug)]` | 구조체/열거형 디버깅 출력 가능             | 내부 상태 확인                |
| `dbg!()`           | 값 + 위치 자동 출력                        | 빠른 디버깅                   |
| `size_of`          | 타입의 메모리 크기 확인                    | 최적화 및 구조 분석           |
| `cargo expand`     | 매크로 확장 결과 확인                      | 코드 이해 및 디버깅           |
| `cargo-asm`        | 어셈블리 코드 확인                         | 성능 분석 및 low-level 디버깅 |
| `RUST_BACKTRACE`   | 패닉 시 스택 트레이스 출력                 | 에러 추적                     |
| `--nocapture`      | 테스트 중 출력 확인                        | 테스트 디버깅                 |

---


