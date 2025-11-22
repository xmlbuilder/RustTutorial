# Knot
## 1. Knot Vector 이론 및 수식 정리
### 기본 정의
- B-spline Knot Vector:

$$
U=\{ u_0,u_1,\dots ,u_m\} ,\quad u_i\leq u_{i+1}
$$


- $p$: 차수 (degree)
- $n+1$: 제어점 개수
- $m=n+p+1$: 마지막 knot 인덱스
### Clamped Knot Vector
- Clamped Uniform:

$$
u_0=\dots =u_p=0,\quad u_{n+1}=\dots =u_{n+p+1}=1
$$

- 내부 knot는 균일 간격:

$$
u_{p+j}=\frac{j}{n-p+1},\quad j=1,\dots ,n-p
$$

- Clamped Non-Uniform (Averaging Formula): 내부 knot:

$$
u_{p+j}=\frac{1}{p}\sum _{i=j}^{j+p-1}\, \mathrm{params}[i],\quad j=1,\dots ,n-p
$$

### Chord-length 파라미터
- 점 집합 \{ Q_i\} 에 대해:

$$
d_i=\| Q_i-Q_{i-1}\| ,\quad u_i=\frac{\sum _{k=1}^id_k}{\sum _{k=1}^{m-1}d_k}
$$

### Basis Function (Cox–de Boor)
- 재귀 정의:

$$
N_{i,0}(u)=
\begin{cases}
1 & \text{if } u_i \le u < u_{i+1}, \\
0 & \text{otherwise}
\end{cases}
$$


$$
N_{i,p}(u)= \frac{u-u_i}{u_{i+p}-u_i}\,N_{i,p-1}(u) + \frac{u_{i+p+1}-u}{u_{i+p+1}-u_{i+1}}\,N_{i+1,p-1}(u)
$$

### 도함수 (Piegl & Tiller A2.3)
- k차 도함수:
Piegl & Tiller의 Algorithm A2.3은 먼저 **정규화되지 않은** k차 도함수 값을 계산하고,  
마지막에 차수 p에 대한 하강 곱(falling factorial)으로 스케일링합니다.

- 즉, 오른쪽의 스케일링 인자만 남기면

$$
p(p-1)\cdots (p-k+1) = \frac{p!}{(p-k)!} = (p)_k
$$

이며, $k>p$ 이면 $\frac{d^k}{du^k}N_{i,p}(u)=0$ 입니다.

- 간단 표기:

$$
\frac{d^k}{du^k}N_{i,p}(u) = D_{i,p}^{(k)}(u) \cdot  \frac{p!}{(p-k)!},
$$

여기서 $D_{i,p}^{(k)}(u)$ 는 Algorithm A2.3으로 얻은 비스케일 값입니다.

- 낙하곱(falling factorial) 표기:

$$
\frac{d^k}{du^k}N_{i,p}(u) = D_{i,p}^{(k)}(u) \cdot  (p)_k,\quad (p)_k=p(p-1)\cdots (p-k+1).
$$

- 자주 헷갈리는 점
    - 끝점 $u=U[n+1]$ 등 경계 특수값에서는 기저함수와 도함수가 특수 처리를 필요로 합니다 (예: 마지막 기저만 1).
    - k=0이면 도함수는 원래 기저함수이며, 스케일링 인자는 1이 됩니다.
    - k>p인 도함수는 0입니다.


## 2. 코드 점검 (문제 가능성)

- is_clamped_full:  
    self.last().unwrap() 호출 → Vec<f64> slice에는 last()가 없음. self[self.len()-1]로 수정 필요.
- split_mid / split_at_value:  
    index 처리 주의 필요. 특히 multiplicity 계산 시 underflow 위험.
- basis_funs:  
    denom = right[r+1] + left[j-r] → Piegl & Tiller 공식과 동일. 단, EPSILON 처리 적절.
- ders_basis_funs:  
    Piegl & Tiller A2.3 구현. scaling factor 적용도 올바름. 다만 인덱스 변환 부분에서 rk+j 음수 가능성 주의.
- on_averaging_internal_surface_knots:  
    averaging formula 구현은 맞음. 단, params가 단조 증가하지 않으면 잘못된 knot 생성 가능.


## 3. 함수 문서화 표

| 함수명                          | 설명                          | 주요 수식/로직                                | 주의점                          |
|---------------------------------|-------------------------------|-----------------------------------------------|---------------------------------|
| on_chord_length_params          | 점 집합 chord-length 파라미터 | u_i = (Σ d_k) / (Σ d)                         | 최소 2점 필요                   |
| on_uniform_length_params        | 균일 파라미터 (0..1)          | u_i = i / (n-1)                               | n=1 처리                        |
| on_averaging_internal_curve_knots | 곡선용 averaging knot        | u_{p+j} = (1/p) Σ params[j..j+p-1]            | params 단조 필요                |
| on_averaging_internal_surface_knots | 표면용 averaging knot       | 동일                                          | 동일                            |
| on_clamped_uniform_knot_vector  | clamped uniform knot 생성     | 앞뒤 p+1 중복, 내부 균일                      | n > p 조건                      |
| basis_funs                      | B-spline 기저 함수            | Cox–de Boor 재귀 공식                         | u=U[n+1] 특수 처리              |
| ders_basis_funs                 | 기저 함수 도함수              | Piegl & Tiller A2.3, scaling factor 적용       | 인덱스 음수 주의                |
| find_span                       | span 찾기 (이진 탐색)         | 표준 알고리즘                                 | u=U[p], U[n+1] 경계 처리        |
| find_span_multi                 | span + multiplicity           | Knot 중복 처리                                | tolerance 사용                   |
| is_clamped_with_ends            | knot vector clamped 여부      | 앞뒤 p+1 동일 여부                            | scale tolerance                 |
| style                           | KnotVector 유형 판별          | Clamped/Unclamped, Uniform 여부                | tolerance 기준                   |
| split_mid                       | Knot 분할 위치 찾기           | multiplicity 기반                             | index underflow 주의             |
| split_at_value                  | 특정 값에서 분할              | multiplicity 기반                             | mid_idx 갱신 주의                |

### 결론
- 수식적 문제 없음: chord-length, averaging, Cox–de Boor, Piegl & Tiller 모두 올바르게 구현됨.
- 코드상 주의점: 인덱스 underflow 가능성.
- 문서화: 위 표를 기준으로 각 함수의 역할과 수식을 정리하면 라이브러리 문서로 활용 가능.


