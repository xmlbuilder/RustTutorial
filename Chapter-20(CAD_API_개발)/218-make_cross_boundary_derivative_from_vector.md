
# on_make_cross_boundary_derivative_from_vector

- 이건 CAD/CAE 커널에서 Class‑A 곡면, G1/G2 경계 조건, Patch 연결 같은 데서 반드시 필요한 기능
- 곡선(curve)의 경계에서, 주어진 방향 벡터를 따라 **Cross-boundary derivative patch** 를 생성하는 함수.
- 곡선이 있고 → 그 경계에서 → 어떤 방향으로 → **미분 벡터(derivative)** 를 가진 곡면을 만들어야 할 때 쓰는 함수.

## 🧠 왜 이런 기능이 필요한가?
- CAD/CAE 커널에서 경계 조건을 맞출 때:
    - G1 (tangent continuity)
    - G2 (curvature continuity)
    - Patch blending
    - Surface extension
    - Boundary derivative control
- 이런 걸 하려면 곡선의 경계에서 미분 벡터를 가진 보조 곡면이 필요

## 🔍 함수가 실제로 하는 일 (단계별 해석)
- 코드를 단계별로 해석하면 이렇게 된다.

### 1) 경계 방향에 따라 U 방향인지 V 방향인지 결정
```rust
let dir = match bndy {
    Left | Right => UDir,
    Bottom | Top => VDir,
};
```
- Left/Right → U 방향으로 extrude
- Bottom/Top → V 방향으로 extrude
- 즉, 곡면의 어느 방향으로 미분 패치를 만들지 결정.

### 2) 벡터 방향을 경계 규칙에 맞게 뒤집기
- Left/Bottom → +V  
- Right/Top → -V


- 왜냐면:
    - Left/Bottom 은 **양의 방향**
    - Right/Top 은 **반대 방향**
- 즉, 경계의 outward 방향을 맞추기 위한 처리.

### 3) generalized cylinder 생성
- on_make_generalized_cylinder(cur, v2, mag, dir)


- 이게 핵심.
    - 곡선(cur)을
    - 방향 벡터(v2)로
    - 길이(mag)만큼
    - U 또는 V 방향으로 extrude
- 즉, 곡선 + 벡터 → 2×N 또는 N×2 control net을 가진 1차 곡면 생성.
- 이 곡면은 사실:
```math
S(u,t)=C(u)+t\cdot v
```
- 이 형태의 선형 곡면.
    - 이게 바로 **cross-boundary derivative surface**.

### 4) Right/Top 경계일 경우 control net 뒤집기
- 왜냐면 extrude 방향이 반대.
    - Right → U=0 ↔ U=1 swap
    - Top → V=0 ↔ V=1 swap
- 이걸 안 하면 미분 방향이 뒤집혀서 G1 조건이 깨짐.

## 🎯 최종적으로 이 함수가 만들어주는 것
- degree‑1 (linear)
- 2×N 또는 N×2 control net
- 곡선의 경계에서 미분 벡터를 표현하는 곡면 patch
- 즉, 곡선의 경계에서 “미분 조건”을 곡면 형태로 만들어주는 함수.
- 이건 CAD 커널에서:
    - Surface extension
    - Boundary derivative enforcement
    - Lofting
    - Skinning
    - Patch blending
    - G1/G2 continuity constraints

---
## 소스 코드
```rust
/// Cross-boundary derivative from curve + constant vector
/// Rules applied:
/// - Point4D is ALWAYS homogeneous (xw,yw,zw,w)
/// - Surface ctrl is row-major: idx = nu*j + i
/// - Extrusion is degree-1 clamped in chosen direction
pub fn on_make_cross_boundary_derivative_from_vector(
    cur: &NurbsCurve,
    v: Vector3D,
    bndy: SideFlag,
) -> Result<NurbsSurface> {
    // ------------------------------------------------------------
    // 1) Determine extrusion direction (UDIR / VDIR)
    // ------------------------------------------------------------
    let dir = match bndy {
        SideFlag::Left | SideFlag::Right => SurfaceDir::UDir,
        SideFlag::Bottom | SideFlag::Top => SurfaceDir::VDir,
    };

    // ------------------------------------------------------------
    // 2) Determine derivative vector orientation
    //    LEFT / BOTTOM :  +V
    //    RIGHT / TOP   :  -V
    // ------------------------------------------------------------
    let mut v2 = v;
    match bndy {
        SideFlag::Left | SideFlag::Bottom => {}
        SideFlag::Right | SideFlag::Top => {
            v2.x = -v2.x;
            v2.y = -v2.y;
            v2.z = -v2.z;
        }
    }

    let mag = v2.length();
    if mag <= Real::MIN_POSITIVE {
        return Err(NurbsError::InvalidInput {
            msg: "cross-boundary derivative vector has zero length".into(),
        });
    }

    // ------------------------------------------------------------
    // 3) Create derivative surface via generalized cylinder
    //    (curve extruded by constant vector)
    // ------------------------------------------------------------
    let mut der = on_make_generalized_cylinder(cur, v2, mag, dir)?;

    // ------------------------------------------------------------
    // 4) Reverse surface net for RIGHT / TOP
    //    (swap the two rows or columns)
    // ------------------------------------------------------------
    if matches!(bndy, SideFlag::Right | SideFlag::Top) {
        let nu = der.nu;
        let nv = der.nv;

        match bndy {
            SideFlag::Right => {
                // swap u=0 <-> u=1  (for all v)
                // UDIR case → nu == 2
                for j in 0..nv {
                    let i0 = on_idx_row_major(nu, 0, j);
                    let i1 = on_idx_row_major(nu, 1, j);
                    der.ctrl.swap(i0, i1);
                }
            }
            SideFlag::Top => {
                // swap v=0 <-> v=1 (for all u)
                // VDIR case → nv == 2
                for i in 0..nu {
                    let j0 = on_idx_row_major(nu, i, 0);
                    let j1 = on_idx_row_major(nu, i, 1);
                    der.ctrl.swap(j0, j1);
                }
            }
            _ => {}
        }
    }

    Ok(der)
}
```
---

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::circle::{CircleDegree, on_conic_circle_curve};
    use nurbslib::core::geom::{Point3D, Vector3D};
    use nurbslib::core::nurbs_surface_extensions::on_make_cross_boundary_derivative_from_vector;
    use nurbslib::core::types::{Real, SideFlag};

    fn assert_point_close(a: Point3D, b: Point3D, tol: Real) {
        let d = a.distance(&b);
        assert!(d <= tol, "point mismatch: {:?} vs {:?} (d={})", a, b, d);
    }

    fn lerp(a: Real, b: Real, t: Real) -> Real { a + (b - a) * t }
