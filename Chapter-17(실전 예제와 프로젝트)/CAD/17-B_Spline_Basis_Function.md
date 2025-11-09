
# 📘 B-spline Basis Function 정리
## 🔹 정의: Cox–de Boor 재귀식
B-spline basis 함수 $N_{i,p}(u)$ 는 다음과 같이 정의됩니다:
- 0차 (p = 0):

$$
N_{i,0}(u) =
\begin{cases}
1, & \text{if } u_i \le u < u_{i+1},\\
0, & \text{otherwise}.
\end{cases}
$$

- p차 (p > 0):

$$ 
N_{i,p}(u)=\frac{u-u_i}{u_{i+p}-u_i}N_{i,p-1}(u)+\frac{u_{i+p+1}-u}{u_{i+p+1}-u_{i+1}}N_{i+1,p-1}(u)
$$

- 이 수식은 재귀적으로 정의되며, 각 basis 함수는 두 개의 낮은 차수 basis 함수의 선형 결합입니다.

| 번호 | 성질 이름             | 수학적 표현 또는 설명                                      |
|------|------------------------|-------------------------------------------------------------|
| P2.1 | 국소 지지성 (Local Support) | $N_{i,p}(u) \ne 0$ only on $[u_i, u_{i+p+1})$             |
| P2.2 | 제한된 비영점 개수     | $[u_j, u_{j+1})$ 구간에서 최대 $p+1$개의 함수만 0이 아님     |
| P2.3 | 비음수성 (Non-negativity) | $N_{i,p}(u) \geq 0$ for all $i, p, u$                    |
| P2.4 | 정규화 (Partition of Unity) | $\sum_i N_{i,p}(u) = 1$ for all $u$                      |
| P2.5 | 연속성 (Smoothness)     | 내부에서는 $C^\infty$, knot에서의 연속성은 중복도에 따라 결정됨 |
| P2.6 | 단일 극대값 존재       | $N_{i,p}(u)$는 정의역 내에서 하나의 최대값만 가짐           |

## 📌 예시 시각화 (n = 3, Uniform Knot Vector)
위 카드에서 확인하실 수 있는 그래프는 다음을 보여줍니다:
- Basis 함수 $N_{0,3}(u)$ 부터 $N_{6,3}(u)$ 까지
- 각 함수는 종 모양이며, 서로 겹쳐서 전체 곡선을 구성
- 각 함수는 최대 4개의 knot span에서만 0이 아님


### 🔹 예시: Uniform Knot Vector에서의 3차 B-spline Basis
- Knot Vector: $U=\{ 0,0,0,0,1,2,3,4,4,4,4\}$ 
- Degree: p=3
- Basis 함수 개수: n=m-p-1=10-3-1=6

### 📈 시각화: N_{i,3}(u) for i=0 to 6
아래 그래프는 위의 knot vector를 기반으로 생성한 3차 B-spline basis 함수들입니다:
- 각 곡선은 $N_{i,3}(u)$ 를 나타내며, 서로 다른 색상으로 구분됩니다
- 정의역은 $u\in [0,4]$
- 각 함수는 최대 4개의 knot span에서만 0이 아님 (국소 지지성)

```python
import numpy as np
import matplotlib.pyplot as plt
# Cox–de Boor 재귀 정의
def N(i, k, u, U):
    if k == 0:
        return 1.0 if U[i] <= u < U[i+1] else 0.0
    denom1 = U[i+k] - U[i]
    denom2 = U[i+k+1] - U[i+1]
    term1 = 0.0 if denom1 == 0 else (u - U[i]) / denom1 * N(i, k-1, u, U)
    term2 = 0.0 if denom2 == 0 else (U[i+k+1] - u) / denom2 * N(i+1, k-1, u, U)
    return term1 + term2

# 설정
degree = 3
U = [0, 0, 0, 0, 1, 2, 3, 4, 4, 4, 4]  # uniform knot vector
n_basis = len(U) - degree - 1
u_vals = np.linspace(0, 4, 500)

# basis 함수 계산
basis_values = []
for i in range(n_basis):
    values = [N(i, degree, u, U) for u in u_vals]
    basis_values.append(values)

# 시각화
plt.figure(figsize=(10, 6))
colors = ['red', 'green', 'blue', 'orange', 'purple', 'brown', 'cyan']
for i, values in enumerate(basis_values):
    plt.plot(u_vals, values, label=f'N{i},{degree}(u)', color=colors[i % len(colors)])

plt.title('B-spline Basis Functions (degree = 3)')
plt.xlabel('u')
plt.ylabel('N_{i,3}(u)')
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.show()
```

