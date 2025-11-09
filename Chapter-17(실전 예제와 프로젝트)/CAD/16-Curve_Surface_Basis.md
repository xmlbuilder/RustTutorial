# Curve & Surface Basis

## 📘 문서 핵심 요약
### 📘 1.1 암시적(Implicit) vs. 매개변수(Parametric) 표현

| 구분             | 정의 방식                          | 수학적 표현                         | 예시 (단위 원)                          |
|------------------|-------------------------------------|--------------------------------------|------------------------------------------|
| 암시적 표현      | 좌표 간의 관계를 직접 정의          | $f(x, y) = 0 $                    | $x^2 + y^2 - 1 = 0$                  |
| 매개변수 표현    | 각 좌표를 독립 변수의 함수로 표현   | $C(u) = (x(u), y(u))$           | $x(u) = \cos(u),\ y(u) = \sin(u)$   |


- 매개변수 표현은 방향성과 경계 제어가 쉬움
- 속도 벡터: $C'(u)=(-\sin (u),\cos (u))$, 속도 크기 일정 → 균일 파라미터화

### 1.2 Power Basis 곡선
- 일반적인 다항식 표현:

$$
C(u)=\sum _{i=0}^na_iu^i
$$

- Horner 알고리즘으로 효율적 계산:

$$
C(u)=((a_nu+a_{n-1})u+\dots +a_0)
$$

### 1.3 Bézier 곡선
- 정의:

$$
C(u)=\sum _{i=0}^nB_{i,n}(u)P_i\quad \mathrm{where\  }B_{i,n}(u)={n \choose i}(1-u)^{n-i}u^i
$$

- 특징:
    - 제어점 기반
    - Convex Hull 속성
    - Variation Diminishing 속성
    - De Casteljau 알고리즘으로 계산 가능

#### 📌 도함수 표현
- 1차 도함수:

$$
C'(u)=n\sum _{i=0}^{n-1}B_{i,n-1}(u)(P_{i+1}-P_i)
$$


- 끝점 도함수:

$$
C'(0)=n(P_1-P_0),\quad C'(1)=n(P_n-P_{n-1})
$$


### 1.4 Rational Bézier 곡선
- 정의:

$$
C(u)=\frac{\sum _{i=0}^nw_iB_{i,n}(u)P_i}{\sum _{i=0}^nw_iB_{i,n}(u)}=\sum _{i=0}^nR_{i,n}(u)P_iR_{i,n}(u)=\frac{w_iB_{i,n}(u)}{\sum _{j=0}^nw_jB_{j,n}(u)}
$$

- 특징:
- 원, 타원 등 정확한 표현 가능
- Homogeneous 좌표로 4차원에서 계산 후 투영

### 1.5 Tensor Product Surface
- 정의:

$$
S(u,v)=\sum _{i=0}^n\sum _{j=0}^mf_i(u)g_j(v)P_{i,j}
$$

- Bézier Surface:

$$
S(u,v)=\sum _{i=0}^n\sum _{j=0}^mB_{i,n}(u)B_{j,m}(v)P_{i,j}
$$

- 특징:
    - 이중 방향 제어 가능
    - Convex Hull 속성 유지
    - Bézier 곡선의 성질을 2D로 확장

## 📐 주요 알고리즘
- Horner’s Method (Power Basis)
- De Casteljau Algorithm (Bézier)
- Rational Bézier → Homogeneous 좌표 변환
- Tensor Product Surface → Horner2, DeCasteljau2


