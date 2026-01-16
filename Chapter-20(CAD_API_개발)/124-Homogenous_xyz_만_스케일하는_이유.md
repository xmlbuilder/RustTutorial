## 🎯 Homogenous xyz 
- scale_xyz = 좌표만 스케일(scale)하고 weight는 그대로 유지
- h_scale = 좌표와 weight 모두 스케일(scale)

## ✅ 두 함수의 차이
```rust
/// This arithmetic routine scales the first three coordinates of a
/// control point
pub fn scale_xyz(&self, s: f64) -> Self
{
    Self
    {
        x: self.x * s,
        y: self.y * s,
        z: self.z * s,
        w: self.w,
    }
}
```
```rust
#[inline]
pub fn h_scale(a: &Self, s: f64) -> Point4D {
    Point4D::non_homogeneous(a.x * s, a.y * s, a.z * s, a.w * s)
}
```

### 1) scale_xyz
- ✔ 의미
  - 기하학적 좌표(x,y,z)만 스케일
  - weight(w)는 절대 건드리지 않음
- ✔ 언제 쓰나?
  - 단순히 점의 위치만 스케일하고 싶을 때
  - weight는 shape에 영향을 주므로 건드리면 안 되는 상황
  - 예: 모델 전체를 2배 확대할 때

### 2) h_scale (scale control point + weight)
```
fn h_scale(self, s: Real) -> Point4D {
    Point4D {
        x: self.x * s,
        y: self.y * s,
        z: self.z * s,
        w: self.w * s,
    }
}
```

- ✔ 의미
  - 좌표(x,y,z)와 weight(w)를 모두 스케일
  - 즉, homogeneous coordinate 전체를 스케일
- ✔ 언제 쓰나?
  - 동차좌표(homogeneous coordinate) 자체를 스케일해야 할 때
- 예:
  - (wx, wy, wz, w) 전체를 어떤 factor로 조정해야 하는 경우
  - rational derivative 계산 중 weight까지 포함한 스케일이 필요한 경우
  - projective transform을 구현할 때

## 🔥 왜 weight를 스케일하면 안 되는 경우가 있을까?
- NURBS에서 weight는 곡선/곡면의 shape을 결정하는 핵심 요소.
  - weight를 바꾸면 곡선이 완전히 다른 모양이 된다.
  - 그래서 단순한 기하학적 스케일(scale)에서는 weight를 건드리면 안 된다.

## ✨ 요약 표
| 함수 | 스케일되는 요소 | 의미 |
|------|------------------|------|
| scale_xyz | x, y, z | 기하학적 좌표만 스케일 (weight 유지) |
| h_scale | x, y, z, w | homogeneous 좌표 전체 스케일 |

---




