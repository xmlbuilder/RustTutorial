# 스플라인 


---

# 1. B-spline 기초 루틴과 불변식

## 1.1 매듭 벡터와 클램프
- **클램프(Open)**: 양 끝 매듭의 다중도 $\(\ge p+1\)$.
- **균등(Uniform)**: (정규화 후) 내부 간격이 동일(허용오차 내).
- **타입 예시**: ClampedUniform, ClampedNonUniform, UnClampedUniform, UnClampedNonUniform.

### 다중도(multiplicity)
- 값 $\(x=U_i\)$ 의 다중도는 **좌우로 동일값을 스캔**하여 개수 집계.  
- 실수 비교는 스케일된 허용오차로:

$$
  \texttt{close}(a,b): |a-b|\le \varepsilon\cdot\max(1,|a|,|b|),\ \varepsilon\approx10^{-12}.
$$

### `is_clamped`/`is_valid_with_pn`
- `is_valid_with_pn(U,p,n)`: 관계 $\(m=n+p+1\)$ 확인 및 비감소성 체크.
- `is_clamped(U,p)`: 좌/우 끝 다중도 $\(\ge p+1\)$ 여부 반환.  
  (테스트 호출 시 **항상 `m = U.len()-1`**를 내부에서 사용)

## 1.2 스팬 찾기 (Piegl & Tiller, Def. 2.1)
- $\(\mathrm{find\_span}(U,n,p,u)\)$:  
  **최대의** $\(i\) s.t. \(U[i]\le u < U[i+1]\)$; 단, $\(u=U[n+1]\)$ 이면 $\(i=n\)$.
- 이 정의를 따르면 경계 $\(u=1\)$ 에서 올바른 스팬을 얻음.

## 1.3 Basis & 도함수 — Algorithm A2.3 (DersBasisFuns)
- NDU 삼각 테이블로 $\(N_{i-p+r,p}(u)\)$ 구축.
- 1차 이상 미분은 보조 테이블 $\(a\)$ 와 경계

$$
  j_1=\begin{cases}
  1,& r-k\ge -1\\
  - (r-k),& \text{else}
  \end{cases},\qquad
  j_2=\begin{cases}
  k-1,& r-1\le p-k\\
  p-r,& \text{else}
  \end{cases}
$$

  을 사용하여 내부 합을 정확히 집계.  
- **반복 매듭 분모 0 보호**: $\(U_{i+p}-U_i=0\)$ 같은 항은 **0으로 처리**.

### 불변식(테스트 아이덴티티)
- **Partition of unity:** $\(\sum_j N_{j,p}(u)=1\)$.  
- **도함수 합 0:** $\(\sum_j N'_{j,p}(u)=0\)$ (위 성질을 미분).  
  ⇒ 구현에서 마지막 열 $\(r=p\)$ 갱신 누락/경계 off-by-one 시 이 합이 $\(\pm 1/h\)$ 로 어긋남.

## 2. 구현 체크리스트 / 팁

- **실수 비교**: `close(a,b)`는 절대+상대 혼합(스케일) 오차 사용.  
- **비매끈 함수**: 0을 가로지르면 **자동 분할**.  
- **B-spline**: 반복 매듭에서 **분모 0 가드**. `find_span`의 경계 정의 엄수.  
- **테스트**:  
  - NURBS/B-spline: $\(\sum N=1\), \(\sum N'=0\)$ 전 구간 샘플.

---

### 참고(교과서)
- Piegl & Tiller, *The NURBS Book*, 2e — Alg. A2.3 (DersBasisFuns), span 정의.  
