# Knot Derivative
## 📘 Piegl & Tiller 기반 NURBS Knot Derivative 공식 문서
- 정리할 함수 4개:
  - bspline_basis_with_knot_vector
  - bspline_basis_derivative_wrt_knot
  - denominator_derivative_wrt_knot
  - basis_derivative_wrt_knot (최종 rational basis derivative)

### 1️⃣ B-spline Basis Function
- 함수: bspline_basis_with_knot_vector(Uh, i, p, u)
  - 📖 Piegl Algorithm A2.2 (Cox–de Boor recursion)
- B-spline basis $N_{i,p}(u)$ 는 다음 재귀식으로 정의된다.

- Zero-degree (p = 0)

$$
N_{i,0}(u) =
\begin{cases}
1, & U_i \le u < U_{i+1}, \\
0, & \text{otherwise}.
\end{cases}
$$


- Higher degree (p > 0)

$$
N_{i,p}(u)=\frac{u-U_i}{U_{i+p}-U_i}N_{i,p-1}(u)+\frac{U_{i+p+1}-u}{U_{i+p+1}-U_{i+1}}N_{i+1,p-1}(u)
$$

- 분모가 0이면 해당 항은 0으로 처리.
- ✔ 역할
  - Knot derivative 계산의 기반이 되는 순수 B-spline basis
  - Rational이 아닌 순수 basis
  - Knot vector를 수정한 $U^h$ 에 대해서도 계산 가능

### 2️⃣ B-spline Basis Derivative w.r.t Knot
- 함수: bspline_basis_derivative_wrt_knot(i, k, u)
  - 📖 Piegl Algorithm A2.5 — N_kntbdk
- 이 알고리즘은 다음 값을 계산한다:

$$
\frac{\partial N_{i,p}(u)}{\partial U_k}
$$

- 단, knot multiplicity < p 이어야 한다.

## 📌 Piegl의 3가지 케이스
- Case 1: i=k-p-1

$$
\frac{\partial N_{i,p}(u)}{\partial U_k}=\frac{N_{k-p,p}(u;U^h)}{U_k-U_{k-p}}
$$

- Case 2: k-p\leq i\leq k-1

$$
\frac{\partial N_{i,p}(u)}{\partial U_k}=\frac{N_{i+1,p}(u;U^h)}{U_{i+p+1}-U_{i+1}}-\frac{N_{i,p}(u;U^h)}{U_{i+p}-U_i}
$$

- Case 3: i=k

$$
\frac{\partial N_{k,p}(u)}{\partial U_k}=-\frac{N_{k,p}(u;U^h)}{U_{k+p}-U_k}
$$

- 여기서 $U^h$ 는 Piegl이 정의한 modified knot vector.


### 📘 수학적 의미 :

$$
N_{k-p,p}(u;U^h)
$$

- 이걸 말로 풀면:
- **파라미터 u 에서 평가한 B-spline basis N_{k-p,p}, 단, 사용된 knot vector는 U^h 이다.**




### 3️⃣ Denominator Derivative w.r.t Knot
- 함수: denominator_derivative_wrt_knot(k, u)
  - 📖 Piegl Algorithm A2.6 — N_cfndrk
- NURBS 곡선의 분모는:

$$
D(u)=\sum _{j=0}^nN_{j,p}(u)w_j
$$

- knot U_k 에 대한 도함수는:

$$
\frac{\partial D(u)}{\partial U_k}=\sum _{i=k-p-1}^kw_i\frac{\partial N_{i,p}(u)}{\partial U_k}
$$

- 즉, 영향을 받는 basis는 p+2개뿐.

### 4️⃣ Rational Basis Derivative w.r.t Knot
- 함수: basis_derivative_wrt_knot(i, k, u)
  - 📖 Piegl Algorithm A2.4 — N_kntrck
- Rational basis:

$$
R_i(u)=\frac{N_{i,p}(u)w_i}{D(u)}
$$

- knot U_k 에 대한 도함수:

$$
\frac{\partial R_i(u)}{\partial U_k}=\frac{w_i\frac{\partial N_{i,p}(u)}{\partial U_k}-R_i(u)\frac{\partial D(u)}{\partial U_k}}{D(u)}
$$

- 즉:
- 첫 항: numerator sensitivity
- 두 번째 항: denominator sensitivity
- 전체를 다시 $D(u)$ 로 나눔

## 📌 전체 흐름 요약 (가장 중요)
### 1) 순수 basis 계산

$$
N_{i,p}(u)
$$

### 2) basis derivative wrt knot

$$
\frac{\partial N_{i,p}(u)}{\partial U_k}
$$

### 3) denominator derivative

$$
\frac{\partial D(u)}{\partial U_k}
$$


### 4) rational basis derivative

