# 📘 NURBS Approximate Knot Removal
- 이 문서는 다음 네 가지 핵심 요소로 구성된다:
    - NURBS knot removal의 수학적 배경
    - 각 함수의 역할과 수식
    - 전체 알고리즘 흐름
    - Rust 구현과의 매핑

## 1. 📐 NURBS Knot Removal의 수학적 배경
- NURBS 곡선은 다음과 같이 정의된다:
```math
C(u)=\frac{\sum _{i=0}^nN_{i,p}(u)w_iP_i}{\sum _{i=0}^nN_{i,p}(u)w_i}
```
- 여기서
    - $N_{i,p}(u)$: B-spline basis
    - $P_i$: 제어점
    - $w_i$: weight
    - p: degree
    - knot vector: $U=\{ u_0,u_1,\dots ,u_m\}$ 

### 🎯 Knot Removal의 목표
- 어떤 내부 knot u_r을 제거하면:
    - 제어점 개수가 줄어들고
    - 곡선이 더 단순해지지만
    - 곡선 형상이 변형될 수 있다
- 따라서 형상 오차가 tolerance 이하일 때만 제거해야 한다.

### 🎯 Approximate Knot Removal의 핵심 아이디어
- 정확한 오차 계산은 매우 비싸므로,  
    다음과 같은 **빠른 upper bound(상한)** 를 사용한다:
```math
\mathrm{error}\leq \max _i\| P_i-P_{i+s}\|
``` 
- 여기서
    - s = multiplicity
    - $P_i$, $P_{i+s}$ = 동차좌표 제어점의 유클리드 복원
- 이 upper bound는 곡선 오차의 보수적 추정치이며,  
    Piegl & Tiller의 NURBS 책에서도 사용되는 방식이다.

## 2. 🧩 함수별 역할 및 수식
- 아래는 네가 새로 정리한 함수 이름을 기준으로 설명한다.

### 2.1 remove_one_knot_occurrence
- ✔ 역할
    - 특정 knot block의 마지막 인덱스 r_end에서 정확히 1개의 knot만 제거한다.
- ✔ 수식
    - block multiplicity = multi
    - block 시작 인덱스:
```math
s_{\mathrm{start}}=r_{\mathrm{end}}+1-\mathrm{multi}
```
- Rust 구현:
let s_start = r_end + 1 - multi;
```rust
self.remove_knot(r_end, s_start, 1);
```

- ✔ 의미
    - **한 번 제거** 를 Rust에서 안전하게 래핑한 함수
    - block 단위가 아니라 occurrence 단위로 제거

### 2.2 KnotRemovalState
- ✔ 역할
    - knot removal 과정에서 필요한 모든 상태를 저장하는 구조체.
- ✔ 구성 요소

| Field             | Meaning                                                                 |
|-------------------|-------------------------------------------------------------------------|
| removal_bound[r]  | Estimated error bound if the knot block ending at index r is removed    |
| accumulated_err[i]| Accumulated error used in knot interval i (tracks tolerance consumption)|
| multiplicity[r]   | Multiplicity of the knot block whose last index is r (0 if not a block end) |

- ✔ 수식
    - 없음 (상태 저장용)

### 2.3 on_estimate_removal_error_bound
- ✔ 역할
- block 끝 인덱스 r_end에서 multiplicity multi를 가진 knot block을 한 번 제거했을 때의      오차 상한을 계산한다.
- ✔ 수식
    - 영향받는 제어점 인덱스 범위:
```math
i=r_{\mathrm{end}}-p,\; \dots ,\; r_{\mathrm{end}}-\mathrm{multi}
```
- 각 i에 대해:
```math
d_i=\| P_i-P_{i+\mathrm{multi}}\|
``` 
- 오차 상한:
```math
b(r_{\mathrm{end}})=\max _id_i
```
- ✔ 의미
    - 실제 곡선 오차를 계산하지 않고
    - 제어점 쌍 거리의 최대값으로 빠르고 보수적인 upper bound를 만든다.
    - Piegl & Tiller의 approximate knot removal 방식과 동일.

### 2.4 on_init_knot_removal_candidates_and_bounds
- ✔ 역할
    - 현재 곡선의 모든 interior knot block을 스캔하여:
        - block 끝 인덱스 r_end
        - multiplicity s
        - removal bound b(r_end)
    - 을 계산하여 KnotRemovalState에 저장한다.
