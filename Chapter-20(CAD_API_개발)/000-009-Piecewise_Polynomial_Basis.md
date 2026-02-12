# 📘 Piecewise Polynomial Curve — 문서화
- (Piegl & Tiller Chapter 2 기반)

## 1. Piecewise Polynomial Curve의 기본 개념
- 곡선을 단일 고차 다항식으로 표현하면:
    - 진동(oscillation)이 커지고
    - 지역 수정(local modification)이 불가능하며
    - 제어가 어렵다
- 그래서 CAD에서는 여러 개의 낮은 차수(polynomial) 조각을 이어 붙인 곡선을 사용한다.
- 이것이 piecewise polynomial curve다.

## 2. 일반적인 곡선 표현식
- 곡선은 다음과 같이 표현된다:
```math
C(u) = Σ f_i(u) * P_i
```

- $P_i$ : control point
- $f_i(u)$ : piecewise polynomial basis function
- 즉, basis function × control point의 선형 결합으로 곡선을 만든다.

## 3. Basis Function의 성질
- ✔ 1) Piecewise polynomial
    - 각 $f_i(u)$ 는 여러 구간으로 나뉘어 있고, 각 구간에서 polynomial이다.
- ✔ 2) Local support
    - 각 basis function은 전체 구간에서 nonzero가 아니다.
- 즉:   
    - $f_i(u) ≠ 0$  only on a few subintervals


- 이 성질 덕분에:
    - 특정 control point를 움직여도
    - 곡선 전체가 아니라 일부 구간만 영향을 받는다.
- 이게 B‑spline의 핵심 장점이다.

## 4. Rational Curve로 확장
- 동차 좌표를 사용하면 rational 형태가 된다.
```math
C^w(u) = Σ f_i(u) * P_i^w
```

- 여기서
```math
P_i^w = (x_i w_i, y_i w_i, z_i w_i, w_i).
```
- 투영하면:
```math
C(u) = (X/W, Y/W, Z/W)
```

- 즉, piecewise rational curve가 된다.

## 5. Surface로 확장 (Tensor Product)
- 곡면은 두 방향 basis를 곱해서 만든다.
```math
S(u,v) = Σ Σ f_i(u) * g_j(v) * P_{i,j}
```

- 동차 버전:
```math
S^w(u,v) = Σ Σ f_i(u) * g_j(v) * P_{i,j}^w
```

- 여기서:
    - $f_i(u)$ : u 방향 basis
    - $g_j(v)$ : v 방향 basis
    - $P_{i,j}$ : 2D control net
- 이 구조는 B‑spline surface, NURBS surface의 기본 틀이다.

## 6. 이 페이지가 말하는 핵심 요약
- 곡선은 $C(u) = Σ f_i(u) P_i$ 형태로 표현된다.
- $f_i(u)$ 는 piecewise polynomial basis이며 local support를 가진다.
- 동차 좌표를 사용하면 rational curve가 된다.
- 두 방향 basis를 곱하면 tensor‑product surface가 된다.
- 이 구조는 이후 B‑spline, NURBS의 기반이 된다.

---

## 📘 1. B‑spline Basis Function
### 1.1 Knot vector
```math
U = {u0, u1, ..., u_{n+p+1}}
```

- 조건:
```math
u_i ≤ u_{i+1}
```


### 1.2 0차(zeroth-degree) basis
- $N_{i,0}(u) = 1$   if  $u_i ≤ u < u_{i+1}$
- $N_{i,0}(u) = 0$  otherwise

### 1.3 p차 basis의 재귀 정의
```math
N_{i,p}(u) =
    (u - u_i) / (u_{i+p}   - u_i)   * N_{i, p-1}(u)
  + (u_{i+p+1} - u) / (u_{i+p+1} - u_{i+1}) * N_{i+1, p-1}(u)
```

- 분모가 0이면 해당 항은 0으로 처리.

## 📘 2. 예제: Basis가 생성되는 전 과정 (u 표기)
- 우리가 직접 하나의 knot vector를 선택해서
    - 0차 → 1차 → 2차 basis가 어떻게 만들어지는지
    - 그리고 어떤 basis가 죽는지까지 예시.

