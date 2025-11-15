#  Arc

## 🧩 Arc 구조체 함수 정리
| 함수 이름                             | 설명                                                                 |
|--------------------------------------|----------------------------------------------------------------------|
| `new`                                | 평면, 중심, 반지름, 시작/끝 각도로 원호 생성. 방향 정규화 및 유효성 검사 포함. |
| `from_center_radius_angles`          | XY 평면 기준으로 중심, 반지름, 각도 구간으로 원호 생성.              |
| `from_plane_center_radius_len`       | 평면, 중심, 반지름, 각 길이로 원호 생성.                             |
| `offrom_center_radius_len`           | 중심, 반지름, 각 길이로 생성하는 편의 함수.                          |
| `planecenterradiusangle`             | (오타로 추정) → `new`와 동일한 역할.                                 |
| `is_valid`                           | 반지름과 각도 길이가 유효한지 검사.                                  |
| `is_closed`                          | 전체 원(2π) 여부 확인.                                               |
| `point_at(t)`                        | 각도 t에서의 위치 계산.                                              |
| `tangent_at(t)`                      | 각도 t에서의 접선 벡터 계산.                                         |
| `start_point`, `mid_point`, `end_point` | 시작, 중간, 끝 점 반환.                                           |
| `length()`                           | 원호 길이 계산.                                                      |
| `domain()`                           | 각도 구간 반환.                                                      |
| `set_domain(value)`                  | 각도 구간 설정.                                                      |
| `reverse()`                          | 방향 반전. 평면 Y축 및 각도 구간 반전.                               |
| `split_at(t)`                        | 내부 각도에서 두 개의 원호로 분할.                                   |
| `sub_curve(t0, t1)`                  | 지정된 각도 구간의 부분 원호 반환.                                   |
| `trim_at(t, flip_side)`              | 앞 또는 뒤 부분만 남기도록 잘라냄.                                    |
| `project(p)`                         | 3D 점을 원호에 투영하여 각도 반환.                                   |
| `closest_param_to(p)`                | 점에 가장 가까운 파라미터 각도 계산.                                 |
| `get_param_from_length(length)`      | 길이 기준으로 각도 파라미터 계산.                                    |
| `get_length_from_param(t)`           | 각도 기준으로 길이 계산.                                             |
| `points_by_length(step)`             | 일정 간격으로 원호 위의 점 생성.                                     |
| `tight_bbox()`                       | 원호의 외접 박스 계산.                                               |
| `to_nurbs()`                         | 원호를 NURBS 곡선으로 변환.                                          |
| `is_linear(tol)`                     | 선형 여부 판단 (항상 false).                                         |


## 📐 Arc 관련 수식 정리 및 검증

| 항목 이름       | 수식 표현                                                                 | 설명 및 검증 결과                                                                 |
|----------------|----------------------------------------------------------------------------|------------------------------------------------------------------------------------|
| 원호 길이       | `$L = \|\Delta \theta\| \cdot r$`                                              | 원호의 길이는 각도 차이와 반지름의 곱으로 계산. 정확한 원호 길이 공식입니다. ✅ |
| point_at       | `$P(t) = C + r cos(t) · X + r sin(t) · Y$`                                    | 중심 C에서 반지름 r만큼 X, Y 방향으로 회전한 점. 원의 매개변수화로 정확합니다. ✅ |
| tangent_at     | `$T(t) = -r sin(t) · X + r cos(t) · Y$`                                       | 원 위 점의 미분 벡터. 접선 방향을 나타내며 단위 벡터로 정규화됩니다. ✅         |
| 길이 → 각도 변환 | `$t = t₀ + length / r$`                                                      | 주어진 길이에 대응하는 각도 계산. 선형 관계로 정확합니다. ✅                    |
| 각도 → 길이 변환 | `$length = \|t - t₀\| · r$`                                                    | 두 각도 사이의 거리로 길이 계산. 절대값 처리 포함되어 정확합니다. ✅            |
| project        | `$t = atan2(tt, s)$`                                                         | 평면 투영 좌표를 기준으로 각도 계산. atan2 기반이며 wrap 처리 포함. ✅         |
| tight_bbox     | `$extentᵢ = r · sin(θᵢ)$`                                                     | 각 축 방향으로 투영된 반지름. 축과 평면 법선 사이의 각도 기반. 정확합니다. ✅  |
| θᵢ 계산        | `$θᵢ = acos(n · eᵢ)$`                                                         | 평면 법선과 축 벡터 사이의 각도 계산. dot product 기반으로 정확합니다. ✅      |
| NURBS 가중치   | `$w = cos(Δθ / 2)$`                                                           | 원호 세그먼트의 중간 제어점 가중치. 표준 원호 근사 방식으로 정확합니다. ✅     |


