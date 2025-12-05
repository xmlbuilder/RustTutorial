# 1D Metric Field and Normalized Parameter Division (`divide.md`)

This document describes the algorithm and formulas used in the C++ function:

```cpp
bool Build1DMetricFieldAutoN(
    double L,
    double m_start0,
    double m_end0,
    double m_max0,
    std::vector<double>& metrics,
    std::vector<double>& u,
    double ramp_fraction = 0.25,
    int    max_segments  = 2000,
    double tol_sum       = 1e-9
);
```

The goal is to construct a **1D sizing field** (`metrics`) and corresponding **normalized parameters** (`u`)  
that can be used to subdivide a curve in a reasonable way.

---

## 1. Problem Setup

- We consider a 1D interval (for example, a curve projected to a parameter range) with:
  - Total length: `L > 0`
  - Desired metric (segment length) at the left end: `m_start0 >= 0`
  - Desired metric at the right end: `m_end0 >= 0`
  - Maximum allowed metric anywhere: `m_max0 > 0`

- We want to produce:
  - `metrics[i]` for `i = 0 .. N-1` (N is chosen automatically)
    - Each `metrics[i]` is a segment length
    - All `metrics[i] > 0`
    - All `metrics[i] <= m_max` (after clamping)
    - The sum of all metrics is approximately `L`:
      - `sum_{i=0..N-1} metrics[i] ~= L`
    - The first and last metrics approximate desired end values:
      - `metrics[0] ~ m_start0`
      - `metrics[N-1] ~ m_end0`

  - Normalized parameters `u[j]` for `j = 0 .. N`:
    - `u[0] = 0`
    - `u[N] = 1`
    - `u[j+1] - u[j]` is proportional to `metrics[j] / L`
    - These `u` values define a subdivision of the unit interval `[0,1]`
      that can be mapped onto the actual curve.

- Internally we clamp the input metrics:

```text
eps     = 1e-9
m_max   = max(m_max0, eps)
m_start = clamp(m_start0, eps, m_max)
m_end   = clamp(m_end0,   eps, m_max)
```

- where:

```text
clamp(x, a, b) = min( max(x, a), b )
```

- The parameter `ramp_fraction` controls how much of the parametric domain is used for "ramp" regions:
  
  - On the left:  `u in [0, ramp_fraction]`
  - On the right: `u in [1 - ramp_fraction, 1]`
  - The middle region is a plateau near `m_max`.

- We clamp:

```text
ramp_fraction = clamp(ramp_fraction, 0.0, 0.5)
```

---

## 2. Very Short Length Case

- If the total length `L` is too short to keep both end metrics as they are, we handle a special case:

```text
if L <= m_start + m_end + eps:
    // cannot keep both end metrics as requested
```

### 2.1. Scaling of End Metrics

- We scale the two end metrics proportionally so that their sum is `L`:

```text
s  = L / (m_start + m_end)
h0 = m_start * s
h1 = m_end   * s
```

- We then clamp each to `m_max` (and minimum `eps`), and if the sum deviates from `L`,
- we apply a second scale to correct:

```text
h0 = clamp(h0, eps, m_max)
h1 = clamp(h1, eps, m_max)

sum = h0 + h1
if |sum - L| > tol_sum and sum > eps:
    s2 = L / sum
    h0 *= s2
    h1 *= s2
```

- Finally we set:

```text
metrics[0] = h0
metrics[1] = h1
N = 2
```

### 2.2. Normalized Parameters in Short Case

We build `u` using cumulative sums:

```text
u[0] = 0
u[1] = metrics[0] / L
u[2] = (metrics[0] + metrics[1]) / L
u[N] = 1   // enforced
```

- This completes the very short length case.

---

## 3. General Case

- For the general case we assume:

```text
L > m_start + m_end + eps
```

- Thus there is room for internal segments.

- Define:

```text
sum_end  = m_start + m_end
S_target = L - sum_end   // desired sum of all internal metrics
```

- If `S_target < eps`, we again fall back to the 2-segment case.

---

## 4. Minimum Internal Segment Count

- The maximum metric allowed anywhere is `m_max`. If there are `K` internal segments, then in the best case:

```text
K * m_max >= S_target
```

- Therefore a minimum internal segment count is:

```text
K_min = ceil( S_target / m_max )
K_min = max(K_min, 1)
```

