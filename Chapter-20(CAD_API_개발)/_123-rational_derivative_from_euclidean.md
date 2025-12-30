# rational_derivative_from_euclidean

## 🎯 이 함수의 의미 — “왜 이런 계산을 하는가?”
- 이 함수는 유리 좌표(homogeneous coordinate)의 도함수를 계산하는 함수.
- NURBS는 control point를 이렇게 저장하지?
```math
Pw=(xw,\, yw,\, zw,\, w)
```
- 즉, 좌표에 weight가 곱해진 형태로 저장.

- ✔ Euclidean derivative D = (dx, dy, dz) 는 “기하학적 도함수”
- 즉,
```math
D=\frac{d}{dt}(x,y,z)
```
- 하지만 NURBS는 실제로는 (w·x, w·y, w·z, w) 를 사용하므로
- 우리가 필요한 건:
```math
\frac{d}{dt}(wx),\quad \frac{d}{dt}(wy),\quad \frac{d}{dt}(wz),\quad \frac{dw}{dt}
```
- ✔ 그래서 A_ratder는 다음을 계산한다
```math
Dw=(wd\cdot x+dx\cdot w,\; wd\cdot y+dy\cdot w,\; wd\cdot z+dz\cdot w,\; wd)
```

- 이건 미분의 곱셈 법칙(product rule) 그대로:
```math
\frac{d}{dt}(wx)=w'x+wx'
```
## 🔥 왜 이게 중요할까?
- NURBS 곡선/곡면의 도함수는 다음 형태로 계산:
```math
C'(u)=\frac{(wP)'-w'P}{w^2}
```      
- 여기서
  - (wP)' = homogeneous derivative
  - P' = Euclidean derivative
  - w' = weight derivative
- 즉, NURBS 도함수 계산의 핵심 중 핵심이 바로 이 함수.

🧠 직관적으로 설명하면
- NURBS는 w가 곱해진 좌표를 사용한다
- 도함수를 계산하려면 w가 곱해진 좌표의 도함수가 필요하다
- 그래서 product rule을 적용해서
```math
(wx)'=w'x+wx'
```
- 이런 형태가 나온다
- 이걸 3D + weight까지 확장한 것이 A_ratder


## ✨ 요약

| 항목 | 의미 |
|------|------|
| Pw   | weight가 곱해진 control point |
| D    | Euclidean derivative (기하학적 도함수) |
| wd   | weight의 도함수 |
| Dw   | homogeneous derivative (NURBS 도함수 계산에 필수) |


---
## 소스 코드
```rust
/// Compute rational derivative from Euclidean derivative.
/// Pw = (xw, yw, zw, w)
/// D  = (dx, dy, dz)  (Euclidean derivative)
/// wd = dw/dt
///
/// Returns Dw = (d(wx)/dt, d(wy)/dt, d(wz)/dt, dw/dt)
pub fn on_rational_derivative_from_euclidean(
    pw: &Point4D,
    d: &Vector3D,
    wd: f64,
) -> Point4D {
    // Convert homogeneous Pw = (xw, yw, zw, w) → Euclidean P = (x, y, z)
    let w = pw.w;
    let p = pw.to_euclidean(); // (x, y, z)

    // Compute rational derivative
    let dx = wd * p.x + d.x * w;
    let dy = wd * p.y + d.y * w;
    let dz = wd * p.z + d.z * w;
    Point4D { x: dx, y: dy, z: dz, w: wd }
}
```

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::geom::{on_rational_derivative_from_euclidean, Point4D, Vector3D};

    #[test]
    fn test_rational_derivative_from_euclidean() {
        // Pw = (xw, yw, zw, w)
        // Euclidean point = (xw/w, yw/w, zw/w)
        let pw = Point4D { x: 2.0, y: 4.0, z: 6.0, w: 2.0 };
        // Euclidean P = (1, 2, 3)

        // Euclidean derivative
        let d = Vector3D { x: 10.0, y: 20.0, z: 30.0 };

        // weight derivative
        let wd = 5.0;

        // Compute rational derivative
        let dw = on_rational_derivative_from_euclidean(&pw, &d, wd);

        // Expected:
        // dx = wd*x + dx*w = 5*1 + 10*2 = 5 + 20 = 25
        // dy = wd*y + dy*w = 5*2 + 20*2 = 10 + 40 = 50
        // dz = wd*z + dz*w = 5*3 + 30*2 = 15 + 60 = 75
        // w' = wd = 5

        assert!((dw.x - 25.0).abs() < 1e-12);
        assert!((dw.y - 50.0).abs() < 1e-12);
        assert!((dw.z - 75.0).abs() < 1e-12);
        assert!((dw.w - 5.0).abs() < 1e-12);
    }
}
```
---

