# 수학 함수 설명 Chapter 2

```rust
use crate::approx_eq;
use crate::core::basis::{bernstein_der_3, on_basis_func, on_binomial,
on_find_span_index};
use crate::core::geom::{Point2D, Point4D, Vector2D};
use crate::core::matrix::{DenseMat, Matrix};
use crate::core::plane::Plane;
use crate::core::point_ops::PointOps;
use crate::core::poly_region2d::PolyRegion2d;
use crate::core::polygon2d::{Polygon2D, PolygonStatus};
use crate::core::prelude::{Degree, Interval, Point3D, Vector3D};
use crate::core::segment2d::{Segment2D, SegmentIntersectionType};
use crate::core::segment3d::Segment3D;
use crate::core::transform::Transform;
use crate::core::types::{Matrix3x3, ON_TOL9, ON_TOL12, ON_TOL14, Real, ON_EPSILON,
ON_UNSET_VALUE, ON_RADIANS_TO_DEGREES, ON_UNSET_POSITIVE_VALUE, ON_DEGREES_TO_RADIANS,
ON_SQRT2, ON_SQRT3, ON_SQRT_EPSILON, SQRT_EPS};
use nalgebra::{Cholesky, DMatrix, DVector};
use std::f64::consts::{PI, TAU};
use std::mem::swap;
use crate::core::tmatrix::TMatrix;
```

- 수학적 의미
- 이 함수는 두 실수 a, b에 대해 유클리드 거리(Euclidean norm)  
    또는 2차원 벡터의 길이를 계산한다.
- 즉, 벡터 (a,b)의 크기:
```math
\| (a,b)\| _2=\sqrt{a^2+b^2}
```
- Rust의 f64::hypot은 이 값을 수치적으로 안정적인 방식으로 계산한다.

- 왜 hypot을 쓰는가?
- 일반적으로:
```math
(a*a + b*b).sqrt()
```
- 로 계산할 수 있지만, 이 방식은:
    - a 또는 b가 매우 크거나 매우 작을 때
    - overflow 또는 underflow가 발생할 수 있다.
- hypot은 내부적으로 다음과 같은 안정화 기법을 사용한다:
```math
\sqrt{a^2+b^2}=\max (|a|,|b|)\cdot \sqrt{1+\left( \frac{\min (|a|,|b|)}{\max (|a|,|b|)}\right) ^2}
```
- 이 방식은:
    - 큰 수의 제곱으로 overflow 방지
    - 작은 수의 제곱으로 underflow 방지
    - IEEE754에서 가장 정확한 결과 제공

- 요약
    - 의미: 2D 벡터의 길이 $\sqrt{a^2+b^2}$
    - 수식: $\| (a,b)\| =\sqrt{a^2+b^2}$
    - 특징: overflow/underflow를 피하는 안정적인 계산

```rust
#[inline]
fn on_hypot2(a: f64, b: f64) -> f64 {
    a.hypot(b)
}
```

## on_jacobi_symmetric_eigen
### 📌 수학적 목적
- 대칭행렬
```math
B\in \mathbb{R^{\mathnormal{n\times n}}},\quad B=B^T
```
- 에 대해 다음을 만족하는 고유분해를 구한다:
```math
B=V\Lambda V^T
```
- $\Lambda$ : 대각행렬 (고유값)
- $V$: 직교행렬 (열벡터가 고유벡터)
- $V^TV$=I
- Jacobi 방법은 반복적으로 2×2 회전(Jacobi rotation)을 적용하여   
    오프대각 성분을 0으로 만드는 방식이다.

### 📌 핵심 아이디어: Jacobi 회전
- 행렬 B의 오프대각 원소 $b_{pq}$ 를 없애기 위해  
    다음과 같은 **Givens 회전(Jacobi rotation)** 을 구성한다:
```math
J(p,q,c,s)=\left[ \begin{matrix}1&&&&\\ &\ddots &&&\\ &&c&s&\\ &&-s&c&\\ &&&&\ddots \end{matrix}\right]
``` 
- 여기서 $c=\cos \theta$ , $s=\sin \theta$.
- 이 회전으로:
```math
B'=J^TBJ
```
- 을 만들면, 새로운 행렬 B'에서 $b'_{pq}=0$ 이 된다.

### 📌 회전각 계산 (Numerical Recipes 방식)
- 오프대각 원소 $b_{pq}$ 를 없애기 위한 회전각은 다음으로 계산된다:
```math
\tau =\frac{b_{qq}-b_{pp}}{2b_{pq}}
```

```math
t =
\begin{cases}
\displaystyle \frac{1}{2\tau},
& \text{if } |\tau| \text{ 가 매우 큰 경우}, \\[6pt]
\displaystyle \frac{\mathrm{sgn}(\tau)}{|\tau| + \sqrt{1+\tau^{2}}},
& \text{일반적 경우}.
\end{cases}
```


```math 
c=\frac{1}{\sqrt{1+t^2}},\qquad s=tc
```
- 이 c,s가 Jacobi 회전의 파라미터이다.

### 📌 행렬 업데이트
- 회전 후의 새로운 대각 원소:
```math
b'_{pp}=b_{pp}-tb_{pq}
```
```math
b'_{qq}=b_{qq}+tb_{pq}
```
- 오프대각 원소는 0으로 설정:
```math
b'_{pq}=b'_{qp}=0
```
- 다른 행/열 r에 대해서는:
```math
b'_{rp}=c\, b_{rp}-s\, b_{rq}
```
```math
b'_{rq}=s\, b_{rp}+c\, b_{rq}
```

### 📌 고유벡터 업데이트
- 고유벡터 행렬 V는 다음과 같이 갱신된다:
```math
V'=VJ
```
- 즉, 각 열벡터에 동일한 회전을 적용한다:
```math
v'_{rp}=c\, v_{rp}-s\, v_{rq}
```
```math
v'_{rq}=s\, v_{rp}+c\, v_{rq}
```

