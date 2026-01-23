## on_integrate_simpson
- 이 두 함수는 적응형(adaptive) Simpson 적분을 **누적 방식(cumulative refinement)** 으로 구현한 고급 버전
- 일반적인 재귀형 adaptive Simpson과 달리, 레벨을 올릴 때마다 기존 계산을 재활용하는 방식이라 매우 효율적이고,  
    수치적으로도 안정적이다.

### 📘 on_integrate_simpson
- 적응형 Simpson 적분 (누적 refinement, Richardson-like correction 포함)
- 이 함수는 구간 [a, b]에서 함수 f(x)를 적분한다.
- 핵심 아이디어는:
    - 레벨이 증가할 때마다 **새로운 중점(midpoints)** 만 평가
    - 이전 레벨의 midpoint 합을 절반만 빼고 새 midpoint를 더해 Simpson 계수를 갱신
    - Simpson 추정값의 변화량이 rel_tol 이하가 되면 조기 종료

### 1️⃣ 전체 알고리즘 흐름
- ✔ level = 0
- 초기 trapezoid 형태의 Simpson 기반 값:
```rust
weighted_sum = f(a) + f(b)
simpson = 0.5 * (b - a) * weighted_sum
```

- 여기서는 Simpson이 아니라 T0(사다리꼴) 형태지만, 이후 refinement에서 Simpson 형태로 수렴한다.

- ✔ level ≥ 1
- 레벨이 증가할 때마다:
- ① 새로운 midpoints 평가
- 레벨 L에서 midpoint 개수:
```math
\mathrm{mid\_ count}=2^{L-1}
```
- 코드:
```rust
let mid_count = 1 << (level - 1);
let mid_step = (b - a) / mid_count;
```

- ② 새 midpoint 합 계산
```rust
mid_sum += f(x);
```

- ③ Simpson 누적 갱신
- 핵심 공식:
```rust
weighted_sum += mid_sum4 - 0.5 * prev_mid_sum4;
```

- 여기서:
    - mid_sum4 = 4 * mid_sum
    - 이전 레벨 midpoint 기여분의 절반을 제거
    - 새 midpoint 기여분을 추가
    - Simpson 계수(1, 4, 2, 4, 2, …)를 누적 방식으로 유지하는 핵심
- ④ Simpson 값 계산
```rust
simpson = (b - a) * weighted_sum / ((1 << level) as f64 * 3.0);
```

- 이 식은 Simpson 규칙의 일반화된 형태:
```math
S_L=\frac{b-a}{3\cdot 2^L}\cdot \mathrm{weighted\_ sum}
```

- ✔ 수렴 검사
- 레벨 5 이후부터:
```rust
*last_delta_out = |simpson - prev_simpson|
if last_delta <= rel_tol * |prev_simpson|:
    return simpson
```

- 즉, 상대 오차 기반 수렴 조건.

### 2️⃣ 출력
- simpson: 적분값
- eval_count_out: f(x) 평가 횟수
- last_delta_out: 마지막 변화량
- 조기 종료 또는 max_levels까지 반복

### 📘 on_integrate_simpson_simple
- 편의 함수
```rust
pub fn on_integrate_simpson_simple<F>(f: F, a: f64, b: f64, rel_tol: f64) -> f64
```

- max_levels = 20
- 평가 횟수나 delta는 무시
- 단순히 적분값만 반환

### 📌 이 구현의 장점
- ✔ 1) 재귀가 아닌 반복 → stack-safe
    - 일반 adaptive Simpson은 재귀 깊이가 커질 수 있지만, 이 방식은 반복문 기반이라 안전하다.
- ✔ 2) midpoint 재활용 → 매우 빠름
    - 레벨이 올라갈 때마다 새 midpoint만 평가하므로 효율적.
- ✔ 3) Simpson 계수 누적 방식 → 수치적으로 안정
    - weighted_sum += mid_sum4 - 0.5 * prev_mid_sum4
    - 이 한 줄이 이 알고리즘의 핵심이다.
- ✔ 4) 상대 오차 기반 수렴
    - 절대 오차보다 훨씬 실용적.
- ✔ 5) max_levels 제한으로 무한 루프 방지

### 📌 최종 요약
- 이 함수는:
    - 적응형 Simpson 적분의 고급 누적 버전
    - midpoint 재활용으로 매우 빠름
    - 수렴 조건도 robust
    - 실전 CAD/Geometry/Physics 엔진에서 쓰기 좋은 구조


