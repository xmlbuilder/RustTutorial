# Weingarten matrix
- Weingarten matrix(바인가르텐 행렬)은 곡면의 곡률(curvature)을 결정하는 핵심 기하학적 객체.
- NURBS, 미분기하, 컴퓨터 그래픽스, FEM 쉘 요소 등에서 모두 등장하는 아주 중요한 개념.
- 아주 쉽게 말하면:
    - 곡면에서 법선벡터가 어떻게 변하는지를 나타내는 2×2 선형 변환 이라고 보면 된다.

## 🌟 1. Weingarten map / Weingarten matrix의 정의
- 곡면 S(u,v)가 있을 때, 단위 법선벡터를 n(u,v)라고 하자.
- Weingarten map(Shape operator)은 다음 선형 변환이다:
```math
dn:T_pS\rightarrow T_pS
```
- 즉,
    - 법선벡터의 변화율을 tangent plane 위에서 표현한 것
- 이걸 행렬로 표현한 것이 Weingarten matrix다.

## 🌟 2. 수식으로 보면 더 명확해짐
- 곡면의 1차 미분:
```math
S_u=\frac{\partial S}{\partial u},\quad S_v=\frac{\partial S}{\partial v}
```
- 곡면의 2차 미분:
```math
S_{uu},\; S_{uv},\; S_{vv}
```
- 법선벡터:
```math
n=\frac{S_u\times S_v}{\| S_u\times S_v\| }
```
- 법선의 변화율:
```math
n_u=\frac{\partial n}{\partial u},\quad n_v=\frac{\partial n}{\partial v}
```
- Weingarten matrix W는 다음을 만족하는 2×2 행렬이다:
```math
\left[ \begin{matrix}n_u\\ n_v\end{matrix}\right] =-W\left[ \begin{matrix}S_u\\ S_v\end{matrix}\right]
``` 
- 즉,
```math
W=-\left[ \begin{matrix}n_u\cdot S_u&n_u\cdot S_v\\ n_v\cdot S_u&n_v\cdot S_v\end{matrix}\right] 
```
## 🌟 3. Weingarten matrix의 기하학적 의미
- Weingarten matrix는 곡률을 결정하는 핵심 행렬이다.
- ✔ 고유값 = 주곡률(principal curvatures)
```math
\kappa _1,\kappa _2=\mathrm{eigenvalues}(W)
```
- ✔ 고유벡터 = 주곡률 방향(principal directions)
- ✔ trace = 평균곡률(mean curvature)
```math
H=\frac{\kappa _1+\kappa _2}{2}
```
- ✔ determinant = 가우스 곡률(Gaussian curvature)
```math
K=\kappa _1\kappa _2
```
- 즉, 곡면의 모든 곡률 정보가 Weingarten matrix 하나에 들어 있다.

## 🌟 4. 왜 중요한가?
- ✔ 1) 곡률 계산의 핵심
    - NURBS surface에서 곡률을 구하려면 반드시 Weingarten matrix가 필요하다.
- ✔ 2) FEM 쉘/판 요소
    - 쉘 요소의 bending stiffness는 곡률 텐서에서 나오는데, 그게 바로 Weingarten matrix다.
- ✔ 3) 컴퓨터 그래픽스
    - shading
    - normal mapping
    - curvature flow
    - mesh smoothing
- 모두 Weingarten matrix 기반.
- ✔ 4) CAD/CAM
    - tool path curvature
    - offset surface
    - surface blending
    - trimming
- 이런 연산에서 곡률이 필요하고, 그 곡률은 Weingarten matrix에서 나온다.

## 🌟 5. 직관적으로 이해하면
- 곡면 위에서 걸어가면 법선벡터가 계속 바뀌지?
- 그 변화율을 “선형 변환”으로 표현한 것이 Weingarten matrix.
    - 법선이 빨리 변하면 → 곡률이 크다
    - 법선이 거의 안 변하면 → 곡률이 작다
    - 두 방향에서 변화율이 다르면 → 두 개의 주곡률이 생긴다
- 이걸 수학적으로 정리한 것이 Weingarten matrix.

## 🌟 6. 요약
- Weingarten matrix는:
    - 곡면의 법선벡터 변화율을 나타내는 2×2 행렬
    - 곡률 텐서(curvature tensor)와 동일
    - 고유값 = 주곡률
    - trace = 평균곡률
    - determinant = 가우스 곡률
    - NURBS surface의 곡률 계산에 필수

---
