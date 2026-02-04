

# on_curve_equally_spaced_points


- 이 함수는 NURBS 커널에서 꽤 중요한 기능

## 📘 on_curve_equally_spaced_points 
- 수식 및 함수 상세 설명
### 1. 목적
- 이 함수는 NURBS 곡선 구간 $[u_0,u_1]$ 에서
    길이가 거의 동일한 n개의 segment를 갖도록  
    n+1개의 점을 생성하는 알고리즘이다.  
- 즉:
    - 곡선 길이를 균등하게 나누고 싶을 때
    - 곡선을 일정 간격으로 샘플링하고 싶을 때
    - marching, offset, tessellation, trimming 등에서 필요할 때  
        사용되는 표준적인 equal-length parameter refinement 알고리즘이다.

### 2. 수학적 정의
- 곡선 C(u)에서 n+1개의 점을 찾고 싶다:
```math
Q_i=C(t_i),\quad i=0,1,\dots ,n
```
- 이때 segment 길이는:
```math
d_i=\| Q_i-Q_{i-1}\|
``` 
- 전체 길이:
```math
L=\sum _{i=1}^nd_i
```
- 평균 segment 길이:
```math
\mathrm{aver}=\frac{L}{n}
```
- 균등 분할 조건(tolerance 기반):
```math
\max _i\frac{|d_i-\mathrm{aver}|}{\mathrm{aver}}\leq \mathrm{tol}
```
- 즉:
    - 모든 segment 길이가 평균 길이와 거의 같아야 한다
    - 오차 비율이 tol 이하이면 “수렴(converged)”으로 판단

## 3. 알고리즘 개요 (Piegl N_CURESP)
- 이 알고리즘은 다음을 반복한다:
- 현재 파라미터 $t_i$ 로 곡선을 평가하여 점 $Q_i$ 생성
- segment 길이 $d_i$ 계산
    - 누적 길이 $s_i=\sum _{k=1}^id_k$ 계산
    - 평균 길이 $\mathrm{aver}=s_n/n$ 계산
    - segment 길이가 평균과 얼마나 다른지 검사
- 오차가 크면
    - 누적 길이 기반으로 새로운 $t_i$ 를 보간(interpolation)
- 다시 반복
    - ITLIM 반복 후에도 수렴하지 않으면
    - 현재 상태를 그대로 반환

## 4. 핵심 수식: t 값 재계산
- 목표 누적 길이:
```math
\mathrm{target_{\mathnormal{i}}}=i\cdot \mathrm{aver}
```
- 이 target이 실제 누적 길이 s_k 사이에 있다고 하면:
```math
s_{k-1}\leq \mathrm{target_{\mathnormal{i}}}\leq s_k
```
이때 새로운 파라미터는 선형 보간으로 계산:
```math
t_i=t_{k-1}^{old}+\frac{t_k^{old}-t_{k-1}^{old}}{s_k-s_{k-1}}\cdot (\mathrm{target_{\mathnormal{i}}}-s_{k-1})
```
- 이 수식이 코드의 다음 부분과 정확히 대응한다:
```rust
let num = oldt[k] - oldt[k - 1];
let den = s[k] - s[k - 1];
t[i] = (num / den) * (target - s[k - 1]) + oldt[k - 1];
```


## 5. 코드 상세 해설
### 5.1 초기 파라미터 설정
```rust
t[0] = u0;
t[n] = u1;
t[i] = u0 + i * (u1 - u0) / n;
```

- 즉, 초기값은 균등한 parameter 분할이다.  
    (길이 균등이 아니라 parameter 균등)

### 5.2 첫 점 계산
```rust
q[0] = on_eval_curve_point_side(cur, u0, Side::Left)?;
```


### 5.3 반복 루프
- (1) 곡선 평가 및 누적 길이 계산
```rust
q[i] = C(t[i])
s[i] = s[i-1] + |q[i] - q[i-1]|
```


- (2) 평균 길이 계산
```rust
aver = s[n] / n
```


- (3) segment 길이 편차 검사
```
dev = |seg - aver|
ratio = dev / aver
if ratio > tol → 수렴 실패
```


- (4) 수렴 또는 반복 종료
- 모든 segment가 tol 이하 → 성공
- 반복 횟수 ITLIM 초과 → best effort 반환

- (5) 새로운 t 계산
- 누적 길이 기반으로 target 길이를 찾고 그에 대응하는 t를 선형 보간한다.

