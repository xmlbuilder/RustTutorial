# 📘 Function Interpolation
- Functional Parametrization & Interpolation with LU Decomposition
- Mathematical Documentation for the Rust Implementation

## 1. Functional Parametrization
- on_parameterization_functional(f, par, ub)
### 1.1 목적
- 스칼라 데이터 $f_i$ 에 대해 파라미터 $u_i$ 를 생성한다.
#### 1.2 입력
- $f=[f_0,f_1,\dots ,f_n]$
- par:
    - Uniform
    - ChordLength
    - Centripetal
- ub: 출력 파라미터 배열
#### 1.3 출력
- u_0=0, u_n=1
- 나머지 u_i 는 선택된 방식에 따라 계산

#### 1.4 Uniform parametrization
```math
u_i=\frac{i}{n},\quad i=0..n
```
- Rust 구현:
```rust
let fact = 1.0 / (last as Real);
ub[i] = i * fact;
```


#### 1.5 Chord-length parametrization
- 거리 기반:
```math
d_i=|f_i-f_{i-1}|
```
```math
S=\sum _{i=1}^nd_i
```
```math
u_i=u_{i-1}+\frac{d_i}{S}
```
- Rust 구현:
```rust
let d = (f[i] - f[i-1]).abs();
sum += d;
on_calc_distance_span(...);
```


#### 1.6 Centripetal parametrization
```math
d_i=\sqrt{|f_i-f_{i-1}|}
````
```math
u_i=u_{i-1}+\frac{d_i}{\sum d_i}
```
- Rust 구현:
```math
let d = (f[i] - f[i-1]).abs().sqrt();
```

## 2. Functional Interpolation
- on_function_interpolation(f, u, p)
### 2.1 목적
- 주어진 데이터 $(u_i,f_i)$ 를 정확히 지나는 B-spline 함수를 구성한다.

#### 2.2 수학적 문제 정의
- 목표
- B-spline 함수:
```math
F(u)=\sum _{j=0}^nN_{j,p}(u)\, F_j
```
- 조건:
```math
F(u_i)=f_i,\quad i=0..n
```
- 여기서 $F_j$ 는 control point 값.

#### 2.3 Knot vector 생성
- clamped
- averaged interior knots
- Rust:
```rust
let kv = on_global_inp_knot_vectors(u, n, p);
```


#### 2.4 선형 시스템 구성
- 각 데이터 점에 대해:
```math
\sum _{j=0}^nN_{j,p}(u_i)F_j=f_i
```
- 행렬 형태:
```math
AF=f
```
- 여기서:
    - $A_{ij}=N_{j,p}(u_i)$
    - $F=[F_0,\dots ,F_n]^T$`

#### 2.5 경계 조건
```math
F_0=f_0,\quad F_n=f_n
```
- Rust:
```rust
a[0][0] = 1.0;
a[n][n] = 1.0;
```


#### 2.6 내부 행 구성
```math
A_{i,\, j}=N_{j,p}(u_i)
```
- Rust:
```rust
on_compute_all_basis(&kv, p, u[i], Side::Left, &mut basis, &mut span)?;
```


#### 2.7 LU 분해로 선형 시스템 해결
- Rust:
```rust
let lu = on_m_lu_decmp_full_with_pivot(a, lu_dt)?;
on_lu_solve_with_pivot(&lu, &mut fu);
```


## 3. LU Decomposition with Partial Pivoting
- on_m_lu_decmp_full_with_pivot
### 3.1 목적
- 행렬 A 를 다음 형태로 분해:
```math
PA=LU
```
- P: pivot permutation
- L: unit lower triangular
- U: upper triangular

### 3.2 Pivot 선택
- 각 column k 에 대해:
```math
p=\arg \max _{i\geq k}|a_{ik}|
```
- Rust:
```rust
let mut p = k;
let mut amax = a[k][k].abs();
for i in k+1..n {
    if a[i][k].abs() > amax { p = i; }
}
```
### 3.3 Row swap
```math
\mathrm{swap}(A[k],A[p])
```
- Rust:
```rust
a.swap(p, k);
```


