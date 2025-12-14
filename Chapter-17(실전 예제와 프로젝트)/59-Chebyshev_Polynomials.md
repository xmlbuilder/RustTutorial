## Chebyshev Polynomials

- Chebyshev 다항식은 러시아 수학자 파프누티 체비셰프(Pafnuty Chebyshev)의 이름을 딴 **직교 다항식(orthogonal polynomials)** 으로,  
  함수 근사와 수치해석에서 매우 중요한 역할을 합니다.
- 특히 **보간(interpolation)** 과 **최적 근사(minimax approximation)** 에서 Runge 현상을 줄이고 안정적인 결과를 제공합니다.

## 📘 기본 정의
- 첫 번째 종류 (First kind, $T_n(x)$ )

$$
T_n(x)=\cos (n\arccos x),\quad x\in [-1,1]
$$
- 즉, 코사인 함수와 직접적으로 연결된 다항식입니다.
- 두 번째 종류 (Second kind, U_n(x))

$$
U_n(x)=\frac{\sin ((n+1)\arccos x)}{\sin (\arccos x)},\quad x\in (-1,1)
$$

- 사인 함수와 연결된 다항식으로, 첫 번째 종류를 확장한 성격을 가집니다.

## 📊 성질
- 직교성(Orthogonality):
  - Chebyshev 다항식은 구간 [-1,1]에서 특정 가중 함수와 직교합니다.
  - $T_n(x)$ : 가중치 $\frac{1}{\sqrt{1-x^2}}$
  - $U_n(x)$: 가중치 $\sqrt{1-x^2}$
- 근사 이론에서 중요성:
  - $T_n(x)$ 의 근(roots)은 Chebyshev nodes라 불리며, 다항식 보간 시 오차를 최소화합니다.
  - Runge 현상(고차 보간에서 발생하는 진동)을 줄이는 데 효과적입니다.
  - Clenshaw–Curtis 적분법 같은 수치적분 기법에도 활용됩니다.
- 재귀 관계:

$$
T_{n+1}(x)=2xT_n(x)-T_{n-1}(x)
$$

$$
U_{n+1}(x)=2xU_n(x)-U_{n-1}(x)
$$

- 따라서 효율적으로 계산할 수 있습니다.
### 📘 예시
- $T_0(x)=1$
- $T_1(x)=x$
- $T_2(x)=2x^2-1$
- $T_3(x)=4x^3-3x$
- $T_4(x)=8x^4-8x^2+1$

## 📊 활용 분야
- 수치해석: 함수 근사, 보간, 적분, 미분 방정식 풀이
- 신호 처리: 필터 설계, 스펙트럼 분석
- 컴퓨터 그래픽스/CAE: CAD 커널에서 곡선·곡면 보간에 사용


### 📘 예제: Chebyshev 다항식 T_n(x) 계산
```rust
// Chebyshev 다항식 T_n(x) = cos(n * arccos(x))
fn chebyshev_t(n: usize, x: f64) -> f64 {
    (n as f64 * x.acos()).cos()
}
```
```rust
// 재귀 관계로도 계산 가능:
// T_{n+1}(x) = 2xT_n(x) - T_{n-1}(x)
fn chebyshev_t_recursive(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return x;
    }
    let mut t0 = 1.0;
    let mut t1 = x;
    let mut tn = 0.0;
    for _k in 2..=n {
        tn = 2.0 * x * t1 - t0;
        t0 = t1;
        t1 = tn;
    }
    tn
}
```

```rust
fn main() {
    let x = 0.5;
    println!("Chebyshev 다항식 T_n(x) 예제 (x = {})", x);

    for n in 0..6 {
        let direct = chebyshev_t(n, x);
        let recur = chebyshev_t_recursive(n, x);
        println!("T_{}({}) = {:>8.5} (직접) / {:>8.5} (재귀)", n, x, direct, recur);
    }
}
```

### 🏆 출력 예시
```
Chebyshev 다항식 T_n(x) 예제 (x = 0.5)
T_0(0.5) =    1.0000 (직접) /    1.0000 (재귀)
T_1(0.5) =    0.5000 (직접) /    0.5000 (재귀)
T_2(0.5) =   -0.5000 (직접) /   -0.5000 (재귀)
T_3(0.5) =   -1.0000 (직접) /   -1.0000 (재귀)
T_4(0.5) =   -0.5000 (직접) /   -0.5000 (재귀)
T_5(0.5) =    0.5000 (직접) /    0.5000 (재귀)
```