```rust
use crate::core::basis::{on_basis_func_ret_vec, on_find_span_usize};
use crate::core::maths::{on_are_equal, on_are_equal_rel_tol, on_solve_linear_system_vec};
use crate::core::matrix::Matrix;
use crate::core::prelude::{Param, Point3D, Point4D};
use crate::core::types::{
    Degree, Index, NurbsError, ON_TOL9, ON_TOL12, Real, Result, on_are_equal_scaled,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnotVectorType {
    Undefined,
    ClampedUniform,
    ClampedNonUniform,
    UnClampedUniform,
    UnClampedNonUniform,
    PiecewiseBezier,
}
```
```rust
#[derive(Debug, Clone)]
pub struct KnotVector {
    pub knots: Vec<Real>, // non-decreasing
}
```
```rust
impl KnotVector {
    pub fn new(knots: Vec<Real>) -> Result<Self> {
        if knots.is_empty() {
            return Err(NurbsError::EmptyKnots);
        }
        if !knots.windows(2).all(|w| w[0] <= w[1]) {
            return Err(NurbsError::KnotNotNonDecreasing);
        }
        Ok(Self { knots })
    }
```
```rust    
    #[inline]
    pub fn len(&self) -> Index {
        self.knots.len()
    }
```
```rust
    #[inline]
    pub fn resize_len(&mut self, new_len: usize, value: Real) {
        self.knots.resize(new_len, value);
    }
```
```rust
    #[inline]
    pub fn resize_like(&mut self, other: &Self, value: Real) {
        self.knots.resize(other.knots.len(), value);
    }
```
```rust
    #[inline]
    pub fn copy_from(&mut self, other: &Self) {
        self.knots.clone_from(&other.knots);
    }
```
```rust
    #[inline]
    pub fn copy_from_slice(&mut self, slice: &[Real]) {
        self.knots.clear();
        self.knots.extend_from_slice(slice);
    }
```
```rust
    #[inline]
    pub fn copy_prefix_from(&mut self, other: &Self, count: usize) {
        let n = count.min(other.knots.len());
        self.knots.resize(n, 0.0);
        self.knots[..n].copy_from_slice(&other.knots[..n]);
    }
```
```rust
    #[inline]
    pub fn copy_range_from(&mut self, other: &Self, range: std::ops::Range<usize>) {
        let r = range.start.min(other.knots.len())..range.end.min(other.knots.len());
        let n = r.end.saturating_sub(r.start);
        self.knots.resize(n, 0.0);
        if n > 0 {
            self.knots[..n].copy_from_slice(&other.knots[r]);
        }
    }
```
```rust
    #[inline]
    pub fn is_non_decreasing(&self) -> bool {
        self.knots.windows(2).all(|w| w[0] <= w[1])
    }
```
```rust
    #[inline]
    pub fn as_slice(&self) -> &[Real] {
        &self.knots
    }
```
```rust
    /// m = n + p + 1, where m = knots.len() - 1 and n+1 = cp_count
    pub fn check_degree_vs_cp(&self, p: Degree, cp_count: Index) -> Result<()> {
        let m = self.len().saturating_sub(1) as isize;
        let n = (cp_count as isize) - 1;
        let expected = n + (p as isize) + 1;
        if m != expected {
            return Err(NurbsError::DimensionMismatch("knot/degree/cp_count"));
        }
        Ok(())
    }
```
```rust
    pub fn first(&self) -> Real {
        self.knots.first().copied().unwrap_or(0.0)
    }
    pub fn last(&self) -> Real {
        self.knots.last().copied().unwrap_or(0.0)
    }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}
```
```rust
pub trait KnotVectorExt {
    // reference
    fn left(&self) -> f64;
    fn right(&self) -> f64;
    fn span_length(&self) -> f64;

    // adjust
    fn offset(&mut self, delta: f64);
    fn scale(&mut self, factor: f64);
    fn normalize(&mut self);

    // Clamp check/correction
    fn is_clamped_with_ends(&mut self, p: usize, n: usize) -> (bool, bool);
    fn is_clamped(&mut self, p: usize, n: usize) -> bool;

    // Validation / Statistics
    fn span_count(&self, p: usize, n: usize) -> usize;
    fn multiplicity(&self, knot_index: isize) -> usize;
    fn is_valid_full(&self, p: usize, n: usize) -> bool;
    fn is_valid(&self, p: usize) -> bool;

    // Search
    fn find_span(&self, n: usize, p: usize, u: f64) -> usize;
    fn find_span_multi(
        &self,
        u: f64,
        p: usize,
    ) -> std::result::Result<(usize, usize), &'static str>;
    fn find_span_multi_snap(
        &self,
        u: &mut f64,
        p: usize,
        min_knot_dist: f64,
    ) -> std::result::Result<(usize, usize), &'static str>;

    fn first_similar_knot_index(&self, p: usize, min_dist: f64) -> Option<(usize, usize)>;

    fn min_acceptable_knot_distance(&self, p: usize) -> f64;

    // reverse
    fn reverse_in_place(&mut self);
    fn reverse_with_offset(&mut self, offset: f64);

    // style
    fn style(&mut self, p: usize, n: usize) -> KnotVectorType;

    fn is_clamped_full(&self, p: usize, n: usize) -> (bool, bool);
    // split
    fn split_mid(&self, m: usize, k: usize) -> (isize, usize, f64);
    fn split_at(&self, u: f64, mid_idx: usize, m: usize, k: usize) -> (usize, usize);

    fn split_at_value(&self, target: f64, mid_idx: &mut usize, m: usize, k: usize) -> (i32, usize);

    // basis
    fn basis_funs(&self, span: usize, u: f64, p: usize) -> Vec<f64>;
    fn ders_basis_funs(&self, span: usize, u: f64, p: usize, n: usize) -> Vec<Vec<f64>>;
}
```
```rust
impl KnotVectorExt for [f64] {
    fn left(&self) -> f64 {
        self[0]
    }
```
```rust    
    fn right(&self) -> f64 {
        self[self.len() - 1]
    }
```
```rust    
    fn span_length(&self) -> f64 {
        on_length_range(self)
    }
```
```rust
    fn offset(&mut self, delta: f64) {
        for v in self.iter_mut() {
            *v += delta;
        }
    }
```
```rust    
    fn scale(&mut self, factor: f64) {
        for v in self.iter_mut() {
            *v *= factor;
        }
    }
```
```rust    
    fn normalize(&mut self) {
        if self.left() != 0.0 {
            let a = self.left();
            self.offset(-a);
        }
        let len = self.span_length();
        if len != 0.0 {
            self.scale(1.0 / len);
        }
    }
```
```rust
    fn is_clamped_with_ends(&mut self, p: usize, n: usize) -> (bool, bool) {
        let a = self[0];
        let b = self[p + n];
        let scale = self.span_length();

        let mut start = true;
        for i in 1..=p {
            start &= on_are_equal_scaled(a, self[i], scale);
        }
        let mut end = true;
        for i in n + 1..=p + n {
            end &= on_are_equal_scaled(self[i], b, scale);
        }
        (start, end)
    }
```
```rust
    fn is_clamped(&mut self, p: usize, n: usize) -> bool {
        let (s, e) = self.is_clamped_with_ends(p, n);
        s & e
    }
```
```rust
    fn span_count(&self, p: usize, n: usize) -> usize {
        let mut cnt = 0;
        for i in p..=n {
            if self[i] > self[i - 1] {
                cnt += 1;
            }
        }
        cnt
    }
```
```rust
    fn multiplicity(&self, mut knot_index: isize) -> usize {
        let len = self.len() as isize;
        if knot_index < 0 || knot_index >= len {
            return 0;
        }
        let mut count;

        while knot_index > 0 && self[knot_index as usize] == self[(knot_index - 1) as usize] {
            knot_index -= 1;
        }
        let start = knot_index as usize;
        let max = self.len() - start;
        count = 1;
        while count < max && self[start] == self[start + count] {
            count += 1;
        }
        count
    }
```
```rust
    fn is_valid_full(&self, p: usize, _n: usize) -> bool {
        // Check if the first and last p+1 values are identical
        let len = self.len();
        if len < 2 {
            return false;
        }

        let idx1 = p + 1;
        let idx2 = len - idx1;
        let a0 = self[0];
        let b0 = self[idx2];

        for i in 1..=p {
            if self[i] != a0 || self[i + idx2] != b0 {
                return false;
            }
        }
        if self[idx1] == a0 || self[idx2 - 1] == b0 {
            return false;
        }
        self.is_valid(p)
    }
```
```rust
    fn is_valid(&self, p: usize) -> bool {
        let m = self.len();
        if m < 2 {
            return false;
        }
        if p == 0 && self[0] == self[1] {
            return false;
        }

        let mut mult = 1usize;
        for i in 0..(m - 1) {
            if self[i] > self[i + 1] {
                return false;
            }
            if i > 0 && i < m - 2 {
                if self[i] != self[i + 1] {
                    mult = 1;
                } else {
                    mult += 1;
                }
                if mult > p {
                    return false;
                }
            }
        }
        true
    }
```
```rust
    fn find_span(&self, n: usize, p: usize, u: f64) -> usize {
        if u <= self[p] {
            return p;
        }
        if u >= self[n + 1] {
            return n;
        }
        let (mut low, mut high) = (p, n + 1);
        let mut mid = (low + high) / 2;
        while u < self[mid] || u >= self[mid + 1] {
            if u < self[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        mid
    }
```
```rust
    fn find_span_multi(
        &self,
        u: f64,
        p: usize,
    ) -> std::result::Result<(usize, usize), &'static str> {
        if u < self.left() || u > self.right() {
            return Err("u out of knot range");
        }
        let len = self.len();
        let k;
        let mut s;

        for i in 1..len {
            if self[i] > u {
                k = i - 1;
                if on_are_equal(u, self[k], self.span_length()) {
                    s = 1;
                    let mut j = k;
                    while j > 0 && j > k.saturating_sub(p) {
                        if self[k] == self[j - 1] {
                            s += 1;
                        }
                        j -= 1;
                    }
                } else {
                    s = 0;
                }
                return Ok((k, s));
            }
        }
        // u == Right()
        s = 1;
        k = len - 1;
        let mut j = k;
        while j > 0 && j > k.saturating_sub(p) {
            if self.right() == self[j - 1] {
                s += 1;
            }
            j -= 1;
        }
        Ok((k, s))
    }
```
```rust
    fn find_span_multi_snap(
        &self,
        u: &mut f64,
        p: usize,
        min_knot_dist: f64,
    ) -> std::result::Result<(usize, usize), &'static str> {
        for &kv in self.iter() {
            if (kv - *u).abs() < min_knot_dist {
                *u = kv;
                break;
            }
        }
        self.find_span_multi(*u, p)
    }
```
```rust
    fn first_similar_knot_index(&self, p: usize, min_dist: f64) -> Option<(usize, usize)> {
        if self.len() < 2 {
            return None;
        }
        for i in p..(self.len() - p - 1) {
            if self[i] != self[i + 1] && (self[i + 1] - self[i]) < min_dist {
                let (k1, s1) = self.find_span_multi(self[i], p).ok()?;
                let (k2, s2) = self.find_span_multi(self[i + 1], p).ok()?;
                if s1 < s2 {
                    return Some((k1, s1));
                } else {
                    return Some((k2, s2));
                }
            }
        }
        None
    }
```
```rust
    fn min_acceptable_knot_distance(&self, p: usize) -> f64 {
        let mut cnt = 0usize;
        for i in p..(self.len() - p - 1) {
            if self[i + 1] != self[i] {
                cnt += 1;
            }
        }
        if cnt == 0 {
            return 0.0;
        }
        1e-7 * self.span_length() / (cnt as f64)
    }
```
```rust
    fn reverse_in_place(&mut self) {
        self.reverse_with_offset(0.0);
    }
```
```rust
    fn reverse_with_offset(&mut self, offset: f64) {
        let copy = self.to_vec();
        let mut j = copy.len();
        for i in 0..self.len() {
            j -= 1;
            self[i] = offset - copy[j];
        }
    }
```
```rust
    fn style(&mut self, p: usize, n: usize) -> KnotVectorType {
        if self.is_clamped(p, n) {
            // Piecewise Bézier check (internal knot intervals repeat in multiples of p)
            let mut i = p + 1;
            while i < n - 1 && on_are_equal(self[i], self[i + p - 1], self.span_length()) {
                i += p;
            }
            if i >= n {
                return KnotVectorType::PiecewiseBezier;
            }
            // Uniform clamping check
            let step = self[p + 1] - self[p];
            for j in (p + 1)..(self.len() - 1 - p) {
                if ((self[j + 1] - self[j]) - step).abs() > 1e-12 {
                    return KnotVectorType::ClampedNonUniform;
                }
            }
            KnotVectorType::ClampedUniform
        } else {
            // Un clamped: uniform / non-uniform
            let step = self[1] - self[0];
            println!("{:?}", self);
            for i in 0..(self.len() - 1) {
                let dist = ((self[i + 1] - self[i]) - step).abs();
                if dist > ON_TOL12 {
                    return KnotVectorType::UnClampedNonUniform;
                }
            }
            KnotVectorType::UnClampedUniform
        }
    }
```
```rust
    fn is_clamped_full(&self, p: usize, n: usize) -> (bool, bool) {
        assert!(self.len() >= p + n + 1, "knot length must be ≥ p+n+1");
        let a = self[0];
        let b = self[p + n];
        let scale = self.last().unwrap() - self[0];

        let start = on_are_equal(a, self[p], scale);
        let end = on_are_equal(self[n], b, scale);

        // In the C# implementation, if start/end are true, the first and last p elements are overwritten with the same value.
        // But here, this is just a predicate function — it doesn't modify values, only returns the status.
        (start, end)
    }
```
```rust    
    fn split_mid(&self, p: usize, n: usize) -> (isize, usize, f64) {
        // C# code: index1 = m + k + 1 (last index), mid = U[index1 / 2]
        // Here, m ≡ n
        assert!(self.len() >= n + p + 1, "knot length < n+p+2");
        let last = n + p; // Last index
        assert!(last < self.len(), "last index out of range");

        let mut mid_idx = last / 2;
        let mut mid_val = self[mid_idx];

        // Count repeated values (multiplicity) around mid
        let mut multi = 1usize;
        // Check for repeated values to the right
        let mut j = mid_idx + 1;
        while j <= last && self[j] == mid_val {
            multi += 1;
            j += 1;
        }
        // Check for repeated values to the left
        let mut i = mid_idx;
        while i > 0 && self[i - 1] == mid_val {
            multi += 1;
            i -= 1;
            mid_idx -= 1; // 좌측으로 한 칸씩 밀림(C#의 index2 감소와 동일)
        }

        // Adjust when identical values are stuck to both ends
        if mid_idx == 0 {
            // C#: midVal = (U[0]+U[index1])/2; index2 = index1/2; while U[index2+1] < midVal) ++index2;
            mid_val = 0.5 * (self[0] + self[last]);
            mid_idx = last / 2;
            while mid_idx + 1 <= last && self[mid_idx + 1] < mid_val {
                mid_idx += 1;
            }
            multi = 0;
        }

        // deficit = k - multi
        let deficit = p as isize - multi as isize;

        // splitPt = (deficit < k) ? index2-1 : index2   (C# 동일)
        let split_pt = if (deficit as isize) < p as isize {
            mid_idx.saturating_sub(1)
        } else {
            mid_idx
        };

        (deficit, split_pt, mid_val)
    }
```
```rust
    fn split_at(&self, u: f64, mut middex: usize, m: usize, k: usize) -> (usize, usize) {
        let idx1 = m + k + 1;
        let a = self[middex];

        if !on_are_equal(a, u, self[idx1] - self[0]) {
            while self[middex + 1] < a {
                middex += 1;
            }
            return (k, middex);
        }

        let mut i_right = middex + 1;
        let mut mult = 1usize;
        while i_right < idx1 && self[i_right] == a {
            i_right += 1;
            mult += 1;
        }
        while middex > 0 && self[middex - 1] == a {
            middex -= 1;
            mult += 1;
        }
        let num2 = k.saturating_sub(mult);
        let split_pt = if num2 < k { middex - 1 } else { middex };
        (num2, split_pt)
    }
```
```rust
    fn split_at_value(
        &self,
        target: f64,
        mid_idx: &mut usize,
        p: usize,
        n_cp: usize,
    ) -> (i32, usize) {
        // last knot index = n_cp + p
        let last = p + n_cp;
        assert!(last < self.len(), "knot length must be n_cp + p + 1");

        // C#: int index1 = m + k + 1; double a = U[*mid_idx];
        // Original logic: AreEqual(a, u, U[index1] - U[0]) → a = U[*mid_idx], u = target
        let a = self[*mid_idx];
        let scale = self[last] - self[0];

        if !on_are_equal(a, target, scale) {
            // Original: while (U[*mid_idx + 1] < u) ++*mid_idx;
            // (The comparison target must be 'u'!)
            while *mid_idx + 1 < last && self[*mid_idx + 1] < target {
                *mid_idx += 1;
            }
            // splitPt = *mid_idx; return k;
            return (p as i32, *mid_idx);
        }

        // a == target (within tolerance) → compute multiplicity of target
        // Count how many identical values appear to the right
        let mut mult: i32 = 1;
        let mut r = *mid_idx + 1;
        while r < last && self[r] == a {
            mult += 1;
            r += 1;
        }
        // Extend to the left
        while *mid_idx > 0 && self[*mid_idx - 1] == a {
            *mid_idx -= 1;
            mult += 1;
        }

        // deficit = p - multiplicity
        let deficit = p as i32 - mult;

        let split_pt = if deficit < p as i32 {
            mid_idx.saturating_sub(1)
        } else {
            *mid_idx
        };

        (deficit, split_pt)
    }
```
```rust
    fn basis_funs(&self, span: usize, t: f64, p: usize) -> Vec<f64> {
        let mut n_vec = vec![0.0; p + 1];
        let mut left = vec![0.0f64; p + 1];
        let mut right = vec![0.0f64; p + 1];

        // ---- Special case: right endpoint (u == U[span+1]) ----
        // For clamped curves, if u == U[n+1] and span == n, then N[p] = 1 and all others are 0
        // (A small tolerance 'tol' is used to account for numerical error)
        let tol = 1e-14 * (self[self.len() - 1] - self[0]).abs().max(1.0);
        if (t - self[span + 1]).abs() <= tol {
            n_vec[p] = 1.0;
            return n_vec;
        }

        n_vec[0] = 1.0;
        for j in 1..=p {
            left[j] = t - self[span + 1 - j];
            right[j] = self[span + j] - t;

            let mut saved = 0.0;
            for r in 0..j {
                let denom = right[r + 1] + left[j - r];
                let temp = if denom.abs() > f64::EPSILON {
                    n_vec[r] / denom
                } else {
                    0.0
                };
                n_vec[r] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            n_vec[j] = saved;
        }
        n_vec
    }
```
```rust
    fn ders_basis_funs(&self, span: usize, u: f64, p: usize, n: usize) -> Vec<Vec<f64>> {
        // Piegl & Tiller Algorithm A2.3
        let n = n.min(p);
        let mut ndu = vec![vec![0.0; p + 1]; p + 1];
        let mut left = vec![0.0; p + 1];
        let mut right = vec![0.0; p + 1];

        ndu[0][0] = 1.0;
        for j in 1..=p {
            left[j] = u - self[span + 1 - j];
            right[j] = self[span + j] - u;
            let mut saved = 0.0;
            for r in 0..j {
                let denom = right[r + 1] + left[j - r];
                let temp = if denom.abs() > f64::EPSILON {
                    ndu[r][j - 1] / denom
                } else {
                    0.0
                };
                ndu[r][j] = saved + right[r + 1] * temp;
                saved = left[j - r] * temp;
            }
            ndu[j][j] = saved;
        }

        // 파생 출력 배열
        let mut ders = vec![vec![0.0; p + 1]; n + 1];
        for j in 0..=p {
            ders[0][j] = ndu[j][p];
        }

        // a: 보조 테이블
        let mut a = vec![vec![0.0; p + 1]; 2];

        for r in 0..=p {
            let mut s1 = 0usize;
            let mut s2 = 1usize;

            a[0][0] = 1.0;

            for k in 1..=n {
                let rk = r as isize - k as isize;
                let pk = p as isize - k as isize;

                let mut d = 0.0;
                // 첫 항
                if r >= k {
                    let denom = ndu[(pk + 1) as usize][rk as usize];
                    a[s2][0] = if denom.abs() > f64::EPSILON {
                        a[s1][0] / denom
                    } else {
                        0.0
                    };
                    d += a[s2][0] * ndu[rk as usize][pk as usize];
                } else {
                    a[s2][0] = 0.0;
                }

                // 중간 항
                let j1 = if rk >= 0 { 1 } else { (-rk) as usize };
                let j2 = if (r as isize - 1) <= pk { k - 1 } else { p - r };

                for j in j1..=j2 {
                    let denom = ndu[(pk + 1) as usize][(rk + j as isize) as usize];
                    let num = a[s1][j] - a[s1][j - 1];
                    a[s2][j] = if denom.abs() > f64::EPSILON {
                        num / denom
                    } else {
                        0.0
                    };
                    d += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
                }

                // 마지막 항
                if r <= (pk as usize) {
                    let denom = ndu[(pk + 1) as usize][r];
                    a[s2][k] = if denom.abs() > f64::EPSILON {
                        -a[s1][k - 1] / denom
                    } else {
                        0.0
                    };
                    d += a[s2][k] * ndu[r][pk as usize];
                } else {
                    a[s2][k] = 0.0;
                }

                ders[k][r] = d;

                // swap rows
                for j in 0..=k {
                    a[s1][j] = 0.0;
                }
                std::mem::swap(&mut s1, &mut s2);
            }
        }

        // 스케일링: k차 도함수에 (p)(p-1)…(p-k+1)
        let mut fac = 1.0;
        for k in 1..=n {
            fac *= (p + 1 - k) as f64;
            for j in 0..=p {
                ders[k][j] *= fac;
            }
        }

        ders
    }
}
```
```rust
#[inline]
pub fn ensure_param_in_knot_domain(kv: &KnotVector, u: Param) -> Result<()> {
    let u_vec = kv.as_slice();
    let umin = u_vec[0];
    let umax = u_vec[u_vec.len() - 1];
    if u < umin || u > umax {
        return Err(NurbsError::SpanOutOfRange { u });
    }
    Ok(())
}
```
```rust
#[inline]
pub fn uniform_params_in_open_range(knots: &[Real], p: Degree, n_seg: usize) -> Vec<Real> {
    // Open interval [U[p], U[m - p]]
    let m = knots.len().saturating_sub(1);
    if m == 0 || (p as usize) > m {
        return vec![];
    }
    let a = knots[p as usize];
    let b = knots[m - p as usize];
    if n_seg == 0 {
        return vec![];
    }
    let mut t = Vec::with_capacity(n_seg + 1);
    for k in 0..=n_seg {
        let s = (k as Real) / (n_seg as Real);
        t.push((1.0 - s) * a + s * b);
    }
    t
}
```
```rust
#[inline]
pub fn uniform_params_in_range(u0: Real, u1: Real, n_seg: usize) -> Vec<Real> {
    assert!(n_seg >= 1);
    let du = (u1 - u0) / (n_seg as Real);
    (0..=n_seg).map(|k| u0 + du * (k as Real)).collect()
}
```
```rust
#[inline]
pub fn on_clamped_uniform_knot_vector(p: usize, n: usize) -> Vec<f64> {
    let m = n + p;
    let mut knots = vec![0.0; m + 1];

    // Front clamped region
    for i in 0..=p {
        knots[i] = 0.0;
    }

    // Back clamped region
    for i in 0..=p {
        knots[n + i] = 1.0;
    }

    // Internal uniform subdivision
    if n > p {
        let step = 1.0 / (n - p) as f64;
        for i in p..n {
            knots[i] = knots[p] + step * (i - p) as f64;
        }
    }
    knots
}
```
```rust
#[inline]
pub fn last_index(len: usize) -> Option<usize> {
    len.checked_sub(1) // len==0 이면 None
}
```
```rust
#[inline]
fn open_range(knots: &[Real], p: Degree) -> Option<(Real, Real)> {
    let m = knots.len().wrapping_sub(1);
    if m == 0 {
        return None;
    }
    let pu = p as usize;
    if pu > m {
        return None;
    }
    Some((knots[pu], knots[m - pu]))
}
```
```rust
/// chord-length 파라미터 (0..1로 정규화)
pub fn on_chord_length_params(points: &[Point3D]) -> Vec<Real> {
    let m = points.len();
    assert!(m >= 2, "Need at least 2 points to parametrize.");
    let mut u = vec![0.0; m];
    let mut total = 0.0;
    for i in 1..m {
        let dx = points[i].x - points[i - 1].x;
        let dy = points[i].y - points[i - 1].y;
        let dz = points[i].z - points[i - 1].z;
        let d = (dx * dx + dy * dy + dz * dz).sqrt();
        total += d;
        u[i] = total;
    }
    if total > 0.0 {
        for i in 1..m {
            u[i] /= total;
        }
    }
    u
}
```
```rust
/// [0,1] 구간 균일 파라미터
#[inline]
pub fn on_uniform_length_params(points: &[Point3D]) -> Vec<Real> {
    let n = points.len();
    if n <= 1 {
        return vec![0.0; n];
    }
    let m = (n - 1) as Real;
    (0..n).map(|i| (i as Real) / m).collect()
}
```
```rust
/// averaging formula 로 내부 knot 생성 (Piegl & Tiller)
pub fn on_averaging_internal_curve_knots(params: &[Real], p: Degree) -> Vec<Real> {
    let m_pts = params.len();    // m+1 = #data = #ctrl
    let n = m_pts - 1;           // n   = #ctrl -1
    let pz = p as usize;

    // 정확한 길이: n + p + 2
    let mut u_vec = vec![0.0; n + pz + 2];

    // leading clamped
    for i in 0..=pz {
        u_vec[i] = params[0];
    }

    // 내부 knot: j = 1 .. n-p
    // U_{p+j} = (u_j + ... + u_{j+p-1}) / p
    if n > pz {
        for j in 1..=(n - pz) {
            let mut sum = 0.0;
            for i in j..=(j + pz - 1) {
                sum += params[i];
            }
            u_vec[pz + j] = sum / (pz as Real);
        }
    }

    // trailing clamped
    for i in (n + 1)..=(n + pz + 1) {
        u_vec[i] = params[n];
    }

    u_vec
}
```
```rust
pub fn on_averaging_internal_surface_knots(params: &[Real], p: Degree) -> Vec<Real> {
    let m_pts = params.len();    // m+1 = #data = #ctrl
    let n = m_pts - 1;           // n   = #ctrl -1
    let pz = p as usize;

    // 정확한 길이: n + p + 2
    let mut u_vec = vec![0.0; n + pz + 2];

    // leading clamped
    for i in 0..=pz {
        u_vec[i] = params[0];
    }

    // 내부 knot: j = 1 .. n-p
    // U_{p+j} = (u_j + ... + u_{j+p-1}) / p
    if n > pz {
        for j in 1..=(n - pz) {
            let mut sum = 0.0;
            for i in j..=(j + pz - 1) {
                sum += params[i];
            }
            u_vec[pz + j] = sum / (pz as Real);
        }
    }

    // trailing clamped
    for i in (n + 1)..=(n + pz + 1) {
        u_vec[i] = params[n];
    }

    u_vec
}
```
```rust
#[inline]
fn on_idx_nu(nu: usize, i: usize, j: usize) -> usize {
    i + nu * j
}
```
```rust
pub fn on_ral_f2d_row_major(
    vals: &mut Vec<Real>,
    old_nu: usize,
    old_nv: usize,
    new_nu: usize,
    new_nv: usize,
    fill: Real,
) {
    if old_nu == new_nu && old_nv == new_nv {
        return;
    }
    let mut new_vals = vec![fill; new_nu * new_nv];
    let cu = old_nu.min(new_nu);
    let cv = old_nv.min(new_nv);
    for j in 0..cv {
        for i in 0..cu {
            let ko = on_idx_nu(old_nu, i, j);
            let kn = on_idx_nu(new_nu, i, j);
            new_vals[kn] = vals[ko];
        }
    }
    *vals = new_vals;
}
```
```rust
/// 1D: CPoint 벡터 크기 맞춤 (겹침 복사)
pub fn on_ral_c1d(ctrl: &mut Vec<Point4D>, new_len: usize, fill: Point4D) {
    let old = ctrl.len();
    if new_len == old {
        return;
    }
    if new_len < old {
        ctrl.truncate(new_len);
    } else {
        ctrl.resize(new_len, fill);
    }
}
```
```rust
/// 2D(row-major 1D 저장): CPoint 그리드 재할당
/// old_nu x old_nv → new_nu x new_nv, 공통 영역 복사
pub fn on_ral_c2d_row_major(
    ctrl: &mut Vec<Point4D>,
    old_nu: usize,
    old_nv: usize,
    new_nu: usize,
    new_nv: usize,
    fill: Point4D,
) {
    if old_nu == new_nu && old_nv == new_nv {
        // 모양 그대로면 아무 것도 안함
        return;
    }

    let mut new_ctrl = vec![fill; new_nu * new_nv];
    let cu = old_nu.min(new_nu);
    let cv = old_nv.min(new_nv);

    for j in 0..cv {
        for i in 0..cu {
            let ko = on_idx_nu(old_nu, i, j);
            let kn = on_idx_nu(new_nu, i, j);
            new_ctrl[kn] = ctrl[ko];
        }
    }
    *ctrl = new_ctrl;
}
```
```rust
/// 1D: Real 벡터 크기 맞춤 (SFun.values 등)
pub fn on_ral_f1d(vals: &mut Vec<Real>, new_len: usize, fill: Real) {
    let old = vals.len();
    if new_len == old {
        return;
    }
    if new_len < old {
        vals.truncate(new_len);
    } else {
        vals.resize(new_len, fill);
    }
}
```
```rust
/// 1D: Index 벡터
pub fn on_ral_i1d(arr: &mut Vec<Index>, new_len: usize, fill: Index) {
    let old = arr.len();
    if new_len == old {
        return;
    }
    if new_len < old {
        arr.truncate(new_len);
    } else {
        arr.resize(new_len, fill);
    }
}
```
```rust
/// 2D(row-major 1D 저장): Index 그리드
pub fn on_ral_i2d_row_major(
    arr: &mut Vec<Index>,
    old_nu: usize,
    old_nv: usize,
    new_nu: usize,
    new_nv: usize,
    fill: Index,
) {
    if old_nu == new_nu && old_nv == new_nv {
        return;
    }
    let mut new_arr = vec![fill; new_nu * new_nv];
    let cu = old_nu.min(new_nu);
    let cv = old_nv.min(new_nv);
    for j in 0..cv {
        for i in 0..cu {
            let ko = on_idx_nu(old_nu, i, j);
            let kn = on_idx_nu(new_nu, i, j);
            new_arr[kn] = arr[ko];
        }
    }
    *arr = new_arr;
}
```
```rust
#[inline]
fn on_eps_from_range(range: f64) -> f64 {
    let base = range.abs().max(1.0);
    ON_TOL12 * base
}
```
```rust
#[inline]
pub fn on_is_rat(ctrl: &[Point4D]) -> bool {
    ctrl.iter().any(|c| c.w.is_finite() && c.w != 1.0)
}
```
```rust
pub fn on_left(u: &[f64]) -> f64 {
    u.first().copied().unwrap_or(0.0)
}
```
```rust
/// U[last]
pub fn on_right(u: &[f64]) -> f64 {
    u.last().copied().unwrap_or(0.0)
}
```
```rust
pub fn on_length_range(u: &[f64]) -> f64 {
    if u.len() < 2 {
        0.0
    } else {
        on_right(u) - on_left(u)
    }
}
```
```rust
pub fn on_min_acceptable_knot_distance(u: &[f64], p: usize) -> f64 {
    let mut distinct = 0usize;
    for i in p..(u.len() - p - 1) {
        if u[i + 1] != u[i] {
            distinct += 1;
        }
    }
    if distinct == 0 {
        return 1e-7;
    }
    1e-7 * on_length_range(u) / (distinct as f64)
}
```
```rust
pub fn on_last_leq_index(u: &[f64], x: f64) -> usize {
    let (mut lo, mut hi) = (0usize, u.len() - 1);
    while lo + 1 < hi {
        let mid = (lo + hi) >> 1;
        if u[mid] <= x || on_are_equal_rel_tol(u[mid], x) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    if u[hi] <= x || on_are_equal_rel_tol(u[hi], x) {
        hi
    } else {
        lo
    }
}
```
```rust
/// Multiplicity at a given knot index
pub fn on_knot_multiplicity(u: &[f64], i: usize) -> usize {
    if u.is_empty() {
        return 0;
    }
    let x = u[i];
    let mut m = 1usize;

    let mut k = i;
    while k > 0 && on_are_equal_rel_tol(u[k - 1], x) {
        m += 1;
        k -= 1;
    }
    let mut k = i;
    while k + 1 < u.len() && on_are_equal_rel_tol(u[k + 1], x) {
        m += 1;
        k += 1;
    }
    m
}
```
```rust
/// FindSpan + multiplicity
/// k = span index (or last_knot index), s = multiplicity (equals boundary count if identical, otherwise 0)
pub fn on_find_span_multi(u: &[f64], x: f64, p: usize) -> (usize, usize) {
    let k = on_last_leq_index(u, x);
    let s = on_knot_multiplicity(u, k);
    // Endpoint correction: if the vector is clamped, then s ≥ p + 1 is guaranteed
    let s = if (k == 0 || k == u.len() - 1) && s < p + 1 {
        p + 1
    } else {
        s
    };
    (k, s)
}
```
```rust
/// find_span_multi_with_snap with snapping to close_knot (min_knot_dist)
pub fn on_find_span_multi_with_snap(
    u_vec: &[f64],
    u: &mut f64,
    p: usize,
    min_knot_dist: f64,
) -> (usize, usize) {
    for &kv in u_vec {
        if (kv - *u).abs() < min_knot_dist {
            *u = kv;
            break;
        }
    }
    on_find_span_multi(u_vec, *u, p)
}
```
```rust
/// U <- offset - reverse(U)
pub fn on_reverse_with_offset(u: &mut [f64], offset: f64) {
    let old = u.to_vec();
    let mut j = old.len();
    for i in 0..u.len() {
        j -= 1;
        u[i] = offset - old[j];
    }
}
```
```rust
pub fn on_reverse(u: &mut [f64]) {
    let off = 0.0;
    on_reverse_with_offset(u, off);
}
```
```rust
pub fn on_offset(u: &mut [f64], delta: f64) {
    for x in u {
        *x += delta;
    }
}
```
```rust
pub fn on_scale(u: &mut [f64], factor: f64) {
    for x in u {
        *x *= factor;
    }
}
```
```rust
/// Normalize to [0,1] by affine transform: shift by -U[0], scale by 1/(U[last]-U[0])
pub fn on_normalize(u: &mut [f64]) {
    if u.is_empty() {
        return;
    }
    let l = on_left(u);
    if l != 0.0 {
        on_offset(u, -l);
    }
    let r = on_length_range(u);
    if r != 0.0 {
        on_scale(u, 1.0 / r);
    }
}
```
```rust
/// Equivalent to C#'s IsClamped(out start, out end): checks clamping status,
/// and if `start` or `end` is true, snaps `p` control points at the corresponding boundary to identical values.
/// - `p`: degree, `n`: last valid span index (= n = num_ctrl - 1)
pub fn on_is_clamped_mut(u: &mut [f64], p: usize, n: usize) -> (bool, bool) {
    let m = p + n + 1; // ← m = n+p+1 (last index)
    assert!(u.len() >= m + 1, "knot vector size not consistent with p,n");
    let a = u[0];
    let b = u[m]; // ← the last element is u[m]

    let rng = on_length_range(u);
    let start = on_are_equal(a, u[p], rng);
    let end = on_are_equal(u[n + 1], b, rng); // ← Right-side clamping compares u[n + 1] and u[m]

    // Snapshots of p boundaries
    for i in 0..p {
        if start {
            u[i + 1] = a;
        }
        if end {
            u[m - 1 - i] = b;
        } // Snap the right boundary block into b
    }
    (start, end)
}
```
```rust
#[inline]
pub fn on_close(a: f64, b: f64) -> bool {
    let scale = a.abs().max(b.abs()).max(1.0);
    (a - b).abs() <= ON_TOL9 * scale
}
```
```rust
#[inline]
fn on_multiplicity_from_start(u: &[f64], m: usize) -> usize {
    if u.is_empty() {
        return 0;
    }
    let m = m.min(u.len() - 1);
    let a = u[0];
    let mut k = 1usize;
    while k <= m && on_close(u[k], a) {
        k += 1;
    }
    k // k values are identical (indices 0..=k-1), i.e., multiplicity = k
}
```
```rust
#[inline]
fn on_multiplicity_from_end(u: &[f64], m: usize) -> usize {
    if u.is_empty() {
        return 0;
    }
    let m = m.min(u.len() - 1);
    let b = u[m];
    let mut count = 1usize; // Including u[m]
    let mut i = m;
    while i > 0 && on_close(u[i - 1], b) {
        count += 1;
        i -= 1;
    }
    count
}
```
```rust
pub fn on_is_clamped(u: &[f64], p: usize, m: usize) -> bool {
    if u.len() < 2 || p == 0 {
        return false;
    }
    let m = m.min(u.len() - 1);

    // (Optional) Fast check for non-decreasing order
    for w in u.windows(2) {
        if w[0] > w[1] && !on_close(w[0], w[1]) {
            return false;
        }
    }

    let left_multi = on_multiplicity_from_start(u, m);
    let right_multi = on_multiplicity_from_end(u, m);

    // Open-clamped condition: multiplicity at both ends must be ≥ p + 1
    left_multi >= p + 1 && right_multi >= p + 1
}
```
```rust
/// Number of spans: count of positions where U[i] > U[i-1] (for i = p..=n)
pub fn on_span_count(u: &[f64], p: usize, n: usize) -> usize {
    let mut cnt = 0usize;
    for i in p..=n {
        if u[i] > u[i - 1] {
            cnt += 1;
        }
    }
    cnt
}
```
```rust
/// Multiplicity at a given knot index
pub fn on_multiplicity(u: &[f64], i: usize) -> usize {
    if u.is_empty() {
        return 0;
    }
    let x = u[i];
    let mut m = 1usize;

    let mut k = i;
    while k > 0 && on_close(u[k - 1], x) {
        m += 1;
        k -= 1;
    }
    let mut k = i;
    while k + 1 < u.len() && on_close(u[k + 1], x) {
        m += 1;
        k += 1;
    }
    m
}
```
```rust
/// Validity conditions:
///   - Boundary: p + 1 identical values at each end
///   - Interior: non-decreasing order and multiplicity ≤ p
pub fn on_is_valid_with_pn(u: &[f64], p: usize, n: usize) -> bool {
    // 1) Check for length consistency (first step)
    let m = n + p + 1;
    if u.len() != m + 1 {
        return false;
    }

    // 2) Non-decreasing (with tolerance)
    for w in u.windows(2) {
        if w[0] > w[1] && !on_close(w[0], w[1]) {
            return false;
        }
    }

    // 3) Endpoint multiplicity ≥ p + 1  (U[0..p] are equal, U[m−p..m] are equal)
    let u0 = u[0];
    let um = u[m];
    for i in 0..=p {
        if !on_close(u[i], u0) {
            return false;
        }
        if !on_close(u[m - i], um) {
            return false;
        }
    }

    // 4) The values immediately after/before the boundaries must strictly enter the interior
    //     That is, U[p+1] > U[0], U[m−p−1] < U[m] — equality within tolerance is considered a failure
    if on_close(u[p + 1], u0) {
        return false;
    }
    if on_close(u[m - p - 1], um) {
        return false;
    }

    // 5) Internal multiplicity ≤ p
    //     (restriction applies only to non-boundary regions)
    let mut mult = 1usize;
    for i in 1..=m {
        if on_close(u[i], u[i - 1]) {
            mult += 1;
            // Restriction applies only to the interior: i ∈ [p + 1 .. m − p − 1]
            //     Alternatively, "when the current block is not a boundary block"
            let left_block = i <= p; // 아직 왼쪽 경계 블록
            let right_block = i > m - p; // 이미 오른쪽 경계 블록
            if !left_block && !right_block && mult > p {
                return false;
            }
        } else {
            mult = 1;
        }
    }

    true
}
```
```rust
/// Interior validity = non-decreasing + multiplicity ≤ p
pub fn on_is_valid_with_p(u: &[f64], p: usize) -> bool {
    if u.len() < 2 {
        return true;
    }
    let mut mult = 1usize;
    for i in 0..u.len() - 1 {
        // Non-decreasing
        if u[i] > u[i + 1] && !on_close(u[i], u[i + 1]) {
            return false;
        }
        // Internal multiplicity ≤ p (excluding boundaries)
        if on_close(u[i], u[i + 1]) {
            mult += 1;
            // Restriction applies only to interior intervals
            if i > 0 && i + 1 < u.len() - 1 && mult > p {
                return false;
            }
        } else {
            mult = 1;
        }
    }
    true
}
```
```rust
/// Finds the first "very close" knot interval and returns its index (on the side with lower multiplicity) and that multiplicity
pub fn on_first_similar_knot_index(u: &[f64], p: usize, min_dist: f64) -> Option<(usize, usize)> {
    for i in p..(u.len() - p - 1) {
        if u[i] != u[i + 1] && (u[i + 1] - u[i]).abs() < min_dist {
            let (k1, s1) = on_find_span_multi(u, u[i], p);
            let (k2, s2) = on_find_span_multi(u, u[i + 1], p);
            return if s1 < s2 {
                Some((k1, s1))
            } else {
                Some((k2, s2))
            };
        }
    }
    None
}
```
```rust
/// KnotVectorType classification
pub fn on_detect_knot_type(u: &[f64], p: usize) -> KnotVectorType {
    if u.len() < 2 {
        return KnotVectorType::Undefined;
    }
    let m = u.len() - 1;

    let left_m = on_multiplicity(u, 0);
    let right_m = on_multiplicity(u, m);
    let clamped = left_m >= p + 1 && right_m >= p + 1;

    // Extract internal nodes
    let il = left_m.min(m);
    let ir = m.saturating_sub(right_m);
    let interior = if il <= ir { &u[il..=ir] } else { &[][..] };

    // Check for uniform spacing after normalization
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
        diffs
            .iter()
            .all(|&d| (d - d0).abs() <= 1e-10_f64.max(1e-8 * d0.abs()))
    };

    match (clamped, uniform) {
        (true, true) => KnotVectorType::ClampedUniform,
        (true, false) => KnotVectorType::ClampedNonUniform,
        (false, true) => KnotVectorType::UnClampedUniform,
        (false, false) => KnotVectorType::UnClampedNonUniform,
    }
}
```
```rust
/// Splitting near the median: (remaining insert count, splitPt, midVal)
/// Equivalent to C# Split(U, m, k, out splitPt, out midVal)
pub fn on_split_mid(u: &[f64], m: usize, k: usize) -> (i32, usize, f64) {
    // m: p + n, k: degree
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
    if index4 == 0 {
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
```
```rust
/// Split with target u value around an index
pub fn on_split_at(u: &[f64], param: f64, mut middex: usize, m: usize, k: usize) -> (i32, usize) {
    let len = m + k + 1;
    assert_eq!(len, u.len());
    let a = u[middex];

    if !on_are_equal(a, param, on_length_range(u)) {
        while u[middex + 1] < a {
            middex += 1;
        }
        return (k as i32, middex);
    }
    // Count multiplicity of identical knot values
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
```
```rust
/// Helper for generating open-uniform knots (for testing)
pub fn on_open_uniform_knots(n_ctrl: usize, degree: usize) -> Vec<f64> {
    let m = n_ctrl + degree + 1;
    let mut k = vec![0.0; m];
    for i in 0..m {
        if i <= degree {
            k[i] = 0.0;
        } else if i >= n_ctrl {
            k[i] = 1.0;
        } else {
            k[i] = (i - degree) as f64 / (n_ctrl - degree) as f64;
        }
    }
    k
}
```
```rust
pub fn on_quantile_sorted(sorted: &[f64], mut alpha: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if alpha.is_nan() {
        alpha = 0.0;
    }
    if alpha < 0.0 {
        alpha = 0.0;
    }
    if alpha > 1.0 {
        alpha = 1.0;
    }
    let n = sorted.len();
    if n == 1 {
        return sorted[0];
    }
    let pos = alpha * (n as f64 - 1.0);
    let i0 = pos.floor() as usize;
    let i1 = pos.ceil() as usize;
    if i0 == i1 {
        return sorted[i0];
    }
    let w = pos - i0 as f64;
    (1.0 - w) * sorted[i0] + w * sorted[i1]
}
```
```rust
pub fn on_quantile(data: &[f64], alpha: f64) -> f64 {
    let mut v = data.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    on_quantile_sorted(&v, alpha)
}
```
```rust
/// Quantile-based clamped_knot (evenly distributed over scatter)
pub fn on_averaging_knots_quantile(params: &[f64], degree: usize, n_ctrl: usize) -> Vec<f64> {
    let p = degree;
    let n = n_ctrl - 1;
    let mut u = vec![0.0; n + p + 2];
    for j in 0..=p {
        u[j] = 0.0;
    }
    for j in (n + 1)..=(n + p + 1) {
        u[j] = 1.0;
    }
    let n_int = n_ctrl as isize - degree as isize - 1;
    if n_int <= 0 {
        return u;
    }

    let mut sorted = params.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    for r in 1..=n_int as usize {
        let alpha = r as f64 / (n_int as f64 + 1.0);
        u[degree + r] = on_quantile_sorted(&sorted, alpha);
    }
    for i in 1..u.len() {
        if u[i] < u[i - 1] {
            u[i] = u[i - 1];
        }
    }
    u
}
```
```rust
pub fn on_averaging_knots_from_params(params: &[f64], degree: usize, n_ctrl: usize) -> Vec<f64> {
    let p = degree;
    let n = n_ctrl - 1;
    let mut u = vec![0.0; n + p + 2];
    for j in 0..=p {
        u[j] = 0.0;
    }
    for j in (n + 1)..=(n + p + 1) {
        u[j] = 1.0;
    }
    if n_ctrl <= p + 1 {
        return u;
    }
    let m = params.len().saturating_sub(1);
    for j in 1..=(n - p) {
        let a = j;
        let b = (j + p - 1).min(m.saturating_sub(1));
        let mut s = 0.0;
        let mut cnt = 0;
        for k in a..=b {
            s += params[k];
            cnt += 1;
        }
        u[j + p] = if cnt > 0 { s / cnt as f64 } else { params[j] };
    }
    for i in 1..u.len() {
        if u[i] < u[i - 1] {
            u[i] = u[i - 1];
        }
    }
    u
}
```
```rust
pub fn on_chord_length_parametrization<T: AsRef<Point3D>>(
    i0: usize,
    i1: usize,
    pts: &[T],
    distances: &mut Vec<f64>,
    u: &mut Vec<f64>,
) -> f64 {
    distances.clear();
    u.clear();
    let mut total = 0.0;
    let mut last: Option<Point3D> = None;
    for i in i0..i1 {
        let p = pts[i].as_ref();
        if let Some(lp) = last {
            let d = lp.distance(&p);
            distances.push(d);
            total += d;
        } else {
            distances.push(0.0);
        }
        last = Some(*p);
    }
    let mut acc = 0.0;
    u.push(0.0);
    for i in 1..distances.len() {
        acc += distances[i];
        u.push(if total > 0.0 { acc / total } else { 0.0 });
    }
    if u.is_empty() {
        u.push(0.0);
    }
    total
}
```
```rust
fn on_find_index_of_min_value(arr: &[f64]) -> usize {
    let mut best = arr[0];
    let mut idx = 0usize;
    for i in 1..arr.len() {
        if arr[i] <= best {
            best = arr[i];
            idx = i;
        }
    }
    idx
}
```
```rust
pub fn on_expand_knot_vector(src: &[f64], p: usize) -> Vec<f64> {
    let n = src.len();
    assert!(n > 0, "src must be non-empty");
    let mut dst = vec![0.0f64; n + p + 1];
    if p > 0 && n > p {
        for index1 in 1..(n - p) {
            let sum: f64 = src[index1..index1 + p].iter().copied().sum();
            dst[index1 + p] = sum / (p as f64);
        }
    }
    for i in 0..=p {
        dst[i] = src[0];
    }
    // Last k + 1 values: filled with src[n - 1]
    let start = dst.len() - p - 1; // = n
    for i in start..dst.len() {
        dst[i] = src[n - 1];
    }
    dst
}
```
```rust
/// 파라메터 배열(0..1)과 차수 p로 clamped knot vector를 만든다 (Uniform 확장).
pub fn on_expand_knot_vector_clamped(params: &[f64], p: usize) -> Vec<f64> {
    // [0..0] * (p+1), 내부 노드는 params[1..len-2], [1..1] * (p+1)
    let n = params.len();
    let mut kv = Vec::with_capacity(n + p + 1);
    for _ in 0..=p {
        kv.push(0.0);
    }
    if n > 2 {
        for k in 1..(n - 1) {
            kv.push(params[k]);
        }
    }
    for _ in 0..=p {
        kv.push(1.0);
    }
    kv
}
```
```rust
pub fn on_build_basis_matrix(
    params: &[f64],
    n_ctrl: usize,
    degree: usize,
    u_vec: &[f64],
) -> Vec<Vec<f64>> {
    let mut a_vec = vec![vec![0.0f64; n_ctrl]; params.len()];
    let p = degree;
    let n = n_ctrl - 1;
    let umin = u_vec[p];
    let umax = u_vec[n + 1];

    let mut n_vec = vec![0.0f64; p + 1];
    for (r, &t) in params.iter().enumerate() {
        let u = t.clamp(umin, umax);

        let span = on_find_span_usize(u_vec, n, p, u);
        n_vec.copy_from_slice(&on_basis_func_ret_vec(u_vec, span, u, p));
        let first = span - p;
        for j in 0..=p {
            a_vec[r][first + j] = n_vec[j];
        }
    }
    a_vec
}
```
```rust
pub fn on_fit_surface_least_squares(
    p_vec: &[Vec<Point3D>], // samples: nu × mv (행=u, 열=v)
    u_par: &[f64],          // nu
    v_par: &[f64],          // mv
    p: usize,
    q: usize, // degrees
    ncu: usize,
    ncv: usize, // # control in u, v
) -> Option<(Vec<f64>, Vec<f64>, Vec<Vec<Point3D>>)> {
    // knots

    let u_vec = on_clamped_uniform_knot_vector(p, ncu);
    let v_vec = on_clamped_uniform_knot_vector(q, ncv);

    let nu = p_vec.len();
    if nu == 0 {
        return None;
    }
    let mv = p_vec[0].len();

    // basis matrices Au (nu×ncu), Av (mv×ncv)
    let au_mat = on_build_basis_matrix(u_par, ncu, p, &u_vec);
    let av_mat = on_build_basis_matrix(v_par, ncv, q, &v_vec);

    // Gram matrices Gu = Au^T Au, Gv = Av^T Av
    let mut gu_mat = vec![vec![0.0f64; ncu]; ncu];
    for i in 0..ncu {
        for j in 0..ncu {
            let mut s = 0.0;
            for r in 0..nu {
                s += au_mat[r][i] * au_mat[r][j];
            }
            gu_mat[i][j] = s;
        }
    }
    let mut gv_mat = vec![vec![0.0f64; ncv]; ncv];
    for i in 0..ncv {
        for j in 0..ncv {
            let mut s = 0.0;
            for r in 0..mv {
                s += av_mat[r][i] * av_mat[r][j];
            }
            gv_mat[i][j] = s;
        }
    }

    // First, expand in U-direction to intermediate grid D(ncu × mv)
    let mut d_mat = vec![vec![Point3D::new(0.0, 0.0, 0.0); mv]; ncu];
    {
        for j in 0..mv {
            let mut bx = vec![0.0; ncu];
            let mut by = vec![0.0; ncu];
            let mut bz = vec![0.0; ncu];

            for i in 0..ncu {
                let mut sx = 0.0;
                let mut sy = 0.0;
                let mut sz = 0.0;
                for r in 0..nu {
                    sx += au_mat[r][i] * p_vec[r][j].x;
                    sy += au_mat[r][i] * p_vec[r][j].y;
                    sz += au_mat[r][i] * p_vec[r][j].z;
                }
                bx[i] = sx;
                by[i] = sy;
                bz[i] = sz;
            }

            // solve (each coordinate is independent)
            let mut gx_mat = Matrix::from_nested_vec(&gu_mat);
            let mut gy_mat = Matrix::from_nested_vec(&gu_mat);
            let mut gz_mat = Matrix::from_nested_vec(&gu_mat);

            if !on_solve_linear_system_vec(&mut gx_mat, &mut bx) {
                return None;
            }
            if !on_solve_linear_system_vec(&mut gy_mat, &mut by) {
                return None;
            }
            if !on_solve_linear_system_vec(&mut gz_mat, &mut bz) {
                return None;
            }

            for i in 0..ncu {
                d_mat[i][j] = Point3D::new(bx[i], by[i], bz[i]);
            }
        }
    }

    // Then, expand in V-direction to final control points C(ncu × ncv)
    let mut c_mat = vec![vec![Point3D::new(0.0, 0.0, 0.0); ncv]; ncu];
    for i in 0..ncu {
        let mut bx = vec![0.0; ncv];
        let mut by = vec![0.0; ncv];
        let mut bz = vec![0.0; ncv];

        for c in 0..ncv {
            let mut sx = 0.0;
            let mut sy = 0.0;
            let mut sz = 0.0;
            for r in 0..mv {
                sx += av_mat[r][c] * d_mat[i][r].x;
                sy += av_mat[r][c] * d_mat[i][r].y;
                sz += av_mat[r][c] * d_mat[i][r].z;
            }
            bx[c] = sx;
            by[c] = sy;
            bz[c] = sz;
        }

        let mut gx_mat = Matrix::from_nested_vec(&gv_mat);
        let mut gy_mat = Matrix::from_nested_vec(&gv_mat);
        let mut gz_mat = Matrix::from_nested_vec(&gv_mat);

        if !on_solve_linear_system_vec(&mut gx_mat, &mut bx) {
            return None;
        }
        if !on_solve_linear_system_vec(&mut gy_mat, &mut by) {
            return None;
        }
        if !on_solve_linear_system_vec(&mut gz_mat, &mut bz) {
            return None;
        }

        for c in 0..ncv {
            c_mat[i][c] = Point3D::new(bx[c], by[c], bz[c]);
        }
    }

    Some((u_vec, v_vec, c_mat))
}
```
```rust
pub fn on_get_first_similar_knot_index(
    u_vec: &[f64],
    p: usize,
    min_dist: f64,
) -> Option<(usize, i32)> {
    if u_vec.len() < 2 * p + 2 {
        return None;
    }
    // i in [p .. len-p-2]
    for i in p..(u_vec.len().saturating_sub(p + 1)) {
        let a = u_vec[i];
        let b = u_vec[i + 1];
        if (b > a) && (b - a) < min_dist {
            // multiplicity at a, b
            let (ka, sa) = on_find_span_multi(u_vec, a, p);
            let (kb, sb) = on_find_span_multi(u_vec, b, p);
            if sa <= sb {
                return Some((ka, sa as i32));
            } else {
                return Some((kb, sb as i32));
            }
        }
    }
    None
}
```
```rust
/// knots 안에서 값 t의 중복도를 반환 (|u - t| <= tol 기준)
pub fn on_multiplicity_value(knots: &[f64], t: f64, tol: f64) -> usize {
    if knots.is_empty() {
        return 0;
    }
    // t 근처 블록을 찾아서 개수 반환
    match on_find_knot_block(knots, t, tol) {
        Some((s, _r_left)) => s,
        None => 0,
    }
}
```
```rust
pub fn on_find_knot_block(knots: &[f64], t: f64, tol: f64) -> Option<(usize, usize)> {
    if knots.is_empty() {
        return None;
    }

    // t에 가장 가까운 인덱스를 이진탐색 비슷하게 찾고,
    // 그 인접 구간에서 |u-t|<=tol 인 블록을 좌우로 확장.
    // (단순 선형 탐색도 충분히 작을 때는 OK)
    // 먼저 t와 가까운 인덱스 탐색
    let mut lo = 0usize;
    let mut hi = knots.len();
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if knots[mid] <= t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    // lo, lo+1 중 t에 더 가까운 쪽으로 시작점 선택
    let mut idx = lo;
    if lo + 1 < knots.len() && (knots[lo + 1] - t).abs() < (knots[lo] - t).abs() {
        idx = lo + 1;
    }
    if (knots[idx] - t).abs() > tol {
        return None;
    }

    // 좌우로 확장하여 블록 경계 찾기
    let mut left = idx;
    while left > 0 && (knots[left - 1] - t).abs() <= tol {
        left -= 1;
    }
    let mut right = idx;
    while right + 1 < knots.len() && (knots[right + 1] - t).abs() <= tol {
        right += 1;
    }

    let mult = right - left + 1;
    Some((mult, left))
}
```
```rust
pub fn on_ders_basis_func(
    knots: &[f64],
    span: usize,
    u: f64,
    p: usize,
    nder: usize,
) -> Vec<Vec<f64>> {
    let nder = nder.min(p);
    let mut ders = vec![vec![0.0; p + 1]; nder + 1];

    // ndu[r][j], j=0..p, r=0..j
    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];

    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - knots[span + 1 - j];
        right[j] = knots[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            let denom = right[r + 1] + left[j - r];
            let temp = if denom.abs() > 1e-15 {
                ndu[r][j - 1] / denom
            } else {
                0.0
            };
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    // 0차: basis
    for r in 0..=p {
        ders[0][r] = ndu[r][p];
    }

    // A2.3: higher derivatives
    let mut a = vec![vec![0.0; p + 1]; 2];
    for r in 0..=p {
        let mut s1 = 0;
        let mut s2 = 1;
        a[0][0] = 1.0;
        for k in 1..=nder {
            let mut d = 0.0;
            let rk = r as isize - k as isize; // r - k
            let pk = p as isize - k as isize; // p - k

            // 첫 항
            if r as isize >= k as isize {
                // a[s2][0] = a[s1][0] / ndu[r-k][p-k+1]
                let denom = ndu[rk as usize][pk as usize + 1];
                a[s2][0] = if denom.abs() < 1e-15 {
                    0.0
                } else {
                    a[s1][0] / denom
                };
                // d += a[s2][0] * ndu[r-k][p-k]
                d += a[s2][0] * ndu[rk as usize][pk as usize];
            }

            // 중간 항들
            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if (r as isize - 1) <= pk { k - 1 } else { p - r };
            for j in j1..=j2 {
                // a[s2][j] = (a[s1][j] - a[s1][j-1]) / ndu[r-k+j][p-k+1]
                let denom = ndu[(rk + j as isize) as usize][pk as usize + 1];
                a[s2][j] = if denom.abs() < 1e-15 {
                    0.0
                } else {
                    (a[s1][j] - a[s1][j - 1]) / denom
                };
                // d += a[s2][j] * ndu[r-k+j][p-k]
                d += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
            }

            // 마지막 항
            if (r as isize) <= pk {
                // a[s2][k] = -a[s1][k-1] / ndu[r][p-k+1]
                let denom = ndu[r][pk as usize + 1];
                a[s2][k] = if denom.abs() < 1e-15 {
                    0.0
                } else {
                    -a[s1][k - 1] / denom
                };
                // d += a[s2][k] * ndu[r][p-k]
                d += a[s2][k] * ndu[r][pk as usize];
            }

            ders[k][r] = d;
            // swap rows
            s1 ^= 1;
            s2 ^= 1;
        }
    }

    // 스케일링
    let mut rfact = p as f64;
    for k in 1..=nder {
        for r in 0..=p {
            ders[k][r] *= rfact;
        }
        rfact *= (p - k) as f64;
    }
    ders
}
```
```rust
pub fn on_clamped_bezier_knot(p: usize) -> Vec<f64> {
    let mut u = vec![0.0; p + 1];
    u.extend(std::iter::repeat(1.0).take(p + 1));
    u
}
```
```rust
pub fn on_is_clamped_knot_degree(knots: &[f64], degree: usize) -> bool {

    if knots.len() < 2 * (degree + 1) {
        return false;
    }
    let t0 = knots[0];
    let t1 = *knots.last().unwrap();
    if !knots.iter().take(degree + 1).all(|&k| on_are_equal(k, t0, 1e-12)) {
        return false;
    }
    if !knots.iter().rev().take(degree + 1).all(|&k| on_are_equal(k, t1, 1e-12)) {
        return false;
    }
    true
}
```
```rust
pub fn on_merge_knot_vectors(a: &[Real], b: &[Real], eps: Real) -> Vec<Real> {
    let mut ia = 0;
    let mut ib = 0;
    let mut out: Vec<Real> = Vec::with_capacity(a.len() + b.len());

    while ia < a.len() || ib < b.len() {
        let v = if ia < a.len() && ib < b.len() {
            let va = a[ia];
            let vb = b[ib];

            if (va - vb).abs() <= eps {
                // 거의 같은 위치의 knot → 하나만 사용
                ia += 1;
                ib += 1;
                va
            } else if va < vb {
                ia += 1;
                va
            } else {
                ib += 1;
                vb
            }
        } else if ia < a.len() {
            let va = a[ia];
            ia += 1;
            va
        } else {
            let vb = b[ib];
            ib += 1;
            vb
        };

        // out 마지막 값과 거의 같으면 건너뛰기
        let push_needed = match out.last() {
            None => true,
            Some(last) => (v - *last).abs() > eps,
        };
        if push_needed {
            out.push(v);
        }
    }

    out
}
```
```rust
/// 그리드 점(pts, row-major: idx = u + nu * v)에 대해
/// U 방향 chord-length 기반 파라미터 u[0..nu-1] 생성
pub fn on_chord_length_params_in_u(pts: &[Point3D], nu: usize, nv: usize) -> Vec<Real> {
    assert!(nu >= 2 && nv >= 1);
    assert_eq!(pts.len(), nu * nv);

    let mut seg_sum_per_u = vec![0.0; nu]; // 각 세그먼트(u-1,u)의 길이 합
    let mut total_len = 0.0;

    // C# LocalInterpolation: numArray1, numArray2, num1 부분에 해당
    for v in 0..nv {
        let mut row_total = 0.0;
        for u in 1..nu {
            let p0 = &pts[(u - 1) + nu * v];
            let p1 = &pts[u + nu * v];

            let dx = p1.x - p0.x;
            let dy = p1.y - p0.y;
            let dz = p1.z - p0.z;
            let d = (dx * dx + dy * dy + dz * dz).sqrt();

            seg_sum_per_u[u] += d;
            row_total += d;
        }
        total_len += row_total;
    }

    // 파라미터 u[0..nu-1]
    let mut u_params = vec![0.0; nu];
    if total_len > 0.0 {
        for i in 1..(nu - 1) {
            // C#: numArray1[i] = numArray1[i-1] + numArray1[i] / num1;
            u_params[i] = u_params[i - 1] + seg_sum_per_u[i] / total_len;
        }
    } else {
        // 모든 점이 같은 위치인 경우 등 → 균등 분포로 fallback
        for i in 1..(nu - 1) {
            u_params[i] = i as Real / (nu as Real - 1.0);
        }
    }
    u_params[nu - 1] = 1.0;

    u_params
}
```
```rust
/// 그리드 점(pts, row-major: idx = u + nu * v)에 대해
/// V 방향 chord-length 기반 파라미터 v[0..nv-1] 생성
pub fn on_chord_length_params_in_v(pts: &[Point3D], nu: usize, nv: usize) -> Vec<Real> {
    assert!(nu >= 1 && nv >= 2);
    assert_eq!(pts.len(), nu * nv);

    let mut seg_sum_per_v = vec![0.0; nv]; // 각 세그먼트(v-1,v)의 길이 합
    let mut total_len = 0.0;

    // C# LocalInterpolation: numArray3, numArray4, num3 부분에 해당
    for u in 0..nu {
        let mut col_total = 0.0;
        for v in 1..nv {
            let p0 = &pts[u + nu * (v - 1)];
            let p1 = &pts[u + nu * v];

            let dx = p1.x - p0.x;
            let dy = p1.y - p0.y;
            let dz = p1.z - p0.z;
            let d = (dx * dx + dy * dy + dz * dz).sqrt();

            seg_sum_per_v[v] += d;
            col_total += d;
        }
        total_len += col_total;
    }

    // 파라미터 v[0..nv-1]
    let mut v_params = vec![0.0; nv];
    if total_len > 0.0 {
        for j in 1..(nv - 1) {
            // C#: numArray3[j] = numArray3[j-1] + numArray3[j] / num3;
            v_params[j] = v_params[j - 1] + seg_sum_per_v[j] / total_len;
        }
    } else {
        // 모든 점이 같은 위치인 경우 등 → 균등 분포 fallback
        for j in 1..(nv - 1) {
            v_params[j] = j as Real / (nv as Real - 1.0);
        }
    }
    v_params[nv - 1] = 1.0;

    v_params
}
```
