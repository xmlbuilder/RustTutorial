
# Adaptive Curve Division by Length Profile

곡선 전체 길이를 보존하면서 시작·중앙·끝에서 서로 다른 세그먼트 길이를 갖도록 **u-파라미터 분포를 자동 생성하는 알고리즘** 입니다.

---

## 1. Overview

### Divide 밀도를 단순 균등 분할이 아니라:

- 시작 구간: 세그먼트 길이 5  
- 중간 구간: 세그먼트 길이 10  
- 끝 구간: 세그먼트 길이 3  

처럼 segment length profile 을 따라가면서도  
전체 분할된 세그먼트 길이의 합은 **정확히 100** 이 되도록 u-breakpoint 를 계산합니다.

---

## 2. 길이 프로파일 Length Profile

`ℓ(s)` (s는 정규화된 곡선 길이) 에 대해 다음을 만족하도록 구성합니다:

- `ℓ(0) = len_start`
- `ℓ(plateau 영역) = len_mid`
- `ℓ(1) = len_end`

```
l(s) =
    if s < a:
        ls + (lm - ls) * F_left(s/a)
    else if s <= 1 - a:
        lm
    else:
        le + (lm - le) * F_right((1 - s)/a)
```

---

## 3. 세그먼트 개수 계산

```
w(s) = total_length / l(s)
W(s) = ∫ w(s) ds
```

총 세그먼트 수:

```
N = round(W(1))
```

---

## 4. Inverse Mapping

```
W_k = W(1) * k / N
```

이를 만족하는 `s_k`를 찾기 위해 W(s)의 샘플 누적 배열을 사용하여 이분법으로 역으로 찾습니다.

---

## 5. U 값 찾기

정규화 길이 함수가 다음일 때:

```
s(u) = normalized arc length
```

목표는:

```
s(u_k) = s_k
```

이 되는 `u_k`를 이분법으로 찾는 것입니다.

---

## 6. 출력

- `u_breaks[]`: u=0..1 사이의 분할점
- `segment_lengths[]`: 각 세그먼트의 실제 길이  
- `Σ segment_lengths = total_length`

100% 길이 보존됨.

---

# 7. Rust 전체 코드

