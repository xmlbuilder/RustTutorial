## 🔹 NURBS 곡선 정의
- 일반적인 NURBS 곡선은 차수 $p$, 제어점 $P_i$, 가중치 $w_i$, knot vector U로 정의됩니다:

$$
C(u)=\frac{\displaystyle \sum _{i=0}^nN_{i,p}(u)\, w_iP_i}{\displaystyle \sum _{i=0}^nN_{i,p}(u)\, w_i},\quad u\in [U_p,U_{n+1}]
$$

- 여기서:
  - $N_{i,p}(u)$ = B-스플라인 basis function of degree p
  - $w_i$ = weight
  - $P_i$ = Euclidean control point

## 🔹 동차 좌표 표현
- 동차 좌표로 쓰면:

$$
R_i(u)=N_{i,p}(u) w_i
$$

$$
C(u)=\frac{\displaystyle \sum _{i=0}^nR_i(u)P_i}{\displaystyle \sum _{i=0}^nR_i(u)}
$$

## 🔹 도함수 공식 (Piegl & Tiller, The NURBS Book)
- 0차 (곡선 자체)

$$
C^{(0)}(u)=C(u)
$$

- 1차 도함수

$$
C'(u)=\frac{1}{w(u)}\left( P'(u)-w'(u)C(u)\right) 
$$

- 여기서:

$$
P'(u)=\sum _iN'_{i,p}(u) w_iP_i
$$

$$
w'(u)=\sum _iN'_{i,p}(u) w_i
$$

$$
w(u)=\sum _iN_{i,p}(u) w_i
$$

- 2차 도함수

$$
C''(u)=\frac{1}{w(u)}\left( P''(u)-2w'(u)C'(u)-w''(u)C(u)\right) 
$$


- 일반 k차 도함수 (재귀 공식)

$$
C^{(k)}(u)=\frac{1}{w(u)}\left( P^{(k)}(u)-\sum _{i=1}^k{k \choose i}\, w^{(i)}(u)\, C^{(k-i)}(u)\right)
$$

- 여기서:

$$
P^{(k)}(u)=\sum _iN_{i,p}^{(k)}(u) w_iP_i
$$

$$
w^{(k)}(u)=\sum _iN_{i,p}^{(k)}(u) w_i
$$

- ${k \choose i}$ = 이항계수


## 🔹 곡률 (Curvature)
- 곡률은 곡선이 얼마나 휘어져 있는지를 나타내는 값입니다.
- NURBS 곡선의 도함수를 이용하면 다음과 같이 정의됩니다:

$$
\kappa (u)=\frac{\| C'(u)\times C''(u)\| }{\| C'(u)\| ^3}
$$

- $C'(u)$: 곡선의 1차 도함수 (속도 벡터)
- $C''(u)$: 곡선의 2차 도함수 (가속도 벡터)
- 분자: 1차와 2차 도함수의 벡터곱 크기 → 곡선의 휘어짐 정도
- 분모: 속도의 크기의 세제곱 → 곡률을 정규화

## 🔹 비틀림 (Torsion)
- 비틀림은 곡선이 평면을 벗어나서 공간적으로 꼬이는 정도를 나타냅니다.
- NURBS 곡선의 도함수를 이용하면 다음과 같이 정의됩니다:

$$
\tau (u)=\frac{(C'(u)\times C''(u))\cdot C'''(u)}{\| C'(u)\times C''(u)\| ^2}
$$

- $C'(u)$: 1차 도함수
- $C''(u)$: 2차 도함수
- $C'''(u)$: 3차 도함수
- 분자: 곡선의 비틀림을 나타내는 삼중곱 (벡터곱과 내적)
- 분모: 곡선이 평면에 머무르는지 여부를 판별하는 벡터곱의 크기 제곱

## ✅ 요약
- 곡률 $\kappa (u)$: 곡선의 휘어짐 정도 → 원의 경우 반지름 R에 대해 $\kappa =1/R$.
- 비틀림 $\tau (u)$: 곡선이 평면을 벗어나 꼬이는 정도 → 원호에서는 0, 헬릭스에서는 양수 값.

---