- Total segment count is:

```text
N_min = 2 + K_min
N     = N_min
```

We ensure:

```text
N >= 2
max_segments = max(max_segments, N)
```

---

## 5. Target Profile `d[i]` (Ramp + Plateau)

We construct a target profile `d[i]` for `i = 0..N-1`:

- `d[0] = m_start`
- `d[N-1] = m_end`
- Internal `d[i]` (for `i = 1..N-2`) are built by a ramp function in the parametric domain:

Define a parametric coordinate:

```text
u_i = i / (N - 1),   i = 0..N-1
```

Let:

```text
left_end  = ramp_fraction
right_beg = 1.0 - ramp_fraction
```

Then:

```text
if u_i <= left_end:
    // left ramp from m_start to m_max
    s = u_i / left_end      // 0..1
    t = m_start + s * (m_max - m_start)
else if u_i >= right_beg:
    // right ramp from m_max to m_end
    s = (1.0 - u_i) / ramp_fraction  // 0..1
    t = m_end + s * (m_max - m_end)
else:
    // plateau
    t = m_max
```

- We clamp:

```text
d[i] = clamp(t, eps, m_max)
```

- We then compute the sum:

```text
sumD = sum_{i=0..N-1} d[i]
```

---

## 6. Increasing N Until `sumD >= L`

- We want to be able to **scale internal elements down** (by a factor <= 1) to reach the target length `L` without exceeding `m_max`.
- For this, we need:

```text
sumD >= L
```

- If `sumD < L - tol_sum`, we increase `N` by 1 and rebuild the profile `d`:

```text
while true:
    build_target_profile(N, d, sumD) // using the rules in section 5

    if sumD + tol_sum >= L:
        break   // good enough
    N += 1

    if N > max_segments:
        return false // cannot satisfy constraints with given max_segments
```

- After this loop:

- `N` is chosen
- `d[0] = m_start`, `d[N-1] = m_end`
- `sumD >= L` (within tolerance)

---

## 7. Scaling the Internal Part to Match `L`

- We then form the final `metrics[i]`:

```text
metrics[0]     = m_start
metrics[N - 1] = m_end
```

- Let:

```text
sumD_end   = d[0] + d[N-1]
sumD_inner = sumD - sumD_end
S_target   = L - sum_end  // from section 3
```

### 7.1. Internal Sum is Almost Zero

- If:

```text
sumD_inner <= eps
```

- we use a uniform internal metric:

```text
if N > 2:
    m_inner = S_target / (N - 2)
    m_inner = clamp(m_inner, eps, m_max)
    metrics[i] = m_inner  for i = 1..N-2
```

### 7.2. General Internal Scaling

- Otherwise, we use a scale factor:

```text
scale = S_target / sumD_inner
```

- Since `sumD >= L` and `sumD_end` is fixed, we expect `scale <= 1`.  
- If due to rounding:

```text
scale > 1 + small_tolerance
```

- we clamp:

```text
scale = 1
```

Then:

```text
for i in 1..N-2:
    metrics[i] = d[i] * scale
    metrics[i] = clamp(metrics[i], eps, m_max)
```

---

## 8. Final Sum Correction

- We compute:

```text
sum_final = sum_{i=0..N-1} metrics[i]
diff      = L - sum_final
```

If the absolute difference is larger than tolerance and `N > 2`:

```text
if |diff| > tol_sum and N > 2:
    per = diff / (N - 2)
    for i in 1..N-2:
        metrics[i] += per
        metrics[i] = clamp(metrics[i], eps, m_max)
```

- This distributes the residual error uniformly across the internal segments while keeping them within the `[eps, m_max]` range.

---

## 9. Building Normalized Parameter Array `u`

- Once `metrics[i]` are finalized, we build a normalized parametric partition `u[j]`:

```text
u has size N+1

cum = 0
u[0] = 0
for i in 0..N-1:
    cum += metrics[i]
    u[i+1] = cum / L
```

- Finally we enforce:

```text
u[0] = 0
u[N] = 1
```

- The u-array is thus a normalized cumulative length:
  - `u[0] = 0`
  - `u[1] = metrics[0] / L`
  - ...
  - `u[i+1] - u[i] = metrics[i] / L`
  - `u[N] = 1`

These `u` values can later be used as approximate subdivision parameters for a curve.  
If the actual curve length is used, you can re-adjust u (via arc-length reparameterization)  
while keeping the **count and approximate distribution** provided by this algorithm.