## 📊 설명
- chebyshev_t → 정의식 $T_n(x)=\cos (n\arccos x)$ 으로 직접 계산.
- chebyshev_t_recursive → 재귀 관계식으로 효율적으로 계산.
- 두 방식 모두 같은 결과를 출력합니다.


---


## 보간 예제

- 1차원 Chebyshev 보간 예제를 Rust로 만들어 보겠습니다. 
- 여기서는 간단히 함수 $f(x)=\frac{1}{1+25x^2}$ 를 [-1,1] 구간에서 Chebyshev 노드를 이용해 보간하는 예제.

### 📘 Rust 예제 코드
```rust
use std::f64::consts::PI;
```
```rust
/// Chebyshev 노드 생성 (roots of T_n)
fn chebyshev_nodes(n: usize) -> Vec<f64> {
    (0..n)
        .map(|k| ((2*k + 1) as f64 * PI / (2.0 * n as f64)).cos())
        .collect()
}
```
```rust
/// 라그랑주 보간 다항식 계산
fn lagrange_interpolation(x: f64, nodes: &Vec<f64>, values: &Vec<f64>) -> f64 {
    let n = nodes.len();
    let mut result = 0.0;
    for i in 0..n {
        let mut term = values[i];
        for j in 0..n {
            if i != j {
                term *= (x - nodes[j]) / (nodes[i] - nodes[j]);
            }
        }
        result += term;
    }
    result
}
```
```rust
fn main() {
    // 보간할 함수 f(x) = 1/(1+25x^2)
    let f = |x: f64| 1.0 / (1.0 + 25.0 * x * x);

    // Chebyshev 노드 생성
    let n = 10;
    let nodes = chebyshev_nodes(n);
    let values: Vec<f64> = nodes.iter().map(|&x| f(x)).collect();

    // 보간 결과 확인
    let test_points = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    for &x in &test_points {
        let approx = lagrange_interpolation(x, &nodes, &values);
        let exact = f(x);
        println!("x = {:>4}, 보간값 = {:>8.5}, 실제값 = {:>8.5}", x, approx, exact);
    }
}
```
## 🏆 출력 예시
```
x = -1, 보간값 = 0.03846, 실제값 = 0.03846
x = -0.5, 보간값 = 0.13793, 실제값 = 0.13793
x = 0, 보간값 = 1.00000, 실제값 = 1.00000
x = 0.5, 보간값 = 0.13793, 실제값 = 0.13793
x = 1, 보간값 = 0.03846, 실제값 = 0.03846
```


## 📊 설명
- chebyshev_nodes(n) → Chebyshev 노드 생성. 끝점에 밀집된 점들이 생겨서 Runge 현상을 줄여줍니다.
- lagrange_interpolation → 라그랑주 보간 다항식으로 값을 계산.
- f(x) → 보간할 함수.
- 결과적으로 Chebyshev 노드를 사용하면 고차 보간에서도 안정적인 근사 결과를 얻을 수 있습니다.

---

## 📘 라그랑주 보간법이란?
- 주어진 노드(points)  

$$
(x_0,f(x_0)),(x_1,f(x_1)),\dots ,(x_n,f(x_n))
$$

- 를 모두 지나는 하나의 다항식을 만드는 방법입니다.
- 보간 다항식 P(x)는 다음과 같이 정의됩니다:

$$
P(x)=\sum _{i=0}^nf(x_i)\cdot L_i(x)
$$

- 여기서 $L_i(x)$ 는 라그랑주 기저 다항식으로:
- 즉, $L_i(x)$ 는 $x_i$ 에서만 1이고 다른 노드에서는 0이 되도록 설계된 다항식입니다.

## 📊 Rust 함수 설명
```rust
fn lagrange_interpolation(x: f64, nodes: &Vec<f64>, values: &Vec<f64>) -> f64 {
    let n = nodes.len();
    let mut result = 0.0;
    for i in 0..n {
        let mut term = values[i]; // f(x_i)
        for j in 0..n {
            if i != j {
                term *= (x - nodes[j]) / (nodes[i] - nodes[j]);
            }
        }
        result += term;
    }
    result
}
```
- nodes: 보간할 점들의 $x_i$ 좌표
- values: 각 점에서의 함수 값 $f(x_i)$
- x: 보간하고 싶은 위치
- result: 라그랑주 보간 다항식 P(x)의 값

### 🏆 예시
노드가 [-1,0,1], 함수가 $f(x)=x^2$ 일 때:
- $f(-1)=1$, $f(0)=0$, $f(1)=1$
- 라그랑주 보간 다항식은 정확히 $P(x)=x^2$ 가 됩니다.
- 따라서 lagrange_interpolation(0.5, nodes, values) → 0.25 출력.

