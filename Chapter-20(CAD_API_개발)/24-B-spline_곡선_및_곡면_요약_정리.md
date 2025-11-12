# 📘 B-spline 곡선 및 곡면 요약 정리

## 3.1 서론
- B-spline은 곡선 및 곡면을 정의하는 데 사용되는 강력한 도구로, 비합리적(nonrational) 형태를 기본으로 설명.
- 제어점과 B-spline 기저 함수의 선형 조합으로 곡선을 구성.
- 주요 알고리즘: 점 계산, 도함수 계산, 국소 수정 등.

## 3.2 B-spline 곡선의 정의
### 📐 곡선 정의

$$
C(u)=\sum _{i=0}^nN_{i,p}(u)\cdot P_i
$$

- $P_i$: 제어점
- $N_{i,p}(u)$: p차 B-spline 기저 함수
- $U=\{ 0,\dots ,0,u_{p+1},\dots ,u_{m-p-1},1,\dots ,1\}$ : knot 벡터

## 📌 성질 요약
| 번호 | 성질 설명                                                                 |
|------|---------------------------------------------------------------------------|
| P3.1 | m = n + p + 1                                                             |
| P3.2 | C(0) = P₀, C(1) = Pₙ                                                      |
| P3.3 | φ(r) = Ar + v ⇒ φ(C(u)) = ∑ N_{i,p}(u) · φ(P_i)                           |
| P3.4 | C(u) ∈ Conv(P_{i−p}, ..., P_i)                                            |
| P3.5 | C(u)는 P_i에 대한 영향을 [u_i, u_{i+p+1}) 구간에서만 받음               |
| P3.6 | 제어 다각형은 곡선의 선형 근사이며, degree가 낮을수록 근사 정확도 높음 |
| P3.7 | 기저 함수 N_{i,p}(u)는 knot을 지나며 스위치처럼 작동함                  |
| P3.8 | 변동 감소 성질: 곡선은 제어 다각형보다 교차 수가 적음                   |
| P3.9 | 연속성: 곡선은 C^k 연속이며, k는 knot 중복도에 따라 달라짐             |



## 3.3 B-spline 곡선의 도함수
### 📐 도함수 정의

$$
C^{(k)}(u)=\sum _{i=0}^{n-k}N_{i,p-k}(u)\cdot P_i^{(k)}
$$

- $P_i^{(k)}$: k차 도함수 제어점
- 재귀적으로 계산:

$$
P_i^{(k)}=\frac{p-k+1}{u_{i+p+1}-u_{i+k}}\cdot (P_{i+1}^{(k-1)}-P_i^{(k-1)})
$$

### 📌 1차 도함수 특수식

$$
C'(u)=\sum _{i=0}^{n-1}N_{i+1,p-1}(u)\cdot Q_i\quad \mathrm{where}\quad Q_i=\frac{p}{u_{i+p+1}-u_{i+1}}\cdot (P_{i+1}-P_i)
$$

### 📌 끝점 도함수

$$
C'(0)=Q_0=\frac{p}{u_{p+1}}(P_1-P_0)\quad C'(1)=Q_{n-1}=\frac{p}{1-u_{m-p-1}}(P_n-P_{n-1})
$$

### 📌 2차 도함수 끝점 예시

$$
C^{(2)}(0)=\frac{p(p-1)}{u_{p+1}}\left( \frac{P_0}{u_{p+1}}-\frac{P_1}{u_{p+1}}+\frac{P_2}{u_{p+2}}\right) 
$$


## 3.4 B-spline 곡면의 정의

### 📐 곡면 정의

$$
S(u,v)=\sum _{i=0}^n\sum _{j=0}^mN_{i,p}(u)\cdot N_{j,q}(v)\cdot P_{i,j}
$$

- 제어점 $P_{i,j}$
- 기저 함수 $N_{i,p}(u)$, $N_{j,q}(v)$
- knot 벡터 U,V

