# 스플라인 


---

## 1. B-spline 기초 루틴과 불변식

### 1.1 매듭 벡터와 클램프
- **클램프(Open)**: 양 끝 매듭의 다중도 $\(\ge p+1\)$.
- **균등(Uniform)**: (정규화 후) 내부 간격이 동일(허용오차 내).
- **타입 예시**: ClampedUniform, ClampedNonUniform, UnClampedUniform, UnClampedNonUniform.

#### 다중도(multiplicity)
- 값 $\(x=U_i\)$ 의 다중도는 **좌우로 동일값을 스캔**하여 개수 집계.  
- 실수 비교는 스케일된 허용오차로:

$$
  \texttt{close}(a,b): |a-b|\le \varepsilon\cdot\max(1,|a|,|b|),\ \varepsilon\approx10^{-12}.
$$

#### `is_clamped`/`is_valid_with_pn`
- `is_valid_with_pn(U,p,n)`: 관계 $\(m=n+p+1\)$ 확인 및 비감소성 체크.
- `is_clamped(U,p)`: 좌/우 끝 다중도 $\(\ge p+1\)$ 여부 반환.  
  (테스트 호출 시 **항상 `m = U.len()-1`**를 내부에서 사용)

### 1.2 스팬 찾기 (Piegl & Tiller, Def. 2.1)
- $\(\mathrm{find\_span}(U,n,p,u)\)$:  
  **최대의** $\(i\) s.t. \(U[i]\le u < U[i+1]\)$; 단, $\(u=U[n+1]\)$ 이면 $\(i=n\)$.
- 이 정의를 따르면 경계 $\(u=1\)$ 에서 올바른 스팬을 얻음.

### 1.3 Basis & 도함수 — Algorithm A2.3 (DersBasisFuns)
- NDU 삼각 테이블로 $\(N_{i-p+r,p}(u)\)$ 구축.
- 1차 이상 미분은 보조 테이블 $\(a\)$ 와 경계

$$
j_1 = \begin{cases}
1, & \text{if } r - k \ge -1 \\
-(r - k), & \text{otherwise}
\end{cases}, \qquad
j_2 = \begin{cases}
k - 1, & \text{if } r - 1 \le p - k \\
p - r, & \text{otherwise}
\end{cases}
$$

  을 사용하여 내부 합을 정확히 집계.  
- **반복 매듭 분모 0 보호**: $\(U_{i+p}-U_i=0\)$ 같은 항은 **0으로 처리**.