### 📌 수렴 조건
- Jacobi 방법은 다음 조건 중 하나를 만족하면 종료한다:
- 한 sweep 동안 변화가 없음
- 오프대각 제곱합이 충분히 작아짐
```math
\sum _{p\neq q}b_{pq}^2<\mathrm{tol}
```
- sweep 횟수가 제한(max_sweeps)을 초과

### 📌 최종 결과
- 갱신된 B는 대각행렬이 된다 → 고유값
- V는 정규직교 고유벡터 행렬
- vals[i] = B[i,i] 로 고유값을 추출

### 📌 요약
- 이 함수는 다음을 수행한다:

- 입력: 대칭행렬 B
- 출력:
    - B → 대각행렬 $\Lambda$ 
    - V → 고유벡터
    - vals → 고유값 벡터
- Jacobi 방법은:
    - 대칭행렬에 대해 항상 수렴
    - 고유벡터가 매우 정확
    - 속도는 느리지만 안정적
- 그래서 CAD/Geometry 엔진에서 작은 행렬의 고유분해에 자주 사용된다.


```rust
/// 대칭행렬 B (n×n)를 야코비 회전으로 고유분해.
/// 결과: B는 대각(고유값), v는 열-고유벡터(정규직교).

fn on_jacobi_symmetric_eigen(b: &mut Matrix, vals: &mut Vec<f64>, v: &mut Matrix) -> bool {
    let n = b.row_count();
    if n == 0 || b.col_count() != n {
        return false;
    }

    // v <- I
    if !v.create(n, n) {
        return false;
    }
    for i in 0..n {
        for j in 0..n {
            *v.at_mut(i as i32, j as i32) = if i == j { 1.0 } else { 0.0 };
        }
    }

    // 반복 파라미터
    let max_sweeps = 50 * n * n;
    let tol = 1e-14_f64;

    // 도움: 합 오프대각의 제곱합
    let off2 = |m: &Matrix| -> f64 {
        let mut s = 0.0;
        for p in 0..n {
            for q in 0..n {
                if p != q {
                    let x = *m.at(p as i32, q as i32);
                    s += x * x;
                }
            }
        }
        s
    };

    // 반복
    let mut sweep = 0usize;
    loop {
        let mut changed = false;

        for p in 0..n {
            for q in (p + 1)..n {
                let app = *b.at(p as i32, p as i32);
                let aqq = *b.at(q as i32, q as i32);
                let apq = *b.at(p as i32, q as i32);
                if apq.abs() <= tol * on_hypot2(app.abs(), aqq.abs()) {
                    continue;
                }

                // 회전계수 (NR 방식)
                let tau = (aqq - app) / (2.0 * apq);
                let t = if tau.abs() + 1.0 == 1.0 {
                    1.0 / (2.0 * tau)
                } else {
                    let sgn = if tau >= 0.0 { 1.0 } else { -1.0 };
                    sgn / (tau.abs() + (1.0 + tau * tau).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;

                // B <- Jᵀ B J  (대칭 유지)
                // 행/열 p,q 업데이트
                let bpp = app - t * apq;
                let bqq = aqq + t * apq;
                *b.at_mut(p as i32, p as i32) = bpp;
                *b.at_mut(q as i32, q as i32) = bqq;
                *b.at_mut(p as i32, q as i32) = 0.0;
                *b.at_mut(q as i32, p as i32) = 0.0;

                for r in 0..n {
                    if r != p && r != q {
                        let arp = *b.at(r as i32, p as i32);
                        let arq = *b.at(r as i32, q as i32);
                        let nrp = c * arp - s * arq;
                        let nrq = s * arp + c * arq;
                        *b.at_mut(r as i32, p as i32) = nrp;
                        *b.at_mut(p as i32, r as i32) = nrp;
                        *b.at_mut(r as i32, q as i32) = nrq;
                        *b.at_mut(q as i32, r as i32) = nrq;
                    }
                }

                // V <- V J (열-고유벡터)
                for r in 0..n {
                    let vrp = *v.at(r as i32, p as i32);
                    let vrq = *v.at(r as i32, q as i32);
                    *v.at_mut(r as i32, p as i32) = c * vrp - s * vrq;
                    *v.at_mut(r as i32, q as i32) = s * vrp + c * vrq;
                }

                changed = true;
            }
        }

        sweep += 1;
        if !changed {
            break;
        }
        if sweep > max_sweeps {
            break;
        } // 안전 탈출
        if off2(b) < tol {
            break;
        }
    }

    // 고유값 추출
    vals.clear();
    vals.resize(n, 0.0);
    for i in 0..n {
        vals[i] = *b.at(i as i32, i as i32);
    }
    true
}
```

## on_point_on_circle
- 원 위의 점을 극좌표(polar coordinates)로 계산
### 📌 수학적 의미
- 이 함수는 중심점 $c=(c_x,c_y,c_z)$, 반지름 $r$, 각도 $\theta$ (라디안 단위)  
가 주어졌을 때 해당 원 위의 3차원 점을 계산한다.
- 원은 xy-평면에 놓여 있고, z 좌표는 중심과 동일하게 유지된다.

### 📌 수식
- 2D 원의 극좌표식:
```math
x=c_x+r\cos \theta
```
```math 
y=c_y+r\sin \theta 
```
- 3D에서는 원이 xy-평면에 있으므로:
```math
z=c_z
```

### 📌 기하학적 의미
- 중심 c를 기준으로
- 반지름 r만큼 떨어진
- 각도 $\theta$  방향의 점을 구한다.
- 즉, 원 위의 점을 매개변수화한 parametric circle equation이다.