### 3.4 L, U 업데이트
- Doolittle 방식:
- U row:
```math
U_{k,j}=A_{k,j}-\sum _{m=0}^{k-1}L_{k,m}U_{m,j}
```
- L column:
```math
L_{i,k}=\frac{A_{i,k}-\sum _{m=0}^{k-1}L_{i,m}U_{m,k}}{U_{k,k}}
```
- Rust:
```rust
a[i][k] /= pivot;
a[i][j] -= a[i][k] * a[k][j];
```


## 4. LU Solve
- on_lu_solve_with_pivot
### 4.1 Permutation 적용
```math
b'=Pb
```
### 4.2 Forward substitution
```math
Ly=b'
```
### 4.3 Back substitution
```math
Ux=y
```
- Rust:
```rust
for i in 0..n { ... }   // Ly=b
for i in (0..n).rev() { ... } // Ux=y
```


## 5. 전체 파이프라인 요약
- Functional parametrization
    - Uniform / Chord-length / Centripetal
    - 생성된 $u_i$ 는 단조 증가
- Functional interpolation
    - Knot vector 생성
    - Basis function 계산
- 선형 시스템 구성
    - LU 분해로 control point 계산
    - LU decomposition with pivoting
    - 안정적인 해 보장

---

## 소스 코드
```rust
pub fn on_function_interpolation(f: &[Real], u: &[Real], p: usize) -> Result<CFun, NurbsError> {
    if f.is_empty() || u.is_empty() || f.len() != u.len() {
        return Err(NurbsError::InvalidArgument {
            msg: "on_function_interpolation: f/u size mismatch or empty".into(),
        });
    }
    ensure_non_decreasing(u)?;
    validate_u_f_for_interpolation(u, f, 0.0, 1e-12)?;
    if p < 1 {
        return Err(NurbsError::InvalidArgument {
            msg: "on_function_interpolation: degree p must be >= 1".into(),
        });
    }

    let n = f.len() - 1; // highest index
    if p > n {
        return Err(NurbsError::InvalidArgument {
            msg: "on_function_interpolation: p > n".into(),
        });
    }

    // 1) Compute knot vector
    let kv = on_global_inp_knot_vectors(u, n, p);

    // 2) Linear splines (C: if p==1, fu[i]=f[i])
    if p == 1 {
        return Ok(CFun {
            degree: p as Degree,
            kv,
            ctrl: f.to_vec(),
        });
    }

    // 3) Build full coefficient matrix A (C builds banded; here we build dense NxN)
    //    Unknowns: fu[0..=n]
    let dim = n + 1;
    let mut a = vec![vec![0.0; dim]; dim];

    // Boundary constraints (C: A[0][sbw]=1, A[n][sbw]=1) -> dense: A[0][0]=1, A[n][n]=1
    a[0][0] = 1.0;
    a[n][n] = 1.0;

    // Internal rows: i=1..n-1
    let mut basis = vec![0.0; p + 1];
    for i in 1..n {
        let mut span = 0usize;

        // C: N_kntalb(knt,p,u[i],LEFT,N,&j)
        on_compute_all_basis(&kv, p, u[i], Side::Left, &mut basis, &mut span)?;

        // Nonzero columns are (span - p) .. span
        // Dense placement: A[i][span-p+k] = N[k]
        if span < p {
            eprintln!("on_function_interpolation: invalid span {} for p {}", span, p);
            return Err(NurbsError::InvalidArgument {
                msg: format!("on_function_interpolation: invalid span {} for p {}", span, p),
            });
        }
        let col0 = span - p;
        if col0 + p > n {
            eprintln!("on_function_interpolation: basis column range out of bounds (col0={}, p={}, n={})", col0, p, n);
            return Err(NurbsError::InvalidArgument {
                msg: format!("on_function_interpolation: basis column range out of bounds (col0={}, p={}, n={})", col0, p, n),
            });
        }

        for k in 0..=p {
            a[i][col0 + k] = basis[k];
        }
    }

    // 4) LU decompose
    //    NOTE: no pivoting (same spirit as your lucmp.rs). If needed, we can add pivoting later.
    let lu_dt = 1e-12; // numeric threshold similar to "NUM_ERR" guard
    let lu = on_m_lu_decmp_full_with_pivot(a, lu_dt).ok_or_else(|| NurbsError::NumericError {
        msg: "on_function_interpolation: LU decomposition failed (singular/ill-conditioned)".into(),
    })?;

    // 5) Solve A * fu = f
    let mut fu = f.to_vec(); // RHS -> overwritten by solution
    if !on_lu_solve_with_pivot(&lu, &mut fu) {
        return Err(NurbsError::NumericError {
            msg: "on_function_interpolation: LU solve failed".into(),
        });
    }

    Ok(CFun {
        degree: p as Degree,
        kv,
        ctrl: fu,
    })
}
```
```rust
/// Parametrization for global curve interpolation
pub fn on_parameterization_euclidean(
    q: &SimpleArray<Point3D>,
    par: ParamMode,
    ub: &mut SimpleArray<Real>,
) {
    let n = q.count();
    if n == 0 {
        ub.data.clear();
        return;
    }
    if n == 1 {
        ub.data.resize(1, 0.0);
        return;
    }

    ub.data.resize(n, 0.0);
    ub.data[0] = 0.0;
    ub.data[n - 1] = 1.0;

    match par {
        ParamMode::Uniform => {
            let fact = 1.0 / ((n - 1) as Real);
            for i in 1..n - 1 {
                ub.data[i] = (i as Real) * fact;
            }
        }

        ParamMode::ChordLength => {
            let mut a_dist = SimpleArray::<Real>::new();
            a_dist.data.resize(n, 0.0);
            on_chord_length_parametrization(0, n, q, &mut a_dist, ub);
        }

        ParamMode::Centripetal => {
            let mut dist = SimpleArray::<Real>::new();
            dist.data.resize(n, 0.0);

            let mut sum = 0.0;
            dist.data[0] = 0.0;

            for i in 1..n {
                let d = q[i].distance(&q[i - 1]);
                let d_sqrt = d.sqrt();
                dist.data[i] = d_sqrt;
                sum += d_sqrt;
            }

            on_calc_distance_span(n, &dist, sum, ub);
        }
        _ => {}
    }
}
```
```rust
/// Parametrization for global curve interpolation
pub fn on_parameterization_homogeneous(
    q: &SimpleArray<Point4D>,
    par: ParamMode,
    ub: &mut SimpleArray<Real>,
) {
    let n = q.count();
    if n == 0 {
        ub.data.clear();
        return;
    }
    if n == 1 {
        ub.data.resize(1, 0.0);
        return;
    }

    ub.data.resize(n, 0.0);
    ub.data[0] = 0.0;
    ub.data[n - 1] = 1.0;

    match par {
        ParamMode::Uniform => {
            let fact = 1.0 / ((n - 1) as Real);
            for i in 1..n - 1 {
                ub.data[i] = (i as Real) * fact;
            }
        }

        ParamMode::ChordLength => {
            let mut dist = SimpleArray::<Real>::new();
            dist.data.resize(n, 0.0);

            let mut sum = 0.0;
            dist.data[0] = 0.0;

            for i in 1..n {
                let d = q[i].distance_to(q[i - 1]); // Point4D용 distance 구현 가정
                dist.data[i] = d;
                sum += d;
            }

            on_calc_distance_span(n, &dist, sum, ub);
        }

        ParamMode::Centripetal => {
            let mut dist = SimpleArray::<Real>::new();
            dist.data.resize(n, 0.0);

            let mut sum = 0.0;
            dist.data[0] = 0.0;

            for i in 1..n {
                let d = q[i].distance_to(q[i - 1]); // G_discpo와 동일 역할
                let d_sqrt = d.sqrt();
                dist.data[i] = d_sqrt;
                sum += d_sqrt;
            }

            on_calc_distance_span(n, &dist, sum, ub);
        }
        _ => {}
    }
}
```
```rust
/// Functional parametrization (scalar data f[i])
/// - f: scalar data values
/// - ub: output parameters (size = f.count())
pub fn on_parameterization_functional(
    f: &SimpleArray<Real>,
    par: ParamMode,
    ub: &mut SimpleArray<Real>,
) {
    let n = f.count();
    if n == 0 {
        ub.data.clear();
        return;
    }
    if n == 1 {
        ub.data.resize(1, 0.0);
        return;
    }

    let last = n - 1;

    // u[0] = 0, u[n] = 1
    ub.data.resize(n, 0.0);
    ub.data[0] = 0.0;
    ub.data[last] = 1.0;

    match par {
        ParamMode::Uniform => {
            let fact = 1.0 / (last as Real);
            for i in 1..last {
                ub.data[i] = (i as Real) * fact;
            }
        }

        ParamMode::ChordLength => {
            let mut dist = SimpleArray::<Real>::new();
            dist.data.resize(n, 0.0);

            let mut sum = 0.0;
            for i in 1..n {
                let d = (f[i] - f[i - 1]).abs();
                dist.data[i] = d;
                sum += d;
            }

            on_calc_distance_span(n, &dist, sum, ub);
        }

        ParamMode::Centripetal => {
            let mut dist = SimpleArray::<Real>::new();
            dist.data.resize(n, 0.0);

            let mut sum = 0.0;
            for i in 1..n {
                let d = (f[i] - f[i - 1]).abs().sqrt();
                dist.data[i] = d;
                sum += d;
            }

            on_calc_distance_span(n, &dist, sum, ub);
        }
        _ => {}
    }
}

---

### 테스크 코드
```rust
#[cfg(test)]
mod tests
{
    use nurbslib::core::cfun::Side;
    use nurbslib::core::knot::on_compute_all_basis;
    use nurbslib::core::prelude::{Real, NurbsError};
    use nurbslib::core::cfun::{CFun};
    use nurbslib::core::functions::on_function_interpolation;
    // -----------------------------
    // Helpers
    // -----------------------------

