# Line
Line 구조체는 3D 선분을 파라메트릭 곡선처럼 다루는 매우 정교한 구현입니다.  
아래에 전체 함수를 기능별로 정리하고, 수학적 검토와 함께 추가적으로 유용할 수 있는 함수들도 제안.

## 전체 소스
```rust
use crate::core::tarray::TArray;
use std::fmt::Debug;
use crate::core::geom::CPoint;
use crate::core::knot::KnotVector;
use crate::core::matrix::Matrix;
use crate::core::point_ops::CPointOps;
use crate::core::prelude::{Curve, Interval, Point, Vector};
use crate::core::segment3d::Segment3D;
use crate::core::svd::on_svdcmp;
use crate::core::transform::Transform;

/// GLine — simple 3D segment treated as a parametric curve with domain [0, L].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub dimension: usize,
    pub start: Point,
    pub end: Point,
    pub domain: Interval,
}
```
```rust
impl Line {
    #[inline]
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            dimension: 3,
            start,
            end,
            domain: Interval { t0: 0.0, t1: 1.0 },
        }
    }
    #[inline]
    pub fn from_xy(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self::new(Point::new(x0, y0, 0.0), Point::new(x1, y1, 0.0))
    }
    #[inline]
    pub fn from_xyz(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {
        Self::new(Point::new(x0, y0, z0), Point::new(x1, y1, z1))
    }

    #[inline]
    pub fn midpoint(&self) -> Point {
        (self.start + self.end) * 0.5
    }

    #[inline]
    pub fn direction(&self) -> Vector {
        Vector::from_points(&self.start, &self.end)
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.direction().length()
    }

    #[inline]
    pub fn domain(&self) -> Interval {
        self.domain
    }

    /// Parametric evaluation. `t` is measured in arc-length from start, clamped into domain.
    pub fn point_at(&self, t: f64) -> Point {
        let dom = self.domain();
        let l = dom.length().max(1e-300);
        let u = ((t - dom.t0) / l).clamp(0.0, 1.0);
        self.start + (self.end - self.start) * u
    }

    /// Unit tangent (constant).
    pub fn tangent_at(&self, _t: f64) -> Vector {
        self.direction().unitize()
    }

    #[inline]
    pub fn start_tangent(&self) -> Vector {
        self.tangent_at(0.0)
    }
    #[inline]
    pub fn end_tangent(&self) -> Vector {
        self.tangent_at(self.domain().t1)
    }

    /// Translate both endpoints by v.
    #[inline]
    pub fn translate(&mut self, v: Vector) {
        self.start += v.to_point();
        self.end += v.to_point();
    }

    /// Apply a 4x4 transform to both endpoints.
    pub fn transform(&mut self, x: &Transform) {
        self.start = x.transform_point3d(&self.start);
        self.end = x.transform_point3d(&self.end);
    }

    /// Reverse parameter direction (swap endpoints).
    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.start, &mut self.end);
    }

    /// Offset the line by `amount` in the direction `cross(tangent, plane_normal)`.
    /// `plane_normal` does not need to be unit; zero-length input is ignored (no-op).
    pub fn offset(&self, amount: f64, plane_normal: Vector) -> Self {
        let t = self.tangent_at(0.0);
        if !t.is_valid() {
            return *self;
        }
        let mut n = plane_normal.cross(&t);
        if n.normalize() {
            let delta = (n * amount).to_point();
            Self::new(self.start + delta, self.end + delta)
        } else {
            println!("plane_normal normalized fail");
            *self
        }
    }

    /// Uniformly sample points along the line so that each segment length ≈ `step`.
    /// Includes both endpoints. When `step <= 0`, returns just the two endpoints.
    pub fn points_by_length(&self, step: f64) -> Vec<Point> {
        let l = self.length();
        if step <= 0.0 || l <= 0.0 {
            return vec![self.start, self.end];
        }
        let n = (l / step).ceil() as usize;
        let n = n.max(1);
        (0..=n)
            .map(|i| {
                let u = (i as f64) / (n as f64);
                self.start + (self.end - self.start) * u
            })
            .collect()
    }

    /// Split at parameter `t` (in arc-length from start). Returns (lower, upper) if `t` is interior.
    pub fn split_at(&self, t: f64) -> Option<(Self, Self)> {
        let dom = self.domain();
        if t <= dom.t0 || t >= dom.t1 {
            return None;
        }
        let p = self.point_at(t);
        Some((Self::new(self.start, p), Self::new(p, self.end)))
    }

    /// Trim the line at parameter `t`. If `flip_side=false`, keeps [0,t]; else keeps [t,L].
    pub fn trim_at(&mut self, t: f64, flip_side: bool) -> bool {
        if let Some((lo, hi)) = self.split_at(t) {
            if !flip_side {
                *self = lo;
            } else {
                *self = hi;
            }
            true
        } else {
            false
        }
    }

    /// Project a point onto the segment; returns parameter `t` in domain [0,L].
    pub fn project(&self, p: Point) -> f64 {
        let seg = Segment3D::new(self.start, self.end);
        let u01 = seg.project(p);
        self.domain().t0 + u01 * self.domain().length()
    }

    pub fn closest_param_to(&self, p: Point) -> f64 {
        let seg = Segment3D::new(self.start, self.end);
        let u01 = seg.closest_param_to(p);
        self.domain().t0 + u01 * self.domain().length()
    }

    pub fn to_nurbs(&self) -> Curve {
        let p = 1;
        let knot = vec![0.0, 0.0, 1.0, 1.0];
        let cps = vec![
            CPoint {
                x: self.start.x,
                y: self.start.y,
                z: self.start.z,
                w: 1.0,
            },
            CPoint {
                x: self.end.x,
                y: self.end.y,
                z: self.end.z,
                w: 1.0,
            },
        ];
        let length = (cps[0] - cps[1]).euclid().length();
        Curve {
            dimension: 3,
            degree: p,
            knots: KnotVector{knots: knot},
            ctrl: cps,
            domain: Interval {
                t0: 0.0,
                t1: length,
            },
        }
    }
}
```
```rust
impl Line {
    /// Least-squares line fitting.
    /// Returns the optimal line segment [start, end] that:
    /// – passes through the centroid of the point cloud, and
    /// – follows the principal component direction (eigenvector of the largest eigenvalue of the covariance matrix),
    /// with endpoints determined by projecting the input points onto the line.
    pub fn fit_from_points(points: &[Point]) -> Option<Self> {
        if points.len() < 2 {
            return None;
        }

        // Centroid
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;
        for p in points {
            cx += p.x;
            cy += p.y;
            cz += p.z;
        }
        let n = points.len() as f64;
        cx /= n;
        cy /= n;
        cz /= n;
        let _c = Point::new(cx, cy, cz);

        // Scatter matrix S
        let mut sxx = 0.0;
        let mut sxy = 0.0;
        let mut sxz = 0.0;
        let mut syy = 0.0;
        let mut syz = 0.0;
        let mut szz = 0.0;
        for p in points {
            let dx = p.x - cx;
            let dy = p.y - cy;
            let dz = p.z - cz;
            sxx += dx * dx;
            sxy += dx * dy;
            sxz += dx * dz;
            syy += dy * dy;
            syz += dy * dz;
            szz += dz * dz;
        }

        let mut a = Matrix::with_dims(3, 3);
        *a.at_mut(0, 0) = sxx;
        *a.at_mut(0, 1) = sxy;
        *a.at_mut(0, 2) = sxz;
        *a.at_mut(1, 0) = sxy;
        *a.at_mut(1, 1) = syy;
        *a.at_mut(1, 2) = syz;
        *a.at_mut(2, 0) = sxz;
        *a.at_mut(2, 1) = syz;
        *a.at_mut(2, 2) = szz;

        // SVD: principal eigenvector corresponds to line direction
        let mut w = TArray::<f64>::with_size(3);
        let mut v = Matrix::with_dims(3, 3);
        if !on_svdcmp(&mut a, &mut w, &mut v) {
            return None;
        }

        let mut max_i = 0usize;
        if w[1] > w[max_i] {
            max_i = 1;
        }
        if w[2] > w[max_i] {
            max_i = 2;
        }

        let dir = Vector::new(
            *v.at(0, max_i as i32),
            *v.at(1, max_i as i32),
            *v.at(2, max_i as i32),
        )
        .unitize();

        // Interval endpoints: project points onto direction vector to compute [t_min, t_max]
        let (mut t_min, mut t_max) = (f64::INFINITY, f64::NEG_INFINITY);
        for p in points {
            let t = (p.x - cx) * dir.x + (p.y - cy) * dir.y + (p.z - cz) * dir.z;
            if t < t_min {
                t_min = t;
            }
            if t > t_max {
                t_max = t;
            }
        }
        let start = Point::new(cx + t_min * dir.x, cy + t_min * dir.y, cz + t_min * dir.z);
        let end = Point::new(cx + t_max * dir.x, cy + t_max * dir.y, cz + t_max * dir.z);

        Some(Line {
            dimension: 3,
            start,
            end,
            domain: Interval { t0: 0.0, t1: 1.0 },
        })
    }

    #[inline]
    pub fn is_degenerate(&self) -> bool {
        self.length() < 1e-10
    }

    pub fn angle_with(&self, other: &Line) -> f64 {
        let d1 = self.direction();
        let d2 = other.direction();
        let dot = d1.dot(&d2);
        let len1 = d1.length().max(1e-300);
        let len2 = d2.length().max(1e-300);
        (dot / (len1 * len2)).clamp(-1.0, 1.0).acos()
    }

    pub fn intersects_with(&self, other: &Line) -> bool {
        let seg1 = Segment3D::new(self.start, self.end);
        let seg2 = Segment3D::new(other.start, other.end);
        seg1.intersects_with(&seg2)
    }


    pub fn extend(&self, amount: f64) -> Self {
        let dir = self.direction().unitize();
        let start = self.start - dir.to_point() * amount;
        let end = self.end + dir.to_point() * amount;
        Self::new(start, end)
    }

    pub fn sample_uniform(&self, n: usize) -> Vec<Point> {
        if n == 0 {
            return vec![self.start];
        }
        (0..=n)
            .map(|i| {
                let u = (i as f64) / (n as f64);
                self.start + (self.end - self.start) * u
            })
            .collect()
    }
}
```