```rust
//! Adaptive curve division by length profile (Rust version)
//!
//! 곡선 전체 길이(total_length)를 보존하면서
//! 시작 / 중앙 / 끝에서 서로 다른 세그먼트 길이를 가지는 u-분포를 생성하는 유틸리티입니다.
//!
//! - 입력: arc_len_norm(u)  : [0,1] → [0,1]  (정규화 길이 함수)
//! - 입력: total_length     : 곡선 전체 실제 길이
//! - 입력: LengthProfileParams (시작/중간/끝 세그먼트 길이, plateau 비율, 지수 계수 등)
//! - 출력: (u_breaks, seg_lengths)
//!         u_breaks.len() = N+1, seg_lengths.len() = N
//!         Σ seg_lengths ≈ total_length
//!
//! C++ 버전을 그대로 옮긴 구조이며, Bezier / NURBS 등에서
//! arc_len_norm(u)만 구현해 주면 그대로 적용할 수 있습니다.

#[derive(Debug, Clone, Copy)]
pub struct LengthProfileParams {
    /// 시작 구간 세그먼트 길이 (예: 5.0)
    pub len_start: f64,
    /// 중앙(plateau) 구간 세그먼트 길이 (예: 10.0)
    pub len_mid: f64,
    /// 끝 구간 세그먼트 길이 (예: 3.0)
    pub len_end: f64,
    /// 중앙 plateau 비율 (0.0 ~ 1.0), 예: 0.6 → 가운데 60%에서 len_mid 유지
    pub plateau_fraction: f64,
    /// 좌측(시작)에서 중앙으로 가는 지수 전이 계수
    pub r_left: f64,
    /// 우측(끝)에서 중앙으로 가는 지수 전이 계수
    pub r_right: f64,
}
```
```rust
impl Default for LengthProfileParams {
    fn default() -> Self {
        Self {
            len_start: 5.0,
            len_mid: 10.0,
            len_end: 3.0,
            plateau_fraction: 0.6,
            r_left: 2.0,
            r_right: 2.0,
        }
    }
}
```
```rust
impl LengthProfileParams {
    pub fn new(
        len_start: f64,
        len_mid: f64,
        len_end: f64,
        plateau_fraction: f64,
        r_left: f64,
        r_right: f64,
    ) -> Self {
        Self {
            len_start,
            len_mid,
            len_end,
            plateau_fraction,
            r_left,
            r_right,
        }
    }
}
```
```rust
/// s ∈ [0,1] 에 대한 세그먼트 "목표 길이" 함수 ℓ(s)
///
/// - [0, a]         : len_start → len_mid (지수 전이)
/// - [a, 1-a]       : len_mid (plateau)
/// - [1-a, 1]       : len_mid → len_end (지수 전이)
fn length_profile(s: f64, p: &LengthProfileParams) -> f64 {
    let ls = p.len_start;
    let lm = p.len_mid;
    let le = p.len_end;

    let a = 0.5 * (1.0 - p.plateau_fraction);
    let a = a.max(0.0).min(0.5); // 안전장치: plateau_fraction이 이상해도 망가지지 않게

    if s <= 0.0 {
        return ls;
    }
    if s >= 1.0 {
        return le;
    }

    if s < a {
        // 좌측 구간 [0, a] : ls -> lm
        let x = s / a; // 0..1
        let k = p.r_left;
        let f = if k.abs() < 1.0e-8 {
            x
        } else {
            let ek = k.exp();
            let ekx = (k * x).exp();
            (ekx - 1.0) / (ek - 1.0)
        };
        ls + (lm - ls) * f
    } else if s <= 1.0 - a {
        // plateau
        lm
    } else {
        // 우측 구간 [1-a, 1] : lm -> le
        let x = (1.0 - s) / a; // 0..1
        let k = p.r_right;
        let f = if k.abs() < 1.0e-8 {
            x
        } else {
            let ek = k.exp();
            let ekx = (k * x).exp();
            (ekx - 1.0) / (ek - 1.0)
        };
        le + (lm - le) * f
    }
}
```
```rust
///
/// 곡선을 length profile에 따라 분할하고,
/// - u 분할점 (0..1 구간)
/// - 각 세그먼트의 실제 길이
/// 를 반환.
///
/// # 입력
/// - `arc_len_norm(u)` : [0,1] → [0,1]
///   - u 에 대해 정규화된 아크 길이 (0 ~ 1)를 반환
///   - 즉, s = arc_len_norm(u) = (0→u까지의 길이) / total_length
/// - `total_length` : 곡선 전체 길이
/// - `params`      : 시작/중간/끝 길이, plateau 비율, r_left/r_right
///
/// # 출력
/// - `Some((u_breaks, seg_lengths))`
///   - u_breaks.len() = N+1
///   - seg_lengths.len() = N
///   - seg_lengths 의 합 ≈ total_length
/// - 실패 시 `None`
///
/// # 주의
/// - arc_len_norm(u)가 [0,1]에서 **단조 증가**한다고 가정함.
///
pub fn on_divide_curve_by_length_profile<F>(
    arc_len_norm: F,
    total_length: f64,
    params: &LengthProfileParams,
) -> Option<(Vec<f64>, Vec<f64>)>
where
    F: Fn(f64) -> f64,
{
    if total_length <= 0.0 {
        return None;
    }

    // 1) s∈[0,1] 에서 w(s) = total_length / ℓ(s) 샘플링
    let samples: usize = 1024;
    let n = samples;

    let mut s_samples = vec![0.0f64; n + 1];
    let mut w_samples = vec![0.0f64; n + 1];
    let mut w_cum = vec![0.0f64; n + 1];

    for i in 0..=n {
        let s = i as f64 / n as f64;
        s_samples[i] = s;

        let mut l_seg = length_profile(s, params);
        if l_seg <= 0.0 {
            l_seg = 1.0e-6;
        }
        w_samples[i] = total_length / l_seg;
    }

    // 2) 사다리꼴 적분으로 W(s) = ∫ w(s) ds 누적
    w_cum[0] = 0.0;
    for i in 1..=n {
        let ds = s_samples[i] - s_samples[i - 1];
        let wavg = 0.5 * (w_samples[i] + w_samples[i - 1]);
        w_cum[i] = w_cum[i - 1] + wavg * ds;
    }

    let w_total = w_cum[n];
    if w_total <= 0.0 {
        return None;
    }

    // 3) 세그먼트 개수 N = round(W_total)
    let mut n_seg = w_total.round() as i32;
    if n_seg < 1 {
        n_seg = 1;
    }
    let n_seg = n_seg as usize;
    let point_count = n_seg + 1;

    let mut s_breaks = vec![0.0f64; point_count];
    s_breaks[0] = 0.0;
    s_breaks[point_count - 1] = 1.0;

    // 4) k=1..N-1 에 대해 W(s_k) = W_total * (k/N)를 만족하는 s_k 찾기 (이분법 on sample index)
    for k in 1..(point_count - 1) {
        let target = w_total * (k as f64) / (n_seg as f64);

        let mut lo: usize = 0;
        let mut hi: usize = n;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if w_cum[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        let idx = lo;
        if idx == 0 {
            s_breaks[k] = s_samples[0];
        } else {
            let w0 = w_cum[idx - 1];
            let w1 = w_cum[idx];
            let mut t = 0.0f64;
            if w1 > w0 {
                t = (target - w0) / (w1 - w0);
            }
            if t < 0.0 {
                t = 0.0;
            }
            if t > 1.0 {
                t = 1.0;
            }
            let s0 = s_samples[idx - 1];
            let s1 = s_samples[idx];
            s_breaks[k] = s0 + (s1 - s0) * t;
        }
    }

    // 5) arc_len_norm(u) = s_k 를 만족하는 u_k를 이분법으로 찾는다.
    let mut u_breaks = vec![0.0f64; point_count];
    for (k, u_ref) in u_breaks.iter_mut().enumerate() {
        let s_target = s_breaks[k];
        let mut u_lo = 0.0f64;
        let mut u_hi = 1.0f64;
        let mut u_mid = 0.0f64;

        for _ in 0..60 {
            u_mid = 0.5 * (u_lo + u_hi);
            let s_mid = arc_len_norm(u_mid);
            if s_mid < s_target {
                u_lo = u_mid;
            } else {
                u_hi = u_mid;
            }
        }

        *u_ref = u_mid;
    }

    // 양 끝점은 정확히 0, 1로 맞춰준다.
    if let Some(first) = u_breaks.first_mut() {
        *first = 0.0;
    }
    if let Some(last) = u_breaks.last_mut() {
        *last = 1.0;
    }

    // 6) 세그먼트 길이 계산 (옵션)
    let mut seg_lengths = Vec::with_capacity(n_seg);
    for i in 0..n_seg {
        let u0 = u_breaks[i];
        let u1 = u_breaks[i + 1];
        let s0 = arc_len_norm(u0);
        let s1 = arc_len_norm(u1);
        let seg_len = (s1 - s0) * total_length;
        seg_lengths.push(seg_len);
    }

    Some((u_breaks, seg_lengths))
}
```

