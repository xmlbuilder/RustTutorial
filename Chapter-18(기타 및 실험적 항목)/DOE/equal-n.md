# equal-n / unequal-n
## 📊 차이 설명
- equal-n (균등 표본 수)
- 모든 그룹의 표본 수가 동일:

$$
n_1=n_2=\cdots =n_a=n
$$

- 계산이 단순해지고, 결정선(UDL/LDL)도 모든 그룹에 동일한 마진을 적용할 수 있습니다.
- 마진 공식:
  - 모든 그룹에 동일한 값.
- unequal-n (불균등 표본 수)
  - 그룹마다 표본 수가 다름:

$$
n_1\neq n_2\neq \cdots \neq n_a
$$

  - 그룹별로 표본 수가 달라서, 각 그룹 평균의 신뢰구간 폭이 달라집니다.
- 마진 공식:
  - 그룹별 $n_i$ 에 따라 달라짐.

## ✅ 정리
- equal-n: 모든 그룹 표본 수가 같아 단순화된 공식을 사용.
- unequal-n: 그룹별 표본 수가 달라서 각 그룹마다 다른 마진을 적용해야 함.

---

## h 값의 영향

equal‑n / unequal‑n 상황에서 ANOM 결정선에 들어가는 h 값이 어떻게 달라지고, 어떤 영향을 미치는지를 수식으로 정리.

## 📐 기본 구조
ANOM에서 각 그룹 평균의 결정선은 다음과 같이 표현됩니다:

- $\bar {Y}$: 전체 평균 (grand mean)
- $s$: 그룹 내 표준편차 (pooled within-group std)
- $n_i$: 그룹 i의 표본 수
- $h$: ANOM 상수 (critical constant)

## 📊 equal‑n (모든 그룹 표본 수 동일)

$$
n_1=n_2=\cdots =n_a=n
$$

- 이 경우, 모든 그룹에 동일한 마진이 적용됩니다:
- 그리고 h는 Bonferroni 보정 기반으로 근사할 수 있습니다:

- $a$: 그룹 수
- $t_{\mathrm{crit}}$: 자유도 df=a(n-1)에 대한 t-분포 임계값
- 즉, 그룹 수가 많아질수록  항 때문에 h가 커져서 결정선 폭이 넓어집니다.

## 📊 unequal‑n (그룹별 표본 수 다름)

$$
n_1,n_2,\ldots ,n_a\quad \mathrm{서로\  다름}
$$

- 이 경우, 그룹별 마진은 다음과 같이 달라집니다:
- 즉, h 대신 직접 $t_{\mathrm{crit}}$ 를 사용하며, 그룹마다 $\sqrt{1/n_i}$ 가 달라지므로 표본 수가  
  작은 그룹일수록 마진이 커지고, 큰 그룹일수록 마진이 작아집니다.

## 🔎 h의 영향 요약
- equal‑n:
  - 모든 그룹에 동일한 마진 적용
  - 그룹 수가 많아질수록 h가 커져서 결정선 폭이 넓어짐
- unequal‑n:
  - 그룹별 n_i에 따라 마진이 달라짐
  - 작은 그룹은 결정선 폭이 넓어져 더 관대하게 판정
  - 큰 그룹은 결정선 폭이 좁아져 더 엄격하게 판정

## ✔ Rust 구현에서의 처리 방식 요약

- Rust 코드에서:
```rust
if equal_n && bonferroni {
    margin_i = h * s_within;
} else {
    margin_i = tcrit * s_within * sqrt(1.0 / n_i);
}
```
- equal_n → 모든 n_i 가 동일 → h 사용
- unequal_n → tcrit × sqrt(1/nᵢ) 사용

## 최종 요약

| 구분         | equal-n                  | unequal-n                          |
|--------------|--------------------------|------------------------------------|
| 조건         | 모든 그룹 n 동일          | nᵢ 불일치                          |
| Margin       | h × s                    | tcrit × s × sqrt(1/nᵢ)             |
| UDL/LDL      | 공통 수평선               | 그룹마다 다른 높이                  |
| 통계적 특징  | 단순, 전통적 ANOM         | 더 일반적, 정확도 높음              |
| Rust 구현    | anom_h_bonferroni_equal_n | sqrt(1/nᵢ) 기반 margin              |

---