    fn eval_cfun(c: &CFun, u: Real) -> Result<Real, NurbsError> {
        let p = c.degree;
        let n = c.ctrl.len().saturating_sub(1);

        let mut basis = vec![0.0; p as usize + 1];
        let mut span = 0usize;

        // local basis + span
        on_compute_all_basis(&c.kv, p as usize, u, Side::Left, &mut basis, &mut span)?;

        if span < p as usize {
            return Err(NurbsError::InvalidArgument {
                msg: format!("eval_cfun: span < p (span={}, p={})", span, p),
            });
        }

        let col0 = span - p as usize;
        if col0 + p as usize > n {
            return Err(NurbsError::InvalidArgument {
                msg: format!(
                    "eval_cfun: basis column out of bounds (col0={}, p={}, n={})",
                    col0, p, n
                ),
            });
        }

        let mut s = 0.0;
        for k in 0..=p {
            s += c.ctrl[col0 + k as usize] * basis[k as usize];
        }
        Ok(s)
    }

    /// deterministic pseudo-rng (LCG) to avoid external rand crate
    fn lcg_next(state: &mut u64) -> u64 {
        // constants from Numerical Recipes-ish
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        *state
    }

    fn lcg_f64(state: &mut u64, a: Real, b: Real) -> Real {
        let x = (lcg_next(state) >> 11) as u64; // 53-bit
        let t = (x as Real) / ((1u64 << 53) as Real);
        a + (b - a) * t
    }