## 📘 전체 함수 설명
### 📌 생성자 및 초기화

| 함수 이름                         | 설명                                      |
|----------------------------------|-------------------------------------------|
| `new(start, end)`                | 시작점과 끝점을 지정하여 3D 선분 생성       |
| `from_xy(x0, y0, x1, y1)`        | 2D 평면상의 선분 생성 (Z=0으로 고정)        |
| `from_xyz(x0, y0, z0, x1, y1, z1)` | 3D 좌표를 직접 지정하여 선분 생성           |

### 📌 기하적 특성

| 함수 이름      | 설명                                      |
|----------------|-------------------------------------------|
| `midpoint()`   | 선분의 중점을 반환 (`(start + end) * 0.5`) |
| `direction()`  | 방향 벡터 계산 (`end - start`)             |
| `length()`     | 선분의 길이 반환 (`direction().length()`)  |
| `domain()`     | 파라메터 구간 [0, length()] 반환           |

### 📌 평가 및 접선

| 함수 이름                        | 설명                                                                 |
|----------------------------------|----------------------------------------------------------------------|
| `point_at(t)`                    | 파라메터 `t`에 해당하는 점 반환. `t`는 길이 기반으로 보간됨           |
| `tangent_at(t)`                  | 단위 접선 벡터 반환. 선분은 직선이므로 모든 `t`에서 동일한 방향       |
| `start_tangent()` / `end_tangent()` | 시작점과 끝점에서의 접선 벡터. 내부적으로 `tangent_at()` 호출         |