### 📌 요약
- 이 함수는 다음을 계산한다:
```math
P(\theta )=c+r(\cos \theta ,\, \sin \theta ,\, 0)
```
- CAD/Geometry 엔진에서:
    - 원호 생성
    - 원 기반의 보간
    - 회전 운동
    - 2D/3D 곡선 생성
- 등에서 기본적으로 사용되는 핵심 함수다.

```rust
#[inline]
pub fn on_point_on_circle(c: Point3D, r: f64, ang: f64) -> Point3D {
    Point3D {
        x: c.x + r * ang.cos(),
        y: c.y + r * ang.sin(),
        z: c.z,
    }
}
```

## on_circle_to_polygon
- 이 함수는 CAD·Geometry 엔진에서 매우 자주 쓰이는 원(circle)의  
    다각형 근사(polygon approximation) 알고리즘.

### 📘 on_circle_to_polygon(center, radius, segments)
- 원을 등분한 다각형으로 근사하는 함수
### 📌 수학적 의미
- 이 함수는 중심점 $C=(c_x,c_y)$, 반지름 r, 분할 개수 N이 주어졌을 때  
    원을 N개의 동일한 각도로 나누어 얻은 **정다각형(regular polygon)** 의  
    꼭짓점들을 생성한다.
- 즉, 원의 매개변수식:
```math
P(\theta )=\left[ \begin{matrix}c_x+r\cos \theta \\ c_y+r\sin \theta \end{matrix}\right]
``` 
- 을 이용하여,
```math
\theta _i=\frac{2\pi i}{N},\quad i=0,1,\dots ,N
```
- 에 해당하는 점들을 생성한다.

### 📌 수식
- 각 꼭짓점은 다음과 같이 계산된다:
```math
\theta _i=\frac{2\pi i}{N}
```
```math
x_i=c_x+r\cos \theta _i
```
```math
y_i=c_y+r\sin \theta _i
```
- 따라서 다각형의 점 집합은:

- 마지막 점 $P_N$ 은 $P_0$ 과 동일한 위치이므로 다각형을 닫기 위해 포함된다.

### 📌 기하학적 의미
- 원을 균등한 각도 간격으로 샘플링하여 정다각형 형태의 근사치를 만든다.
- CAD, 충돌 감지, 렌더링, 메쉬 생성 등에서 매우 흔히 사용된다.
- 분할 수 N이 커질수록 다각형은 원에 가까워진다.

### 📌 요약
- 이 함수는 다음을 계산한다:
- 즉, 원을 N등분한 정다각형을 생성하는 함수이다.
```rust
pub fn on_circle_to_polygon(center: Point2D, radius: f64, segments: usize) -> Polygon2D {
    let mut pts = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let theta = (i as f64) * std::f64::consts::TAU / (segments as f64);
        let x = center.x + radius * theta.cos();
        let y = center.y + radius * theta.sin();
        pts.push(Point2D::new(x, y));
    }
    Polygon2D::from_points(pts)
}
```

## on_ellipse_to_polygon
- 이 함수는 원을 다각형으로 근사하는 함수의 타원 버전.

### 📘 on_ellipse_to_polygon(center, rx, ry, segments)
- 타원을 등분한 다각형으로 근사하는 함수
### 📌 수학적 의미
- 이 함수는 중심점
```math
C=(c_x,c_y)
```
- 가 주어지고,
- 타원의 반지름(장축/단축 길이)
```math
r_x,\quad r_y
```
- 그리고 분할 개수 N이 주어졌을 때,
- 타원을 N개의 동일한 각도로 나누어 얻은 **다각형 근사(Polygon Approximation)** 를 생성한다.
- 타원의 매개변수식(parametric equation)은 다음과 같다:
```math
P(\theta )=\left[ \begin{matrix}c_x+r_x\cos \theta \\ c_y+r_y\sin \theta \end{matrix}\right] 
```

### 📌 수식
- 각 분할점의 각도는:
```math
\theta _i=\frac{2\pi i}{N},\quad i=0,1,\dots ,N
```
- 각 점의 좌표는:
```math
x_i=c_x+r_x\cos \theta _i\\
y_i=c_y+r_y\sin \theta _i
```
- 따라서 다각형의 점 집합은:

- 마지막 점 $P_N$ 은 $P_0$ 과 동일한 위치이므로 다각형을 닫기 위해 포함된다.

### 📌 기하학적 의미
- 타원을 균등한 매개변수 각도로 샘플링하여 다각형 형태로 근사한다.
- CAD, 메쉬 생성, 충돌 감지, 렌더링 등에서 매우 흔히 사용되는 기법이다.
- 분할 수 N이 커질수록 타원에 더 가까운 근사치를 얻는다.

### 📌 요약
- 즉, 타원을 N등분한 다각형을 생성하는 함수이다.

```rust
pub fn on_ellipse_to_polygon(center: Point2D, rx: f64, ry: f64, segments: usize) -> Polygon2D {
    let mut pts = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let theta = (i as f64) * std::f64::consts::TAU / (segments as f64);
        let x = center.x + rx * theta.cos();
        let y = center.y + ry * theta.sin();
        pts.push(Point2D::new(x, y));
    }
    Polygon2D::from_points(pts)
}
```

## on_is_clockwise

- 이 함수는 다각형의 시계/반시계(clockwise / counter‑clockwise) 방향을  
    판별하는 고전적인 알고리즘.

### 📘 on_is_clockwise(points)
- 다각형의 방향(시계/반시계)을 판별하는 함수
### 📌 수학적 의미
- 이 함수는 2D 다각형의 꼭짓점 집합
```math
P_0,P_1,\dots ,P_{n-1}
```
- 이 주어졌을 때, 다각형이 시계(clockwise) 방향으로 나열되어 있는지 판별한다.
- 사용된 공식은 **Shoelace Formula(신발끈 공식)** 의 변형으로,  
다각형의 **부호 있는 면적(signed area)** 의 부호를 이용한다.

