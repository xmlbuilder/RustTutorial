## 📘 원호의 비유리(non‑rational) B‑spline 근사 알고리즘
- 1. 목적
    - 주어진 원(circle) 또는 원호(arc)를 비유리(non‑rational) B‑spline 곡선으로 근사한다.
    - 근사 곡선의 차수는 2, 3, 4 중 하나이며, 오차는 반경(radial) 오차 기준으로 제어한다.

## 2. 입력 데이터

| 기호 | 설명 |
|------|------|
| C | 원의 중심점 (Center point) |
| X, Y | 원의 국소 좌표계(직교 단위 벡터) |
| r | 반지름 (radius) |
| a_s, a_e | 시작/끝 각도 (start angle, end angle, radians) |
| p | 근사 B-spline 차수 (2, 3, 4) |
| tol | 허용 반경 오차 (absolute 또는 relative) |
| flag | 오차 방식: ABSOLUTE 또는 RELATIVE |


## 3. 각도 정규화
- 원호의 sweep angle:
```math
\theta_s = a_s,\qquad
\theta_e =
\begin{cases}
a_e + 2\pi & \text{if } a_e < a_s, \\
a_e        & \text{otherwise}.
\end{cases}
```
```math 
\Delta \theta =\theta _e-\theta _s
```
- 조건:
```math
\Delta \theta >0
```

## 4. 오차 정의 (Radial Error)
- 원호를 chord로 근사할 때 발생하는 최대 오차는 sagitta(현의 높이) 로 표현된다.
### 4.1 Sagitta 공식
- 원호를 각도 $\Delta$ $\theta$  만큼 chord로 근사할 때 sagitta s 는:
```math
s=r\left( 1-\cos \frac{\Delta \theta }{2}\right)
``` 
- 근사 오차 조건:
```math
s\leq tol
```
## 5. 허용 각도 간격 Δθ 계산
- 오차 조건을 만족하는 최대 각도 간격 $\Delta \theta _{\max }$ 는:
```math
r\left( 1-\cos \frac{\Delta \theta _{\max }}{2}\right) \leq tol
```
- 정리하면:
```math
\cos \frac{\Delta \theta _{\max }}{2}\geq 1-\frac{tol}{r}
```
- 따라서:
```math
\Delta \theta _{\max }=2\arccos \left( 1-\frac{tol}{r}\right) 
```

## 6. 세그먼트 개수 결정
- 전체 sweep angle $\Delta \theta$  를 위 조건으로 나누면:
```math
N_{\mathrm{seg}}=\left\lceil \frac{\Delta \theta }{\Delta \theta _{\max }}\right\rceil
``` 
- Rust 구현에서는 안정성을 위해:
    - 최소 1
    - 최대 256

## 7. 샘플 포인트 생성
- 세그먼트당 3개 정도의 샘플을 사용하여 충분히 부드러운 곡선을 얻는다.
- 총 샘플 수:
```math
N_{\mathrm{samples}}=3\cdot N_{\mathrm{seg}}
```
- 샘플링:
```math
t_i=\theta _s+\frac{i}{N_{\mathrm{samples}}}\Delta \theta
``` 
- 각 샘플 포인트:
```rust
P(t_i)=C+r\cos t_i\cdot X+r\sin t_i\cdot Y
```

## 8. 최소제곱 비유리 B‑spline 근사
- 샘플 포인트 집합 $\{ P_i\}$  에 대해:
- Chord-length parameterization
```math
u_0=0,\quad u_i=\frac{\sum _{k=1}^i\| P_k-P_{k-1}\| }{\sum _{k=1}^{m-1}\| P_k-P_{k-1}\| }
```
- Knot vector 생성
```math
U=\mathrm{averaging\  knot\  vector}(p,n_{\mathrm{ctrl}},\{ u_i\} )
```
- 선형 시스템 구성
```math
\sum _{j=0}^{n_{\mathrm{ctrl}}-1}N_{j,p}(u_i)\, C_j=P_i
```
- 행렬식:
```math
AC=P
```
- 최소제곱 해
```math
C=(A^TA)^{-1}A^TP
```
- 비유리 곡선 생성
모든 weight = 1

## 9. 최종 출력
- 비유리 B‑spline 곡선:
```math
\mathrm{NurbsCurve}=\{ p,U,C_j,w_j=1\}
``` 

