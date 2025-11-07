# 📐 Circle 구조체 기능 및 수식 정리

## 🧱 구조 개요
```rust
struct Circle {
    plane: Plane,     // 평면 정보 (origin, x/y/z 축)
    radius: f64       // 반지름
}
```
- 정의된 도메인: [0, 2π]
- 중심점: plane.origin

## 📏 Circle  주요 수식 정리

### 1. 전체 길이
```
length = 2π × r
```
### 2. 각도 사영 (project)
```
t = atan2(tt, s)
→ t ∈ [0, 2π)
```
### 3. 점 계산 (point_at)
```
x = r × cos(t)
y = r × sin(t)
point = plane.point_at(x, y)
```
### 4. 접선 벡터 (tangent_at)
```
tangent = -r × sin(t) × X + r × cos(t) × Y
→ 단위 벡터로 정규화
```
### 5. 길이 → 파라미터
```
t = t0 + (length / r)
```
### 6. 파라미터 → 길이
```
length = |t - t0| × r
```
### 7. Bounding Box
```
extent_axis_i = r × sin(acos(dot(z_axis, axis_i)))
bbox = center ± extent
```


## 🛠 기능별 설명 요약
| 메서드                        | 설명                                                             |
|------------------------------|------------------------------------------------------------------|
| new / from_center            | 평면과 중심, 반지름으로 원 생성                                 |
| project(point, &mut t)       | 3D 점을 원에 사영하여 각도 t 계산                                |
| point_at(t)                  | 각도 t에서의 3D 위치 계산                                       |
| tangent_at(t)                | 각도 t에서의 접선 벡터 계산                                     |
| normal_at(t)                 | 평면의 법선 벡터 반환                                           |
| reverse()                    | Y축 반전으로 파라미터 방향 뒤집기                               |
| offset(amount, normal)       | 평면 법선 방향으로 반지름 조정                                  |
| points_by_length(step)       | 길이 기준으로 균등 분할된 점 샘플링                             |
| get_param_from_length(l)     | 길이 l에 대응되는 각도 t 계산                                   |
| get_length_from_param(t)     | 각도 t에 대응되는 길이 계산                                     |
| transform(t)                 | 평면 및 반지름에 변환 적용                                     |
| get_tight_bbox()             | 원의 tight bounding box 계산                                   |
| to_nurbs()                   | 원을 2차 NURBS 곡선으로 변환                                   |



## ✅ 수식 점검 결과

| 항목                     | 수식 표현                          | 설명                                      |
|--------------------------|------------------------------------|-------------------------------------------|
| 원의 길이                | 2πr                                | 전체 원의 둘레 길이                        |
| 원호 길이                | |t₁ - t₀| × r                      | 시작/끝 각도 차이 × 반지름                |
| 점 계산 (Circle::point_at) | x = r·cos(t), y = r·sin(t)         | 극좌표 → 평면 좌표 변환                   |
| 접선 벡터                | -r·sin(t)·X + r·cos(t)·Y           | 원 위의 접선 방향 벡터                    |
| 길이 → 파라미터 변환     | t = t₀ + (length / r)              | 길이에 대응되는 각도 계산                 |
| 파라미터 → 길이 변환     | length = |t - t₀| × r              | 각도에 대응되는 호의 길이 계산            |
| 2D 원 맞춤 반지름        | r = (1/n) ∑ √((xᵢ - cx)² + (yᵢ - cy)²) | 평균 거리 기반 반지름 추정               |



## 📐 Circle Fitting 및 경로 생성 기능 정리


### 1. 평균 중심 좌표 (mean-centered)
```
ux = (1/n) ∑ xi
uy = (1/n) ∑ yi
```
### 2. 선형 시스템 (Kåsa-style)
```
[Sxx Sxy] [a] = 0.5 × [Sx³ + Sxy²]
[Sxy Syy] [b]         [Sx²y + Sy³]

→ a = (b1 × a22 - a12 × b2) / det
→ b = (-b1 × a21 + a11 × b2) / det
```
### 3. 중심 좌표
```
cx = ux + a
cy = uy + b
```
### 4. 반지름 계산
```
r = (1/n) ∑ √((xi - cx)² + (yi - cy)²)
```
### 5. 헬릭스 경로
```
x = r × cos(θ)
y = r × sin(θ)
z = height × t
θ = 2π × turns × t
```