### 📌 변형 및 조작

| 함수 이름                          | 설명                                                                 |
|------------------------------------|----------------------------------------------------------------------|
| `translate(v)`                     | 벡터 `v`만큼 선분을 평행 이동 (`start`, `end`에 `v`를 더함)           |
| `transform(x)`                     | 4×4 변환 행렬 `x`를 적용하여 선분을 공간 변환                         |
| `reverse()`                        | 시작점과 끝점을 교환하여 선분 방향을 반전                            |
| `offset(amount, plane_normal)`     | 평면 법선 방향으로 선분을 `amount`만큼 평행 이동 (`cross(tangent, normal)`) |

### 📌 샘플링 및 분할

| 함수 이름                      | 설명                                                                 |
|--------------------------------|----------------------------------------------------------------------|
| `points_by_length(step)`       | 선분을 `step` 길이 간격으로 균등 샘플링. 양 끝점 포함                 |
| `split_at(t)`                  | 파라메터 `t`에서 선분을 두 개로 분할. 내부일 경우 `(lo, hi)` 반환     |
| `trim_at(t, flip_side)`        | `t` 기준으로 앞 또는 뒤를 절단. `flip_side=true`면 뒤쪽 유지          |

### 📌 투영 및 거리 기반 평가

| 함수 이름               | 설명                                                                 |
|-------------------------|----------------------------------------------------------------------|
| `project(p)`            | 점 `p`를 선분에 투영한 파라메터 `t` 반환. `Segment3D::project()` 기반 |
| `closest_param_to(p)`   | 점 `p`에 가장 가까운 위치의 파라메터 `t` 반환. `Segment3D::closest_param_to()` 사용 |