### 📌 성질 요약
| 번호 | 성질 설명                                                                 |
|------|---------------------------------------------------------------------------|
| P3.12 | 기저 함수는 항상 0 이상                                                  |
| P3.13 | $∑_{i,j} N_{i,p}(u) · N_{j,q}(v) = 1$ (단위 분할 성질)                     |
| P3.14 | Bézier 곡면은 B-spline의 특수한 경우                                     |
| P3.15 | (u,v)가 $[u_i, u_{i+p+1}) × [v_j, v_{j+q+1})$ 범위 밖이면 기저 함수는 0     |
| P3.16 | 한 사각형 내에서 최대 (p+1)(q+1)개의 기저 함수가 비영                    |
| P3.17 | 각 기저 함수는 하나의 최대값을 가짐                                      |
| P3.18 | 사각형 내부에서는 모든 편도함수가 존재하며, knot에서 연속성은 중복도에 따라 결정 |
| P3.19 | 네 모서리 제어점은 항상 보간됨 (S(0,0), S(1,0), S(0,1), S(1,1))           |
| P3.20 | 아핀 불변성: 제어점에 아핀 변환 적용 시 곡면도 동일하게 변환됨           |
| P3.21 | 강한 볼록 껍질 성질: S(u,v)는 해당 사각형의 제어점 볼록 껍질에 포함       |
| P3.22 | 제어점 $P_{i,j}$ 를 움직이면 해당 사각형 내에서만 곡면이 변경됨             |
| P3.23 | 제어망은 곡면의 선형 근사 표현이며, degree가 낮을수록 근사 정확도 높음   |
| P3.24 | 연속성: $S(u,v)$ 는 $C^k$  연속이며, k는 knot 중복도에 따라 달라짐             |



## 📦 C 언어로 변환된 B-spline 알고리즘
### 1. CurvePoint (곡선 위의 점 계산)
```cpp
void CurvePoint(int n, int p, double* U, Point* P, double u, Point* C) {
    int span = FindSpan(n, p, u, U);
    double N[p + 1];
    BasisFuns(span, u, p, U, N);
    *C = PointZero();
    for (int i = 0; i <= p; i++) {
        *C = PointAdd(*C, PointScale(P[span - p + i], N[i]));
    }
}
```
### 2. CurveDerivsAlg1 (곡선의 도함수 계산)
```cpp
void CurveDerivsAlg1(int n, int p, double* U, Point* P, double u, int d, Point* CK) {
    int du = (d < p) ? d : p;
    for (int k = p + 1; k <= d; k++) CK[k] = PointZero();
    int span = FindSpan(n, p, u, U);
    double nders[d + 1][p + 1];
    DersBasisFuns(span, u, p, du, U, nders);
    for (int k = 0; k <= du; k++) {
        CK[k] = PointZero();
        for (int j = 0; j <= p; j++) {
            CK[k] = PointAdd(CK[k], PointScale(P[span - p + j], nders[k][j]));
        }
    }
}
```

### 3. CurveDerivCpts (도함수 제어점 계산)
```cpp
void CurveDerivCpts(int n, int p, double* U, Point* P, int d, int r1, int r2, Point** PK) {
    int r = r2 - r1;
    for (int i = 0; i <= r; i++) PK[0][i] = P[r1 + i];
    for (int k = 1; k <= d; k++) {
        int tmp = p - k + 1;
        for (int i = 0; i <= r - k; i++) {
            double denom = U[r1 + i + p + 1] - U[r1 + i + k];
            PK[k][i] = PointScale(PointSub(PK[k - 1][i + 1], PK[k - 1][i]), tmp / denom);
        }
    }
}
```


### 4. CurveDerivsAlg2 (도함수 계산 - 알고리즘 2)
```cpp
void CurveDerivsAlg2(int n, int p, double* U, Point* P, double u, int d, Point* CK) {
    int du = (d < p) ? d : p;
    for (int k = p + 1; k <= d; k++) CK[k] = PointZero();
    int span = FindSpan(n, p, u, U);
    double N[p + 1][p + 1];
    AllBasisFuns(span, u, p, U, N);
    Point PK[d + 1][p + 1];
    CurveDerivCpts(n, p, U, P, du, span - p, span, PK);
    for (int k = 0; k <= du; k++) {
        CK[k] = PointZero();
        for (int j = 0; j <= p - k; j++) {
            CK[k] = PointAdd(CK[k], PointScale(PK[k][j], N[j][p - k]));
        }
    }
}
```


