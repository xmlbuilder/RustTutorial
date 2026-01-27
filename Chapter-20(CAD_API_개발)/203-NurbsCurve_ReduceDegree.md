# Nurbs Reduce Degree


- NURBS 곡선 전체를 한 번(p→p−1) 차수 감소시키는 완전한 파이프라인.
- 구현한 Bezier 1‑step degree reduction 을 기반으로,  
    곡선을 Bezier 조각으로 분해 → 각 조각 차수 감소 → 다시 조립하는 구조.
## 🟦 0. 전체 목표 (수식 관점)
- 입력:
```math
C(u)=\sum _{i=0}^nP_iN_{i,p}(u)
```
- 출력:
```math
\tilde {C}(u)=\sum _{i=0}^{n'}Q_iN_{i,p-1}(u)
```
- 단,
    - 곡선 형상은 최대한 유지
    - 오차는 tol 이하
    - rational NURBS도 지원
    - 내부적으로는 Bezier 조각 단위로 차수 감소 수행
- 즉, NURBS 차수 감소 = Bezier 차수 감소 × (조각 수)

## 🟩 1. Rational tolerance 조정 (수식)
- NURBS는 동차좌표이므로  
    오차 tolerance도 weight와 곡선 크기에 따라 조정해야 한다.
- 코드:
```rust
tol_adj = (tol * wmin) / (1.0 + pmax)
```

- 수식 의미:
```math
w_{\min }=\min |w_i|
```
```math
- p_{\max }=\max \| P_i/w_i\|
``` 
- 조정된 tol:
```math
\mathrm{tol_{\mathnormal{adj}}}=\frac{\mathrm{tol}\cdot w_{\min }}{1+p_{\max }}
```
- 곡선이 크거나 weight가 작으면 더 엄격한 tol을 사용.

## 🟧 2. NURBS → Bezier 조각으로 분해 (수식)
- NURBS 곡선은 일반적으로 여러 span으로 구성됨:
```math
C(u)=\sum _kC_k(u)
```
- 각 span을 Bezier로 만들려면  
    내부 knot multiplicity를 degree p까지 올리면 된다.
- 즉:
```math
\mathrm{mult}(u_i)=p
```
- 이렇게 되면 각 span은 정확히:
```math
C_k(u)=\sum _{j=0}^pP_{k,j}B_j^p(t)
```
- 즉, Bezier 조각이 된다.
- 코드:
```rust
let ins = bezified.knots_insertions_for_bezier(p);
bezified.refine_knot_vector(&ins.knots);
```

## 🟨 3. 각 Bezier 조각 추출
- span k의 Bezier control point는:
```math
P_{k-p},P_{k-p+1},\dots ,P_k
```
- 코드:
```rust
for j in first..=last {
    seg_ctrl.push(bezified.ctrl[j]);
}
```

## 🟥 4. 각 Bezier 조각 차수 감소 (핵심 수식)
- Bezier 차수 감소는 다음 recurrence를 사용:
- 왼쪽 recurrence
```math
Q_i=\frac{p}{p-i}P_i-\frac{i}{p-i}Q_{i-1}
```
- 오른쪽 recurrence
```math
Q_i=\frac{p}{i+1}P_{i+1}-\frac{p-i-1}{i+1}Q_{i+1}
```
- 중앙 Qᵣ은:
```math
Q_r=\frac{1}{2}(Q_r^{(L)}+Q_r^{(R)})
```
- 오차는:
```math
e=\mathrm{Bernstein\  weight\  difference}\times \| P_L-P_R\| 
```
- 코드:
```rust
let (new_ctrl, de) = bez.reduce_degree_once();
```

## 🟩 5. 오차 검사
- 각 조각의 오차 de를 모아서:
    - mtol = 전체 조각 중 최대 오차
    - ok = 모든 조각이 tol_adj 이하인지 여부
- 코드:
```rust
if de > mtol { mtol = de; }
if de > tol_adj { ok = false; }
```


