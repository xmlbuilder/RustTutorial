# 정적 메서드 호출 방식

아래의 버그는 정적 메서드 호출 방시과 관련된 아주 미묘한 부분.

## 문제 코드
```rust
    #[inline]
    pub fn h_lerp(a: &Self, b: &Self, t: f64) -> Point4D {
        // (1-t)*a + t*b (homogeneous)
        h_add(&h_scale(a, 1.0 - t), &h_scale(b, t))
    }
```

## 🔍 문제 핵심
`h_add`, `h_scale`, `h_lerp`는 모두 impl Point4D 블록 안에 있는 정적 메서드.  
즉, `self`를 받지 않고 `&Self`를 인자로 받는 associated function입니다.  
그런데 `h_lerp` 안에서 `h_add(...)`라고만 쓰면,  
Rust는 현재 스코프에서 h_add라는 함수가 어디 있는지 모릅니다.  

## ✅ 해결 방법
### 1. Self::를 붙여서 호출해야 합니다
```rust
#[inline]
pub fn h_lerp(a: &Self, b: &Self, t: f64) -> Point4D {
    Self::h_add(Self::h_scale(a, 1.0 - t), Self::h_scale(b, t))
}

#[inline]
pub fn h_add(a: &Self, b: &Self) -> Point4D {
    Point4D::new(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w)
}

#[inline]
pub fn h_scale(a: &Self, s: f64) -> Point4D {
    Point4D::new(a.x * s, a.y * s, a.z * s, a.w * s)
}
```

→ 이렇게 하면 Rust는 h_add, h_scale이 현재 impl 블록의 메서드라는 걸 인식합니다.

## 💡 왜 이런 규칙이 있을까?
Rust는 함수 이름만으로는 전역 함수인지, 메서드인지, 다른 모듈에서 온 것인지 구분하지 못함.  
그래서 Self:: 또는 Point4D::처럼 명시적으로 경로를 지정해야 합니다.

## ✨ 보너스: 연산자 오버로딩으로 더 깔끔하게 만들기
```rust
use std::ops::{Add, Mul};

impl Add for Point4D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl Mul<f64> for Point4D {
    type Output = Self;
    fn mul(self, s: f64) -> Self::Output {
        Self::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }
}

→ 그럼 h_lerp는 이렇게 쓸 수 있어요:
pub fn h_lerp(a: &Self, b: &Self, t: f64) -> Point4D {
    (*a * (1.0 - t)) + (*b * t)
}
```