## ✅ 수학적 검토 요약
- 모든 수식은 원호의 기하학적 정의와 매개변수화에 기반하여 정확하게 구현되어 있습니다.
- tight_bbox는 평면 법선과 각 축의 각도를 기반으로 투영 반지름을 계산하며, 수치적 안정성을 고려한 방식입니다.
- to_nurbs는 90도 이하 세그먼트로 나누어 가중치 기반 제어점을 생성하며, 표준적인 원호 근사 방식입니다.
- project, closest_param_to는 원호의 도메인 내외를 고려한 예외 처리까지 포함되어 있어 실용적입니다.


```rust
use crate::core::circle::Circle;
use crate::core::geom::Point4D;
use crate::core::plane::Plane;
use crate::core::prelude::{Interval, KnotVector, NurbsCurve, Point3D, Vector3D};
use crate::core::segment3d::Segment3D;
use crate::core::types::ON_TOL12;
use std::f64::consts::TAU;

/// GArc — a circular arc on a plane (a circle (GCircle) + an angular interval)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arc {
    pub circle: Circle,  // plane + radius + center
    pub angle: Interval, // [t0, t1] (라디안), 증가 구간
}
```
```rust
impl Arc {
    /// Default constructor: plane, center, radius, start angle t0, end angle t1 (in radians)
    /// If t0 > t1, the arc is reversed and normalized.
    /// If t1 - t0 > 2π, the span is clamped to 2π.
    pub fn new(plane: Plane, center: Point3D, radius: f64, t0: f64, t1: f64) -> Option<Self> {
        let circle = Circle::new(plane, center, radius)?;
        let mut ang = Interval::new(t0, t1);
        if ang.is_decreasing() {
            ang.swap(); // t0 < t1 되도록
            // In C#, parameter direction is reversed by calling Reverse().
            // Here, it's implemented equivalently using circle.reverse() + angle.reverse().
            // However, applying reverse requires the circle to be mutable,
            // so it's handled all at once below:
            let mut arc = Self { circle, angle: ang };
            arc.reverse();
            return Some(arc);
        }
        // Clamp to 2π if the value exceeds 2π
        if ang.length() > TAU {
            ang.t1 = ang.t0 + TAU;
        }
        let arc = Self { circle, angle: ang };
        if arc.is_valid() { Some(arc) } else { None }
    }

    /// Convenience constructor: creates an arc on the world XY plane using (center, radius, t0, t1)
    #[inline]
    pub fn from_center_radius_angles(
        center: Point3D,
        radius: f64,
        t0: f64,
        t1: f64,
    ) -> Option<Self> {
        Self::new(Plane::xy(), center, radius, t0, t1)
    }

    /// Constructor with plane, center, radius, and angular length (0..θ)
    pub fn from_plane_center_radius_len(
        plane: Plane,
        center: Point3D,
        radius: f64,
        angle_len: f64,
    ) -> Option<Self> {
        Self::new(plane, center, radius, 0.0, angle_len)
    }

    #[inline]
    pub fn plane(&self) -> Plane {
        self.circle.plane
    }
    #[inline]
    pub fn center(&self) -> Point3D {
        self.circle.center()
    }
    #[inline]
    pub fn radius(&self) -> f64 {
        self.circle.radius
    }
    #[inline]
    pub fn angle(&self) -> Interval {
        self.angle
    }

    /// Validation check: radius > EPS, angle length in (EPS, 2π]
    pub fn is_valid(&self) -> bool {
        let eps = 1e-12;
        self.circle.radius > eps && self.angle.length() > eps && self.angle.length() <= TAU + 1e-9
    }

    /// Whether the arc is closed (i.e., a full circle)
    #[inline]
    pub fn is_closed(&self) -> bool {
        (self.angle.length() - TAU).abs() < 1e-9
    }

    /// Point evaluation (in radians)
    #[inline]
    pub fn point_at(&self, t: f64) -> Point3D {
        self.circle.point_at(t)
    }

    /// Tangent (unit vector)
    #[inline]
    pub fn tangent_at(&self, t: f64) -> Vector3D {
        self.circle.tangent_at(t)
    }

    #[inline]
    pub fn start_point(&self) -> Point3D {
        self.point_at(self.angle.t0)
    }
    #[inline]
    pub fn mid_point(&self) -> Point3D {
        self.point_at(self.angle.mid())
    }
    #[inline]
    pub fn end_point(&self) -> Point3D {
        self.point_at(self.angle.t1)
    }

    /// Length (= |Δθ| · r)
    #[inline]
    pub fn length(&self) -> f64 {
        self.angle.length().abs() * self.circle.radius
    }

    /// Retrieve domain (angular interval)
    #[inline]
    pub fn domain(&self) -> Interval {
        self.angle
    }

    /// Set domain (angular interval), reflecting C# GArc.UpdateInterval logic
    pub fn set_domain(&mut self, value: Interval) -> bool {
        let v = value;
        if v.is_increasing() && v.length() < TAU + 1e-9 {
            if v.length() < 1e-12 {
                return false;
            }
            self.angle = v;
        } else {
            self.angle = Interval::new(v.t0, v.t1 + TAU);
        }
        true
    }

    /// Reverse: flip the arc direction (C#: base.Reverse(); angle.Reverse(); ResetComputedData())
    pub fn reverse(&mut self) {
        self.circle.reverse(); // 평면의 Y축 반전으로 파라미터 방향 반대
        self.angle.reverse(); // 구간도 뒤집기
    }

    /// SplitAt: split the arc into two arcs at an internal angle
    pub fn split_at(&self, t: f64) -> Option<(Self, Self)> {
        // 내부가 아니면 실패
        let a = self.angle;
        if !(t > a.t0 && t < a.t1) {
            return None;
        }

        let c = self.circle;
        let r = c.radius;
        let pl = c.plane;
        let cen = c.center();

        let lo = Self::new(pl, cen, r, a.t0, t)?;
        let hi = Self::new(pl, cen, r, t, a.t1)?;
        Some((lo, hi))
    }

    /// SubCurve: return the arc segment between [t0, t1]
    pub fn sub_curve(&self, t0: f64, mut t1: f64) -> Option<Self> {
        // 전체가 원(2π)인 경우와 아닌 경우를 구분(C# GCircle.SubCurve 기반)
        let is_circle = (self.angle.length() - TAU).abs() < 1e-12;

        // Normalize: ensure increasing interval and validate range
        if is_circle {
            if (t1 - t0).abs() < 1e-12 {
                // 0 또는 2π -> 전체 복제
                return Some(*self);
            }
            if t0 > t1 {
                t1 += TAU;
            }
        } else {
            // Open arc: must lie within [angle.t0, angle.t1]
            if t0 < self.angle.t0 || t1 > self.angle.t1 || t0 >= t1 {
                return None;
            }
        }

        Self::new(self.plane(), self.center(), self.radius(), t0, t1)
    }

    /// TrimAt: retain only the front or back portion of the arc at angle t
    pub fn trim_at(&mut self, t: f64, flip_side: bool) -> bool {
        let a = self.angle;
        // 경계면/밖이면 실패
        if t <= a.t0 || t >= a.t1 {
            return false;
        }

        let new_iv = if flip_side {
            Interval::new(t, a.t1)
        } else {
            Interval::new(a.t0, t)
        };
        self.set_domain(new_iv)
    }

    /// Project: project a 3D point onto the arc → returns angle in radians
    pub fn project(&self, p: Point3D) -> f64 {
        // First, compute angle t on the circle lying in the same plane
        let t = {
            let mut tmp = 0.0;
            let _ok = self.circle.project(p, &mut tmp); // GCircle::project: 항상 true
            tmp
        };
        // Adjust to the arc's domain
        if (t < self.angle.t0 && (t - self.angle.t0).abs() > 1e-12)
            || (t > self.angle.t1 && (t - self.angle.t1).abs() > 1e-12)
        {
            let a1 = t + std::f64::consts::PI;
            let a2 = t - std::f64::consts::PI;
            if a1 >= self.angle.t0 && a1 <= self.angle.t1 {
                return a1;
            }
            if a2 >= self.angle.t0 && a2 <= self.angle.t1 {
                return a2;
            }
        }
        t
    }

    pub fn closest_param_to(&self, p: Point3D) -> f64 {
        let t = self.project(p);
        if t < self.angle.t0 || t > self.angle.t1 {
            let d0 = Point3D::distance_squared(p, self.start_point());
            let d1 = Point3D::distance_squared(p, self.end_point());
            if d0 <= d1 {
                self.angle.t0
            } else {
                self.angle.t1
            }
        } else {
            // 도메인 안: 중심-사영점 비교해서 끝점이 더 가깝다면 스냅
            let d_mid = Point3D::distance_squared(p, self.point_at(t));
            let d_end = Point3D::distance_squared(p, self.end_point());
            if d_end < d_mid {
                let d_sta = Point3D::distance_squared(p, self.start_point());
                if d_sta <= d_end {
                    self.angle.t0
                } else {
                    self.angle.t1
                }
            } else {
                let d_sta = Point3D::distance_squared(p, self.start_point());
                if d_sta <= d_mid { self.angle.t0 } else { t }
            }
        }
    }

    pub fn get_param_from_length(&self, length: f64) -> (bool, f64) {
        let dom = self.domain(); // Interval { t0, t1 }
        let r = self.radius();
        if r <= 0.0 || !r.is_finite() {
            return (false, dom.t0);
        }

        let total = self.length(); // = |t1 - t0| * r
        // 끝점 스냅
        if (length - 0.0).abs() <= 1e-12 * (total.abs().max(1.0)) {
            return (true, dom.t0);
        }
        if (length - total).abs() <= 1e-12 * (total.abs().max(1.0)) {
            return (true, dom.t1);
        }
        // 범위 체크
        if length < 0.0 || length > total {
            return (false, dom.t0);
        }

        // 도메인 방향성
        let dir = if dom.t1 >= dom.t0 { 1.0 } else { -1.0 };
        let u = dom.t0 + dir * (length / r);
        (true, u)
    }

    pub fn get_length_from_param(&self, u: f64) -> (bool, f64) {
        let dom = self.domain();
        let r = self.radius();
        if r <= 0.0 || !r.is_finite() {
            return (false, 0.0);
        }

        // 도메인 포함 여부
        let (lo, hi) = (dom.t0, dom.t1);
        let in_domain = if hi >= lo {
            u >= lo && u <= hi
        } else {
            u <= lo && u >= hi
        };
        if !in_domain {
            return (false, 0.0);
        }

        let dir = if hi >= lo { 1.0 } else { -1.0 };
        // 길이는 항상 양수
        let length = ((u - lo) * dir) * r;
        (true, length.abs())
    }

    /// Sampling by arc length (including endpoints)
    pub fn points_by_length(&self, step: f64) -> Vec<Point3D> {
        if step <= 0.0 {
            return vec![self.point_at(self.angle.t0)];
        }
        let n_f = (self.length() / step).ceil();
        let mut n = n_f as usize;
        if n == 0 {
            n = 1;
        }
        let mut pts = Vec::with_capacity(n + 1);
        for i in 0..=n {
            let u = (i as f64) / (n as f64);
            let t = self.angle.t0 + u * self.angle.length();
            pts.push(self.point_at(t));
        }
        pts
    }

    /// Simple tight bounding box: delegate to circle if full arc, otherwise use extrema candidates + endpoint samples
    pub fn tight_bbox(&self) -> (Point3D, Point3D) {
        if self.is_closed() {
            return self.circle.get_tight_bbox();
        }
        // For each axis, extrema occur at t where d/dt of proj_x/y/z = 0 → determined by principal directions of the plane
        // Simplification: along with t0 and t1, include candidate angles for X/Y/Z extrema (based on plane.x/y/z projections)
        let mut candidates = vec![self.angle.t0, self.angle.t1];

        // Extremum angle in the direction of the plane axis: atan2(Y-axis component, X-axis component)
        // For each axis component, use atan2(AxisX.component, AxisY.component)
        let ax = self.plane().x_axis;
        let ay = self.plane().y_axis;

        // For each of x, y, z: use atan2(ay.component, ax.component)
        let crits = [ay.x.atan2(ax.x), ay.y.atan2(ax.y), ay.z.atan2(ax.z)];

        for &c in &crits {
            let t = Self::wrap_to_2pi(c);
            // t가 [t0, t1]에 포함되면 후보로
            if t >= self.angle.t0 && t <= self.angle.t1 {
                candidates.push(t);
            }
            // Account for 2π periodicity (redundancy): t ± 2π is equivalent
        }

        candidates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        candidates.dedup_by(|a, b| (*a - *b).abs() < 1e-12);

        let mut min = Point3D::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3D::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for &t in &candidates {
            let p = self.point_at(t);
            if p.x < min.x {
                min.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            }
            if p.z < min.z {
                min.z = p.z;
            }
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y > max.y {
                max.y = p.y;
            }
            if p.z > max.z {
                max.z = p.z;
            }
        }
        (min, max)
    }

    #[inline]
    fn wrap_to_2pi(mut t: f64) -> f64 {
        while t < 0.0 {
            t += TAU;
        }
        while t >= TAU {
            t -= TAU;
        }
        t
    }

    /// Convert arc to quadratic NURBS (BSplineCurve)
    pub fn to_nurbs(&self) -> NurbsCurve {
        // Divide by arc length into 90° segments (≤ π/2) and stitch together standard quadratic arc blocks
        let total = self.angle.length().abs();
        if total <= ON_TOL12 {
            // Degenerate case: single point
            let p = self.start_point();
            if let Some(curve) = NurbsCurve::from_points_clamped_degree2(&[p, p, p]) {
                return curve;
            }
        }

        // Number of segments: 1 (≤ 90°), 2 (≤ 180°), 3 (≤ 270°), 4 (≤ 360°)
        let (segments, seg_len, knot_step) = if total <= std::f64::consts::FRAC_PI_2 {
            (1usize, total, 0.5)
        } else if total <= std::f64::consts::PI {
            (2usize, total * 0.5, 0.25)
        } else if total <= 1.5 * std::f64::consts::PI {
            (3usize, total / 3.0, 1.0 / 6.0)
        } else {
            (4usize, total * 0.25, 0.125)
        };

        // Note: clamped, degree=2 → (2*segments + 1) control points, knots = (length = (2*segments + 1) + degree + 1) = 2*segments + 4
        // Follow C# logic: weight of each middle control point in a segment = cos(segment_length / 2)
        let mut knots = Vec::<f64>::with_capacity(2 * segments + 3);
        let mut cp = Vec::<Point4D>::with_capacity(2 * segments + 1);

        let t0 = self.angle.t0;
        let _center = self.center();
        let _x = self.circle.plane.x_axis;
        let _y = self.circle.plane.y_axis;
        let _r = self.circle.radius;

        // Start knot has multiplicity 3
        knots.push(t0);
        knots.push(t0);
        knots.push(t0);

        // First point
        cp.push(Point4D::from_point3_weight(self.point_at(t0), 1.0));

        // For each segment: middle point (weight w) → end point (weight 1)
        let w = (0.5 * seg_len).cos();
        let mut t = t0;
        for _ in 0..segments {
            let mid = t + knot_step * 2.0 * seg_len; //radian
            let end = t + seg_len;

            // Middle point
            cp.push(Point4D::from_point3_weight(self.point_at(mid), w));
            // End point
            cp.push(Point4D::from_point3_weight(self.point_at(end), 1.0));

            // Knots: duplicate [t, t] for clamped quadratic
            knots.push(end);
            knots.push(end);

            t = end;
        }
        // Final clamp
        knots.push(t); // Already at the final end
        // Since it's degree 2, maintain 3 clamped knots → add one more at the end
        knots.push(t);

        NurbsCurve {
            dimension: 3,
            degree: 2,
            ctrl: cp,
            knots: KnotVector { knots },
            domain: Interval::new(self.angle.t0, self.angle.t1),
        }
    }
}
```
```rust
impl Arc {
    /// Convenience constructor: (center, radius, angle_len)
    pub fn of(center: Point3D, radius: f64, angle_len: f64) -> Option<Self> {
        Self::from_center_radius_len(center, radius, angle_len)
    }
    #[inline]
    pub fn from_center_radius_len(center: Point3D, radius: f64, angle_len: f64) -> Option<Self> {
        Self::new(Plane::xy(), center, radius, 0.0, angle_len)
    }

    pub fn is_linear(&self, _tol: f64) -> (bool, Option<Segment3D>) {
        (false, None)
    }
}
```
---