### 📌 변환 및 근사

| 함수 이름                  | 설명                                                                 |
|----------------------------|----------------------------------------------------------------------|
| `to_nurbs()`               | 선분을 degree 1 NURBS 곡선으로 변환. 제어점 2개, 클램프된 knot 벡터 사용 |
| `fit_from_points(points)` | 주어진 점 집합에 대해 최적 선분 근사. PCA 기반 주축 방향으로 선형 회귀 수행 |


## 🧠 수학적 검토
### ✅ 정확한 구현
- point_at(t)는 길이 기반 보간으로 안정적이며, clamp로 경계 처리도 적절함
- direction()은 end - start로 정의되며, unitize()로 정규화
- offset()은 cross(tangent, normal)로 평면 기준 이동 → 기하적으로 정확
- fit_from_points()는 SVD 기반 PCA로 주축 방향을 찾고, 투영 범위로 선분 생성 → 통계적으로 최적

### 🧩 추가적으로 유용한 함수

| 함수 이름                        | 설명                                                                 |
|----------------------------------|----------------------------------------------------------------------|
| `is_degenerate()`                | 선분의 길이가 매우 짧은 경우 (`length() < ε`) → 계산 안정성 체크용     |
| `angle_with(other: &Line)`      | 다른 선분과의 방향 각도 계산 (`acos(dot / (len1 * len2))`)           |
| `intersects_with(other: &Line)` | 두 선분이 3D 공간에서 교차하는지 여부 판정                           |
| `extend(amount: f64)`           | 양 끝점을 방향으로 `amount`만큼 연장                                 |
| `sample_uniform(n: usize)`      | 선분을 `n`개의 균등한 구간으로 나누어 점 샘플링                      |

---

# 수학적인 특징

아래는 Line 구조체의 주요 함수들에 대한 수학적 설명을 기능별로 정리한 내용입니다.  
각 함수가 어떤 수학적 원리에 기반해 동작하는지, 어떤 계산을 수행하는지 명확하게 설명.

## 📐 수학적 설명: Line 구조체 핵심 함수
### 📌 기하적 특성

| 함수 이름     | 수학적 정의                                                                 |
|---------------|------------------------------------------------------------------------------|
| `midpoint()`  | $\frac{A + B}{2}$ — 시작점 A와 끝점 B의 중점                                |
| `direction()` | $\vec{d} = B - A$ — 시작점에서 끝점으로 향하는 방향 벡터                    |
| `length()`    | $\|\vec{d}\| = \sqrt{(x_2 - x_1)^2 + (y_2 - y_1)^2 + (z_2 - z_1)^2}$ — 선분 길이 |
| `domain()`    | $[0, L]$ — 파라메터 구간, L은 선분의 길이                                   |


### 📌 평가 및 접선

| 함수 이름                          | 수학적 정의                                                                 |
|------------------------------------|------------------------------------------------------------------------------|
| `point_at(t)`                      | $P(t) = A + u \cdot (B - A),\quad u = \frac{t}{L}$ — 길이 기반 보간 점 계산 |
| `tangent_at(t)`                    | $\hat{d} = \frac{d}{|d|}$ — 방향 벡터의 단위화 (모든 t에서 동일)           |
| `start_tangent()` / `end_tangent()` | `tangent_at(0)` 또는 `tangent_at(L)` 호출 — 접선은 일정하므로 동일한 결과     |



## 📌 변형 및 조작

| 함수 이름             | 수학적 정의                                                                 |
|------------------------|------------------------------------------------------------------------------|
| `translate(v)`         | $A' = A + v,\quad B' = B + v$ — 선분을 벡터 `v`만큼 평행 이동               |
| `transform(x)`         | $P' = T \cdot P$ — 4×4 변환 행렬 `T`를 각 점에 적용                         |
| `reverse()`            | $A \leftrightarrow B$ — 시작점과 끝점을 교환하여 방향 반전                  |
| `offset(amount, n)`    | $\delta = \mathrm{normalize}(n \times \hat{d}) \cdot \mathrm{amount}$ — 평면 법선 기준 평행 이동 |