- ✔ 수식
    - block 탐색:
    ```math
    r_{\mathrm{end}}=\max \{ j\mid U_j=U_r\}
    ``` 
    - multiplicity:
    ```math
    s=r_{\mathrm{end}}-r+1
    ```

### 2.5 on_try_remove_one_occurrence_with_bound
- ✔ 역할
    - 특정 block 끝 인덱스 r에 대해:
    - 오차 budget 검사
    - 조건 만족 시 knot 1개 제거
    - 누적 오차 업데이트
- ✔ 수식
- (1) 영향 구간 계산
    - Piegl & Tiller의 공식:
    - 홀수/짝수에 따라:
```math
k=\left\{ \, \begin{array}{ll}\textstyle \frac{p+s+1}{2},&\textstyle (p+s)\mathrm{\  odd}\\ \textstyle \frac{p+s}{2},&\textstyle (p+s)\mathrm{\  even}\end{array}\right. 
\mathrm{start}=r-k
```
```math
\mathrm{end}=r-k+p\quad \mathrm{또는}\quad r-k+p+1
```
- (2) 오차 budget 검사
```math
\forall i\in [\mathrm{start},\mathrm{end}]\mathrm{\  where\  }U_i\neq U_{i+1}:\quad \mathrm{accumulated\_ err}[i]+b(r)\leq \mathrm{tol}
```
- (3) 제거 수행
```math
\mathrm{remove\_ one\_ knot\_ occurrence}(r,s)
```
- (4) 누적 오차 업데이트
```math
\mathrm{accumulated\_ err}[i]\leftarrow \mathrm{accumulated\_ err}[i]+b(r)
```

### 2.6 on_remove_all_approx_removable_knots
- ✔ 역할
    - 전체 알고리즘의 메인 엔트리.
    - 다음 과정을 반복한다:
        - 모든 block의 removal bound 계산
        - bound가 가장 작은 block 선택
        - 제거 시도
        - 성공하면 다시 1로
        - 실패하면 해당 block은 제거 불가로 표시
        - 더 이상 제거할 knot가 없으면 종료
- ✔ 알고리즘 흐름 (pseudo-code)
```
loop:
    state.resize_to(knot_count)
    init_candidates_and_bounds()

    removed_any = false

    loop:
        r = argmin(removal_bound[r])
        if removal_bound[r] > tol: break

        if try_remove(r):
            removed_any = true
            break
        else:
            removal_bound[r] = NOT_REMOVABLE

    if !removed_any:
        break
```

- ✔ 의미
    - greedy 방식으로 “가장 제거하기 쉬운 knot”부터 제거
    - tolerance를 넘지 않는 선에서 최대한 knot를 제거
    - 곡선 형상을 유지하면서 제어점 수를 줄이는 곡선 단순화 알고리즘

## 3. 🔄 전체 알고리즘 요약
### 1) 후보 knot block 찾기
- on_init_knot_removal_candidates_and_bounds
### 2) 각 block의 제거 오차 상한 계산
- on_estimate_removal_error_bound
### 3) 가장 제거하기 쉬운 block 선택
- removal_bound 최소값
### 4) 오차 budget 검사 후 제거 시도
- on_try_remove_one_occurrence_with_bound
### 5) 제거 성공 시 곡선 업데이트
- remove_one_knot_occurrence
### 6) 더 이상 제거할 knot가 없을 때 종료
- on_remove_all_approx_removable_knots

## 4. 🧠 이 알고리즘이 왜 중요한가?
- NURBS 곡선은 복잡한 형상을 표현할 수 있지만,  
    불필요하게 많은 knot와 control point를 갖는 경우가 많다.
- knot removal은 CAD/CAM/CAE에서 매우 중요한 곡선 단순화(simplification) 기법이다.
- 이 알고리즘은:
    - 빠르고
    - 안정적이며
    - tolerance 기반으로 형상 보존을 보장한다.

---

