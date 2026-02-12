## 📘 Tensor Product Surface 정리 + Horner 2D와의 연결
    - (Piegl & Tiller Section 1.5 기반)

### 1. 텐서 곱 곡면의 기본 정의
- 곡선은 1개의 파라미터 u를 가지는 함수:
```math
C(u)=(x(u),y(u),z(u))
```
- 곡면은 2개의 파라미터 u,v를 가지는 함수:
```math
S(u,v)=(x(u,v),y(u,v),z(u,v))
```
- Tensor product surface는 다음과 같은 형태를 가진다:

- 여기서:
    - $f_i(u)$: u-방향 basis
    - $g_j(v)$: v-방향 basis
    - $b_{i,j}$: control net의 점
- 즉, 두 방향의 basis를 곱해서 2D surface basis를 만든다.

### 2. Power Basis Surface의 특수 형태
- Power basis surface는 다음과 같다:

- 여기서:
    - $f_i(u)=u^i$
    - $g_j(v)=v^j$
즉, basis는 단순히 $u^iv^j$.

### 3. u를 고정하면 v-방향 곡선이 된다
```math
C_{u_0}(v)=S(u_0,v)
```
- 전개하면:

- 여기서:
```math
b_j(u_0)=\sum _{i=0}^na_{i,j}u_0^i
```
- 즉:
- 먼저 **각 column(j)** 에 대해 u-방향 power basis를 평가하여
- b_j(u_0)를 얻고
- 그 다음 v-방향 power basis로 다시 평가한다.
- 이 구조가 바로 2D Horner의 핵심이다.

### 4. Horner 2D 알고리즘과의 직접적 연결
- Power basis surface:
```math
S(u,v)=\sum _{i=0}^n\sum _{j=0}^ma_{i,j}u^iv^j
```
- 이를 Horner 방식으로 계산하면:
- ✔ Step 1 — v 방향 Horner (각 i에 대해)
```math
c_i(v)=(((a_{i,m}v+a_{i,m-1})v+\cdots )v+a_{i,0})
```
- ✔ Step 2 — u 방향 Horner
```math
S(u,v)=(((c_n(v)u+c_{n-1}(v))u+\cdots )u+c_0(v))
```
- 즉:
    - 2D Horner = v 방향 Horner → u 방향 Horner  
        (또는 반대로도 가능)


### 5. Algorithm A1.6 (Horner2D)의 의미
- 책의 알고리즘:
```rust
for i = 0..n:
    Horner1(a[i][], m, v0, b[i])   // v 방향 Horner
Horner1(b, n, u0, S)               // u 방향 Horner
```

