
# 📘 Scalar Surface Local Evaluation — 3 Function Documentation
- 아래 세 함수는 모두 스칼라 B‑spline/NURBS surface function
```math
F(u,v)=\sum _{i=0}^n\sum _{j=0}^mf_{ij}N_{i,p}(u)N_{j,q}(v)
```
- 을 평가하거나 미분하기 위한 함수들이다.
- 각 함수는 역할이 명확히 다르므로 반드시 구분해서 사용해야 한다.

## 1️⃣ on_eval_surface_value_local_span
- ✔ 역할: 순수 Local Evaluator (analytic)
- ✔ 수학적 의미:
- span_u, span_v가 이미 주어졌을 때
- 해당 local support 영역에서만 surface function을 평가한다.

```math
F(u,v)=\sum _{i=0}^p\sum _{j=0}^qf_{(span_u-p+i),(span_v-q+j)}\, N_{i,p}(u)\, N_{j,q}(v)
```
- ✔ 특징
    - basis_funs(span, u, p) 사용
    - **local support (p+1 × q+1)**만 사용
    - 가장 순수한 analytic evaluator
- SFun eval / on_surface_function_eval_grid 의 local 버전
- ✔ 언제 사용?
    - span을 이미 알고 있을 때
    - analytic evaluator 내부에서
    - numeric(local-only) evaluator 내부에서

## 2️⃣ on_eval_surface_derivative_analytic
- ✔ 역할: analytic evaluator (함수값 + 도함수)
- ✔ 수학적 의미
- k = 0
- 단순 surface function 평가
- F(u,v)
    - 내부적으로 on_eval_surface_value_local_span 호출
- k > 0
    - analytic derivative 평가
```math
\frac{\partial ^kF}{\partial u^k}(u,v)\quad \mathrm{또는}\quad \frac{\partial ^kF}{\partial v^k}(u,v)
```
- dir = UDir → ∂F/∂u
- dir = VDir → ∂F/∂v
- flk = LEFT/RIGHT (k>0일 때만 의미 있음)
    - ulp, vlp = parameter side
- ✔ 특징
    - span 자동 계산
    - k=0 → 함수값
    - k>0 → analytic derivative (N_SFNDER 포팅 버전 사용)
- ✔ 언제 사용?
    - analytic derivative 필요할 때
    - LEFT/RIGHT derivative 필요할 때
    - numeric(local-only) evaluator에서 F(u±h, v) 계산할 때

## 3️⃣ on_eval_surface_derivative_numeric_local
- ✔ 역할: numeric(local-only) derivative evaluator
- ✔ 수학적 의미
    - analytic evaluator를 기반으로 finite difference 적용:
- U 방향
```math
F_u(u,v)\approx \frac{F_{\mathrm{local}}(u+h,v)-F_{\mathrm{local}}(u-h,v)}{2h}
```
- V 방향
```math
F_v(u,v)\approx \frac{F_{\mathrm{local}}(u,v+h)-F_{\mathrm{local}}(u,v-h)}{2h}
```
- 여기서
```math
F_{\mathrm{local}}=\mathrm{on\_eval\_surface\_derivative\_analytic}(k=0)
```
- ✔ 특징
    - analytic evaluator와 동일한 local support 사용
    - numeric(global)과 달리 analytic과 거의 완벽하게 일치
    - boundary에서도 안정적
    - h는 보통 1e‑5 ~ 1e‑6
✔ 언제 사용?
    - analytic(N_SFNDER) 검증
    - numeric(global)과 analytic의 차이 분석
    - kernel correctness 테스트

## 📘 세 함수의 관계 요약
| 함수 이름                                   | 역할                          | Analytic? | Numeric? | Span 필요? | 사용 목적                                      |
|---------------------------------------------|-------------------------------|-----------|----------|------------|-----------------------------------------------|
| on_eval_surface_value_local_span           | 순수 Local Evaluator          | ✔         | ✘        | ✔          | basis × fuv local block 계산                  |
|                                             | (span 기반)                   |           |          |            | N_SFNEVN / N_SFNEVG local 버전                |
| on_eval_surface_derivative_analytic                | Analytic Evaluator            | ✔         | ✘        | 자동       | k차 도함수 평가 (N_SFNDER 포팅)               |
|                                             | (k, dir, Side 기반)           |           |          |            | k=0 → 함수값, k>0 → analytic derivative       |
| on_eval_surface_derivative_numeric_local  | Numeric(Local-only) Evaluator | ✘         | ✔        | 자동       | analytic 검증용 finite diff                   |
|                                             | (analytic local 기반 numeric) |           |          |            | analytic과 동일한 local support 기반 numeric  |


## 📌 핵심 정리
- local_span → 가장 순수한 analytic local evaluator
- local → analytic derivative evaluator
- local_numeric_deriv → analytic과 동일한 local support 기반 numeric derivative
- 이 세 가지가 정확히 분리되어 있어야 analytic vs numeric 비교가 정확하게 이루어진다.