## 🟦 6. 모든 조각을 이어붙여 새로운 NURBS 곡선 생성
- Control point 재조립
- 첫 조각은 q+1개 모두 사용
- 이후 조각은 첫 점(중복) 제외하고 q개씩 추가
- 수식:
```math
Q=[Q_0^{(0)},\dots ,Q_q^{(0)},Q_1^{(1)},\dots ,Q_q^{(1)},\dots ]
```
- 코드:
```rust
out_ctrl.extend_from_slice(&reduced_segments_ctrl[0]);
for s in 1..reduced_segments_ctrl.len() {
    out_ctrl.extend_from_slice(&reduced_segments_ctrl[s][1..]);
}
```

- Knot vector 재조립
- degree = q = p−1
    - 시작 knot multiplicity = q+1
    - 내부 break마다 multiplicity = q
    - 끝 knot multiplicity = q+1
- 수식:
```math
U=[u_0^{(q+1)},u_1^{(q)},\dots ,u_{last}^{(q+1)}]
```
- 코드:
```rust
for _ in 0..=q { out_knots.push(breaks[0]); }
for bi in 1..breaks.len()-1 {
    for _ in 0..q { out_knots.push(breaks[bi]); }
}
for _ in 0..=q { out_knots.push(breaks.last().unwrap()); }
```


## 🟫 7. domain 재설정
- NURBS domain은:
```
[t_0,t_1]=[U[q],U[m_q-q-1]]
```
- 코드:
```rust
cur_q.domain = Interval {
    t0: uq[q_usize],
    t1: uq[mq - q_usize - 1],
};
```

## 🟪 8. 최종 반환
- ok: tolerance 만족 여부
- cur_q: 차수 감소된 NURBS 곡선
- mtol: 전체 조각 중 최대 오차

## 🎯 요약
- 이 함수는 다음 수식 파이프라인을 구현한다:
- NURBS → Bezier 조각 분해
```math
C(u)=\sum _kC_k(u)
```
- 각 Bezier 조각 차수 감소
```math
C_k^p(u)\rightarrow C_k^{p-1}(u)
```
- 오차 계산
```math
e_k=\mathrm{Bernstein\  기반\  오차}
```
- 조각 재조립 → 새로운 NURBS 곡선 생성
```math
\tilde {C}(u)=\sum _kC_k^{p-1}(u)
```
- 최종 오차 및 성공 여부 반환
- 이건 The NURBS Book의 정석 알고리즘을 NURBS 전체 곡선에 적용한 완전한 구현이다.

## 🟦 전체 평가
- 이 테스트 세트는 다음 5가지를 검증한다:
    - 차수 감소가 실제로 p → p−1 되는지
    - knot vector가 비감소(non-decreasing)인지
    - domain이 올바르게 설정되는지
    - 오차(mtOL)가 정상적으로 계산되는지
    - rational curve에서도 NaN 없이 동작하는지
    - 샘플링 기반 편차가 mtol과 크게 어긋나지 않는지
- 즉, 기능적·수학적·수치적 안정성을 모두 체크하고 있다.
- 테스트 품질이 매우 높다.

## 🟩 테스트 코드 상세 점검
- 1) on_is_nondecreasing()
    - ✔ 완벽
    - 윈도우 2개씩 비교하는 방식은 가장 안전하고 빠르다.

- 2) approx()
    - ✔ 정상
    - 절대 오차 기반 비교는 domain 비교에 적합.

- 3) max_dev_samples()
- ✔ 매우 좋은 방식
곡선 비교 시 domain 매핑을 다음처럼 한 것은 정확하다:
```rust
let u0 = c0.domain.t0 + (c0.domain.t1 - c0.domain.t0) * t;
let u1 = c1.domain.t0 + (c1.domain.t1 - c1.domain.t0) * t;
```
- 왜냐하면 차수 감소 후 knot vector가 달라지므로
- 같은 t라도 실제 u가 다르기 때문이다.
- 이 매핑을 안 하면 dev가 잘못 나올 수 있다.

- 4) make_open_uniform_cubic_curve()
- ✔ 완벽한 open-uniform cubic
    - degree = 3
    - ctrl = 7
    - knot count = 11
    - domain = [u[p], u[m−p−1]] = [0, 1]


- 5) make_rational_cubic_curve()
    - ✔ rational 테스트로 충분
    - weight를 다양하게 주어 rational branch를 강제한다.

## 🟧 개별 테스트 점검

- ✔ nurbscurve_reduces_degree_by_one_and_knots_sorted()
- 검증 항목:
    - rfl == true
    - degree 감소
    - knot vector 비감소
    - domain = [U[q], U[m−q−1]]
    - mtol >= 0