```
```rust
    #[test]
    fn left_boundary() {
        let tol = 1e-8;

        // curve
        let center = Point3D::new(0.0, 0.0, 0.0);
        let mut x = Vector3D::new(1.0, 0.0, 0.0);
        let mut y = Vector3D::new(0.0, 1.0, 0.0);
        x.normalize();
        y.normalize();

        let cur = on_conic_circle_curve(
            center, x, y, 1.0, 0.0, 180.0,
            CircleDegree::Quadratic,
            1e-10
        ).unwrap();

        let v = Vector3D::new(0.0, 0.0, 2.0);

        let sur = on_make_cross_boundary_derivative_from_vector(&cur, v, SideFlag::Left).unwrap();

        // ✅ 샘플 파라미터는 surface의 "curve 방향 도메인"을 써야 함
        // LEFT/RIGHT는 dir=UDir 이므로 curve는 v방향
        let (t0, t1) = (sur.domain_v.t0, sur.domain_v.t1);

        let samples = 10;
        for i in 0..=samples {
            let s = i as Real / samples as Real;
            let tv = lerp(t0, t1, s);

            let p0 = sur.eval_point(sur.domain_u.t0, tv); // u=0
            let p1 = sur.eval_point(sur.domain_u.t1, tv); // u=1

            let c = cur.eval_point(tv);

            assert_point_close(p0, c, tol);
            assert_point_close(p1, c + v, tol);
        }
    }
```
```rust
    #[test]
    fn right_boundary() {
        let tol = 1e-8;

        let center = Point3D::new(0.0, 0.0, 0.0);
        let mut x = Vector3D::new(1.0, 0.0, 0.0);
        let mut y = Vector3D::new(0.0, 1.0, 0.0);
        x.normalize();
        y.normalize();

        let cur = on_conic_circle_curve(
            center, x, y, 1.0, 0.0, 180.0,
            CircleDegree::Quadratic,
            1e-10
        ).unwrap();

        let v = Vector3D::new(0.0, 0.0, 2.0);

        let sur = on_make_cross_boundary_derivative_from_vector(&cur, v, SideFlag::Right).unwrap();

        // RIGHT도 curve는 v방향
        let (t0, t1) = (sur.domain_v.t0, sur.domain_v.t1);

        let samples = 10;
        for i in 0..=samples {
            let s = i as Real / samples as Real;
            let tv = lerp(t0, t1, s);

            let p0 = sur.eval_point(sur.domain_u.t0, tv); // u=0
            let p1 = sur.eval_point(sur.domain_u.t1, tv); // u=1

            let c = cur.eval_point(tv);

            // ✅ C 규약: RIGHT는 (curve - V, curve)
            assert_point_close(p0, c - v.to_point(), tol);
            assert_point_close(p1, c, tol);
        }
    }
```
```rust
    #[test]
    fn top_boundary() {
        let tol = 1e-8;

        let center = Point3D::new(0.0, 0.0, 0.0);
        let mut x = Vector3D::new(1.0, 0.0, 0.0);
        let mut y = Vector3D::new(0.0, 1.0, 0.0);
        x.normalize();
        y.normalize();

        let cur = on_conic_circle_curve(
            center, x, y, 1.0, 180.0, 360.0,
            CircleDegree::Quadratic,
            1e-10
        ).unwrap();

        let v = Vector3D::new(0.0, 0.0, 2.0);

        let sur = on_make_cross_boundary_derivative_from_vector(&cur, v, SideFlag::Top).unwrap();

        // TOP/BOTTOM은 dir=VDir 이므로 curve는 u방향
        let (t0, t1) = (sur.domain_u.t0, sur.domain_u.t1);

        let samples = 10;
        for i in 0..=samples {
            let s = i as Real / samples as Real;
            let tu = lerp(t0, t1, s);

            let p0 = sur.eval_point(tu, sur.domain_v.t0); // v=0
            let p1 = sur.eval_point(tu, sur.domain_v.t1); // v=1

            let c = cur.eval_point(tu);

            // ✅ C 규약: TOP은 (curve - V, curve)
            assert_point_close(p0, c - v.to_point(), tol);
            assert_point_close(p1, c, tol);
        }
    }
}
```
---

