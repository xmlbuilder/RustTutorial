# Pascal Row
## 🎯 결론부터: pascal_row는 이항계수(binomial coefficients) 를 만드는 함수다
- 즉,
- pascal_row(row, k) 는 다음을 만든다:
```math
{k \choose 0},\  {k \choose 1},\  {k \choose 2},\  \dots ,\  {k \choose k}
```
- 예를 들어:
  - k = 0 → [1]
  - k = 1 → [1, 1]
  - k = 2 → [1, 2, 1]
  - k = 3 → [1, 3, 3, 1]
  - k = 4 → [1, 4, 6, 4, 1]
- 즉, 파스칼 삼각형(Pascal’s Triangle) 의 한 행을 만드는 함수다.

## 🎯 왜 NURBS에서 이항계수가 필요할까?
- 유리 기저함수의 도함수는 다음과 같은 형태를 가진다:

```math
R_i^{(k)}(u)=\frac{1}{W(u)}\left( w_iN_i^{(k)}(u)-\sum _{j=1}^k{k \choose j}W^{(j)}(u)R_i^{(k-j)}(u)\right)
```
 
- 여기서 등장하는 ${k \choose j}$
  - 이게 바로 pascal_row가 만드는 이항계수다.
- 즉:
  - Rational basis 도함수는 단순한 미분이 아니라
  - Quotient Rule(나눗셈 미분) 의 고차 도함수 버전
  - 즉, Leibniz rule 을 사용한다
- Leibniz rule:
```math
(f/g)^{(k)}=\frac{1}{g}\left( f^{(k)}-\sum _{j=1}^k{k \choose j}g^{(j)}(f/g)^{(k-j)}\right)
``` 
- 이항계수는 여기서 반드시 필요하다.

## 🎯 pascal_row가 쓰이는 위치
- on_evaluate_rational_bases_and_derivatives:
```cpp
for( j=1; j<=k; j++ )
{
    v -= tri[k][j] * d[j] * RD[k-j][i];
}
```

- 여기서 tri[k][j] 가 바로 이항계수다.
- Rust에서는:
```rust
on_pascal_row(&mut row, k);
...
v -= (row[j] as Real) * d[j] * rd[k - j];
```

- 즉:
  - row[j] = binomial(k, j)
  - $d[j] = W^{(j)}(u)$
  - $rd[k-j] = R_i^{(k-j)}(u)$
- 이 조합이 바로 유리 기저함수의 고차 도함수 공식이다.

## 🎯 요약: pascal_row 의 수학적 의미

| 항목 | 의미 | 수식 |
|------|------|-------|
| pascal_row | 파스칼 삼각형의 한 행 생성 |  |
| 생성되는 값 | 이항계수(binomial coefficients) | $\(\binom{k}{j}\)$ |
| 사용 목적 | 유리 기저함수 도함수 계산(Leibniz rule) |  |
| 등장 이유 | $R_i(u) = (w_i N_i) / W(u)$ 의 고차 도함수에서 필요 |  |
| 적용 위치 | $R_i^{(k)}(u)$ 계산 시 $\(\binom{k}{j} W^{(j)} R_i^{(k-j)}\)$ 항에 사용 |  |


## 🔥 직관적 예시
- 예를 들어 2차 도함수:
```math
R_i''(u)=\frac{1}{W}\left( w_iN_i''-2W'R_i'-W''R_i\right)
``` 
- 여기서 2 는 ${2 \choose 1}=2$
- 이항계수다.
- 3차 도함수는 더 복잡해지고 이항계수가 더 많이 등장한다.

---

## 코드

```rust
pub fn on_evaluate_rational_bases_and_derivatives(
    curve: &NurbsCurve,
    u: Real,
    side: Side,
    der: usize,
) -> Result<(usize, Vec<Vec<Real>>), NurbsError> {
    // ---- basic checks ----
    let cv_count = curve.cv_count();
    if cv_count == 0 {
        return Err(NurbsError::InvalidArgument {
            msg: "curve has no control points".into(),
        });
    }
    let n = cv_count - 1;

    let degree: Degree = curve.degree();
    let p = degree as usize;
    if p > n {
        return Err(NurbsError::InvalidArgument {
            msg: format!("degree p={} > n={} (cv_count-1)", p, n),
        });
    }

    let kv = curve.knots();
    on_ensure_param_in_knot_domain(kv, u)?;

    // ---- find span with LEFT/RIGHT semantics ----
    let span = on_find_span_left_right(kv, degree, u, side)?;
    if span < p {
        return Err(NurbsError::InvalidArgument {
            msg: format!("span={} < degree p={}", span, p),
        });
    }

    let first = span - p;

    // ---- (1) ND: non-rational basis derivatives on this span ----
    // ND[k][i_local] = N_{first+i_local}^{(k)}(u)
    let nd: Vec<Vec<Real>> = on_basis_ders_at_span(kv, p, u, span, der);

    // ---- (2) denominator derivatives d[k] = Σ w_j * ND[k][j_local] ----
    let mut d = vec![0.0; der + 1];
    for k in 0..=der {
        let mut acc = 0.0;
        for i_local in 0..=p {
            let j = first + i_local;
            let wj = curve.weight(j).unwrap_or(1.0);
            acc += wj * nd[k][i_local];
        }
        d[k] = acc;
    }

    if d[0].abs() < 1e-14 {
        return Err(NurbsError::NumericError {
            msg: "denominator W(u) is zero or too small".into(),
        });
    }

    // ---- (3) RD via Pascal recursion ----
    // RD[k][i_local] = rational basis derivative
    let mut rd = vec![vec![0.0; p + 1]; der + 1];
    let mut row = vec![0usize; der + 1];

    for i_local in 0..=p {
        let wi = curve.weight(first + i_local).unwrap_or(1.0);

        for k in 0..=der {
            let mut v = wi * nd[k][i_local];

            on_pascal_row(&mut row, k);
            for j in 1..=k {
                v -= (row[j] as Real) * d[j] * rd[k - j][i_local];
            }

            rd[k][i_local] = v / d[0];
        }
    }

    Ok((span, rd))
}
```
```rust
fn on_pascal_row(row: &mut [usize], k: usize) {
    debug_assert!(row.len() >= k + 1);
    row[0] = 1;
    for i in 1..=k {
        row[i] = row[i - 1] * (k + 1 - i) / i;
    }
}
```
---