### 📌 사용된 수식
- 코드에서 계산하는 값은 다음 합이다:
```math
S=\sum _{i=0}^{n-1}(x_{i+1}-x_i)(y_{i+1}+y_i)
```
- 여기서 인덱스는 순환:
```math
P_n=P_0
```
- 이 식은 사실 다음과 같은 signed area의 부호와 동일한 정보를 가진다:
```math
A=\frac{1}{2}\sum _{i=0}^{n-1}(x_iy_{i+1}-x_{i+1}y_i)
```
    - $A<0$ → 시계(clockwise)
    - $A>0$ → 반시계(counter‑clockwise)
- 코드에서 사용한 식은 이를 다음 형태로 변형한 것이다:
```math
S=-2A
```
- 따라서:
```math
S>0\quad \Longleftrightarrow \quad A<0\quad \Longleftrightarrow \quad \mathrm{시계\  방향}
```
### 📌 알고리즘적 의미
- 다각형의 꼭짓점을 순서대로 따라가며 각 변의 기여도를 누적한다.
- 누적된 값의 부호를 통해 다각형이 시계 방향인지 판별한다.
- 매우 빠르고, 삼각형·볼록·오목 다각형 모두에서 동작한다.

### 📌 최종 정리
- 이 함수는 다음을 판별한다:
- 즉, signed area의 부호를 이용해 다각형의 방향을 판별하는 함수이다.
```rust
pub fn on_is_clockwise(points: &[Point2D]) -> bool {
    let mut sum = 0.0;
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        sum += (p2.x - p1.x) * (p2.y + p1.y);
    }
    sum > 0.0
}
```
## on_ensure_winding_order
- 이 함수는 짧지만 다각형의 방향을 강제로 원하는 방향(clockwise 또는 CCW)으로 맞추는 역할을 한다.

### 📘 on_ensure_winding_order(points, clockwise)
- 다각형의 꼭짓점 나열 방향을 시계/반시계로 강제 조정
### 📌 수학적 의미
- 다각형의 꼭짓점 집합
```math
P_0,P_1,\dots ,P_{n-1}
```
- 이 주어졌을 때, 이 함수는 다각형의 **현재 방향(winding order)** 을 검사하고,  
    사용자가 원하는 방향(clockwise = true 또는 false)과 다르면  
    점 순서를 뒤집어(reverse) 방향을 맞춘다.
- 다각형의 방향은 **부호 있는 면적(signed area)** 의 부호로 판별된다.
- Signed area:
```math
A=\frac{1}{2}\sum _{i=0}^{n-1}(x_iy_{i+1}-x_{i+1}y_i)
```
- $A<0$ → 시계(clockwise)
- $A>0$ → 반시계(counter‑clockwise)
- on_is_clockwise(points)는 이 부호를 이용해 방향을 판별한다.

### 📌 이 함수가 하는 일
- 현재 다각형이 시계인지 판별
```math
\mathrm{cw\_ now}=\mathrm{on\_ is\_ clockwise(points)}
```
- 원하는 방향(clockwise)과 다르면
```math
\mathrm{cw\_ now}\neq \mathrm{clockwise}
```
- 점 순서를 뒤집는다:
```math
(P_0,P_1,\dots ,P_{n-1})\rightarrow (P_{n-1},\dots ,P_1,P_0)
```
- 이렇게 하면 다각형의 방향이 정확히 원하는 방향으로 맞춰진다.
### 📌 기하학적 의미
- 다각형의 방향은 법선 방향, 면적 부호, 메쉬 winding rule, inside/outside  
    테스트 등에서 매우 중요하다.
- CAD, 메쉬 처리, 폴리곤 클리핑, 삼각분할 등에서 필수적인 전처리 과정이다.
- 이 함수는 다각형의 방향을 일관성 있게 유지하기 위한 유틸리티 함수다.
### 📌 요약즉, 
- 다각형의 방향을 원하는 시계/반시계 방향으로 강제 조정하는 함수이다.

```rust
pub fn on_ensure_winding_order(points: &mut Vec<Point2D>, clockwise: bool) {
    if on_is_clockwise(points) != clockwise {
        points.reverse();
    }
}
```

## on_distance
- 3차원 공간에서 두 점 사이의 유클리드 거리
### 📌 수학적 의미
- 3차원 공간의 두 점
```math
A=(a_x,a_y,a_z),\quad B=(b_x,b_y,b_z)
```
- 이 주어졌을 때, 두 점 사이의 거리 d(A,B)는 **유클리드 거리(Euclidean distance)** 로 정의된다.
- 이는 피타고라스 정리를 3차원으로 확장한 형태다.

### 📌 수식
- 두 점 사이의 거리:
- Rust 코드에서 수행하는 연산과 정확히 일치한다.

### 📌 기하학적 의미
- 3D 공간에서 두 점 사이의 직선 거리를 의미한다.
- CAD, 3D 모델링, 충돌 감지, 곡선/곡면 계산 등 거의 모든 기하 알고리즘의 기본 요소.
- 벡터 길이(norm) 계산과 동일한 개념이다.

### 📌 요약
- 이 함수는 다음을 계산한다:
```math
d=\| B-A\| _2
```
- 즉, 두 3D 점 사이의 유클리드 거리를 반환하는 함수다.
```rust
#[inline]
pub fn on_distance(a: &Point3D, b: &Point3D) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt()
}
```
## on_eq_pt

- 이 함수는 아주 단순해 보이지만 기하학적으로는 **두 점의 동일성(equality)** 을  
    정의하는 중요한 기본 연산.

