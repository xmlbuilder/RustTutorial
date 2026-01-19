# on_project_curve_to_plane
- 이 함수는 NURBS 곡선을 평면에 투영(project) 하는 기능을 구현한 것으로,  
  기하학적으로도 꽤 깊은 내용을 담고 있음.
- 아래에 수식 기반 설명 + 기능 요약 + Parallel/Perspective 각각의 투영 공식

## 📘 on_project_curve_to_plane
- NURBS Curve Projection onto a Plane (Parallel / Perspective)
- 이 함수는 NURBS 곡선의 control point들을 평면에 투영하여 새로운 NURBS 곡선을 생성한다.
- degree 유지
- knot vector 유지
- control point 개수 유지
- rational 여부 유지
- control point만 투영됨
- 즉, 기하학적 형태만 평면으로 투영한 동일 구조의 곡선을 만든다.

### 1. 입력 파라미터 의미

| 이름 | 의미 |
|------|--------------------------------------------------------------|
| O    | 평면 위의 한 점 (plane point)                               |
| N    | 평면의 법선 벡터 (plane normal)                              |
| E    | Parallel: 투영 방향 벡터 / Perspective: 투영 중심점          |
| tol  | near-zero 검사용 tolerance                                   |
| mode | Parallel 또는 Perspective                                    |



### 2. 출력
- 동일한 degree
- 동일한 knot vector
- 동일한 control point 개수
- 투영된 control point로 구성된 새로운 NurbsCurve

### 3. 전체 구조
- 곡선 전체를 투영하는 대신:
```math
C(u)=\sum _iN_{i,p}(u)P_i
```
- 여기서 각 control point $P_i$ 를 평면에 투영하여:
```math
Q_i=\mathrm{Project}(P_i)
```
- 새 곡선:
```math
C'(u)=\sum _iN_{i,p}(u)Q_i
````
- 즉, basis 함수는 그대로 두고 control point만 투영한다.

## 4. Parallel Projection (평행 투영)
- ✔ 투영 방향: $\vec {E}$
- ✔ 투영 평면: 점 O, 법선 N
- 투영 공식:
- 점 P에서 평면까지의 signed distance:
```math
\mathrm{nop}=N\cdot (O-P)
```
- 방향 벡터와 평면 법선의 dot:
```math
\mathrm{ne}=N\cdot E
```
- 스칼라 배율:
```math
\beta =\frac{\mathrm{nop}}{\mathrm{ne}}
```
- 투영된 점:
```math
Q=P+\beta E
```

- ✔ Rational curve 처리
- Parallel 모드에서는 weight 유지
```math
w'=w
```

![Parallel projection](/image/prj_curve_parallel.png)

## 5. Perspective Projection (원근 투영)
- ✔ 투영 중심점: E
- ✔ 평면: 점 O, 법선 N
- 평면과 투영 중심의 관계:
```math
\mathrm{neo}=N\cdot (E-O)
```
- 평면과 점 P의 관계:
```math
\mathrm{nep}=N\cdot (E-P)
```
- 스칼라:
```math
\alpha =\frac{\mathrm{neo}}{\mathrm{nep}}
```
```math
\beta =1-\alpha
``` 
- 투영된 점:
```math
Q=\alpha P+\beta E
```

- ✔ Rational curve 처리
- 코드 규칙 그대로:
```math
w'=w\cdot (N\cdot (E-P))
```
- 즉, 원근 투영에서는 weight가 스케일됨.

## 6. tol 검사 의미
- N 또는 E 가 0 벡터인지 검사
- N 과 E 가 거의 직교(orthogonal)인지 검사
- Perspective에서 $N\cdot (E-P)$ 가 0에 가까우면 division-by-zero 방지


![Perspective projection](/image/prj_curve_perspective.png)

## 7. 함수의 기능 요약
- NURBS 곡선을 평면에 투영한다.
- Parallel / Perspective 두 방식 지원.
- degree, knot vector, control point 개수는 그대로 유지.
- control point만 투영하여 새로운 곡선을 생성.
- Rational curve의 weight 처리:
  - Parallel: weight 유지
  - Perspective: weight = w * (N · (E - P))



## 8. 왜 control point만 투영해도 되는가?
- NURBS 곡선은:
```math
C(u)=\frac{\sum _iN_{i,p}(u)w_iP_i}{\sum _iN_{i,p}(u)w_i}
```
- 투영은 선형 변환이므로:
```math
\mathrm{Project}(C(u))=C'(u)
```
- 여기서:
```math
C'(u)=\frac{\sum _iN_{i,p}(u)w_i\mathrm{Project}(P_i)}{\sum _iN_{i,p}(u)w_i}
```
- 즉, control point만 투영하면 전체 곡선도 정확히 투영된다.

## 9. 정리
- 이 함수는 CAD/CAM/CAE 시스템에서 매우 중요한 기능인  
  곡선의 평면 투영을 정확하게 구현한 것이다.
- Parallel → 직교 투영
- Perspective → 원근 투영
- Rational weight 처리까지 원본 C 코드와 동일
- 기하학적으로 완전한 투영 곡선을 생성

---
## 소스 코드
```rust

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveProjectionMode {
    Parallel,
    Perspective,
}

