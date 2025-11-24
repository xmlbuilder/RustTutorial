# Horner
요약: Horner 방법(호너 법칙)은 다항식 $p(t)=a_0+a_1t+a_2t^2+\cdots +a_nt^n$ 을 **중첩된 형태(nested form)**  
로 변환하여 곱셈과 덧셈만으로 효율적으로 계산하는 알고리즘입니다.  
이 방식은 계산량을 최소화하며, 수치적으로도 안정적입니다.

## 🔎 Horner 정리(Horner’s method) 핵심 아이디어
- 일반적인 다항식 계산은 각 항마다 거듭제곱을 구해야 해서 비효율적입니다.
- Horner 방법은 다항식을 다음과 같이 중첩된 형태로 씁니다:

$$
p(t)=a_0+t(a_1+t(a_2+\cdots +t(a_{n-1}+ta_n)\cdots ))
$$

- 이 표현을 이용하면 곱셈 n번 + 덧셈 n번만으로 계산할 수 있습니다.
- 시간 복잡도는 O(n)이며, 이는 다항식 평가에서 최적입니다.

## ✅ 계산 절차
- 결과값을 최고차항 계수 a_n으로 초기화합니다.
- 차수를 하나씩 줄여가며:

$$
result=result\cdot t+a_i
$$

- 를 반복합니다.
- 마지막에 result가 $p(t)$ 값이 됩니다.

### 📌 예시
다항식 $p(t)=2t^3-6t^2+2t-1$ 을 $t=3$ 에서 계산한다고 합시다.
- Horner 변환:

$$
p(t)=((2t-6)t+2)t-1
$$

- 계산 과정:
    - result = 2
    - result = 2*3 - 6 = 0
    - result = 0*3 + 2 = 2
    - result = 2*3 - 1 = 5
    - 따라서 p(3)=5.

## 📚 응용
- 다항식 평가: 빠르고 안정적으로 값 계산.
- 다항식 나눗셈/근 찾기: Synthetic division과 연결되어 근사 근 찾기에도 활용.
- 수치해석: Newton 방법에서 다항식 및 도함수 평가에 자주 사용.
- 정리: Horner 정리는 다항식을 중첩된 형태로 변환해 최소 연산으로 평가하는 방법입니다.  
- 구현은 단순한 루프 구조로 가능하며, 공학·수치해석·컴퓨터 그래픽스 등 다양한 분야에서 기본 알고리즘으로 쓰입니다.

---

## Rust 버전 Horner 함수들
```rust
/// Horner 방법: 다항식 평가 (coefficients[0] + coefficients[1] t + ... + coefficients[n] t^n)
pub fn horner(coefficients: &[f64], t: f64) -> f64 {
    let mut result = 0.0;
    for &c in coefficients.iter().rev() {
        result = result * t + c;
    }
    result
}
```
```rust
/// Horner 방법 (오름차순: a0 + a1 t + ... + an t^n)
pub fn horner_ascending(coefficients: &[f64], t: f64) -> f64 {
    let mut s = 0.0;
    for i in (0..coefficients.len()).rev() {
        s = s * t + coefficients[i];
    }
    s
}
```
```rust
/// Horner 방법 (내림차순: an + ... + a0)
pub fn horner_descending(coefficients: &[f64], t: f64) -> f64 {
    let mut s = 0.0;
    for &c in coefficients.iter() {
        s = s * t + c;
    }
    s
}
```
```rust
/// 2차원 Horner (예: Bezier surface 평가)
/// coefficients: 행렬 형태 [degree_u+1][degree_v+1]
pub fn horner_2d(coefficients: &[Vec<f64>], u: f64, v: f64) -> f64 {
    let degree_u = coefficients.len() - 1;
    let mut temp = Vec::with_capacity(degree_u + 1);

    for row in coefficients {
        temp.push(horner(row, v));
    }
    horner(&temp, u)
}
```


### ✅ 사용 예시
```rust
fn main() {
    // p(t) = 2t^3 - 6t^2 + 2t - 1
    let coeffs = vec![-1.0, 2.0, -6.0, 2.0];
    let val = horner(&coeffs, 3.0);
    println!("p(3) = {}", val); // 결과: 5

    // 2D 예시 (Bezier surface)
    let coeffs2d = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let val2d = horner_2d(&coeffs2d, 0.5, 0.5);
    println!("Surface(0.5,0.5) = {}", val2d);
}
```


## 📌 특징
- Rust에서는 Vec<f64>나 &[f64] slice를 받아 안전하게 처리.
- horner, horner_ascending, horner_descending 모두 구현.
- 2차원 Horner(horner_2d)로 곡면 평가도 가능.



## Bezier 평가 함수

Bezier 곡선은 제어점 $P_i$ 와  Bernstein basis $B_i^n(u)$ 의 선형 결합으로 정의됩니다:

$$
C(u)=\sum _{i=0}^nP_i\, B_i^n(u)
$$

## 🦀 Rust 구현 예시