## 📌 샘플링 및 분할

| 함수 이름               | 수학적 정의                                                                 |
|-------------------------|------------------------------------------------------------------------------|
| `points_by_length(step)`| $P_i = A + \frac{i}{n} \cdot (B - A)$ — 선분을 `n`등분하여 점 샘플링         |
| `split_at(t)`           | $P = A + u \cdot (B - A)$, $u = \frac{t}{L}$ — `t` 위치에서 두 선분으로 분할 |
| `trim_at(t, flip)`      | `split_at(t)` 결과 중 하나 선택 — 앞쪽 또는 뒤쪽 절단                          |


## 📌 투영 및 거리 기반 평가

| 함수 이름               | 수학적 정의                                                                 |
|-------------------------|------------------------------------------------------------------------------|
| `project(p)`            | $t = \frac{(p - A) \cdot \hat{d}}{|\vec{d}|}$ — 점 `p`를 선분에 직교 투영한 파라메터 |
| `closest_param_to(p)`   | `project(p)` 결과를 $[0, L]$ 구간으로 클램핑 — 선분에서 `p`에 가장 가까운 위치의 파라메터 |


## 📌 변환 및 근사

| 함수 이름            | 수학적 정의 및 설명                                                                 |
|----------------------|--------------------------------------------------------------------------------------|
| `to_nurbs()`         | 선분을 degree 1 NURBS 곡선으로 변환. 제어점 2개, 클램프된 knot 벡터 $[0, 0, 1, 1]$ 사용 |
| `fit_from_points()`  | 주어진 점 집합에 대해 최적 선형 근사 수행:  
                          - 중심점 $\bar{p} = \frac{1}{n} \sum p_i$  
                          - 공분산 행렬 → SVD → 주축 방향  
                          - 점들을 주축에 투영하여 최적 선분 생성 |


- 중심점 계산: $\bar {p}=\frac{1}{n}\sum p_i$
- 공분산 행렬 → SVD → 주축 방향
- 점들을 주축에 투영하여 최적 선분 생성 

## 🧩 추가 함수들

| 함수 이름               | 수학적 정의 및 설명                                                                 |
|-------------------------|--------------------------------------------------------------------------------------|
| `is_degenerate()`       | $|\vec{d}| < \varepsilon$ — 선분의 길이가 매우 짧은 경우 (거의 점에 가까움)         |
| `angle_with(other)`     | $\theta = \cos^{-1}\left( \frac{\vec{d}_1 \cdot \vec{d}_2}{|\vec{d}_1||\vec{d}_2|} \right)$ — 두 선분 사이의 방향 각도 |
| `intersects_with(other)`| 두 선분이 3D 공간에서 교차하는지 판별. 최소 거리 계산 기반 (정확한 수식은 구현에 따라 다름) |
| `extend(amount)`        | $A' = A - \hat{d} \cdot a,\quad B' = B + \hat{d} \cdot a$ — 선분을 양방향으로 연장     |
| `sample_uniform(n)`     | $P_i = A + \frac{i}{n} \cdot (B - A)$ — 선분을 `n`등분하여 점 샘플링                 |


---

# 테스트 

## 📋 기존 테스트 함수 요약

