# Circle
Circle 구조체의 전체 함수 목록과 관련 수식 정리, 그리고 수학적 검토 결과입니다.  
각 함수가 어떤 역할을 수행하는지, 어떤 수학적 원리에 기반하는지, 그리고 구현상 문제가 없는지를 분석했습니다.

## 🧩 Circle 구조체 함수 정리
| 함수 이름                                | 수식 사용 여부 | 설명 및 수학적 의미                                                                 |
|------------------------------------------|----------------|--------------------------------------------------------------------------------------|
| `new`                                    | ✅ 있음         | 평면과 중심, 반지름으로 원 생성. 반지름 유효성 검사 포함.                           |
| `from_center`                            | ✅ 있음         | XY 평면 기준으로 중심과 반지름으로 원 생성.                                          |
| `center`                                 | ✅ 있음        | 원의 중심 좌표 반환.                                                                 |
| `domain`                                 | ✅ 있음         | 파라미터 범위 [0, 2π] 반환.                                                          |
| `length`                                 | ✅ 있음         | $L = 2\pi r$ — 원 둘레 길이 계산.                                                |
| `is_point`                               | ✅ 있음        | 반지름이 매우 작으면 점으로 간주.                                                    |
| `point_at(t)`                            | ✅ 있음         | $P(t) = C + r \cos t \cdot X + r \sin t \cdot Y$ — 원 위 점 계산.                |
| `tangent_at(t)`                          | ✅ 있음         | $T(t) = -r \sin t \cdot X + r \cos t \cdot Y$ — 접선 벡터 계산.                  |
| `normal_at(t)`                           | ✅ 있음         | 평면 법선 벡터 반환.                                                                 |
| `reverse()`                              | ✅ 있음         | Y축 반전하여 파라미터 방향 반전.                                                     |
| `offset(amount, plane_normal)`           | ✅ 있음         | 법선 방향에 따라 반지름 조정. dot product 기반 분기 처리.                           |
| `points_by_length(step)`                 | ✅ 있음         | 일정 간격으로 원 위의 점 생성. $t_i = \frac{2\pi i}{n}$                         |
| `get_param_from_length(length)`          | ✅ 있음         | $t = \frac{\text{length}}{r}$ — 길이 기준 파라미터 계산.                         |
| `get_param_from_length_with_total(l, L)` | ✅ 있음         | $t = t_0 + \frac{l}{r}$ — 전체 길이 기준 파라미터 계산.                          |
| `get_length_from_param(t)`               | ✅ 있음         | $\text{length} = |t - t_0| \cdot r$ — 파라미터 기준 길이 계산.                   |
| `transform(t)`                           | ✅ 있음         | 평면 및 반지름에 변환 적용. 축별 스케일 고려.                                       |
| `get_tight_bbox()`                       | ✅ 있음         | $r_i = r \cdot \sin(\theta_i)$ — 각 축에 대한 투영 반지름 계산.                  |
| `to_nurbs()`                             | ✅ 있음         | 8분할 원 근사. 가중치 $w = \frac{1}{\sqrt{2}}$, degree 2, 9 control points.      |
| `fit_from_points(points)`                | ✅ 있음         | 3D 점들로부터 평면 적합 → 2D 투영 → 원 적합.                                        |
| `on_fit_from_points(points)`            | ✅ 있음         | 위와 동일. 중복 구현.                                                                |
| `fit_circle_2d(points)`                  | ✅ 있음         | Kåsa 방식 선형 근사. 2차 모멘트 기반 중심 및 반지름 계산.                           |
| `project(point, &mut t)`                 | ✅ 있음         | $t = \tan^{-1}\left(\frac{tt}{s}\right)$ — 평면 투영 후 각도 계산.               |
| `is_linear(tol)`                         | ✅ 있음         | line이 아니므로 항상 false 반환.                                                    |


## 📐 Circle 관련 수식 정리표