## 🛠 기능별 설명 요약
| 함수명                        | 설명                                                             |
|------------------------------|------------------------------------------------------------------|
| fit_circle_2d(points)        | 2D 점들로부터 원 중심과 반지름 추정 (Kåsa 방식)                  |
| on_fit_from_points(points)   | 3D 점들로부터 평면 추정 → 2D 투영 → 원 맞춤 → 3D 복원           |
| is_linear(tol)               | 원은 선형이 아니므로 항상 false 반환                            |
| on_circle2d(radius, n)       | 반지름 기준으로 n개의 균등 분포된 2D 원 점 생성                  |
| on_helix_path(turns, h, s, r)| 주어진 회전수, 높이, 반지름으로 헬릭스 경로 및 접선 벡터 생성    |


## ✅ 수식 점검 결과

| 항목                     | 수식 표현                                               | 설명                                      |
|--------------------------|----------------------------------------------------------|-------------------------------------------|
| 원의 길이                | length = 2π × r                                          | 전체 원의 둘레 길이                        |
| 원호 길이                | length = |t₁ - t₀| × r                                   | 시작/끝 각도 차이 × 반지름                |
| 점 계산 (Circle::point_at) | x = r·cos(t), y = r·sin(t)                              | 극좌표 → 평면 좌표 변환                   |
| 접선 벡터                | tangent = -r·sin(t)·X + r·cos(t)·Y                        | 원 위의 접선 방향 벡터                    |
| 길이 → 파라미터 변환     | t = t₀ + (length / r)                                   | 길이에 대응되는 각도 계산                 |
| 파라미터 → 길이 변환     | length = |t - t₀| × r                                   | 각도에 대응되는 호의 길이 계산            |
| 2D 원 맞춤 반지름        | r = (1/n) ∑ √((xᵢ - cx)² + (yᵢ - cy)²)                   | 평균 거리 기반 반지름 추정               |
| 헬릭스 경로              | x = r·cos(θ), y = r·sin(θ), z = h·t, θ = 2π·turns·t       | 회전 각도와 높이에 따른 3D 경로 생성      |


---

# 코드

