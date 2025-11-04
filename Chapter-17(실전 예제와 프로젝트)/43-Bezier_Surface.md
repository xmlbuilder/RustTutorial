# Bezier Surface
## ✅ 주요 기능 점검 및 수식 정리
### 1. from_ctrl_grid / with_degrees
- ctrl의 크기에서 차수를 추론하거나 명시적으로 설정
- 직사각형 여부 검증 포함 → 정상

### 2. evaluate(u, v)
기능: 베지어 곡면의 점 평가
수식:

$$
S(u,v)=\frac{\sum _{i=0}^p\sum _{j=0}^qB_i^p(u)B_j^q(v)P_{ij}w_{ij}}{\sum _{i=0}^p\sum _{j=0}^qB_i^p(u)B_j^q(v)w_{ij}}
$$

- bernstein_all_clamped(p, u) → $B_i^p(u)$
- 동차 좌표로 누적 후 유클리드 변환 → 정확

### 3. elevate_u / elevate_v
기능: u 또는 v 방향 차수 상승
수식:

$$
P_i'=\sum _{k=\max (0,i-inc)}^{\min (p,i)}E_{ik}P_k
$$ 

- degree_elev_matrix(p, inc) → 차수 상승 행렬 E
- 각 열 또는 행에 대해 적용 → 정확

### 4. split_u / split_v
기능: u 또는 v 방향 분할 (de Casteljau 알고리즘)
수식:
- 1D 곡선 분할을 각 열/행에 적용
- split_curve_lerp() 사용 → 선형 보간 기반 분할 → 정상

### 5. elevate_degree_dir(dir, inc)
기능: 방향에 따라 차수 상승
- SurfaceDir::UDir 또는 VDir에 따라 elevate_u / elevate_v와 동일한 로직 수행 → 정상

### 6. to_power_basis(a, b, c, d)
기능: 베지어 곡면을 power basis로 변환
수식 흐름:
- 베지어 → power basis 변환 행렬:  

$$
P_{\mathrm{UM}} = \mathrm{power\_basis\_matrix}(p), \quad P_{\mathrm{VM}} = \mathrm{power\_basis\_matrix}(q)
$$


- 구간 재매핑:

$$
RUM=\mathrm{reparam_matrix}(p,a,b,0,1),\quad RVM=\mathrm{reparam_matrix}(q,c,d,0,1)
$$

- 전체 변환 행렬:

$$
CUM=RUM\cdot PUM,\quad CVM=RVM\cdot PVM
$$

- 최종 변환:

$$
BW_{ij}=\sum _{u=0}^p\sum _{v=0}^qCUM_{iu}\cdot CVM_{jv}\cdot P_{uv}
$$

- 동차 좌표로 누적 → 정확하고 수학적으로 타당

## 🧪 테스트 제안
- evaluate()에서 u, v = 0, 1, 0.5 등 경계값 테스트
- elevate_u() 후 evaluate() 결과 비교
- split_u() 후 두 곡면의 evaluate() 합이 원래와 일치하는지 확인
- to_power_basis() 결과를 수치적으로 검증


## ✅ bernstein_all_clamped(p, u)
### 📌 역할
- 차수 p에 대해 클램핑된 Bernstein basis 함수 $B_i^p(u)$ 전체를 계산
- 반환값: Vec<Real> 형태의 $[B_0^p(u),B_1^p(u),...,B_p^p(u)]$

### 📐 수식 정의
Bernstein basis 함수:

$$
B_i^p(u)={p \choose i}u^i(1-u)^{p-i}
$$

- 이 구현은 재귀적 누적 방식으로 계산하여 수치적으로 안정적이고 효율적입니다.

### ⚙️ 코드 검토
- u=0 또는 u=1일 때 특수 처리 → OK
- 누적 방식으로 basis 계산 → OK
- saved와 omu = 1 - u를 활용한 안정적 계산 → OK

## ✅ split_curve_lerp(a, t)
### 📌 역할
- 1D 베지어 곡선을 매개변수 t\in [0,1]에서 분할
- 입력: 제어점 배열 a (복사본)
- 출력: (left, right) → 각각 [0,t], [t,1] 구간의 제어점

### 📐 수식 설명 (de Casteljau 알고리즘)
- 반복적으로 선형 보간:

$$
P_i^{(k)}=(1-t)P_i^{(k-1)}+tP_{i+1}^{(k-1)}
$$

- 최종적으로:

$$
\mathrm{left}[k]=P_0^{(k)},\quad \mathrm{right}[p-k]=P_{p-k}^{(k)}
$$

## ⚙️ 코드 검토
- left[0] = a[0], right[p] = a[p] → 시작/끝점 설정 OK
- 내부 루프에서 a[i] = a[i].lerp(a[i+1], t) → 선형 보간 OK
- left[k] = a[0], right[p-k] = a[p-k] → 누적 결과 저장 OK

---