## 소스 코드
```rust
pub const KNOT_NOT_REMOVABLE_MARKER: Real = 1.0e25;

/// Working state.
#[derive(Clone, Debug)]
pub struct KnotRemovalState {
    /// removal_bound[r] : estimated removal error bound for knot at index r (only meaningful at "block end" indices)
    pub removal_bound: Vec<Real>,
    /// accumulated_err[i] : accumulated error per knot-interval index (C keeps this per i where U[i]!=U[i+1])
    pub accumulated_err: Vec<Real>,
    /// multiplicity[r] : multiplicity of knot block whose last index is r (0 for non-block-end indices)
    pub multiplicity: Vec<usize>,
}
```
```rust
impl KnotRemovalState {
    pub fn new(knot_count: usize) -> Self {
        Self {
            removal_bound: vec![KNOT_NOT_REMOVABLE_MARKER; knot_count],
            accumulated_err: vec![0.0; knot_count],
            multiplicity: vec![0; knot_count],
        }
    }

    fn resize_to(&mut self, knot_count: usize) {
        self.removal_bound.resize(knot_count, KNOT_NOT_REMOVABLE_MARKER);
        self.accumulated_err.resize(knot_count, 0.0);
        self.multiplicity.resize(knot_count, 0);
    }

    fn clear_br_sr(&mut self) {
        for b in &mut self.removal_bound { *b = KNOT_NOT_REMOVABLE_MARKER; }
        for s in &mut self.multiplicity { *s = 0; }
        // er는 누적이라 유지
    }
}
```
```rust
/// Approximate error bound like N_toocrb(cur, r, s, &b).
/// This is a common fast bound used in “approx knot removal”:
/// max distance between paired control points in the affected region.
pub fn on_estimate_removal_error_bound(cur: &NurbsCurve, r_end: usize, multi: usize) -> Result<Real> {
    let p = cur.degree as usize;
    // C uses first=r-p, last=r-s
    if r_end < p || r_end < multi {
        return Ok(KNOT_NOT_REMOVABLE_MARKER);
    }
    let first = r_end - p;
    let last = r_end - multi;

    let mut max_err = 0.0;
    for i in first..=last {
        // pair i and i+s (safe if indices exist)
        let j = i + multi;
        if j >= cur.ctrl.len() {
            break;
        }
        let d = cur.control_point_distance(i, j)?;
        if d > max_err {
            max_err = d;
        }
    }
    Ok(max_err)
}
```
```rust
/// Build br/sr for current curve:
/// - iterate distinct interior knot blocks
/// - mark only the block-end index `r_end` with sr[r_end]=multiplicity, br[r_end]=toocrb(...)
pub fn on_init_knot_removal_candidates_and_bounds(cur: &NurbsCurve, st: &mut KnotRemovalState) -> Result<()> {
    let p = cur.degree as usize;
    let n = match cur.ctrl.len().checked_sub(1) {
        Some(v) => v,
        None => return Ok(()),
    };
    if n <= p { return Ok(()); }


    st.clear_br_sr();

    // interior block ends are in [p+1 .. n] (same as C)
    let mut r = p + 1;
    while r <= n {
        let u = cur.kv.knots[r];
        let mut r_end = r;
        while r_end + 1 <= n && cur.kv.knots[r_end + 1] == u {
            r_end += 1;
        }
        let s = r_end - r + 1;
        st.multiplicity[r_end] = s;
        st.removal_bound[r_end] = on_estimate_removal_error_bound(cur, r_end, s)?;
        r = r_end + 1;
    }

    Ok(())
}
```
```rust
pub fn on_try_remove_one_occurrence_with_bound(cur: &mut NurbsCurve, r: usize,
  st: &mut KnotRemovalState, tol: Real) -> Result<bool> {
    let p = cur.degree as usize;
    let s = st.multiplicity[r];
    if s == 0 {
        return Ok(false);
    }

    let (k, l) = if ((p + s) & 1) == 1 {
        let k = (p + s + 1) / 2;
        let l = r - k + p + 1;
        (k, l)
    } else {
        let k = (p + s) / 2;
        let l = r - k + p;
        (k, l)
    };

    let mut start = r.saturating_sub(k);
    let mut end = l;

    // knot vector 범위로 클램핑
    if cur.kv.knots.len() < 2 {
        return Ok(false);
    }
    if end + 1 >= cur.kv.knots.len() {
        end = cur.kv.knots.len() - 2;
    }
    if start > end {
        return Ok(false);
    }

    let b = st.removal_bound[r];

    // accumulated error check
    for i in start..=end {
        if cur.kv.knots[i] != cur.kv.knots[i + 1] {
            if st.accumulated_err[i] + b > tol {
                return Ok(false);
            }
        }
    }

    // perform ONE removal
    cur.remove_one_knot_occurrence(r, s)?;

    // update accumulated error
    for i in start..=end {
        if cur.kv.knots[i] != cur.kv.knots[i + 1] {
            st.accumulated_err[i] += b;
        }
    }

    Ok(true)
}
```
```rust
/// Public entry: remove all approximately removable knots (global greedy).
///
/// This corresponds to the original idea of N_TOOCRR:
/// - compute error bounds for each distinct interior knot
/// - repeatedly remove the knot with smallest bound if it passes accumulated error test
/// - stop when no removable knot remains
pub fn on_remove_all_approx_removable_knots(cur: &mut NurbsCurve, mut tol: Real) -> Result<()> {
    let p = cur.degree as usize;

    // quick outs
    if cur.ctrl.is_empty() || cur.kv.knots.is_empty() || p == 0 {
        return Ok(());
    }
    let mut n = match cur.ctrl.len().checked_sub(1) {
        Some(v) => v,
        None => return Ok(()),
    };
    if n <= p {
        return Ok(());
    }

    // Rational tolerance adjustment (same idea as your existing remove_knot_tol scaling)
    if cur.is_rational() {
        // nurbs_curve.rs 안에 이미 있는 함수가 있다고 했으니 그걸 사용
        if let Some((w_min, _w_max, _p_min, p_max)) = cur.compute_min_max_weights_and_positions() {
            tol = (tol * w_min) / (1.0 + p_max);
        }
    }

    let mut st = KnotRemovalState::new(cur.kv.knots.len());

    loop {
        // ensure state sizes follow current knot vector
        st.resize_to(cur.kv.knots.len());

        // recompute br/sr for current curve (er stays)
        on_init_knot_removal_candidates_and_bounds(cur, &mut st)?;

        // inner: repeatedly pick current-best br, try remove; if fail, mark NOREM and try next
        let mut removed_any = false;

        loop {
            // recompute n each time (curve can shrink after removal)
            n = match cur.ctrl.len().checked_sub(1) {
                Some(v) => v,
                None => return Ok(()),
            };
            if n <= p {
                return Ok(());
            }

            // find best removable candidate (minimum br) among interior indices
            let mut best_r: Option<usize> = None;
            let mut best_b = KNOT_NOT_REMOVABLE_MARKER;

            // interior indices in knot vector are [p+1 .. n]
            for i in (p + 1)..=n {
                let b = st.removal_bound[i];
                if b < best_b {
                    best_b = b;
                    best_r = Some(i);
                }
            }

            let r = match best_r {
                Some(r) => r,
                None => break,
            };

            if best_b == KNOT_NOT_REMOVABLE_MARKER || best_b > tol {
                break; // nothing else removable
            }

            if on_try_remove_one_occurrence_with_bound(cur, r, &mut st, tol)? {
                removed_any = true;
                // after removal, break to outer loop to rebuild bounds for new curve
                break;
            } else {
                // this knot not removable under accumulated error → mark NOREM and search next
                st.removal_bound[r] = KNOT_NOT_REMOVABLE_MARKER;
            }
        }

        if !removed_any {
            break;
        }
    }
    Ok(())
}
```
```rust
/// tiny deterministic RNG (no rand crate)
pub struct TinyRng(u64);
impl TinyRng {
    pub fn new(seed: u64) -> Self { Self(seed) }
    pub fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.0 >> 32) as u32
    }
    fn next_f64(&mut self) -> f64 {
        let v = self.next_u32() as f64 / (u32::MAX as f64);
        v
    }
    fn next_real(&mut self, lo: Real, hi: Real) -> Real {
        lo + (hi - lo) * (self.next_f64() as Real)
    }
    pub fn range_f64(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next_f64()
    }
}
```
```rust
pub fn on_make_curve_non_rational(p: Degree, n_ctrl: usize, seed: u64) -> NurbsCurve {
    let mut rng = TinyRng::new(seed);
    let mut ctrl = Vec::with_capacity(n_ctrl);
    for _ in 0..n_ctrl {
        let x = rng.next_real(-3.0, 3.0);
        let y = rng.next_real(-3.0, 3.0);
        let z = rng.next_real(-3.0, 3.0);
        ctrl.push(Point4D::non_homogeneous(x, y, z, 1.0)); // non-rational
    }
    NurbsCurve::from_ctrl_clamped_uniform(p, ctrl)
}
```
```rust
pub fn on_make_curve_rational(p: Degree, n_ctrl: usize, seed: u64) -> NurbsCurve {
    let mut rng = TinyRng::new(seed);
    let mut ctrl = Vec::with_capacity(n_ctrl);
    for _ in 0..n_ctrl {
        let x = rng.next_real(-2.0, 2.0);
        let y = rng.next_real(-2.0, 2.0);
        let z = rng.next_real(-2.0, 2.0);
        let w = rng.next_real(0.2, 2.5); // positive weights
        // NOTE: your Point4D convention is non_homogeneous(x,y,z,w) means stored as-is (xw,yw,zw,w) already
        ctrl.push(Point4D::non_homogeneous(x * w, y * w, z * w, w));
    }
    NurbsCurve::from_ctrl_clamped_uniform(p, ctrl)
}
```