### 5. SurfacePoint (곡면 위의 점 계산)
```cpp
void SurfacePoint(int n, int p, double* U, int m, int q, double* V, Point** P, double u, double v, Point* S) {
    int uspan = FindSpan(n, p, u, U);
    double Nu[p + 1];
    BasisFuns(uspan, u, p, U, Nu);
    int vspan = FindSpan(m, q, v, V);
    double Nv[q + 1];
    BasisFuns(vspan, v, q, V, Nv);
    int uind = uspan - p;
    *S = PointZero();
    for (int l = 0; l <= q; l++) {
        Point temp = PointZero();
        int vind = vspan - q + l;
        for (int k = 0; k <= p; k++) {
            temp = PointAdd(temp, PointScale(P[uind + k][vind], Nu[k]));
        }
        *S = PointAdd(*S, PointScale(temp, Nv[l]));
    }
}
```


### 📌 참고: 필요한 구조체 및 연산 함수 예시
```cpp
typedef struct {
    double x, y, z;
} Point;

Point PointZero() { return (Point){0.0, 0.0, 0.0}; }
Point PointAdd(Point a, Point b) { return (Point){a.x + b.x, a.y + b.y, a.z + b.z}; }
Point PointSub(Point a, Point b) { return (Point){a.x - b.x, a.y - b.y, a.z - b.z}; }
Point PointScale(Point a, double s) { return (Point){a.x * s, a.y * s, a.z * s}; }
```


## 📘 B-spline 알고리즘 원문 의사코드 정리
### 🔹 ALGORITHM A3.1: CurvePoint
```cpp
CurvePoint(n, p, U, P, u, C)
{ /* Compute curve point */
  /* Input: n, p, U, P, u */
  /* Output: C */
  span = FindSpan(n, p, u, U);
  BasisFuns(span, u, p, U, N);
  C = 0.0;
  for (i = 0; i <= p; i++)
    C = C + N[i] * P[span - p + i];
}
```

### 🔹 ALGORITHM A3.2: CurveDerivsAlg1
```cpp
CurveDerivsAlg1(n, p, U, P, u, d, CK)
{ /* Compute curve derivatives */
  /* Input: n, p, U, P, u, d */
  /* Output: CK */
  du = min(d, p);
  for (k = p + 1; k <= d; k++) CK[k] = 0.0;
  span = FindSpan(n, p, u, U);
  DersBasisFuns(span, u, p, du, U, nders);
  for (k = 0; k <= du; k++)
  {
    CK[k] = 0.0;
    for (j = 0; j <= p; j++)
      CK[k] = CK[k] + nders[k][j] * P[span - p + j];
  }
}
```


### 🔹 ALGORITHM A3.3: CurveDerivCpts
```cpp
CurveDerivCpts(n, p, U, P, d, r1, r2, PK)
{ /* Compute control points of curve derivatives */
  /* Input: n, p, U, P, d, r1, r2 */
  /* Output: PK */
  r = r2 - r1;
  for (i = 0; i <= r; i++)
    PK[0][i] = P[r1 + i];
  for (k = 1; k <= d; k++)
  {
    tmp = p - k + 1;
    for (i = 0; i <= r - k; i++)
      PK[k][i] = tmp * (PK[k - 1][i + 1] - PK[k - 1][i]) /
                 (U[r1 + i + p + 1] - U[r1 + i + k]);
  }
}
```


### 🔹 ALGORITHM A3.4: CurveDerivsAlg2
```cpp
CurveDerivsAlg2(n, p, U, P, u, d, CK)
{ /* Compute curve derivatives */
  /* Input: n, p, U, P, u, d */
  /* Output: CK */
  du = min(d, p);
  for (k = p + 1; k <= d; k++) CK[k] = 0.0;
  span = FindSpan(n, p, u, U);
  AllBasisFuns(span, u, p, U, N);
  CurveDerivCpts(n, p, U, P, du, span - p, span, PK);
  for (k = 0; k <= du; k++)
  {
    CK[k] = 0.0;
    for (j = 0; j <= p - k; j++)
      CK[k] = CK[k] + N[j][p - k] * PK[k][j];
  }
}
```


