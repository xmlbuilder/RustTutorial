# Bezier 곡선
## 1. Bézier 곡선의 정의
- n차 Bézier 곡선:
```math
\mathbf{C}(u)=\sum _{i=0}^nB_{i,n}(u)\, \mathbf{P_{\mathnormal{i}}},\quad 0\leq u\leq 1
```
- 여기서 $$\mathbf{P_{\mathnormal{i}}}$ 는 control point,
- $B_{i,n}(u)$ 는 n차 Bernstein 다항식:
```math
B_{i,n}(u)={n \choose i}u^i(1-u)^{n-i}=\frac{n!}{i!(n-i)!}u^i(1-u)^{n-i}
```
## 2. Bernstein 다항식의 성질 (네가 그림에 적어둔 것들 수식화)
### 2.1 비음수성 (nonnegativity)
```math
B_{i,n}(u)\geq 0,\quad \forall i,\  n,\  0\leq u\leq 1
```
### 2.2 partition of unity
```math
\sum _{i=0}^nB_{i,n}(u)=1,\quad \forall 0\leq u\leq 1
```
### 2.3 끝점 값
```math
B_{0,n}(0)=1,\quad B_{i>0,n}(0)=0
```

```math
B_{n,n}(1)=1,\quad B_{i\lt n,n}(1)=0
```

### 2.4 최대값 위치
```math
B_{i,n}(u)\  \mathrm{는}\  u=\frac{i}{n}\  \mathrm{에서\  정확히\  한\  번\  최대값을\  갖는다.}
```
- 정확히 쓰면:
```math
\arg \max _{u\in [0,1]}B_{i,n}(u)=\frac{i}{n}
```
### 2.5 대칭성 (symmetry)
```math
B_{i,n}(u)=B_{n-i,n}(1-u)
```
- 즉, $B_{i,n}(u)$ 는 에 대해 대칭.
### 2.6 재귀 정의 (de Casteljau와 연결되는 형태)
```math
B_{i,n}(u)=(1-u)\, B_{i,n-1}(u)+u\, B_{i-1,n-1}(u)
```
여기서 i<0 또는 i>n이면 $B_{i,n}(u)\equiv 0$ 로 정의.
### 2.7 도함수

- 경계에서:
```math
B_{-1,n-1}(u)\equiv 0,\quad B_{n,n-1}(u)\equiv 0
```
## 3. 저차 예시들 (n=1,2)
### 3.1 n=1
```math
B_{0,1}(u)=1-u,\quad B_{1,1}(u)=u
```
```math
\mathbf{C}(u)=(1-u)\mathbf{P_{\mathnormal{0}}}+u\mathbf{P_{\mathnormal{1}}}
```
### 3.2 n=2
```math
B_{0,2}(u)=(1-u)^2
```
```math
B_{1,2}(u)=2u(1-u)
```
```math
B_{2,2}(u)=u^2
```
---
## Bernstein (또는 Bézier) basis의 도함수 수식
```math
\frac{d}{du} B_{i,n}(u)
= B'_{i,n}(u)
= n\big( B_{i-1,n-1}(u) - B_{i,n-1}(u) \big)
```

```math
B_{-1,n-1}(u) \equiv 0,\quad B_{n,n-1}(u) \equiv 0
```
---
## 소스 코드
```rust
/// Bezier 곡선 평가 (제어점과 매개변수 u)
pub fn on_bezier_curve(control_points: &[Point2D], u: Real) -> Point2D {
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
pub fn on_bezier_curve_derivative(control_points: &[Point2D], u: Real) -> Point2D {
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
```rust

pub fn on_bernstein(p: usize, i: usize, u: Real) -> Real {
    assert!(i <= p && u >= 0.0 && u <= 1.0);
    let mut tmp = vec![0.0; p + 1];
    tmp[p - i] = 1.0;
    let omu = 1.0 - u;
    for k in 1..=p {
        for j in (k..=p).rev() {
            tmp[j] = omu * tmp[j] + u * tmp[j - 1];
        }
    }
    tmp[p]
}
```
```rust
#[allow(unused)]
pub fn on_bernstein_der(i: usize, n: usize, t: Real) -> Real {
    // d/dt of B_{i,n}(t)  (assuming n=3)
    // Simplified implementation of the formula
    let b = on_bernstein(i, n, t);
    if t == 0.0 || t == 1.0 {
        return 0.0;
    } // 경계 안정화
    // 정확식: B'_{i,n} = n*(B_{i-1,n-1} - B_{i,n-1})
    let b_im1 = if i > 0 {
        on_bernstein(i - 1, n - 1, t)
    } else {
        0.0
    };
    let b_i = on_bernstein(i, n - 1, t);
    (n as Real) * (b_im1 - b_i)
}
```
---