```rust
/// Bezier 곡선 평가 (제어점과 매개변수 u)
pub fn bezier_curve(control_points: &[Point2D], u: f64) -> Point2D {
    let n = control_points.len() - 1;
    let mut x = 0.0;
    let mut y = 0.0;
    for (i, p) in control_points.iter().enumerate() {
        let b = on_bernstein(n, i, u);
        x += p.x * b;
        y += p.y * b;
    }
    Point2D { x, y }
}
```

```rust
/// Bezier 곡선의 도함수 (속도 벡터)
pub fn bezier_curve_derivative(control_points: &[Point2D], u: f64) -> Point2D {
    let n = control_points.len() - 1;
    let mut dx = 0.0;
    let mut dy = 0.0;
    for (i, p) in control_points.iter().enumerate() {
        let b_der = on_bernstein_der(n, i, u);
        dx += p.x * b_der;
        dy += p.y * b_der;
    }
    Point2D { x: dx, y: dy }
}
```


### ✅ 사용 예시
```rust
fn main() {
    let control_points = vec![
        Point2D { x: 0.0, y: 0.0 },
        Point2D { x: 1.0, y: 2.0 },
        Point2D { x: 3.0, y: 3.0 },
        Point2D { x: 4.0, y: 0.0 },
    ];

    let u = 0.5;
    let point = bezier_curve(&control_points, u);
    let tangent = bezier_curve_derivative(&control_points, u);

    println!("Bezier(0.5) = ({}, {})", point.x, point.y);
    println!("Bezier'(0.5) = ({}, {})", tangent.x, tangent.y);
}
```


### 📌 설명
- bezier_curve: Bernstein basis를 이용해 Bezier 곡선 점을 계산.
- bezier_curve_derivative: Bernstein basis의 도함수를 이용해 곡선의 접선 벡터(속도)를 계산.
- 제어점이 4개라면 3차 Bezier 곡선이 됩니다.


## 2차원 Horner / Bezier Surface

앞서 만든 Horner와 Bernstein을 이용해서 2차원 Horner 평가와 이를 기반으로 한 Bezier Surface 확장.

## 🦀 2차원 Horner 함수
```rust
/// 1차원 Horner (다항식 평가)
pub fn horner(coeffs: &[f64], t: f64) -> f64 {
    let mut result = 0.0;
    for &c in coeffs.iter().rev() {
        result = result * t + c;
    }
    result
}
```

```rust
/// 2차원 Horner (행렬 계수 기반 다변수 다항식 평가)
/// coefficients: 행렬 형태 [degree_u+1][degree_v+1]
pub fn horner_2d(coefficients: &[Vec<f64>], u: f64, v: f64) -> f64 {
    let degree_u = coefficients.len() - 1;
    let mut temp = Vec::with_capacity(degree_u + 1);

    for row in coefficients {
        temp.push(horner(row, v));
    }
    horner(&temp, u)
}
```


## 🧩 Bezier Surface 확장
Bezier Surface는 제어점 $P_{i,j}$ 와 Bernstein basis의 곱으로 정의됩니다:

$$
S(u,v)=\sum _{i=0}^n\sum _{j=0}^mP_{i,j}\, B_i^n(u)\, B_j^m(v)
$$

```rust
/// Bezier Surface 평가
pub fn bezier_surface(control_points: &[Vec<Point3D>], u: f64, v: f64) -> Point3D {
    let n = control_points.len() - 1;       // u 방향 차수
    let m = control_points[0].len() - 1;    // v 방향 차수

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    for i in 0..=n {
        let bu = bernstein(n, i, u);
        for j in 0..=m {
            let bv = bernstein(m, j, v);
            let p = control_points[i][j];
            let b = bu * bv;
            x += p.x * b;
            y += p.y * b;
            z += p.z * b;
        }
    }

    Point3D { x, y, z }
}
```


## ✅ 사용 예시
```rust
fn main() {
    // 3x3 제어점 (2차 Bezier Surface)
    let control_points = vec![
        vec![
            Point3D { x: 0.0, y: 0.0, z: 0.0 },
            Point3D { x: 0.0, y: 1.0, z: 0.0 },
            Point3D { x: 0.0, y: 2.0, z: 0.0 },
        ],
        vec![
            Point3D { x: 1.0, y: 0.0, z: 1.0 },
            Point3D { x: 1.0, y: 1.0, z: 1.0 },
            Point3D { x: 1.0, y: 2.0, z: 1.0 },
        ],
        vec![
            Point3D { x: 2.0, y: 0.0, z: 0.0 },
            Point3D { x: 2.0, y: 1.0, z: 0.0 },
            Point3D { x: 2.0, y: 2.0, z: 0.0 },
        ],
    ];

    let u = 0.5;
    let v = 0.5;
    let point = bezier_surface(&control_points, u, v);

    println!("Bezier Surface(0.5,0.5) = ({}, {}, {})", point.x, point.y, point.z);
}
```


## 📌 정리
- horner_2d: 2차원 계수 행렬을 Horner 방법으로 평가.
- bezier_surface: Bernstein basis를 이용해 Bezier Surface 점을 계산.
- 제어점 배열을 `Vec<Vec<Point3D>>` 로 두어 직관적으로 확장 가능.


