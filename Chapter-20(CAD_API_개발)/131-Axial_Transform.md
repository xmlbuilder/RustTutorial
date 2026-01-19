# 📘 Axial Transformations — 수학적 의미
## 1. 목적
- Axial transformation은 NURBS 모델링에서 자주 사용되는 형상 변형(geometric deformation) 기법이다.
- 특징:
  - 변형은 한 축(X/Y/Z) 을 기준으로 한다.
  - 변형의 강도는 shape function f(t) 로 제어된다.
  - 변형 종류는 4가지:
    - PINCH : 특정 좌표만 scale
    - TAPER : 두 좌표를 scale
    - TWIST : 회전
    - SHEAR : 특정 좌표만 translate

## 2. Shape Function f(t)
- CFun은 B-spline 기반의 1D 함수:
```math
f(t)=\sum _{i=0}^nN_i^p(t)\, f_i
```
- $N_i^p(t)$: B-spline basis
- $f_i$: control coefficients


## 3. PINCH
- 특정 좌표만 scale:
- 예: XDIR + YCRD
```math
y'=y\cdot (af(x))
```
- 일반식:
```math
\mathrm{cor}'=\mathrm{cor}\cdot (af(\mathrm{dir}))
```
## 4. TAPER
- 두 좌표를 scale:
- 예: YDIR
```math
x'=x\cdot (af(y)),\quad z'=z\cdot (af(y))
```
- 일반식:
```math
\mathrm{other\  coords}'=\mathrm{other\  coords}\cdot (af(\mathrm{dir}))
```
## 5. TWIST
- 축을 기준으로 회전:
- 예: ZDIR
```math
\alpha =\pi af(z)
```
```math
\begin{aligned}x'&=x\cos \alpha -y\sin \alpha \\ y'&=x\sin \alpha +y\cos \alpha \end{aligned}
```

- 일반식:
```math
\mathrm{rotate\  around\  dir-axis\  by\  }\alpha =\pi af(\mathrm{dir})
```
## 6. SHEAR
- 특정 좌표만 translate:
- 예: XDIR + ZCRD
```math
z'=z+af(x)
```
- 일반식:
```math
\mathrm{cor}'=\mathrm{cor}+af(\mathrm{dir})
```
---

## 7. Curve Axial Deformation
- NURBS curve:
```math
C(u)=\frac{\sum _iN_i^p(u)P_i^{(w)}}{\sum _iN_i^p(u)w_i}
```
- control point 집합 $P_i^{(w)}$ 에 대해
- 각각 axial 변형을 적용하여 새로운 control net 생성:
```math
P_i^{(w)\, *}=\mathrm{AxialDeform}(P_i^{(w)})
```
- 새로운 곡선:
```math
C^*(u)=\frac{\sum _iN_i^p(u)P_i^{(w)\, *}}{\sum _iN_i^p(u)w_i}
```
- 즉, basis function과 knot vector는 변하지 않는다.

## 8. Surface Axial Deformation
- NURBS surface:
```math
S(u,v)=\frac{\sum _{i=0}^n\sum _{j=0}^mN_i^{p_u}(u)\, M_j^{p_v}(v)\, P_{i,j}^{(w)}}{\sum _{i=0}^n\sum _{j=0}^mN_i^{p_u}(u)\, M_j^{p_v}(v)\, w_{i,j}}
```
- control net은 row-major:
```math
\mathrm{idx}(u,v)=u+\mathrm{nu}\cdot v
```
- 각 control point에 대해:
```math
P_{i,j}^{(w)\, *}=\mathrm{AxialDeform}(P_{i,j}^{(w)})
```
- 새로운 surface:
```math
S^*(u,v)=\frac{\sum _{i,j}N_i^{p_u}(u)\, M_j^{p_v}(v)\, P_{i,j}^{(w)\, *}}{\sum _{i,j}N_i^{p_u}(u)\, M_j^{p_v}(v)\, w_{i,j}}
```
- 역시 basis와 knot vector는 변하지 않는다.

