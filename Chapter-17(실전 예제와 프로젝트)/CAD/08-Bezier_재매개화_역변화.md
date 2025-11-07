# Bezier 재매개화 / 역변환
Bezier 곡선의 재매개화와 그 역변환에 대한 수학적으로 정확하고 자세한 설명 문서입니다.  
이 문서는 알고리즘의 목적, 수학적 배경, 행렬 구성 방식, 그리고 구현상의 주의점까지 모두 포함합니다.

## 📘 Bezier 곡선의 재매개화와 역변환: 수학적 설명
##  1. 개요
Bezier 곡선은 매개변수 $t\in [a,b]$ 에 대해 정의된 곡선입니다.
재매개화(reparameterization)는 이 곡선을 다른 구간 $t'\in [a',b']$ 로 표현하고자 할 때 사용됩니다.
이 과정은 단순한 구간 변경이 아니라, Bezier 계수 자체를 변환하는 수학적 작업입니다.

## 2. 목표
주어진 Bezier 곡선:

$$
C(t)=\sum _{i=0}^nB_i^n(t)\cdot c_i,\quad t\in [a,b]
$$

이를 새로운 구간 $t'\in [a',b']$ 에 대해 동일한 곡선으로 표현:

$$
C(t')=\sum _{i=0}^nB_i^n(t')\cdot c'_i
$$

→ 여기서 $c'=M\cdot$ $c$, $M$ 은 재매개화 행렬

## 3. 수학적 절차
### 3.1. 매개변수 치환
구간 변경은 선형 치환으로 표현됩니다:  

$t=\alpha u+\beta$  
- $u\in [a',b']$
- $t\in [a,b]$
- $\alpha =\frac{b-a}{b'-a'}$
- $\beta =\frac{b'a-a'b}{b'-a'}$

### 3.2. 기저 변환
Bezier 계수는 Bezier basis에서 정의되어 있으므로, 다음과 같은 변환이 필요합니다:
- Bezier → Power basis

$$
\mathrm{power}=T\cdot \mathrm{bezier}
$$

- Affine 치환  

$$
\mathrm{power}'=R\cdot \mathrm{power}- R_{i,j}={i \choose j}\cdot \beta ^{i-j}\cdot \alpha ^j
$$

- Power → Bezier basis  

$$
\mathrm{bezier}'=P\cdot \mathrm{power}'
$$

### 3.3. 최종 재매개화 행렬
전체 변환:

$c'=M\cdot c=P\cdot R\cdot T\cdot c$

- $T$: Bezier → Power basis 변환 행렬
- $R$: affine 치환 행렬
- $P$: Power → Bezier basis 변환 행렬

## 4. 역변환
역변환은 위의 순서를 정확히 반대로 수행합니다:

$c=M^{-1}\cdot c'=T^{-1}\cdot R^{-1}\cdot P^{-1}\cdot c'$

- $T^{-1} = \texttt{power\\_to\\_bezier\\_matrix}(n)$
- $P^{-1} = \texttt{bezier\\_to\\_power\\_matrix}(n)$
- $R^{-1}$ : affine 역변환 t=\alpha u+\beta 의 다항식 전개

## 5. 구현 요약
### 5.1. on_re_param_matrix(p, a, b, a', b')
- 1. α, β 계산
- 2. R 행렬 구성
- 3. T = bezier_to_power_matrix(p)
- 4. P = power_to_bezier_matrix(p)
- 5. M = P · R · T

### 5.2. on_re_param_matrix_inverse(p, a, b, a', b')
- 1. α, β 계산 (역방향)
- 2. R⁻¹ 행렬 구성
- 3. T⁻¹ = power_to_bezier_matrix(p)
- 4. P⁻¹ = bezier_to_power_matrix(p)
- 5. M⁻¹ = T⁻¹ · R⁻¹ · P⁻¹

## 6. 주의사항
- Bezier 계수는 구간에 따라 달라지므로, 구간 변경 시 반드시 재매개화 필요
- 역행렬은 수치적으로 불안정할 수 있으므로 nalgebra를 통한 안정적인 계산 권장
- 재매개화는 곡선의 형태는 유지하면서 정의역만 바꾸는 작업임

## 7. 활용 예시
- 곡선의 일부 추출 (트리밍)
- 곡면 패치 분할
- 정규화된 구간으로 변환
- CAD 모델링에서 곡선 병합, 클리핑

---


## ✅ 언제 inverse를 이용하는가?
### 1. 재매개화된 Bezier 계수를 원래 구간으로 되돌릴 때
- 예: 어떤 곡선을 $[0,1]\rightarrow [0.25,0.75]$ 로 재매개화한 후
- 다시 $[0.25,0.75]\rightarrow [0,1]$ 로 되돌리고 싶을 때
```rust
let m = on_re_param_matrix(3, 0.0, 1.0, 0.25, 0.75);
let m_inv = on_re_param_matrix_inverse(3, 0.0, 1.0, 0.25, 0.75);
let c_new = Matrix::mul_vec(&m, &c);       // 재매개화된 계수
let c_orig = Matrix::mul_vec(&m_inv, &c_new); // 원래 계수 복원
```
→ 정확히 원래 곡선으로 복원됨

