# Hermite Curve Domain
- Hermite Curve(에르미트 곡선)의 도메인(domain)은 기본적으로 $[0, 1]$ 을 쓰는 것이 정석.
- 즉, $u ∈ [0, 1]$ 로 두면 됩니다.

## 🎯 왜 Hermite Curve는 [0,1]을 쓰는가?
- Hermite 곡선은 다음 네 가지 데이터를 기반으로 정의됩니다:
  - 시작점 P_0
  - 끝점 P_1
  - 시작 접선(미분) T_0
  - 끝 접선(미분) T_1
- 그리고 곡선은 다음과 같은 Hermite basis로 표현됩니다:

```math
C(u)=(2u^3-3u^2+1)P_0+(u^3-2u^2+u)T_0+(-2u^3+3u^2)P_1+(u^3-u^2)T_1
```

- 여기서 u는 반드시 0~1 사이로 정의됩니다.
  - $u=0 → C(0)=P_0$
  - $u=1 → C(1)=P_1$
- 즉, Hermite basis 자체가 [0,1] 구간을 기준으로 만들어진 함수라서  
  다른 구간을 쓰면 수식이 깨짐.

## 📌 만약 도메인을 [a, b]로 쓰고 싶다면?
- 가능하지만, 반드시 리매핑해야 합니다.

$$
u=\frac{t-a}{b-a}
$$

- 이렇게 변환해서 Hermite basis에 넣어야 합니다.
- 즉:
  - 외부 파라미터 $t\in [a,b]$
  - Hermite 내부 파라미터 $u\in [0,1]$

- ✔ 결론
  - HermiteCurve는 domain을 $[0,1]$로 주는 것이 맞습니다.
  - 만약 다른 구간을 쓰고 싶다면 반드시 $[0,1]$ 로 변환해서 사용해야 함.
 
---