---

# 8. 예제 설명

입력:

- len_start = 5
- len_mid = 10
- len_end = 3
- plateau_fraction = 0.6
- total_length = 100

결과:

- u 분포는 시작에서 작은 길이 → 점진 증가 → 중간 plateau 10 유지 → 끝에서 3 감소  
- 모든 세그먼트 길이 합 = 정확히 100

---

# 9. Input / Output 정리

## Input
```
arc_len_norm(u): 정규화된 곡선 길이 함수 0..1 → 0..1
total_length    : 전체 길이
params:
    len_start
    len_mid
    len_end
    plateau_fraction
    r_left
    r_right
```

## Output
```
Vec<f64> u_breaks      // u 분할점
Vec<f64> seg_lengths   // 각 세그먼트 길이
sum(seg_lengths) == total_length
```

---

# 10. 결론

이 알고리즘은 곡선의 시작·중간·끝에서 서로 다른 segment length profile을 유지하면서도  
전체 길이를 정확하게 보존하며, 세그먼트 수가 지나치게 많아지지 않도록 지수 전이를 통해 안정된 분할을 제공합니다.

Bezier / NURBS / CompositeCurve 등 어떤 곡선에도 적용 가능하며  
arc_len_norm(u)만 정의하면 즉시 활용할 수 있습니다.


## 11. 결과 가시화