/// - O: point on projection plane
/// - N: plane normal
/// - E: direction (Parallel) OR center-of-projection (Perspective)
/// - tol: tolerance for checking near-orthogonality / near-zero divisions
///
/// 반환:
/// - 새로운 곡선(입력 곡선과 동일한 degree/knots/control count 유지, control point만 투영됨)
///
/// 주의(원본 로직 유지):
/// - Rational curve인 경우:
///   - Parallel: weight 유지 (w 그대로)
///   - Perspective: weight가 w * (N · (E - P)) 로 스케일됨 (원본 그대로)
pub fn on_project_curve_to_plane(
    cur_p: &NurbsCurve,
    o: Point3D,
    n: Vector3D,
    e: Point3D,
    mode: CurveProjectionMode,
    tol: Real,
) -> Result<NurbsCurve> {
    // output curve: same structure, projected ctrl only
    let mut cur_q = cur_p.clone();

    let is_rat = cur_p.is_rational();

    match mode {
        CurveProjectionMode::Parallel => {
            // NN = unit(N), EN = unit(E-dir)
            let mut nn = n;
            if !nn.normalize() {
                return Err(NurbsError::InvalidArgument
                  { msg: "on_project_curve_to_plane: normal N is zero".into() });
            }

            let mut edir = Vector3D::new(e.x, e.y, e.z);
            if !edir.normalize() {
                return Err(NurbsError::InvalidArgument
                  { msg: "on_project_curve_to_plane: direction E is zero".into() });
            }

            // check N not orthogonal to E (avoid division by near zero)
            let cos_ne = nn.dot(&edir);
            if cos_ne.abs() < tol {
                return Err(NurbsError::InvalidArgument
                  { msg: "on_project_curve_to_plane: N and E are (nearly) orthogonal".into() });
            }

            // ne = dot(N, E)  (원본은 정규화 아닌 N/E로 dot 한 뒤 bet=nop/ne 사용)
            let ne = n.dot(&Vector3D::new(e.x, e.y, e.z));
            if ne.abs() < tol {
                return Err(NurbsError::InvalidArgument
                  { msg: "on_project_curve_to_plane: dot(N,E) too small".into() });
            }

            let evec = Vector3D::new(e.x, e.y, e.z);

            for i in 0..cur_p.ctrl.len() {
                let pw = cur_p.ctrl[i];
                let p3 = pw.from_w(); // euclidean P

                // OP = O - P
                let op = Vector3D::new(o.x - p3.x, o.y - p3.y, o.z - p3.z);
                let nop = n.dot(&op);

                let bet = nop / ne;
                let q3 = Point3D::new(
                    p3.x + bet * evec.x,
                    p3.y + bet * evec.y,
                    p3.z + bet * evec.z,
                );

                if is_rat {
                    // keep weight (w)
                    let w = pw.w;
                    cur_q.ctrl[i] = Point4D::homogeneous(q3.x, q3.y, q3.z, w);
                } else {
                    cur_q.ctrl[i] = Point4D::homogeneous(q3.x, q3.y, q3.z, 1.0);
                }
            }
        }

        CurveProjectionMode::Perspective => {
            // EO = E - O
            let eo = Vector3D::new(e.x - o.x, e.y - o.y, e.z - o.z);
            let neo = n.dot(&eo);

            for i in 0..cur_p.ctrl.len() {
                let pw = cur_p.ctrl[i];
                let p3 = pw.from_w();

                // EP = E - P
                let ep = Vector3D::new(e.x - p3.x, e.y - p3.y, e.z - p3.z);
                let nep = n.dot(&ep);

                // division check: neo/nep
                if nep.abs() < tol {
                    return Err(NurbsError::NumericError
                      { msg: "on_project_curve_to_plane: perspective division by near-zero (N·(E-P))".into() });
                }

                let alf = neo / nep;
                let bet = 1.0 - alf;

                // Q = alf*P + bet*E
                let q3 = Point3D::new(
                    alf * p3.x + bet * e.x,
                    alf * p3.y + bet * e.y,
                    alf * p3.z + bet * e.z,
                );

                if is_rat {
                    // 원본: w = w * nep
                    let w = pw.w * nep;
                    cur_q.ctrl[i] = Point4D::homogeneous(q3.x, q3.y, q3.z, w);
                } else {
                    cur_q.ctrl[i] = Point4D::homogeneous(q3.x, q3.y, q3.z, 1.0);
                }
            }
        }
    }
    // knots/domain remain identical
    cur_q.kv = cur_p.kv.clone();
    cur_q.domain = cur_p.domain;

    Ok(cur_q)
}
```

---
### 테스트 코드
```rust
#[cfg(test)]
mod tests_project_curve_to_plane {
    use nurbslib::core::geom::{Point3D, Point4D, Vector3D};
    use nurbslib::core::types::Real;
    use nurbslib::core::nurbs_curve::{on_project_curve_to_plane, CurveProjectionMode, NurbsCurve};

