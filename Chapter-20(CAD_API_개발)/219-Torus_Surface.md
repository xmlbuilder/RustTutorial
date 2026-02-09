# Torus
## 🟦 1. 토러스의 기본 수학적 정의
- 토러스(torus)는 다음과 같은 곡면이다.
    - 축(axis): 점 S, 방향 벡터 T
    - 중심 원(center circle)의 중심: C
    - 중심 원 반지름: R
    - 단면 원(minor circle) 반지름: r
- 토러스의 점 P는 다음 조건을 만족한다:
```math
(\rho -R)^2+z'^2=r^2
```
- 여기서:
  - $\rho$ : 축에서의 거리 (radial distance)
  - $z'$: 축 방향 성분
  - $R=\| C-A_C\|$ : 중심 원 반지름
  - $A_C$: 점 C를 축에 정사영한 점

## 🟦 2. 테스트 코드에서 계산하는 모든 수식
- 테스트 코드에서 수행하는 계산을 전부 수식으로 정리하면 다음과 같다.

### ✔ 2.1 축 방향 단위 벡터
```math
\hat {T}=\frac{T}{\| T\| }
```

### ✔ 2.2 점 C를 축에 정사영
- 축은 다음 직선이다:
```math
L(u)=S+uT
```
- 점 C를 축에 정사영한 점 $A_C$:
```math
u_C=\frac{(C-S)\cdot T}{T\cdot T}
```
```math
A_C=S+u_CT
```
### ✔ 2.3 중심 원 반지름 R
```math
R=\| C-A_C\| 
```
### ✔ 2.4 토러스 표면의 임의 점 P(u,v)
- 테스트에서는:
```math
P=Srf(u,v)
```
### ✔ 2.5 점 P를 축에 정사영
```math
u_P=\frac{(P-S)\cdot T}{T\cdot T}
```
```math
A_P=S+u_PT
```
### ✔ 2.6 축에서의 거리 ρ
```math
\rho =\| P-A_P\| 
```
### ✔ 2.7 축 방향 성분 z′
```math
z'=(P-A_C)\cdot \hat {T}
```
### ✔ 2.8 토러스 방정식 검증
- 테스트에서 검증하는 핵심 수식:
```math
(\rho -R)^2+z'^2=r^2
```
- 좌변:
```math
LHS=(\rho -R)^2+z'^2
```
우변:
```math
RHS=r^2
```
- 테스트는 다음을 확인한다:
```math
|LHS-RHS|<\varepsilon 
```
## 🟦 3. on_create_torus_surface() 수식화
- 이 함수는 다음 과정을 수행한다.

### ✔ 3.1 축 방향 단위 벡터
```math
\hat {T}=\frac{T}{\| T\| }
```

### ✔ 3.2 점 C를 축에 정사영
```math
A_C=S+\frac{(C-S)\cdot \hat {T}}{\hat {T}\cdot \hat {T}}\hat {T}
```
### ✔ 3.3 중심 원 반지름 R
```math
R=\| C-A_C\| 
```
### ✔ 3.4 중심 원의 평면 기저 벡터 생성
```math
X=C-A_C
```
```math
\hat {X}=\frac{X}{\| X\| }
```
- 이 벡터는 중심 원의 시작 방향이다.

### ✔ 3.5 단면 원(profile circle) 생성
- 단면 원은 다음 평면에서 생성된다:
    - 중심: C
    - 기저 벡터: $\hat {X},\hat {T}$
    - 반지름: r
- 파라미터 $\theta$ 에 대해:
```math
P_{profile}(\theta )=C+r(\cos \theta \hat {X}+\sin \theta \hat {T})
```
### ✔ 3.6 단면 원을 축(S, T) 주위로 회전
- 회전 각도: $\phi$ 
- 회전 변환:
```math
P(u,v)=S+R(\phi )(P_{profile}(u)-S)
```
여기서 $R(\phi )$ 는 축 $\hat {T}$ 에 대한 회전 행렬.

## 🟦 4. 테스트가 검증하는 것 (수식 기반)
- 테스트는 다음을 검증한다:

### ✔ 4.1 생성된 표면의 모든 점 P(u,v)가 토러스 방정식을 만족하는지
```math
(\rho -R)^2+z'^2=r^2
```
- 즉:
    - 축에서의 거리 $\rho$
    - 중심 원 반지름 $R$
    - 축 방향 성분 $z'$
    - 단면 반지름 $r$
- 이 네 가지가 정확히 토러스의 정의를 만족하는지 검사한다.

### ✔ 4.2 표면의 모든 샘플 점에 대해 검사
- 테스트는 11×11 grid로 샘플링:
```math
u_i=\frac{i}{10},\quad v_j=\frac{j}{10}
```
- 각 점에 대해:
```math
|(\rho -R)^2+z'^2-r^2|<5\times 10^{-5}
```
- 이면 성공.