    fn max_abs_err(c: &CFun, u: &[Real], f: &[Real]) -> Real {
        let mut e_max = 0.0;
        for i in 0..u.len() {
            let y = eval_cfun(c, u[i]).unwrap();
            let e = (y - f[i]).abs();
            if e > e_max { e_max = e; }
        }
        e_max
    }

```
```rust
    // -----------------------------
    // Tests
    // -----------------------------

    #[test]
    fn func_inp_p1_linear_is_exact_copy() {
        let n = 10usize;
        let p = 1usize;

        let mut u = vec![0.0; n + 1];
        let mut f = vec![0.0; n + 1];
        for i in 0..=n {
            u[i] = i as Real;                   // monotone
            f[i] = (i as Real) * 2.5 - 7.0;     // arbitrary
        }

        let c = on_function_interpolation(&f, &u, p).unwrap();
        assert_eq!(c.degree, 1);
        assert_eq!(c.ctrl.len(), f.len());
        assert_eq!(c.ctrl, f); // p==1 branch is direct copy
    }
```
```rust
    #[test]
    fn func_inp_cubic_uniform_params_interpolates_samples() {
        let n = 12usize;
        let p = 3usize;

        let mut u = vec![0.0; n + 1];
        let mut f = vec![0.0; n + 1];

        for i in 0..=n {
            u[i] = (i as Real) / (n as Real); // [0,1]
            // smooth but nontrivial
            let t = u[i];
            f[i] = (2.0 * std::f64::consts::PI * t).sin() + 0.25 * (7.0 * t).cos();
        }

        let c = on_function_interpolation(&f, &u, p).unwrap();

        // should interpolate at the supplied parameter samples
        let e_max = max_abs_err(&c, &u, &f);
        assert!(e_max < 1e-9, "max abs error too large: {}", e_max);
    }
```
```rust
    #[test]
    fn func_inp_quintic_nonuniform_params_interpolates_samples() {
        let n = 18usize;
        let p = 5usize;

        let mut u = vec![0.0; n + 1];
        let mut f = vec![0.0; n + 1];

        // non-uniform, strictly increasing in [0,1]
        for i in 0..=n {
            let t = (i as Real) / (n as Real);
            u[i] = t * t; // clustered near 0
            f[i] = (3.0 * t).exp() * (1.0 + 0.1 * (11.0 * t).sin());
        }

        println!("u = {:?}", u);
        println!("f = {:?}", f);
        println!("p = {:?}", p);

        let c = on_function_interpolation(&f, &u, p).unwrap();
        let emax = max_abs_err(&c, &u, &f);
        assert!(emax < 1e-8, "max abs error too large: {}", emax);
    }
```
```rust
    #[test]
    fn func_inp_minimal_case_n_equals_p_should_work() {
        // n = p => dim = p+1, still valid
        let p = 3usize;
        let n = 3usize;

        let u = vec![0.0, 0.2, 0.7, 1.0];
        let f = vec![1.0, -0.5, 2.25, 0.75];

        let c = on_function_interpolation(&f, &u, p).unwrap();
        let e_max = max_abs_err(&c, &u, &f);
        assert!(e_max < 1e-10, "max abs error too large: {}", e_max);
    }
```
```rust
    #[test]
    fn func_inp_invalid_degree_p_gt_n_is_error() {
        let u = vec![0.0, 0.5, 1.0];
        let f = vec![1.0, 2.0, 3.0];
        let p = 4usize; // p > n(=2)

        let r = on_function_interpolation(&f, &u, p);
        assert!(r.is_err());
    }
```
```rust
    #[test]
    fn func_inp_unsorted_u_should_error() {
        // u not non-decreasing => knot vector becomes non-monotone => span/basis should fail
        let u = vec![0.0, 0.7, 0.2, 1.0];
        let f = vec![1.0, 2.0, 3.0, 4.0];
        let p = 3usize;

        let r = on_function_interpolation(&f, &u, p);
        assert!(r.is_err(), "expected error for unsorted u");
    }
```
```rust
    #[test]
    fn func_inp_nearly_duplicate_params_can_be_singular_and_should_fail_cleanly() {
        // internal u values almost identical => rows become almost identical => LU may fail
        let u = vec![0.0, 0.25, 0.25000000000000006, 0.2500000000000001, 1.0];
        let f = vec![1.0, 2.0, 2.0, 2.0, 3.0];
        let p = 3usize; // n=4, p=3

        let r = on_function_interpolation(&f, &u, p);
        assert!(r.is_err(), "expected numeric failure or invalid argument");
    }
    // -----------------------------
    // Random parameter generators
    // -----------------------------

