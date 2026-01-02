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