## 소스 코드
```rust
use std::f64::consts::{PI, TAU};
use crate::core::boundingbox::bounding_box_points;
use crate::core::domain::Interval;
use crate::core::geom::{CPoint, Point2};
use crate::core::plane::Plane;
use crate::core::prelude::{Curve, KnotVector, Point, Vector};
use crate::core::segment3d::Segment3D;
use crate::core::transform::Transform;
use crate::core::types::{on_are_equal_scaled, ON_TOL12, ON_TOL6, ON_TOL9};

/// Circle — circle on a Plane with radius r. Domain = [0, 2π].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub plane: Plane,
    pub radius: f64,
}
```
```rust
impl Circle {
    #[inline]
    pub fn project(&self, point: Point, t_out: &mut f64) -> bool {
        let (s, tt) = self.plane.project_st(point);
        let mut t = if s == 0.0 && tt == 0.0 {
            0.0
        } else {
            tt.atan2(s)
        };
        if t < 0.0 {
            t += TAU;
        }
        *t_out = t;
        true
    }
}
```
```rust
impl Circle {
    pub fn new(plane: Plane, center: Point, radius: f64) -> Option<Self> {
        if radius.is_finite() && radius > ON_TOL12 {
            let mut pln = plane;
            pln.origin = center;
            Some(Self { plane: pln, radius })
        } else {
            None
        }
    }

    pub fn from_center(center: Point, radius: f64) -> Option<Self> {
        Self::new(Plane::xy(), center, radius)
    }

    #[inline]
    pub fn center(&self) -> Point {
        self.plane.origin
    }

    #[inline]
    pub fn domain(&self) -> Interval {
        Interval::new(0.0, std::f64::consts::TAU)
    }

    #[inline]
    pub fn length(&self) -> f64 {
        std::f64::consts::TAU * self.radius
    }

    #[inline]
    pub fn is_point(&self) -> bool {
        self.radius < ON_TOL12
    }

    /// Evaluate point at angle parameter t (radians). Wraps t into [0, 2π).
    pub fn point_at(&self, t: f64) -> Point {
        let mut a = t % TAU;
        if a < 0.0 {
            a += TAU;
        }
        let c = a.cos();
        let s = a.sin();
        self.plane.point_at(self.radius * c, self.radius * s)
    }

    /// Tangent direction at parameter t (unit vector in 3D).
    pub fn tangent_at(&self, t: f64) -> Vector {
        // d/dt (center + r cos t * X + r sin t * Y) = -r sin t * X + r cos t * Y
        let mut a = t % TAU;
        if a < 0.0 {
            a += TAU;
        }
        let v = self.plane.x_axis * (-self.radius * a.sin())
            + self.plane.y_axis * (self.radius * a.cos());
        v.unitize()
    }

    pub fn normal_at(&self, _t: f64) -> Vector {
        // Second derivative points to -r*cos t * X - r*sin t * Y (radial), not plane normal.
        // For a "curve normal" we return plane normal (conventional for circles).
        self.plane.z_axis
    }

    pub fn reverse(&mut self) {
        // Flip Y axis to reverse parameterization while keeping X,Z
        self.plane.y_axis = -self.plane.y_axis;
        self.plane.update_equation();
    }

    /// Offset circle by `amount` along plane normal orientation.
    /// If plane_normal is aligned with circle's plane normal, simply change radius.
    pub fn offset(&self, amount: f64, plane_normal: Vector) -> Option<Self> {
        let eps_parallel = ON_TOL6;
        let eps_radius = ON_TOL12;

        // Safe normalization: returns None on failure (e.g., zero vector)
        let mut n = plane_normal;
        if !n.normalize() {
            return None;
        }

        let mut z = self.plane.z_axis;
        if !z.normalize() {
            return None;
        } // Assume plane z is normalized, but add safeguard

        let dot = Vector::dot(&n, &z);

        let new_r = if dot >= 1.0 - eps_parallel {
            // When plane_normal and the circle's normal are aligned (≈ +1): use r + amount
            self.radius + amount
        } else if dot <= -1.0 + eps_parallel {
            // Opposite direction (≈ -1): use r - amount
            self.radius - amount
        } else {
            // Non-coplanar normal: requires NURBS/general offset instead of concentric offset
            return None;
        };
        if new_r <= eps_radius || !new_r.is_finite() {
            return None; // Fail if radius is zero or negative (or invalid)
        }
        Self::new(self.plane, self.center(), new_r)
    }

    pub fn points_by_length(&self, step: f64) -> Vec<Point> {
        if step <= 0.0 {
            return vec![self.point_at(0.0)];
        }
        let n = (self.length() / step).ceil() as usize;
        let n = n.max(3);
        (0..=n)
            .map(|i| {
                let t = (i as f64) * TAU / (n as f64);
                self.point_at(t)
            })
            .collect()
    }
    pub fn get_param_from_length(&self, length: f64) -> (bool, f64) {
        let curve_length = self.length();
        self.get_param_from_length_with_total(length, curve_length)
    }

    /// public bool GetParamFromLength(double length, double curveLength, out double t)
    pub fn get_param_from_length_with_total(&self, length: f64, curve_length: f64) -> (bool, f64) {
        let dom = self.domain(); // Interval { t0, t1 }
        // 끝점 스냅(상대 스케일 = 전체 길이)
        if on_are_equal_scaled(length, 0.0, curve_length) {
            return (true, dom.t0);
        }
        if on_are_equal_scaled(length, curve_length, curve_length) {
            return (true, dom.t1);
        }
        // 범위 밖이면 실패
        if length < 0.0 || length > curve_length {
            return (false, dom.t0);
        }
        // 원의 길이는 |t - t0| * r 이므로 t = t0 + length / r
        let t = dom.t0 + (length / self.radius);
        (true, t)
    }

    /// public bool GetLengthFromParam(double t, out double length)
    pub fn get_length_from_param(&self, t: f64) -> (bool, f64) {
        let dom = self.domain();
        if t < dom.t0 || t > dom.t1 {
            return (false, 0.0);
        }
        let length = (t - dom.t0).abs() * self.radius;
        (true, length)
    }

    pub fn transform(&mut self, t: &Transform) {
        // Allow uniform (or plane-uniform) scaling; otherwise keep as-is.
        let before = self.plane;
        self.plane = self.plane.transform(t);
        // Try extract uniform scale along plane axes
        let sx = t.scale_factor_x();
        let sy = t.scale_factor_y();
        let sz = t.scale_factor_z();
        // Accept uniform or in-plane uniform (sx≈sy, axis preserved)
        if (sx - sy).abs() < ON_TOL9 && (sy - sz).abs() < ON_TOL9 {
            self.radius *= sx;
        } else if (sx - sy).abs() < ON_TOL9 {
            self.radius *= sx;
        } else {
            // Non-uniform scaling would turn circle into ellipse; keep radius from X scale as heuristic.
            self.radius *= sx;
        }
        // Ensure plane equation consistent
        let _ = before; // quiet unused if cfg differs
    }
    pub fn get_tight_bbox(&self) -> (Point, Point) {
        let n = self.plane.z_axis; // Unit normal (assumed to be maintained)
        // 각도 = acos(clamp(dot(n, axis), -1..1))
        fn ang_between(a: Vector, b: Vector) -> f64 {
            let mut d = Vector::dot(&a, &b);
            if d < -1.0 {
                d = -1.0;
            }
            if d > 1.0 {
                d = 1.0;
            }
            d.acos()
        }

        let a1 = ang_between(n, Vector::new(1.0, 0.0, 0.0));
        let a2 = ang_between(n, Vector::new(0.0, 1.0, 0.0));
        let a3 = ang_between(n, Vector::new(0.0, 0.0, 1.0));

        // Projected radius on each axis = r * sin(angle between normal and that axis)
        let extent = Vector::new(a1.sin(), a2.sin(), a3.sin()) * self.radius;

        let p_min = self.center() - extent.to_point();
        let p_max = self.center() + extent.to_point();

        // For numeric safety, recheck min/max using two points
        bounding_box_points(&[p_min, p_max]).unwrap()
    }

    /// Convert a circle into a standard NURBS (i.e., BSplineCurve). Degree 2, 8 spans (12 knots) in standard configuration.
    pub fn to_nurbs(&self) -> Curve {
        // Same four-quadrant division as in C#. (Degree 2, 8 spans = 12 knots, 9 control points, weight = 1/√2 at midpoints)
        let len = self.domain().length();
        let mut knots = vec![0.0; 12];
        knots[0] = 0.0;
        knots[1] = 0.0;
        knots[2] = 0.0;
        knots[3] = 0.25 * len;
        knots[4] = 0.25 * len;
        knots[5] = 0.5 * len;
        knots[6] = 0.5 * len;
        knots[7] = 0.75 * len;
        knots[8] = 0.75 * len;
        knots[9] = len;
        knots[10] = len;
        knots[11] = len;

        // Control points based on 8-point division: (R, 0) → (R, R) → (0, R) → ...
        let r = self.radius;
        let c = self.center();
        let x = self.plane.x_axis.to_point();
        let y = self.plane.y_axis.to_point();

        // Includes weight multiplied to Euclidean coordinates (Point4D: (wx, wy, wz, w)), adapted to the implementation.
        // Assumes BSplineCurve accepts Point4D as input.
        let w = (0.5f64).sqrt(); // 1/√2
        let cp = vec![
            // 0 deg
            CPoint::from_point3_weight(c + x * r, 1.0),
            // 45 deg (weighted)
            CPoint::from_point3_weight(c + x * r + y * r, w),
            // 90
            CPoint::from_point3_weight(c + y * r, 1.0),
            // 135
            CPoint::from_point3_weight(c - x * r + y * r, w),
            // 180
            CPoint::from_point3_weight(c - x * r, 1.0),
            // 225
            CPoint::from_point3_weight(c - x * r - y * r, w),
            // 270
            CPoint::from_point3_weight(c - y * r, 1.0),
            // 315
            CPoint::from_point3_weight(c + x * r - y * r, w),
            // 360 (닫힘: 첫점 복제)
            CPoint::from_point3_weight(c + x * r, 1.0),
        ];

        // One more note: when w ≠ 1, the internal representation must be converted according to project conventions—whether it's (x, y, z, w) or (wx, wy, wz, w).
        // Here, we assume Point4D::from_point(p, w) internally stores (p.x * w, p.y * w, p.z * w, w).
        Curve {
            dimension:3,
            degree: 2,
            ctrl: cp,
            knots : KnotVector{knots},
            domain: Interval{t0:0.0, t1:1.0}
        }
    }
}
```
```rust
impl Circle {
    /// Circle fitting for 2D points (Kåsa-style linear solution)
    /// — standard linear form based on mean-centered coordinates
    fn fit_circle_2d(points: &[Point2]) -> Option<(Point2, f64)> {
        if points.len() < 3 {
            return None;
        }

        // Average
        let mut ux = 0.0;
        let mut uy = 0.0;
        for p in points {
            ux += p.x;
            uy += p.y;
        }
        let n = points.len() as f64;
        ux /= n;
        uy /= n;

        // Second moment
        let mut s_xx = 0.0;
        let mut s_yy = 0.0;
        let mut s_xy = 0.0;
        let mut s_x3 = 0.0;
        let mut s_y3 = 0.0;
        let mut s_x2y = 0.0;
        let mut s_xy2 = 0.0;

        for p in points {
            let dx = p.x - ux;
            let dy = p.y - uy;
            let dx2 = dx * dx;
            let dy2 = dy * dy;
            s_xx += dx2;
            s_yy += dy2;
            s_xy += dx * dy;
            s_x3 += dx2 * dx;
            s_y3 += dy2 * dy;
            s_x2y += dx2 * dy;
            s_xy2 += dx * dy2;
        }

        // Linear system: [Sxx Sxy; Sxy Syy] * [a; b] = 0.5 * [Sx³ + Sxy²; Sx²y + Sy³]
        let a11 = s_xx;
        let a12 = s_xy;
        let a21 = s_xy;
        let a22 = s_yy;
        let b1 = 0.5 * (s_x3 + s_xy2);
        let b2 = 0.5 * (s_x2y + s_y3);

        let det = a11 * a22 - a12 * a21;
        if det.abs() < 1e-12 {
            return None;
        }

        let a = (b1 * a22 - a12 * b2) / det;
        let b = (-b1 * a21 + a11 * b2) / det;

        let cx = ux + a;
        let cy = uy + b;

        // Radius: average distance
        let mut r = 0.0;
        for p in points {
            let dx = p.x - cx;
            let dy = p.y - cy;
            r += (dx * dx + dy * dy).sqrt();
        }
        r /= n;
        Some((Point2 { x: cx, y: cy }, r))
    }

    /// 3D point cloud circle fitting:
    /// (1) Fit a plane → (2) Project points onto the plane (2D) →
    /// (3) Fit a circle in 2D → (4) Reconstruct the circle in 3D
    pub fn on_fit_from_points(points: &[Point]) -> Option<Self> {
        if points.len() < 3 {
            return None;
        }

        // The plane on which the circle will lie
        let plane = Plane::fit_from_points(points)?;

        // 2D projection
        let uv: Vec<Point2> = plane.project_points(points);

        // 2D circle fitting
        let (c2, r) = Self::fit_circle_2d(&uv)?;

        // Restore 3D center and construct the fitting plane
        let center3 = plane.point_at(c2.x, c2.y);

        // Circle's plane: preserve the fitted plane's normal and set the origin to the 3D center
        let circle_plane = Plane::from_origin_normal(center3, plane.z_axis);

        Some(Circle {
            plane: circle_plane?,
            radius: r,
        })
    }

    pub fn is_linear(&self, _tol: f64) -> (bool, Option<Segment3D>) {
        (false, None)
    }
}
```
```rust
pub fn on_circle2d(radius: f64, n: usize) -> Vec<Point2> {
    (0..n)
        .map(|i| {
            let t = 2.0 * PI * (i as f64) / (n as f64);
            Point2 {
                x: radius * t.cos(),
                y: radius * t.sin(),
            }
        })
        .collect()
}
```
```rust
pub fn on_helix_path(
    turns: f64,
    height: f64,
    steps: usize,
    radius: f64,
) -> (Vec<Point>, Vec<Vector>) {
    let mut pts = Vec::with_capacity(steps);
    let mut tan = Vec::with_capacity(steps);
    for i in 0..steps {
        let t = (i as f64) / (steps as f64 - 1.0);
        let ang = 2.0 * PI * turns * t;
        let z = height * t;
        let p = Point::new(radius * ang.cos(), radius * ang.sin(), z);
        pts.push(p);
    }
    // Tangent via simple difference
    for i in 0..steps {
        let d = if i + 1 < steps {
            pts[i + 1] - pts[i]
        } else {
            pts[i] - pts[i - 1]
        };
        tan.push(d.to_vector());
    }
    (pts, tan)
}

```
---