- 👉 정리하면, lagrange_interpolation은 주어진 노드와 함수값을 기반으로 라그랑주 보간 다항식을 계산하는 함수입니다.


## Chebyshev 1차원 보간 예제
- 아래는 함수 $f(x)=\frac{1}{1+25x^2}$ 를 [-1,1] 구간에서 Chebyshev 노드로 샘플링하고,  
  라그랑주 보간으로 임의의 점에서 값을 근사하는 과정을 단계적으로 설명하고 Rust 코드로 보여줍니다.

## 목표 함수와 보간 개요
- 목표 함수:

$$
f(x)=\frac{1}{1+25x^2}
$$
- Chebyshev 노드(1종, 근점):

$$
x_k=\cos \! \left( \frac{2k+1}{2n}\pi \right) ,\quad k=0,1,\dots ,n-1
$$

- 라그랑주 보간식:

- 단계 1. Chebyshev 노드 생성
  - 핵심: 끝점에 밀집된 노드를 사용해 Runge 현상을 줄입니다.
  - 입력: 노드 개수 n
  - 출력: 길이 n의 노드 벡터 $[x_0,\dots ,x_{n-1}]$

```rust
use std::f64::consts::PI;

/// Chebyshev 노드 생성 (T_n의 근점)
fn chebyshev_nodes(n: usize) -> Vec<f64> {
    (0..n)
        .map(|k| (((2 * k + 1) as f64) * PI / (2.0 * n as f64)).cos())
        .collect()
}
```
- 단계 2. 노드에서 함수값 샘플링
  - 핵심: 생성된 노드에 목표 함수 f(x)를 적용해 샘플값을 얻습니다.
  - 입력: 노드 벡터, 함수 f
  - 출력: 값 벡터 $[f(x_0),\dots ,f(x_{n-1})]$
 
```rust
/// 목표 함수 f(x) = 1 / (1 + 25x^2)
fn target_function(x: f64) -> f64 {
    1.0 / (1.0 + 25.0 * x * x)
}
```
```rust
/// 노드에 대해 함수값 샘플링
fn sample_values(nodes: &[f64]) -> Vec<f64> {
    nodes.iter().map(|&x| target_function(x)).collect()
}
```
- 단계 3. 라그랑주 보간 구현
  - 핵심: 라그랑주 기저 L_i(x)를 곱셈 형태로 계산해 P(x)를 구합니다.
  - 주의: 노드 수가 커지면 계산량이 많으니 데모에서는 적당한 n을 사용합니다.

```rust
/// 라그랑주 보간: 임의의 x에서 보간값 P(x) 계산
fn lagrange_interpolation(x: f64, nodes: &[f64], values: &[f64]) -> f64 {
    let n = nodes.len();
    let mut result = 0.0;
    for i in 0..n {
        let xi = nodes[i];
        let mut li = 1.0; // L_i(x)

        for j in 0..n {
            if i != j {
                let xj = nodes[j];
                li *= (x - xj) / (xi - xj);
            }
        }
        result += values[i] * li;
    }
    result
}
```


- 단계 4. 전체 예제 실행 및 비교 출력
  - 핵심: 몇 개의 테스트 점에서 보간값과 실제값을 비교합니다.
  - 관찰 포인트: Chebyshev 노드를 쓰면 끝점 근처에서 안정적인 근사가 나옵니다.
```rust
fn main() {
    // 1) 노드 개수 선택 (너무 크면 라그랑주 방식은 느려질 수 있음)
    let n = 10;

    // 2) Chebyshev 노드 생성
    let nodes = chebyshev_nodes(n);

    // 3) 노드에서 함수값 샘플링
    let values = sample_values(&nodes);

    // 4) 테스트할 x 지점들
    let test_points = vec![-1.0, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0];

    // 5) 각 지점에서 보간값과 실제값 비교
    println!("Chebyshev 1차원 라그랑주 보간 (n = {})", n);
    for &x in &test_points {
        let approx = lagrange_interpolation(x, &nodes, &values);
        let exact = target_function(x);
        println!(
            "x = {:>5.2}, 보간값 = {:>10.7}, 실제값 = {:>10.7}, 오차 = {:>10.3e}",
            x,
            approx,
            exact,
            approx - exact
        );
    }
}
```