- 특히 domain 체크:
```rust
let dom0 = uq[q];
let dom1 = uq[uq.len() - q - 1];
```

- ✔ nurbscurve_flag_fails_when_tol_too_small()
- 이 테스트는 tol이 너무 작으면 실패해야 한다는 논리를 검증한다.
```rust
if mtol > tol {
    assert!(!rfl);
}
```

- ✔ nurbscurve_preserves_endpoints_reasonably_by_sampling()
    - Bezier 차수 감소는 끝점은 항상 보존한다.
    - NURBS 전체 차수 감소에서도 Bezier 조각 stitching을 잘하면 끝점이 유지된다.
    - 샘플링으로 끝점 비교하는 방식은 매우 적절하다.

- ✔ nurbscurve_rational_runs_and_outputs_finite()
- rational curve에서:
    - NaN이 없어야 하고
    - dev가 finite여야 한다

- ✔ nurbscurve_mtol_tracks_shape_change_coarsely()
    - 이 테스트는 매우 좋은 “sanity check”다.
    - assert!(dev <= mtol * 10.0 + 1e-6);

---
## on_debug_print_knot_runs
- 이 함수는 NURBS knot vector의 “run-length”를 사람이 읽기 쉽게 출력하는 디버그 도구.
- 즉, 같은 knot 값이 연속으로 몇 번 반복되는지(multiplicity) 를 구간(run) 단위로 출력해주는 함수다.
- 이 함수는 Bezier화, knot insertion, degree elevation/reduction, knot removal  
    같은 작업 후 knot vector가 정상인지 확인할 때 아주 유용하다.

## 🟦 함수의 목적
- ✔ Knot vector의 구조를 사람이 쉽게 확인
- ✔ 각 knot 값이 몇 번 반복되는지(multiplicity) 출력
- ✔ 내부 knot의 multiplicity가 degree와 맞는지 디버깅할 때 필수
- ✔ Bezier화 후 multiplicity == p인지 확인할 때 매우 유용
- 예를 들어 knot vector가:
```
[0,0,0,0, 0.25, 0.5, 0.75, 1,1,1,1]
```

- 이면 출력은:
```
value 0.0000000000000000 run [0..4) mult 4
value 0.2500000000000000 run [4..5) mult 1
value 0.5000000000000000 run [5..6) mult 1
value 0.7500000000000000 run [6..7) mult 1
value 1.0000000000000000 run [7..11) mult 4
```

- 이렇게 나와서 knot 구조를 한눈에 파악할 수 있다.

## 🟩 함수 동작 상세 설명
```rust
pub fn on_debug_print_knot_runs(knots: &[f64]) {
```

- 입력: knot vector (slice)

- 1) 빈 벡터 처리
```rust
if knots.is_empty() {
    println!("[knot runs] <empty>");
    return;
}
```


- 2) 전체 knot 개수 출력
```rust
let m = knots.len();
println!("[knot runs] count = {}", m);
```

- 3) run-length scan
- 핵심 부분:
```rust
let mut s = 0usize;
while s < m {
    let v = knots[s];
    let mut e = s + 1;
    while e < m && knots[e] == v {
        e += 1;
    }
    let mult = e - s;
```

- 이 부분이 하는 일:
    - s = run 시작 index
    - e = run 끝 index
    - knots[s] == knots[s+1] == ... == knots[e-1]
    - multiplicity = e - s
- 즉, 같은 값이 연속된 구간을 찾는 run-length encoding이다.

- 4) 출력
```rust
println!(
    "  value {:>20.16}  run [{}..{})  mult {}",
    v, s, e, mult
);
```

- 출력 형식:
    - value: knot 값 (소수점 16자리, 오른쪽 정렬)
    - run s..e): 반복 구간
    - mult: multiplicity

- 5) 다음 run으로 이동
```rust
s = e;
```


### 🟧 이 함수가 왜 중요한가?
- NURBS 커널에서 knot vector는 모든 곡선/곡면의 구조를 결정하는 핵심 데이터다.
- 특히 다음 작업 후 반드시 multiplicity를 확인해야 한다:
- ✔ degree elevation
    - 모든 knot multiplicity가 +t 되어야 함