## 6. 반환값
```rust
Ok((Option<Vec<Point3D>>, Option<Vec<Real>>))
```
- want_points = false → 점을 계산하지 않고 파라미터만 반환
- want_params = false → 파라미터 없이 점만 반환
- 둘 다 true → 둘 다 반환

## 7. 언제 쓰면 좋은가?
### ✔ 7.1 곡선 tessellation
- 곡선을 일정 길이 간격으로 샘플링하여 polyline 생성.
### ✔ 7.2 marching 알고리즘
- 곡선 위에서 일정한 arc-length step으로 이동해야 할 때.
### ✔ 7.3 trimming curve sampling
- 트리밍 곡선을 균등하게 샘플링하여 정확한 경계 생성.
### ✔ 7.4 offset curve 생성
- offset marching step을 일정하게 유지해야 할 때.
### ✔ 7.5 곡선 길이 기반 parameterization
- 곡선을 arc-length parameter로 재매핑할 때.

## 8. 장점
- arc-length 기반의 균등 분할
- 수렴 기준이 명확하고 robust
- ITLIM 초과 시에도 best-effort 반환 (C와 동일)

## 9. 단점 / 주의사항
- 곡선이 매우 구불구불하면 iteration이 많이 필요
- 곡선이 거의 직선이면 빠르게 수렴
- parameter domain이 매우 비선형이면 t 보간이 불안정할 수 있음
- tol이 너무 작으면 수렴하지 않을 수 있음

## ⭐ 최종 요약
- on_curve_equally_spaced_points()는:
    - 곡선 구간을 길이 기준으로 균등하게 분할하는 알고리즘
    - 누적 길이 기반의 iterative parameter refinement
    - tolerance 기반의 수렴 판정
    - tessellation, marching, trimming 등에서 핵심적으로 사용됨
---

## 🎯 t값을 찾는 순서 요약
- ✔ t 배열은 처음엔 등간격 (parameter uniform)
- ✔ 반복하면서 t 배열을 수정
- ✔ 목표는 “곡선 길이 기준으로 등간격”
- ✔ 즉, t 배열이 점점 “arc-length parameterization”에 가까워짐

## 🔍 단계별로 보면 정확히 이런 구조
- 1) 초기 t 값
```rust
t[0] = u0
t[n] = u1
t[i] = u0 + i*(u1-u0)/n
```
- 즉, parameter uniform.

- 2) 현재 t 값으로 곡선 평가 → Q[i]
```rust
Q[i] = C(t[i])
```
- 3) segment 길이 계산
```
d[i] = |Q[i] - Q[i-1]|
```
- 4) 누적 길이 s[i] 계산
```rust
s[i] = d[1] + d[2] + ... + d[i]
```

- 5) 평균 segment 길이
```rust
aver = s[n] / n
```
- 6) 각 segment가 평균과 얼마나 다른지 검사
```rust
ratio = |d[i] - aver| / aver
```
- ratio가 tol 이하이면 수렴.

- 7) 수렴 안 되면 t 값을 다시 계산
- 이 부분이 핵심.
- 목표 누적 길이
```
target_i = i * aver
```

- 즉,
- 0, aver, 2aver, 3aver, …, n*aver
- 이렇게 길이 기준 등간격이 되도록 목표를 잡는다.
- 그리고 이 target이 실제 누적 길이 s[k] 사이에 오도록
- 선형 보간으로 t[i]를 다시 계산한다.
```rust
t[i] = t[k-1] + (t[k] - t[k-1]) * (target_i - s[k-1]) / (s[k] - s[k-1])
```
## ⭐ 결론:
- ✔ 맞아, 이 알고리즘은 “t 배열을 단계적으로 조정해서 등간격이 되도록 만드는 구조”다
- 하지만 중요한 건:
- ❌ t 배열이 등간격이 되는 게 아니라
✔-  t 배열로 평가한 곡선 점들의 “길이”가 등간격이 되도록 t를 조정하는 것
- 즉:
  - parameter uniform → arc-length uniform으로 iterative하게 변환하는 알고리즘

## 🎨 직관적으로 보면
- 초기:
```
t: 0   0.33   0.66   1.0
Q: |----|--|------|
    d1   d2   d3   (길이가 제각각)
```

- 반복 후:
```
t: 0   0.28   0.62   1.0
Q: |----|----|----|
    d1   d2   d3   (길이가 거의 동일)
```
---