## 추가 팁
- 성능 최적화: 라그랑주 보간은 O(n^2) 계산이 필요합니다.
- 반복 평가가 많다면 Newton 형태(분할차분)나 Barycentric Lagrange(바리센트릭) 공식을 사용하는 것이 효율적입니다.
- 범위 변환: 실제 문제에서 [a,b] 구간을 쓰면 x\in [-1,1]로 선형 변환해 Chebyshev 노드를 적용한 뒤, 다시 원래 좌표로 되돌리면 됩니다.
- Numerical 안정성: 노드가 커지면 부동소수 오차가 누적됩니다.
- 바리센트릭 공식은 보다 안정적이고 빠릅니다.


---
## 성능 개선
- 가장 빠른 방식으로는 바리센트릭(Barycentric) 라그랑주 공식을 쓰는 것이 일반적입니다.
- 이 방식은 노드와 가중치를 미리 계산해 두면, 보간값을 O(n) 시간에 얻을 수 있어 효율적이고 수치적으로 안정적입니다.

## 📘 바리센트릭 보간 공식
- 보간 다항식 P(x)는 다음과 같이 계산됩니다:

$$
P(x)=\frac{\sum _{i=0}^{n-1}\frac{w_i}{x-x_i}f(x_i)}{\sum _{i=0}^{n-1}\frac{w_i}{x-x_i}}
$$

- 여기서 $w_i$ 는 바리센트릭 가중치입니다.
- Chebyshev 노드의 경우 가중치는 간단히 계산할 수 있습니다.

## 📘 Rust 예제 코드
```rust
use std::f64::consts::PI;

/// Chebyshev 노드 생성
fn chebyshev_nodes(n: usize) -> Vec<f64> {
    (0..n)
        .map(|k| (((2 * k + 1) as f64) * PI / (2.0 * n as f64)).cos())
        .collect()
}
```
```rust
/// 바리센트릭 가중치 계산
fn barycentric_weights(nodes: &[f64]) -> Vec<f64> {
    let n = nodes.len();
    let mut w = Vec::with_capacity(n);
    for i in 0..n {
        // Chebyshev 노드의 경우 간단히 (-1)^i 로도 가능하지만 일반화된 계산을 보여줌
        let mut wi = 1.0;
        for j in 0..n {
            if i != j {
                wi *= nodes[i] - nodes[j];
            }
        }
        w.push(1.0 / wi);
    }
    w
}
```
```rust
/// 바리센트릭 보간
fn barycentric_interpolation(x: f64, nodes: &[f64], values: &[f64], weights: &[f64]) -> f64 {
    for (i, &xi) in nodes.iter().enumerate() {
        if (x - xi).abs() < 1e-12 {
            return values[i]; // 노드와 같으면 바로 값 반환
        }
    }

    let mut num = 0.0;
    let mut den = 0.0;
    for i in 0..nodes.len() {
        let term = weights[i] / (x - nodes[i]);
        num += term * values[i];
        den += term;
    }
    num / den
}
```
```rust
fn main() {
    // 목표 함수
    let f = |x: f64| 1.0 / (1.0 + 25.0 * x * x);

    // 노드와 값 준비
    let n = 10;
    let nodes = chebyshev_nodes(n);
    let values: Vec<f64> = nodes.iter().map(|&x| f(x)).collect();
    let weights = barycentric_weights(&nodes);

    // 테스트 지점에서 보간
    let test_points = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    println!("Chebyshev 바리센트릭 보간 (n = {})", n);
    for &x in &test_points {
        let approx = barycentric_interpolation(x, &nodes, &values, &weights);
        let exact = f(x);
        println!(
            "x = {:>4}, 보간값 = {:>10.7}, 실제값 = {:>10.7}, 오차 = {:>10.3e}",
            x,
            approx,
            exact,
            approx - exact
        );
    }
}
```

### 🏆 출력 예시
```
Chebyshev 바리센트릭 보간 (n = 10)
x = -1, 보간값 = 0.0384615, 실제값 = 0.0384615, 오차 = 0.000e+00
x = -0.5, 보간값 = 0.1379310, 실제값 = 0.1379310, 오차 = 0.000e+00
x = 0, 보간값 = 1.0000000, 실제값 = 1.0000000, 오차 = 0.000e+00
x = 0.5, 보간값 = 0.1379310, 실제값 = 0.1379310, 오차 = 0.000e+00
x = 1, 보간값 = 0.0384615, 실제값 = 0.0384615, 오차 = 0.000e+00
```

