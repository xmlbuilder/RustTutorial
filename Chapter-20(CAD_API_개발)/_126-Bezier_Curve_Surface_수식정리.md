# 📘 BezierCurve & BezierSurface — 수식 및 의미 정리 문서

## 1. Bezier Curve
### 1.1 정의
- 차수 p 의 Bezier Curve:
```math
C(t)=\sum _{i=0}^pP_i\, B_{i,p}(t),\quad t\in [0,1]
```
- 여기서:
  - $P_i$: control point (Point3D 또는 Point4D)
  - $B_{i,p}(t)$: Bernstein basis function
```math
B_{i,p}(t)={p \choose i}t^i(1-t)^{p-i}
```
### 1.2 성질
- ✔ Convex Hull Property
```math
C(t)\in \mathrm{ConvHull}(P_0,\dots ,P_p)
```
- ✔ End-point Interpolation
```math
C(0)=P_0,\quad C(1)=P_p
```
- ✔ Variation Diminishing
  - 곡선은 control polygon보다 더 많이 진동하지 않는다.

### 1.3 미분
- 1차 미분
```math
C'(t)=p\sum _{i=0}^{p-1}(P_{i+1}-P_i)B_{i,p-1}(t)
```
- 2차 미분
```math
C''(t)=p(p-1)\sum _{i=0}^{p-2}(P_{i+2}-2P_{i+1}+P_i)B_{i,p-2}(t)
```

### 1.4 De Casteljau 알고리즘
- 재귀적 정의:
```math
P_i^{(0)}=P_i
```
```math
P_i^{(k)}(t)=(1-t)P_i^{(k-1)}(t)+tP_{i+1}^{(k-1)}(t)
```
- 최종:
```math
C(t)=P_0^{(p)}(t)
```
### 1.5 곡선 분할 (Split)
- De Casteljau 삼각형을 이용해:
- Left curve: $Q_i=P_0^{(i)}$
- Right curve: $R_i=P_{p-i}^{(i)}$

### 1.6 차수 상승 (Degree Elevation)
```math
C(t)=\sum _{i=0}^pP_iB_{i,p}(t)=\sum _{i=0}^{p+1}P'_iB_{i,p+1}(t)
```
- 새 control point:
```math
P'_i=\frac{i}{p+1}P_{i-1}+\frac{p+1-i}{p+1}P_i
```
### 1.7 Rational Bezier Curve
- Point4D 사용:
```math
C(t)=\frac{\sum P_i^wB_{i,p}(t)}{\sum w_iB_{i,p}(t)}
```

## 2. Bezier Surface
### 2.1 정의 (Tensor Product Surface)
- 차수 p,q 의 Bezier Surface:
```math
S(u,v)=\sum _{i=0}^p\sum _{j=0}^qP_{i,j}\, B_{i,p}(u)\, B_{j,q}(v)
```
- $P_{i,j}$: control net
- $B_{i,p}(u)$: u 방향 Bernstein
- $B_{j,q}(v)$: v 방향 Bernstein