## 📘 on_eq_pt(a, b)
- 두 3D 점의 좌표가 완전히 동일한지 검사하는 함수
### 📌 수학적 의미
- 3차원 공간의 두 점
```math
A=(a_x,a_y,a_z),\quad B=(b_x,b_y,b_z)
```
- 이 주어졌을 때, 이 함수는 두 점이 정확히 동일한 좌표를 가지는지 판별한다.
- 즉, 다음 조건을 검사한다:
- Rust 코드에서 수행하는 연산과 정확히 일치한다.

### 📌 수식
- 두 점의 동일성은 다음과 같이 정의된다:
```math
\mathrm{on\_ eq\_ pt}(A,B)=\left\{ \, \begin{array}{ll}\textstyle \mathrm{true},&\textstyle \mathrm{if\  }a_x=b_x,\; a_y=b_y,\; a_z=b_z\\ \textstyle \mathrm{false},&\textstyle \mathrm{otherwise}\end{array}\right.
``` 
- 즉, 모든 좌표가 정확히 일치해야 true가 된다.

### 📌 기하학적 의미
- 두 점이 완전히 같은 위치인지 판별하는 기본 연산
- CAD, 메쉬 처리, 곡선/곡면 계산, 중복 점 제거 등에서 자주 사용
- 부동소수점 오차를 고려하지 않는 정확 비교(exact comparison) 방식
- 필요에 따라 epsilon 기반 비교 함수가 별도로 필요할 수 있다

### 📌 요약
- 이 함수는 다음을 검사한다:
```math
A=B\quad \mathrm{(좌표가\  모두\  동일)}
```
- 즉, 두 3D 점이 동일한지 판별하는 가장 단순한 형태의 equality 함수이다.
```rust
#[inline]
pub fn on_eq_pt(a: &Point3D, b: &Point3D) -> bool {
    (a.x == b.x) && (a.y == b.y) && (a.z == b.z)
}
```

## on_lerp_point
- 두 3D 점 사이의 선형 보간 (Linear Interpolation)
### 📌 수학적 의미
- 두 점
```math
A=(a_x,a_y,a_z),\quad B=(b_x,b_y,b_z)
```
- 이 있을 때, 매개변수 $t\in \mathbb{R}$ 에 대해  
    점 A와 B 사이를 직선으로 보간한 점을 반환한다.
- LERP는 다음과 같이 정의된다:

### 📌 수식 전개
- 좌표별로 풀면:
```math
x(t)=(1-t)a_x+tb_x\\
y(t)=(1-t)a_y+tb_y\\
z(t)=(1-t)a_z+tb_z
```
- 따라서 최종 점은:


### 📌 기하학적 의미
- t=0 → P(0)=A
- t=1 → P(1)=B
- 0<t<1 → A와 B 사이의 점
- t<0 또는 t>1 → 선분을 넘어선 외삽(extrapolation)
- LERP는 다음과 같은 곳에서 매우 중요하다:
    - 곡선/곡면 보간
    - 애니메이션 키프레임 보간
    - 베지어 곡선의 De Casteljau 알고리즘
    - 카메라 이동, 물체 이동
    - 수치적 안정성이 높은 보간 방식

### 📌 요약
- 이 함수는 다음을 계산한다:
```math
P(t)=(1-t)A+tB
```
- 즉, 두 3D 점 사이의 선형 보간을 수행하는 기본 함수이다.

```rust
#[inline]
pub fn on_lerp_point(a: &Point3D, b: &Point3D, t: f64) -> Point3D {
    Point3D {
        x: (1.0 - t) * a.x + t * b.x,
        y: (1.0 - t) * a.y + t * b.y,
        z: (1.0 - t) * a.z + t * b.z,
    }
}
```
## on_unitize

- 이 함수는 **3D 벡터의 정규화(normalization)** 를 수행하는 매우 중요한 기본 연산.

## 📘 on_unitize(v)
- 3D 벡터의 정규화(Normalization)
###📌 수학적 의미
- 3차원 벡터
```math
\mathbf{v}=(v_x,v_y,v_z)
```
- 가 주어졌을 때, 이 함수는 벡터의 길이를 1로 만드는 **단위벡터(unit vector)** 를 계산한다.
- 벡터의 길이(norm)는 다음과 같다:
```math
\| \mathbf{v}\| =\sqrt{v_x^2+v_y^2+v_z^2}
```

### 📌 수식 전개
- 정규화된 벡터의 각 성분은 다음과 같다:
```math
\hat {v}_x=\frac{v_x}{\| \mathbf{v}\| }\\
\hat {v}_y=\frac{v_y}{\| \mathbf{v}\| }\\
\hat {v}_z=\frac{v_z}{\| \mathbf{v}\| }
```

### 📌 특수 처리 (길이가 매우 작은 경우)
- 벡터의 길이가 매우 작아
```math
\| \mathbf{v}\| \leq \mathrm{ON\_ TOL12}
```
- 이면, 수치적으로 불안정해지므로 **영벡터(0,0,0)** 를 반환한다.
- 즉:
```math
\| \mathbf{v}\| \approx 0\quad \Rightarrow \quad \hat {\mathbf{v}}=(0,0,0)
```
- 이는 CAD/Geometry 엔진에서 매우 중요한 안정성 처리다.

### 📌 기하학적 의미
- 정규화는 다음과 같은 상황에서 필수적이다:
    - 방향 벡터(direction vector) 계산
    - 법선 벡터(normal) 계산
    - 회전/변환에서 단위벡터 필요
    - 내적/외적 계산의 안정성 확보
    - 곡선/곡면의 tangent, normal, binormal 계산
- 정규화된 벡터는 크기 정보는 버리고 방향만 유지한다.

### 📌 요약
- 이 함수는 다음을 계산한다:
    - 즉, 3D 벡터를 단위벡터로 변환하는 함수이며,
    - 길이가 너무 작은 경우에는 안전하게 0벡터를 반환한다.