### 테스트 코드
```rust
use nurbslib::core::nurbs_curve::on_remove_all_approx_removable_knots;
use nurbslib::core::prelude::{NurbsCurve, Point4D};
```
```rust
#[test]
fn knot_remove_basic_reduces_or_keeps_knots() {
    // 너희 프로젝트의 curve 생성 함수로 교체해도 됨
    let mut c = NurbsCurve::from_ctrl_clamped_uniform(
        3,
        vec![
            Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
            Point4D::homogeneous(1.0, 0.2, 0.0, 1.0),
            Point4D::homogeneous(2.0, 0.0, 0.0, 1.0),
            Point4D::homogeneous(3.0, 0.0, 0.0, 1.0),
            Point4D::homogeneous(4.0, 0.0, 0.0, 1.0),
            Point4D::homogeneous(5.0, 0.0, 0.0, 1.0),
        ],
    );

    let k0 = c.kv.knots.len();
    let n0 = c.ctrl.len();

    on_remove_all_approx_removable_knots(&mut c, 1e-8).unwrap();

    assert!(c.kv.knots.len() <= k0);
    assert!(c.ctrl.len() <= n0);
    // 기본 불변: |U| = |P| + p + 1
    assert_eq!(c.kv.knots.len(), c.ctrl.len() + (c.degree as usize) + 1);
}
```
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::cfun::Degree;
    use nurbslib::core::nurbs_curve::{on_init_knot_removal_candidates_and_bounds,
      on_estimate_removal_error_bound,
      on_remove_all_approx_removable_knots, on_try_remove_one_occurrence_with_bound, NurbsCurve,
      KnotRemovalState, KNOT_NOT_REMOVABLE_MARKER};
    use nurbslib::core::prelude::{KnotVector, Point4D};
    use nurbslib::core::types::Real;

    fn make_simple_nonrational_curve() -> NurbsCurve {
        // 여기는 네 프로젝트의 실제 생성자에 맞게 수정해야 함
        // 예시: degree 2, 4 control points, uniform knot vector
        NurbsCurve::new(
            2 as Degree,
            vec![
                // (x, y, z, w)
                Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
                Point4D::homogeneous(1.0, 0.0, 0.0, 1.0),
                Point4D::homogeneous(2.0, 0.0, 0.0, 1.0),
                Point4D::homogeneous(3.0, 0.0, 0.0, 1.0),
            ],
            KnotVector::from_vec(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0]),
        ).expect("Failed to create new NurbsCurve")
    }