---
## 소스 코드
```rust
/// Create a NURBS torus / toroidal patch by revolving a circle (arc) about an axis.
/// Inputs:
/// - axis point S, axis direction T
/// - C: center of profile circle (or arc) to be revolved about axis
/// - r: radius of profile circle
/// - as_deg, ae_deg: start/end angles of profile arc measured in local (X,T) frame
/// - al_deg: revolution angle about axis (clockwise about axis)
/// - ctp: profile circle degree (Quadratic/Quintic/Quartic if supported)
/// - rtp: revolution iso-curve degree (Quadratic/Quintic)
/// Returns:
/// - NurbsSurface torus or patch
pub fn on_create_torus_surface(
    s: Point3D,
    t: Vector3D,
    c: Point3D,
    r: Real,
    as_deg: Real,
    ae_deg: Real,
    al_deg: Real,
    ctp: CircleDegree,
    rtp: CircleDegree,
    m_tol: Real, // for quintic circle builder if needed
) -> Result<NurbsSurface> {
    // ---- basic checks similar to C
    if r <= ON_ZERO_TOL {
        return Err(NurbsError::InvalidArgument {
            msg: "on_create_torus_surface: r must be > 0".into(),
        });
    }


    // 1) normalize axis direction ONCE and use it everywhere
    let t_hat = t.unitize();

    if !t_hat.is_valid() || t_hat.length_squared() < 1e-18 {
        return Err(NurbsError::InvalidArgument {
            msg: "on_create_torus_surface: axis direction T invalid".into(),
        });
    }

    // ---- Project C onto the (infinite) axis line through S with direction T
    let (a, d_line, prj_ok) = on_project_point_onto_line_infinite(s, t_hat, c);
    if !prj_ok {
        return Err(NurbsError::InvalidArgument {
            msg: "on_create_torus_surface: projection failed".into(),
        });
    }
    let _ = d_line; // C code stores it; not used later.

    // ---- X = C - A, d = |X|
    let x_raw = Vector3D::new(c.x - a.x, c.y - a.y, c.z - a.z);
    if x_raw.length_squared() < 1e-18 {
        return Err(NurbsError::InvalidArgument {
            msg: "on_create_torus_surface: C too close to axis (degenerate torus)".into(),
        });
    }
    let x_hat = orthonormalize_x_to_t(x_raw, t_hat)?;

    println!("x_hat, {}", x_hat);


    // 4) Build profile circle/arc in plane spanned by (x_hat, t_hat), centered at C
    // IMPORTANT:
    // - concir assumes orthonormal-ish frame; we pass x_hat, t_hat
    let profile = on_conic_circle_curve(
        c,
        x_hat,
        t_hat,
        r,
        as_deg,
        ae_deg,
        ctp,
        m_tol,
    )?;
    // 5) Revolve profile about axis (S, t_hat)
    let sur = on_surface_of_revolution_with_degree(
        &profile,
        s,
        t_hat,
        al_deg,
        rtp,
        m_tol, // use a meaningful tol, not ON_ZERO_TOL
    )?;
    Ok(sur)
}
```
```rust
// ------------------------------------------------------------
// Torus section circle invariant (cmstor)
//    축 좌표계에서 (rho - R)^2 + z'^2 = r^2
//    - 여기서 rho: 축에서의 거리, z': 축 방향 성분
//    - R: 중심원 반지름 (axis->C 거리)
// ------------------------------------------------------------
#[test]
fn torus_equation_in_axis_frame() -> Result<(), NurbsError> {
    let s_axis = Point3D { x: 0.5, y: -1.0, z: 2.0 };
    let t_axis = Vector3D::new(0.2, 0.3, 1.0); // not unit
    let c = Point3D { x: 3.5, y: -1.0, z: 2.0 };
    let r_minor = 0.75;

    let srf = on_create_torus_surface(
        s_axis,
        t_axis,
        c,
        r_minor,
        0.0,
        360.0,
        360.0,
        CircleDegree::Quintic,
        CircleDegree::Quintic,
        1e-12,
    )?;

    let t_hat = normalize(t_axis);

    // axis projection
    let a_c = {
        let sc = sub(c, s_axis);
        let u = dot(sc, t_axis) / dot(t_axis, t_axis);
        Point3D { x: s_axis.x + u * t_axis.x, y: s_axis.y + u * t_axis.y, z: s_axis.z + u * t_axis.z }
    };
    let R = norm(sub(c, a_c));

    for (u, v) in sample_surface_uv(&srf, 11, 11) {
        let p = srf.eval_point(u, v);

        let a_p = {
            let sp = sub(p, s_axis);
            let uu = dot(sp, t_axis) / dot(t_axis, t_axis);
            Point3D { x: s_axis.x + uu * t_axis.x, y: s_axis.y + uu * t_axis.y, z: s_axis.z + uu * t_axis.z }
        };

        let v_ap = sub(p, a_p);
        let zprime = dot(sub(p, a_c), t_hat);
        let rho = norm(sub(p, a_p));

        // torus eq
        let lhs = (rho - R) * (rho - R) + zprime * zprime;
        let rhs = r_minor * r_minor;

        assert!(
            approx(lhs, rhs, 5e-5),
            "torus eq mismatch at (u,v)=({u},{v}): lhs={lhs}, rhs={rhs}, rho={rho} R={R} z'={zprime} p=({},{},{})",
            p.x, p.y, p.z
        );
    }

    Ok(())
}
```

![Torus](/image/torusxzcurve.gif)
---