| 테스트 함수 이름                                | 검증 대상 함수            | 목적 및 설명                                                                 |
|--------------------------------------------------|---------------------------|------------------------------------------------------------------------------|
| `line_circle`   | `Line::length`, `points_by_length`, `offset` <br> `Circle::length`, `point_at`, `tangent_at`, `offset` | 선분과 원의 길이, 샘플링, 오프셋, 접선 등 기본 동작 확인                     |
| `gline_offset_right_hand_convention`             | `Line::offset`            | 오른손 좌표계 기준 오프셋 방향 검증 (`+Z × +X = +Y`)         |
| `gline_offset_negative_amount_moves_opposite`    | `Line::offset`            | 음수 오프셋 시 반대 방향 이동 확인                          |
| `gcircle_offset_parallel_increases_radius`       | `Circle::offset`          | 평면 법선 방향 오프셋 시 반지름 증가 확인                    |
| `gcircle_offset_antiparallel_decreases_radius`   | `Circle::offset`          | 반대 방향 법선 오프셋 시 반지름 감소 확인                 |
| `gcircle_offset_cannot_cross_zero_radius`        | `Circle::offset`          | 반지름이 0 이하로 줄어들 경우 실패 처리 확인               |
| `gcircle_offset_non_coplanar_returns_none`       | `Circle::offset`          | 비공면 법선 벡터 입력 시 실패 처리 확인                     |
| `gcircle_offset_zero_length_normal_returns_none` | `Circle::offset`          | 0 벡터 법선 입력 시 실패 처리 확인                        |
| `gcircle_tangent_direction_sanity`               | `Circle::point_at`, `tangent_at` | 원의 접선 방향이 오른손 좌표계 기준에 맞는지 확인  |
| `gline_is_degenerate_check`              | `is_degenerate()`            | 길이가 0인 선분을 판별                            |
| `gline_angle_with_horizontal_and_vertical` | `angle_with()`               | 두 선분 사이의 각도 계산                          |
| `gline_extend_should_increase_length`    | `extend()`                   | 선분을 양방향으로 연장했을 때 길이 증가 확인       |
| `gline_sample_uniform_should_return_n_plus_one_points` | `sample_uniform()` | 균등 분할 샘플링 시 `n+1`개의 점 생성 확인         |