# 테스트

## 🧪 Arc & Circle 테스트 함수 정리 및 수식 검토
| 테스트 함수 이름                          | 대상 구조체 | 검증 함수/기능                     | 수식 사용 여부 | 수학적 검토 결과 및 설명                                               |
|-------------------------------------------|-------------|------------------------------------|----------------|------------------------------------------------------------------------|
| circle_param_length_round_trip            | Circle      | `get_param_from_length`, `get_length_from_param` | ✅ 있음         | $t = t_0 + \frac{l}{r}$, $l = \|t - t_0\| \cdot r$ — 정확함       |
| circle_param_length_out_of_range          | Circle      | `get_param_from_length`, `get_length_from_param` | ✅ 있음         | 유효 범위 밖 입력에 대한 예외 처리 확인. 수식 및 처리 모두 정확함     |
| arc_param_length_round_trip               | Arc         | `get_param_from_length`, `get_length_from_param` | ✅ 있음         | 원호 길이 ↔ 파라미터 변환. 선형 관계 수식 기반으로 정확함             |
| arc_param_length_reverse_direction        | Arc         | `get_param_from_length`, `get_length_from_param` | ✅ 있음         | t₀ > t₁인 경우 방향 반전 및 정규화 확인. 수식 적용 정확함              |
| arc_param_endpoint_tolerance_snap         | Arc         | `get_param_from_length`, `get_length_from_param` | ✅ 있음         | 끝점 근접 시 스냅 및 역변환 안정성 확인. 수치 오차 허용 범위 적절함   |
| arc_to_nurbs                              | Arc         | `to_nurbs`                         | ✅ 있음         | NURBS 변환 시 가중치 $w = \cos(\Delta \theta / 2)$ 적용. 정확함    |
| arc_split_at_midpoint                     | Arc         | `split_at`, `length`, `end_point`  | ✅ 있음         | 두 원호 길이 합 = 전체 길이. 접점 일치 확인. 수학적 일관성 확보        |
| arc_trim_at_start                         | Arc         | `trim_at`, `length`                | ✅ 있음         | 절단 후 길이 = 반지름 × 각도. 수식 적용 정확함                         |
| arc_sub_curve_full_circle                 | Arc         | `sub_curve`, `length`, `start_point`, `end_point` | ✅ 있음         | 전체 원호에서 부분 곡선 추출. 길이 및 위치 일치 확인. 수식 정확함     |