    fn approx(a: Real, b: Real, eps: Real) -> bool {
        (a - b).abs() <= eps
    }

    fn assert_pt3_near(got: Point3D, exp: Point3D, eps: Real) {
        assert!(approx(got.x, exp.x, eps), "x: got {} exp {}", got.x, exp.x);
        assert!(approx(got.y, exp.y, eps), "y: got {} exp {}", got.y, exp.y);
        assert!(approx(got.z, exp.z, eps), "z: got {} exp {}", got.z, exp.z);
    }
```
```rust
    #[test]
    fn project_parallel_to_plane_z0_preserves_xy_sets_z0() {
        // plane: z=0
        let o = Point3D::new(0.0, 0.0, 0.0);
        let n = Vector3D::new(0.0, 0.0, 1.0);
        // parallel direction: -Z (NOTE: 함수 시그니처가 Point3D로 받는 버전 기준)
        let e = Point3D::new(0.0, 0.0, -1.0);

        // simple line curve (non-rational)
        let ctrl = vec![
            Point4D::homogeneous(1.0, 2.0, 5.0, 1.0),
            Point4D::homogeneous(-3.0, 1.0, -2.0, 1.0),
        ];
        let cur_p = NurbsCurve::from_ctrl_clamped_uniform(1, ctrl);

        let cur_q = on_project_curve_to_plane(
            &cur_p,
            o,
            n,
            e,
            CurveProjectionMode::Parallel,
            1e-12,
        )
            .expect("projection failed");

        // knots identical
        assert_eq!(cur_q.kv.knots, cur_p.kv.knots);

        // projected control points: z==0, x/y preserved
        let q0 = cur_q.ctrl[0].to_point();
        let q1 = cur_q.ctrl[1].to_point();

        assert_pt3_near(q0, Point3D::new(1.0, 2.0, 0.0), 1e-12);
        assert_pt3_near(q1, Point3D::new(-3.0, 1.0, 0.0), 1e-12);

        // weights unchanged for non-rational (still w=1)
        assert!(approx(cur_q.ctrl[0].w, 1.0, 1e-14));
        assert!(approx(cur_q.ctrl[1].w, 1.0, 1e-14));
    }
```
```rust
    #[test]
    fn project_perspective_to_plane_z0_matches_known_intersection() {
        // plane: z=0
        let o = Point3D::new(0.0, 0.0, 0.0);
        let n = Vector3D::new(0.0, 0.0, 1.0);
        // center of projection
        let e = Point3D::new(0.0, 0.0, 10.0);

        // two points in space
        let ctrl = vec![
            Point4D::homogeneous(1.0, 2.0, 5.0, 1.0),   // expected -> (2,4,0)
            Point4D::homogeneous(-1.0, 0.0, 8.0, 1.0),  // expected -> (-5,0,0)
        ];
        let cur_p = NurbsCurve::from_ctrl_clamped_uniform(1, ctrl);

        let cur_q = on_project_curve_to_plane(
            &cur_p,
            o,
            n,
            e,
            CurveProjectionMode::Perspective,
            1e-12,
        )
            .expect("projection failed");

        // knots identical
        assert_eq!(cur_q.kv.knots, cur_p.kv.knots);

        let q0 = cur_q.ctrl[0].to_point();
        let q1 = cur_q.ctrl[1].to_point();

        assert_pt3_near(q0, Point3D::new(2.0, 4.0, 0.0), 1e-12);
        assert_pt3_near(q1, Point3D::new(-5.0, 0.0, 0.0), 1e-12);

        // non-rational => weights stay 1
        assert!(approx(cur_q.ctrl[0].w, 1.0, 1e-14));
        assert!(approx(cur_q.ctrl[1].w, 1.0, 1e-14));
    }
```
```rust
    #[test]
    fn project_parallel_rational_keeps_weights() {
        // plane: z=0, parallel dir -Z
        let o = Point3D::new(0.0, 0.0, 0.0);
        let n = Vector3D::new(0.0, 0.0, 1.0);
        let e = Point3D::new(0.0, 0.0, -1.0);

        // rational curve: weights not all 1
        let ctrl = vec![
            Point4D::homogeneous(1.0, 2.0, 5.0, 2.0),   // w=2
            Point4D::homogeneous(-1.0, 0.0, 8.0, 0.5),  // w=0.5
        ];
        let cur_p = NurbsCurve::from_ctrl_clamped_uniform(1, ctrl);
        assert!(cur_p.is_rational(), "test expects rational curve");

        let cur_q = on_project_curve_to_plane(
            &cur_p,
            o,
            n,
            e,
            CurveProjectionMode::Parallel,
            1e-12,
        )
            .expect("projection failed");

        // weights unchanged
        assert!(approx(cur_q.ctrl[0].w, 2.0, 1e-14));
        assert!(approx(cur_q.ctrl[1].w, 0.5, 1e-14));

        // projected euclid points: z==0
        assert_pt3_near(cur_q.ctrl[0].to_point(), Point3D::new(1.0, 2.0, 0.0), 1e-12);
        assert_pt3_near(cur_q.ctrl[1].to_point(), Point3D::new(-1.0, 0.0, 0.0), 1e-12);
    }
```
```rust
    #[test]
    fn project_perspective_rational_scales_weights_by_nep() {
        // plane z=0, center E=(0,0,10)
        let o = Point3D::new(0.0, 0.0, 0.0);
        let n = Vector3D::new(0.0, 0.0, 1.0);
        let e = Point3D::new(0.0, 0.0, 10.0);

        // P0 z=5 => nep = N·(E-P)=10-5=5
        // P1 z=8 => nep = 2
        let ctrl = vec![
            Point4D::homogeneous(1.0, 2.0, 5.0, 2.0),   // w=2 -> w'=10
            Point4D::homogeneous(-1.0, 0.0, 8.0, 0.5),  // w=0.5 -> w'=1
        ];
        let cur_p = NurbsCurve::from_ctrl_clamped_uniform(1, ctrl);
        assert!(cur_p.is_rational(), "test expects rational curve");

        let cur_q = on_project_curve_to_plane(
            &cur_p,
            o,
            n,
            e,
            CurveProjectionMode::Perspective,
            1e-12,
        )
            .expect("projection failed");

        // expected weights (원본 C 로직 그대로)
        assert!(approx(cur_q.ctrl[0].w, 10.0, 1e-12), "w0={}", cur_q.ctrl[0].w);
        assert!(approx(cur_q.ctrl[1].w, 1.0, 1e-12), "w1={}", cur_q.ctrl[1].w);

        // expected projected points (euclidean)
        assert_pt3_near(cur_q.ctrl[0].to_point(), Point3D::new(2.0, 4.0, 0.0), 1e-12);
        assert_pt3_near(cur_q.ctrl[1].to_point(), Point3D::new(-5.0, 0.0, 0.0), 1e-12);

        // knots identical
        assert_eq!(cur_q.kv.knots, cur_p.kv.knots);
    }
}
```
---