![Nurbs N basis(/image/Nurbs_basis.png)


### 🔹 도함수 공식
- 1차 도함수:

$$
\frac{d}{du}N_{i,p}(u)=\frac{p}{u_{i+p}-u_i}N_{i,p-1}(u)-\frac{p}{u_{i+p+1}-u_{i+1}}N_{i+1,p-1}(u)
$$

- 일반 도함수 (k차):
$$
N_{i,p}^{(k)}(u)=\sum _{j=0}^ka_{k,j}N_{i+j,p-k}(u)
$$

- $a_{k,j}$ 는 재귀적으로 계산되는 계수

### 🔹 다중 Knot의 영향
- Knot 중복도가 높아질수록 연속성이 감소
- 예: 3차 함수에서 knot가 2번 반복되면 C^1 연속성만 유지
- Knot가 p+1번 반복되면 해당 지점에서 곡선은 비연속이 됨

---

# 소스

문서에 포함된 B-spline 관련 C 코드 알고리즘들을 아래에 ASCII 그대로 정리.  
이들은 B-spline basis 함수와 그 도함수를 계산하는 데 사용되는 핵심 알고리즘입니다.

## 🧮 ALGORITHM A2.1 — Knot Span Index 찾기
```cpp
int FindSpan(n, p, u, U)
{ /* Determine the knot span index */
  /* Input: n, p, u, U */
  /* Return: the knot span index */
  if (u == U[n+1]) return(n); /* Special case */
  low = p;
  high = n + 1;
  mid = (low + high) / 2;
  while (u < U[mid] || u >= U[mid+1])
  {
    if (u < U[mid]) high = mid;
    else low = mid;
    mid = (low + high) / 2;
  }
  return(mid);
}
```


## 🧮 ALGORITHM A2.2 — Basis 함수 
- $N_{i-p,p}(u) ~ N_{i,p}(u)$ 계산

```cpp
BasisFuns(i, u, p, U, N)
{ /* Compute the nonzero basis functions */
  /* Input: i, u, p, U */
  /* Output: N[0] ... N[p] */
  N[0] = 1.0;
  for (j = 1; j <= p; j++)
  {
    left[j] = u - U[i+1-j];
    right[j] = U[i+j] - u;
    saved = 0.0;
    for (r = 0; r < j; r++)
    {
      temp = N[r] / (right[r+1] + left[j-r]);
      N[r] = saved + right[r+1] * temp;
      saved = left[j-r] * temp;
    }
    N[j] = saved;
  }
}
```


## 🧮 ALGORITHM A2.3 — Basis 함수 도함수 계산

```cpp
void DersBasisFuns(int i, double u, int p, int n, double *U, double **ders)
{
    double **ndu = new double*[p+1];
    for (int j = 0; j <= p; j++)
        ndu[j] = new double[p+1];

    double *left = new double[p+1];
    double *right = new double[p+1];

    ndu[0][0] = 1.0;

    for (int j = 1; j <= p; j++) {
        left[j] = u - U[i+1-j];
        right[j] = U[i+j] - u;
        double saved = 0.0;
        for (int r = 0; r < j; r++) {
            double temp = ndu[r][j-1] / (right[r+1] + left[j-r]);
            ndu[r][j] = saved + right[r+1] * temp;
            saved = left[j-r] * temp;
        }
        ndu[j][j] = saved;
    }

    for (int j = 0; j <= p; j++)
        ders[0][j] = ndu[j][p];

    double **a = new double*[2];
    for (int j = 0; j < 2; j++)
        a[j] = new double[p+1];

    for (int r = 0; r <= p; r++) {
        int s1 = 0, s2 = 1;
        a[0][0] = 1.0;

        for (int k = 1; k <= n; k++) {
            double d = 0.0;
            int rk = r - k;
            int pk = p - k;
            int j1, j2;

            if (r >= k)
                j1 = 0;
            else
                j1 = k - r;

            if (r - 1 <= pk)
                j2 = k - 1;
            else
                j2 = p - r;

            a[s2][0] = 0.0;
            for (int j = j1; j <= j2; j++) {
                a[s2][j] = (a[s1][j] - a[s1][j-1]) / (U[i+r+1+j] - U[i+r-k+1+j]);
                d += a[s2][j] * ndu[r-k+j][pk];
            }

            ders[k][r] = d;
            int temp = s1;
            s1 = s2;
            s2 = temp;
        }
    }

    int r = p;
    for (int k = 1; k <= n; k++) {
        for (int j = 0; j <= p; j++)
            ders[k][j] *= r;
        r *= (p - k);
    }

    // 메모리 해제
    for (int j = 0; j <= p; j++)
        delete[] ndu[j];
    delete[] ndu;
    delete[] left;
    delete[] right;
    delete[] a[0];
    delete[] a[1];
    delete[] a;
}
```

### 📌 주요 설명
- ndu: basis 함수 계산을 위한 삼각 테이블
- ders[k][j]: k차 도함수에서 j번째 basis 함수의 값
- a[2][p+1]: 도함수 계산을 위한 보조 테이블
- 마지막 루프에서 계수 p(p-1)...(p-k+1)을 곱해 최종 도함수 값 완성
- 이 알고리즘들은 B-spline 곡선 및 곡면을 계산할 때 매우 중요한 역할을 합니다.

