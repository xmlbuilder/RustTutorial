# 📘 Axial Transformations (G_TRAAXL) — 수학적 의미
## 1. 목적
- Axial transformation은 NURBS 모델링에서 자주 사용되는 형상 변형(geometric deformation) 기법이다.
- 특징:
  - 변형은 한 축(X/Y/Z) 을 기준으로 한다.
  - 변형의 강도는 shape function f(t) 로 제어된다.
  - 변형 종류는 4가지:
    - PINCH : 특정 좌표만 scale
    - TAPER : 두 좌표를 scale
    - TWIST : 회전
    - SHEAR : 특정 좌표만 translate

## 2. Shape Function f(t)
- CFun은 B-spline 기반의 1D 함수:
```math
f(t)=\sum _{i=0}^nN_i^p(t)\, f_i
```
- $N_i^p(t)$: B-spline basis
- $f_i$: control coefficients
- Piegl의 N_cfnevn()과 동일한 방식으로 평가된다.

## 3. PINCH
- 특정 좌표만 scale:
- 예: XDIR + YCRD
```math
y'=y\cdot (af(x))
```
- 일반식:
```math
\mathrm{cor}'=\mathrm{cor}\cdot (af(\mathrm{dir}))
```
## 4. TAPER
- 두 좌표를 scale:
- 예: YDIR
```math
x'=x\cdot (af(y)),\quad z'=z\cdot (af(y))
```
- 일반식:
```math
\mathrm{other\  coords}'=\mathrm{other\  coords}\cdot (af(\mathrm{dir}))
```
## 5. TWIST
- 축을 기준으로 회전:
- 예: ZDIR
```math
\alpha =\pi af(z)
```
```math
\begin{aligned}x'&=x\cos \alpha -y\sin \alpha \\ y'&=x\sin \alpha +y\cos \alpha \end{aligned}
```

- 일반식:
```math
\mathrm{rotate\  around\  dir-axis\  by\  }\alpha =\pi af(\mathrm{dir})
```
## 6. SHEAR
- 특정 좌표만 translate:
- 예: XDIR + ZCRD
```math
z'=z+af(x)
```
- 일반식:
```math
\mathrm{cor}'=\mathrm{cor}+af(\mathrm{dir})
```
---

## 7. Curve Axial Deformation
- NURBS curve:
```math
C(u)=\frac{\sum _iN_i^p(u)P_i^{(w)}}{\sum _iN_i^p(u)w_i}
```
- control point 집합 $P_i^{(w)}$ 에 대해
- 각각 axial 변형을 적용하여 새로운 control net 생성:
```math
P_i^{(w)\, *}=\mathrm{AxialDeform}(P_i^{(w)})
```
- 새로운 곡선:
```math
C^*(u)=\frac{\sum _iN_i^p(u)P_i^{(w)\, *}}{\sum _iN_i^p(u)w_i}
```
- 즉, basis function과 knot vector는 변하지 않는다.

## 8. Surface Axial Deformation
- NURBS surface:
```math
S(u,v)=\frac{\sum _{i=0}^n\sum _{j=0}^mN_i^{p_u}(u)\, M_j^{p_v}(v)\, P_{i,j}^{(w)}}{\sum _{i=0}^n\sum _{j=0}^mN_i^{p_u}(u)\, M_j^{p_v}(v)\, w_{i,j}}
```
- control net은 row-major:
```math
\mathrm{idx}(u,v)=u+\mathrm{nu}\cdot v
```
- 각 control point에 대해:
```math
P_{i,j}^{(w)\, *}=\mathrm{AxialDeform}(P_{i,j}^{(w)})
```
- 새로운 surface:
```math
S^*(u,v)=\frac{\sum _{i,j}N_i^{p_u}(u)\, M_j^{p_v}(v)\, P_{i,j}^{(w)\, *}}{\sum _{i,j}N_i^{p_u}(u)\, M_j^{p_v}(v)\, w_{i,j}}
```
- 역시 basis와 knot vector는 변하지 않는다.

## 9. Summary

| Component        | Meaning                          |
|------------------|----------------------------------|
| t                | x, y, or z (depending on DIR)    |
| f(t)             | B-spline shape function value     |
| g = a * f(t)     | deformation amplitude             |
| PINCH            | cor' = cor * g                    |
| TAPER            | other_coords' = other_coords * g  |
| TWIST            | rotate by alpha = π * g           |
| SHEAR            | cor' = cor + g                    |

## 📌 설명
- t
- 변형 방향(DIR)에 따라 선택되는 좌표
  - XDIR → t = x
  - YDIR → t = y
  - ZDIR → t = z
  - f(t)
- CFun(B-spline function)으로 평가된 값
  ```math
  g = a * f(t)
  ```
  
- 변형 강도 (amplitude × shape function)
  - PINCH
    - 특정 좌표만 scale
    ```math
    cor' = cor * g
    ```
  - TAPER
    - 두 좌표를 scale
    ```math
    other_coords' = other_coords * g
    ```
  - TWIST
    - 축 기준 회전
    ```math
    alpha = π * g
    ```
  - SHEAR
    - 특정 좌표 translate
    ```math
    cor' = cor + g
    ```
---