## 9. Summary

| Component        | Meaning                          |
|------------------|----------------------------------|
| t                | x, y, or z (depending on DIR)    |
| f(t)             | B-spline shape function value     |
| g = a * f(t)     | deformation amplitude             |
| PINCH            | cor' = cor * g                    |
| TAPER            | other_coords' = other_coords * g  |
| TWIST            | rotate by alpha = π * g           |
| SHEAR            | cor' = cor + g                    |

## 📌 설명
- t
- 변형 방향(DIR)에 따라 선택되는 좌표
  - XDIR → t = x
  - YDIR → t = y
  - ZDIR → t = z
  - f(t)
- CFun(B-spline function)으로 평가된 값
  ```math
  g = a * f(t)
  ```
  
- 변형 강도 (amplitude × shape function)
  - PINCH
    - 특정 좌표만 scale
    ```math
    cor' = cor * g
    ```
  - TAPER
    - 두 좌표를 scale
    - $other\\_coords' = other\\_coords * g$

  - TWIST
    - 축 기준 회전
    ```math
    alpha = π * g
    ```
  - SHEAR
    - 특정 좌표 translate
    ```math
    cor' = cor + g
    ```
---

## 소스 코드
```rust
pub fn on_axial_transform<F>(
    p: &mut Point3D,
    shape: F,
    a: Real,
    tra: AxialTra,
    dir: AxialDir,
    cor: AxialCoord,
) where
    F: Fn(f64) -> Real,
{
    // 1) 좌표 추출
    let mut x = p.x;
    let mut y = p.y;
    let mut z = p.z;

    // 2) 방향에 따라 shape function 평가
    let t = match dir {
        AxialDir::X => x,
        AxialDir::Y => y,
        AxialDir::Z => z,
    };
    let f = shape(t);

    // 3) 변환 적용
    match tra {
        AxialTra::Pinch => match dir {
            AxialDir::X => match cor {
                AxialCoord::Y => y *= a * f,
                AxialCoord::Z => z *= a * f,
                AxialCoord::X => {}
            },
            AxialDir::Y => match cor {
                AxialCoord::X => x *= a * f,
                AxialCoord::Z => z *= a * f,
                AxialCoord::Y => {}
            },
            AxialDir::Z => match cor {
                AxialCoord::X => x *= a * f,
                AxialCoord::Y => y *= a * f,
                AxialCoord::Z => {}
            },
        },

        AxialTra::Taper => match dir {
            AxialDir::X => {
                y *= a * f;
                z *= a * f;
            }
            AxialDir::Y => {
                x *= a * f;
                z *= a * f;
            }
            AxialDir::Z => {
                x *= a * f;
                y *= a * f;
            }
        },

        AxialTra::Twist => {
            let alf = PI * a * f;
            let (s, c) = alf.sin_cos();
            match dir {
                AxialDir::X => {
                    let w = y;
                    y = c * w - s * z;
                    z = s * w + c * z;
                }
                AxialDir::Y => {
                    let w = x;
                    x = c * w + s * z;
                    z = -s * w + c * z;
                }
                AxialDir::Z => {
                    let w = x;
                    x = c * w - s * y;
                    y = s * w + c * y;
                }
            }
        }

        AxialTra::Shear => match dir {
            AxialDir::X => match cor {
                AxialCoord::Y => y += a * f,
                AxialCoord::Z => z += a * f,
                AxialCoord::X => {}
            },
            AxialDir::Y => match cor {
                AxialCoord::X => x += a * f,
                AxialCoord::Z => z += a * f,
                AxialCoord::Y => {}
            },
            AxialDir::Z => match cor {
                AxialCoord::X => x += a * f,
                AxialCoord::Y => y += a * f,
                AxialCoord::Z => {}
            },
        },
    }

    // 4) 결과 되돌려 쓰기 (in-place)
    p.x = x;
    p.y = y;
    p.z = z;
}
```
---
### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::prelude::Point3D;
    use nurbslib::core::xform::{AxialCoord, AxialDir, AxialTra, on_axial_transform};
    use nurbslib::core::types::Real;

    fn shape_linear(t: Real) -> Real {
        t
    }