```rust
#[cfg(test)]
mod honer_tests {
    use nurbslib::core::honer::{bezier_surface, horner, horner_2d};
    use nurbslib::core::prelude::Point3D;

    #[test]
    fn honer_test1() {
        // p(t) = 2t^3 - 6t^2 + 2t - 1
        let co_effs = vec![-1.0, 2.0, -6.0, 2.0];
        let val = horner(&co_effs, 3.0);
        println!("p(3) = {}", val); // 결과: 5

        // 2D 예시 (Bezier surface)
        let co_effs2d = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let val2d = horner_2d(&co_effs2d, 0.5, 0.5);
        println!("Surface(0.5,0.5) = {}", val2d);
    }
```    
```rust
    #[test]
    fn honer_test2() {
        // 3x3 제어점 (2차 Bezier Surface)
        let control_points = vec![
            vec![
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 0.0, y: 1.0, z: 0.0 },
                Point3D { x: 0.0, y: 2.0, z: 0.0 },
            ],
            vec![
                Point3D { x: 1.0, y: 0.0, z: 1.0 },
                Point3D { x: 1.0, y: 1.0, z: 1.0 },
                Point3D { x: 1.0, y: 2.0, z: 1.0 },
            ],
            vec![
                Point3D { x: 2.0, y: 0.0, z: 0.0 },
                Point3D { x: 2.0, y: 1.0, z: 0.0 },
                Point3D { x: 2.0, y: 2.0, z: 0.0 },
            ],
        ];

        let u = 0.5;
        let v = 0.5;
        let point = bezier_surface(&control_points, u, v);

        println!("Bezier Surface(0.5,0.5) = ({}, {}, {})", point.x, point.y, point.z);
    }

}
```

```rust
#[cfg(test)]
mod tests_case2 {
    use nurbslib::core::geom::Point2D;
    use nurbslib::core::honer::{bezier_curve, bezier_surface, horner, horner_2d};
    use nurbslib::core::prelude::Point3D;

    /// Horner 1D 테스트: p(t) = 2t^3 - 6t^2 + 2t - 1
    #[test]
    fn test_horner_1d() {
        let coeffs = vec![-1.0, 2.0, -6.0, 2.0]; // a0..a3
        let val = horner(&coeffs, 3.0);
        assert!((val - 5.0).abs() < 1e-12, "Expected 5, got {}", val);
    }
```
```rust
    /// Horner 2D 테스트: f(u,v) = u^2 + v^2
    #[test]
    fn test_horner_2d() {
        // 계수 행렬: f(u,v) = u^2 + v^2
        // 행렬 형태: [ [coeffs for v], ... ]
        let coeffs2d = vec![
            vec![0.0, 0.0, 1.0], // u^0: v^2 = 1
            vec![0.0, 0.0, 0.0], // u^1: 모두 0
            vec![1.0, 0.0, 0.0], // u^2: v^0 = 1

        ];
        let val = horner_2d(&coeffs2d, 2.0, 3.0); // 2^2 + 3^2 = 13
        assert!((val - 13.0).abs() < 1e-12, "Expected 13, got {}", val);
    }
```
```rust
    /// Bezier Curve 테스트 (3차)
    #[test]
    fn test_bezier_curve() {
        let control_points = vec![
            Point2D { x: 0.0, y: 0.0 },
            Point2D { x: 1.0, y: 2.0 },
            Point2D { x: 3.0, y: 3.0 },
            Point2D { x: 4.0, y: 0.0 },
        ];
        let u = 0.5;
        let point = bezier_curve(&control_points, u);
        // 대략적인 기대값 검증
        assert!((point.x - 2.0).abs() < 1e-6, "x ≈ 2, got {}", point.x);
        assert!((point.y - 1.875).abs() < 1e-6, "y ≈ 1.875, got {}", point.y);
    }
```
```rust
    /// Bezier Surface 테스트 (2차)
    #[test]
    fn test_bezier_surface() {
        let control_points = vec![
            vec![
                Point3D { x: 0.0, y: 0.0, z: 0.0 },
                Point3D { x: 0.0, y: 1.0, z: 0.0 },
                Point3D { x: 0.0, y: 2.0, z: 0.0 },
            ],
            vec![
                Point3D { x: 1.0, y: 0.0, z: 1.0 },
                Point3D { x: 1.0, y: 1.0, z: 1.0 },
                Point3D { x: 1.0, y: 2.0, z: 1.0 },
            ],
            vec![
                Point3D { x: 2.0, y: 0.0, z: 0.0 },
                Point3D { x: 2.0, y: 1.0, z: 0.0 },
                Point3D { x: 2.0, y: 2.0, z: 0.0 },
            ],
        ];

        let u = 0.5;
        let v = 0.5;
        let point = bezier_surface(&control_points, u, v);

        // 대략적인 기대값 검증 (중심 근처 값)
        assert!(point.x >= 0.5 && point.x <= 1.5, "x in [0.5,1.5], got {}", point.x);
        assert!(point.y >= 0.5 && point.y <= 1.5, "y in [0.5,1.5], got {}", point.y);
        assert!(point.z >= 0.0 && point.z <= 1.0, "z in [0,1], got {}", point.z);
    }
}
```

---