### 🧮 Bernstein Basis 함수 시각화 코드 (ASCII)
```python
# Creating visualization of Bernstein basis functions B_{i,3}(u) for i = 0 to 3
import numpy as np
import matplotlib.pyplot as plt

# Define Bernstein basis functions for n = 3
def bernstein(i, n, u):
    from math import comb
    return comb(n, i) * (u ** i) * ((1 - u) ** (n - i))

# Generate u values
u_vals = np.linspace(0, 1, 500)

# Compute basis functions
B0 = [bernstein(0, 3, u) for u in u_vals]
B1 = [bernstein(1, 3, u) for u in u_vals]
B2 = [bernstein(2, 3, u) for u in u_vals]
B3 = [bernstein(3, 3, u) for u in u_vals]

# Plotting
plt.style.use('seaborn-v0_8')
plt.figure(figsize=(10, 6))
plt.plot(u_vals, B0, label='B₀,₃(u)', color='red')
plt.plot(u_vals, B1, label='B₁,₃(u)', color='green')
plt.plot(u_vals, B2, label='B₂,₃(u)', color='blue')
plt.plot(u_vals, B3, label='B₃,₃(u)', color='purple')
plt.title('Bernstein Basis Functions for n = 3')
plt.xlabel('u')
plt.ylabel('B_{i,3}(u)')
plt.legend()
plt.grid(True)
plt.tight_layout()

# Save plot
output_path = "/mnt/data/bernstein_basis_n3.png"
plt.savefig(output_path)

print("Plotted Bernstein basis functions B_{i,3}(u) for i = 0 to 3 and saved as bernstein_basis_n3.png")
```
![Bernstein Basis Function n = 3](/image/Bernstein_Basis_3.png)

- 이 코드는 Bernstein 다항식의 정의를 기반으로 각 basis 함수의 값을 계산하고, 그래프로 시각화합니다.


---

# C 코드

Bézier 곡선과 Power Basis 곡선 계산을 위한 여러 C 코드 예제가 포함되어 있습니다.  
아래에 문서에서 등장한 주요 C 코드들을 ASCII 그대로 정리.

## 🧮 ALGORITHM A1.1 — Horner 방식 (Power Basis 곡선 계산)

### ALGORITHM A1.1
```cpp
Horner1(a, n, u0, C)
{ /* Compute point on power basis curve. */
  /* Input: a, n, u0 */
  /* Output: C */
  C = a[n];
  for (i = n - 1; i >= 0; i--)
    C = C * u0 + a[i];
}
```

## 🧮 ALGORITHM A1.2 — 단일 Bernstein 다항식 계산

### ALGORITHM A1.2

```cpp
Bernstein(i, n, u, B)
{ /* Compute the value of a Bernstein polynomial. */
  /* Input: i, n, u */
  /* Output: B */
  for (j = 0; j <= n; j++)
    temp[j] = 0.0;
  temp[n - i] = 1.0;

  u1 = 1.0 - u;
  for (k = 1; k <= n; k++)
    for (j = n; j >= k; j--)
      temp[j] = u1 * temp[j] + u * temp[j - 1];

  B = temp[n];
}
```


## 🧮 ALGORITHM A1.3 — 모든 Bernstein 다항식 계산

### ALGORITHM A1.3

```cpp
AllBernstein(n, u, B)
{ /* Compute all nth-degree Bernstein polynomials. */
  /* Input: n, u */
  /* Output: B (an array B[0] ... B[n]) */
  B[0] = 1.0;
  u1 = 1.0 - u;
  for (j = 1; j <= n; j++)
  {
    saved = 0.0;
    for (k = 0; k < j; k++)
    {
      temp = B[k];
      B[k] = saved + u1 * temp;
      saved = u * temp;
    }
    B[j] = saved;
  }
}
```


## 🧮 ALGORITHM A1.4 — Bézier 곡선의 점 계산
### ALGORITHM A1.4
```cpp
PointOnBezierCurve(P, n, u, C)
{ /* Compute point on Bézier curve. */
  /* Input: P, n, u */
  /* Output: C (a point) */
  AllBernstein(n, u, B); /* B is a local array */
  C = 0.0;
  for (k = 0; k <= n; k++)
    C = C + B[k] * P[k];
}
```


## 🧮 ALGORITHM A1.5 — de Casteljau 알고리즘
### ALGORITHM A1.5

```cpp
deCasteljau1(P, n, u, C)
{ /* Compute point on a Bézier curve using de Casteljau */
  /* Input: P, n, u */
  /* Output: C (a point) */
  for (i = 0; i <= n; i++)
    Q[i] = P[i]; /* Use local array so we do not destroy control points */

  for (k = 1; k <= n; k++)
    for (i = 0; i <= n - k; i++)
      Q[i] = (1.0 - u) * Q[i] + u * Q[i + 1];

  C = Q[0];
}

```
- 이 코드는 Bézier 곡선의 점 계산, Bernstein 다항식 생성, 그리고 Power Basis 곡선 계산을 위한 핵심 알고리즘들입니다.

