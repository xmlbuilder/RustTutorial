# Curve Tangent Calculation Review
## ✅ 1. 전체 알고리즘 구조
- 이 함수는 다음 공식을 그대로 구현하고 있음:
- Homogeneous curve:
```math
C_w(u)=\sum _jN_{j,p}(u)P_j
```
```math
C'_w(u)=\sum _jN'_{j,p}(u)P_j
```
- Dehomogenization:
```rust
C(u)=\frac{C_w(u)}{w(u)}
```
- Derivative:
```math
C'(u)=\frac{C'_w(u)w-C_w(u)w'}{w^2}
```
- 코드의 마지막 부분이 정확히 이 식을 구현하고 있음:
```rust
(dx, dy, dz) = (cw1 * w - cw0 * w1) / w^2
```
- 수학적으로 완벽하게 맞다.

---
## 소스 코드
```rust
pub fn eval_tangent(&self, u: Real) -> Option<Vector3D> {
    let p = self.degree as usize;
    let n_ctrl = self.ctrl.len();
    if n_ctrl < p + 1 {
        return None;
    }

    let n = n_ctrl - 1;
    let u0 = self.kv.knots[p];
    let u1 = self.kv.knots[n + 1];
    let uu = u.clamp(u0, u1);

    let span = self.find_span(uu);
    let ders = on_ders_basis_funs(&self.kv.knots, span, uu, p, 1);
    let base = span - p;

    // A안: ctrl는 homogeneous (X,Y,Z,W)
    let mut cw0 = Point4D::default(); // Cw
    let mut cw1 = Point4D::default(); // Cw'
    for j in 0..=p {
        let cp = self.ctrl[base + j];
        let n0 = ders[0][j];
        let n1 = ders[1][j];
        cw0 = cw0 + (n0 * cp);
        cw1 = cw1 + (n1 * cp);
    }

    let w = cw0.w;
    let w1 = cw1.w;
    let denom = w * w;
    if denom.abs() < 1e-30 {
        return None;
    }
    // C' = (Cw' * w - Cw * w') / w^2
    let dx = (cw1.x * w - cw0.x * w1) / denom;
    let dy = (cw1.y * w - cw0.y * w1) / denom;
    let dz = (cw1.z * w - cw0.z * w1) / denom;
    Some(Vector3D::new(dx, dy, dz))
}
```

--- 
## 테스트 코드
```rust
/// 공통 검증: 끝점/반지름/접선
fn check_arc(as_deg: Real, ae_deg: Real, m_tol: Real) {
    let c = on_conic_arc_degree5_unit_tol(as_deg, ae_deg, m_tol).expect("arc build failed");

    assert_curve_basic_bezier5(c.degree as usize, c.ctrl.len(), c.kv.as_slice().len());

    // domain
    assert_close(c.domain.t0, 0.0, ON_TOL12, "domain t0");
    assert_close(c.domain.t1, 1.0, ON_TOL12, "domain t1");

    // endpoints
    let ps = on_point_on_unit_circle_deg(as_deg);
    let pe = on_point_on_unit_circle_deg(ae_deg);
    let p0 = c.eval_point(0.0);
    let p1 = c.eval_point(1.0);

    assert_pt_close(p0, ps, m_tol * 10.0, "start point mismatch");
    assert_pt_close(p1, pe, m_tol * 10.0, "end point mismatch");

    // tangents (direction)
    let ts_exp = on_tangent_on_unit_circle_deg(as_deg);
    let te_exp = on_tangent_on_unit_circle_deg(ae_deg);

    let ts = c.eval_tangent(0.0).expect("tangent at start failed");
    let te = c.eval_tangent(1.0).expect("tangent at end failed");

    assert_vec_same_dir(ts, ts_exp, 1e-8, "start tangent direction mismatch");
    assert_vec_same_dir(te, te_exp, 1e-8, "end tangent direction mismatch");

    // radius check: sample interior points should lie on unit circle (z=0, r=1)
    for &t in &[0.1, 0.25, 0.5, 0.75, 0.9] {
        let p = c.eval_point(t);
        assert_close(p.z, 0.0, m_tol * 100.0, "z should be 0 on unit circle plane");
        let r = (p.x * p.x + p.y * p.y).sqrt();
        assert_close(r, 1.0, m_tol * 100.0, "radius should be ~1");
    }
}
```
---