```python
import numpy as np
import matplotlib.pyplot as plt

class LengthProfileParams:
    def __init__(self, ls=5.0, lm=10.0, le=3.0, plateau=0.6, rl=2.0, rr=2.0):
        self.len_start = ls
        self.len_mid = lm
        self.len_end = le
        self.plateau_fraction = plateau
        self.r_left = rl
        self.r_right = rr

def length_profile(s, p: LengthProfileParams):
    ls = p.len_start
    lm = p.len_mid
    le = p.len_end
    a = 0.5 * (1.0 - p.plateau_fraction)
    b = a

    if s <= 0.0:
        return ls
    if s >= 1.0:
        return le

    if s < a:
        x = s / a
        k = p.r_left
        if abs(k) < 1e-8:
            f = x
        else:
            ek = np.exp(k)
            ekx = np.exp(k * x)
            f = (ekx - 1.0) / (ek - 1.0)
        return ls + (lm - ls) * f
    elif s <= 1.0 - b:
        return lm
    else:
        x = (1.0 - s) / b
        k = p.r_right
        if abs(k) < 1e-8:
            f = x
        else:
            ek = np.exp(k)
            ekx = np.exp(k * x)
            f = (ekx - 1.0) / (ek - 1.0)
        return le + (lm - le) * f

def divide_curve_by_length_profile(total_length, params: LengthProfileParams):
    # 샘플: arc_len_norm(u) = u
    def arc_len_norm(u: float) -> float:
        u = max(0.0, min(1.0, u))
        return u

    samples = 1024
    s_samples = np.linspace(0.0, 1.0, samples + 1)
    w_samples = np.zeros_like(s_samples)

    for i, s in enumerate(s_samples):
        Lseg = length_profile(s, params)
        if Lseg <= 0.0:
            Lseg = 1e-6
        w_samples[i] = total_length / Lseg

    W_cum = np.zeros_like(s_samples)
    for i in range(1, samples + 1):
        ds = s_samples[i] - s_samples[i - 1]
        wavg = 0.5 * (w_samples[i] + w_samples[i - 1])
        W_cum[i] = W_cum[i - 1] + wavg * ds

    W_total = W_cum[-1]
    if W_total <= 0.0:
        return None, None, None

    N = int(round(W_total))
    if N < 1:
        N = 1

    point_count = N + 1
    s_breaks = np.zeros(point_count)
    s_breaks[0] = 0.0
    s_breaks[-1] = 1.0

    for k in range(1, point_count - 1):
        target = W_total * k / N
        lo, hi = 0, samples
        while lo < hi:
            mid = (lo + hi) // 2
            if W_cum[mid] < target:
                lo = mid + 1
            else:
                hi = mid
        idx = max(lo, 0)
        if idx == 0:
            s_breaks[k] = s_samples[0]
        else:
            W0, W1 = W_cum[idx - 1], W_cum[idx]
            t = 0.0
            if W1 > W0:
                t = (target - W0) / (W1 - W0)
            t = max(0.0, min(1.0, t))
            s0, s1 = s_samples[idx - 1], s_samples[idx]
            s_breaks[k] = s0 + (s1 - s0) * t

    u_breaks = s_breaks.copy()
    seg_lengths = (u_breaks[1:] - u_breaks[:-1]) * total_length
    seg_centers = 0.5 * (u_breaks[1:] + u_breaks[:-1])

    return u_breaks, seg_centers, seg_lengths

if __name__ == "__main__":
    params = LengthProfileParams(
        ls=5.0,
        lm=10.0,
        le=3.0,
        plateau=0.6,
        rl=2.0,
        rr=2.0
    )
    total_length = 100.0

    u_breaks, seg_centers, seg_lengths = divide_curve_by_length_profile(total_length, params)

    print("Segment count N =", len(u_breaks) - 1)
    print("Sum of segment lengths =", np.sum(seg_lengths))

    plt.figure()
    plt.plot(seg_centers, seg_lengths, marker='o')
    plt.xlabel('u (segment center)')
    plt.ylabel('segment length')
    plt.title('Segment length vs u (length profile 5 → 10 → 3, plateau=60%)')
    plt.grid(True)
    plt.tight_layout()
    plt.show()
```




![Bias Curve Divide](/image/bias_divide_result.png)

