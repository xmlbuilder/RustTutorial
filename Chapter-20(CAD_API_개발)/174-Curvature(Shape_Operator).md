
# Curvature 
## 🎯 1. 코드가 계산하는 것은 **곡률 텐서(Shape Operator)** 이다
- 즉, 기하학적 곡률을 계산하는 코드다.
- 이 코드는 Newton–Raphson에서 쓰는 **최적화용 Hessian** 과는 전혀 관계가 없다.

## 🎯 2. 전체 수학 구조 요약
- 곡면 S(u,v) 에 대해:
- 1) 1차 미분 (tangent vectors)
```math
S_u,\quad S_v
```
- 2) 단위 법선
```math
N=\frac{S_u\times S_v}{\| S_u\times S_v\| }
```
- 3) 1차 기본형식 (metric tensor)
```math
I=\left[ \begin{matrix}E&F\\ F&G\end{matrix}\right] =\left[ \begin{matrix}S_u\cdot S_u&S_u\cdot S_v\\ S_u\cdot S_v&S_v\cdot S_v\end{matrix}\right]
``` 
- 4) 2차 미분
```math
S_{uu},\quad S_{uv},\quad S_{vv}
```
- 5) 2차 기본형식 (curvature tensor numerator)
```math
II=\left[ \begin{matrix}L&M\\ M&N\end{matrix}\right] =\left[ \begin{matrix}\langle S_{uu},N\rangle &\langle S_{uv},N\rangle \\ \langle S_{uv},N\rangle &\langle S_{vv},N\rangle \end{matrix}\right]
``` 
- 6) Shape Operator (Weingarten matrix)
```math
W=I^{-1}II
```
- 이 행렬의 고유값이 바로:
    - k_1: 최대 주곡률
    - k_2: 최소 주곡률
- 그리고:
    - 평균곡률
    ```math
    H=\frac{k_1+k_2}{2}
    ```
    - 가우스곡률
    ```math
    K=k_1k_2
```

## 🎯 3. 네 Rust 코드가 정확히 계산하는 수식
- ✔ 1) 법선 계산
```rust
let n = (su × sv) / |su × sv|
```

- ✔ 2) 1차 기본형식
```rust
E = su·su
F = su·sv
G = sv·sv
denom = E*G - F*F
```

- 즉,
```math
I=\left[ \begin{matrix}E&F\\ F&G\end{matrix}\right]
``` 
- ✔ 3) 2차 기본형식
```rust
L = suu·n
M = suv·n
N = svv·n
```

- 즉,
```math
II=\left[ \begin{matrix}L&M\\ M&N\end{matrix}\right]
``` 
- ✔ 4) Gaussian curvature
```rust
K = (L*N - M*M) / (E*G - F*F)
```

- 수식 그대로:
```math
K=\frac{LN-M^2}{EG-F^2}
```
- ✔ 5) Mean curvature
```rust
H = 0.5 * (E*N - 2*F*M + G*L) / (E*G - F*F)
```

- 수식 그대로:
```math
H=\frac{EN-2FM+GL}{2(EG-F^2)}
```
- ✔ 6) Principal curvatures
```rust
disc = H*H - K
k1 = H + sqrt(disc)
k2 = H - sqrt(disc)
```

- 즉,
```math
k_{1,2}=H\pm \sqrt{H^2-K}
```
- ✔ 7) Weingarten matrix (Shape operator)
```math
S = I^{-1} II
```

- 코드에서:
```rust
s11 = (G*L - F*M) / denom
s12 = (G*M - F*N) / denom
s21 = (-F*L + E*M) / denom
s22 = (-F*M + E*N) / denom
```

- 이건 정확히:
```math
W=I^{-1}II
```
- ✔ 8) 고유벡터 → 주곡률 방향
```rust
eig = on_eigen2x2(W)
```

- 고유벡터를 (du, dv)로 얻고:
```rust
dir = v1*Su + v2*Sv
```