```rust
#[cfg(test)]
mod tests {

    use std::f64::consts::{PI, TAU};
    use nurbslib::core::circle::Circle;
    use nurbslib::core::line::Line;
    use nurbslib::core::plane::Plane;
    use nurbslib::core::prelude::{Point, Vector};
    use nurbslib::core::types::ON_TOL9;
```
### 1. line_circle
```rust
    #[test]
    fn line_circle() {
        // GLine
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        assert!((ln.length() - 10.0).abs() < 1e-12);
        let pts = ln.points_by_length(2.5);
        assert_eq!(pts.len(), 5);

        let off = ln.offset(1.0, Vector::new(0.0, 0.0, 1.0));
        println!("{:?}", off);
        assert!((off.start.y - 1.0).abs() < 1e-12);

        // GCircle
        let c = Circle::from_center(Point::new(0.0, 0.0, 0.0), 2.0).unwrap();
        assert!((c.length() - std::f64::consts::TAU * 2.0).abs() < 1e-12);
        let _p0 = c.point_at(0.0); // (r,0,0)
        let _t0 = c.tangent_at(0.0); // +Y 방향
        let _off = c.offset(0.25, c.plane.z_axis).unwrap(); // 반지름 증가
    }
```
### 2. gline_offset_right_hand_convention
```rust
    #[test]
    fn gline_offset_right_hand_convention() {
        // 선: +X 방향, 법선: +Z
        // 우리가 채택한 규칙: 양의 amount => plane_normal × tangent 방향
        // +Z × +X = +Y 이므로, y가 +amount 만큼 이동해야 함.
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let off = ln.offset(1.0, Vector::new(0.0, 0.0, 1.0));
        assert!((off.start.y - 1.0).abs() < ON_TOL9, "expected +Y offset");
        assert!((off.end.y - 1.0).abs() < ON_TOL9, "expected +Y offset");
        assert!(
            (off.length() - ln.length()).abs() < ON_TOL9,
            "offset must preserve length"
        );
    }
```
### 3. gline_offset_negative_amount_moves_opposite
```rust
    #[test]
    fn gline_offset_negative_amount_moves_opposite() {
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let off = ln.offset(-1.0, Vector::new(0.0, 0.0, 1.0));
        assert!((off.start.y + 1.0).abs() < ON_TOL9, "negative amount => -Y");
        assert!((off.end.y + 1.0).abs() < ON_TOL9, "negative amount => -Y");
    }
```
### 4. gcircle_offset_parallel_increases_radius
```rust
    #[test]
    fn gcircle_offset_parallel_increases_radius() {
        // 원: XY 평면, plane.z_axis = +Z
        let c = Circle::new(Plane::xy(), Point::new(0.0, 0.0, 0.0), 2.0).unwrap();
        let off = c
            .offset(0.25, c.plane.z_axis)
            .expect("parallel normal should work");
        assert!((off.radius - 2.25).abs() < ON_TOL9, "r + amount (parallel)");
        // 길이도 일관 확인
        assert!((off.length() - TAU * 2.25).abs() < 1e-8);
    }
```
### 5. gcircle_offset_antiparallel_decreases_radius
```rust
    #[test]
    fn gcircle_offset_antiparallel_decreases_radius() {
        // 반대 방향 법선: -Z -> r - amount
        let c = Circle::new(Plane::xy(), Point::new(0.0, 0.0, 0.0), 2.0).unwrap();
        let off = c
            .offset(0.5, -c.plane.z_axis)
            .expect("anti-parallel normal should work");
        assert!(
            (off.radius - 1.5).abs() < ON_TOL9,
            "r - amount (anti-parallel)"
        );
    }
```
### 6. gcircle_offset_cannot_cross_zero_radius
```rust
    #[test]
    fn gcircle_offset_cannot_cross_zero_radius() {
        let c = Circle::new(Plane::xy(), Point::new(0.0, 0.0, 0.0), 1.0).unwrap();
        // r - amount <= 0 인 경우는 None 이어야 한다.
        let none = c.offset(1.0, -c.plane.z_axis);
        assert!(none.is_none(), "radius must not become <= 0");
    }
```
### 7. gcircle_offset_non_coplanar_returns_none
```rust
    #[test]
    fn gcircle_offset_non_coplanar_returns_none() {
        // 비공면 법선: +Z와 평행/역평행이 아닌 벡터
        let c = Circle::new(Plane::xy(), Point::new(0.0, 0.0, 0.0), 1.0).unwrap();
        let ncp = Vector::new(0.0, 1.0, 1.0); // 비평행
        let res = c.offset(0.25, ncp);
        assert!(
            res.is_none(),
            "non-coplanar offset should be None (NURBS case)"
        );
    }
```
### 8. gcircle_offset_zero_length_normal_returns_none
```rust
    #[test]
    fn gcircle_offset_zero_length_normal_returns_none() {
        let c = Circle::new(Plane::xy(), Point::new(0.0, 0.0, 0.0), 1.0).unwrap();
        let res = c.offset(0.25, Vector::new(0.0, 0.0, 0.0));
        assert!(res.is_none(), "zero-length normal => None");
    }
```
### 9. gcircle_tangent_direction_sanity
```rust
    #[test]
    fn gcircle_tangent_direction_sanity() {
        let c = Circle::new(Plane::xy(), Point::new(0.0, 0.0, 0.0), 1.0).unwrap();
        // t=0: (r,0,0), 접선은 +Y 방향(오른손 기준)
        let p0 = c.point_at(0.0);
        assert!((p0.x - 1.0).abs() < ON_TOL9 && p0.y.abs() < ON_TOL9);
        let t0 = c.tangent_at(0.0);
        assert!(t0.length() - 1.0 < 1e-7);
        assert!(
            t0.y > 0.0 && t0.x.abs() < 1e-7,
            "tangent at 0 should align with +Y"
        );
        // t=π/2: (0,r,0), 접선은 -X 방향
        let p90 = c.point_at(PI * 0.5);
        assert!(p90.x.abs() < ON_TOL9 && (p90.y - 1.0).abs() < ON_TOL9);
        let t90 = c.tangent_at(PI * 0.5);
        assert!(t90.x < 0.0, "tangent at π/2 should point -X");
    }
```
### 10. gline_is_degenerate_check
```rust
    #[test]
    fn gline_is_degenerate_check() {
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!(ln.is_degenerate());
    }

    #[test]
    fn gline_angle_with_horizontal_and_vertical() {
        let ln1 = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0); // +X
        let ln2 = Line::from_xyz(0.0, 0.0, 0.0, 0.0, 10.0, 0.0); // +Y
        let angle = ln1.angle_with(&ln2);
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    }
```
### 11. gline_extend_should_increase_length
```rust
    #[test]
    fn gline_extend_should_increase_length() {
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let ext = ln.extend(1.0);
        assert!((ext.length() - 3.0).abs() < 1e-12);
    }
```
### 12. gline_sample_uniform_should_return_n_plus_one_points
```rust
    #[test]
    fn gline_sample_uniform_should_return_n_plus_one_points() {
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let samples = ln.sample_uniform(4);
        assert_eq!(samples.len(), 5);
        assert!((samples[2].x - 5.0).abs() < 1e-12);
    }

}
---