## 소스 코드
```rust
/// Evaluate a scalar surface function F(u,v) locally,
/// assuming the u/v spans are already known.
///
/// F(u,v) = Σ_i Σ_j fuv[i][j] N_{i,p}(u) N_{j,q}(v)
///
/// - fuv[i][j] : scalar surface coefficients
/// - knu, knv  : knot vectors in u, v
/// - p, q      : degrees in u, v
/// - u, v      : parameter values
/// - span_u    : span index in U such that u ∈ [U[span_u], U[span_u+1])
/// - span_v    : span index in V such that v ∈ [V[span_v], V[span_v+1])
pub fn on_eval_surface_value_local_span(
    fuv: &[Vec<f64>],
    knu: &KnotVector,
    knv: &KnotVector,
    p: usize,
    q: usize,
    u: f64,
    v: f64,
    span_u: usize,
    span_v: usize,
) -> Result<f64, NurbsError> {
    let nu = fuv.len();
    if nu == 0 {
        return Err(NurbsError::DimensionMismatch {
            msg: "empty fuv in on_eval_surface_derivative_analytic",
        });
    }
    let mv = fuv[0].len();

    if span_u < p || span_u >= knu.len() - 1 {
        return Err(NurbsError::IndexOutOfRange);
    }
    if span_v < q || span_v >= knv.len() - 1 {
        return Err(NurbsError::IndexOutOfRange);
    }

    let u_knots = knu.as_slice();
    let v_knots = knv.as_slice();

    // local basis on [span_u-p .. span_u], [span_v-q .. span_v]
    let nu_basis = u_knots.basis_funs(span_u, u, p);
    let nv_basis = v_knots.basis_funs(span_v, v, q);

    let mut f = 0.0;

    for i in 0..=p {
        let a = span_u - p + i;
        if a >= nu {
            continue;
        }
        let mut t = 0.0;
        for j in 0..=q {
            let b = span_v - q + j;
            if b >= mv {
                continue;
            }
            t += nv_basis[j] * fuv[a][b];
        }
        f += nu_basis[i] * t;
    }
    Ok(f)
}
```
```rust
pub fn on_eval_surface_derivative_analytic(
    fuv: &[Vec<f64>],
    knu: &KnotVector,
    knv: &KnotVector,
    p: usize,
    q: usize,
    k: usize,
    u: f64,
    v: f64,
    dir: SurfaceDir,
    flk: Side,
    ulp: Side,
    vlp: Side,
) -> Result<f64, NurbsError> {
    // span 계산
    let span_u = knu.as_slice().find_span(fuv.len() - 1, p, u);
    let span_v = knv.as_slice().find_span(fuv[0].len() - 1, q, v);

    // k=0 → 함수값
    if k == 0 {
        return on_eval_surface_value_local_span(
            fuv, knu, knv, p, q, u, v, span_u, span_v
        );
    }

    // k>0 → 도함수 (analytic)
    // 여기서는 N_SFNDER 포팅 버전 사용
    let FD = on_surface_function_derivatives(
        fuv, knu, knv, p, q, u, v, k, k
    )?;

    match dir {
        SurfaceDir::UDir => Ok(FD[k][0]),
        SurfaceDir::VDir => Ok(FD[0][k]),
    }
}
```
```rust
pub fn on_eval_surface_derivative_numeric_local(
    fuv: &[Vec<f64>],
    knu: &KnotVector,
    knv: &KnotVector,
    p: usize,
    q: usize,
    u: f64,
    v: f64,
    dir: SurfaceDir,
    h: f64,
    ulp: Side,
    vlp: Side,
) -> Result<f64, NurbsError> {
    match dir {
        SurfaceDir::UDir => {
            let f_plus = on_eval_surface_derivative_analytic(
                fuv, knu, knv,
                p,
                q,
                0,              // k=0 → 함수값
                u + h,
                v,
                SurfaceDir::UDir,  // 의미 없음 (k=0)
                Side::Left,     // flk: k=0이므로 무시됨
                ulp, vlp,
            )?;

            let f_minus = on_eval_surface_derivative_analytic(
                fuv, knu, knv, p, q,
                0,
                u - h, v,
                SurfaceDir::UDir,
                Side::Left,
                ulp, vlp,
            )?;

            Ok((f_plus - f_minus) / (2.0 * h))
        }

        SurfaceDir::VDir => {
            let f_plus = on_eval_surface_derivative_analytic(
                fuv, knu, knv, p, q,
                0,
                u, v + h,
                SurfaceDir::VDir,
                Side::Left,
                ulp, vlp,
            )?;

            let f_minus = on_eval_surface_derivative_analytic(
                fuv, knu, knv, p, q,
                0,
                u, v - h,
                SurfaceDir::VDir,
                Side::Left,
                ulp, vlp,
            )?;

            Ok((f_plus - f_minus) / (2.0 * h))
        }
    }
}

```

---
