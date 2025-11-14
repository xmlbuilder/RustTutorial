# FloatScalar

## 🧠 목적: 왜 FloatScalar가 필요한가?
Rust의 기본 부동소수 타입인 f32, f64는 각각 독립적인 타입이라서 제네릭 코드에서 공통 수학 연산을 추상화하기 어렵습니다.  
예를 들어, 다음과 같은 제네릭 함수는 기본적으로 작동하지 않습니다:
```rust
fn compute<T>(x: T) -> T {
    x.sqrt() + T::one()
}
```

왜냐하면 T가 어떤 타입인지 모르기 때문에 sqrt()나 one() 같은 메서드를 사용할 수 없음.  
그래서 FloatScalar 트레이트를 정의해서:
- sqrt, abs, max, min 같은 수학 연산을 추상화하고
- zero, one, from_f64 같은 유틸리티도 제공하며
- f32, f64에 대해 구현함으로써 제네릭 코드에서 사용할 수 있게 합니다.

## 🧩 구조 설명
### 🔷 트레이트 정의
```rust
pub trait FloatScalar: Copy + PartialEq + PartialOrd + Add + Sub + Mul + Div + Neg + AddAssign {
    fn zero() -> Self;
    fn one() -> Self;
    fn from_f64(x: f64) -> Self;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
}
```

- Copy, PartialEq, PartialOrd 등은 기본적인 연산을 위한 trait bound
- Add, Sub, Mul, Div, Neg, AddAssign은 수학 연산을 위한 trait bound
- zero, one, from_f64는 초기값 생성용
- sqrt, abs, max, min은 수학 함수
### 🔷 구현 예시: f32
```rust
impl FloatScalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn from_f64(x: f64) -> Self { x as f32 }
    fn sqrt(self) -> Self { f32::sqrt(self) }
    ...
}
```
- f32::sqrt(self)처럼 표준 라이브러리의 메서드를 그대로 래핑

## 🛠️ 어떻게 활용하나요?
### ✅ 제네릭 함수에서 사용
```rust
fn normalize<T: FloatScalar>(x: T, y: T) -> (T, T) {
    let length = (x * x + y * y).sqrt();
    (x / length, y / length)
}
```
- T가 FloatScalar를 구현했기 때문에 sqrt, zero, one 등을 자유롭게 사용 가능
- f32, f64 모두 지원됨
### ✅ 다양한 수학 연산을 추상화
```rust
fn clamp<T: FloatScalar>(value: T, min: T, max: T) -> T {
    value.max(min).min(max)
}
```
- max, min을 통해 범위 제한을 제네릭하게 처리

## 📌 FloatScalar 트레이트의 장점 요약

| 장점 | 설명 |
|------|------|
| ✅ f32, f64 모두 지원 | 동일한 제네릭 코드에서 `f32`와 `f64`를 자유롭게 사용할 수 있음 |
| ✅ 수학 연산 추상화 | `sqrt`, `abs`, `max`, `min` 등 부동소수점 연산을 트레이트로 통합 |
| ✅ 초기값 유틸리티 제공 | `zero()`, `one()`, `from_f64()` 등으로 초기값 생성이 간편 |
| ✅ 제네릭 함수 작성 가능 | 타입에 관계없이 수학적 로직을 재사용 가능 |
| ✅ 타입 안전성 향상 | 컴파일 시점에 타입 제약을 명확히 하여 오류 방지 |
| ✅ 확장 가능 | 필요 시 다른 수치 타입에도 `FloatScalar` 구현 가능 |


## 🧪 샘플 코드: 벡터 정규화
```rust
use std::ops::{Add, Div, Mul, Neg, Sub, AddAssign};

pub trait FloatScalar:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
{
    fn zero() -> Self;
    fn one() -> Self;
    fn from_f64(x: f64) -> Self;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
}
```
```rust
impl FloatScalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn from_f64(x: f64) -> Self { x as f32 }
    fn sqrt(self) -> Self { self.sqrt() }
    fn abs(self) -> Self { self.abs() }
    fn max(self, other: Self) -> Self { self.max(other) }
    fn min(self, other: Self) -> Self { self.min(other) }
}
```
```rust
impl FloatScalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn from_f64(x: f64) -> Self { x }
    fn sqrt(self) -> Self { self.sqrt() }
    fn abs(self) -> Self { self.abs() }
    fn max(self, other: Self) -> Self { self.max(other) }
    fn min(self, other: Self) -> Self { self.min(other) }
}
```
#[derive(Debug, Clone, Copy)]
struct Vec2<T: FloatScalar> {
    x: T,
    y: T,
}
```
```rust
impl<T: FloatScalar> Vec2<T> {
    fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Self {
        let len = self.length();
        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }
}
```

## ✅ 테스트 코드

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_f32() {
        let v = Vec2 { x: 3.0f32, y: 4.0f32 };
        let len = v.length();
        assert!((len - 5.0).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn test_normalize_f64() {
        let v = Vec2 { x: 0.0f64, y: 2.0f64 };
        let norm = v.normalize();
        assert!((norm.x - 0.0).abs() < 1e-12);
        assert!((norm.y - 1.0).abs() < 1e-12);
    }
}
```

## 📌 요약
- Vec2<T>는 FloatScalar를 제네릭으로 받아서 f32, f64 모두 지원
- length()는 벡터의 길이를 계산
- normalize()는 단위 벡터로 변환

---
