# Power basis to NurbCurve
- power basis 와  동차 4D 계수:
```rust
Point4D::new(0.0, 0.0, 0.0, 1.0), // u^0
Point4D::new(1.0, 0.0, 0.0, 0.0), // u^1
Point4D::new(0.0, 1.0, 0.0, 0.0), // u^2
```

- 이를 수식으로 쓰면, 동차 좌표에서의 곡선 $\mathbf{P^{\mathnormal{h}}}(u)$ 는
```math
\mathbf{P^{\mathnormal{h}}}(u)=\sum _{i=0}^2\mathbf{a_{\mathnormal{i}}}u^i
```
- 여기서
- $\mathbf{a_{\mathnormal{0}}}=(0,0,0,1)$
- $\mathbf{a_{\mathnormal{1}}}=(1,0,0,0)$
- $\mathbf{a_{\mathnormal{2}}}=(0,1,0,0)$
- 이니까, 각 성분별로 보면:
- X 성분
```math
X(u)=0\cdot u^0+1\cdot u^1+0\cdot u^2=u
```
- Y 성분
```math
Y(u)=0\cdot u^0+0\cdot u^1+1\cdot u^2=u^2
```
- Z 성분
```math
Z(u)=0
```
- W 성분
```math
W(u)=1\cdot u^0+0\cdot u^1+0\cdot u^2=1
```
- 그러니까 동차 좌표 곡선은
```math
\mathbf{P^{\mathnormal{h}}}(u)=(u,\; u^2,\; 0,\; 1)
```
- 이고, 실제 3D(비동차) 곡선은
```math
\mathbf{P}(u)=\left( \frac{X(u)}{W(u)},\; \frac{Y(u)}{W(u)},\; \frac{Z(u)}{W(u)}\right) =(u,\; u^2,\; 0)
```
- 즉, 이 세 개의 Point4D는 다항식 곡선
```math
x(u)=u,\quad y(u)=u^2,\quad z(u)=0
```
- 을 나타내는 power basis + homogeneous 표현.
- 핵심 매칭 요약:
- 인덱스 i는 $u^i$ 의 차수
- Point4D::new($x_i$, $y_i$, $z_i$, $w_i$) 는 $u^i$ 항의 동차 계수 $(x_i,y_i,z_i,w_i)$
- 전체 곡선은
```math
(X(u),Y(u),Z(u),W(u))=\sum _i(x_i,y_i,z_i,w_i)u^i
```
- 비동차 곡선은 (X/W,Y/W,Z/W)
- 여기서는 W(u)=1 이라서 비유리(polynomial) 곡선이 정확히 $(u,u^2,0)$ 로 떨어지는 구조.

---

## 소스 코드
```rust

/// power basis → Bezier NURBS curve 변환
///
/// aw[i] = u^i 항의 4D homogeneous 계수 (i = 0..n)
/// 정의역: u ∈ [a,b]
/// 출력: degree=n Bezier NurbsCurve
pub fn on_power_basis_curve_to_nurbs(
    aw: &[Point4D],   // length = n+1
    n: Index,
    a: Real,
    b: Real,
) -> Result<NurbsCurve> {
    let n = n as usize;

    if aw.len() < n + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("aw length < n+1 (got {}, need {})", aw.len(), n + 1),
        });
    }
    if !(a < b) {
        return Err(NurbsError::InvalidArgument {
            msg: "invalid domain (a < b required)".into(),
        });
    }

    let p: Degree = n as Degree;

    // --- power basis → Bernstein control points ---
    // 각 성분(x,y,z,w)에 대해 1D 변환 수행
    let mut bx = vec![0.0; n + 1];
    let mut by = vec![0.0; n + 1];
    let mut bz = vec![0.0; n + 1];
    let mut bw = vec![0.0; n + 1];

    for comp in 0..4 {
        let mut a_coeff = vec![0.0; n + 1];
        for i in 0..=n {
            a_coeff[i] = match comp {
                0 => aw[i].x,
                1 => aw[i].y,
                2 => aw[i].z,
                _ => aw[i].w,
            };
        }

        // power basis → shifted/scaled power basis on [a,b]
        let c = on_shift_scale_power_basis(&a_coeff, a, b);

        // shifted power basis → Bernstein
        let b_vec = on_power_to_bernstein_1d(&c);

        for i in 0..=n {
            match comp {
                0 => bx[i] = b_vec[i],
                1 => by[i] = b_vec[i],
                2 => bz[i] = b_vec[i],
                _ => bw[i] = b_vec[i],
            }
        }
    }

    // --- control points ---
    let mut ctrl = Vec::<Point4D>::with_capacity(n + 1);
    for i in 0..=n {
        ctrl.push(Point4D::new(bx[i], by[i], bz[i], bw[i]));
    }

    // --- Bezier knot vector ---
    let mut ku = Vec::<Real>::with_capacity(2 * (p as usize + 1));
    for _ in 0..=p {
        ku.push(a);
    }
    for _ in 0..=p {
        ku.push(b);
    }

    // --- construct NurbsCurve ---
    let mut curve = NurbsCurve::from_rational_control_points(
        p,
        ctrl,
        KnotVector{knots: ku.clone()}
    ).expect("Invalid Nurbs Curve");

    curve.set_domain(a, b);

    Ok(curve)
}
```
---
### 테스트 코드 
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::nurbs_curve::on_power_basis_curve_to_nurbs;
    use nurbslib::core::prelude::{Point4D, Real};

    #[test]
    fn test_on_power_basis_curve_to_nurbs() {
        // 테스트할 power basis 곡선:
        // C(u) = (u, u^2, 0)
        //
        // x(u) = u → x0=0, x1=1
        // y(u) = u^2 → y0=0, y1=0, y2=1
        // z(u) = 0 → 모두 0
        // w(u) = 1 → 비유리 곡선 (w0=1, 나머지 0)

        let aw = vec![
            Point4D::new(0.0, 0.0, 0.0, 1.0), // u^0
            Point4D::new(1.0, 0.0, 0.0, 0.0), // u^1
            Point4D::new(0.0, 1.0, 0.0, 0.0), // u^2
        ];

        let curve = on_power_basis_curve_to_nurbs(&aw, 2, 0.0, 1.0)
            .expect("conversion failed");

        // degree 확인
        assert_eq!(curve.degree, 2);

        // control point 개수 확인 (Bezier → degree+1)
        assert_eq!(curve.ctrl.len(), 3);

        // knot vector 확인 (Bezier → [a,a,a,b,b,b])
        assert_eq!(curve.kv.knots, vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);

        // 몇 개의 점을 평가해서 원래 다항식과 일치하는지 확인
        let check = |u: Real| {
            let p = curve.eval_point(u);
            let expected_x = u;
            let expected_y = u * u;

            assert!((p.x - expected_x).abs() < 1e-9);
            assert!((p.y - expected_y).abs() < 1e-9);
            assert!((p.z - 0.0).abs() < 1e-9);
        };

        check(0.0);
        check(0.25);
        check(0.5);
        check(0.75);
        check(1.0);
    }
}
```
---