## 10. 알고리즘 전체 흐름 요약
- 입력 검증
- 각도 정규화
- 오차 기반 최대 각도 간격 계산
- 세그먼트 개수 결정
- 원호 샘플링
- chord-length 파라미터화
- knot vector 생성
- 최소제곱으로 control point 계산
- 비유리 B‑spline 곡선 반환


```rust
/// - degree: 2,3,4 (그 외는 None 반환)
/// - tol:   허용 반경 오차
/// - flag:
///     - ErrorFlag::Absolute → tol = 절대 길이
///     - ErrorFlag::Relative → tol = 반지름 대비 % (예: 0.1 => 0.1%)
///
/// 내부 동작:
///   1) Arc를 충분히 촘촘하게 샘플링 (sagitta 기반 각도 간격)
///   2) 그 샘플 포인트들을 on_least_squares_curve 로 근사
///   3) 비유리 NurbsCurve (w=1) 반환
pub fn approximate_non_rational(
    &self,
    degree: Degree,
    mut tol: Real,
    flag: ErrorFlag,
) -> Option<NurbsCurve> {
    let p = degree as usize;
    if p < 2 || p > 4 {
        return None;
    }

    let r = self.radius();
    if r <= 0.0 || !r.is_finite() {
        return None;
    }

    let as_rad = self.angle.t0; // already radians
    let mut ae_rad = self.angle.t1;
    if (ae_rad - as_rad).abs() < 1e-12 {
        return None;
    }

    // sweep 각도 정규화
    if ae_rad < as_rad {
        ae_rad += TAU;
    }
    let sweep = ae_rad - as_rad;
    if sweep <= 0.0 {
        return None;
    }

    // RELATIVE → tol [% of radius] 로 처리 (Piegl와 동일 의미)
    match flag {
        ErrorFlag::Absolute => {
            // 그대로 사용
        }
        ErrorFlag::Relative => {
            // tol[%] → 절대 거리
            // 예: tol=0.1 이면 0.1% → tol_abs = 0.001 * r
            tol = (tol * 0.01) * r;
        }
    }

    if tol <= 0.0 {
        // 너무 작은 값이면 최소값 부여
        tol = 1e-6 * r.max(1.0);
    }

    // sagitta 공식 기반으로 최대 각도 간격 Δθ 결정
    // sagitta s = r (1 - cos(Δθ/2)) ≤ tol
    // => cos(Δθ/2) ≥ 1 - tol/r
    let ratio = (tol / r).min(2.0);
    let delta_theta_max = if ratio >= 2.0 {
        sweep
    } else {
        let c = 1.0 - ratio;
        if c <= -1.0 {
            sweep
        } else {
            let val = c.max(-1.0).min(1.0);
            2.0 * val.acos()
        }
    };

    // 필요한 segment 개수
    let mut n_seg = (sweep / delta_theta_max).ceil() as usize;
    if n_seg < 1 {
        n_seg = 1;
    }
    // 너무 과도하지 않게 상한
    n_seg = n_seg.min(256);

    // 샘플 포인트 개수
    let n_samples = n_seg * 3; // segment당 3포인트 정도 → 부드럽게

    // 샘플링
    let mut samples = Vec::<Point3D>::with_capacity(n_samples + 1);
    for i in 0..=n_samples {
        let t = as_rad + (i as Real) * sweep / (n_samples as Real);
        samples.push(self.point_at(t));
    }

    // control point 개수 선택: degree+1 이상, 샘플 수보다 적게
    let mut n_ctrl = (n_samples / 2).max(p + 1);
    n_ctrl = n_ctrl.min(n_samples.saturating_sub(1));
    if n_ctrl < p + 1 {
        n_ctrl = p + 1;
    }

    // 최소제곱 비유리 근사
    let curve = on_least_squares_curve(&samples, degree, n_ctrl)?;

    Some(curve)
}
```
```rust
pub fn on_least_squares_curve(
    sample_points: &[Point3D],
    degree: Degree,
    control_point_count: usize,
) -> Option<NurbsCurve> {
    let params = on_chord_parameterization(sample_points)?;
    let knots = on_least_squares_build_knot_vector(degree, control_point_count, &params)?;
    let ctrl =
        on_least_squares_curve_solve(sample_points, degree, control_point_count, &params, &knots)?;

    Some(NurbsCurve {
        dimension: 3,
        degree,
        kv: KnotVector { knots },
        ctrl,
        domain: Interval { t0: 0.0, t1: 1.0 },
    })
}
```
```rust
/// chord-length parameterization in [0,1]
pub fn on_chord_parameterization(points: &[Point3D]) -> Option<Vec<Real>> {
    let m = points.len();
    if m == 0 {
        return None;
    }
    if m == 1 {
        return Some(vec![0.0]);
    }

    let mut u = vec![0.0f64; m];
    for i in 1..m {
        let d = (points[i] - points[i - 1]).length();
        u[i] = u[i - 1] + d;
    }
    let total = u[m - 1];
    if !(total > 1e-300) {
        return None;
    }
    for v in &mut u {
        *v /= total;
    }
    Some(u)
}
```
```rust
pub fn on_least_squares_build_knot_vector(
    degree: Degree,
    control_point_count: usize,
    parameters: &[Real],
) -> Option<Vec<Real>> {
    if degree > 7 {
        return None;
    }
    let p = degree as usize;
    if control_point_count < p + 1 {
        return None;
    }
    if parameters.len() < 2 {
        return None;
    }

    Some(on_build_knot_vector(
        p as Degree,
        control_point_count,
        parameters,
    ))
}
```
```rust
pub fn on_least_squares_curve_solve(
    sample_points: &[Point3D],
    degree: Degree,
    control_point_count: usize,
    parameters: &[Real],
    knot_vector: &[Real],
) -> Option<Vec<Point4D>> {
    if degree > 7 {
        return None;
    }
    let p = degree as usize;

    let m = sample_points.len();
    if m < 2 {
        return None;
    }

    if control_point_count < 2 {
        return None;
    }
    if control_point_count < p + 1 {
        return None;
    }

    let n_ctrl = control_point_count;
    let n = n_ctrl - 1;

    // basis helpers expect KnotVector form
    let kv = KnotVector {
        knots: knot_vector.to_vec(),
    };
    if kv.check_degree_vs_cp(degree, n_ctrl).is_err() {
        return None;
    }

    // N matrix
    let mut mat = Matrix::with_dims(m, n_ctrl);
    mat.zero();
    *mat.at_mut(0, 0) = 1.0;
    *mat.at_mut((m - 1) as i32, n as i32) = 1.0;

    let q0 = sample_points[0];
    let qn = sample_points[m - 1];

    let mut rk = vec![Point3D::new(0.0, 0.0, 0.0); m];

    for i in 0..m {
        let u = parameters[i];
        let span = on_find_span(n, p, u, &kv.knots);
        let bf = on_basis_funs_ret_vec(&kv.knots, span, u, p);

        for j in 0..=p {
            let col = span - p + j;
            *mat.at_mut(i as i32, col as i32) = bf[j];
        }

        let n0 = mat.get(i, 0);
        let nn = mat.get(i, n);
        rk[i] = sample_points[i] - q0 * n0 - qn * nn;
    }

    // solve interior
    let mut ctrl = vec![Point4D::homogeneous(0.0, 0.0, 0.0, 1.0); n_ctrl];
    ctrl[0] = Point4D::homogeneous(q0.x, q0.y, q0.z, 1.0);
    ctrl[n] = Point4D::homogeneous(qn.x, qn.y, qn.z, 1.0);

    if n_ctrl >= 3 {
        let k = n_ctrl - 2;
        let rows = m - 2;

        let mut mat_a = Matrix::with_dims(k, k);
        mat_a.zero();
        let mut bx = vec![0.0; k];
        let mut by = vec![0.0; k];
        let mut bz = vec![0.0; k];

        for r in 0..rows {
            let i_data = r + 1;
            for a in 0..k {
                let na = mat.get(i_data, a + 1);
                bx[a] += na * rk[i_data].x;
                by[a] += na * rk[i_data].y;
                bz[a] += na * rk[i_data].z;
                for b in 0..k {
                    let nb = mat.get(i_data, b + 1);
                    *mat_a.at_mut(a as i32, b as i32) += na * nb;
                }
            }
        }

        let mut mat_ax = mat_a.clone();
        let mut mat_ay = mat_a.clone();
        let mut mat_az = mat_a.clone();
        if !on_solve_linear_system_vec(&mut mat_ax, &mut bx) {
            return None;
        }
        if !on_solve_linear_system_vec(&mut mat_ay, &mut by) {
            return None;
        }
        if !on_solve_linear_system_vec(&mut mat_az, &mut bz) {
            return None;
        }

        for i in 0..k {
            ctrl[i + 1] = Point4D::homogeneous(bx[i], by[i], bz[i], 1.0);
        }
    }

    Some(ctrl)
}
```
---