- 즉, tangent plane에서 3D 방향으로 lift.

## 🎯 4. 왜 Hessian과 헷갈리면 안 되는가?
- 둘 다 **2차 미분** 을 쓰지만 목적이 다르다.

| 구분 | 최적화용 Hessian | 곡률 텐서(Weingarten Matrix) |
|------|------------------|-------------------------------|
| 대상 함수 | 스칼라 함수 F(u,v) | 곡면 S(u,v) |
| 목적 | Newton–Raphson 수렴, 최소화 | 기하학적 곡률 계산 |
| 구성 요소 | $F_{uu}$, $F_{uv}$, $F_{vv}$ | $<S_{uu},N>$, $<S_{uv},N>$, $<S_{vv},N>$ |
| 의미 | 함수의 굽음(curvature of scalar field) | 곡면의 휘어짐(기하학적 curvature) |
| 결과 | 최적화 step, search direction | k1, k2, H, K, principal directions |
| 좌표계 | 파라미터 공간에서 스칼라 함수 | 접평면(tangent plane)에서 법선 변화율 |


- 즉,
    - Hessian은 최적화용이고, Weingarten은 기하학적 곡률용이다.


## 🎯 5. **두 개가 미묘하게 비슷해 보인다** 의 정확한 이유
- 둘 다 2차 미분을 사용한다
- 둘 다 2×2 행렬을 만든다
- 둘 다 고유값을 구한다
- 그래서 비슷해 보이지만…
    - 대상 함수가 다르기 때문에 완전히 다른 의미를 가진다.
    - Hessian → 스칼라 함수의 굽음
    - Weingarten → 곡면의 기하학적 곡률

## 🎯 6. 결론
- 코드는:
    - 1차/2차 기본형식
    - Weingarten matrix
    - Gaussian curvature
    - Mean curvature
    - Principal curvatures
    - Principal directions
- 을 정확히 계산하는 정석적인 곡률 텐서 구현이다.
- 이건 Hessian과는 완전히 다른 개념

---