### 🔹 ALGORITHM A3.5: SurfacePoint
```cpp
SurfacePoint(n, p, U, m, q, V, P, u, v, S)
{ /* Compute surface point */
  /* Input: n, p, U, m, q, V, P, u, v */
  /* Output: S */
  uspan = FindSpan(n, p, u, U);
  BasisFuns(uspan, u, p, U, Nu);
  vspan = FindSpan(m, q, v, V);
  BasisFuns(vspan, v, q, V, Nv);
  uind = uspan - p;
  S = 0.0;
  for (l = 0; l <= q; l++)
  {
    temp = 0.0;
    vind = vspan - q + l;
    for (k = 0; k <= p; k++)
      temp = temp + Nu[k] * P[uind + k][vind];
    S = S + Nv[l] * temp;
  }
}
```

### CurvePoint 알고리즘 + 테스트 코드 (C)
```cpp
#include <stdio.h>
#include <stdlib.h>

#define MAX_P 10

typedef struct {
    double x, y, z;
} Point;

Point PointZero() {
    return (Point){0.0, 0.0, 0.0};
}

Point PointAdd(Point a, Point b) {
    return (Point){a.x + b.x, a.y + b.y, a.z + b.z};
}

Point PointScale(Point a, double s) {
    return (Point){a.x * s, a.y * s, a.z * s};
}

// FindSpan: returns the knot span index
int FindSpan(int n, int p, double u, double* U) {
    if (u >= U[n + 1]) return n;
    if (u <= U[p]) return p;
    int low = p, high = n + 1, mid;
    while (high - low > 1) {
        mid = (low + high) / 2;
        if (u < U[mid]) high = mid;
        else low = mid;
    }
    return low;
}

// BasisFuns: computes nonzero basis functions
void BasisFuns(int i, double u, int p, double* U, double* N) {
    double left[MAX_P], right[MAX_P];
    N[0] = 1.0;
    for (int j = 1; j <= p; j++) {
        left[j] = u - U[i + 1 - j];
        right[j] = U[i + j] - u;
        double saved = 0.0;
        for (int r = 0; r < j; r++) {
            double temp = N[r] / (right[r + 1] + left[j - r]);
            N[r] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        N[j] = saved;
    }
}

// CurvePoint: computes a point on the B-spline curve
void CurvePoint(int n, int p, double* U, Point* P, double u, Point* C) {
    int span = FindSpan(n, p, u, U);
    double N[MAX_P + 1];
    BasisFuns(span, u, p, U, N);
    *C = PointZero();
    for (int i = 0; i <= p; i++) {
        *C = PointAdd(*C, PointScale(P[span - p + i], N[i]));
    }
}

// 테스트용 메인 함수
int main() {
    // 예제: p = 2, U = {0,0,0,1,2,3,4,4,5,5,5}, u = 2.5
    int p = 2;
    int n = 7; // control points 개수 - 1
    double U[] = {0,0,0,1,2,3,4,4,5,5,5};
    Point P[] = {
        {0,0,0}, {1,0,0}, {2,0,0}, {3,0,0},
        {4,0,0}, {5,0,0}, {6,0,0}, {7,0,0}
    };
    double u = 2.5;
    Point C;
    CurvePoint(n, p, U, P, u, &C);
    printf("C(%.2f) = (%.3f, %.3f, %.3f)\n", u, C.x, C.y, C.z);
    return 0;
}
```


## ✅ 설명
- FindSpan: u가 속한 knot 구간을 찾습니다.
- BasisFuns: 해당 구간에서 비영인 B-spline 기저 함수들을 계산합니다.
- CurvePoint: 기저 함수와 제어점을 선형 결합하여 곡선 위의 점을 계산합니다.
- main: 테스트용으로 p=2, u=2.5인 곡선 점을 계산하고 출력합니다.

----