```
```rust
    #[test]
    fn control_point_distance_basic() {
        let cur = make_simple_nonrational_curve();
        let d01 = cur.control_point_distance(0, 1).unwrap();
        let d02 = cur.control_point_distance(0, 2).unwrap();

        assert!((d01 - 1.0).abs() < 1.0e-9);
        assert!((d02 - 2.0).abs() < 1.0e-9);
    }

    #[test]
    fn control_point_distance_out_of_range() {
        let cur = make_simple_nonrational_curve();
        let res = cur.control_point_distance(0, 10);
        assert!(res.is_err());
    }
```
```rust
    #[test]
    fn control_point_distance_zero_weight_error() {
        let mut cur = make_simple_nonrational_curve();
        cur.ctrl[1].w = 0.0;
        let res = cur.control_point_distance(0, 1);
        assert!(res.is_err());
    }
```
```rust
    #[test]
    fn knot_remove_zero_when_indices_invalid() {
        let cur = make_simple_nonrational_curve();
        let p = cur.degree as usize;
        // r < p 인 경우 KNOT_NOT_REMOVABLE_MARKER
        let val = on_estimate_removal_error_bound(&cur, p - 1, 1).unwrap();
        assert_eq!(val, KNOT_NOT_REMOVABLE_MARKER);
    }
