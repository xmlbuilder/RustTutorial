# 1D Metric Division Algorithm 
- Mathematical Documentation
  - This document summarizes the mathematical model and equations used to   
    generate a **1D adaptive metric field** for mesh subdivision along a curve.  
  - It is designed so that copy-paste does **not** break formulas in GitHub Markdown.


## 1. Problem Definition

- Given:
  - Total curve length: `L`
  - Metric size to satisfy at the start: `m_start`
  - Metric size to satisfy at the end: `m_end`
  - Maximum allowed metric size in mid-domain: `m_max`
  - Ratio of exponential ramp region: `ramp_fraction ∈ [0, 0.5]`

- We want to compute:

  - A sequence of metric sizes: `h[i]` (i = 0..N-1)
  - Total sum: `Σ h[i] ≈ L`
  - Normalized parameter positions:  
    `u[0] = 0`,  
    `u[k] = (h[0] + ... + h[k-1]) / L`,  
    `u[N] = 1`.

- Goal:
  - 1. Enforce end constraints `h[0] = m_start`, `h[N-1] = m_end`
  - 2. Build a **smooth exponential-like ramp** toward `m_max`
  - 3. Maintain plateau at `m_max`
  - 4. Ensure total matches the curve length:  
     `Σ h[i] = L`
  - 5. Automatically determine **N** (number of segments)

---

## 2. Exponential Ramp Formulation
- Let:
  - `t ∈ [0, 1]` be normalized position inside ramp
  - Left ramp domain:  
    `0 ≤ s ≤ L_left = ramp_fraction * L`
  - Right ramp domain:  
    `(1 − ramp_fraction) * L ≤ s ≤ L`

- We map position along the curve `s` to ramp parameter:

```
Left ramp:
    t = s / L_left

Right ramp:
    t = (L − s) / L_right
```

- We use the geometric ramp formula:

```
h(t) = h_min * (ratio^t)
```

- Where:

```
ratio = h_max / h_min
```

- For left side:

```
h_left(s) = m_start * ( (m_max / m_start)^t )
```

- For right side:

```
h_right(s) = m_end * ( (m_max / m_end)^t )
```

- This creates a true **exponential growth/decay**.

---

## 3. Plateau Region

For positions between both ramp zones:

```
h(s) = m_max
```

This ensures uniform cell sizes in the central region.

---

## 4. Computing Number of Segments N

- We first compute a **target curve** `d[i]` of metric sizes following:
  - geometric ramp → plateau → geometric ramp

- Compute:

```
sum_d = Σ d[i]
```

- If:

```
sum_d < L
```

- We increase `N` until:

```
sum_d >= L
```

- This guarantees enough segments to fill the curve.


## 5. Scaling Interior Metrics to Match the Length

- We separate:

```
sum_end = d[0] + d[N-1]
sum_inner = sum_d - sum_end
target_inner = L - sum_end
```

- Scale only interior part:

```
scale = target_inner / sum_inner
```

- Then:

```
h[i] = d[i] * scale    for i = 1..N-2
h[0] = m_start
h[N-1] = m_end
```

- This preserves end constraints and the ramp **shape**, while ensuring:

```
Σ h[i] = L
```


## 6. Final Correction Pass

- Residual errors due to floating point rounding:

```
diff = L - Σ h[i]
```

- We distribute uniformly:

```
h[i] += diff / (N - 2)   for i = 1..N-2
```

- Then clamp to `[ε, m_max]`.

---

## 7. Normalized Parameter Output u[i]

- We compute:

```
u[0] = 0
u[k] = (h[0] + ... + h[k-1]) / L
u[N] = 1
```

These `u` values are used for actual curve subdivision.

---

## 8. Summary of Entire Algorithm (Pseudo-code)


- 1. Clamp input metrics
- 2. If L < m_start + m_end:
  - Special 2-point solution
- 3. Compute minimal segments K_min ≈ ceil( (L - (m_start + m_end)) / m_max )
- 4. For N = K_min + 2 up to max_segments:
  - Build exponential-ramp target profile d[i]
  - If Σ d[i] ≥ L: break
- 5. Scale interior d[i] → h[i] to match length L
- 6. Distribute small residual difference over interior
- 7. Compute normalized u[i]
- 8. Output h[] and u[]

## 9. Advantages of This Algorithm

- ✔ Always respects boundary metrics  
- ✔ Smooth exponential growth/decay-shaped size transitions  
- ✔ Automatic segmentation count  
- ✔ Plateau region for stable mesh quality  
- ✔ Works for very short or very long curves  
- ✔ C++ and Rust implementations match bit-level behavior  


## 10. Example Output

- For the input:

```
L = 10
m_start = 0.1
m_end   = 0.1
m_max   = 1.0
ramp_fraction = 0.25
```

- A typical output:

```
h = [0.1, 0.1756, 0.3244, 0.5995, 0.9501, ... 0.5995, 0.3244, 0.1756, 0.1]
cum_x = [0.1, 0.2756, 0.6000, 1.1995, ... 10.0]
u = cum_x / 10
```

