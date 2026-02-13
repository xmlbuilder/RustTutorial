# Compute FindSpan / Basis Function
## 📌 1. FindSpan이 하는 일 (Algorithm A2.1)
```rust
pub fn on_find_span(n: Index, p: usize, u: Real, knots: &[Real]) -> usize {
    // NOTE:
    // - n: last basis index (n = num_ctrl - 1)
    // - knots length should be n + p + 2
    // - find span i such that knots[i] <= u < knots[i+1] (typical LEFT policy)
    // - special cases: clamp to ends
    if u >= knots[n + 1] {
        return n;
    }
    if u <= knots[p] {
        return p;
    }
    on_find_span_left_eval(
        &KnotVector {
            knots: knots.to_vec(),
        },
        p as Degree,
        u,
    )
    .expect("Invalid Knot")
}
```
```rust

pub fn on_find_span_left_eval(kv: &KnotVector, p: Degree, u: Param) -> Result<Index> {
    on_ensure_param_in_knot_domain(kv, u)?;
    Ok(on_find_span_left_slice(kv.as_slice(), p as usize, u)? as Index)
}
```
```rust
pub fn on_find_span_left_slice(knots: &[Real], p: usize, u: Real) -> Result<usize> {
    let m = knots.len() - 1;
    if m < p + 1 {
        return Err(NurbsError::InvalidInput {
            msg: "Invalid Knot".into(),
        });
    }
    let n = m - p - 1;

    if on_are_equal(u, knots[n + 1], ON_TOL12) {
        return Ok(n);
    }
    if on_are_equal(u, knots[p], ON_TOL12) {
        return Ok(p);
    }

    let mut low = p;
    let mut high = n + 1;
    let mut mid = (low + high) / 2;

    let mut guard = 0usize;
    while u < knots[mid] || u >= knots[mid + 1] {
        guard += 1;
        if guard > 128 {
            return Err(NurbsError::EmptyKnots);
        }

        if u < knots[mid] {
            high = mid;
        } else {
            low = mid;
        }

        if high == low + 1 {
            mid = low;
            break;
        }
        mid = (low + high) / 2;
    }
    Ok(mid)
}
```

- FindSpan은 다음을 찾는 함수:
    - 즉, 파라미터 u가 속한 knot span의 index i를 반환한다.
- 입력:
    - n = basis function 개수 − 1
    - p = degree
    - u = 파라미터
    - U = knot vector
- 출력:
    - i = u가 속한 span index
- 즉, FindSpan은 **u가 어느 구간에 있는지** 를 알려주는 함수다.

## 📌 2. FindSpan이 반환한 i는 무엇을 의미하는가?
- FindSpan이 반환한 i는 다음을 의미한다:
    - ✔ u가 속한 구간은 $U[i], U[i+1])$ 이다.
- 그리고 더 중요한 의미는:
- ✔ 이 구간에서 non‑zero인 p차 B‑spline basis는 정확히 p+1개이며, 그 인덱스는 다음과 같다
```math
N_{i-p,p}(u),\; N_{i-p+1,p}(u),\; \dots ,\; N_{i,p}(u)
```
- 즉, **FindSpan(i)** 는
    - **이제부터 계산해야 할 basis function의 시작 인덱스가 i-p이다** 라는 것을 알려준다.

## 📌 3. 왜 basis function의 index가 i-p부터 i까지인가?
- B‑spline의 국소 지지(local support) 성질 때문.
```math
N_{j,p}(u)\neq 0\quad \mathrm{iff}\quad u\in [U[j],U[j+p+1])
```
- FindSpan이 i를 반환했다는 것은:
```math
u\in [U[i],U[i+1])
```
- 이 구간에서 non-zero가 될 수 있는 j는:
```math
j=i-p,\; i-p+1,\; \dots ,\; i
```
- 즉, p+1개의 basis만 계산하면 된다.

## 📌 4. FindSpan → Basis Function 계산 흐름
- 아래는 Piegl & Tiller의 전체 알고리즘 흐름이야.

### 🔹 Step 1: FindSpan(u)
```rust
i = FindSpan(n, p, u, U)
```

- 예를 들어:
    - p = 3
    - u가 5.2
    - FindSpan이 i = 6을 반환했다고 하자
- 그러면 non-zero basis는:
```math
N_{6-3,3},N_{6-2,3},N_{6-1,3},N_{6,3}
```
- 즉:
```math
N_{3,3},N_{4,3},N_{5,3},N_{6,3}
```
### 🔹 Step 2: Basis Function 계산 (Algorithm A2.2)
- FindSpan이 반환한 i를 기반으로
- 다음 basis들을 계산한다:
```math
N_{i-p,p}(u),\; \dots ,\; N_{i,p}(u)
```
- 이때 inverted triangular scheme을 사용한다:
```
N_{i-p,0}

N_{i-p+1,0}   N_{i-p+1,1}

N_{i-p+2,0}   N_{i-p+2,1}   N_{i-p+2,2}

...

N_{i,0}       N_{i,1}       ...        N_{i,p}
```

- 마지막 열이 우리가 원하는 p차 basis 값이다.