| 기능 또는 맥락             | 수식 표현                                                                 |
|----------------------------|----------------------------------------------------------------------------|
| 원 둘레 길이               | $L = 2\pi r$                                                           |
| 원 위 점 계산 (`point_at`) | $P(t) = C + r \cos t \cdot X + r \sin t \cdot Y$                       |
| 접선 벡터 (`tangent_at`)   | $T(t) = -r \sin t \cdot X + r \cos t \cdot Y$                          |
| 보간 계수 계산             | $t = \frac{z - z_0}{z_1 - z_0}$                                        |
| 길이 → 파라미터 변환       | $t = t_0 + \frac{\text{length}}{r}$                                    |
| 파라미터 → 길이 변환       | $\text{length} = \|t - t_0\| \cdot r$                                    |


## ✅ 1. 2D 원 적합 수식 (Kåsa 방식)
- 수식:

$$
\left[ \begin{matrix}S_{xx}&S_{xy}\\ \quad S_{xy}&S_{yy}\end{matrix}\right] \left[ \begin{matrix}a\\ \quad b\end{matrix}\right] =\frac{1}{2}\left[ \begin{matrix}S_{x^3}+S_{xy^2}\\ \quad S_{x^2y}+S_{y^3}\end{matrix}\right]
$$

- 검토 결과:
    - 이 수식은 평균 중심 좌표계에서 원의 중심을 선형 시스템으로 근사하는 Kåsa 알고리즘의 핵심입니다.
    - 좌변은 2차 모멘트 행렬, 우변은 3차 모멘트 기반의 선형 항입니다.
    - 문제 없음: 수식 구조는 정확하며, 원의 중심을 구하는 데 적절합니다.
    - 단, 이 방식은 최소자승 근사이며, 특이행렬일 경우 적합 실패 가능성이 있으므로 det ≈ 0 체크가 필요합니다.

## ✅ 2. 원의 외접 박스 계산 수식
- 수식:

$$
\mathrm{extent_{\mathnormal{i}}}=r\cdot \sin (\theta _i)\quad \mathrm{where\  }\theta _i=\cos ^{-1}(\vec {n}\cdot \vec {e}_i)
$$

- 검토 결과:
    - 이 수식은 원의 법선 벡터 $\vec {n}$ 과 각 축 벡터 $\vec {e}_i (x, y, z)$ 에 대해 이루는 각도 $\theta _i$ 를 구하고, 그 축 방향으로 투영된 반지름을 계산합니다.
    - \sin (\theta _i)는 해당 축에 대한 원의 최대 확장을 의미합니다.
    - 문제 없음: 수치적으로 안정적이며, 3D 공간에서 원의 tight bounding box를 계산하는 데 적절합니다.


## ✅ 수학적 검토 결과

| 항목                         | 검토 결과 설명                                                                 |
|------------------------------|----------------------------------------------------------------------------------|
| 원 둘레 길이                 | $L = 2\pi r$ — 정확한 공식 사용. 구현상 문제 없음.                          |
| 원 위 점 계산 (`point_at`)   | $P(t) = C + r \cos t \cdot X + r \sin t \cdot Y$ — 원의 매개변수화로 정확함. |
| 접선 벡터 (`tangent_at`)     | $T(t) = -r \sin t \cdot X + r \cos t \cdot Y$ — 미분 기반으로 정확함.        |
| 길이 → 파라미터 변환         | $t = t_0 + \frac{\text{length}}{r}$ — 선형 관계로 정확함.                    |
| 파라미터 → 길이 변환         | $\text{length} = \|t - t_0\| \cdot r$ — 반지름 기반 거리 계산.                 |
| 보간 계수 계산               | $t = \frac{z - z_0}{z_1 - z_0}$ — 선형 보간 공식으로 정확함.                 |
| 2D 원 적합 (`fit_circle_2d`) | Kåsa 방식 선형 근사. 특이행렬 처리 포함. 수학적으로 타당하고 안정적.             |
| NURBS 변환 (`to_nurbs`)      | 8분할 원 근사. 가중치 및 노드 배치 모두 표준 방식 따름.                         |


