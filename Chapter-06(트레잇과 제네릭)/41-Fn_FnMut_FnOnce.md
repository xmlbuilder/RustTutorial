# Borrow 문제시 Fn/FnMut/FnOnce 사용하기
- 이건 정말 많은 Rust 개발자들이 처음에 부딪히는 문제.
- Fn, FnMut, FnOnce를 람다(closure) 대신 일반 함수로 넘기면 borrow 문제가 훨씬 줄어 듬.
- 왜냐면 일반 함수는 캡처(capture)를 하지 않기 때문에 mutable borrow 충돌이 거의 없음.

## ✅ 1) Fn — 불변 호출자 (immutable closure/function)
- 일반 함수는 기본적으로 Fn을 만족한다.
```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```
```rust
fn apply_fn<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}
```
```rust
fn main() {
    let result = apply_fn(add_one, 10);
    println!("{}", result); // 11
}
```

- ✔ 함수는 캡처가 없으므로 Fn으로 자동 변환
- ✔ borrow 문제 없음

## ✅ 2) FnMut — 내부 상태를 변경하는 함수/객체
- 일반 함수는 상태를 가지지 않기 때문에 FnMut도 만족한다.
- 즉, 함수는 Fn + FnMut + FnOnce 모두 만족한다.
```rust
fn mul_two(x: i32) -> i32 {
    x * 2
}
```
```rust
fn apply_fn_mut<F>(mut f: F, value: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(value)
}
```
```rust
fn main() {
    let result = apply_fn_mut(mul_two, 5);
    println!("{}", result); // 10
}
```

- ✔ 함수는 캡처가 없으므로 FnMut도 OK
- ✔ mutable borrow 충돌 없음

## ✅ 3) FnOnce — 한 번만 호출 가능한 함수
- 일반 함수는 소유권을 소비하지 않기 때문에 FnOnce도 만족한다.
```rust
fn square(x: i32) -> i32 {
    x * x
}
```
```rust
fn apply_fn_once<F>(f: F, value: i32) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(value)
}
```
```rust
fn main() {
    let result = apply_fn_once(square, 4);
    println!("{}", result); // 16
}
```

- ✔ 함수는 FnOnce도 만족
- ✔ 소유권 문제 없음

## 🎯 정리: 일반 함수는 Fn, FnMut, FnOnce 모두 만족한다

| Trait   | 일반 함수가 만족? | 이유 |
|---------|-------------------|------|
| Fn      | Yes               | 캡처 없음 (불변 호출 가능) |
| FnMut   | Yes               | 내부 상태 없음 (mutable 호출 가능) |
| FnOnce  | Yes               | 소유권 소비 없음 (한 번 호출도 문제 없음) |


- 그래서 람다 대신 함수 포인터를 넘기면 borrow 충돌이 거의 사라진다.

## 🔥 추가: 함수 포인터 타입 명시도 가능
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```
```rust
fn apply(f: fn(i32, i32) -> i32) {
    println!("{}", f(3, 4));
}
```
```rust
fn main() {
    apply(add);
}
```


- 🌟 왜 closure는 borrow 문제가 생기고 함수는 안 생길까?
- closure는 환경을 캡처한다:
```rust
let mut count = 0;
let f = |x| { count += x; x };
```

- 이 closure는 count를 mutable로 캡처
- FnMut 또는 FnOnce가 됨
- borrow 충돌 발생 가능
- 반면 일반 함수는:
    - 캡처 없음
    - 상태 없음
    - borrow 없음
- 그래서 훨씬 안전하고 단순하다.

---