```
```rust
    #[test]
    fn knot_remove_positive_for_valid_block() {
        let cur = make_simple_nonrational_curve();
        let p = cur.degree as usize;
        // r >= p, s=1
        let r = p;
        let val = on_estimate_removal_error_bound(&cur, r, 1).unwrap();
        assert!(val >= 0.0);
        assert!(val < KNOT_NOT_REMOVABLE_MARKER);
    }

    #[test]
    fn compute_initial_bounds_sets_sr_and_br() {
        let cur = make_simple_nonrational_curve();
        let mut st = KnotRemovalState::new(cur.kv.knots.len());

        on_init_knot_removal_candidates_and_bounds(&cur, &mut st).unwrap();

        // interior indices [p+1 .. n]
        let p = cur.degree as usize;
        let n = cur.ctrl.len() - 1;

        let mut any_block = false;
        for r in (p + 1)..=n {
            if st.multiplicity[r] > 0 {
                any_block = true;
                assert!(st.removal_bound[r] < KNOT_NOT_REMOVABLE_MARKER);
            }
        }
        assert!(any_block);
    }
```
```rust
    #[test]
    fn try_remove_does_not_panic_on_small_curve() {
        let mut cur = make_simple_nonrational_curve();
        let mut st = KnotRemovalState::new(cur.kv.knots.len());
        on_init_knot_removal_candidates_and_bounds(&cur, &mut st).unwrap();

        let p = cur.degree as usize;
        let n = cur.ctrl.len() - 1;

        // interior 후보 중 하나를 골라서 시도
        for r in (p + 1)..=n {
            let _ = on_try_remove_one_occurrence_with_bound(&mut cur, r, &mut st, 1.0e-3);
        }
    }
```
```rust
    #[test]
    fn remove_all_approx_removable_knots_does_not_increase_ctrl_count() {
        let mut cur = make_simple_nonrational_curve();
        let before_ctrl = cur.ctrl.len();
        let before_knots = cur.kv.knots.len();

        super::on_remove_all_approx_removable_knots(&mut cur, 1.0e-3).unwrap();

        assert!(cur.ctrl.len() <= before_ctrl);
        assert!(cur.kv.knots.len() <= before_knots);
    }
```
```rust
    #[test]
    fn remove_all_approx_removable_knots_respects_zero_tol() {
        let mut cur = make_simple_nonrational_curve();
        let before_ctrl = cur.ctrl.len();
        let before_knots = cur.kv.knots.len();

        on_remove_all_approx_removable_knots(&mut cur, 0.0).unwrap();

        // tol=0이면 거의 제거가 안 되어야 함 (정확히 같아야 한다고 단정하진 않지만)
        assert_eq!(cur.ctrl.len(), before_ctrl);
        assert_eq!(cur.kv.knots.len(), before_knots);
    }
```
```rust
    #[test]
    fn test_offset_straight_line() {

        // x축 위의 직선: (0,0) -> (3,0)
        let curve = NurbsCurve::new(
            1 as Degree,
            vec![
                Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
                Point4D::homogeneous(3.0, 0.0, 0.0, 1.0),
            ],
            KnotVector::from_vec(vec![0.0, 0.0, 1.0, 1.0]),
        ).expect("Failed to create line NurbsCurve");

        let dist: Real = 2.0;
        let offset = curve
            .offset_curve_approx(dist, 16, 1 as Degree, 2)
            .expect("offset_curve_approx failed");

        // 몇 개의 파라미터에서 y좌표가 거의 dist인지 확인
        for &u in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let p = offset.eval_point(u);
            println!("p {}, dist {}", p, dist);
            assert!((p.z - dist).abs() < 1.0e-3, "y={} not close to {}", p.y, dist);
        }
    }
}
```
---