## 소스 코드
```rust

/// Compute n+1 approximately equally spaced points on a NURBS curve between u0..u1.
///
/// - tol condition (C semantics):
///   let aver = total_length / n
///   let maxdev = max_i |dist(i-1,i) - aver|
///   require maxdev/aver <= tol
///
/// - If ITLIM iterations are exceeded, returns the best current solution (same as C).
///
/// Returns:
/// - Ok((points_opt, params_opt))
///   - If `want_points=false`, points_opt=None
///   - If `want_params=false`, params_opt=None
pub fn on_curve_equally_spaced_points(
    cur: &NurbsCurve,
    u0: Real,
    u1: Real,
    n: usize,          // generate n+1 points (n > 1)
    tol: Real,
    itlim: usize,      // C global ITLIM
    want_points: bool, // C: P != NULL
    want_params: bool, // C: u != NULL
) -> Result<(Option<Vec<Point3D>>, Option<Vec<Real>>)> {
    const RNAME: &str = "on_curve_equally_spaced_points";

    if n <= 1 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("{RNAME}: n must be > 1"),
        });
    }
    if !tol.is_finite() || tol < 0.0 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("{RNAME}: tol must be finite and >= 0"),
        });
    }

    {
        let knots = cur.kv.knots.as_slice();
        let umin = knots[0];
        let umax = knots[knots.len() - 1];
        on_check_param_range(u0, umin, umax)?;
        on_check_param_range(u1, umin, umax)?;
    }

    // ---- allocate and init t/oldt/s/Q ----
    let mut q: Vec<Point3D> = vec![Point3D { x: 0.0, y: 0.0, z: 0.0 }; n + 1];
    let mut t: Vec<Real> = vec![0.0; n + 1];
    let mut oldt: Vec<Real> = vec![0.0; n + 1];
    let mut s: Vec<Real> = vec![0.0; n + 1];

    oldt[0] = u0;
    oldt[n] = u1;
    t[0] = u0;
    t[n] = u1;

    let dt = (u1 - u0) / (n as Real);
    for i in 1..n {
        t[i] = u0 + (i as Real) * dt;
    }

    // ---- initial start point ----
    q[0] = on_eval_curve_point_side(cur, u0, Side::Left)?;

    // ---- iterate ----
    s[0] = 0.0;

    for its in 1..=itlim {
        // Compute points and cumulative distances s[i]
        for i in 1..=n {
            q[i] = on_eval_curve_point_side(cur, t[i], Side::Left)?;
            let d = q[i - 1].distance(&q[i]);
            s[i] = s[i - 1] + d;
        }

        let aver = s[n] / (n as Real);

        // If aver is ~0, curve segment is degenerate -> treat as converged
        if !aver.is_finite() || aver.abs() <= 1e-30 {
            let params_opt = if want_params { Some(t.clone()) } else { None };
            let points_opt = if want_points { Some(q.clone()) } else { None };
            return Ok((points_opt, params_opt));
        }

        // Check deviations
        let mut i_break: Option<usize> = None;
        for i in 1..=n {
            let seg = s[i] - s[i - 1];
            let dev = (seg - aver).abs();
            let ratio = dev / aver.abs();
            if !ratio.is_finite() {
                return Err(NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: numeric issue dev/aver (dev={dev}, aver={aver})"),
                });
            }
            if ratio > tol {
                i_break = Some(i);
                break;
            }
        }

        // Converged or exceeded iteration limit -> return best current
        if i_break.is_none() || its >= itlim {
            let params_opt = if want_params { Some(t.clone()) } else { None };
            let points_opt = if want_points { Some(q.clone()) } else { None };
            return Ok((points_opt, params_opt));
        }

        // Recompute t-values using linear interpolation on cumulative distances
        // swap(t, oldt) like C
        std::mem::swap(&mut t, &mut oldt);

        let mut k = 1usize;
        for i in 1..n {
            let target = (i as Real) * aver;
            while k <= n && target > s[k] {
                k += 1;
            }
            if k > n {
                // Shouldn't happen, but clamp safely
                t[i] = oldt[n];
                continue;
            }

            let num = oldt[k] - oldt[k - 1];
            let den = s[k] - s[k - 1];
            if den.abs() <= 1e-30 || !den.is_finite() || !num.is_finite() {
                return Err(NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: division issue in interpolation (num={num}, den={den})"),
                });
            }

            t[i] = (num / den) * (target - s[k - 1]) + oldt[k - 1];
        }
    }

    // If loop exits (itlim==0 etc.) return whatever we have
    let params_opt = if want_params { Some(t) } else { None };
    let points_opt = if want_points { Some(q) } else { None };
    Ok((points_opt, params_opt))
}
```
---