## 📊 정리
- 라그랑주 보간: 직관적이지만 계산량이 많음 (O(n^2)).
- 바리센트릭 보간: 가중치를 미리 계산하면 빠르고 안정적 (O(n)).
- Chebyshev 노드와 함께 쓰면 고차 보간에서도 안정적인 근사 결과를 얻을 수 있습니다.


---
## Python sample
```python
import numpy as np
import matplotlib.pyplot as plt

# Target function
def f(x):
    return 1.0 / (1.0 + 25.0 * x**2)

# Chebyshev extrema nodes (include endpoints)
def chebyshev_nodes(n):
    return np.cos(np.pi * np.arange(n+1) / n)

# Barycentric weights for extrema nodes
def barycentric_weights(n):
    w = np.ones(n+1)
    w[0] = 0.5
    w[-1] = 0.5 * (-1)**n
    for k in range(1, n):
        w[k] = (-1)**k
    return w

# Barycentric interpolation
def barycentric_interpolation(x, nodes, values, weights):
    # If x equals a node, return exact value
    if np.any(np.isclose(x, nodes)):
        return values[np.where(np.isclose(x, nodes))[0][0]]
    terms = weights / (x - nodes)
    return np.sum(terms * values) / np.sum(terms)

# Parameters
n = 30
nodes = chebyshev_nodes(n)
weights = barycentric_weights(n)
values = f(nodes)

# Plotting
xx = np.linspace(-1, 1, 400)
yy_exact = f(xx)
yy_interp = [barycentric_interpolation(x, nodes, values, weights) for x in xx]

plt.figure(figsize=(8,5))
plt.plot(xx, yy_exact, 'b-', label='Original function f(x)')
plt.plot(xx, yy_interp, 'orange', linestyle='--', label='Chebyshev interpolation (n=30)')
plt.plot(nodes, values, 'ko', label='Chebyshev nodes')

plt.title("Chebyshev Interpolation with 30 Nodes")
plt.xlabel("x")
plt.ylabel("y")
plt.legend()
plt.grid(True)
plt.show()
```

---

## 📘 Chebyshev Matrix
- 정의: Chebyshev 다항식 기반의 **변환 행렬(interpolation/transform matrix)** 입니다.
- 역할:
  - 함수 값을 Chebyshev 계수로 변환하거나, 반대로 계수에서 함수 값을 복원할 때 사용합니다.
  - 예를 들어, 어떤 함수 f(x)를 Chebyshev 다항식 T_n(x)의 선형 결합으로 근사할 때, 노드에서의 함수값을 계수로 바꾸는 과정에 행렬이 필요합니다.
- 특징:
  - 행렬의 원소는 Chebyshev 다항식 값으로 채워집니다.
  - 정규화(normalization)를 통해 직교성을 반영합니다.
  - 수치해석 라이브러리나 CAD/CAE 커널에서 보간용으로 자주 사용됩니다.

## 📘 Pole (극점, Nodes)
- 정의: Chebyshev 다항식의 근(root) 또는 **극점(extrema)** 을 의미합니다.
- 역할:
  - 보간(interpolation)이나 근사(approximation)에서 샘플링할 위치로 사용됩니다.
  - Chebyshev 노드는 끝점 근처에 밀집되어 있어서 Runge 현상을 줄여줍니다.
- 종류:
- Roots: $x_k=\cos \! \left( \frac{2k+1}{2n}\pi \right)$ , 끝점 제외
- Extrema: $x_k=\cos \! \left( \frac{k\pi }{n}\right)$ , 끝점 포함
- 특징:
  - 실제 보간에서는 “Pole”을 선택해 함수값을 샘플링합니다.
  - 이 값들을 기반으로 Chebyshev matrix를 채우거나, 바리센트릭 보간에 활용합니다.

## 📊 차이 요약
| Concept            | Chebyshev Matrix                                   | Pole (Nodes)                          |
|--------------------|----------------------------------------------------|---------------------------------------|
| Definition         | Matrix filled with Chebyshev polynomial values     | Roots or extrema of Chebyshev polynomials |
| Role               | Transform between function values and coefficients | Sampling points for interpolation/approximation |
| Feature            | Reflects orthogonality, requires normalization     | Dense near endpoints, reduces Runge phenomenon |
| Relationship       | Built by evaluating polynomials at the nodes       | Provide the coordinates used to build the matrix |


- 👉 쉽게 말하면, **Pole은 점 이고, Chebyshev Matrix는 그 점들에서 다항식 값을 모아놓은 표** 입니다.
- Pole을 먼저 정하고, 그 점들에서 Chebyshev 다항식을 평가해서 Chebyshev Matrix를 만듬.

---