```rust
#[inline]
pub fn on_unitize(v: Vector3D) -> Vector3D {
    let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if len <= ON_TOL12 {
        Vector3D::zero()
    } else {
        Vector3D {
            x: v.x / len,
            y: v.y / len,
            z: v.z / len,
        }
    }
}
```
## on_dot_vec

- 이 함수는 3D 기하학에서 가장 기본적이면서도 중요한 연산 중 하나인  
    **내적(dot product)** 을 계산하는 함수.

### 📘 on_dot_vec(a, b)
- 3차원 벡터의 내적(Dot Product, Scalar Product)
### 📌 수학적 의미
- 두 3차원 벡터
```math
\mathbf{a}=(a_x,a_y,a_z),\quad \mathbf{b}=(b_x,b_y,b_z)
```
- 가 주어졌을 때, 이 함수는 두 벡터의 **내적(dot product)** 을 계산한다.
- 내적은 다음과 같이 정의된다:

- Rust 코드와 정확히 일치한다.

### 📌 기하학적 의미
- 내적은 두 벡터 사이의 각도와 길이와 깊은 관련이 있다.
```math
\mathbf{a}\cdot \mathbf{b}=\| \mathbf{a}\| \, \| \mathbf{b}\| \cos \theta
``` 
- 여기서 $\theta$ 는 두 벡터 사이의 각도.
- 따라서:
- $\mathbf{a}\cdot \mathbf{b}>0$ → 두 벡터가 예각
- $\mathbf{a}\cdot \mathbf{b}=0$ → 두 벡터가 직교(orthogonal)
- $\mathbf{a}\cdot \mathbf{b}<0$ → 두 벡터가 둔각

### 📌 응용 분야
- 내적은 3D 그래픽스, CAD, 물리 엔진 등에서 매우 중요한 역할을 한다.
    - 두 벡터의 각도 계산
    - 정규화(normalization) 과정에서 길이 계산
    - 투영(projection) 연산
    - 조명 계산 (Lambertian shading)
    - 법선 벡터와 방향 벡터의 관계 분석
    - 평면 방정식 계산
- 기하 알고리즘의 거의 모든 곳에서 등장하는 핵심 연산이다.
```rust
#[inline]
pub fn on_dot_vec(a: &Vector3D, b: &Vector3D) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
```
## on_dot_pt

- 이 함수는 이름만 보면 **점(Point)끼리의 dot product** 라서 조금 어색할 수 있지만,  
    수학적으로는 **3D 좌표를 벡터로 간주했을 때의 내적(dot product)** 을 계산하는 함수.

### 📘 on_dot_pt(a, b)
- 3차원 점(Point)을 벡터로 간주한 내적(Dot Product)
### 📌 수학적 의미
- 3차원 점
```math
A=(a_x,a_y,a_z),\quad B=(b_x,b_y,b_z)
```
- 이 주어졌을 때, 이 함수는 두 점을 원점 기준 벡터로 해석하여  
    그 벡터들의 내적(dot product)을 계산한다.
- 즉, 점을 다음과 같은 벡터로 본다:
```math
\vec {A}=\left[ \begin{matrix}a_x\\ a_y\\ a_z\end{matrix}\right] ,\quad \vec {B}=\left[ \begin{matrix}b_x\\ b_y\\ b_z\end{matrix}\right]
``` 
- 이 두 벡터의 내적은:
    - Rust 코드와 정확히 일치한다.

### 📌 기하학적 의미
- 점(Point)을 벡터로 간주하는 것은 기하학에서 매우 흔한 관례다.
    - 원점에서 점까지의 위치벡터(position vector)
    - 두 점의 내적을 통해
        - 각도 계산
        - 투영(projection)
        - 거리 제곱(norm²) 계산
        - 기하 알고리즘의 중간 연산
- 등에 사용된다.
- 내적의 기하학적 의미는 다음과 같다:
```math
\vec {A}\cdot \vec {B}=\| \vec {A}\| \, \| \vec {B}\| \cos \theta
``` 
- 따라서:
    - 값이 양수 → 두 벡터가 예각
    - 값이 0 → 두 벡터가 직교
    - 값이 음수 → 두 벡터가 둔각

### 📌 왜 Point에도 dot을 쓰는가?
- CAD/Geometry 엔진에서는 다음과 같은 상황이 많다:
    - 점을 벡터처럼 계산해야 하는 경우
    - 벡터와 점의 연산이 동일한 수학적 의미를 가질 때
    - 성능을 위해 별도의 변환 없이 바로 dot을 계산할 때
- 그래서 on_dot_vec과 동일한 수식을 사용하지만 입력 타입만 Point3D일 뿐이다.

### 📌 요약
- 이 함수는 다음을 계산한다:
    - 즉, 두 3D 점을 원점 기준 벡터로 간주하여 내적을 계산하는 함수이다.
```rust
#[inline]
pub fn on_dot_pt(a: &Point3D, b: &Point3D) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
```
## on_is_valid_pt
- 이 함수는 기하 엔진에서 수치적으로 유효한 점인지 검사하는 기본 안정성 체크 함수.
- 아주 단순해 보이지만, CAD·Geometry·Simulation 엔진에서는 필수적인 안전장치

### 📘 on_is_valid_pt(p)
- 3D 점의 좌표가 유효한 실수(finite real number)인지 검사
### 📌 수학적 의미
- 3차원 점
```math
P=(x,y,z)
```
- 이 주어졌을 때, 이 함수는 각 좌표가 **유한한 실수(finite real number)** 인지 검사한다.
- 즉, 다음 조건을 모두 만족해야 한다:
```math
x\in \mathbb{R},\quad y\in \mathbb{R},\quad z\in \mathbb{R}
```
- 여기서 “유한한 실수”란:
    - NaN (Not a Number) 아님
    - +∞, -∞ 아님
    - IEEE 754에서 정상적인 실수 값