## 소스 코드
```rust

/// BasisFuns(i, u, p) → returns length p + 1
pub fn on_basis_funs_ret_vec(knots: &[Real], span: usize, u: Real, p: usize) -> Vec<Real> {
    let mut n_vec = vec![0.0; p + 1];
    let mut left = vec![0.0f64; p + 1];
    let mut right = vec![0.0f64; p + 1];

    // ---- Special case at right endpoint (u == U[span+1]) ----
    // For a clamped curve, if u == U[n+1] and span == n, then N[p] = 1 and others = 0
    // (Use a small tolerance to account for numerical error)
    let tol = ON_TOL14 * (knots[knots.len() - 1] - knots[0]).abs().max(1.0);
    if (u - knots[span + 1]).abs() <= tol && span + 1 == knots.len() - 1 - p {
        n_vec[p] = 1.0;
        return n_vec;
    }

    n_vec[0] = 1.0;
    for j in 1..=p {
        left[j] = u - knots[span + 1 - j];
        right[j] = knots[span + j] - u;

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
### 🔹 Step 3: Basis Derivatives (Algorithm A2.3)
- FindSpan이 반환한 i를 기반으로
- 다음 도함수들을 계산한다:
```math
N_{i-p,p}^{(k)}(u),\; \dots ,\; N_{i,p}^{(k)}(u)
```
- k = 0..p

```rust

pub fn on_basis_ders_at_span(
    kv: &KnotVector,
    p: usize,
    u: Param,
    span: Index,
    der: usize,
) -> Vec<Vec<Real>> {
    let mut bd = vec![vec![0.0; p + 1]; der + 1];
    let u_vec = kv.as_slice();

    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let mut a = vec![vec![0.0; p + 1]; 2];
    let mut left = vec![0.0; p + 1];
    let mut right = vec![0.0; p + 1];

    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - u_vec[span + 1 - j];
        right[j] = u_vec[span + j] - u;
        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let temp = ndu[r][j - 1] / ndu[j][r];
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    for j in 0..=p {
        bd[0][j] = ndu[j][p];
    }

    let m_der = der.min(p);
    for r in 0..=p {
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        a[0][0] = 1.0;

        for k in 1..=m_der {
            let mut dval = 0.0;
            let rk = r as isize - k as isize;
            let pk = p as isize - k as isize;

            if r >= k {
                a[s2][0] = a[s1][0] / ndu[(pk + 1) as usize][rk as usize];
                dval = a[s2][0] * ndu[rk as usize][pk as usize];
            }

            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            //let j2 = if (r as isize - 1) <= pk { k - 1 } else { p - r };
            let j2 = if r <= (pk as usize) { k - 1 } else { p - r };

            for j in j1..=j2 {
                a[s2][j] =
                    (a[s1][j] - a[s1][j - 1]) / ndu[(pk + 1) as usize][(rk + j as isize) as usize];
                dval += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
            }

            if r as isize <= pk {
                a[s2][k] = -a[s1][k - 1] / ndu[(pk + 1) as usize][r];
                dval += a[s2][k] * ndu[r][pk as usize];
            }

            bd[k][r] = dval;
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    let mut r = p as Real;
    for k in 1..=m_der {
        for j in 0..=p {
            bd[k][j] *= r;
        }
        r *= (p - k) as Real;
    }
    bd
}
```
---

### 📌 5. 전체 흐름을 그림으로 정리
- ✔ FindSpan → basis index 결정
```
u ----> FindSpan ----> i
```

- ✔ basis index → non-zero basis 범위 결정
```
non-zero basis index = [i-p .. i]
```

- ✔ basis 계산
    - Compute $N_{i-p,p}(u) ... N_{i,p}(u)$

- ✔ 도함수 계산
    - Compute $N^{(k)}_{i-p,p}(u) ... N^{(k)}_{i,p}(u)$

### 📌 6. 예시로 완전히 이해하기
- Knot vector:
```math
U=\{ 0,0,0,1,5,6,8,8,8\}
``` 
- degree:
    - p=3
- u = 5.2
#### Step 1: FindSpan(u)
- 5.2는 5,6) 구간에 있으므로:
    - i=4
#### Step 2: non-zero basis index
    - i-p=4-3=1
- 따라서:
```math
N_{1,3}(u),N_{2,3}(u),N_{3,3}(u),N_{4,3}(u)
```
- 이 4개만 non-zero.
#### Step 3: basis 계산
    - Algorithm A2.2로 위 4개를 계산.
#### Step 4: 도함수 계산
    - Algorithm A2.3로
- 각각의 1차, 2차, 3차 도함수까지 계산.

## 🎯 최종 요약
- FindSpan이 반환한 i는
    - u가 속한 knot span의 index이며,
- 이 값이 곧:
    - 어떤 basis function이 non-zero인지
    - 어떤 basis function을 계산해야 하는지
    - basis function의 시작 index(i-p)를 어디로 잡을지
- 를 결정한다.
- 즉,
    - ✔ FindSpan(i)는 basis function 계산의 출발점이다
    - ✔ i-p부터 i까지가 실제 계산 대상이다
    - ✔ 모든 B‑spline/NURBS 알고리즘은 이 인덱스 체계를 기반으로 동작한다

---