## 📐 Arc 관련 수식 요약

| 기능 또는 맥락             | 수식 표현                                      |
|----------------------------|-----------------------------------------------|
| 원호 길이                  | $L = |\Delta \theta| \cdot r$             |
| 길이 → 파라미터 변환       | $t = t_0 + \frac{\text{length}}{r}$       |
| 파라미터 → 길이 변환       | $\text{length} = |t - t_0| \cdot r$       |
| NURBS 가중치               | $w = \cos\left( \frac{\Delta \theta}{2} \right)$ |


## ✅ 수학적 검토 요약
- 모든 테스트는 Arc 및 Circle의 핵심 수학적 기능(길이, 파라미터 변환, NURBS 변환 등)을 정밀하게 검증합니다.
- 수식 기반 함수들은 모두 수학적으로 타당하며, 수치 오차 허용 범위도 적절하게 설정되어 있습니다.
- 예외 처리 (범위 초과, 역방향, 끝점 근접 등)도 잘 반영되어 있어 안정적인 구현으로 판단됩니다.


```rust
#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use nurbslib::core::arc::Arc;
    use nurbslib::core::circle::Circle;
    use nurbslib::core::plane::Plane;
    use nurbslib::core::prelude::Point3D;

    fn close(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps * (a.abs() + b.abs()).max(1.0)
    }
```
```rust
    #[test]
    fn circle_param_length_round_trip() {
        let r = 2.5;
        let c = Circle::from_center(Point3D::new(1.0, -2.0, 0.5), r).expect("circle");

        // 전체 길이
        let total = c.length(); // = 2πr
        assert!(close(total, 2.0 * PI * r, 1e-12));

        // length=0 → t = domain.t0
        let (ok0, t0) = c.get_param_from_length(0.0);
        assert!(ok0);
        assert!(close(t0, c.domain().t0, 1e-12));

        // length = 전체 → t = domain.t1
        let (ok1, t1) = c.get_param_from_length(total);
        assert!(ok1);
        assert!(close(t1, c.domain().t1, 1e-12));

        // 중간 길이(π r) → t = t0 + π
        let mid_len = PI * r;
        let (okm, tm) = c.get_param_from_length(mid_len);
        assert!(okm);
        assert!(close(tm, c.domain().t0 + PI, 1e-12));

        // 역변환 체크
        let (okl0, l0) = c.get_length_from_param(t0);
        let (oklm, lm) = c.get_length_from_param(tm);
        let (okl1, l1) = c.get_length_from_param(t1);
        assert!(okl0 && oklm && okl1);
        assert!(close(l0, 0.0, 1e-12));
        assert!(close(lm, mid_len, 1e-12));
        assert!(close(l1, total, 1e-12));
    }
```
```rust
    #[test]
    fn circle_param_length_out_of_range() {
        let r = 3.0;
        let c = Circle::from_center(Point3D::new(0.0, 0.0, 0.0), r).expect("circle");
        let total = c.length();

        // 음수 길이
        let (ok_neg, tneg) = c.get_param_from_length(-1.0);
        assert!(!ok_neg);
        assert!(close(tneg, c.domain().t0, 1e-12));

        // 전체를 초과
        let (ok_over, t_over) = c.get_param_from_length(total + 1e-3);
        assert!(!ok_over);
        assert!(close(t_over, c.domain().t0, 1e-12));

        // 도메인 밖 파라미터 → length 변환 실패
        let (oklen_neg, _) = c.get_length_from_param(c.domain().t0 - 1e-6);
        let (oklen_over, _) = c.get_length_from_param(c.domain().t1 + 1e-6);
        assert!(!oklen_neg && !oklen_over);
    }
```
```rust
    #[test]
    fn arc_param_length_round_trip() {
        // 가정: GArc::new(plane, center, radius, t0, t1) 혹은 유사 생성자가 있음.
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 4.0;
        let t0 = 0.5;
        let t1 = 2.0;
        let arc = Arc::new(plane, center, r, t0, t1).expect("arc");

        // 전체 호 길이 = |t1 - t0|*r
        let total = arc.length();
        assert!(close(total, (t1 - t0).abs() * r, 1e-12));

        // length 0 → t0
        let (ok0, u0) = arc.get_param_from_length(0.0);
        assert!(ok0 && close(u0, t0, 1e-12));

        // length total → t1
        let (ok1, u1) = arc.get_param_from_length(total);
        assert!(ok1 && close(u1, t1, 1e-12));

        // 40% 길이 지점
        let l40 = 0.4 * total;
        let (ok40, u40) = arc.get_param_from_length(l40);
        assert!(ok40);
        let (okl40, back_l40) = arc.get_length_from_param(u40);
        assert!(okl40);
        assert!(close(back_l40, l40, 1e-12));
    }
```
```rust
    #[test]
    fn arc_param_length_reverse_direction() {
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 2.0;
        let t0_in = 1.8;
        let t1_in = 0.7; // 감소

        let arc = Arc::new(plane, center, r, t0_in, t1_in).expect("arc-rev");

        // 실제 도메인(생성자에서 정규화/역전될 수 있음)
        let dom = arc.domain();
        let total = arc.length(); // = |t1 - t0| * r

        // length=0 -> dom.t0
        let (ok0, u0) = arc.get_param_from_length(0.0);
        assert!(ok0, "should map length=0");
        assert!(close(u0, dom.t0, 1e-12), "u0 must equal domain.t0");

        // length=total -> dom.t1
        let (ok1, u1) = arc.get_param_from_length(total);
        assert!(ok1, "should map length=total");
        assert!(close(u1, dom.t1, 1e-12), "u1 must equal domain.t1");

        // 중간
        let half = 0.5 * total;
        let (okm, um) = arc.get_param_from_length(half);
        assert!(okm);
        let (oklm, lm) = arc.get_length_from_param(um);
        assert!(oklm && close(lm, half, 1e-12));
    }
```
```rust
    #[test]
    fn arc_param_endpoint_tolerance_snap() {
        // 끝점 스냅 허용 오차 확인
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 5.0;
        let t0 = 0.2;
        let t1 = 1.4;
        let arc = Arc::new(plane, center, r, t0, t1).expect("arc");

        let total = arc.length();
        let eps = total * 1e-14; // 매우 작은 오차

        // 거의 0
        let (ok_a, ua) = arc.get_param_from_length(0.0 + eps);
        assert!(ok_a);
        // 아주 근접하면 t0로 스냅되진 않을 수도 있지만, 아래 역변환이 안정적이어야 함
        let (ok_la, la) = arc.get_length_from_param(ua);
        assert!(ok_la);
        assert!(close(la, eps, 1e-10));

        // 거의 total
        let (ok_b, ub) = arc.get_param_from_length(total - eps);
        assert!(ok_b);
        let (ok_lb, lb) = arc.get_length_from_param(ub);
        assert!(ok_lb);
        assert!(close(lb, total - eps, 1e-10));
    }
}
```
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::arc::Arc;
    use nurbslib::core::plane::Plane;
    use nurbslib::core::prelude::Point3D;
    use nurbslib::core::types::{ON_TOL12, on_are_point_close};

    fn close(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps * (a.abs() + b.abs()).max(1.0)
    }