### ✔ 예제 설정
- degree p = 2 (quadratic)
- control point 개수 = 4 → n = 3
- open uniform knot vector
```
U = {0, 0, 0, 1, 2, 3, 3, 3}
```

- 인덱스:
```
u0 u1 u2 u3 u4 u5 u6 u7
0  0  0  1  2  3  3  3
```

- 이때 basis는:
```math
N_{0,2}, N_{1,2}, N_{2,2}, N_{3,2}
```


## 📘 3. Step 1 — 0차 basis N_{i,0}(u)
- 0차 basis는 단순히 구간 indicator.

| i | interval $[u_i, u_{i+1})$ | $N_{i,0}(u)$ value |
|---|--------------------------|------------------|
| 0 | [0, 0)                  | always 0         |
| 1 | [0, 0)                  | always 0         |
| 2 | [0, 1)                  | 1                |
| 3 | [1, 2)                  | 1                |
| 4 | [2, 3)                  | 1                |
| 5 | [3, 3)                  | always 0         |
| 6 | [3, 3)                  | always 0         |

여기서 중요한 사실
- u0=u1=u2=0 → 첫 두 basis는 support가 없다 → 죽음
- u5=u6=u7=3 → 마지막 두 basis도 support 없음 → 죽음
- 즉, 살아남는 0차 basis는:
```math
N_{2,0}, N_{3,0}, N_{4,0}
```


## 📘 4. Step 2 — 1차 basis $N_{i,1}(u)$
- 재귀식:
```math
N_{i,1}(u) =
 (u - u_i)/(u_{i+1}-u_i) * N_{i,0}(u)
+ (u_{i+2}-u)/(u_{i+2}-u_{i+1}) * N_{i+1,0}(u)
```

- 살아남는 1차 basis는:
- $N_{1,1}$: 0,1)
- $N_{2,1}$: 0,2)
- $N_{3,1}$: 1,3)
- 죽는 basis:
    - i=0,4,5,6 → 분모 0 또는 0차 basis가 0 → 항상 0

## 📘 5. Step 3 — 2차 basis N_{i,2}(u)
- 재귀식:
```math
N_{i,2}(u) =
 (u - u_i)/(u_{i+2}-u_i) * N_{i,1}(u)
+ (u_{i+3}-u)/(u_{i+3}-u_{i+1}) * N_{i+1,1}(u)
```

- 살아남는 basis:

| basis     | support interval | description                          |
|-----------|------------------|--------------------------------------|
| $N_{0,2}(u)$ | [0, 2)           | left-side quadratic bump             |
| $N_{1,2}(u)$ | [0, 3)           | widest basis, center of the domain   |
| $N_{2,2}(u)$ | [1, 3)           | right-shifted quadratic bump         |
| $N_{3,2}(u)$ | [2, 3)           | right-end bump (short support)       |


- 죽는 basis:
    - i ≥ 4 → support 없음
    - i < 0 → 없음
    - i=0은 살아남지만 i=-1은 존재하지 않음

## 📘 6. 왜 어떤 basis는 “죽는가”?
- 이유 1 — Knot 중복 때문에 support가 사라짐
- 예:

```
u0 = u1 = u2 = 0
```

- u0, u1), u1, u2) 는 길이가 0
    - $N_{0,0}, $N_{1,0} = 0$
    - higher degree basis도 모두 0
- 이유 2 — 재귀식 분모가 0
- 예:
```
u_i = u_{i+p}
```

- (u - u_i)/(u_{i+p}-u_i) = 0/0 → 정의상 0
    - 해당 항 전체가 0
- 이유 3 — support가 비어 있음
- 예:
```
u5 = u6 = u7 = 3
```

- 3,3) 구간은 measure 0
- N_{5,0}, N_{6,0} = 0
- higher degree basis도 0

## 📘 7. 전체 요약
- B‑spline basis는 knot vector + degree로 완전히 결정된다.
- basis는 0차부터 재귀적으로 만들어진다.
- knot가 중복되면 일부 basis는 support가 사라져 항상 0이 되어 죽는다.
- 살아남는 basis만이 실제 곡선을 구성한다.
- degree가 커질수록 basis는 넓은 support를 갖고 더 부드러워진다.

---