## 코드

```rust
use crate::core::boundingbox::on_bounding_box_points;
use crate::core::domain::Interval;
use crate::core::geom::{Point2D, Point4D};
use crate::core::plane::Plane;
use crate::core::prelude::{KnotVector, NurbsCurve, Point3D, Vector3D};
use crate::core::segment3d::Segment3D;
use crate::core::transform::Transform;
use crate::core::types::{ON_TOL6, ON_TOL9, ON_TOL12, on_are_equal_scaled};
use std::f64::consts::{PI, TAU};

/// Circle — circle on a Plane with radius r. Domain = [0, 2π].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub plane: Plane,
    pub radius: f64,
}
```
```rust
impl Circle {
    pub fn fit_from_points(points: &[Point3D]) -> Option<Self> {
        if points.len() < 3 {
            return None;
        }

        // The plane on which the circle will lie
        let plane = Plane::fit_from_points(points)?;

        // 2D projection
        let uv: Vec<Point2D> = plane.project_points(points);

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
}
```
```rust
impl Circle {
    #[inline]
    pub fn project(&self, point: Point3D, t_out: &mut f64) -> bool {
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
    pub fn new(plane: Plane, center: Point3D, radius: f64) -> Option<Self> {
        if radius.is_finite() && radius > ON_TOL12 {
            let mut pln = plane;
            pln.origin = center;
            Some(Self { plane: pln, radius })
        } else {
            None
        }
    }

    pub fn from_center(center: Point3D, radius: f64) -> Option<Self> {
        Self::new(Plane::xy(), center, radius)
    }

    #[inline]
    pub fn center(&self) -> Point3D {
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
    pub fn point_at(&self, t: f64) -> Point3D {
        let mut a = t % TAU;
        if a < 0.0 {
            a += TAU;
        }
        let c = a.cos();
        let s = a.sin();
        self.plane.point_at(self.radius * c, self.radius * s)
    }

    /// Tangent direction at parameter t (unit vector in 3D).
    pub fn tangent_at(&self, t: f64) -> Vector3D {
        // d/dt (center + r cos t * X + r sin t * Y) = -r sin t * X + r cos t * Y
        let mut a = t % TAU;
        if a < 0.0 {
            a += TAU;
        }
        let v = self.plane.x_axis * (-self.radius * a.sin())
            + self.plane.y_axis * (self.radius * a.cos());
        v.unitize()
    }

    pub fn normal_at(&self, _t: f64) -> Vector3D {
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
    pub fn offset(&self, amount: f64, plane_normal: Vector3D) -> Option<Self> {
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

        let dot = Vector3D::dot(&n, &z);

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

    pub fn points_by_length(&self, step: f64) -> Vec<Point3D> {
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
    pub fn get_tight_bbox(&self) -> (Point3D, Point3D) {
        let n = self.plane.z_axis; // Unit normal (assumed to be maintained)
        // 각도 = acos(clamp(dot(n, axis), -1..1))
        fn ang_between(a: Vector3D, b: Vector3D) -> f64 {
            let mut d = Vector3D::dot(&a, &b);
            if d < -1.0 {
                d = -1.0;
            }
            if d > 1.0 {
                d = 1.0;
            }
            d.acos()
        }

        let a1 = ang_between(n, Vector3D::new(1.0, 0.0, 0.0));
        let a2 = ang_between(n, Vector3D::new(0.0, 1.0, 0.0));
        let a3 = ang_between(n, Vector3D::new(0.0, 0.0, 1.0));

        // Projected radius on each axis = r * sin(angle between normal and that axis)
        let extent = Vector3D::new(a1.sin(), a2.sin(), a3.sin()) * self.radius;

        let p_min = self.center() - extent.to_point();
        let p_max = self.center() + extent.to_point();

        // For numeric safety, recheck min/max using two points
        on_bounding_box_points(&[p_min, p_max]).unwrap()
    }

    /// Convert a circle into a standard NURBS (i.e., BSplineCurve). Degree 2, 8 spans (12 knots) in standard configuration.
    pub fn to_nurbs(&self) -> NurbsCurve {
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
            Point4D::from_point3_weight(c + x * r, 1.0),
            // 45 deg (weighted)
            Point4D::from_point3_weight(c + x * r + y * r, w),
            // 90
            Point4D::from_point3_weight(c + y * r, 1.0),
            // 135
            Point4D::from_point3_weight(c - x * r + y * r, w),
            // 180
            Point4D::from_point3_weight(c - x * r, 1.0),
            // 225
            Point4D::from_point3_weight(c - x * r - y * r, w),
            // 270
            Point4D::from_point3_weight(c - y * r, 1.0),
            // 315
            Point4D::from_point3_weight(c + x * r - y * r, w),
            // 360 (닫힘: 첫점 복제)
            Point4D::from_point3_weight(c + x * r, 1.0),
        ];

        // One more note: when w ≠ 1, the internal representation must be converted according to project conventions—whether it's (x, y, z, w) or (wx, wy, wz, w).
        // Here, we assume Point4D::from_point(p, w) internally stores (p.x * w, p.y * w, p.z * w, w).
        NurbsCurve {
            dimension: 3,
            degree: 2,
            ctrl: cp,
            knots: KnotVector { knots },
            domain: Interval { t0: 0.0, t1: 1.0 },
        }
    }
}
```
```rust
impl Circle {
    /// Circle fitting for 2D points (Kåsa-style linear solution)
    /// — standard linear form based on mean-centered coordinates
    fn fit_circle_2d(points: &[Point2D]) -> Option<(Point2D, f64)> {
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
        Some((Point2D { x: cx, y: cy }, r))
    }
```
```rust
    /// 3D point cloud circle fitting:
    /// (1) Fit a plane → (2) Project points onto the plane (2D) →
    /// (3) Fit a circle in 2D → (4) Reconstruct the circle in 3D
    pub fn on_fit_from_points(points: &[Point3D]) -> Option<Self> {
        if points.len() < 3 {
            return None;
        }

        // The plane on which the circle will lie
        let plane = Plane::fit_from_points(points)?;

        // 2D projection
        let uv: Vec<Point2D> = plane.project_points(points);

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
pub fn on_circle2d(radius: f64, n: usize) -> Vec<Point2D> {
    (0..n)
        .map(|i| {
            let t = 2.0 * PI * (i as f64) / (n as f64);
            Point2D {
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
) -> (Vec<Point3D>, Vec<Vector3D>) {
    let mut pts = Vec::with_capacity(steps);
    let mut tan = Vec::with_capacity(steps);
    for i in 0..steps {
        let t = (i as f64) / (steps as f64 - 1.0);
        let ang = 2.0 * PI * turns * t;
        let z = height * t;
        let p = Point3D::new(radius * ang.cos(), radius * ang.sin(), z);
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

# 테스트


## 🧪 테스트 코드 정리표
| 테스트 함수 이름                              | 대상 구조체 | 검증 기능 또는 메서드                        | 핵심 검증 내용                                      |
|-----------------------------------------------|-------------|---------------------------------------------|-----------------------------------------------------|
| `line_circle`                                  | Line, Circle| `length`, `points_by_length`, `offset`, `point_at`, `tangent_at` | 선 길이, 점 샘플링, 오프셋, 원의 점 및 접선 계산     |
| `line_offset_right_hand_convention`            | Line        | `offset`                                    | 오른손 좌표계 기준 오프셋 방향 확인                 |
| `line_offset_negative_amount_moves_opposite`   | Line        | `offset`                                    | 음수 오프셋 시 반대 방향 이동 확인                  |
| `circle_offset_parallel_increases_radius`      | Circle      | `offset`, `length`                          | 평행 법선일 때 반지름 증가 및 길이 일관성 확인      |
| `circle_offset_anti_parallel_decreases_radius` | Circle      | `offset`                                    | 반대 방향 법선일 때 반지름 감소 확인                |
| `circle_offset_cannot_cross_zero_radius`       | Circle      | `offset`                                    | 반지름이 0 이하가 되면 None 반환 확인              |
| `circle_offset_non_coplanar_returns_none`      | Circle      | `offset`                                    | 비공면 법선일 경우 None 반환 확인                   |
| `circle_offset_zero_length_normal_returns_none`| Circle      | `offset`                                    | 법선 벡터 길이가 0일 경우 None 반환 확인            |
| `circle_tangent_direction_sanity`              | Circle      | `point_at`, `tangent_at`                    | 각도별 위치 및 접선 방향 검증                       |
| `line_is_degenerate_check`                     | Line        | `is_degenerate`                             | 시작과 끝이 같은 경우 퇴화 여부 확인                |
| `line_angle_with_horizontal_and_vertical`      | Line        | `angle_with`                                | 수직 각도 계산 확인                                 |
| `line_extend_should_increase_length`           | Line        | `extend`                                    | 선 연장 시 길이 증가 확인                           |
| `line_sample_uniform_should_return_n_plus_one_points` | Line | `sample_uniform`                            | 균등 샘플링 시 n+1개 점 반환 확인                   |



## ✅ 테스트 커버리지 요약
- Line 구조체: 길이, 오프셋, 각도, 샘플링, 퇴화 여부 등 대부분의 기능을 포괄적으로 테스트.
- Circle 구조체: 반지름 조정, 접선 계산, 점 생성, 예외 처리 등 핵심 기능을 정밀하게 검증.
- 오프셋 로직: 평행/역평행/비공면/0벡터 등 다양한 케이스에 대해 예외 처리 포함.
- 기하학적 정확성: 접선 방향, 각도 계산, 길이 일관성 등 수학적 검증 포함.


```rust
#[cfg(test)]
mod tests {

