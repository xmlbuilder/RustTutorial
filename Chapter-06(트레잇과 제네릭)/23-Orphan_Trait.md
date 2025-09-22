# 오펜 트레이트 규칙

```rust
let mut vv =  Vector3D::new(1.0, 1.0, 1.0);
vv = 1.0 / vv; // ❌ 에러
```

## 문제 원인
### 1. 트레이트 구현 방향이 반대

Rust의 std::ops::Div 트레이트는 다음과 같이 동작합니다:
```rust
trait Div<RHS = Self> {
    type Output;
    fn div(self, rhs: RHS) -> Self::Output;
}
```

- 1.0 / vv는 f64 타입의 Div<Vector3D>를 찾습니다.
- 하지만 f64는 외부 타입이기 때문에 Rust에서는 해당 타입에 대해 트레이트를 직접 구현할 수 없습니다.

❌ Rust에서는 외부 타입에 대해 외부 트레이트를 구현하는 것을 금지합니다.  
이를 **오펜 트레이트 규칙 (orphan rule)**이라고 합니다.

### 2. 오펜 트레이트 규칙 (Orphan Rule)
Rust에서는 다음 조건을 만족해야만 트레이트를 구현할 수 있습니다:
- 트레이트가 현재 크레이트에 정의되어 있거나
- 타입이 현재 크레이트에 정의되어 있어야 함
#### 즉, impl Div<Vector3D> for f64는 f64도 외부 타입이고 Div도 외부 트레이트이므로 불가능합니다.

## ✅ 해결 방법
### 방법 1: Vector3D에 대해 Div<f64> 구현
```rust
impl Div<f64> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
```

이렇게 하면 vv / 1.0은 동작합니다.
### 방법 2: Vector3D에 대해 Div<f64>를 참조 타입으로도 구현
```rust
impl<'a> Div<f64> for &'a Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
```
이렇게 하면 &vv / 1.0도 동작합니다.

## 🔁 대안: inv() 메서드로 역수 처리
```rust
impl Vector3D {
    pub fn inv(&self) -> Vector3D {
        Vector3D::new(1.0 / self.x, 1.0 / self.y, 1.0 / self.z)
    }
}

let vv_inv = vv.inv();
```

이 방식은 의도도 명확하고 오펜 룰도 피할 수 있어 실무에서 더 선호됩니다.

## 📘 요약 테이블
| 구현 방식                   | 가능 여부 | 설명                                           | 예시                         |
|----------------------------|-----------|------------------------------------------------|------------------------------|
| `impl Div<Vector3D> for f64` | ❌        | 오펜 트레이트 규칙 위반 (외부 타입 + 외부 트레이트) | `1.0 / vv` → 컴파일 에러     |
| `impl Div<f64> for Vector3D` | ✅        | 내부 타입에 대해 연산자 오버로딩 가능              | `vv / 1.0`                   |
| `Vector3D::inv()`           | ✅        | 각 성분에 대해 역수 처리하는 명시적 메서드         | `vv.inv()` → `(1/x, 1/y, 1/z)` |

---


