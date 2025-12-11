# Greville Parameter
- Greville 파라미터(Greville abscissae)는 NURBS 곡선에서 각 제어점(Control Point)에 대응하는 대표적인 파라미터 값을 말합니다.  
- 이는 곡선의 매개변수 공간에서 제어점 위치를 평가하거나 곡선을 샘플링할 때 기준점으로 자주 사용됩니다.

## 🔎 자세한 설명
- 정의:
  - NURBS 곡선은 제어점, 차수(degree), 그리고 노트 벡터(knot vector)로 정의됩니다.
  - Greville 파라미터는 각 제어점에 대해 해당 제어점과 관련된 노트들의 평균값으로 계산됩니다.
- 수식:
  - 차수가 p인 B-스플라인/NURBS 곡선에서 i-번째 제어점의 Greville 파라미터 $\xi _i$ 는 다음과 같이 정의됩니다:
  - 여기서 $u_j$ 는 노트 벡터(knot vector)의 원소입니다.
$$
\xi _i=\frac{u_{i+1}+u_{i+2}+\cdots +u_{i+p}}{p}
$$

- 의미:
  - 제어점이 실제 곡선 위에 직접 놓이지 않는 경우가 많기 때문에, Greville 파라미터는 제어점과 곡선의 대응 관계를 설명하는 기준 좌표 역할을 합니다.
  - 곡선을 균등하게 샘플링하거나, 제어점 기반으로 곡선의 형태를 분석할 때 사용됩니다.
  - CAD/CAE 소프트웨어(Rhino, AutoCAD 등)에서도 NURBS 곡선을 다룰 때 내부적으로 활용됩니다.

## 📌 예시
- 차수 p=3 (큐빅 NURBS)이고, 노트 벡터가 [0,0,0,0,1,2,3,3,3,3]라면:
  - 첫 번째 제어점의 Greville 파라미터는 (u_1+u_2+u_3)/3=(0+0+0)/3=0.
  - 두 번째 제어점은 (u_2+u_3+u_4)/3=(0+0+1)/3=0.33....
  - 이런 식으로 각 제어점마다 대응하는 파라미터가 계산됩니다.

## ⚖️ 요약
- Greville 파라미터 = 제어점에 대응하는 평균 노트 값
- 용도 = 곡선 샘플링, 제어점-곡선 대응 분석, CAD/CAE 내부 계산
- 특징 = 제어점이 직접 곡선 위에 있지 않아도, 파라미터 공간에서 대응 관계를 제공

---

## 샘플 소스

### Overview
- 아래 예제는 Greville 파라미터를 계산하고, 이를 이용해 NURBS 곡선을 샘플링하는 최소 구현입니다.
  - 입력: 제어점, 가중치, 차수 p, 노트 벡터
  - 출력: Greville 파라미터 배열과 해당 파라미터에서의 곡선 점들
  - 기법: Cox–de Boor 재귀를 이용한 B-스플라인 기반함수, NURBS는 가중합으로 평가

### C++ 예제
```cpp
#include <vector>
#include <array>
#include <iostream>
#include <iomanip>
#include <stdexcept>

struct Vec3 {
    double x, y, z;
    Vec3 operator*(double s) const { return {x*s, y*s, z*s}; }
    Vec3 operator+(const Vec3& o) const { return {x+o.x, y+o.y, z+o.z}; }
};

static int findSpan(int n, int p, double u, const std::vector<double>& U) {
    if (u >= U[n+1]) return n;
    if (u <= U[p]) return p;
    int low = p, high = n+1, mid = (low+high)/2;
    while (u < U[mid] || u >= U[mid+1]) {
        if (u < U[mid]) high = mid;
        else            low = mid;
        mid = (low+high)/2;
    }
    return mid;
}

static std::vector<double> basisFuns(int span, double u, int p, const std::vector<double>& U) {
    std::vector<double> N(p+1, 0.0);
    std::vector<double> left(p+1, 0.0), right(p+1, 0.0);
    N[0] = 1.0;
    for (int j = 1; j <= p; ++j) {
        left[j]  = u - U[span+1-j];
        right[j] = U[span+j] - u;
        double saved = 0.0;
        for (int r = 0; r < j; ++r) {
            double denom = right[r+1] + left[j-r];
            double temp = (denom == 0.0) ? 0.0 : N[r] / denom;
            double val  = saved + right[r+1] * temp;
            saved       = left[j-r] * temp;
            N[r] = val;
        }
        N[j] = saved;
    }
    return N;
}

// Greville parameters: xi_i = (U[i+1] + ... + U[i+p]) / p
static std::vector<double> grevilleParameters(int p, const std::vector<double>& U, int nCtrl) {
    std::vector<double> xi(nCtrl, 0.0);
    for (int i = 0; i < nCtrl; ++i) {
        double sum = 0.0;
        for (int k = 1; k <= p; ++k) sum += U[i + k];
        xi[i] = sum / p;
    }
    return xi;
}

static Vec3 evaluateNURBS(double u,
                          int p,
                          const std::vector<double>& U,
                          const std::vector<Vec3>& P,
                          const std::vector<double>& W) {
    int n = (int)P.size() - 1;
    int span = findSpan(n, p, u, U);
    auto N = basisFuns(span, u, p, U);

    Vec3 C{0,0,0};
    double denom = 0.0;
    for (int j = 0; j <= p; ++j) {
        int i = span - p + j;
        double wN = W[i] * N[j];
        C = C + P[i] * wN;
        denom += wN;
    }
    if (denom == 0.0) return C; // degenerate case
    return C * (1.0 / denom);
}
```
```cpp
int main() {
    // Example: cubic NURBS (p=3), open uniform knot
    int p = 3;
    std::vector<Vec3> P = {
        {0,0,0}, {1,2,0}, {3,3,0}, {4,2,0}, {5,0,0}
    };
    std::vector<double> W = {1, 1, 1, 1, 1}; // weights
    // Knot vector length = n+p+2, where n = #ctrl-1
    // Here #ctrl=5 => n=4 => len=4+3+2=9
    std::vector<double> U = {0,0,0,0,1,2,3,3,3};

    if ((int)U.size() != (int)P.size() + p + 2)
        throw std::runtime_error("Invalid knot vector length");

    // Compute Greville parameters
    auto xi = grevilleParameters(p, U, (int)P.size());

    // Sample curve at Greville parameters
    std::cout << std::fixed << std::setprecision(6);
    for (size_t i = 0; i < xi.size(); ++i) {
        Vec3 c = evaluateNURBS(xi[i], p, U, P, W);
        std::cout << "xi[" << i << "]=" << xi[i]
                  << " => C=(" << c.x << ", " << c.y << ", " << c.z << ")\n";
    }

    // Optional: uniform sampling between U[p] and U[n+1]
    double uStart = U[p], uEnd = U[P.size()];
    int samples = 20;
    for (int s = 0; s <= samples; ++s) {
        double u = uStart + (uEnd - uStart) * (double)s / samples;
        Vec3 c = evaluateNURBS(u, p, U, P, W);
        // ...
    }

    return 0;
}
```

---