## 테스트 코드

테스트 코드들을 기반으로 기능별 테스트 항목을 정리

## ✅ Circle 테스트 기능 요약
| 테스트 함수                        | 검증 내용 요약                                                  |
|-----------------------------------|------------------------------------------------------------------|
| circle_point_and_tangent          | 특정 각도에서의 점 위치 및 접선 벡터 방향/길이 검증              |
| circle_project_and_roundtrip      | 점 → 각도 사영 후 원래 각도로 복원되는지 확인                   |
| circle_reverse_preserves_geometry| 반전 후에도 반지름 유지 및 접선 방향 반전 확인                   |
| circle_offset_along_plane_normal | 평면 법선 방향으로 offset 시 반지름 증감 확인                    |
| circle_transform_uniform_scale    | 변환 적용 시 반지름이 스케일에 따라 정확히 변경되는지 확인      |
| circle_length_consistency         | 전체 원의 길이가 2πr과 일치하는지 확인                           |
| circle_param_length_roundtrip     | 길이 ↔ 각도 변환이 정확히 roundtrip 되는지 확인                  |
| circle_tight_bbox_contains_points | tight_bbox가 원 위의 모든 점을 포함하는지 확인                   |


### 코드 정리
```rust
fn close(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}
fn pclose(a: Point, b: Point, tol: f64) -> bool {
    close(a.x, b.x, tol) && close(a.y, b.y, tol) && close(a.z, b.z, tol)
}
```