### 2. 곡선 병합 시 역변환 필요
- 여러 곡선을 하나로 병합할 때, 각 곡선의 구간을 정규화한 뒤
- 병합 후 다시 원래 구간으로 되돌릴 때 사용

### 3. 곡면 패치 역정규화
- 곡면을 정규화된 구간 [0,1]\times [0,1]에서 처리한 후
- 실제 좌표계로 되돌릴 때 u-방향, v-방향 각각에 역행렬 적용

### 4. 트리밍된 곡선 복원
- 트리밍된 곡선의 일부만 사용한 후, 전체 곡선으로 복원하고 싶을 때

## ✅ 핵심 요약: 재매개화 행렬과 역행렬의 용도 비교

| 상황 또는 목적                              | on_re_param_matrix 사용 | on_re_param_matrix_inverse 사용 |
|--------------------------------------------|--------------------------|----------------------------------|
| Bezier 곡선을 다른 구간으로 재매개화할 때     | ✅ 사용                   | ❌ 사용 안 함                     |
| 재매개화된 Bezier 계수를 원래 구간으로 복원할 때 | ❌ 사용 안 함             | ✅ 사용                           |
| 곡선의 일부 구간만 추출하고 싶을 때           | ✅ 사용                   | ❌ 사용 안 함                     |
| 정규화된 구간으로 변환 (예: [a,b] → [0,1])   | ✅ 사용                   | ❌ 사용 안 함                     |
| 정규화된 구간에서 원래 구간으로 되돌릴 때     | ❌ 사용 안 함             | ✅ 사용                           |
| 곡면 패치의 매개변수 방향을 재정렬할 때       | ✅ 사용                   | ✅ (역정규화 시)                  |
| 곡선 병합 후 각 곡선을 원래 구간으로 되돌릴 때 | ❌ 사용 안 함             | ✅ 사용                           |

---


##  재매개화 행렬 (on_re_param_matrix)
```rust
/// 재매개화 행렬  M  (Bezier 계수 변환:  c' = M · c)
/// Piegl의 B_REPMAT에 해당. 위의 reparam_affine(α,β) 사용.
/// 참고: Bezier(n)의 모노미얼로 확장해 α,β 적용 후 다시 Bezier로 투영하는 표준 구성.
pub fn on_re_param_matrix(p: usize, a: Real, b: Real, ap: Real, bp: Real) -> Vec<Vec<Real>> {
    let (alpha, beta) = on_re_param_affine(a, b, ap, bp);

    // Step 1: R 행렬 생성 — (αu' + β)^i 전개
    let mut r = vec![vec![0.0; p + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=i {
            let comb = binomial_usize(i, j) as f64;
            r[i][j] = comb * beta.powi((i - j) as i32) * alpha.powi(j as i32);
        }
    }

    // Step 2: Bezier → Power basis 변환 행렬 T
    let t = bezier_to_power_matrix(p);

    // Step 3: Power → Bezier basis 변환 행렬 P
    let p_mat = power_to_bezier_matrix(p);

    // Step 4: 최종 재매개화 행렬 M = P · R · T
    let rt = Matrix::mul(&r, &t);
    let m = Matrix::mul(&p_mat, &rt);
    m
}
```

## 역변환 (on_re_param_inverse_matrix)
```rust
pub fn on_re_param_inverse_matrix(p: usize, a: f64, b: f64, ap: f64, bp: f64) -> Vec<Vec<f64>> {
    // 1. affine 역변환 계수
    let (alpha, beta) = on_re_param_affine(ap, bp, a, b); // 역방향

    // 2. R⁻¹ 행렬 구성: (αt + β)^i 전개
    let mut r_inv = vec![vec![0.0; p + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=i {
            let comb = binomial_usize(i, j) as f64;
            r_inv[i][j] = comb * beta.powi((i - j) as i32) * alpha.powi(j as i32);
        }
    }

    // 3. T⁻¹ = power_to_bezier_matrix(p)
    let t_inv = power_to_bezier_matrix(p);

    // 4. P⁻¹ = bezier_to_power_matrix(p)
    let p_inv = bezier_to_power_matrix(p);

    // 5. nalgebra 로 행렬 곱: T⁻¹ · R⁻¹ · P⁻¹
    let r_na = DMatrix::from_row_slice(p + 1, p + 1, &r_inv.concat());
    let t_na = DMatrix::from_row_slice(p + 1, p + 1, &t_inv.concat());
    let p_na = DMatrix::from_row_slice(p + 1, p + 1, &p_inv.concat());

    let m_inv = t_na * r_na * p_na;

    // 6. DMatrix → Vec<Vec<f64>>
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
