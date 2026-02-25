# on_power_reparam_matrix_interval
## 🎯 이 함수의 정확한 용도
- Bezier 계수 → Power 계수 → Reparam → 다시 Bezier 계수
- 이 전체 변환의 역행렬(inverse reparameterization matrix) 를 만드는 함수다.
- 즉:
    - Bezier 곡선이 어떤 구간에서 정의되어 있을 때,
    - 그 구간을 다른 구간으로 바꾸면서도
    - Bezier 계수를 그대로 유지하려면 필요한 변환 행렬의 역행렬

- 을 계산한다.

## 🧩 왜 이렇게 복잡한가?
- Bezier 계수는 직접적으로 affine reparam(u = αt + β) 를 적용할 수 없다.
- 왜냐하면 Bezier basis는 Bernstein basis이기 때문.
- 그래서 변환은 반드시 아래 순서를 거친다:
    - Bezier → Power → Reparam → Power → Bezier


## 이 함수는 그 중에서도 역방향(inverse) 을 계산한다.

## 🧱 함수 내부 단계별 설명
- 함수 전체를 그림처럼 설명하면 이렇게 된다:
```
Bezier (old)
   ↓ P⁻¹
Power (old)
   ↓ R⁻¹
Power (new)
   ↓ T⁻¹
Bezier (new)
```

- 즉:
  - M_inv = T⁻¹ · R⁻¹ · P⁻¹
  
  - 이걸 계산하는 함수가 바로 on_power_reparam_matrix_interval.

## 🔍 이제 코드 단계별로 정확히 설명해줄게

- ① (alpha, beta) 계산 — 역방향 affine 변환
```rust
let (alpha, beta) = on_re_param_affine(ap, bp, a, b);
```

- 여기서 하는 일:
    - 우리가 원하는 건 u → up 변환의 역행렬
- 즉,
```rust
up = ap + (bp - ap) * t
```

- 이런 식의 affine map을 역으로 풀어서
```math
u = alpha * t + beta
```
- 형태로 만든다.
- 이게 R⁻¹ 행렬의 기반이 된다.
- ② R⁻¹ 행렬 구성 
- Power basis에서의 역 reparamr_inv[i][j] = C(i,j) * $\beta^{i-j}$ * $\alpha^j$
이건 다음 식의 전개:
```math
(\alpha t+\beta )^i=\sum _{j=0}^i{i \choose j}\beta ^{i-j}(\alpha t)^j
```
- 즉:
  - old power basis 계수 → new power basis 계수
  - 역방향 reparameterization
  - ③ T⁻¹ = power_to_bezier_matrix(p)
      - Power basis → Bezier basis 변환 행렬의 역행렬.
  - ④ P⁻¹ = bezier_to_power_matrix(p)
      - Bezier basis → Power basis 변환 행렬의 역행렬.
  - ⑤ 최종 행렬 조합
      - m_inv = T⁻¹ · R⁻¹ · P⁻¹
- 이게 바로: **Bezier 계수를 직접 reparameterize하는 역행렬** 이다.
## 🎨 그림으로 다시 정리
```
Bezier(old) -- P⁻¹--> Power(old) -- R⁻¹--> Power(new) -- T⁻¹--> Bezier(new)
```
- 즉:
```
Bezier(new) = M_inv * Bezier(old)
```
```
C_bez_new = M_inv · C_bez_old
M_inv = T⁻¹ · R⁻¹ · P⁻¹
```
```rust

///B_IREMAT: Non-zero elements of inverse of reparametrization matrix
pub fn on_power_reparam_matrix_interval(
    p: usize,
    a: Real,
    b: Real,
    ap: Real,
    bp: Real,
) -> Vec<Vec<Real>> {
    
    // 1. Affine inverse transformation coefficients
    let (alpha, beta) = on_re_param_affine(ap, bp, a, b);

    // 2. Construct the R⁻¹ matrix: expansion of (αt + β)^i
    let mut r_inv = vec![vec![0.0; p + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=i {
            let comb = on_binomial_usize(i, j) as f64;
            r_inv[i][j] = comb * beta.powi((i - j) as i32) * alpha.powi(j as i32);
        }
    }

    // 3. T⁻¹ = power_to_bezier_matrix(p)
    let t_inv = on_power_to_bezier_vec(p);

    // 4. P⁻¹ = bezier_to_power_matrix(p)
    let p_inv = on_bezier_to_power_matrix(p);

    // 5. Matrix multiplication with nalgebra: T⁻¹ · R⁻¹ · P⁻¹
    let r_na = DMatrix::from_row_slice(p + 1, p + 1, &r_inv.concat());
    let t_na = DMatrix::from_row_slice(p + 1, p + 1, &t_inv.concat());
    let p_na = DMatrix::from_row_slice(p + 1, p + 1, &p_inv.concat());

    let m_inv = t_na * r_na * p_na;

    // 6. DMatrix → Vec<Vec<Real>>
    let mut result = vec![vec![0.0; p + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=p {
            result[i][j] = m_inv[(i, j)];
        }
    }
    result
}
```
---