---

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::divide_curve_by_length_profile::{on_divide_curve_by_length_profile, LengthProfileParams};

    #[test]
    fn test_divide_curve_by_length_profile_line_100() {
        // 테스트용: arc_len_norm(u) = u (직선이고 0..1이 곧 정규화 길이)
        let arc_len_norm = |u: f64| -> f64 {
            if u <= 0.0 {
                0.0
            } else if u >= 1.0 {
                1.0
            } else {
                u
            }
        };

        let total_length = 100.0;
        let params = LengthProfileParams::new(5.0, 10.0, 3.0, 0.6, 2.0, 2.0);

        let (u_breaks, seg_lengths) =
            on_divide_curve_by_length_profile(arc_len_norm, total_length, &params)
                .expect("division failed");

        let n = seg_lengths.len();
        println!("Segment count = {n}");

        let mut sum = 0.0;
        for i in 0..n {
            sum += seg_lengths[i];
            let u0 = u_breaks[i];
            let u1 = u_breaks[i + 1];
            let uc = 0.5 * (u0 + u1);
            println!(
                "i={:2}, u0={:.6}, u1={:.6}, uc={:.6}, seg_len={:.6}",
                i, u0, u1, uc, seg_lengths[i]
            );
        }
        println!("sum lengths = {:.9} (target = {:.9})", sum, total_length);

        // 총합이 total_length 근처인지 확인 (수치 오차 허용)
        assert!((sum - total_length).abs() < 1.0e-6 * total_length);
        // u 분포가 0..1 사이인지 확인
        assert!((u_breaks[0] - 0.0).abs() < 1.0e-12);
        assert!((u_breaks.last().copied().unwrap_or(0.0) - 1.0).abs() < 1.0e-12);
    }
}

```
---
# 검증 문서
## 📐 Adaptive Curve Division by Length Profile 
– 수학적 해설 및 검증 문서
### 1. 목적
곡선의 전체 길이 L를 보존하면서, 시작·중간·끝 구간에서 서로 다른 세그먼트 길이 $\ell (s)$ 를 갖도록  
u-파라미터 분포 $\{ u_k\}$ 를 생성하는 알고리즘입니다.  
이 알고리즘은 다음 조건을 만족합니다:  

$$
\sum _{k=0}^{N-1}\mathrm{segment\\_ length_{\mathnormal{k}}}=L
$$

- $\ell (s)$ 는 구간별로 다르게 정의된 길이 프로파일
- $s(u)$ 는 정규화된 아크 길이 함수로, $s(0)=0$, $s(1)=1$, 단조 증가

### 2. 길이 프로파일 함수 $\ell (s)$
곡선 상의 정규화된 위치 $s\in [0,1]$ 에 대해 세그먼트 길이 함수 $\ell (s)$ 는 다음과 같이 정의됩니다:

$$
\ell(s) =
\begin{cases}
\ell_s + (\ell_m - \ell_s) F_{\text{L}}\bigl(s/a\bigr),
& 0 \le s < a,
\\
\ell_m,
& a \le s \le 1 - a,
\\
\ell_e + (\ell_m - \ell_e) F_{\text{R}}\bigl((1 - s)/a\bigr),
& 1 - a < s \le 1.
\end{cases}
$$

여기서:
- $\ell _s$: 시작 구간 세그먼트 길이
- $\ell _m$: 중간 plateau 구간 세그먼트 길이
- $\ell _e$: 끝 구간 세그먼트 길이
- $a=\frac{1-\mathrm{plateau\\_ fraction}}{2}$: 전이 구간의 길이
- $F_{\mathrm{left}}(x),F_{\mathrm{right}}(x)$: 지수 전이 함수
지수 전이 함수

$$
F(x; r) =
\begin{cases}
x, & |r| < \varepsilon, \\
\dfrac{e^{r x} - 1}{e^{r} - 1}, & \text{otherwise}.
\end{cases}
$$
- $r>0$: 점진적 전이
- $r\rightarrow 0$: 선형 전이

### 3. 세그먼트 개수 계산
세그먼트 밀도 함수 w(s)는 다음과 같이 정의됩니다:  

$$
w(s)=\frac{L}{\ell (s)}
$$

누적 함수 W(s)는:  

$$
W(s)=\int _0^sw(t) dt
$$

총 세그먼트 수 $N$ 은 다음과 같이 근사합니다:  

$$
N=\mathrm{round}(W(1))
$$

### 4. 역 매핑 (Inverse Mapping)
균등한 누적 분포 $\frac{k}{N}$ 에 대해 $s_k$ 를 찾습니다:  

$$
W(s_k)=\frac{k}{N}\cdot W(1)
$$


이때 $W(s)$ 는 수치 적분으로 구한 누적 배열이므로, 이분법으로 $s_k$ 를 찾습니다.

### 5. u-파라미터 역변환
정규화된 아크 길이 함수 $s(u)$ 에 대해:  

$$
s(u_k)=s_k
$$

이 조건을 만족하는 $u_k$ 를 이분법으로 찾습니다.
$s(u)$ 는 단조 증가 함수이므로 역함수가 존재하며, 수치적으로 안정적으로 계산 가능합니다.

### 6. 출력 결과
- $\{ u_k\} _{k=0}^N$: u-분할점 (0에서 1까지)
- $\mathrm{segment\\_ length_{\mathnormal{k}}}=(s_{k+1}-s_k)\cdot L$
- $\sum \mathrm{segment\\_ length_{\mathnormal{k}}}=L$ (수치 오차 허용 범위 내에서)

### 7. 수학적 정당성 요약
| 수학 조건 또는 성질             | 관련 함수 또는 개념             | 설명 또는 의미                                      |
|-------------------------------|-------------------------------|----------------------------------------------------|
| ℓ(s) > 0                      | w(s)                          | 세그먼트 길이가 항상 양수 → 밀도 함수 정의 가능       |
| s(u) 단조 증가                | uₖ                            | 역함수 존재 → u 분할점 계산 가능                     |
| W(s) 연속 증가                | sₖ                            | 누적 분포로부터 s 분할점 계산 가능                   |
| ∑ segₖ = L                   |                                | 전체 세그먼트 길이 합이 정확히 total_length와 일치   |


### 8. 예제 시각화 (설명)
- 시작 구간: 짧은 세그먼트 → 곡선의 세밀한 제어
- 중간 구간: 긴 세그먼트 → 효율적 분할
- 끝 구간: 다시 짧아짐 → 끝단 정밀도 확보
이러한 분포는 곡선의 시작/끝에서 더 많은 제어점을 필요로 하는 경우 (예: Bezier, NURBS)에서 매우 유용합니다.

---

## C++ 코드
```cpp
#pragma once

