# trait 가시성
trait 메서드 호출 방식과 가시성(visibility) 문제를 동시에 건드린 사례.


## 소스
```rust

trait PerpLike {
    fn perp_dot_like(a: &Self, b: &Self) -> f64;
}
impl PerpLike for Vector3D {
    #[inline]
    fn perp_dot_like(a: &Self, b: &Self) -> f64 {
        // |a × b|의 크기가 0에 가까우면 평행, 여기서는 수치 간단화를 위해
        // 세 축 성분 교차 조합의 합을 사용 (정확한 cross 대신).
        let c = Vector3D::cross(a, b);
        c.length()
    }
}


 if Vector3D::perp_dot_like(&u, &v).abs() < 1e-12 {
            // 시점 간 벡터도 같은 직선 위인지 검사
            let mut w = Vector3D::from_points(&s1.p0, &s2.p0);
            if w.length() < 1.0 { return true; }
            let _ = w.normalize();
            return Vector3D::perp_dot_like(&w, &v).abs() < 1e-12;
        }

```

## ❌ 에러 요약
- E0599: Vector3D::perp_dot_like()를 찾을 수 없음
- E0603: perp_dot_like는 private이라 접근 불가

## 🔍 원인 분석
### 🔸 문제 1: 정적 호출 방식
```rust
Vector3D::perp_dot_like(&w, &v)
```


이건 정적 메서드 호출 방식인데, perp_dot_like()는 trait PerpLike의 associated function이기 때문에
Rust에서는 이걸 직접 impl 타입에서 호출할 수 없어요.
trait 경유로 호출해야 합니다.
### 🔸 문제 2: 가시성 제한
```rust
trait PerpLike {
    fn perp_dot_like(a: &Self, b: &Self) -> f64;
}
```

이 trait이 pub이 아니면, 다른 모듈에서 perp_dot_like()를 호출할 수 없습니다.
즉, trait, impl, 그리고 fn 모두 public이어야 외부에서 접근 가능해요.

## ✅ 해결 방법
### 1. trait과 함수 모두 pub으로 선언
```rust
pub trait PerpLike {
    fn perp_dot_like(a: &Self, b: &Self) -> f64;
}

impl PerpLike for Vector3D {
    #[inline]
    fn perp_dot_like(a: &Self, b: &Self) -> f64 {
        let c = Vector3D::cross(a, b);
        c.length()
    }
}
```

🔑 pub trait, pub fn가 아니면 외부에서 호출 불가


### 2. trait 경유로 호출
```rust
use crate::geometry::math::vector3D::PerpLike;

if Vector3D::perp_dot_like(&u, &v).abs() < 1e-12{
    // ...
}
```
단, 이 경우 PerpLike trait이 in scope에 있어야 해요. use로 가져와야 함.

## 🧠 요약
| 에러  | 원인                          | 해결 방법                                                   |
|--------------|-------------------------------|-------------------------------------------------------------|
| `E0599`      | 정적 호출 방식 오류           | `w.perp_dot_like(&v)` 또는 `<Vector3D as PerpLike>::perp_dot_like()` |
| `E0603`      | 함수 또는 trait가 private     | `pub trait`, `pub fn`으로 공개 선언                         |


---