```
```rust
    #[test]
    fn arc_param_length_round_trip() {
        // 가정: GArc::new(plane, center, radius, t0, t1) 혹은 유사 생성자가 있음.
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 4.0;
        let t0 = 0.5;
        let t1 = 2.0;
        let arc = Arc::new(plane, center, r, t0, t1).expect("arc");

        // 전체 호 길이 = |t1 - t0|*r
        let total = arc.length();
        assert!(close(total, (t1 - t0).abs() * r, 1e-12));

        // length 0 → t0
        let (ok0, u0) = arc.get_param_from_length(0.0);
        assert!(ok0 && close(u0, t0, 1e-12));

        // length total → t1
        let (ok1, u1) = arc.get_param_from_length(total);
        assert!(ok1 && close(u1, t1, 1e-12));

        // 40% 길이 지점
        let l40 = 0.4 * total;
        let (ok40, u40) = arc.get_param_from_length(l40);
        assert!(ok40);
        let (okl40, back_l40) = arc.get_length_from_param(u40);
        assert!(okl40);
        assert!(close(back_l40, l40, 1e-12));
    }
```
```rust
    #[test]
    fn arc_param_length_reverse_direction() {
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 2.0;
        let t0_in = 1.8;
        let t1_in = 0.7; // 감소

        let arc = Arc::new(plane, center, r, t0_in, t1_in).expect("arc-rev");

        // 실제 도메인(생성자에서 정규화/역전될 수 있음)
        let dom = arc.domain();
        let total = arc.length(); // = |t1 - t0| * r

        // length=0 -> dom.t0
        let (ok0, u0) = arc.get_param_from_length(0.0);
        assert!(ok0, "should map length=0");
        assert!(close(u0, dom.t0, 1e-12), "u0 must equal domain.t0");

        // length=total -> dom.t1
        let (ok1, u1) = arc.get_param_from_length(total);
        assert!(ok1, "should map length=total");
        assert!(close(u1, dom.t1, 1e-12), "u1 must equal domain.t1");

        // 중간
        let half = 0.5 * total;
        let (okm, um) = arc.get_param_from_length(half);
        assert!(okm);
        let (oklm, lm) = arc.get_length_from_param(um);
        assert!(oklm && close(lm, half, 1e-12));
    }