- 을 의미한다.
- Rust의 is_finite()는 다음을 검사한다:
```math
\mathrm{is\_ finite}(v)=\left\{ \, \begin{array}{ll}\textstyle \mathrm{true},&\textstyle v\neq \pm \infty \mathrm{\  AND\  }v\neq \mathrm{NaN}\\ \textstyle \mathrm{false},&\textstyle \mathrm{otherwise}\end{array}\right.
``` 

### 📌 함수의 논리
- 즉, 세 좌표가 모두 유효한 실수일 때만 true.

### 📌 기하학적 의미
- 이 함수는 다음과 같은 오류를 방지하기 위해 사용된다:
    - 계산 중 overflow → 좌표가 inf가 되는 경우
    - 0으로 나누기 → NaN 발생
    - 잘못된 변환/행렬 연산
    - 비정상적인 기하 연산 결과
- 유효하지 않은 점이 기하 알고리즘에 들어가면:
    - 거리 계산 실패
    - 교차 테스트 실패
    - 노멀 계산 오류
    - 메쉬 깨짐
    - 곡선/곡면 평가 실패
- 등의 문제가 발생할 수 있기 때문에
- 모든 기하 엔진에서 반드시 필요한 안정성 체크다.

### 📌 요약
- 즉, 3D 점의 좌표가 NaN 또는 무한대가 아닌지 검사하는 함수이다.
```rust
#[inline]
pub fn on_is_valid_pt(p: &Point3D) -> bool {
    p.x.is_finite() && p.y.is_finite() && p.z.is_finite()
}
```
## on_get_divide_number

- 이 함수는 원호(arc)를 일정한 최대 편차(deviation) 이하로 근사하기 위해  
    필요한 분할 개수(divisions)를 계산하는 매우 중요한 기하 알고리즘.
- CAD·CAM·Geometry 엔진에서 자주 쓰이는 “Chord error 기반 분할 수 계산 공식”을 그대로 구현.

### 📘 on_get_divide_number(radius, delta_radians, deviation)
- 원호를 chord error(현 오차) 기준으로 분할 개수를 계산하는 함수
### 📌 문제 정의
- 반지름 r인 원호가 있고, 그 원호의 각도 길이가
```math
\Delta \theta =\mathrm{delta\_ radians}
```
- 일 때, 원호를 직선 조각(chord)으로 근사하려면 각 chord가 허용 오차 deviation 이하가 되도록  
    분할해야 한다.
- 여기서 deviation은 **원호와 chord 사이의 최대 거리(= sagitta, 현고)** 이다.

### 📌 핵심 기하 공식: Sagitta(현고)
- 반지름 r인 원에서 chord의 중심에서 원호까지의 최대 오차(현고)는:
```math
\mathrm{sagitta}=r-\sqrt{r^2-\left( \frac{c}{2}\right) ^2}
```
- 여기서 c는 chord 길이.
- 이를 각도 $\phi$  (chord가 대응하는 중심각)로 표현하면:
```math
c=2r\sin \left( \frac{\phi }{2}\right) 
```
- 따라서 sagitta는 다음과 같이 단순화된다:
```math
\mathrm{sagitta}=r\left( 1-\cos \left( \frac{\phi }{2}\right) \right) 
```
- 이 sagitta가 deviation 이하가 되려면:
```math
r\left( 1-\cos \left( \frac{\phi }{2}\right) \right) \leq \mathrm{deviation}
```
- 이를 $\phi$ 에 대해 풀면:
```math
\cos \left( \frac{\phi }{2}\right) \geq \frac{r-\mathrm{deviation}}{r}
```
```math
\frac{\phi }{2}\leq \arccos \left( \frac{r-\mathrm{deviation}}{r}\right) 
```
- 따라서 chord 하나가 허용하는 최대 중심각은:

### 📌 분할 개수 계산
- 전체 원호 각도 $\Delta$ $\theta$ 를
- 각 chord가 허용하는 최대 각도 $\phi _{\max }$ 로 나누면:
```math
N=\left\lceil \frac{|\Delta \theta |}{\phi _{\max }}\right\rceil
``` 
- 단, 최소 2분할은 필요하므로:
```math
N=\max (2,N)
```
- 각 분할의 실제 각도는:
```math
\theta _{\mathrm{step}}=\frac{\Delta \theta }{N}
```
### 📌 코드와 수식의 대응
- 코드에서:
```rust
t = (radius - deviation) / radius;
denom = 2.0 * t.acos();
``

- 이는 정확히 다음을 의미한다:
```math
\phi _{\max }=2\arccos \left( \frac{r-\mathrm{deviation}}{r}\right) 
```
- 그리고:
```rust
div = ceil(delta_radians.abs() / denom)
```

- 즉:
```math
N=\left\lceil \frac{|\Delta \theta |}{\phi _{\max }}\right\rceil 
```
- 마지막으로:
```rust
angle = delta_radians / div
```

- 즉:
```math
\theta _{\mathrm{step}}=\frac{\Delta \theta }{N}
```
### 📌 최종 요약
- 이 함수는 다음을 계산한다:
- 즉, 원호를 deviation 이하의 chord error로 근사하기 위해 필요한 분할 개수와  
    각 분할의 각도를 반환하는 함수이다.

```rust
pub fn on_get_divide_number(radius: f64, delta_radians: f64, deviation: f64) -> (usize, f64) {
    // 2*acos((r-dev)/r) 각도 당 하나의 chord
    if !(radius > 0.0) || !(deviation > 0.0) || delta_radians.abs() < f64::EPSILON {
        return (2, delta_radians / 2.0);
    }
    let mut t = (radius - deviation) / radius;
    if !t.is_finite() {
        t = 1.0;
    }
    t = t.clamp(-1.0, 1.0);
    let denom = 2.0 * t.acos();
    let div = if denom <= f64::EPSILON {
        2
    } else {
        (delta_radians.abs() / denom).ceil().max(2.0) as usize
    };
    let angle = delta_radians / (div as f64);
    (div, angle)
}
```
## on_number_of_segments

- 이 함수는 원호(arc)를 다각형으로 근사할 때 필요한 분할 개수(segment count)를  
    결정하는 고급 알고리즘.
- 바로 이전에 설명했던 on_get_divide_number를 기반으로, **특수한 각도(π, 2π)** 에  
    대해 더 안정적이고 균일한 분할을 보장하는 로직이 추가된 형태.

### 📘 on_number_of_segments(radius, delta_radians, deviation, angle_limit)
- 원호를 chord error + 각도 제한(angle limit) 기준으로 분할 개수를 계산하는 함수
### 📌 목적
- 이 함수는 다음 두 조건을 모두 만족하도록 원호를 분할한다:
    - Chord error(현 오차) ≤ deviation
    - 각 분할의 중심각 ≤ angle_limit (선택적)
- 즉, 원호를 직선 조각으로 근사할 때
    - 너무 휘어지지 않도록
    - 너무 큰 각도로 분할되지 않도록
- 안정적이고 균일한 분할 수를 계산한다.

### 📌 핵심 개념 요약
#### 1) Chord error 기반 최대 분할 각도
- 이전 함수에서 구한 공식:
```math
\phi _{\max }=2\arccos \left( \frac{r-\mathrm{deviation}}{r}\right)
``` 
- 이 각도보다 큰 분할은 허용되지 않는다.

#### 2) angle_limit 기반 제한
- 사용자가 angle_limit을 지정하면:
```math
\phi _{\mathrm{step}}\leq \mathrm{angle\_ limit}
```
- 이 조건도 만족해야 한다.

#### 3) 특수 각도 처리
- 전체 원(2π)
- 반원(π)
- 이 두 경우는 정확한 대칭 분할이 필요하므로 각각을 π/2(90°) 단위로 나누어 처리한다.

### 📌 알고리즘 상세
- ✔ Case 1: delta_radians ≈ 2π (전체 원)
- 전체 원을 4등분하여 각 구간을 π/2로 나눈 뒤:
```math
\Delta \theta =\frac{\pi }{2}
```
- 각 구간에 대해:
```math
(\mathrm{local\_ div},a)=\mathrm{on\_ get\_ divide\_ number}(r,\pi /2,\mathrm{deviation})
```
- 그리고 angle_limit 조건이 있으면:
```math
\mathrm{if\  }a>\mathrm{angle\_ limit}:\quad \mathrm{local\_ div}=\left\lceil \frac{\pi /2}{\mathrm{angle\_ limit}}\right\rceil
``` 
- 최종 분할 수:
```math
\mathrm{div}=4\cdot \mathrm{local\_ div}
```

- ✔ Case 2: delta_radians ≈ π (반원)
- 반원을 두 개의 π/2 구간으로 나누어 처리:
```math
\mathrm{div}=2\cdot \mathrm{local\_ div}
```
나머지 로직은 Case 1과 동일.

- ✔ Case 3: 일반 각도
- 그냥 on_get_divide_number로 계산:
```math
(d,a)=\mathrm{on\_ get\_ divide\_ number}(r,\Delta \theta ,\mathrm{deviation})
```
- angle_limit 조건이 있으면:
```math
d=\left\lceil \frac{\Delta \theta }{\mathrm{angle\_ limit}}\right\rceil
``` 
- 최종 분할 수:
```math
\mathrm{div}=d
```
- ✔ 최소 분할 수 보장
```math
\mathrm{div}\geq 2
```

### 📌 최종 요약
- 이 함수는 다음 조건을 모두 만족하는 최적의 분할 개수를 계산한다:
- 특수 각도(π, 2π)는 대칭성을 유지하기 위해 π/2 단위로 나누어 처리한다.

### 📌 한 줄 요약
- 원호를 chord error + angle limit 기준으로 가장 안정적이고 균일하게 분할하는  
    segment 수를 계산하는 고급 기하 알고리즘.

```rust
pub fn on_number_of_segments(
    radius: f64,
    mut delta_radians: f64,
    deviation: f64,
    angle_limit: f64,
) -> usize {
    let pi = std::f64::consts::PI;
    let two_pi = 2.0 * pi;
    let mut div;

    if approx_eq!(delta_radians, two_pi, ON_TOL12) {
        delta_radians = pi / 2.0;
        let (mut local_div, a) = on_get_divide_number(radius, delta_radians, deviation);
        if angle_limit > 0.0 && !crate::approx_eq!(a, angle_limit, ON_TOL12) && a > angle_limit {
            local_div = ((pi / 2.0) / angle_limit).ceil() as usize;
        }
        div = local_div * 4;
    } else if approx_eq!(delta_radians, pi, ON_TOL12) {
        delta_radians = pi / 2.0;
        let (mut local_div, a) = on_get_divide_number(radius, delta_radians, deviation);
        if angle_limit > 0.0 && !approx_eq!(a, angle_limit, ON_TOL12) && a > angle_limit {
            local_div = ((pi / 2.0) / angle_limit).ceil() as usize;
        }
        div = local_div * 2;
    } else {
        let (mut d, a) = on_get_divide_number(radius, delta_radians, deviation);
        if angle_limit > 0.0 && !approx_eq!(a, angle_limit, ON_TOL12) && a > angle_limit {
            d = (delta_radians / angle_limit).ceil() as usize;
        }
        div = d;
    }
    if div < 2 {
        div = 2;
    }
    div
}
```
---