## 소스 코드
```rust
pub fn on_build_1d_metric_field_auto_n(
    l: f64,
    m_start0: f64,
    m_end0: f64,
    m_max0: f64,
    ramp_fraction: f64,
    max_segments: usize,
    tol_sum: f64,
    metrics: &mut Vec<f64>,
    u: &mut Vec<f64>,
) -> bool {
    fn clamp_val(x: f64, a: f64, b: f64) -> f64 {
        if x < a {
            a
        } else if x > b {
            b
        } else {
            x
        }
    }

    metrics.clear();
    u.clear();

    let eps = 1.0e-9;

    if !(l > 0.0) || !(m_max0 > 0.0) {
        return false;
    }

    let mut m_max = m_max0;
    if m_max < eps {
        m_max = eps;
    }

    let mut m_start = clamp_val(m_start0, eps, m_max);
    let mut m_end = clamp_val(m_end0, eps, m_max);

    let mut ramp_fraction = ramp_fraction;
    if ramp_fraction < 0.0 {
        ramp_fraction = 0.0;
    } else if ramp_fraction > 0.5 {
        ramp_fraction = 0.5;
    }

    // --------------------------------------------------------
    // Case 1: very short length (cannot keep both end metrics)
    // --------------------------------------------------------
    if l <= m_start + m_end + eps {
        let s = l / (m_start + m_end);
        if s <= 0.0 {
            metrics.resize(2, 0.0);
            metrics[0] = 0.5 * l;
            metrics[1] = 0.5 * l;
        } else {
            let mut h0 = m_start * s;
            let mut h1 = m_end * s;

            h0 = clamp_val(h0, eps, m_max);
            h1 = clamp_val(h1, eps, m_max);

            let sum = h0 + h1;
            if (sum - l).abs() > tol_sum && sum > eps {
                let s2 = l / sum;
                h0 *= s2;
                h1 *= s2;
            }

            metrics.clear();
            metrics.push(h0);
            metrics.push(h1);
        }

        // build normalized u (size N+1 = 3)
        u.resize(3, 0.0);
        let mut cum = 0.0;
        u[0] = 0.0;
        cum += metrics[0];
        u[1] = cum / l;
        cum += metrics[1];
        u[2] = cum / l;
        u[0] = 0.0;
        u[2] = 1.0;
        return true;
    }

    // --------------------------------------------------------
    // General case
    // --------------------------------------------------------
    let sum_end = m_start + m_end;
    let s_target = l - sum_end; // sum of internal metrics

    if s_target < eps {
        // basically two segments
        metrics.clear();
        metrics.push(m_start);
        let h_last = clamp_val(l - m_start, eps, m_max);
        metrics.push(h_last);

        u.resize(3, 0.0);
        let mut cum = 0.0;
        u[0] = 0.0;
        cum += metrics[0];
        u[1] = cum / l;
        cum += metrics[1];
        u[2] = cum / l;
        u[0] = 0.0;
        u[2] = 1.0;
        return true;
    }

    // minimal internal segment count K_min so that K_min * m_max >= s_target
    let mut k_min = (s_target / m_max).ceil() as i32;
    if k_min < 1 {
        k_min = 1;
    }

    let mut n = 2 + k_min; // total segments
    if n < 2 {
        n = 2;
    }

    let mut max_segments = max_segments;
    if max_segments < n as usize {
        max_segments = n as usize;
    }

    // target profile storage
    let mut d: Vec<f64> = Vec::with_capacity(max_segments);

    // --------------------------------------------------------
    // target profile builder with geometric (exp-like) ramps
    // --------------------------------------------------------
    let mut build_profile = |n_local: i32, dvec: &mut Vec<f64>, sum_d: &mut f64| {
        let n_local_usize = n_local as usize;
        dvec.clear();
        dvec.resize(n_local_usize, 0.0);

        dvec[0] = m_start;
        dvec[n_local_usize - 1] = m_end;

        let left_end = ramp_fraction;
        let right_beg = 1.0 - ramp_fraction;

        for i in 1..(n_local_usize - 1) {
            let uu = i as f64 / (n_local_usize as f64 - 1.0);
            let mut t;

            if ramp_fraction > 0.0 {
                if uu <= left_end {
                    // geometric ramp: m_start -> m_max
                    let s = uu / left_end; // 0..1
                    let mut ratio = m_max / m_start;
                    if ratio <= 0.0 {
                        ratio = 1.0;
                    }
                    t = m_start * ratio.powf(s);
                } else if uu >= right_beg {
                    // geometric ramp: m_max -> m_end
                    let s = (1.0 - uu) / ramp_fraction; // 0..1
                    let mut ratio = m_max / m_end;
                    if ratio <= 0.0 {
                        ratio = 1.0;
                    }
                    t = m_end * ratio.powf(s);
                } else {
                    // plateau region
                    t = m_max;
                }
            } else {
                // no ramp region -> constant m_max
                t = m_max;
            }

            t = clamp_val(t, eps, m_max);
            dvec[i] = t;
        }

        *sum_d = dvec.iter().sum();
    };

    let mut sum_d = 0.0;

    // increase n until sum_d >= l (so we can scale down internal part)
    loop {
        if n as usize > max_segments {
            return false;
        }

        build_profile(n, &mut d, &mut sum_d);
        if sum_d + tol_sum >= l {
            break;
        }

        n += 1;
    }

    // now d[0] = m_start, d[n-1] = m_end, and sum_d >= l
    let n_usize = n as usize;
    metrics.clear();
    metrics.resize(n_usize, 0.0);
    metrics[0] = m_start;
    metrics[n_usize - 1] = m_end;

    let sum_d_end = d[0] + d[n_usize - 1];
    let sum_d_inner = sum_d - sum_d_end;

    if sum_d_inner <= eps {
        // internal region nearly zero in the target profile
        if n_usize > 2 {
            let mut m_inner = s_target / (n_usize as f64 - 2.0);
            m_inner = clamp_val(m_inner, eps, m_max);
            for i in 1..(n_usize - 1) {
                metrics[i] = m_inner;
            }
        }
    } else {
        // scale factor for internal metrics so that sum(internal) = s_target
        let mut scale = s_target / sum_d_inner;
        if scale > 1.0 + 1.0e-6 {
            scale = 1.0;
        }
        for i in 1..(n_usize - 1) {
            let val = d[i] * scale;
            metrics[i] = clamp_val(val, eps, m_max);
        }
    }

    // final correction: distribute small residual error over internal segments
    let mut sum_final: f64 = metrics.iter().sum();
    let diff = l - sum_final;

    if diff.abs() > tol_sum && n_usize > 2 {
        let per = diff / (n_usize as f64 - 2.0);
        for i in 1..(n_usize - 1) {
            let val = metrics[i] + per;
            metrics[i] = clamp_val(val, eps, m_max);
        }
        sum_final = metrics.iter().sum();
    }

    // build normalized u array: size = n+1
    u.clear();
    u.resize(n_usize + 1, 0.0);
    let mut cum = 0.0;
    u[0] = 0.0;
    for i in 0..n_usize {
        cum += metrics[i];
        u[i + 1] = cum / l;
    }
    u[0] = 0.0;
    u[n_usize] = 1.0;

    true
}
```
### 테스트 코드
```rust
#[cfg(test)]
mod test {
    use nurbslib::core::build_1d_metric_field::on_build_1d_metric_field_auto_n;

    #[test]
    fn case1() {
        let l = 10.0;
        let m_start = 0.1;
        let m_end = 0.5;
        let m_max = 2.0;

        let ramp_fraction = 0.25;
        let max_segments = 2000usize;
        let tol_sum = 1e-9;

        let mut metrics = Vec::new();
        let mut u = Vec::new();

        let ok = on_build_1d_metric_field_auto_n(
            l,
            m_start,
            m_end,
            m_max,
            ramp_fraction,
            max_segments,
            tol_sum,
            &mut metrics,
            &mut u,
        );

        if !ok {
            println!("on_build_1d_metric_field_auto_n FAILED");
            return;
        }

        println!("N = {}", metrics.len());

        let mut cum = 0.0;
        for (i, h) in metrics.iter().enumerate() {
            cum += *h;
            println!("i={:2}  h={:16.12}  cum_x={:16.12}", i, h, cum);
        }

        println!("\nTotal length = {}", cum);
        println!("Target L     = {}", l);

        println!("\nu (normalized):");
        for (i, uu) in u.iter().enumerate() {
            println!("u[{:2}] = {:16.12}", i, uu);
        }
    }
}
```

### 출력 결과
```
-------------------------------------------
RESULT: N = 8 segments
L      = 10
m_start = 0.1, m_end = 0.5, m_max = 2
ramp_fraction = 0.25
-------------------------------------------

metrics and cumulative length:
i= 0  h=  0.100000000000  cum_x=  0.100000000000
i= 1  h=  0.539120707217  cum_x=  0.639120707217
i= 2  h=  1.946571214941  cum_x=  2.585691922159
i= 3  h=  1.946571214941  cum_x=  4.532263137100
i= 4  h=  1.946571214941  cum_x=  6.478834352041
i= 5  h=  1.946571214941  cum_x=  8.425405566982
i= 6  h=  1.074594433018  cum_x=  9.500000000000
i= 7  h=  0.500000000000  cum_x= 10.000000000000

Total length sum(metrics) = 10.000000000000
Target L                  = 10.000000000000

u (normalized positions):
u[ 0] =   0.000000000000
u[ 1] =   0.010000000000
u[ 2] =   0.063912070722
u[ 3] =   0.258569192216
u[ 4] =   0.453226313710
u[ 5] =   0.647883435204
u[ 6] =   0.842540556698
u[ 7] =   0.950000000000
u[ 8] =   1.000000000000
```