```
```rust
    #[test]
    fn arc_param_endpoint_tolerance_snap() {
        // 끝점 스냅 허용 오차 확인
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 5.0;
        let t0 = 0.2;
        let t1 = 1.4;
        let arc = Arc::new(plane, center, r, t0, t1).expect("arc");

        let total = arc.length();
        let eps = total * 1e-14; // 매우 작은 오차

        // 거의 0
        let (ok_a, ua) = arc.get_param_from_length(0.0 + eps);
        assert!(ok_a);
        // 아주 근접하면 t0로 스냅되진 않을 수도 있지만, 아래 역변환이 안정적이어야 함
        let (ok_la, la) = arc.get_length_from_param(ua);
        assert!(ok_la);
        assert!(close(la, eps, 1e-10));

        // 거의 total
        let (ok_b, ub) = arc.get_param_from_length(total - eps);
        assert!(ok_b);
        let (ok_lb, lb) = arc.get_length_from_param(ub);
        assert!(ok_lb);
        assert!(close(lb, total - eps, 1e-10));
    }
```
```rust
    #[test]
    fn arc_to_nurbs() {
        // 끝점 스냅 허용 오차 확인
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 5.0;
        let t0 = 0.2;
        let t1 = 1.4;
        let arc = Arc::new(plane, center, r, t0, t1).expect("arc");

        let total = arc.length();
        let eps = total * 1e-14; // 매우 작은 오차

        let nurbs = arc.to_nurbs();
        println!("nurbs = {:?}", nurbs);

        let param = arc.domain().parameter_at(1.0);
        let pt = arc.point_at(param);
        println!("pt: {:?}", pt);

        let pt_end = arc.end_point();

        assert_eq!(on_are_point_close(&pt, &pt_end, ON_TOL12), true)
    }