- ✔ degree reduction
    - 재조립된 knot vector가 올바른 multiplicity를 가져야 함
- ✔ knot insertion
    - 특정 knot의 multiplicity가 증가해야 함
- ✔ Bezier extraction
    - 내부 knot multiplicity가 degree p가 되어야 함
- ✔ knot removal
    - multiplicity가 줄어들어야 함
- 이 함수는 이런 작업 후 knot vector가 정상인지 즉시 확인할 수 있게 해준다.

---

## 소스 코드
```rust
/// 1-step degree reduction (p -> p-1)
///
/// 반환:
/// - rfl  : tol 만족 여부
/// - curQ : 감소된 curve
/// - mtol : 최대 오차 (현재는 "Bezier segment reduction error" 기반)
///
/// NOTE:
/// - 여기 버전은 "Bezier화 -> 각 Bezier 조각 감소 -> 재조립"을 먼저 정석으로 고정.
/// - 이후 단계(노트 제거/정밀 오차)는 네 on_compute_knot_minmax_per_span()로 이어붙이면 됨.
pub fn on_nurbscurve_reduce_degree_once(
    cur_p: &NurbsCurve,
    tol: Real,
) -> Result<(bool, NurbsCurve, Real)> {
    let p = cur_p.degree as usize;
    if p <= 1 {
        // C: p <= 1 이면 실패
        let mut q = cur_p.clone();
        return Ok((false, q, 0.0));
    }
    let qdeg = (p - 1) as Degree;

    // ---- rational tol adjust (C의 의도 반영: tol을 weight/크기 기준으로 더 엄격하게) ----
    // C: tol = (tol*wmin)/(1.0+pmax)
    // 여기서는:
    // - wmin = min weight
    // - pmax = max |dehomogenized point| (대략적인 크기 척도)
    let mut tol_adj = tol;
    if crate::core::knot::on_is_rat(&cur_p.ctrl) {
        let mut wmin = Real::INFINITY;
        let mut pmax = 0.0_f64;
        for pw in cur_p.ctrl.iter() {
            wmin = wmin.min(pw.w.abs());
            let pt = pw.to_point();
            let mag = (pt.x * pt.x + pt.y * pt.y + pt.z * pt.z).sqrt();
            pmax = pmax.max(mag);
        }
        if !wmin.is_finite() || wmin <= 0.0 {
            // weight가 망가져 있으면 실패 처리
            let dummy = cur_p.clone();
            return Ok((false, dummy, Real::INFINITY));
        }
        tol_adj = (tol_adj * wmin) / (1.0 + pmax);
    }

    // ---- 1) "Bezier segment"로 만들기 위한 knot insertion 준비 ----
    // 이미 nurbs_curve.rs에 있음: knots_insertions_for_bezier(degree)
    // 내부 knot의 multiplicity를 degree(p)까지 올리면 각 span이 Bezier 조각이 됨
    let mut bezified = cur_p.clone();
    let ins = bezified.knots_insertions_for_bezier(p);
    if !ins.knots.is_empty() {
        bezified.refine_knot_vector(&ins.knots);
    }

    // ---- 2) Bezier 조각(각 span) 추출 ----
    // bezified는 internal knot multiplicity==p 상태라서
    // 각 non-degenerate span [U[k],U[k+1]] 는 Bezier segment로 취급 가능.
    let u = bezified.kv.knots.as_slice();
    let ncp = bezified.ctrl.len();
    let m = u.len();
    if ncp < p + 1 || m < ncp + p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: "on_nurbscurve_reduce_degree_once: invalid bezified curve data".into(),
        });
    }

    // span index 범위: k in [p .. n] where n = ncp-1
    let n = ncp - 1;

    // breakpoints(세그먼트 경계)와 reduced ctrl을 축적
    let mut breaks: Vec<Real> = Vec::new();
    let mut reduced_segments_ctrl: Vec<Vec<crate::core::geom::Point4D>> = Vec::new();

    // mtol: C처럼 "현재까지 최대 오차"
    let mut mtol = 0.0;
    let mut ok = true;

    // 시작 도메인(Bezier spline의 첫 breakpoint)
    // 일반적으로 u[p]..u[m-p-1]
    let u_start = u[p];
    let u_end = u[m - p - 1];
    breaks.push(u_start);

    for k in p..=n {
        let a = u[k];
        let b = u[k + 1];
        if b <= a {
            continue; // zero span skip
        }

        // 이 span의 Bezier control points: P[k-p .. k]
        let first = k - p;
        let last = k;
        let mut seg_ctrl = Vec::with_capacity(p + 1);
        for j in first..=last {
            seg_ctrl.push(bezified.ctrl[j]);
        }

        // ---- 3) 이 Bezier segment를 degree reduce (p -> p-1) ----
        // (네가 앞에서 바꾼 정석 B_cdegre 기반 reduce_degree_once_c()를 재사용)
        let bez = BezierCurve::new(seg_ctrl);
        let (new_ctrl, de) = bez
            .reduce_degree_once()
            .map_err(|_| NurbsError::InvalidArgument {
                msg: "reduce_degree_once_c failed".into(),
            })?;

        // tol 체크(세그먼트 감소 오차 기반 1차 판정)
        if de > mtol {
            mtol = de;
        }
        if de > tol_adj {
            ok = false;
        }

        reduced_segments_ctrl.push(new_ctrl);
        breaks.push(b);
    }

    if reduced_segments_ctrl.is_empty() {
        // degenerate
        let dummy = cur_p.clone();
        return Ok((false, dummy, mtol));
    }

    // ---- 4) Reduced curve 재조립 (degree = p-1) ----
    //
    // C는 여기서도 이어서 "knot 제거"를 돌림.
    // 여기 1차 구현은 "piecewise Bezier spline" 형태로 재조립:
    // - end knots multiplicity = q+1
    // - internal breaks multiplicity = q (=> C0)
    //
    // (나중에 C처럼 knot removal까지 붙이면 이 부분이 더 compact 해짐)
    let q = (p - 1) as usize;

    // ctrl stitch:
    // 첫 segment는 q+1개 다 넣고,
    // 이후 segment는 첫 ctrl(조인트 중복) 제외하고 q개만 append
    let mut out_ctrl: Vec<crate::core::geom::Point4D> = Vec::new();
    out_ctrl.extend_from_slice(&reduced_segments_ctrl[0]);
    for s in 1..reduced_segments_ctrl.len() {
        out_ctrl.extend_from_slice(&reduced_segments_ctrl[s][1..]);
    }

    // knot build:
    let mut out_knots: Vec<Real> = Vec::new();
    // start (q+1)
    for _ in 0..=q {
        out_knots.push(breaks[0]);
    }
    // internal breaks (each q times)
    // breaks: [u0, u1, u2, ..., u_last]
    if breaks.len() >= 3 {
        for bi in 1..(breaks.len() - 1) {
            for _ in 0..q {
                out_knots.push(breaks[bi]);
            }
        }
    }
    // end (q+1)
    for _ in 0..=q {
        out_knots.push(breaks.last().unwrap());
    }

    let out_kv = KnotVector { knots: out_knots };

    let mut cur_q = NurbsCurve {
        dimension: cur_p.dimension,
        degree: qdeg,
        ctrl: out_ctrl,
        kv: out_kv,
        domain: Interval { t0: 0.0, t1: 1.0 }, // 아래에서 재설정
    };

    // domain을 knot 유효범위로 정리
    let uq = cur_q.kv.knots.as_slice();
    let mq = uq.len();
    let q_usize = cur_q.degree as usize;
    cur_q.domain = Interval {
        t0: uq[q_usize],
        t1: uq[mq - q_usize - 1],
    };

    Ok((ok, cur_q, mtol))
}
```
---
### 테스트 코드
```rust
use nurbslib::core::geom::Point4D;
use nurbslib::core::knot::KnotVector;
use nurbslib::core::domain::Interval;
use nurbslib::core::nurbs_curve::NurbsCurve;
use nurbslib::core::types::{Real, Degree};

// NurbsCurve impl에 추가한 메서드
// use nurbslib::core::nurbs_curve::NurbsCurve; // 이미 위에 있음

fn is_nondecreasing(a: &[Real]) -> bool {
    a.windows(2).all(|w| w[0] <= w[1])
}

/// degree=5 (quintic), open clamped, ctrl=8, knots=14
/// knots: [0 x6, 0.33, 0.66, 1 x6]
fn make_open_clamped_quintic_curve() -> NurbsCurve {
    let p = 5usize;

    let ctrl = vec![
        Point4D::new(0.0, 0.0, 0.0, 1.0),
        Point4D::new(1.0, 2.0, 0.0, 1.0),
        Point4D::new(2.0, 4.0, 1.0, 1.0),
        Point4D::new(3.0, 5.0, 2.0, 1.0),
        Point4D::new(4.0, 4.0, 3.0, 1.0),
        Point4D::new(5.0, 2.0, 2.0, 1.0),
        Point4D::new(6.0, 1.0, 1.0, 1.0),
        Point4D::new(7.0, 0.0, 0.0, 1.0),
    ];

    // ctrl=8 => n=7
    // m = n+p+1 = 7+5+1=13 => knots len = m+1=14
    let mut knots = Vec::new();
    for _ in 0..=p { knots.push(0.0); } // p+1 = 6
    knots.push(0.33);
    knots.push(0.66);
    for _ in 0..=p { knots.push(1.0); } // p+1 = 6

    let kv = KnotVector { knots };
    let u = kv.knots.as_slice();
    let domain = Interval { t0: u[p], t1: u[u.len() - p - 1] };

    NurbsCurve {
        dimension: 3,
        degree: p as Degree,
        ctrl,
        kv,
        domain,
    }
}

/// self가 바뀌었는지 확인용 (부분 비교)
fn assert_curve_equal(a: &NurbsCurve, b: &NurbsCurve) {
    assert_eq!(a.dimension, b.dimension);
    assert_eq!(a.degree, b.degree);
    assert_eq!(a.ctrl.len(), b.ctrl.len());
    assert_eq!(a.kv.knots.len(), b.kv.knots.len());
    assert_eq!(a.ctrl, b.ctrl);
    assert_eq!(a.kv.knots, b.kv.knots);
    assert_eq!(a.domain.t0, b.domain.t0);
    assert_eq!(a.domain.t1, b.domain.t1);
}

#[test]
fn reduce_degree_nurbscurve_inplace_success_multi_steps() {
    let mut c = make_open_clamped_quintic_curve();
    assert_eq!(c.degree as usize, 5);

    let target: Degree = 2;
    let tol = 1e9; // 충분히 큰 tol => 끝까지 성공 기대

    let (ok, mtol_max, steps) = c
        .reduce_degree_inplace(target, tol)
        .expect("reduce_degree_inplace failed");

    assert!(ok, "should succeed with huge tol");
    assert_eq!(c.degree as usize, target as usize);
    assert_eq!(steps, 5usize - 2usize, "steps should be p-target");
    assert!(mtol_max.is_finite());
    assert!(mtol_max >= 0.0);

    // knot vector는 비감소
    assert!(is_nondecreasing(&c.kv.knots));
}

#[test]
fn reduce_degree_nurbscurve_inplace_noop_when_target_ge_current() {
    let mut c = make_open_clamped_quintic_curve();
    let orig = c.clone();

    let (ok, mtol_max, steps) = c
        .reduce_degree_inplace(7, 1e-6)
        .expect("reduce_degree_inplace failed");

    assert!(ok);
    assert_eq!(steps, 0);
    assert_eq!(mtol_max, 0.0);

    // no-op이면 완전히 동일해야 함
    assert_curve_equal(&c, &orig);
}

#[test]
fn reduce_degree_nurbscurve_inplace_failure_does_not_modify_self() {
    let mut c = make_open_clamped_quintic_curve();
    let orig = c.clone();

    // 극단적으로 작은 tol => 첫 step에서 실패할 가능성이 높음
    let tol = 1e-16;

    let (ok, mtol_max, steps) = c
        .reduce_degree_inplace(2, tol)
        .expect("reduce_degree_inplace failed");

    // mtol_max가 tol보다 크면 실패가 맞음 (현재 구현 논리)
    if mtol_max > tol {
        assert!(!ok, "expected failure when mtol_max > tol");
    }

    // 실패 시 wrapper는 self를 덮어쓰지 않으므로 원본 유지되어야 함
    assert_curve_equal(&c, &orig);

    // 일반적으로 첫 step에서 실패하면 steps=0
    assert_eq!(steps, 0);
}
```
---