$$
\frac{\partial R_i(u)}{\partial U_k}
$$

## 🎉 이렇게 정리하면 무엇이 좋은가?
- Rust 코드가 Piegl 알고리즘과 정확히 일치하는지 검증 가능
- analytic 값이 왜 numeric 값과 달랐는지 원인을 찾기 쉬움
- 앞으로 surface(2D) knot derivative 확장도 쉬움
- Knot optimization, knot removal error estimation 등 고급 기능 구현 가능

---

## 소스 코드
```rust
impl NurbsCurve {
    /// Evaluate a single B-spline basis N_{i,p}(u) on a custom knot vector Uh.
    /// - Supports p==0 correctly (was bugged before).
    /// - Uses Side to decide interval closure at knot boundaries.
    fn bspline_basis_with_knot_vector(
        &self,
        Uh: &[f64],
        i: usize,
        p: usize,
        u: f64,
        flp: Side,
    ) -> f64 {
        let m = Uh.len().saturating_sub(1);
        if m == 0 {
            return 0.0;
        }
        if i + p + 1 > m {
            return 0.0;
        }

        // Domain check (allow exact ends; boundary handling done in n0)
        if u < Uh[0] || u > Uh[m] {
            return 0.0;
        }

        // Zero-degree B-spline: N_{i,0}(u)
        fn n0(U: &[f64], i: usize, u: f64, flp: Side) -> f64 {
            match flp {
                Side::Left => {
                    // [U[i], U[i+1])
                    if u >= U[i] && u < U[i + 1] { 1.0 } else { 0.0 }
                }
                Side::Right => {
                    // (U[i], U[i+1]]
                    if u > U[i] && u <= U[i + 1] { 1.0 } else { 0.0 }
                }
            }
        }

        // Cox–de Boor recursion
        fn nip(U: &[f64], i: usize, p: usize, u: f64, flp: Side) -> f64 {
            if p == 0 {
                return n0(U, i, u, flp);
            }
            let left_denom = U[i + p] - U[i];
            let right_denom = U[i + p + 1] - U[i + 1];

            let mut left = 0.0;
            if left_denom.abs() > 1e-15 {
                left = (u - U[i]) / left_denom * nip(U, i, p - 1, u, flp);
            }
            let mut right = 0.0;
            if right_denom.abs() > 1e-15 {
                right = (U[i + p + 1] - u) / right_denom * nip(U, i + 1, p - 1, u, flp);
            }
            left + right
        }

        if p == 0 {
            return n0(Uh, i, u, flp);
        }
        nip(Uh, i, p, u, flp)
    }
}
```
```rust
impl NurbsCurve {
    /// ∂N_{i,p}(u) / ∂U_k with left/right sided knot insertion convention.
    ///
    /// IMPORTANT:
    /// - This is defined for INTERNAL knots only: k ∈ (p .. m-p-1]
    /// - The test must perturb U[k] without breaking nondecreasing order!
    fn bspline_basis_derivative_wrt_knot(
        &self,
        i: usize,
        k: usize,
        u: f64,
        flk: Side,
        flp: Side,
    ) -> f64 {
        let p = self.degree as usize;
        let U = &self.kv.knots;
        let m = U.len().saturating_sub(1);
        if m == 0 || p == 0 {
            return 0.0;
        }

        // internal knot only (Piegl): k ∈ (p .. m-p-1]
        if k <= p || k > m - p - 1 {
            return 0.0;
        }

        // i range for basis functions: i ∈ [0 .. m-p-1]
        if i > m - p - 1 {
            return 0.0;
        }

        // multiplicity around k based on flk
        let mlt = match flk {
            Side::Left => {
                // left-sided insertion: require U[k] != U[k-1]
                if U[k] == U[k.saturating_sub(1)] {
                    return 0.0;
                }
                // count multiplicity to the RIGHT starting at k
                let mut a = k;
                while a + 1 <= m && U[a] == U[a + 1] {
                    a += 1;
                }
                a - k + 1
            }
            Side::Right => {
                // right-sided insertion: require U[k] != U[k+1]
                if U[k] == U[k + 1] {
                    return 0.0;
                }
                // count multiplicity to the LEFT ending at k
                let mut a = k;
                while a > 0 && U[a] == U[a - 1] {
                    a -= 1;
                }
                k - a + 1
            }
        };

        if mlt >= p {
            return 0.0;
        }

        // Build modified knot vector Uh by inserting one extra U[k]
        let mut Uh = Vec::with_capacity(m + 2);
        match flk {
            Side::Left => {
                // U[0 .. k+mlt-1], then extra U[k], then rest from k+mlt
                for a in 0..=(k + mlt - 1) {
                    Uh.push(U[a]);
                }
                Uh.push(U[k]);
                for a in (k + mlt)..=m {
                    Uh.push(U[a]);
                }
            }
            Side::Right => {
                // U[0 .. k], then extra U[k], then rest from k+1
                for a in 0..=k {
                    Uh.push(U[a]);
                }
                Uh.push(U[k]);
                for a in (k + 1)..=m {
                    Uh.push(U[a]);
                }
            }
        }

        // Support of derivative is i ∈ [k-p-1, k]
        if i < k.saturating_sub(p + 1) || i > k {
            return 0.0;
        }

        // Case 1: i == k-p-1
        if i == k - p - 1 {
            let Na = self.bspline_basis_with_knot_vector(&Uh, k - p, p, u, flp);
            let denom = U[k] - U[k - p];
            if denom.abs() < 1e-15 {
                return 0.0;
            }
            return Na / denom;
        }

        // Case 2: k-p <= i <= k-1
        if i >= k - p && i <= k - 1 {
            let Na = self.bspline_basis_with_knot_vector(&Uh, i + 1, p, u, flp);
            let Nb = self.bspline_basis_with_knot_vector(&Uh, i, p, u, flp);

            let d1 = U[i + p + 1] - U[i + 1];
            let d2 = U[i + p] - U[i];

            let a = if d1.abs() < 1e-15 { 0.0 } else { Na / d1 };
            let b = if d2.abs() < 1e-15 { 0.0 } else { Nb / d2 };

            return a - b;
        }

        // Case 3: i == k
        if i == k {
            let Nb = self.bspline_basis_with_knot_vector(&Uh, k, p, u, flp);
            let denom = U[k + p] - U[k];
            if denom.abs() < 1e-15 {
                return 0.0;
            }
            return -Nb / denom;
        }

        0.0
    }
}
```
```rust
impl NurbsCurve {
    fn denominator_derivative_wrt_knot(&self, k: usize, u: f64, flk: Side, flp: Side) -> f64 {
        let p = self.degree as usize;
        let U = &self.kv.knots;
        let m = U.len().saturating_sub(1);
        let n = self.ctrl.len().saturating_sub(1);

        // internal knot only
        if k <= p || k > m - p - 1 {
            return 0.0;
        }

        let mut sum = 0.0;
        let i_start = k.saturating_sub(p + 1);
        let i_end = (std::cmp::min)(k, n);

        for i in i_start..=i_end {
            let dNi = self.bspline_basis_derivative_wrt_knot(i, k, u, flk, flp);
            if dNi == 0.0 {
                continue;
            }
            let wi = self.ctrl[i].w;
            sum += wi * dNi;
        }

        sum
    }
}
```
```rust
impl NurbsCurve {
    fn rational_basis_value(
        &self,
        i: usize,
        u: f64,
        _flp: Side, // span/basis는 LEFT 기준
    ) -> f64 {
        let p = self.degree as usize;
        let n = self.ctrl.len().saturating_sub(1);
        if i > n {
            return 0.0;
        }

        let span = self.kv.knots.as_slice().find_span(n, p, u);
        let N = self.kv.knots.as_slice().basis_funs(span, u, p);

        let mut denom = 0.0;
        for j in 0..=p {
            let idx = span - p + j;
            denom += N[j] * self.ctrl[idx].w;
        }
        if denom.abs() < 1e-15 {
            return 0.0;
        }

        let base = span - p;
        if i < base || i > base + p {
            return 0.0;
        }
        let r = i - base;
        let Ni = N[r];
        let wi = self.ctrl[i].w;

        wi * Ni / denom
    }
}
```
```rust
impl NurbsCurve {
    pub fn basis_derivative_wrt_knot(
        &self,
        i: usize,
        k: usize,
        u: f64,
        flk: Side,
        flp: Side,
    ) -> f64 {
        let p = self.degree as usize;
        let n = self.ctrl.len().saturating_sub(1);
        if i > n {
            return 0.0;
        }

        // D(u) = Σ N_j(u) w_j
        let span = self.kv.knots.as_slice().find_span(n, p, u);
        let N = self.kv.knots.as_slice().basis_funs(span, u, p);

        let mut den = 0.0;
        for j in 0..=p {
            let idx = span - p + j;
            den += N[j] * self.ctrl[idx].w;
        }
        if den.abs() < 1e-15 {
            return 0.0;
        }

        // ∂N_i/∂U_k
        let dNi = self.bspline_basis_derivative_wrt_knot(i, k, u, flk, flp);

        // ∂D/∂U_k
        let dD = self.denominator_derivative_wrt_knot(k, u, flk, flp);

        // R_i(u)
        let R = self.rational_basis_value(i, u, flp);
        let wi = self.ctrl[i].w;

        (wi * dNi - dD * R) / den
    }
}
```