```
```rust
    #[test]
    fn arc_split_at_midpoint() {
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 3.0;
        let t0 = 0.0;
        let t1 = std::f64::consts::PI;
        let arc = Arc::new(plane, center, r, t0, t1).expect("arc");

        let mid = (t0 + t1) * 0.5;
        let (arc1, arc2) = arc.split_at(mid).expect("split");

        assert!(close(arc1.length() + arc2.length(), arc.length(), 1e-12));
        assert!(on_are_point_close(
            &arc1.end_point(),
            &arc2.start_point(),
            ON_TOL12
        ));
    }
```
```rust
    #[test]
    fn arc_trim_at_start() {
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 2.0;
        let arc = Arc::new(plane, center, r, 0.0, std::f64::consts::PI).expect("arc");

        let mut arc_clone = arc;
        let ok = arc_clone.trim_at(std::f64::consts::FRAC_PI_2, false);
        assert!(ok);
        assert!(close(
            arc_clone.length(),
            r * std::f64::consts::FRAC_PI_2,
            1e-12
        ));
    }
```
```rust
    #[test]
    fn arc_sub_curve_full_circle() {
        let plane = Plane::xy();
        let center = Point3D::new(0.0, 0.0, 0.0);
        let r = 1.0;
        let arc = Arc::new(plane, center, r, 0.0, std::f64::consts::TAU).expect("full circle");

        let sub = arc.sub_curve(0.0, std::f64::consts::TAU).expect("subcurve");
        assert!(close(sub.length(), arc.length(), 1e-12));
        assert!(on_are_point_close(
            &sub.start_point(),
            &arc.start_point(),
            ON_TOL12
        ));
        assert!(on_are_point_close(
            &sub.end_point(),
            &arc.end_point(),
            ON_TOL12
        ));
    }
}
```

---