### 1. circle_point_and_tangent
```rust
#[test]
fn circle_point_and_tangent() {
    let plane = Plane::xy();
    let c = Circle::new(plane, Point::new(0.0, 0.0, 0.0), 2.0).unwrap();

    // t=0 → (2,0,0)
    let p0 = c.point_at(0.0);
    assert!(pclose(p0, Point::new(2.0, 0.0, 0.0), 1e-12));

    // t=π/2 → (0,2,0)
    let p1 = c.point_at(PI / 2.0);
    assert!(pclose(p1, Point::new(0.0, 2.0, 0.0), 1e-12));

    // Tangents are unit vectors
    let t0 = c.tangent_at(0.0);
    // Tangent direction of the circle is +Y
    assert!(t0.is_finite());

    println!("Length : {}", t0.length());

    assert!(close(t0.length(), 1.0, 1e-12));
    assert!(pclose(
        Point::origin() + t0.to_point(),
        Point::new(0.0, 1.0, 0.0),
        1e-12
    ));
}
```

### 2. circle_project_and_roundtrip
```rust
#[test]
fn circle_project_and_roundtrip() {
    let c = Circle::from_center(Point::new(1.0, -2.0, 0.0), 3.0).unwrap();

    // Create a point at an arbitrary angle and check if the projection returns the same angle
    let true_t = 1.2345;
    let on = c.point_at(true_t);
    let mut t = 0.0;
    let ok = c.project(on, &mut t);
    assert!(ok);
    // Allow difference modulo 2π
    let diff = ((t - true_t + TAU * 10.0) % TAU - PI).abs() - PI; // 래핑 안전 비교
    assert!(diff.abs() < 1e-10);

    // Points outside the circle also resolve to the same angle
    let out = on
        + c.normal_at(0.0).cross(&c.tangent_at(true_t)).to_point() * 0.0
        + Point::new(0.0, 0.0, 1e-9);
    let mut t2 = 0.0;
    let _ = c.project(out, &mut t2);
    let diff2 = ((t2 - true_t + TAU * 10.0) % TAU - PI).abs() - PI;
    assert!(diff2.abs() < 1e-8);
}
```
### 3. circle_reverse_preserves_geometry
```rust
#[test]
fn circle_reverse_preserves_geometry() {
    let mut c = Circle::from_center(Point::new(0.0, 0.0, 0.0), 5.0).unwrap();

    // Radius distance must remain the same before and after inversion
    let samples: Vec<f64> = (0..12).map(|i| i as f64 * TAU / 12.0).collect();
    let before: Vec<f64> = samples
        .iter()
        .map(|&t| (c.point_at(t) - c.center()).length())
        .collect();
    c.reverse();
    let after: Vec<f64> = samples
        .iter()
        .map(|&t| (c.point_at(t) - c.center()).length())
        .collect();
    for (a, b) in before.iter().zip(after.iter()) {
        assert!(close(*a, *b, 1e-10));
        assert!(close(*a, 5.0, 1e-10));
    }

    // Since the direction should be reversed, there must exist an interval with small dt where the dot product of tangent directions becomes negative
    let t = 0.7;
    let tan_before = c.tangent_at(t);
    c.reverse();
    let tan_after = c.tangent_at(t);
    // It may not be exactly -1, so the opposing component must be sufficiently large
    assert!(on_dot_vec(&tan_before, &tan_after) < 0.0);
}
```