- 이 알고리즘은 다음 수식을 그대로 구현한다:
- 각 row(i)에 대해
```math
c_i(v_0)=\sum _{j=0}^ma_{i,j}v_0^j
```
- v 방향 Horner
- 그 결과를 다시 u 방향으로 Horner
```math
S(u_0,v_0)=\sum _{i=0}^nc_i(v_0)u_0^i
````
- 즉:
- Tensor product surface의 power basis 평가는
    1D Horner를 두 번 적용한 것과 완전히 동일하다.
### 6. 왜 Tensor Product Surface = Horner 2D인가?
- Tensor product surface는 본질적으로:
```math
S(u,v)=\sum _{i,j}a_{i,j}u^iv^j
```
- 이중 합 구조이기 때문에:
- 먼저 v에 대해 1D polynomial 평가
- 그 결과를 u에 대해 다시 1D polynomial 평가
- 이 두 단계가 정확히 2D Horner 알고리즘이다.
- 즉:
- Tensor product surface는 1D polynomial evaluator(Horner)를  
    두 방향으로 확장한 것에 불과하다.
- 그래서 Piegl이 “curve 알고리즘을 그대로 확장하면 surface 알고리즘이 된다”고 말하는 것이다.

### 7. 한 문장 요약
- Tensor product surface는 두 방향의 power basis를 곱한 형태이며,
- 그 평가는 v 방향 Horner → u 방향 Horner의 2단계로 이루어진다.

---

## 1. 비유리 Bézier 곡면 수식 정리
### 1.1 Bézier 곡면 정의

- $B_{i,n}(u)$: u-방향 n차 Bernstein basis
- $B_{j,m}(v)$: v-방향 m차 Bernstein basis
- $P_{i,j}$: $(n+1)\times (m+1)$ control net의 점
### 1.2 u를 고정했을 때: v-방향 Bézier 곡선

- 여기서
```math
Q_j(u_0)=\sum _{i=0}^nB_{i,n}(u_0)\, P_{i,j},\quad j=0,\dots ,m
```
즉:
- 먼저 각 열 j에 대해 u-방향 Bézier 곡선을 평가하여 $Q_j(u_0)$ 를 얻고
- 그 $Q_j(u_0)$ 들을 v-방향 Bézier 곡선의 control point로 사용해 $C_{u_0}(v_0)$ 를 구한다.
- 이게 바로 deCasteljau2 알고리즘의 수식적 구조다.

### 2. deCasteljau2에서 “선형 보간 횟수”를 세는 이유
- de Casteljau 알고리즘은 **순수 선형 보간(linear interpolation)** 만으로 Bézier 곡선을 평가한다.
- 따라서:
- 1D Bézier 곡선: 차수 n → 선형 보간 횟수는 $\frac{n(n+1)}{2}$
- 2D Bézier 곡면: u, v 두 방향에 대해 de Casteljau를 적용 →  
    총 선형 보간 횟수를 세면 **계산 비용(연산량)** 을 정확히 알 수 있다.

### 3. 1D de Casteljau에서 선형 보간 횟수
- n차 Bézier 곡선에서:
    - 레벨 1: n번 보간
    - 레벨 2: n-1번 보간
    - …
    - 레벨 n: 1번 보간
- 합:
```math
n+(n-1)+\cdots +1=\frac{n(n+1)}{2}
```
- 이게 1D de Casteljau의 기본 비용이다.

### 4. Bézier 곡면에서의 선형 보간 횟수
#### 4.1 경우 1: 먼저 u 방향, 그 다음 v 방향 (n ≤ m일 때)
- 알고리즘 A1.7의 첫 번째 가지:
```cpp
if (n <= m)
{
  for (j=0; j<=m; j++)
    deCasteljau(P[j][], n, u0, Q[j]);  // u 방향
  deCasteljau(Q, m, v0, S);            // v 방향
}
```

- 1단계: 각 열 j에 대해 u-방향 de Casteljau
    - 각 열은 n차 Bézier 곡선
    - 한 번 평가에 \frac{n(n+1)}{2} 보간
    - 열이 총 m+1개
- 따라서:
```math
(m+1)\cdot \frac{n(n+1)}{2}
```
- 2단계: Q[j]들을 v-방향 Bézier 곡선으로 평가
    - m차 Bézier 곡선 1개
    - 보간 횟수: $\frac{m(m+1)}{2}$
- 총합:

##### 4.2 경우 2: 먼저 v 방향, 그 다음 u 방향 (n > m일 때)
- 알고리즘 A1.7의 두 번째 가지:
```cpp
else
{
  for (i=0; i<=n; i++)
    deCasteljau(P[][i], m, v0, Q[i]);  // v 방향
  deCasteljau(Q, n, u0, S);            // u 방향
}
```

- 1단계: 각 행 i에 대해 v-방향 de Casteljau
    - 각 행은 m차 Bézier 곡선
    - 한 번 평가에 $\frac{m(m+1)}{2}$ 보간
    - 행이 총 n+1개
```math
(n+1)\cdot \frac{m(m+1)}{2}
```
- 2단계: Q[i]들을 u-방향 Bézier 곡선으로 평가
    - n차 Bézier 곡선 1개
    - 보간 횟수: $\frac{n(n+1)}{2}$


### 5. 왜 두 가지 경우를 나누고, 작은 쪽을 먼저 처리하나?
- 핵심은 연산량 최소화다.
    - n ≤ m이면: u-방향(n차)을 (m+1)번, v-방향(m차)을 1번
    - n > m이면: v-방향(m차)을 (n+1)번, u-방향(n차)을 1번
- 항상 차수가 더 작은 방향을 여러 번,
    - 차수가 더 큰 방향을 한 번만 쓰도록 분기한다.
- 이렇게 하면:
    - $\frac{n(n+1)(m+1)}{2}+\frac{m(m+1)}{2}$
    - $\frac{m(m+1)(n+1)}{2}+\frac{n(n+1)}{2}$
- 중에서 더 작은 쪽을 자동으로 선택하게 된다.
- 즉:
    - deCasteljau2는 Bézier 곡면 평가를 위해 필요한 선형 보간 횟수를 최소화하기 위해  
        n과 m의 크기를 비교해서 방향을 선택한다.


### 6. 한 문장 요약
- 식 (1.25), (1.26)은 Bézier 곡면을 de Casteljau로 평가할 때
- u, v 두 방향에 대해 수행되는 선형 보간의 총 개수를 나타내며,
- 알고리즘은 항상 차수가 작은 방향을 먼저 처리해서
- 이 연산 횟수를 최소화하도록 설계되어 있다.

---

## 소스 코드
```rust
/// 2차원 Horner (예: Bezier surface 평가)
/// coefficients: 행렬 형태 [degree_u+1][degree_v+1]
pub fn on_horner_2d(coefficients: &[Vec<Real>], u: Real, v: Real) -> Real {
    let degree_u = coefficients.len() - 1;
    let mut temp = Vec::with_capacity(degree_u + 1);

    for row in coefficients {
        temp.push(on_horner(row, v));
    }
    on_horner(&temp, u)
}
```
```rust
pub fn on_bezier_surface(control_points: &[Vec<Point3D>], u: Real, v: Real) -> Point3D {
    let n = control_points.len().saturating_sub(1); // u 방향 차수
    let m = control_points[0].len().saturating_sub(1); // v 방향 차수

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    for i in 0..=n {
        let bu = on_bernstein(n, i, u);
        for j in 0..=m {
            let bv = on_bernstein(m, j, v);
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
```rust
/// Evaluate a polynomial curve in power basis using Horner's method.
/// aw[i] = coefficient of u^i
pub fn on_eval_power_basis_horner(aw: &[Point3D], u: Real) -> Point3D {
    let n = aw.len();
    if n == 0 {
        return Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    // Start with highest degree coefficient
    let mut c = aw[n - 1];

    // Horner iteration
    for i in (0..n - 1).rev() {
        c = c * u + aw[i];
    }

    c
}
```
## 테스트 코드
```rust
#[cfg(test)]
mod honer_tests {
    use nurbslib::core::honer::{on_bezier_surface, on_horner, on_horner_2d};
    use nurbslib::core::prelude::Point3D;

    #[test]
    fn horner_test() {
        // p(t) honer_test1= 2t^3 - 6t^2 + 2t - 1
        let co_effs = vec![-1.0, 2.0, -6.0, 2.0];
        let val = on_horner(&co_effs, 3.0);
        println!("p(3) = {}", val); // 결과: 5

        // 2D 예시 (Bezier surface)
        let co_effs2d = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let val2d = on_horner_2d(&co_effs2d, 0.5, 0.5);
        println!("Surface(0.5,0.5) = {}", val2d);
    }
```
```rust
    #[test]
    fn honer_test2() {
        // 3x3 제어점 (2차 Bezier Surface)
        let control_points = vec![
            vec![
                Point3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Point3D {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
                },
            ],
            vec![
                Point3D {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                Point3D {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Point3D {
                    x: 1.0,
                    y: 2.0,
                    z: 1.0,
                },
            ],
            vec![
                Point3D {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point3D {
                    x: 2.0,
                    y: 1.0,
                    z: 0.0,
                },
                Point3D {
                    x: 2.0,
                    y: 2.0,
                    z: 0.0,
                },
            ],
        ];

        let u = 0.5;
        let v = 0.5;
        let point = on_bezier_surface(&control_points, u, v);

        println!(
            "Bezier Surface(0.5,0.5) = ({}, {}, {})",
            point.x, point.y, point.z
        );
    }
}
```
```rust
#[cfg(test)]
mod tests_case2 {
    use nurbslib::core::geom::Point2D;
    use nurbslib::core::honer::{on_bezier_curve, on_bezier_surface, on_horner, on_horner_2d};
    use nurbslib::core::prelude::Point3D;

    /// Horner 1D 테스트: p(t) = 2t^3 - 6t^2 + 2t - 1
    #[test]
    fn test_horner_1d() {
        let coeffs = vec![-1.0, 2.0, -6.0, 2.0]; // a0..a3
        let val = on_horner(&coeffs, 3.0);
        assert!((val - 5.0).abs() < 1e-12, "Expected 5, got {}", val);
    }
```
```rust
    #[test]
    fn test_horner_2d_case2() {
        let coeffs2d = vec![
            vec![0.0, 0.0, 0.0, 1.0], // u^0: v^3 = 1
            vec![0.0, 0.0, 0.0, 0.0], // u^1: 모두 0
            vec![1.0, 0.0, 0.0, 0.0], // u^2: v^0 = 1
        ];
        let val = on_horner_2d(&coeffs2d, 2.0, 3.0); // 2^2 + 3^3 = 4 + 27 = 31
        assert!((val - 31.0).abs() < 1e-12, "Expected 31, got {}", val);
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
        let val = on_horner_2d(&coeffs2d, 2.0, 3.0); // 2^2 + 3^2 = 13
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
        let point = on_bezier_curve(&control_points, u);
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
                Point3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Point3D {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
                },
            ],
            vec![
                Point3D {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                Point3D {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Point3D {
                    x: 1.0,
                    y: 2.0,
                    z: 1.0,
                },
            ],
            vec![
                Point3D {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                Point3D {
                    x: 2.0,
                    y: 1.0,
                    z: 0.0,
                },
                Point3D {
                    x: 2.0,
                    y: 2.0,
                    z: 0.0,
                },
            ],
        ];

        let u = 0.5;
        let v = 0.5;
        let point = on_bezier_surface(&control_points, u, v);

        // 대략적인 기대값 검증 (중심 근처 값)
        assert!(
            point.x >= 0.5 && point.x <= 1.5,
            "x in [0.5,1.5], got {}",
            point.x
        );
        assert!(
            point.y >= 0.5 && point.y <= 1.5,
            "y in [0.5,1.5], got {}",
            point.y
        );
        assert!(
            point.z >= 0.0 && point.z <= 1.0,
            "z in [0,1], got {}",
            point.z
        );
    }
}
```
---