## 소스 코드
```rust
use crate::core::cfun::NurbsSurface;
use crate::core::types::Real;
use crate::core::geom::{Vector3D};
use crate::core::math_extensions::on_eigen2x2;
use crate::core::maths::on_clamp_small_negative_to_zero;
```
```rust
#[derive(Debug, Clone, Copy)]
pub struct CurvatureProps {
    pub gaussian: Real,         // K
    pub mean: Real,             // H
    pub k1: Real,               // principal curvature 1
    pub k2: Real,               // principal curvature 2
    pub dir1: Vector3D,         // principal direction 1 (3D, unit)
    pub dir2: Vector3D,         // principal direction 2 (3D, unit)
}
```
```rust
/// 반환 None: 특이점(법선 불안정) 또는 metric 분모가 너무 작음 등.
pub fn on_curvature_properties_from_ders(
    su: Vector3D,
    sv: Vector3D,
    suu: Vector3D,
    suv: Vector3D,
    svv: Vector3D,
    // 선택: 외부에서 법선을 주고 싶으면 Some(N_unit)로 넣어도 됨
    n_unit_opt: Option<Vector3D>,
) -> Option<CurvatureProps> {
    // ====== 법선 ======
    let n = if let Some(nu) = n_unit_opt {
        nu
    } else {
        let nn = su.cross(&sv);
        let len = nn.length();
        if len <= 1e-15 {
            return None;
        }
        nn / len
    };

    // ====== 1차 기본형 (E,F,G) ======
    let e = su.dot(&su);
    let f = su.dot(&sv);
    let g = sv.dot(&sv);

    let denom = e * g - f * f;
    if denom.abs() <= 1e-24 {
        return None; // metric singular
    }
    let inv_denom = 1.0 / denom;

    // ====== 2차 기본형 (L,M,N) ======
    let l = suu.dot(&n);
    let m = suv.dot(&n);
    let n2 = svv.dot(&n);

    // ====== K, H ======
    let gaussian = (l * n2 - m * m) * inv_denom;
    let mean = 0.5 * (e * n2 - 2.0 * f * m + g * l) * inv_denom;

    // ====== principal curvatures ======
    let disc = mean * mean - gaussian;
    let disc = on_clamp_small_negative_to_zero(disc, 1e-14);
    let s = disc.sqrt();

    let k1 = mean + s;
    let k2 = mean - s;

    // ====== 주방향: Weingarten matrix S = I^{-1} II ======
    // I^{-1} = (1/denom) * [[ G, -F ], [ -F, E ]]
    // II = [[ L, M ], [ M, N ]]
    //
    // S = I^{-1} II
    //   = (1/den) * [[ G*L - F*M,  G*M - F*N ],
    //               [ -F*L + E*M, -F*M + E*N ]]
    let s11 = ( g * l - f * m) * inv_denom;
    let s12 = ( g * m - f * n2) * inv_denom;
    let s21 = (-f * l + e * m) * inv_denom;
    let s22 = (-f * m + e * n2) * inv_denom;

    // eigenvectors in (du,dv) space
    let eig = on_eigen2x2(s11, s12, s21, s22)?;
    // 고유값이 k1,k2와 순서가 다를 수 있으니 매칭
    let ((lam_a, va), (lam_b, vb)) = eig;

    let (v1_2d, v2_2d) = if (lam_a - k1).abs() <= (lam_a - k2).abs() {
        (va, vb)
    } else {
        (vb, va)
    };

    // lift to 3D: dir = a*Su + b*Sv
    let dir1 = (su * v1_2d.0 + sv * v1_2d.1).unitize();
    // dir2는 직교 보장 위해 cross를 쓰는 게 더 안정적
    // (eigenvector가 거의 같은 방향으로 붕괴하는 케이스 대비)
    let mut dir2 = n.cross(&dir1).unitize();

    // dir2가 너무 작으면(거의 0) fallback으로 직접 lift
    if dir2.length() <= 1e-15 {
        dir2 = (su * v2_2d.0 + sv * v2_2d.1).unitize();
    }

    Some(CurvatureProps {
        gaussian,
        mean,
        k1,
        k2,
        dir1,
        dir2,
    })
}
```
```rust
/// Surface 메서드 예시: eval_ders_nder(u,v,2) 연결
pub fn on_surface_curvature_properties(surface: &NurbsSurface, u: Real, v: Real) -> Option<CurvatureProps> {
    let ders = surface.eval_ders_nder(u, v, 2);
    // ders[0][0]=S, ders[1][0]=Su, ders[0][1]=Sv, ders[2][0]=Suu, ders[1][1]=Suv, ders[0][2]=Svv
    let su  = ders[1][0];
    let sv  = ders[0][1];
    let suu = ders[2][0];
    let suv = ders[1][1];
    let svv = ders[0][2];

    on_curvature_properties_from_ders(su, sv, suu, suv, svv, None)
}
```
---
### 테스트 코드
```rust
#[cfg(test)]
mod curvature_properties_tests {
    use nurbslib::core::curvature_props::on_curvature_properties_from_ders;
    use nurbslib::core::types::Real;
    use nurbslib::core::geom::Vector3D;

    // ---------------------------------------
    // helpers
    // ---------------------------------------
    fn v(x: Real, y: Real, z: Real) -> Vector3D {
        Vector3D { x, y, z }
    }

    fn assert_near(a: Real, b: Real, eps: Real, msg: &str) {
        let d = (a - b).abs();
        assert!(d <= eps, "{} |{}-{}|={}", msg, a, b, d);
    }

    fn assert_vec_near(a: Vector3D, b: Vector3D, eps: Real, msg: &str) {
        assert_near(a.x, b.x, eps, &format!("{}: x", msg));
        assert_near(a.y, b.y, eps, &format!("{}: y", msg));
        assert_near(a.z, b.z, eps, &format!("{}: z", msg));
    }

    fn assert_unit(vv: Vector3D, eps: Real, msg: &str) {
        let len = vv.length();
        assert!((len - 1.0).abs() <= eps, "{} |len-1|={} len={}", msg, (len - 1.0).abs(), len);
    }

    fn assert_perp(a: Vector3D, b: Vector3D, eps: Real, msg: &str) {
        let d = a.dot(&b).abs();
        assert!(d <= eps, "{} |dot|={} (dot={})", msg, d, a.dot(&b));
    }
```
```rust
    // ---------------------------------------
    // tests
    // ---------------------------------------

    #[test]
    fn curvature_plane_is_zero() {
        // S(u,v) = (u, v, 0)
        let su  = v(1.0, 0.0, 0.0);
        let sv  = v(0.0, 1.0, 0.0);
        let suu = v(0.0, 0.0, 0.0);
        let suv = v(0.0, 0.0, 0.0);
        let svv = v(0.0, 0.0, 0.0);
        let n   = v(0.0, 0.0, 1.0);

        let props = on_curvature_properties_from_ders(su, sv, suu, suv, svv, Some(n))
            .expect("plane should succeed");

        assert_near(props.gaussian, 0.0, 1e-12, "plane K");
        assert_near(props.mean,     0.0, 1e-12, "plane H");
        assert_near(props.k1,       0.0, 1e-12, "plane k1");
        assert_near(props.k2,       0.0, 1e-12, "plane k2");

        assert_unit(props.dir1, 1e-10, "plane dir1 unit");
        assert_unit(props.dir2, 1e-10, "plane dir2 unit");
        assert_perp(props.dir1, n, 1e-10, "plane dir1 ⟂ N");
        assert_perp(props.dir2, n, 1e-10, "plane dir2 ⟂ N");
        assert_perp(props.dir1, props.dir2, 1e-10, "plane dir1 ⟂ dir2");
    }
```
```rust
    #[test]
    fn curvature_sphere_at_north_pole_local_patch() {
        // Local Monge patch of sphere radius R at (0,0,R):
        // S(u,v) = (u, v, sqrt(R^2-u^2-v^2))
        // At (0,0): Su=(1,0,0), Sv=(0,1,0),
        // Suu=(0,0,-1/R), Suv=0, Svv=(0,0,-1/R), N=(0,0,1).
        let r = 2.5;
        let su  = v(1.0, 0.0, 0.0);
        let sv  = v(0.0, 1.0, 0.0);
        let suu = v(0.0, 0.0, -1.0 / r);
        let suv = v(0.0, 0.0,  0.0);
        let svv = v(0.0, 0.0, -1.0 / r);
        let n   = v(0.0, 0.0,  1.0);

        let props = on_curvature_properties_from_ders(su, sv, suu, suv, svv, Some(n))
            .expect("sphere patch should succeed");

        // K should be +1/R^2 (sign independent for outward vs inward normal)
        assert_near(props.gaussian, 1.0 / (r * r), 1e-12, "sphere K");
        // |H| should be 1/R (sign depends on normal convention)
        assert_near(props.mean.abs(), 1.0 / r, 1e-12, "sphere |H|");

        // principal curvatures should both be |1/R|
        assert_near(props.k1.abs(), 1.0 / r, 1e-12, "sphere |k1|");
        assert_near(props.k2.abs(), 1.0 / r, 1e-12, "sphere |k2|");

        assert_unit(props.dir1, 1e-10, "sphere dir1 unit");
        assert_unit(props.dir2, 1e-10, "sphere dir2 unit");
        assert_perp(props.dir1, n, 1e-10, "sphere dir1 ⟂ N");
        assert_perp(props.dir2, n, 1e-10, "sphere dir2 ⟂ N");
        assert_perp(props.dir1, props.dir2, 1e-10, "sphere dir1 ⟂ dir2");
    }
```
```rust
    #[test]
    fn curvature_cylinder_axis_point() {
        // Cylinder radius R: S(u,v)=(R cos u, R sin u, v)
        // At u=0:
        // Su=(0,R,0), Sv=(0,0,1)
        // Suu=(-R,0,0), Suv=0, Svv=0
        // N=(1,0,0) outward
        let r = 3.0;
        let su  = v(0.0, r, 0.0);
        let sv  = v(0.0, 0.0, 1.0);
        let suu = v(-r, 0.0, 0.0);
        let suv = v(0.0, 0.0, 0.0);
        let svv = v(0.0, 0.0, 0.0);
        let n   = v(1.0, 0.0, 0.0);

        let props = on_curvature_properties_from_ders(su, sv, suu, suv, svv, Some(n))
            .expect("cylinder should succeed");

        assert_near(props.gaussian, 0.0, 1e-12, "cylinder K");
        // mean should be -1/(2R) for this orientation (sign depends on normal)
        assert_near(props.mean, -1.0 / (2.0 * r), 1e-12, "cylinder H");

        // principal curvatures should be {0, -1/R} (order k1>=k2)
        assert_near(props.k1, 0.0, 1e-12, "cylinder k1");
        assert_near(props.k2, -1.0 / r, 1e-12, "cylinder k2");

        assert_unit(props.dir1, 1e-10, "cylinder dir1 unit");
        assert_unit(props.dir2, 1e-10, "cylinder dir2 unit");
        assert_perp(props.dir1, n, 1e-10, "cylinder dir1 ⟂ N");
        assert_perp(props.dir2, n, 1e-10, "cylinder dir2 ⟂ N");
        assert_perp(props.dir1, props.dir2, 1e-10, "cylinder dir1 ⟂ dir2");
    }
```
```rust
    #[test]
    fn curvature_saddle_at_origin() {
        // Hyperbolic paraboloid z = (x^2 - y^2)/(2a)
        // At origin:
        // Su=(1,0,0), Sv=(0,1,0)
        // Suu=(0,0,1/a), Suv=0, Svv=(0,0,-1/a), N=(0,0,1)
        let a = 4.0;
        let su  = v(1.0, 0.0, 0.0);
        let sv  = v(0.0, 1.0, 0.0);
        let suu = v(0.0, 0.0,  1.0 / a);
        let suv = v(0.0, 0.0,  0.0);
        let svv = v(0.0, 0.0, -1.0 / a);
        let n   = v(0.0, 0.0,  1.0);

        let props = on_curvature_properties_from_ders(su, sv, suu, suv, svv, Some(n))
            .expect("saddle should succeed");

        assert_near(props.gaussian, -1.0 / (a * a), 1e-12, "saddle K");
        assert_near(props.mean, 0.0, 1e-12, "saddle H");
        assert_near(props.k1,  1.0 / a, 1e-12, "saddle k1");
        assert_near(props.k2, -1.0 / a, 1e-12, "saddle k2");

        assert_unit(props.dir1, 1e-10, "saddle dir1 unit");
        assert_unit(props.dir2, 1e-10, "saddle dir2 unit");
        assert_perp(props.dir1, n, 1e-10, "saddle dir1 ⟂ N");
        assert_perp(props.dir2, n, 1e-10, "saddle dir2 ⟂ N");
        assert_perp(props.dir1, props.dir2, 1e-10, "saddle dir1 ⟂ dir2");
    }
```
```rust
    #[test]
    fn curvature_returns_none_on_singular_normal() {
        // Su and Sv parallel -> cross == 0 -> None
        let su  = v(1.0, 0.0, 0.0);
        let sv  = v(2.0, 0.0, 0.0);
        let suu = v(0.0, 0.0, 0.0);
        let suv = v(0.0, 0.0, 0.0);
        let svv = v(0.0, 0.0, 0.0);

        let props = on_curvature_properties_from_ders(su, sv, suu, suv, svv, None);
        assert!(props.is_none(), "expected None for singular normal");
    }
}
```
---