#### 불변식(테스트 아이덴티티)
- **Partition of unity:** $\(\sum_j N_{j,p}(u)=1\)$.  
- **도함수 합 0:** $\(\sum_j N'_{j,p}(u)=0\)$ (위 성질을 미분).  
  ⇒ 구현에서 마지막 열 $\(r=p\)$ 갱신 누락/경계 off-by-one 시 이 합이 $\(\pm 1/h\)$ 로 어긋남.

### 2. 구현 체크리스트 / 팁

- **실수 비교**: `close(a,b)`는 절대+상대 혼합(스케일) 오차 사용.  
- **비매끈 함수**: 0을 가로지르면 **자동 분할**.  
- **B-spline**: 반복 매듭에서 **분모 0 가드**. `find_span`의 경계 정의 엄수.  
- **테스트**:  
  - NURBS/B-spline: $\(\sum N=1\), \(\sum N'=0\)$ 전 구간 샘플.
 
### 구현 소스
```rust
// src/knots.rs
#![allow(clippy::needless_range_loop)]

use std::cmp::Ordering;
use crate::geom::utils::math::{close, last_leq_index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnotVectorType {
    Undefined,
    ClampedUniform,
    ClampedNonUniform,
    UnClampedUniform,
    UnClampedNonUniform,
    PiecewiseBezier,
}

#[inline]
fn eps_from_range(range: f64) -> f64 {
    // C# 코드의 용례(1e-12 등)를 참조해, 범위가 0에 가깝더라도 너무 빡세지 않게.
    let base = range.abs().max(1.0);
    1e-12 * base
}

#[inline]
fn are_equal(a: f64, b: f64, range: f64) -> bool {
    (a - b).abs() <= eps_from_range(range)
}

/// U[0]
pub fn left(u: &[f64]) -> f64 {
    u.first().copied().unwrap_or(0.0)
}

/// U[last]
pub fn right(u: &[f64]) -> f64 {
    u.last().copied().unwrap_or(0.0)
}

/// U[last]-U[0]
pub fn length_range(u: &[f64]) -> f64 {
    if u.len() < 2 { 0.0 } else { right(u) - left(u) }
}

pub fn offset(u: &mut [f64], delta: f64) {
    for x in u { *x += delta; }
}

pub fn scale(u: &mut [f64], factor: f64) {
    for x in u { *x *= factor; }
}

/// Normalize to [0,1] by affine transform: shift by -U[0], scale by 1/(U[last]-U[0])
pub fn normalize(u: &mut [f64]) {
    if u.is_empty() { return; }
    let l = left(u);
    if l != 0.0 { offset(u, -l); }
    let r = length_range(u);
    if r != 0.0 { scale(u, 1.0 / r); }
}

/// C#의 IsClamped(out start,out end)와 동일 동작: 판단 + (start/end가 true면) 경계쪽 p개를 정확히 같은 값으로 스냅
/// - p: degree, n: 마지막 유효 span 인덱스 (= n = num_ctrl-1)
pub fn is_clamped_mut(u: &mut [f64], p: usize, n: usize) -> (bool, bool) {
    let m = u.len();
    assert!(m >= p + n + 2, "knot vector size not consistent with p,n");
    let a = u[0];
    let b = u[p + n + 1 - 1]; // C# 코드에서 p+n (0-based) 는 사실상 (p+n+1-1)
    let rng = length_range(u);

    let mut start = are_equal(a, u[p], rng);
    let mut end   = are_equal(u[n], b, rng);

    // C#은 start/end가 true면 경계 쪽 p개의 값들을 a/b로 '정확히' 스냅
    for i in 0..p {
        if start { u[i + 1] = a; }
        if end   { u[n + i] = b; }
    }
    (start, end)
}


#[inline]
fn multiplicity_from_start(u: &[f64], m: usize) -> usize {
    if u.is_empty() { return 0; }
    let m = m.min(u.len() - 1);
    let a = u[0];
    let mut k = 1usize;
    while k <= m && close(u[k], a) { k += 1; }
    k // k개가 같음 (인덱스 0..=k-1), 즉 다중도 = k
}

#[inline]
fn multiplicity_from_end(u: &[f64], m: usize) -> usize {
    if u.is_empty() { return 0; }
    let m = m.min(u.len() - 1);
    let b = u[m];
    let mut k = 1usize;
    // m, m-1, ... 내려가며 같은 값 카운트
    while k <= m && close(u[m - (k - 1)], b) {
        if k == m + 1 { break; }
        k += 1;
    }
    k // 다중도 = k
}


pub fn is_clamped(u: &[f64], p: usize, m: usize) -> bool {
    if u.len() < 2 || p == 0 { return false; }
    let m = m.min(u.len() - 1);

    // (선택) 비감소성 빠른 체크
    for w in u.windows(2) {
        if w[0] > w[1] && !close(w[0], w[1]) { return false; }
    }

    let left_mult  = multiplicity_from_start(u, m);
    let right_mult = multiplicity_from_end(u, m);

    // Open-clamped 조건: 양 끝 다중도 >= p+1
    left_mult >= p + 1 && right_mult >= p + 1
}

/// Span 개수: U[i]>U[i-1]인 지점 개수 (i=p..=n)
pub fn span_count(u: &[f64], p: usize, n: usize) -> usize {
    let mut cnt = 0usize;
    for i in p..=n {
        if u[i] > u[i-1] { cnt += 1; }
    }
    cnt
}

/// 해당 knot 인덱스에서의 중복도(multiplicity)
pub fn multiplicity(u: &[f64], mut i: usize) -> usize {
    if u.is_empty() { return 0; }
    let x = u[i];
    let mut m = 1usize;

    let mut k = i;
    while k > 0 && close(u[k - 1], x) { m += 1; k -= 1; }
    let mut k = i;
    while k + 1 < u.len() && close(u[k + 1], x) { m += 1; k += 1; }

    m
}

/// 유효성(경계 p+1개 동일, 내부 단조비감소, 내부에서 multiplicity<=p)
pub fn is_valid_with_pn(u: &[f64], p: usize, n: usize) -> bool {
    let m = u.len();
    if m < p + n + 2 { return false; }
    let head = p + 1;
    let tail = m - head;
    let a0 = u[0];
    let b0 = u[tail];

    for i in 1..head {
        if u[i] != a0 || u[i + tail] != b0 { return false; }
    }
    if u[head] == a0 || u[tail - 1] == b0 { return false; }
    is_valid_with_p(u, p)
}

/// 내부 단조비감소 + 내부 multiplicity <= p
pub fn is_valid_with_p(u: &[f64], p: usize) -> bool {
    if u.len() >= 2 && p == 0 && u[0] == u[1] { return false; }
    let mut mult = 1usize;
    for i in 0..u.len()-1 {
        if u[i] > u[i+1] { return false; }
        if i > 0 && i < u.len()-2 {
            if u[i] != u[i+1] {
                mult = 1;
            } else {
                mult += 1;
            }
            if mult > p { return false; }
        }
    }
    true
}

/// FindSpan(n,p,u) – Piegl & Tiller 알고리즘
/// - n = #ctrl-1
pub fn find_span(uvec: &[f64], n: usize, p: usize, u: f64) -> usize {
    if u <= uvec[p] { return p; }
    if u >= uvec[n+1] { return n; }
    let mut low = p;
    let mut high = n + 1;
    let mut mid = (low + high)/2;
    while u < uvec[mid] || u >= uvec[mid+1] {
        if u < uvec[mid] { high = mid; } else { low = mid; }
        mid = (low + high)/2;
    }
    mid
}

/// FindSpan + multiplicity
/// k = span index (또는 마지막 knot index), s = multiplicity(같으면 양끝 갯수, 다르면 0)
pub fn find_span_mult(u: &[f64], x: f64, p: usize) -> (usize, usize) {

    let k = last_leq_index(u, x);
    let s = multiplicity(u, k);
    // 끝점 보정: 클램프 벡터라면 s >= p+1 보장
    let s = if (k == 0 || k == u.len()-1) && s < p + 1 { p + 1 } else { s };
    (k, s)
}

/// FindSpanMult with snapping to close knot (min_knot_dist)
pub fn find_span_mult_with_snap(uvec: &[f64], u: &mut f64, p: usize, min_knot_dist: f64) -> (usize, usize) {
    for &kv in uvec {
        if (kv - *u).abs() < min_knot_dist {
            *u = kv;
            break;
        }
    }
    find_span_mult(uvec, *u, p)
}

/// 첫 번째 "매우 가까운" 노트 구간을 찾아 index(낮은 multiplicity의 쪽)와 그 multiplicity를 반환
pub fn first_similar_knot_index(u: &[f64], p: usize, min_dist: f64) -> Option<(usize, usize)> {
    for i in p..(u.len() - p - 1) {
        if u[i] != u[i+1] && (u[i+1] - u[i]).abs() < min_dist {
            let (k1,s1) = find_span_mult(u, u[i], p);
            let (k2,s2) = find_span_mult(u, u[i+1], p);
            return if s1 < s2 { Some((k1, s1)) } else { Some((k2, s2)) };
        }
    }
    None
}

/// 허용 knot 거리 추정값
pub fn min_acceptable_knot_distance(u: &[f64], p: usize) -> f64 {
    let mut distinct = 0usize;
    for i in p..(u.len()-p-1) {
        if u[i+1] != u[i] { distinct += 1; }
    }
    if distinct == 0 { return 1e-7; }
    1e-7 * length_range(u) / (distinct as f64)
}

/// U <- offset - reverse(U)
pub fn reverse_with_offset(u: &mut [f64], offset: f64) {
    let old = u.to_vec();
    let mut j = old.len();
    for i in 0..u.len() {
        j -= 1;
        u[i] = offset - old[j];
    }
}
pub fn reverse(u: &mut [f64]) {
    let off = 0.0;
    reverse_with_offset(u, off);
}

/// KnotVectorType 판정
pub fn detect_knot_type(u: &[f64], p: usize) -> KnotVectorType {
    if u.len() < 2 { return KnotVectorType::Undefined; }
    let m = u.len() - 1;

    let left_m  = multiplicity(u, 0);
    let right_m = multiplicity(u, m);
    let clamped = left_m >= p + 1 && right_m >= p + 1;

    // 내부 노드 추출
    let il = left_m.min(m);
    let ir = m.saturating_sub(right_m);
    let interior = if il <= ir { &u[il..=ir] } else { &[][..] };

    // 정규화 간격 균등성 검사
    let a = u[0];
    let b = u[m];
    let len = (b - a).abs().max(1.0);
    let mut diffs = Vec::new();
    for w in interior.windows(2) {
        diffs.push((w[1] - w[0]) / len);
    }
    let uniform = if diffs.len() <= 1 {
        true
    } else {
        let d0 = diffs[0];
        diffs.iter().all(|&d| (d - d0).abs() <= 1e-10_f64.max(1e-8 * d0.abs()))
    };

    match (clamped, uniform) {
        (true,  true)  => KnotVectorType::ClampedUniform,
        (true,  false) => KnotVectorType::ClampedNonUniform,
        (false, true)  => KnotVectorType::UnClampedUniform,
        (false, false) => KnotVectorType::UnClampedNonUniform,
    }
}

/// 중앙값 근처에서 쪼개기: (남겨야 하는 삽입 개수, splitPt, midVal)
/// C# Split(U, m, k, out splitPt, out midVal)
pub fn split_mid(u: &[f64], m: usize, k: usize) -> (i32, usize, f64) {
    // m: p + n (C#에서 사용하던 형태), k: degree
    // length = m + k + 1
    let len = m + k + 1;
    assert_eq!(len, u.len(), "m+k+1 must equal len(U)");

    let index1 = len;
    let mut index2 = index1 / 2;
    let mut mid_val = u[index2];
    let mut index3 = index2 + 1;
    let mut num1 = 1;

    while index3 < index1 && u[index3] == mid_val {
        index3 += 1;
        num1 += 1;
    }
    let mut index4 = index2 - 1;
    while index4 > 0 && u[index4] == mid_val {
        index4 -= 1;
        index2 -= 1;
        num1 += 1;
    }
    if index4 <= 0 {
        mid_val = (u[0] + u[index1 - 1]) * 0.5;
        index2 = index1 / 2;
        while u[index2 + 1] < mid_val {
            index2 += 1;
        }
        num1 = 0;
    }
    let num2 = k as i32 - num1 as i32;
    let split_pt = if num2 < k as i32 { index2 - 1 } else { index2 };
    (num2, split_pt, mid_val)
}

/// Split with target u value around an index (C# 오버로드)
pub fn split_at(
    u: &[f64],
    mut param: f64,
    mut middex: usize,
    m: usize,
    k: usize
) -> (i32, usize) {
    let len = m + k + 1;
    assert_eq!(len, u.len());
    let a = u[middex];

    if !are_equal(a, param, length_range(u)) {
        while u[middex + 1] < a { middex += 1; }
        return (k as i32, middex);
    }
    // 동일 knot 값에서 multiplicity 세기
    let mut index2 = middex + 1;
    let mut num1 = 1i32;
    while index2 < len && u[index2] == a {
        index2 += 1;
        num1 += 1;
    }
    let mut index3 = middex - 1;
    while index3 > 0 && u[index3] == a {
        index3 -= 1;
        middex -= 1;
        num1 += 1;
    }
    let num2 = k as i32 - num1;
    let split_pt = if num2 < k as i32 { middex - 1 } else { middex };
    (num2, split_pt)
}

/// BasisFuns(i, u, p) → 길이 p+1
pub fn basis_funs(uvec: &[f64], i: usize, u: f64, p: usize) -> Vec<f64> {
    let mut n = vec![0.0; p+1];
    let mut left = vec![0.0; p+1];
    let mut right = vec![0.0; p+1];
    n[0] = 1.0;
    for j in 1..=p {
        left[j]  = u - uvec[i + 1 - j];
        right[j] = uvec[i + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let temp = n[r] / (right[r+1] + left[j - r]);
            n[r] = saved + right[r+1] * temp;
            saved = left[j - r] * temp;
        }
        n[j] = saved;
    }
    n
}

/// DersBasisFuns(i, u, p, n_der) → (n_der+1) x (p+1) (The NURBS Book Algorithm A2.3)
pub fn ders_basis_funs(u: &[f64], i: usize, x: f64, p: usize, nder: usize) -> Vec<Vec<f64>> {
    let nder = nder.min(p);
    let mut ders = vec![vec![0.0; p + 1]; nder + 1];

    // NDU 삼각 테이블
    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let mut left  = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];

    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j]  = x - u[i + 1 - j];
        right[j] = u[i + j] - x;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            // 분모 0 보호 (반복 매듭일 때 해당 항은 0)
            let temp = if close(denom, 0.0) { 0.0 } else { ndu[r][j - 1] / denom };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    // 0차: N_i-p..i
    for j in 0..=p {
        ders[0][j] = ndu[j][p];
    }

    // 미분 준비: lower triangle
    let mut a = vec![vec![0.0; p + 1]; 2];
    for r in 0..=p {
        // N_{i-p+r,p}' 계산
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        a[0][0] = 1.0;

        // 상수 계수: p!/(p-k)!
        for k in 1..=nder {
            let mut d = 0.0;
            let rk = r as isize - k as isize;
            let pk = p as isize - k as isize;

            // 첫 항
            if r as isize >= k as isize {
                let denom = ndu[pk as usize + 1][rk as usize];
                let val = if close(denom, 0.0) { 0.0 } else { a[s1][0] / denom };
                a[s2][0] = val;
                d += val * ndu[rk as usize][pk as usize];
            }

            // 내부 항들: j = j1..j2   (★ 여기 경계가 가장 잘 틀림)
            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if (r as isize - 1) <= pk { (k - 1) } else { (p - r) };
            for j in j1..=j2 {
                let denom = ndu[pk as usize + 1][(rk + j as isize) as usize];
                let val = if close(denom, 0.0) { 0.0 } else { (a[s1][j] - a[s1][j - 1]) / denom };
                a[s2][j] = val;
                d += val * ndu[(rk + j as isize) as usize][pk as usize];
            }

            // 마지막 항
            if (r as isize) <= pk {
                let denom = ndu[pk as usize + 1][r];
                let val = if close(denom, 0.0) { 0.0 } else { -a[s1][k - 1] / denom };
                a[s2][k] = val;
                d += val * ndu[r][pk as usize];
            }

            ders[k][r] = d;
            // swap rows
            s1 ^= 1;
            s2 ^= 1;
        }
    }

    // k차 미분 스케일링: p!/(p-k)!
    let mut rfact = p as f64;
    for k in 1..=nder {
        for j in 0..=p {
            ders[k][j] *= rfact;
        }
        rfact *= (p - k) as f64;
    }

    ders
}

/// Open-uniform knot 생성 보조(테스트용)
pub fn open_uniform_knots(n_ctrl: usize, degree: usize) -> Vec<f64> {
    let m = n_ctrl + degree + 1;
    let mut k = vec![0.0; m];
    for i in 0..m {
        if i <= degree { k[i] = 0.0; }
        else if i >= n_ctrl { k[i] = 1.0; }
        else { k[i] = (i - degree) as f64 / (n_ctrl - degree) as f64; }
    }
    k
}
```

### 테스트 코드
```rust
use geometry::geom::utils::knots::{basis_funs,
    ders_basis_funs,
    find_span,
    find_span_mult, is_clamped,
    is_valid_with_pn,
    left,
    multiplicity,
    normalize,
    open_uniform_knots,
    reverse_with_offset,
    right,
    KnotVectorType};

#[cfg(test)]
mod tests {
    use geometry::geom::utils::knots::detect_knot_type;
    use super::*;

    fn approx(a: f64, b: f64) -> bool { (a-b).abs() < 1e-9 }

    #[test]
    fn normalize_and_reverse() {
        let mut u = vec![2.0, 2.0, 2.5, 3.5, 4.0, 4.0];
        normalize(&mut u);
        assert!(approx(left(&u), 0.0));
        assert!(approx(right(&u), 1.0));

        let mut r = u.clone();
        reverse_with_offset(&mut r, 1.0);
        // reverse about 1.0 should mirror
        for i in 0..u.len() {
            assert!(approx(r[i] + u[u.len()-1-i], 1.0));
        }
    }

    #[test]
    fn style_detection_open_uniform() {
        let u = open_uniform_knots(6, 2); // n_ctrl=6, degree=2
        // [0,0,0, 1/4, 2/4, 3/4, 1,1,1]
        assert_eq!(detect_knot_type(&u, 2), KnotVectorType::ClampedUniform);
        assert!(is_clamped(&u, 2, u.len() - 1));
        assert!(is_valid_with_pn(&u, 2, 5));
    }

    #[test]
    fn basis_sum_to_one() {
        let u = open_uniform_knots(5, 3); // p=3, n=4
        let n = 4usize;
        let p = 3usize;
        let u_mid = 0.5;
        let i = find_span(&u, n, p, u_mid);
        let nvals = basis_funs(&u, i, u_mid, p);
        let s: f64 = nvals.iter().sum();
        assert!(approx(s, 1.0));
    }

    #[test]
    fn ders_first_derivative_sum_zero() {
        let u = open_uniform_knots(7, 2); // n=6, p=2
        let n = 6usize;
        let p = 2usize;
        let u_mid = 0.37;
        let i = find_span(&u, n, p, u_mid);
        let ders = ders_basis_funs(&u, i, u_mid, p, 1);
        // d/du sum N_i^p(u) = 0
        let s: f64 = ders[1].iter().sum();
        assert!(approx(s, 0.0));
    }

    #[test]
    fn span_mult_and_multiplicity() {
        let u = vec![0.0,0.0,0.0, 0.4,0.6, 1.0,1.0,1.0];
        let (k,s) = find_span_mult(&u, 0.0, 2);
        assert!(k>0 && s>=1);
        assert_eq!(multiplicity(&u, 0), 3);

        let (k2,s2) = find_span_mult(&u, 1.0, 2);
        assert_eq!(s2, 3);
        assert_eq!(k2, u.len()-1);
    }
}
```

---

### 참고(교과서)
- Piegl & Tiller, *The NURBS Book*, 2e — Alg. A2.3 (DersBasisFuns), span 정의.  