---

## 10. Summary of Constraints

- The algorithm guarantees:

- 1. `metrics[i] > 0` for all i
- 2. `metrics[i] <= m_max` for all i
- 3. `metrics[0]` and `metrics[N-1]` stay as close as possible to `m_start0` and `m_end0`  
   (exact in the general case, proportionally scaled in very short cases)
- 4. The total sum of metrics approximates `L`:

   ```text
   sum_{i=0..N-1} metrics[i] ≈ L
   ```

   with the residual error pushed inside tolerance `tol_sum`.

- 5. The number of segments `N` is **automatically chosen** based on the relation between `L`, `m_start`, `m_end`, and `m_max`.
- 6. The normalized parameters `u` form a valid partition of `[0,1]` aligned with the metric distribution.

- You can now treat `metrics` as a 1D sizing field and `u` as the corresponding subdivision parameters,  
  and combine this with an existing arc-length based inverse mapping to refine the curve subdivision if needed.

### Subdivide Image
![Subdivide Result](/image/subdivide_result.png)

---

## 소스 코드
```rust
/// Clamp helper: clamp x into [min_val, max_val].
#[inline]
fn clamp(x: f64, min_val: f64, max_val: f64) -> f64 {
    if x < min_val {
        min_val
    } else if x > max_val {
        max_val
    } else {
        x
    }
}

/// Build a 1D metric (segment length) field and corresponding normalized
/// parameter positions `u` in [0,1].
///
/// Inputs:
///   L           : total length (> 0)
///   m_start0    : desired metric at the left end (>= 0)
///   m_end0      : desired metric at the right end (>= 0)
///   m_max0      : maximum allowed metric (> 0)
///
/// Outputs (cleared and filled inside):
///   metrics : segment lengths h[i], each > 0 and <= m_max, sum ~ L
///   u       : normalized positions in [0,1] of length N+1,
///             u[0] = 0, u[N] = 1, and u[i+1] - u[i] ~ metrics[i] / L
///
/// Parameters:
///   ramp_fraction : 0..0.5. Fraction of param domain used for ramps near ends.
///   max_segments  : safety cap for maximum number of segments.
///   tol_sum       : tolerance for sum(metrics) ~ L.
///
/// Return:
///   true  : success
///   false : failed to build a valid distribution.
pub fn build_1d_metric_field_auto_n(
    L: f64,
    m_start0: f64,
    m_end0: f64,
    m_max0: f64,
    metrics: &mut Vec<f64>,
    u: &mut Vec<f64>,
    ramp_fraction: f64,
    max_segments: usize,
    tol_sum: f64,
) -> bool {
    metrics.clear();
    u.clear();

    let eps = 1e-9_f64;
    if !(L > 0.0) || !(m_max0 > 0.0) {
        return false;
    }

    // Basic clamps
    let mut m_max = m_max0.max(eps);
    let mut m_start = m_start0.max(eps);
    let mut m_end = m_end0.max(eps);

    m_start = clamp(m_start, eps, m_max);
    m_end = clamp(m_end, eps, m_max);
    m_max = m_max.max(eps); // already ensured

    let mut ramp_fraction = clamp(ramp_fraction, 0.0, 0.5);

    // ----------------------------------------------------------------
    // 1) Very short length case:
    //    L <= m_start + m_end
    // ----------------------------------------------------------------
    if L <= (m_start + m_end) + eps {
        let s = L / (m_start + m_end);
        if s <= 0.0 {
            // Degenerate: use uniform split
            metrics.resize(2, 0.0);
            metrics[0] = 0.5 * L;
            metrics[1] = 0.5 * L;
        } else {
            let mut h0 = m_start * s;
            let mut h1 = m_end * s;

            // Clamp by m_max and re-adjust sum if needed
            h0 = clamp(h0, eps, m_max);
            h1 = clamp(h1, eps, m_max);

            let mut sum = h0 + h1;
            if (sum - L).abs() > tol_sum && sum > eps {
                let s2 = L / sum;
                h0 *= s2;
                h1 *= s2;
            }

            metrics.resize(2, 0.0);
            metrics[0] = h0;
            metrics[1] = h1;
        }

        // Build normalized u: size N+1 (N=2 -> size 3)
        u.resize(3, 0.0);
        let mut cum = 0.0;
        u[0] = 0.0;
        cum += metrics[0];
        u[1] = cum / L;
        cum += metrics[1];
        u[2] = cum / L;
        u[0] = 0.0;
        u[2] = 1.0;

        return true;
    }

    // ----------------------------------------------------------------
    // 2) General case
    // ----------------------------------------------------------------
    let sum_end = m_start + m_end;
    let mut s_target = L - sum_end; // internal sum target

    if s_target < eps {
        // No real internal region: just 2 segments
        metrics.resize(2, 0.0);
        metrics[0] = m_start;
        metrics[1] = (L - m_start).max(eps);
        metrics[1] = clamp(metrics[1], eps, m_max);

        let mut sum = metrics[0] + metrics[1];
        if (sum - L).abs() > tol_sum && sum > eps {
            let s = L / sum;
            metrics[0] *= s;
            metrics[1] *= s;
            sum = metrics[0] + metrics[1];
        }

        // Build u
        u.resize(3, 0.0);
        let mut cum = 0.0;
        u[0] = 0.0;
        cum += metrics[0];
        u[1] = cum / L;
        cum += metrics[1];
        u[2] = cum / L;
        u[0] = 0.0;
        u[2] = 1.0;

        return true;
    }

    // Internal segment count K_min such that K_min * m_max >= s_target
    let mut k_min = (s_target / m_max).ceil() as usize;
    if k_min < 1 {
        k_min = 1;
    }

    // N = 2 (ends) + K (internal)
    let mut n = 2 + k_min;
    if n < 2 {
        n = 2;
    }
    let mut max_segments = max_segments.max(n);

    // target profile
    let mut d: Vec<f64> = Vec::with_capacity(max_segments);
    let mut sum_d: f64 = 0.0;

    // Closure to build target profile (ramp + plateau)
    let mut build_target_profile = |n_local: usize,
                                    out_d: &mut Vec<f64>,
                                    out_sum: &mut f64| {
        out_d.clear();
        out_d.resize(n_local, 0.0);

        out_d[0] = m_start;
        out_d[n_local - 1] = m_end;

        let left_end = ramp_fraction;
        let right_beg = 1.0 - ramp_fraction;

        for i in 1..(n_local - 1) {
            let uu = i as f64 / (n_local as f64 - 1.0);
            let mut t = m_max;

            if ramp_fraction > 0.0 {
                if uu <= left_end {
                    // left ramp
                    let s = uu / left_end; // 0..1
                    t = m_start + s * (m_max - m_start);
                } else if uu >= right_beg {
                    // right ramp
                    let s = (1.0 - uu) / ramp_fraction; // 0..1
                    t = m_end + s * (m_max - m_end);
                } else {
                    // plateau
                    t = m_max;
                }
            } else {
                t = m_max;
            }

            t = clamp(t, eps, m_max);
            out_d[i] = t;
        }

        *out_sum = out_d.iter().sum::<f64>();
    };

    // Increase n until sum_d >= L (so we can scale down internal metrics)
    loop {
        if n > max_segments {
            return false;
        }

        build_target_profile(n, &mut d, &mut sum_d);

        if sum_d + tol_sum >= L {
            break;
        }

        n += 1;
        max_segments = max_segments.max(n);
    }

    // Now sum_d >= L and d[0] = m_start, d[n-1] = m_end
    let sum_d_end = d[0] + d[n - 1];
    let sum_d_inner = sum_d - sum_d_end;

    metrics.clear();
    metrics.resize(n, 0.0);
    metrics[0] = m_start;
    metrics[n - 1] = m_end;

    if sum_d_inner <= eps {
        // Internal region is almost zero in the target profile.
        if n > 2 {
            let mut m_inner = s_target / ((n - 2) as f64);
            m_inner = clamp(m_inner, eps, m_max);
            for i in 1..(n - 1) {
                metrics[i] = m_inner;
            }
        }
    } else {
        // Scale factor for internal metrics
        let mut scale = s_target / sum_d_inner;
        if scale > 1.0 + 1e-6 {
            scale = 1.0;
        }

        for i in 1..(n - 1) {
            metrics[i] = d[i] * scale;
            metrics[i] = clamp(metrics[i], eps, m_max);
        }
    }

    // Final sum correction (small residual error)
    let mut sum_final = metrics.iter().sum::<f64>();
    let diff = L - sum_final;

    if diff.abs() > tol_sum && n > 2 {
        let per = diff / ((n - 2) as f64);
        for i in 1..(n - 1) {
            metrics[i] += per;
            metrics[i] = clamp(metrics[i], eps, m_max);
        }
        sum_final = metrics.iter().sum::<f64>();
        let _ = sum_final; // for debugging
    }

    // Build normalized u
    u.clear();
    u.resize(n + 1, 0.0);
    let mut cum = 0.0;
    u[0] = 0.0;
    for i in 0..n {
        cum += metrics[i];
        u[i + 1] = cum / L;
    }
    u[0] = 0.0;
    u[n] = 1.0;

    true
}

/// Convenience wrapper with typical default parameters.
/// ramp_fraction = 0.25, max_segments = 2000, tol_sum = 1e-9.
pub fn on_build_1d_metric_field_auto_n_default(
    L: f64,
    m_start0: f64,
    m_end0: f64,
    m_max0: f64,
    metrics: &mut Vec<f64>,
    u: &mut Vec<f64>,
) -> bool {
    build_1d_metric_field_auto_n(
        L,
        m_start0,
        m_end0,
        m_max0,
        metrics,
        u,
        0.25,
        2000,
        1e-9,
    )
}
```
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::mesh_tools::on_build_1d_metric_field_auto_n_default;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn test_basic_long_curve() {
        let L = 10.0;
        let m_start = 0.1;
        let m_end = 0.1;
        let m_max = 1.0;

        let mut h = Vec::new();
        let mut u = Vec::new();

        let ok = on_build_1d_metric_field_auto_n_default(L, m_start, m_end, m_max, &mut h, &mut u);
        assert!(ok);
        assert!(h.len() >= 2);
        assert_eq!(u.len(), h.len() + 1);

        let sum: f64 = h.iter().sum();
        assert!(approx_eq(sum, L, 1e-6));

        // ends
        assert!(h[0] <= m_max + 1e-9);
        assert!(h[h.len() - 1] <= m_max + 1e-9);

        // all segments <= m_max
        for &v in &h {
            assert!(v > 0.0);
            assert!(v <= m_max + 1e-9);
        }

        // u monotone and in [0,1]
        for i in 0..u.len() - 1 {
            assert!(u[i] <= u[i + 1] + 1e-12);
        }
        assert!(approx_eq(u[0], 0.0, 1e-12));
        assert!(approx_eq(u[u.len() - 1], 1.0, 1e-12));

        println!("{:?}", h);
        println!("{:?}", u);
    }

    #[test]
    fn test_very_short_curve() {
        let L = 0.2;
        let m_start = 0.1;
        let m_end = 0.5;
        let m_max = 1.0;

        let mut h = Vec::new();
        let mut u = Vec::new();

        let ok = on_build_1d_metric_field_auto_n_default(L, m_start, m_end, m_max, &mut h, &mut u);
        assert!(ok);
        assert_eq!(h.len(), 2);
        let sum: f64 = h.iter().sum();
        assert!(approx_eq(sum, L, 1e-6));

        println!("{:?}", h);
        println!("{:?}", u);
    }
}
```
```
-------------------------------------------
RESULT (C++): N = 14 segments
-------------------------------------------
metrics:
i=0  h=0.100000000000
i=1  h=0.372248062016
i=2  h=0.645736434109
i=3  h=0.919224806202
i=4  h=0.987596899225
i=5  h=0.987596899225
i=6  h=0.987596899225
i=7  h=0.987596899225
i=8  h=0.987596899225
i=9  h=0.987596899225
i=10  h=0.919224806202
i=11  h=0.645736434109
i=12  h=0.372248062016
i=13  h=0.100000000000

sum(metrics) = 10.000000000000
target L     = 10.000000000000

u (normalized positions):
u[0] = 0.000000000000
u[1] = 0.010000000000
u[2] = 0.047224806202
u[3] = 0.111798449612
u[4] = 0.203720930233
u[5] = 0.302480620155
u[6] = 0.401240310078
u[7] = 0.500000000000
u[8] = 0.598759689922
u[9] = 0.697519379845
u[10] = 0.796279069767
u[11] = 0.888201550388
u[12] = 0.952775193798
u[13] = 0.990000000000
u[14] = 1.000000000000
```