### 2.2 성질
- ✔ Convex Hull
```math
S(u,v)\in \mathrm{ConvHull}(P_{i,j})
```
- ✔ Corner Interpolation
```math
S(0,0)=P_{0,0},\quad S(1,0)=P_{p,0},\quad S(0,1)=P_{0,q},\quad S(1,1)=P_{p,q}
```
### 2.3 부분 미분
- 1차 미분
```math
S_u(u,v)=\sum _{i=0}^{p-1}\sum _{j=0}^qp(P_{i+1,j}-P_{i,j})B_{i,p-1}(u)B_{j,q}(v)
```
```math
S_v(u,v)=\sum _{i=0}^p\sum _{j=0}^{q-1}q(P_{i,j+1}-P_{i,j})B_{i,p}(u)B_{j,q-1}(v)
```
- 2차 미분
```math
S_{uu}(u,v)=p(p-1)\sum (P_{i+2,j}-2P_{i+1,j}+P_{i,j})B_{i,p-2}(u)B_{j,q}(v)
```
```math
S_{vv}(u,v)=q(q-1)\sum (P_{i,j+2}-2P_{i,j+1}+P_{i,j})B_{i,p}(u)B_{j,q-2}(v)
```
```math
S_{uv}(u,v)=pq\sum (P_{i+1,j+1}-P_{i+1,j}-P_{i,j+1}+P_{i,j})B_{i,p-1}(u)B_{j,q-1}(v)
```
### 2.4 법선 벡터 (Normal)
```math
N(u,v)=S_u(u,v)\times S_v(u,v)
```
- 단위 법선:
```math
\hat {N}=\frac{N}{\| N\| }
```
### 2.5 곡률
- First fundamental form
```math
E=S_u\cdot S_u,\quad F=S_u\cdot S_v,\quad G=S_v\cdot S_v
```
- Second fundamental form
```math
L=S_{uu}\cdot \hat {N},\quad M=S_{uv}\cdot \hat {N},\quad N=S_{vv}\cdot \hat {N}
```
- Gaussian curvature
```math
K=\frac{LN-M^2}{EG-F^2}
```
- Mean curvature
```math
H=\frac{EN-2FM+GL}{2(EG-F^2)}
```
### 2.6 Rational Bezier Surface
- Point4D 사용:
```math
S(u,v)=\frac{\sum P_{i,j}^wB_{i,p}(u)B_{j,q}(v)}{\sum w_{i,j}B_{i,p}(u)B_{j,q}(v)}
```

### 2.7 Surface Split (u, v)
- 각 방향으로 De Casteljau 적용:
  - u-split → 각 v-column에 대해 1D split
  - v-split → 각 u-row에 대해 1D split

## 2.8 Degree Elevation
- u 방향
```math
P'_{i,j}=\sum _kE_{i,k}P_{k,j}
```
- v 방향
```math
P'_{i,j}=\sum _kE_{j,k}P_{i,k}
```
- 여기서 E 는 degree elevation matrix.

### 2.9 Trim
- u 방향 split 두 번
- v 방향 split 두 번
- 결과를 [0,1]×[0,1] 로 재매핑

## 3. BezierCurve vs BezierSurface 비교
| 항목                 | Bezier Curve                     | Bezier Surface                          |
|----------------------|----------------------------------|------------------------------------------|
| 차수 (Degree)        | p                                | (p, q)                                   |
| Basis                | 1D Bernstein                     | 2D Tensor Bernstein                      |
| Control Structure    | $P_i$ (1D control points)        | $P_{i,j}$ (2D control net)                 |
| Domain               | [0, 1]                           | [0, 1] × [0, 1]                          |
| Evaluation Cost      | O(p)                             | O(p × q)                                 |
| Split                | 1D De Casteljau                  | 2D De Casteljau (u-split, v-split)       |
| Derivatives          | $C$', $C''$                      | $S_u$, $S_v$, $S_{uu}$, $S_{uv}$, $S_{vv}$ |
| Normal               | 없음                             | $S_u × S_v$                              |
| Curvature            | 없음                             | Gaussian, Mean curvature                 |
| Degree Elevation     | 1D elevation                     | u-direction, v-direction elevation       |
| Rational Form        | Weighted control points (Point4D)| Weighted control net (Point4D grid)      |
| Conversion to NURBS  | Simple                           | Tensor-product NURBS                     |


## 4. 엔진 구현 관점 요약
- ✔ Curve
  - point_at(t)
  - split(t)
  - elevate(t)
  - re_parameterize(f)
  - dot, cross
  - to_power_basis
- ✔ Surface
  - point_at(u,v)
  - evaluate_with_ders
  - normal, curvatures
  - split_u, split_v
  - elevate_u, elevate_v
  - trim
  - to_power_basis
  - to_nurbs

---