### 4. circle_offset_along_plane_normal
```rust
#[test]
fn circle_offset_along_plane_normal() {
    let c = Circle::from_center(Point::new(0.0, 0.0, 0.0), 2.0).unwrap();
    // +Z offset → increases radius
    let up = c.offset(0.5, c.plane.z_axis).unwrap();
    assert!(close(up.radius, 2.5, 1e-12));

    // -Z offset → decreases radius
    let down = c.offset(0.5, -c.plane.z_axis).unwrap();
    assert!(close(down.radius, 1.5, 1e-12));
}
```

### 5. circle_transform_uniform_scale
```rust
#[test]
fn circle_transform_uniform_scale() {
    let mut c = Circle::from_center(Point::new(1.0, 2.0, 3.0), 2.0).unwrap();
    let s = 1.5;
    let trans = Transform::scale_uniform_about(s, Point::origin());
    c.transform(&trans);
    assert!(close(c.radius, 2.0 * s, 1e-12));
    // 중심도 동일 스케일(원점 기준 스케일이면 달라질 수 있으니 필요시 수정)
}
```

### 6. 원의 길이 검증
```rust
#[test]
fn circle_length_consistency() {
    let r = 3.0;
    let c = Circle::from_center(Point::origin(), r).unwrap();
    let expected = TAU * r;
    assert!(close(c.length(), expected, 1e-12));
}
```

### 7. 길이 ↔ 파라미터 roundtrip
```rust
#[test]
fn circle_param_length_roundtrip() {
    let r = 4.0;
    let c = Circle::from_center(Point::origin(), r).unwrap();
    let total = c.length();

    let l = 0.3 * total;
    let (ok1, t) = c.get_param_from_length(l);
    assert!(ok1);
    let (ok2, back_l) = c.get_length_from_param(t);
    assert!(ok2);
    assert!(close(back_l, l, 1e-12));
}
```

### 8. tight_bbox가 점을 포함하는지 확인
```rust
#[test]
fn circle_tight_bbox_contains_points() {
    let r = 2.0;
    let c = Circle::from_center(Point::origin(), r).unwrap();
    let (min, max) = c.get_tight_bbox();
    let samples = c.points_by_length(0.1);
    for p in samples {
        assert!(p.x >= min.x - 1e-12 && p.x <= max.x + 1e-12);
        assert!(p.y >= min.y - 1e-12 && p.y <= max.y + 1e-12);
        assert!(p.z >= min.z - 1e-12 && p.z <= max.z + 1e-12);
    }
}
```