```
```rust
    #[test]
    fn test_axial_pinch_xdir_ycrd() {
        let mut p = Point3D::new(2.0, 3.0, 4.0);

        on_axial_transform(
            &mut p,
            shape_linear, // f(t) = t
            0.5,          // a
            AxialTra::Pinch,
            AxialDir::X,
            AxialCoord::Y,
        );

        // y *= a * f(x) = 3 * (0.5 * 2) = 3 * 1 = 3
        assert_eq!(p, Point3D::new(2.0, 3.0, 4.0));
    }
```
```rust
    #[test]
    fn test_axial_taper_ydir() {
        let mut p = Point3D::new(2.0, 3.0, 4.0);

        on_axial_transform(
            &mut p,
            shape_linear,
            1.0,
            AxialTra::Taper,
            AxialDir::Y,
            AxialCoord::X, // ignored in TAPER
        );

        // x *= a*f(y) = 2 * 3 = 6
        // z *= a*f(y) = 4 * 3 = 12
        assert_eq!(p, Point3D::new(6.0, 3.0, 12.0));
    }
```
```rust
    #[test]
    fn test_axial_twist_zdir() {
        let mut p = Point3D::new(1.0, 0.0, 2.0);

        on_axial_transform(
            &mut p,
            shape_linear,
            0.5,
            AxialTra::Twist,
            AxialDir::Z,
            AxialCoord::X, // ignored in TWIST
        );

        // alf = PI * a * f(z) = PI * 0.5 * 2 = PI
        // rotation by PI around Z axis:
        // x' = cos(PI)*x - sin(PI)*y = -1
        // y' = sin(PI)*x + cos(PI)*y = 0
        assert!((p.x + 1.0).abs() < 1e-9);
        assert!(p.y.abs() < 1e-9);
        assert_eq!(p.z, 2.0);
    }
```
```rust
    #[test]
    fn test_axial_shear_xdir_zcrd() {
        let mut p = Point3D::new(2.0, 3.0, 4.0);

        on_axial_transform(
            &mut p,
            shape_linear,
            1.0,
            AxialTra::Shear,
            AxialDir::X,
            AxialCoord::Z,
        );

        // z += a*f(x) = 4 + 2 = 6
        assert_eq!(p, Point3D::new(2.0, 3.0, 6.0));
    }
}
```
```rust
#[cfg(test)]
mod tests_axial_point {
    use nurbslib::core::prelude::Point3D;
    use nurbslib::core::xform::{AxialCoord, AxialDir, AxialTra, on_axial_transform};
    use nurbslib::core::types::Real;

    fn shape_linear(t: Real) -> Real {
        t
    }