```rust
// on_integrate_simpson (적응형 Simpson, 누적 방식)
pub fn on_integrate_simpson<F>(
    mut f: F,
    a: f64,
    b: f64,
    rel_tol: f64,
    max_levels: i32,
    eval_count_out: &mut i32,
    last_delta_out: &mut f64,
) -> f64
where
    F: FnMut(f64) -> f64,
{
    let mut simpson = 0.0_f64;
    let mut prev_simpson = 0.0_f64;
    let mut prev_mid_sum4 = 0.0_f64;
    let mut weighted_sum = 0.0_f64;
    *eval_count_out = 0;
    *last_delta_out = f64::INFINITY;

    for level in 0..=max_levels {
        if level == 0 {
            weighted_sum = f(a) + f(b);
            *eval_count_out = 2;

            // Initial T0 (reference)
            simpson = 0.5 * (b - a) * weighted_sum;
        } else {
            let mid_count = 1 << (level - 1);
            let mid_step = (b - a) / (mid_count as f64);
            let mut x = a + 0.5 * mid_step;
            let mut mid_sum = 0.0;
            for _ in 0..mid_count {
                mid_sum += f(x);
                x += mid_step;
            }
            *eval_count_out += mid_count as i32;

            let mid_sum4 = 4.0 * mid_sum;

            // Add 4 new midpoints and apply correction by subtracting half of the previous midpoint sum (-0.5 * prev_mid_sum4)
            weighted_sum += mid_sum4 - 0.5 * prev_mid_sum4;

            simpson = (b - a) * weighted_sum / ((1 << level) as f64 * 3.0);

            if level >= 5 {
                *last_delta_out = (simpson - prev_simpson).abs();
                if *last_delta_out <= rel_tol * prev_simpson.abs() {
                    return simpson;
                }
            }
            prev_mid_sum4 = mid_sum4;
            prev_simpson = simpson;
        }
    }
    simpson
}
```
```rust
pub fn on_integrate_simpson_simple<F>(f: F, a: f64, b: f64, rel_tol: f64) -> f64
where
    F: FnMut(f64) -> f64,
{
    let mut n = 0;
    let mut d = 0.0;
    on_integrate_simpson(f, a, b, rel_tol, 20, &mut n, &mut d)
}
```
## on_polynomial_f_df

- 이 구현은 Horner 방식으로 다항식 f(u)와 도함수 f′(u)를 동시에 계산하는 정석적인 패턴
- 게다가 계수 배열이 오름차순(ascending) 또는 내림차순(descending) 어느 방식으로 저장되어 있어도 처리할 수 있도록 설계.


### 📘 on_polynomial_f_df(a, u, ascending)
- Horner 방식으로 다항식 f(u)와 f′(u)를 동시에 계산
- 다항식이 다음 형태일 때:
- ascending = true
```math
f(u)=a_0+a_1u+a_2u^2+\dots +a_nu^n
```
- ascending = false
```math
f(u)=a_0+a_1u+a_2u^2+\dots +a_nu^n
```
- (단, 배열이 내림차순 저장: a[0] = 최고차항)
- 즉, 계수 배열의 저장 순서만 다르고, 다항식 자체는 동일한 형태다.
### 1️⃣ ascending = true (계수가 낮은 차수부터 저장)
- 예: [a0, a1, a2, ..., an]Horner를 역순으로 적용해야 한다:
```rust
let mut f = a[n];
let mut df = 0.0;
```
```rust
for k in (0..n).rev() {
    df = df * u + f;
    f  = f  * u + a[k];
}
```
- 이 패턴은 다음을 만족한다:
- f = Horner로 계산된 f(u)
- df = Horner로 계산된 f′(u)
- 도함수의 Horner 전개는:
```math
f'(u)=a_1+2a_2u+3a_3u^2+\dots +na_nu^{n-1}
```
Horner 변환하면:
```math
\begin{aligned}f&=a_n\\ df&=0\\ \mathrm{반복:\  }df&=df\cdot u+f\\ f&=f\cdot u+a_k\end{aligned}
```
이 공식이 정확히 코드와 일치한다.

### 2️⃣ ascending = false (계수가 높은 차수부터 저장)
- 예: [a0, a1, a2, ..., an] 이지만 a0이 최고차항.
이 경우 Horner를 정방향으로 적용하면 된다:
```rust
let mut f = a[0];
let mut df = 0.0;

for k in 1..=n {
    df = df * u + f;
    f  = f  * u + a[k];
}
```
- 이 역시 Horner의 도함수 동시 계산 공식과 정확히 일치한다.
### 3️⃣ 빈 배열 처리
```rust
if a.is_empty() {
    return (0.0, 0.0);
}
```
- f(u) = 0
- f′(u) = 0
- 정확한 처리.
### 4️⃣ 전체적인 성능
- Horner 방식 → O(n), 빠르고 안정적
- f와 f′를 동시에 계산 → 불필요한 반복 없음
- ascending/descending 모두 지원
- derivative 계산이 Horner 내부에 자연스럽게 포함
- 분기 구조가 명확하고 안전
- 이건 CAD/Geometry 엔진에서 곡선 평가, Newton iteration, root finding 등에 바로 쓰는 패턴.
### 📌 최종 요약
- 이 함수는:
    - Horner 방식으로 f(u)와 f′(u)를 동시에 계산
    - 계수 배열이 오름차순/내림차순 어느 방식이든 처리
    - 빠르고 수치적으로 안정
    - Newton solver와 spline/curve 평가에 최적

```rust
// ------------------------------------------------------------
// Polynomial f(u) and its derivative f'(u) using Horner's method.
// `a` is the coefficient array.
// If `ascending = true`, the form is: a[0] + a[1] * u + ... + a[n] * u^n
// ------------------------------------------------------------
pub fn on_polynomial_f_df(a: &[f64], u: f64, ascending: bool) -> (f64, f64) {
    let n = a.len().wrapping_sub(1);
    if a.is_empty() {
        return (0.0, 0.0);
    }

    if ascending {
        // Reverse Horner (ascending order of coefficients)
        let mut f = a[n];
        let mut df = 0.0;
        for k in (0..n).rev() {
            df = df * u + f;
            f = f * u + a[k];
        }
        (f, df)
    } else {
        // Stored in descending order: a[0] + a[1] * u + ...
        let mut f = a[0];
        let mut df = 0.0;
        for k in 1..=n {
            df = df * u + f;
            f = f * u + a[k];
        }
        (f, df)
    }
}
```
---