#include <functional>
#include <vector>
#include <optional>

struct LengthProfileParams {
    double len_start = 5.0;
    double len_mid = 10.0;
    double len_end = 3.0;
    double plateau_fraction = 0.6;
    double r_left = 2.0;
    double r_right = 2.0;

    LengthProfileParams() = default;
    LengthProfileParams(double ls, double lm, double le, double pf, double rl, double rr)
        : len_start(ls), len_mid(lm), len_end(le),
          plateau_fraction(pf), r_left(rl), r_right(rr) {}
};

double length_profile(double s, const LengthProfileParams& p);

std::optional<std::pair<std::vector<double>, std::vector<double>>> divide_curve_by_length_profile(
    const std::function<double(double)>& arc_len_norm,
    double total_length,
    const LengthProfileParams& params
);
```
```cpp
#include "main.h"
#include <algorithm>

double length_profile(double s, const LengthProfileParams& p) {
    double ls = p.len_start;
    double lm = p.len_mid;
    double le = p.len_end;

    double a = std::clamp(0.5 * (1.0 - p.plateau_fraction), 0.0, 0.5);

    if (s <= 0.0) return ls;
    if (s >= 1.0) return le;

    if (s < a) {
        double x = s / a;
        double k = p.r_left;
        double f = std::abs(k) < 1e-8 ? x : (std::exp(k * x) - 1.0) / (std::exp(k) - 1.0);
        return ls + (lm - ls) * f;
    } else if (s <= 1.0 - a) {
        return lm;
    } else {
        double x = (1.0 - s) / a;
        double k = p.r_right;
        double f = std::abs(k) < 1e-8 ? x : (std::exp(k * x) - 1.0) / (std::exp(k) - 1.0);
        return le + (lm - le) * f;
    }
}
```
```cpp
std::optional<std::pair<std::vector<double>, std::vector<double>>> divide_curve_by_length_profile(
    const std::function<double(double)>& arc_len_norm,
    double total_length,
    const LengthProfileParams& params
) {
    if (total_length <= 0.0) return std::nullopt;

    const int samples = 1024;
    std::vector<double> s_samples(samples + 1);
    std::vector<double> w_samples(samples + 1);
    std::vector<double> w_cum(samples + 1);

    for (int i = 0; i <= samples; ++i) {
        double s = static_cast<double>(i) / samples;
        s_samples[i] = s;
        double l_seg = std::max(length_profile(s, params), 1e-6);
        w_samples[i] = total_length / l_seg;
    }

    w_cum[0] = 0.0;
    for (int i = 1; i <= samples; ++i) {
        double ds = s_samples[i] - s_samples[i - 1];
        double wavg = 0.5 * (w_samples[i] + w_samples[i - 1]);
        w_cum[i] = w_cum[i - 1] + wavg * ds;
    }

    double w_total = w_cum[samples];
    if (w_total <= 0.0) return std::nullopt;

    int n_seg = std::max(1, static_cast<int>(std::round(w_total)));
    int point_count = n_seg + 1;

    std::vector<double> s_breaks(point_count);
    s_breaks[0] = 0.0;
    s_breaks[point_count - 1] = 1.0;

    for (int k = 1; k < point_count - 1; ++k) {
        double target = w_total * k / n_seg;
        int lo = 0, hi = samples;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (w_cum[mid] < target) lo = mid + 1;
            else hi = mid;
        }

        int idx = lo;
        double s0 = s_samples[std::max(0, idx - 1)];
        double s1 = s_samples[idx];
        double w0 = w_cum[std::max(0, idx - 1)];
        double w1 = w_cum[idx];
        double t = (w1 > w0) ? std::clamp((target - w0) / (w1 - w0), 0.0, 1.0) : 0.0;
        s_breaks[k] = s0 + (s1 - s0) * t;
    }

    std::vector<double> u_breaks(point_count);
    for (int k = 0; k < point_count; ++k) {
        double s_target = s_breaks[k];
        double u_lo = 0.0, u_hi = 1.0, u_mid = 0.0;
        for (int iter = 0; iter < 60; ++iter) {
            u_mid = 0.5 * (u_lo + u_hi);
            double s_mid = arc_len_norm(u_mid);
            if (s_mid < s_target) u_lo = u_mid;
            else u_hi = u_mid;
        }
        u_breaks[k] = u_mid;
    }

    u_breaks.front() = 0.0;
    u_breaks.back() = 1.0;

    std::vector<double> seg_lengths;
    seg_lengths.reserve(n_seg);
    for (int i = 0; i < n_seg; ++i) {
        double s0 = arc_len_norm(u_breaks[i]);
        double s1 = arc_len_norm(u_breaks[i + 1]);
        seg_lengths.push_back((s1 - s0) * total_length);
    }

    return std::make_pair(u_breaks, seg_lengths);
}
```
```cpp
bool divide_curve_by_length_profile(
    const std::function<double(double)>& arc_len_norm,
    double total_length,
    const LengthProfileParams& params,
    std::vector<double>& out_u,
    std::vector<double>* out_seg_lengths /*= nullptr*/
)
{
    out_u.clear();
    if (total_length <= 0.0)
        return false;

    const int samples = 1024;
    std::vector<double> s_samples(samples + 1);
    std::vector<double> w_samples(samples + 1);
    std::vector<double> W_cum(samples + 1);

    for (int i = 0; i <= samples; ++i)
    {
        double s = static_cast<double>(i) / static_cast<double>(samples);
        s_samples[i] = s;

        double Lseg = length_profile(s, params); // 원하는 세그먼트 길이
        if (Lseg <= 0.0)
            Lseg = 1.0e-6; // 방어

        w_samples[i] = total_length / Lseg;      // 단위 s 당 세그먼트 개수
    }

    // 2) 사다리꼴 적분으로 누적 W(s) 계산
    W_cum[0] = 0.0;
    for (int i = 1; i <= samples; ++i)
    {
        double ds   = s_samples[i] - s_samples[i - 1];
        double wavg = 0.5 * (w_samples[i] + w_samples[i - 1]);
        W_cum[i]    = W_cum[i - 1] + wavg * ds;
    }
    double W_total = W_cum[samples];
    if (W_total <= 0.0)
        return false;

    // 3) 예상 세그먼트 개수 N = round(W_total)
    int N = static_cast<int>(std::round(W_total));
    if (N < 1) N = 1;

    const int point_count = N + 1;
    std::vector<double> s_breaks(point_count);

    s_breaks[0]             = 0.0;
    s_breaks[point_count-1] = 1.0;

    // 4) k=1..N-1 에 대해 W(s_k) = W_total * (k/N) 을 만족하는 s_k 찾기
    for (int k = 1; k < point_count - 1; ++k)
    {
        double target = W_total * static_cast<double>(k) / static_cast<double>(N);

        int lo = 0;
        int hi = samples;
        while (lo < hi)
        {
            int mid = (lo + hi) / 2;
            if (W_cum[mid] < target)
                lo = mid + 1;
            else
                hi = mid;
        }

        int idx = (lo > 0) ? lo : 0;
        if (idx == 0)
        {
            s_breaks[k] = s_samples[0];
        }
        else
        {
            double W0 = W_cum[idx - 1];
            double W1 = W_cum[idx];
            double t  = 0.0;
            if (W1 > W0)
                t = (target - W0) / (W1 - W0);

            if (t < 0.0) t = 0.0;
            if (t > 1.0) t = 1.0;

            double s0 = s_samples[idx - 1];
            double s1 = s_samples[idx];
            s_breaks[k] = s0 + (s1 - s0) * t;
        }
    }

    // 5) 각 s_k 에 대해 arc_len_norm(u) = s_k 를 이분법으로 풀어서 u_k 찾기
    out_u.resize(point_count);
    for (int k = 0; k < point_count; ++k)
    {
        double s_target = s_breaks[k];

        double u_lo = 0.0;
        double u_hi = 1.0;
        double u_mid = 0.0;

        for (int it = 0; it < 60; ++it)
        {
            u_mid = 0.5 * (u_lo + u_hi);
            double s_mid = arc_len_norm(u_mid);

            if (s_mid < s_target)
                u_lo = u_mid;
            else
                u_hi = u_mid;
        }

        out_u[k] = u_mid;
    }

    // 정확하게 끝단 고정
    out_u.front() = 0.0;
    out_u.back()  = 1.0;

    // 옵션: 세그먼트 길이 계산
    if (out_seg_lengths)
    {
        out_seg_lengths->clear();
        out_seg_lengths->reserve(N);
        for (int i = 0; i < N; ++i)
        {
            double u0 = out_u[i];
            double u1 = out_u[i+1];

            double s0 = arc_len_norm(u0);
            double s1 = arc_len_norm(u1);
            double seg_len = (s1 - s0) * total_length; // 실제 길이

            out_seg_lengths->push_back(seg_len);
        }
    }
    return true;
}
```
```cpp
int main()
{
    // 예시: arc_len_norm(u) = u (직선이고 0..1이 정규화 길이)
    auto arc_len_norm = [](double u) -> double {
        if (u <= 0.0) return 0.0;
        if (u >= 1.0) return 1.0;
        return u;
    };

    double total_length = 100.0;
    LengthProfileParams params(
        5.0,   // len_start
        10.0,  // len_mid
        3.0,   // len_end
        0.6,   // plateau_fraction
        2.0,   // r_left
        2.0    // r_right
    );

    std::vector<double> u_breaks;
    std::vector<double> seg_lengths;

    bool ok = divide_curve_by_length_profile(
        arc_len_norm,
        total_length,
        params,
        u_breaks,
        &seg_lengths
    );

    if (!ok)
    {
        std::printf("divide_curve_by_length_profile failed.\n");
        return 1;
    }

    int N = static_cast<int>(u_breaks.size()) - 1;
    std::printf("Segment count N = %d\n", N);

    double sum_len = 0.0;
    for (int i = 0; i < N; ++i)
    {
        double u0 = u_breaks[i];
        double u1 = u_breaks[i+1];
        double L  = seg_lengths[i];
        sum_len += L;

        double uc = 0.5 * (u0 + u1);
        std::printf("i=%2d: u0=%.6f, u1=%.6f, uc=%.6f, seg_len=%.6f\n",
                    i, u0, u1, uc, L);
    }
    std::printf("Sum of segment lengths = %.6f (should be ~ %.6f)\n",
                sum_len, total_length);

    return 0;
}

```

---