    /// Build a strictly increasing u in [0,1] with u[0]=0, u[n]=1.
    ///
    /// IMPORTANT: This avoids duplicates by construction.
    /// We accumulate positive steps, then normalize by the last value.
    fn make_random_strict_u01(seed: &mut u64, n: usize, min_step: Real, max_step: Real) -> Vec<Real> {
        debug_assert!(n >= 1);
        debug_assert!(min_step > 0.0 && max_step > min_step);

        let mut u = vec![0.0; n + 1];
        u[0] = 0.0;

        for i in 1..=n {
            let step = lcg_f64(seed, min_step, max_step);
            u[i] = u[i - 1] + step; // no clamp
        }

        let last = u[n];
        for i in 0..=n {
            u[i] /= last; // u[n] becomes exactly 1.0
        }

        // sanity: strictly increasing
        for i in 1..=n {
            assert!(u[i] > u[i - 1], "make_random_strict_u01 produced non-strict u");
        }

        u
    }

    fn make_smoothish_f(seed: &mut u64, u: &[Real]) -> Vec<Real> {
        let mut f = vec![0.0; u.len()];
        for i in 0..u.len() {
            let t = u[i];
            f[i] = 0.3 * lcg_f64(seed, -1.0, 1.0)
                + (2.0 * std::f64::consts::PI * t).sin()
                + 0.2 * (5.0 * t).cos();
        }
        f
    }
```
```rust
    #[test]
    fn func_inp_random_monotone_params_multiple_runs_strict_u() {
        // ✅ strict u generator: no duplicates, always interpolation-feasible
        let mut seed = 0x1234_5678_9abc_def0u64;

        for case_id in 0..200 {
            let p = 4usize;
            let n = 10usize;

            let u = make_random_strict_u01(&mut seed, n, 1e-3, 0.2);
            let f = make_smoothish_f(&mut seed, &u);

            let c = on_function_interpolation(&f, &u, p)
                .unwrap_or_else(|e| panic!("case {} failed with {:?}\nu={:?}\nf={:?}\np={}", case_id, e, u, f, p));

            let emax = max_abs_err(&c, &u, &f);
            assert!(
                emax < 1e-7,
                "case {} err too large: {}\nu={:?}\nf={:?}\np={}",
                case_id,
                emax,
                u,
                f,
                p
            );
        }
    }
```
```rust
    #[test]
    fn func_inp_duplicate_u_with_different_f_should_error() {
        // ✅ should hit your guard: "duplicate u ... but f differs"
        let u = vec![0.0, 0.3, 0.7, 1.0, 1.0];
        let f = vec![0.0, 1.0, 2.0, 3.0, 3.1]; // differs at duplicate u
        let p = 3usize;

        let r = on_function_interpolation(&f, &u, p);
        assert!(r.is_err(), "expected error for duplicate u with different f");

        if let Err(NurbsError::InvalidArgument { msg }) = r {
            assert!(msg.contains("duplicate u"), "unexpected message: {}", msg);
        }
    }
}
```



---