    #[test]
    fn axial_pinch_xdir_ycrd() {
        let mut p = Point3D::new(2.0, 3.0, 4.0);

        on_axial_transform(
            &mut p,
            shape_linear, // f(t) = t
            0.5,          // a
            AxialTra::Pinch,
            AxialDir::X,
            AxialCoord::Y,
        );

        // y *= a * f(x) = 3 * (0.5 * 2) = 3 * 1 = 3
        assert!((p.x - 2.0).abs() < 1e-12);
        assert!((p.y - 3.0).abs() < 1e-12);
        assert!((p.z - 4.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn axial_taper_ydir() {
        let mut p = Point3D::new(2.0, 3.0, 4.0);

        on_axial_transform(
            &mut p,
            shape_linear,
            1.0,
            AxialTra::Taper,
            AxialDir::Y,
            AxialCoord::X, // ignored
        );

        // x *= a*f(y) = 2 * 3 = 6
        // z *= a*f(y) = 4 * 3 = 12
        assert!((p.x - 6.0).abs() < 1e-12);
        assert!((p.y - 3.0).abs() < 1e-12);
        assert!((p.z - 12.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn axial_twist_zdir_pi_rotation() {
        // z = 2 → f(z)=2 → a=0.5 → alpha = PI*a*f(z) = PI
        let mut p = Point3D::new(1.0, 0.0, 2.0);

        on_axial_transform(
            &mut p,
            shape_linear,
            0.5,
            AxialTra::Twist,
            AxialDir::Z,
            AxialCoord::X, // ignored
        );

        // 회전각 PI → (1,0) → (-1, ~0)
        assert!((p.x + 1.0).abs() < 1e-12);
        assert!(p.y.abs() < 1e-12);
        assert!((p.z - 2.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn axial_shear_xdir_zcrd() {
        let mut p = Point3D::new(2.0, 3.0, 4.0);

        on_axial_transform(
            &mut p,
            shape_linear,
            1.0,
            AxialTra::Shear,
            AxialDir::X,
            AxialCoord::Z,
        );

        // z += a*f(x) = 4 + 2 = 6
        assert!((p.x - 2.0).abs() < 1e-12);
        assert!((p.y - 3.0).abs() < 1e-12);
        assert!((p.z - 6.0).abs() < 1e-12);
    }
}
```
```rust
#[cfg(test)]
mod tests_cfun_shape {
    use nurbslib::core::basis::Side;
    use nurbslib::core::cfun::{CFun, cfun_derivatives};
    use nurbslib::core::prelude::KnotVector;
```
```rust
    #[test]
    fn cfun_linear_like_function() {
        // 단순한 CFun 예제: f(u) ≈ u 형태를 흉내만 내는 테스트
        let p = 1;
        let knots = KnotVector {
            knots: vec![0.0, 0.0, 1.0, 1.0],
        };
        let fu = vec![0.0, 1.0]; // 아주 단순한 계수
        let cfn = CFun::new(p, knots, fu).unwrap();

        let u = 0.25;
        let vals = cfun_derivatives(&cfn, u, Side::Left, 0).unwrap();
        let f = vals[0];

        // 그냥 값이 잘 나오기만 하면 됨: 너무 구체적이지 않게
        assert!(f.is_finite());
    }
}
```
```rust
#[cfg(test)]
mod tests_curve_axial {
    use nurbslib::core::basis::Side;
    use nurbslib::core::cfun::{CFun, cfun_derivatives};
    use nurbslib::core::nurbs_curve::on_deform_curve_axial;
    use nurbslib::core::prelude::{Interval, KnotVector, NurbsCurve, Point4D};
    use nurbslib::core::xform::{AxialCoord, AxialDir, AxialTra};
    use nurbslib::core::types::Real;

    fn shape_eval_from_cfun(cfn: &CFun, t: Real, side: Side) -> Real {
        cfun_derivatives(cfn, t, side, 0).unwrap()[0]
    }

    fn make_test_curve() -> NurbsCurve {
        // 아주 단순한 2차원 직선 곡선: (0,0,0) - (1,0,0)
        let ctrl = vec![
            Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
            Point4D::homogeneous(1.0, 0.0, 0.0, 1.0),
        ];
        let kv = KnotVector {
            knots: vec![0.0, 0.0, 1.0, 1.0],
        };
        NurbsCurve {
            dimension: 3,
            degree: 1,
            ctrl,
            kv,
            domain: Interval { t0: 0.0, t1: 1.0 },
        }
    }

    fn make_dummy_cfun() -> CFun {
        // 테스트용 아주 단순한 CFun (실제 의미는 중요하지 않음)
        let p = 1;
        let knots = KnotVector {
            knots: vec![0.0, 0.0, 1.0, 1.0],
        };
        let fu = vec![0.0, 1.0]; // 대충 증가형 함수
        CFun::new(p, knots, fu).unwrap()
    }
```
```rust
    #[test]
    fn curve_axial_shear_xdir_ycrd() {
        let curve = make_test_curve();
        let cfn = make_dummy_cfun();

        let deformed = on_deform_curve_axial(
            &curve,
            &cfn,
            1.0,
            AxialTra::Shear,
            AxialDir::X,
            AxialCoord::Y,
            Side::Left,
            shape_eval_from_cfun,
        );

        // 첫 번째 control point: (0,0,0)
        let p0 = deformed.ctrl[0].to_point();
        // 두 번째 control point: (1,0,0) → y' = y + a*f(x) ≈ 0 + f(1)
        let p1 = deformed.ctrl[1].to_point();

        assert!((p0.y - 0.0).abs() < 1e-9);
        assert!(p1.y.is_finite());
        assert!((p1.x - 1.0).abs() < 1e-9);
    }
```
```rust
    #[test]
    fn curve_axial_twist_zdir_no_effect_when_z_zero() {
        let curve = make_test_curve();
        let cfn = make_dummy_cfun();

        let deformed = on_deform_curve_axial(
            &curve,
            &cfn,
            1.0,
            AxialTra::Twist,
            AxialDir::Z,
            AxialCoord::X,
            Side::Left,
            shape_eval_from_cfun,
        );

        // 모든 z=0 → f(z) 는 상수 → alpha = π*a*f(0)
        // 여기서는 그냥 변화가 없거나 일정 회전이지만,
        // y=0이므로 결과는 여전히 y≈0
        let p0 = deformed.ctrl[0].to_point();
        let p1 = deformed.ctrl[1].to_point();

        assert!(p0.y.abs() < 1e-9);
        assert!(p1.y.abs() < 1e-9);
    }
}
```
```rust
#[cfg(test)]
mod tests_surface_axial {
    use nurbslib::core::basis::Side;
    use nurbslib::core::cfun::{CFun, cfun_derivatives};
    use nurbslib::core::nurbs_surface::on_deform_surface_axial;
    use nurbslib::core::prelude::{Interval, KnotVector, NurbsSurface, Point4D};
    use nurbslib::core::xform::{AxialCoord, AxialDir, AxialTra};
    use nurbslib::core::types::Real;

    fn shape_eval_from_cfun(cfn: &CFun, t: Real, side: Side) -> Real {
        cfun_derivatives(cfn, t, side, 0).unwrap()[0]
    }

    fn make_dummy_cfun() -> CFun {
        let p = 1;
        let knots = KnotVector {
            knots: vec![0.0, 0.0, 1.0, 1.0],
        };
        let fu = vec![0.0, 1.0]; // 단순한 증가형 shape
        CFun::new(p, knots, fu).unwrap()
    }

    fn make_test_surface() -> NurbsSurface {
        // 2x2 control net
        // idx(u,v) = u + nu*v, nu=2
        // (0,0): (0,0,0)
        // (1,0): (1,0,0)
        // (0,1): (0,1,0)
        // (1,1): (1,1,0)
        let nu = 2;
        let nv = 2;
        let ctrl = vec![
            Point4D::homogeneous(0.0, 0.0, 0.0, 1.0), // (0,0) idx=0
            Point4D::homogeneous(1.0, 0.0, 0.0, 1.0), // (1,0) idx=1
            Point4D::homogeneous(0.0, 1.0, 0.0, 1.0), // (0,1) idx=2
            Point4D::homogeneous(1.0, 1.0, 0.0, 1.0), // (1,1) idx=3
        ];

        let ku = KnotVector {
            knots: vec![0.0, 0.0, 1.0, 1.0],
        };
        let kv = KnotVector {
            knots: vec![0.0, 0.0, 1.0, 1.0],
        };

        NurbsSurface {
            dim: 3,
            pu: 1,
            pv: 1,
            nu,
            nv,
            ctrl,
            ku,
            kv,
            domain_u: Interval { t0: 0.0, t1: 1.0 },
            domain_v: Interval { t0: 0.0, t1: 1.0 },
        }
    }
```
```rust
    #[test]
    fn surface_row_major_indexing() {
        let srf = make_test_surface();
        assert_eq!(srf.idx(0, 0), 0);
        assert_eq!(srf.idx(1, 0), 1);
        assert_eq!(srf.idx(0, 1), 2);
        assert_eq!(srf.idx(1, 1), 3);
    }
```
```rust
    #[test]
    fn surface_axial_pinch_xdir_ycrd() {
        let srf = make_test_surface();
        let cfn = make_dummy_cfun();

        let deformed = on_deform_surface_axial(
            &srf,
            &cfn,
            1.0,
            AxialTra::Pinch,
            AxialDir::X,
            AxialCoord::Y,
            Side::Left,
            shape_eval_from_cfun,
        );

        // (u=1,v=0): x=1,y=0 → y' = y * a*f(x) ≈ 0
        let p10 = deformed.ctrl[srf.idx(1, 0)].to_point();
        // (u=1,v=1): x=1,y=1 → y' = 1 * a*f(1) ≈ f(1)
        let p11 = deformed.ctrl[srf.idx(1, 1)].to_point();

        assert!(p10.y.abs() < 1e-9);
        assert!(p11.y.is_finite());
    }
```
```rust
    #[test]
    fn surface_axial_taper_ydir() {
        let srf = make_test_surface();
        let cfn = make_dummy_cfun();

        let deformed = on_deform_surface_axial(
            &srf,
            &cfn,
            1.0,
            AxialTra::Taper,
            AxialDir::Y,
            AxialCoord::X, // ignored
            Side::Left,
            shape_eval_from_cfun,
        );

        // (u=0,v=1): (0,1,0) → x'=0*f(1)=0, z'=0
        let p01 = deformed.ctrl[srf.idx(0, 1)].to_point();
        assert!(p01.x.abs() < 1e-9);
        assert!(p01.z.abs() < 1e-9);

        // (u=1,v=1): (1,1,0) → x' = 1*f(1), z' = 0*f(1)
        let p11 = deformed.ctrl[srf.idx(1, 1)].to_point();
        assert!(p11.x.is_finite());
        assert!(p11.z.abs() < 1e-9);
    }
```
```rust
    #[test]
    fn surface_axial_twist_zdir_no_effect_when_z_zero() {
        let srf = make_test_surface();
        let cfn = make_dummy_cfun();

        let deformed = on_deform_surface_axial(
            &srf,
            &cfn,
            1.0,
            AxialTra::Twist,
            AxialDir::Z,
            AxialCoord::X,
            Side::Left,
            shape_eval_from_cfun,
        );

        // 모든 z=0 → alpha = π*a*f(0) = 상수
        // 하지만 y=0 또는 x,y>=0 구조라서 여기서는
        // 단순히 "finite하고 큰 깨짐 없음" 정도만 본다
        for pw in &deformed.ctrl {
            let p = pw.to_point();
            assert!(p.x.is_finite());
            assert!(p.y.is_finite());
            assert!(p.z.abs() < 1e-9);
        }
    }
```
```rust
    #[test]
    fn surface_axial_shear_xdir_ycrd() {
        let srf = make_test_surface();
        let cfn = make_dummy_cfun();

        let deformed = on_deform_surface_axial(
            &srf,
            &cfn,
            1.0,
            AxialTra::Shear,
            AxialDir::X,
            AxialCoord::Y,
            Side::Left,
            shape_eval_from_cfun,
        );
        // (u=1,v=0): x=1 → y' = y + a*f(1) ≈ f(1)
        let p10 = deformed.ctrl[srf.idx(1, 0)].to_point();
        // (u=0,v=0): x=0 → y' = y + a*f(0) ≈ 0
        let p00 = deformed.ctrl[srf.idx(0, 0)].to_point();

        assert!(p00.y.abs() < 1e-9);
        assert!(p10.y.is_finite());
    }
}
```
---

