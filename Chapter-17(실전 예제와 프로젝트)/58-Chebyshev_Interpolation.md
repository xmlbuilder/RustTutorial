## 📘 Chebyshev Interpolation 설명
- Chebyshev interpolation은 Chebyshev 다항식을 기반으로 한 보간(interpolation) 기법입니다.
  - 일반적인 다항식 보간은 Runge 현상(진동 문제)이 발생할 수 있는데, Chebyshev 노드를 사용하면 이를 크게 줄일 수 있습니다.
  - 특히 곡선/곡면 보간에서 끝점의 접선(derivative) 조건을 자연스럽게 반영할 수 있어, Bezier 곡선과 유사한 방식으로 매끄러운 보간을 제공합니다.
  - 위 코드에서는:
  - q: 주어진 점들 (control points)
  - u: 시작점에서의 접선 벡터
  - v: 끝점에서의 접선 벡터
- InterpolationMatrix(m): Chebyshev 계수 행렬을 생성하여 각 점과 접선에 대한 가중치를 계산
- 결과적으로 positionArray: 보간된 점들의 배열을 반환

  


## 🥇 Step 1: Chebyshev 다항식 정의
- Chebyshev 다항식 T_n(x) 은 다음과 같이 정의됩니다:

$$
T_n(x)=\cos (n\cdot \arccos (x)),\quad -1\leq x\leq 1
$$

- Rust 코드로:
```rust
pub fn chebyshev_polynomial(n: usize, x: f64) -> f64 {
    (n as f64 * x.acos()).cos()
}
```


## 🥈 Step 2: Chebyshev 노드 (Interpolation Nodes)
- 보간에 필요한 노드 x_k 는 다음과 같이 정의됩니다:

$$
x_k=\cos \left( \frac{(2k-1)\pi }{2n}\right) ,\quad k=1,\dots ,n
$$

- Rust 코드:
```rust
pub fn chebyshev_nodes(n: usize) -> Vec<f64> {
    (1..=n)
        .map(|k| ((std::f64::consts::PI * (2.0 * k as f64 - 1.0)) / (2.0 * n as f64)).cos())
        .collect()
}
```


## 🥉 Step 3: Interpolation Matrix 생성
- 문서에서 사용된 InterpolationMatrix(m) 은 Chebyshev 노드와 다항식을 기반으로 행렬을 구성합니다.
- 일반적으로:

$$
M_{i,j}=T_i(x_j)
$$

- Rust 코드:
```rust
pub fn chebyshev_interpolation_matrix(m: usize) -> Vec<Vec<f64>> {
    let nodes = chebyshev_nodes(m);
    let mut matrix = vec![vec![0.0; m]; m];

    for i in 0..m {
        for j in 0..m {
            matrix[i][j] = chebyshev_polynomial(i, nodes[j]);
        }
    }

    matrix
}
```

## 🥇 Step 4: 행렬 정규화 (Normalization)
- Chebyshev 보간 행렬은 단순히 $M_{i,j}=T_i(x_j)$ 로 구성되지만, 실제 보간에 사용할 때는 가중치(normalization) 를 적용해야 합니다.
- 일반적으로 Chebyshev 보간은 직교성을 이용해 다음과 같은 정규화 계수를 둡니다:

$$
w_k =
\begin{cases}
\frac{1}{n}, & k = 0 \\
\frac{2}{n}, & k > 0
\end{cases}
$$

- 이 가중치를 통해 보간 행렬을 안정화합니다.
- Rust 코드:
```rust
pub fn normalize_matrix(matrix: &mut Vec<Vec<f64>>) {
    let n = matrix.len();
    for i in 0..n {
        let weight = if i == 0 { 1.0 / n as f64 } else { 2.0 / n as f64 };
        for j in 0..n {
            matrix[i][j] *= weight;
        }
    }
}
```
## 🥈 Step 5: 보간 점 계산 (Interpolation Points)
- 앞서 정의한 chebyshev_interpolation 함수에서, 이제 행렬을 불러와서 점과 접선 벡터를 조합합니다.
- 수식은 다음과 같습니다:
  - 끝점 조건:

$$  
p_0=q_0,\quad p_1=q_0+\frac{1}{m}u,\quad p_{m-1}=q_{m-2}-\frac{1}{m}v,\quad p_m=q_{m-2}
$$


- 내부 점:

$$
p_i=C_{0,i}\cdot u+C_{1,i}\cdot v+\sum _{j=2}^mC_{j,i}\cdot q_{j-2} \quad  i=2,\dots ,m-2
$$

- Rust 코드:

```rust
pub fn chebyshev_interpolation(q: &[Point3D], u: Vector3D, v: Vector3D) -> Vec<Point3D> {
    let m = q.len() + 1;
    let length = m + 1;

    let mut matrix = chebyshev_interpolation_matrix(m);
    normalize_matrix(&mut matrix);

    let mut position_array: Vec<Point3D> = vec![Point3D::default(); length];
    let num = 1.0 / (m as f64);

    // 끝점 조건
    position_array[0] = q[0].clone();
    position_array[1] = q[0].clone() + (u * num);
    position_array[m - 1] = q[m - 2].clone() - (v * num);
    position_array[m] = q[m - 2].clone();

    // 내부 점 계산
    for i in 2..(m - 1) {
        position_array[i] = (u * matrix[0][i]) + (v * matrix[1][i]);

        for j in 2..(m + 1) {
            position_array[i] = position_array[i].clone() + (q[j - 2].clone() * matrix[j][i]);
        }
    }

    position_array
}
```

## 🏆 Step 6: 전체 흐름
- Chebyshev 다항식 정의
  - Chebyshev 노드 생성
  - Interpolation Matrix 생성
  - 행렬 정규화 (가중치 적용)
  - 끝점 조건 반영
  - 내부 점 계산 (접선 + 주어진 점들의 선형 결합)
  - 최종 보간 점 배열 반환

---

## 소스 코드
```rust
use crate::core::prelude::{Point3D, Vector3D};

pub fn chebyshev_polynomial(n: usize, x: f64) -> f64 {
    (n as f64 * x.acos()).cos()
}
```
```rust
pub fn chebyshev_interpolation_matrix(m: usize) -> Vec<Vec<f64>> {
    let nodes = chebyshev_nodes(m);
    let mut matrix = vec![vec![0.0; m]; m];

    for i in 0..m {
        for j in 0..m {
            matrix[i][j] = chebyshev_polynomial(i, nodes[j]);
        }
    }

    matrix
}
```
```rust
pub fn chebyshev_nodes(n: usize) -> Vec<f64> {
    (1..=n)
        .map(|k| ((std::f64::consts::PI * (2.0 * k as f64 - 1.0)) / (2.0 * n as f64)).cos())
        .collect()
}
```
```rust
/// Chebyshev 보간 행렬을 정규화하는 함수
/// 각 행에 대해 weight를 곱해줍니다.
/// - 첫 번째 행은 1/n
/// - 나머지 행은 2/n
pub fn normalize_matrix(matrix: &mut Vec<Vec<f64>>) {
    let n = matrix.len();
    for i in 0..n {
        let weight = if i == 0 {
            1.0 / n as f64
        } else {
            2.0 / n as f64
        };
        for j in 0..n {
            matrix[i][j] *= weight;
        }
    }
}
```
```rust
pub fn chebyshev_interpolation(q: &[Point3D], u: Vector3D, v: Vector3D) -> Vec<Point3D> {
    let m = q.len() + 1;
    let length = m + 1;

    let mut matrix = chebyshev_interpolation_matrix(m);
    normalize_matrix(&mut matrix);

    let mut position_array: Vec<Point3D> = vec![Point3D::default(); length];
    let num = 1.0 / (m as f64);

    // 끝점 조건
    position_array[0] = q[0].clone();
    position_array[1] = q[0].clone() + (u * num).to_point();
    position_array[m - 1] = q[m - 2].clone() - (v * num).to_point();
    position_array[m] = q[m - 2].clone();

    // 내부 점 계산
    for i in 2..(m - 1) {
        position_array[i] = (u * matrix[0][i]).to_point() + (v * matrix[1][i]).to_point();

        for j in 2..(m + 1) {
            position_array[i] = position_array[i].clone() + (q[j - 2].clone() * matrix[j][i]);
        }
    }
    position_array
}
```
---