    use nurbslib::core::circle::Circle;
    use nurbslib::core::line::Line;
    use nurbslib::core::plane::Plane;
    use nurbslib::core::prelude::{Point3D, Vector3D};
    use nurbslib::core::types::ON_TOL9;
    use std::f64::consts::{PI, TAU};
```
```rust
    #[test]
    fn line_circle() {
        // GLine
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        assert!((ln.length() - 10.0).abs() < 1e-12);
        let pts = ln.points_by_length(2.5);
        assert_eq!(pts.len(), 5);

        let off = ln.offset(1.0, Vector3D::new(0.0, 0.0, 1.0));
        println!("{:?}", off);
        assert!((off.start.y - 1.0).abs() < 1e-12);

        // GCircle
        let c = Circle::from_center(Point3D::new(0.0, 0.0, 0.0), 2.0).unwrap();
        assert!((c.length() - std::f64::consts::TAU * 2.0).abs() < 1e-12);
        let _p0 = c.point_at(0.0); // (r,0,0)
        let _t0 = c.tangent_at(0.0); // +Y 방향
        let _off = c.offset(0.25, c.plane.z_axis).unwrap(); // 반지름 증가
    }
```
```rust
    #[test]
    fn line_offset_right_hand_convention() {
        // 선: +X 방향, 법선: +Z
        // 우리가 채택한 규칙: 양의 amount => plane_normal × tangent 방향
        // +Z × +X = +Y 이므로, y가 +amount 만큼 이동해야 함.
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let off = ln.offset(1.0, Vector3D::new(0.0, 0.0, 1.0));
        assert!((off.start.y - 1.0).abs() < ON_TOL9, "expected +Y offset");
        assert!((off.end.y - 1.0).abs() < ON_TOL9, "expected +Y offset");
        assert!(
            (off.length() - ln.length()).abs() < ON_TOL9,
            "offset must preserve length"
        );
    }
```
```rust
    #[test]
    fn line_offset_negative_amount_moves_opposite() {
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let off = ln.offset(-1.0, Vector3D::new(0.0, 0.0, 1.0));
        assert!((off.start.y + 1.0).abs() < ON_TOL9, "negative amount => -Y");
        assert!((off.end.y + 1.0).abs() < ON_TOL9, "negative amount => -Y");
    }
```
```rust
    #[test]
    fn circle_offset_parallel_increases_radius() {
        // 원: XY 평면, plane.z_axis = +Z
        let c = Circle::new(Plane::xy(), Point3D::new(0.0, 0.0, 0.0), 2.0).unwrap();
        let off = c
            .offset(0.25, c.plane.z_axis)
            .expect("parallel normal should work");
        assert!((off.radius - 2.25).abs() < ON_TOL9, "r + amount (parallel)");
        // 길이도 일관 확인
        assert!((off.length() - TAU * 2.25).abs() < 1e-8);
    }
```
```rust
    #[test]
    fn circle_offset_anti_parallel_decreases_radius() {
        // 반대 방향 법선: -Z -> r - amount
        let c = Circle::new(Plane::xy(), Point3D::new(0.0, 0.0, 0.0), 2.0).unwrap();
        let off = c
            .offset(0.5, -c.plane.z_axis)
            .expect("anti-parallel normal should work");
        assert!(
            (off.radius - 1.5).abs() < ON_TOL9,
            "r - amount (anti-parallel)"
        );
    }
```
```rust
    #[test]
    fn circle_offset_cannot_cross_zero_radius() {
        let c = Circle::new(Plane::xy(), Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();
        // r - amount <= 0 인 경우는 None 이어야 한다.
        let none = c.offset(1.0, -c.plane.z_axis);
        assert!(none.is_none(), "radius must not become <= 0");
    }
```
```rust
    #[test]
    fn circle_offset_non_coplanar_returns_none() {
        // 비공면 법선: +Z와 평행/역평행이 아닌 벡터
        let c = Circle::new(Plane::xy(), Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();
        let ncp = Vector3D::new(0.0, 1.0, 1.0); // 비평행
        let res = c.offset(0.25, ncp);
        assert!(
            res.is_none(),
            "non-coplanar offset should be None (NURBS case)"
        );
    }
```
```rust
    #[test]
    fn circle_offset_zero_length_normal_returns_none() {
        let c = Circle::new(Plane::xy(), Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();
        let res = c.offset(0.25, Vector3D::new(0.0, 0.0, 0.0));
        assert!(res.is_none(), "zero-length normal => None");
    }
```
```rust
    #[test]
    fn circle_tangent_direction_sanity() {
        let c = Circle::new(Plane::xy(), Point3D::new(0.0, 0.0, 0.0), 1.0).unwrap();
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
```rust
    #[test]
    fn line_is_degenerate_check() {
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!(ln.is_degenerate());
    }
```
```rust
    #[test]
    fn line_angle_with_horizontal_and_vertical() {
        let ln1 = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0); // +X
        let ln2 = Line::from_xyz(0.0, 0.0, 0.0, 0.0, 10.0, 0.0); // +Y
        let angle = ln1.angle_with(&ln2);
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn line_extend_should_increase_length() {
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let ext = ln.extend(1.0);
        assert!((ext.length() - 3.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn line_sample_uniform_should_return_n_plus_one_points() {
        let ln = Line::from_xyz(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let samples = ln.sample_uniform(4);
        assert_eq!(samples.len(), 5);
        assert!((samples[2].x - 5.0).abs() < 1e-12);
    }
}
```

---

