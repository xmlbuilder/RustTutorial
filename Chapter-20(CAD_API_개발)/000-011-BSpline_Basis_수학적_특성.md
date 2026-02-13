# B-Spline Basis Function 특징

## B-Spline Basis Function의 성질 정리 (P2.1 ~ P2.6)
### P2.1 — Local Support (국소 지지 특성)
- B-spline 기저함수 $N_{i,p}(u)$ 는 다음 구간에서만 0이 아니다.
```math
\mathrm{supp}(N_{i,p})=[u_i,\; u_{i+p+1})
```
- 즉,
```math
N_{i,p}(u)=0\quad \mathrm{if\  }u\notin [u_i,u_{i+p+1})
```
- 예:
    - $N_{1,3}$은 $N_{1,0}$, $N_{2,0}$, $N_{3,0},N_{4,0}$ 의 조합이므로
    - 지지구간은 $[u_1,u_5)$ 이다.

![Local Support](/image/b-spline_basis_p2_1.png)

### P2.2 — 하나의 Knot Span에서 최대 p+1개의 함수만 비영
- 임의의 knot span $[u_j,u_{j+1}]$ 에서 0이 아닌 p-차 B-spline 함수는 최대 p+1개이며, 다음 함수들이다.
$N_{j-p,p},\; \dots ,\; N_{j,p}$
- 예:
  - 구간 $[u_3,u_4]$ 에서
  - 0차 함수는 $N_{3,0}$ 만 비영
  - 따라서 3차 함수는 $N_{0,3},N_{1,3},N_{2,3},N_{3,3}$ 만 비영 가능

![Local Support](/image/b-spline_basis_p2_2.png)

### P2.3 — Nonnegativity (비음수성)
- 모든 B-spline 기저함수는 항상 0 이상이다.
```math
N_{i,p}(u)\geq 0
```
- 증명은 p에 대한 귀납법으로 이루어진다.
    - p=0일 때는 정의상 자명
    - Cox–de Boor 재귀식이 비음수성을 유지하므로 귀납적으로 성립

### P2.4 — Partition of Unity (단위 분할)
- 임의의 knot span $[u_i,u_{i+1}]$ 에서 해당 구간에서 비영인 모든 p-차 B-spline 함수의 합은 항상 1이다.
```math
\sum _{j=i-p}^iN_{j,p}(u)=1
```
- 이는 Cox–de Boor 재귀식을 반복 적용하면 결국
```math
\sum _{j=i}^iN_{j,0}(u)=1
```
- 로 귀결되며, 0차 함수는 정의상 단위 분할을 만족한다.

### P2.5 — Knot에서의 연속성과 미분 가능성
- Knot span 내부에서는 $N_{i,p}(u)$ 는 다항식이므로 무한 미분 가능
- Knot u_k에서의 연속성은 knot의 중복도 m에 따라 결정됨
```math
\mathrm{연속성}=C^{\, p-m}
```
- 예:
    - 중복도 1 → $C^{p-1}$
    - 중복도 p → $C^0$
    - 중복도 p+1 → 불연속
- 또한 knot multiplicity는
    - knot vector에서의 중복도
    - 특정 basis function이 “보는” 중복도
- 두 가지 의미가 있다.

### P2.6 — 최대값의 유일성
- 차수가 1 이상일 때, 각 B-spline 기저함수 N_{i,p}(u)는 지지구간 내에서 정확히 하나의 최대값을 가진다.
- 즉, 하나의 봉우리를 가진 단봉형(unimodal) 함수이다.

- 추가: Knot 중복의 영향
- 예시 knot vector:
```math
U=\{ 0,0,0,1,2,3,4,4,5,5\}
``` 
- 각 함수가 “보는” knot multiplicity는 다르다.
- 예: u=0에서
- $N_{0,2}$: multiplicity 3 → 불연속
- $N_{1,2}$: multiplicity 2 → $C^0$
- $N_{2,3}$: multiplicity 1 → $C^1$
- 또한 중복도가 높을수록 지지구간이 짧아지고, 함수가 비영인 구간이 줄어든다.
- 예: $N_{3,2}$는 $[4,5]$ 에서만 비영.

- $N_{0,2}:\{ \, 0,\; 0,\; 0,\; 1\, \}$ 
- $N_{1,2}:\{ \, 0,\; 0,\; 1,\; 2\, \}$ 
- $N_{2,2}:\{ \, 0,\; 1,\; 2,\; 3\, \}$ 
- $N_{5,2}:\{ \, 3,\; 4,\; 4,\; 5\, \}$
- $N_{6,2}:\{ \, 4,\; 4,\; 5,\; 5\, \}$ 

---
