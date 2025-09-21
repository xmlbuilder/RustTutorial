# to_bits
Rust의 to_bits는 부동소수점(floating-point) 타입에서 자주 사용되는 메서드로, 해당 값을 정수형 비트 표현으로 변환해주는 기능.

## 🧠 to_bits란?
to_bits는 Rust의 f32 또는 f64 타입에서 사용 가능한 메서드로, IEEE 754 표준에 따라 부동소수점 값을 32비트 또는 64비트의 u32 또는 u64로 변환합니다.
```rust
let x: f64 = 3.141592;
let bits: u64 = x.to_bits();
println!("{:b}", bits); // 이진수 출력 4614256656552045848
```

## 🔍 어떤 상황에서 쓰이나요?
| 상황               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| NaN 비교           | `NaN`은 값 비교(`==`)로는 같지 않지만, `to_bits()`로 비트 비교 가능     |
| -0.0 vs +0.0       | 수학적으로는 같지만 비트 표현은 다르므로 `to_bits()`로 구분 가능         |
| 디버깅             | 부동소수점의 내부 비트 구조를 확인할 때 유용                           |
| 바이너리 직렬화    | 데이터를 비트 단위로 저장하거나 전송할 때 사용                          |
| 하드웨어 통신      | 비트 기반 프로토콜에서 float 값을 전송할 때 필요                        |


⚠️ 주의할 점
- to_bits는 부동소수점의 내부 표현을 그대로 반환하므로, 수학적 의미보다는 비트 패턴 자체에 집중.
- NaN 값은 여러 비트 패턴으로 표현될 수 있어서, == 비교 대신 to_bits()를 써야 정확하게 비교할 수 있음.
```rust
let a = f64::NAN;
let b = f64::NAN;
println!("{}", a == b); // false
println!("{}", a.to_bits() == b.to_bits()); // true or false (depends on NaN encoding)
```


## 🔁 반대로 비트를 다시 float으로?
Rust에서는 from_bits를 사용해서 다시 부동소수점으로 복원할 수 있음:
```rust
let bits: u64 = 4614256656552045848;
let x = f64::from_bits(bits);
println!("{}", x); // 3.141592
```


## 🧪 예제: f32 / f64 의 비트 표현 차이 확인
```rust
fn main() {
    let a: f32 = -0.0;
    let b: f32 = 0.0;

    println!("a == b: {}", a == b); // true
    println!("a.to_bits() == b.to_bits(): {}", a.to_bits() == b.to_bits()); // false
}
```
```rust
fn main() {
    let a: f64 = -0.0;
    let b: f64 = 0.0;

    println!("a == b: {}", a == b); // true
    println!("a.to_bits() == b.to_bits(): {}", a.to_bits() == b.to_bits()); // false
}
```

-0.0과 0.0은 수학적으로 같지만, 비트 표현은 다르기 때문에 to_bits()로 비교하면 차이를 알 수 있음.

---