## 소스 코드
```rust
use crate::core::bezier_curve::BezierCurve;
use crate::core::boundingbox::BoundingBox;
use crate::core::domain::Interval;
use crate::core::geom::{Point2D, Point4D, Vector2D};
use crate::core::maths::{on_dot_pt, on_dot_vec};
use crate::core::prelude::{NurbsCurve, Point3D, Real, Vector3D};
use crate::core::types::ON_TOL12;
use crate::topology::geom_kernel::CurveGeom;
```
```rust
#[derive(Clone, Debug)]
pub struct HermiteCurve {
    // P(t) = A + t(B + t(C + tD)), A=P1, B=D1
    p1: Point3D,
    d1: Vector3D,
    p2: Point3D,
    d2: Vector3D,
    c: Vector3D,
    d: Vector3D,
    dim: usize, // 2 or 3 (정보용)
}
```
```rust
impl CurveGeom for HermiteCurve {
    fn domain(&self) -> Interval {
        Interval { t0: 0.0, t1: 1.0 }
    }

    fn eval_point(&self, t: Real) -> Point3D {
        self.point_at(t)
    }

    fn eval_tangent(&self, t: Real) -> Option<Vector3D> {
        Some(self.tangent_at(t))
    }

    fn point_at_start(&self) -> Point3D {
        self.point_at(0.0)
    }

    fn point_at_end(&self) -> Point3D {
        self.point_at(1.0)
    }

    fn tangent_at_start(&self) -> Vector3D {
        self.tangent_at(0.0)
    }

    fn tangent_at_end(&self) -> Vector3D {
        self.tangent_at(1.0)
    }
}
```
```rust
impl HermiteCurve {
    pub fn new_3d(p1: Point3D, v1: Vector3D, p2: Point3D, v2: Vector3D) -> Self {
        let c = Vector3D {
            x: -3.0 * p1.x - 2.0 * v1.x + 3.0 * p2.x - v2.x,
            y: -3.0 * p1.y - 2.0 * v1.y + 3.0 * p2.y - v2.y,
            z: -3.0 * p1.z - 2.0 * v1.z + 3.0 * p2.z - v2.z,
        };
        let d = Vector3D {
            x: 2.0 * p1.x + v1.x - 2.0 * p2.x + v2.x,
            y: 2.0 * p1.y + v1.y - 2.0 * p2.y + v2.y,
            z: 2.0 * p1.z + v1.z - 2.0 * p2.z + v2.z,
        };
        Self {
            p1,
            d1: v1,
            p2,
            d2: v2,
            c,
            d,
            dim: 3,
        }
    }
```
```rust
    pub fn new_2d(p1: Point3D, v1: Vector2D, p2: Point3D, v2: Vector2D) -> Self {
        let p1_3 = Point3D {
            x: p1.x,
            y: p1.y,
            z: 0.0,
        };
        let p2_3 = Point3D {
            x: p2.x,
            y: p2.y,
            z: 0.0,
        };
        let v1_3 = Vector3D {
            x: v1.x,
            y: v1.y,
            z: 0.0,
        };
        let v2_3 = Vector3D {
            x: v2.x,
            y: v2.y,
            z: 0.0,
        };
        let mut h = Self::new_3d(p1_3, v1_3, p2_3, v2_3);
        h.dim = 2;
        h
    }
```
```rust
    pub fn offset_curve(
        dim: usize,
        curve: &HermiteCurve,
        pln_norm: &Vector3D,
        offset: f64,
    ) -> Self {
        let mut new_curve = curve.clone();
        new_curve.dim = dim;

        // Convert to Bezier form
        let s_p0 = curve.p1;
        let s_p1 = curve.p1 + (curve.d1 / 3.0).to_point();
        let s_p2 = curve.p2 - (curve.d2 / 3.0).to_point();
        let s_p3 = curve.p2;

        // Compute a
        let a0 = s_p1 - s_p0;
        let a1 = s_p2 - s_p1;
        let a2 = s_p3 - s_p2;
        let a3 = s_p3 - s_p0;

        if a0.length_squared() < ON_TOL12 {
            return new_curve;
        };
        if a2.length_squared() < ON_TOL12 {
            return new_curve;
        };

        // Compute a0 Transpose and a2 Transpose
        let a0t = a0.cross_vec(pln_norm).unitize();
        let a2t = a2.cross_vec(pln_norm).unitize();
        if a0t.length_squared() < ON_TOL12 {
            return new_curve;
        };
        if a2t.length() < ON_TOL12 {
            return new_curve;
        };

        // Test for first case where all points are on same line (relative to offset plane
        // projection.
        let dist = offset;
        let s_q0 = s_p0 + (dist * a0t).to_point();
        let s_q3 = s_p3 + (dist * a2t).to_point();
        let s_q1: Point3D;
        let s_q2: Point3D;
        if on_dot_vec(&a1.to_vector(), &a0t).abs() < ON_TOL12
            && on_dot_vec(&a2.to_vector(), &a0t).abs() < ON_TOL12
        {
            // Have straight line.
            s_q1 = s_p1 + (dist * a0t).to_point();
            s_q2 = s_p2 + (dist * a2t).to_point();
        } else if on_dot_vec(&a2.to_vector(), &a0t).abs() < ON_TOL12 {
            // Have case where end edges of control polygon are parallel
            s_q1 = s_p1
                + (dist * a0t).to_point()
                + (8.0 * dist / 3.0) * a0 / (a0.length() + a2.length());
            s_q2 = s_p2 + (dist * a2t).to_point()
                - (8.0 * dist / 3.0) * a2 / (a0.length() + a2.length());
        } else {
            // Have standard Bezier offset case
            // Compute vec
            let a1a3 = a1 + a3;
            if a1a3.length_squared() < ON_TOL12 {
                return new_curve;
            }

            let vec = 2.0 * (a1 + a3) / a1a3.length() - a0 / a0.length() - a2 / a2.length();
            s_q1 = s_p1
                + (dist * a0t).to_point()
                + (4.0 * dist / 3.0)
                    * ((on_dot_pt(&vec, &a2)) / (on_dot_vec(&a0.to_vector(), &(a2t * a2.length()))))
                    * a0;
            s_q2 = s_p2
                + (dist * a2t).to_point()
                + (4.0 * dist / 3.0)
                    * ((on_dot_pt(&vec, &a0)) / (on_dot_vec(&a2.to_vector(), &(a0t * a0.length()))))
                    * a2;
        }

        let p1 = s_q0;
        let d1 : Point3D = 3.0 * (s_q1 - s_q0);
        let p2 = s_q3;
        let d2 : Point3D = 3.0 * (s_q3 - s_q2);
        let c : Point3D = -3.0 * p1 - 2.0 * d1 + 3.0 * p2 - d2;
        let d : Point3D = 2.0 * p1 + d1 - 2.0 * p2 + d2;

        new_curve.p1 = p1;
        new_curve.p2 = p2;
        new_curve.d1 = d1.to_vector();
        new_curve.d2 = d2.to_vector();
        new_curve.c = c.to_vector();
        new_curve.d = d.to_vector();

        new_curve
    }
```
```rust
    /// 베지어 4개 컨트롤 포인트 반환 (P1, P1 + D1/3, P2 - D2/3, P2)
    pub fn bezier_points(&self) -> (Point3D, Point3D, Point3D, Point3D) {
        let p1 = self.p1;
        let p2 = Point3D {
            x: self.p1.x + self.d1.x / 3.0,
            y: self.p1.y + self.d1.y / 3.0,
            z: self.p1.z + self.d1.z / 3.0,
        };
        let p3 = Point3D {
            x: self.p2.x - self.d2.x / 3.0,
            y: self.p2.y - self.d2.y / 3.0,
            z: self.p2.z - self.d2.z / 3.0,
        };
        let p4 = self.p2;
        (p1, p2, p3, p4)
    }
```
```rust
    /// 위치/도함수 평가. l_num_derivatives: 0..=3 지원
    pub fn evaluate(&self, t: f64, l_num_derivatives: usize) -> [Point3D; 4] {
        let u = t;
        let a = self.p1;
        let b = self.d1;
        let c = self.c;
        let d = self.d;

        // P
        let p = Point3D {
            x: a.x + u * (b.x + u * (c.x + u * d.x)),
            y: a.y + u * (b.y + u * (c.y + u * d.y)),
            z: a.z + u * (b.z + u * (c.z + u * d.z)),
        };

        // 1st
        let dp = if l_num_derivatives >= 1 {
            Point3D {
                x: b.x + u * (2.0 * c.x + 3.0 * u * d.x),
                y: b.y + u * (2.0 * c.y + 3.0 * u * d.y),
                z: b.z + u * (2.0 * c.z + 3.0 * u * d.z),
            }
        } else {
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        };

        // 2nd
        let ddp = if l_num_derivatives >= 2 {
            Point3D {
                x: 2.0 * c.x + 6.0 * u * d.x,
                y: 2.0 * c.y + 6.0 * u * d.y,
                z: 2.0 * c.z + 6.0 * u * d.z,
            }
        } else {
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        };

        // 3rd
        let ddd_p = if l_num_derivatives >= 3 {
            Point3D {
                x: 6.0 * d.x,
                y: 6.0 * d.y,
                z: 6.0 * d.z,
            }
        } else {
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        };

        [p, dp, ddp, ddd_p]
    }
```
```rust
    #[inline]
    pub fn point_at(&self, t: f64) -> Point3D {
        self.evaluate(t, 0)[0]
    }
```
```rust
    pub fn tangent_at(&self, t: f64) -> Vector3D {
        self.evaluate(t, 0)[1].to_vector()
    }
```
```rust
    pub fn nth_derivative(&self, t: f64, n: usize) -> Option<Vector3D> {
        match n {
            0 => Some(self.point_at(t).to_vector()),
            1 => Some((self.evaluate(t, 1)[1] - Point3D::origin()).to_vector()),
            2 => Some((self.evaluate(t, 2)[2] - Point3D::origin()).to_vector()),
            3 => Some((self.evaluate(t, 3)[3] - Point3D::origin()).to_vector()),
            _ => None, // 4차 이상은 0
        }
    }
```
```rust
    #[allow(unused)]
    fn control_bounding_box(&self) -> BoundingBox {
        let mut bb = BoundingBox::default();
        let pts = [
            &self.p1,
            &(self.p1 + (self.d1 / 3.0).to_point()),
            &(self.p2 - (self.d2 / 3.0).to_point()),
            &self.p2,
        ];
        for pt in pts {
            bb.grow_point3d(pt);
        }
        bb
    }
```
```rust
    fn to_bezier(&self) -> Option<BezierCurve> {
        if !self.is_valid() {
            return None;
        };

        let pt1 = &self.p1;
        let pt2 = self.p1 + (self.d1 / 3.0).to_point();
        let pt3 = self.p2 - (self.d2 / 3.0).to_point();
        let pt4 = &self.p2;

        let pt_4d1 = Point4D::homogeneous(pt1.x, pt1.y, pt1.z, 1.0);
        let pt_4d2 = Point4D::homogeneous(pt2.x, pt2.y, pt2.z, 1.0);
        let pt_4d3 = Point4D::homogeneous(pt3.x, pt3.y, pt3.z, 1.0);
        let pt_4d4 = Point4D::homogeneous(pt4.x, pt4.y, pt4.z, 1.0);

        Some(BezierCurve::new(vec![pt_4d1, pt_4d2, pt_4d3, pt_4d4]))
    }
```
```rust
    pub fn to_nurbs(&self) -> Option<NurbsCurve> {
        if !self.is_valid() {
            return None;
        };

        if let Some(curve) = self.to_bezier() {
            return Some(curve.to_nurbs());
        }
        None
    }
}
```
```rust
impl HermiteCurve {
    pub fn is_valid(&self) -> bool {
        if !self.p1.is_valid() || !self.p2.is_valid() {
            return false;
        }

        if !self.d1.is_valid() || !self.d2.is_valid() {
            return false;
        }

        if !self.c.is_valid() || !self.d.is_valid() {}
        self.dim == 2 || self.dim == 3
    }
}
```
```rust
pub fn on_hermite_spline_2d(
    p0: Point2D,
    t0: Vector2D,
    p1: Point2D,
    t1: Vector2D,
    t: f64,
) -> Point2D {
    let t2 = t * t;
    let t3 = t2 * t;

    // Hermite basis
    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    let x = h00 * p0.x + h10 * t0.x + h01 * p1.x + h11 * t1.x;
    let y = h00 * p0.y + h10 * t0.y + h01 * p1.y + h11 * t1.y;

    Point2D { x, y }
}
```
```rust
pub fn on_hermite_color_rgb(c1: (u8, u8, u8), c2: (u8, u8, u8), t: f64) -> (u8, u8, u8) {
    // p0=(0,0), p1=(1,1), t0=(1,0), t1=(1,0) → y(t)= -2t^3 + 3t^2
    let p0 = Point2D { x: 0.0, y: 0.0 };
    let p1 = Point2D { x: 1.0, y: 1.0 };
    let v0 = Vector2D { x: 1.0, y: 0.0 };
    let v1 = Vector2D { x: 1.0, y: 0.0 };
    let h = on_hermite_spline_2d(p0, v0, p1, v1, t).y.clamp(0.0, 1.0);

    let r = on_lerp_i32(h, c1.0 as i32, c2.0 as i32).clamp(0, 255) as u8;
    let g = on_lerp_i32(h, c1.1 as i32, c2.1 as i32).clamp(0, 255) as u8;
    let b = on_lerp_i32(h, c1.2 as i32, c2.2 as i32).clamp(0, 255) as u8;
    (r, g, b)
}
```
```rust
#[inline]
pub fn on_lerp_f64(t: f64, x: f64, y: f64) -> f64 {
    // If x == y, return x as-is (even if t is NaN)

    if x == y && t == x {
        return x;
    }
    let mut z = (1.0 - t) * x + t * y;

    // If x and y are not NaN and t is within [0, 1], clamp t to that range
    if x < y {
        if z < x && t >= 0.0 {
            z = x;
        } else if z > y && t <= 1.0 {
            z = y;
        }
    } else if x > y {
        if z < y && t >= 0.0 {
            z = y;
        } else if z > x && t <= 1.0 {
            z = x;
        }
    }
    z
}
```
```rust
#[inline]
pub fn on_lerp_i32(t: f64, a: i32, b: i32) -> i32 {
    let z = on_lerp_f64(t, a as f64, b as f64);
    z.round() as i32
}
```
```rust
pub fn on_hermite_spline_3d(
    p0: Point3D,
    t0: Vector3D,
    p1: Point3D,
    t1: Vector3D,
    t: Real,
) -> Point3D {
    let curve = HermiteCurve::new_3d(p0, t0, p1, t1);
    if !curve.is_valid() {
        return Point3D::nan();
    }

    let pt = curve.evaluate(t, 0);
    pt[0]
}
```
---
