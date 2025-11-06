# Hermite Curve
Hermite Curve는 두 점과 각 점에서의 접선 벡터를 기반으로 정의되는 **보간 곡선(interpolating curve)** 입니다.  
주어진 시작점과 끝점, 그리고 각 점에서의 방향(접선)을 기반으로 곡선을 생성하며,  
C¹ 연속성을 보장합니다.

### 수식
일반적인 형태
Hermite 곡선은 다음과 같은 3차 다항식으로 표현됩니다:
```
P(t) = a·t³ + b·t² + c·t + d
```
- 여기서 `a`, `b`, `c`, `d` 는 벡터 계수이며,


### 경계 조건
다음의 경계 조건을 만족해야 합니다:
```
- P(0) = P₁ (시작점)
- P(1) = P₂ (끝점)
- P′(0) = D₁ (시작점에서의 접선)
- P′(1) = D₂ (끝점에서의 접선
```

### 기저 함수

Hermite 곡선은 다음의 4개 기저 함수로 구성됩니다:
- h₁(t) = 2t³ − 3t² + 1
- h₂(t) = −2t³ + 3t²
- h₃(t) = t³ − 2t² + t
- h₄(t) = t³ − t²

### 행렬 표현
```
P(t) = [t³ t² t 1] · M_H · [P₁ P₂ D₁ D₂]ᵀ
```

$P(t) = h_1(t) P_1 + h_2(t) P_2 + h_3(t) D_1 + h_4(t) D_2$



#### Bezier 변환
Hermite 곡선은 다음의 4개 Bezier 컨트롤 포인트로 변환 가능합니다:
```
(P₁, P₁ + D₁/3, P₂ − D₂/3, P₂)
```
#### Rust 구현
```
P(t) = A + t(B + t(C + tD))
```
- A = P₁ (시작점)
- B = D₁ (시작점 접선)
- C, D = 곡선 형상을 위한 보정 벡터 (P₂, D₂ 기반으로 계산됨)


이 방식은 Hermite 곡선을 다항식 계수 기반으로 직접 구성하는 방식이며,
행렬 곱 없이 빠르게 평가할 수 있는 장점이 있습니다.



## 소스 코드
```rust
#[derive(Clone, Debug)]
pub struct HermiteCurve {
    // P(t) = A + t(B + t(C + tD)), A=P1, B=D1
    p1: Point,
    d1: Vector,
    p2: Point,
    d2: Vector,
    c:  Vector,
    d:  Vector,
    dim: usize, // 2 or 3 (정보용)
}

impl HermiteCurve {
    pub fn new_3d(p1: Point, v1: Vector, p2: Point, v2: Vector) -> Self {
        let c = Vector {
            x: -3.0*p1.x - 2.0*v1.x + 3.0*p2.x - v2.x,
            y: -3.0*p1.y - 2.0*v1.y + 3.0*p2.y - v2.y,
            z: -3.0*p1.z - 2.0*v1.z + 3.0*p2.z - v2.z,
        };
        let d = Vector {
            x:  2.0*p1.x + v1.x - 2.0*p2.x + v2.x,
            y:  2.0*p1.y + v1.y - 2.0*p2.y + v2.y,
            z:  2.0*p1.z + v1.z - 2.0*p2.z + v2.z,
        };
        Self { p1, d1: v1, p2, d2: v2, c, d, dim: 3 }
    }

    pub fn new_2d(p1: Point2D, v1: Vector2D, p2: Point2D, v2: Vector2D) -> Self {
        let p1_3 = Point { x: p1.x, y: p1.y, z: 0.0 };
        let p2_3 = Point { x: p2.x, y: p2.y, z: 0.0 };
        let v1_3 = Vector { x: v1.x, y: v1.y, z: 0.0 };
        let v2_3 = Vector { x: v2.x, y: v2.y, z: 0.0 };
        let mut h = Self::new_3d(p1_3, v1_3, p2_3, v2_3);
        h.dim = 2;
        h
    }

    #[inline] pub fn is_valid(&self) -> bool {
        (self.dim == 2 || self.dim == 3)
    }

    /// 베지어 4개 컨트롤 포인트 반환 (P1, P1 + D1/3, P2 - D2/3, P2)
    pub fn bezier_points(&self) -> (Point, Point, Point, Point) {
        let p1 = self.p1;
        let p2 = Point {
            x: self.p1.x + self.d1.x / 3.0,
            y: self.p1.y + self.d1.y / 3.0,
            z: self.p1.z + self.d1.z / 3.0,
        };
        let p3 = Point {
            x: self.p2.x - self.d2.x / 3.0,
            y: self.p2.y - self.d2.y / 3.0,
            z: self.p2.z - self.d2.z / 3.0,
        };
        let p4 = self.p2;
        (p1, p2, p3, p4)
    }

    /// 위치/도함수 평가. l_num_derivatives: 0..=3 지원
    pub fn evaluate(&self, t: f64, l_num_derivatives: usize) -> [Point; 4] {
        let u  = t;
        let a  = self.p1;
        let b  = self.d1;
        let c  = self.c;
        let d  = self.d;

        // P
        let p = Point {
            x: a.x + u*(b.x + u*(c.x + u*d.x)),
            y: a.y + u*(b.y + u*(c.y + u*d.y)),
            z: a.z + u*(b.z + u*(c.z + u*d.z)),
        };

        // 1st
        let dp = if l_num_derivatives >= 1 {
            Point {
                x: b.x + u*(2.0*c.x + 3.0*u*d.x),
                y: b.y + u*(2.0*c.y + 3.0*u*d.y),
                z: b.z + u*(2.0*c.z + 3.0*u*d.z),
            }
        } else { Point { x:0.0,y:0.0,z:0.0 } };

        // 2nd
        let ddp = if l_num_derivatives >= 2 {
            Point {
                x: 2.0*c.x + 6.0*u*d.x,
                y: 2.0*c.y + 6.0*u*d.y,
                z: 2.0*c.z + 6.0*u*d.z,
            }
        } else { Point { x:0.0,y:0.0,z:0.0 } };

        // 3rd
        let dddp = if l_num_derivatives >= 3 {
            Point { x: 6.0*d.x, y: 6.0*d.y, z: 6.0*d.z }
        } else { Point { x:0.0,y:0.0,z:0.0 } };

        [p, dp, ddp, dddp]
    }

    #[inline]
    pub fn evaluate_point(&self, t: f64) -> Point {
        self.evaluate(t, 0)[0]
    }
}

```
# 기능 추가
## offset_curve
```rust
pub fn offset_curve(
    dim: usize,
    curve: &HermiteCurve,
    pln_norm: &Vector,
    offset: f64,
) -> Self {
    let mut new_curve = curve.clone();
    new_curve.dim = dim;

    // Convert to Bezier form
    let s_p0 = curve.p1;
    let s_p1 = curve.p1 + curve.d1 / 3.0;
    let s_p2 = curve.p2 - curve.d2 / 3.0;
    let s_p3 = curve.p2;

    // Compute a
    let a0 = s_p1 - s_p0;
    let a1 = s_p2 - s_p1;
    let a2 = s_p3 - s_p2;
    let a3 = s_p3 - s_p0;

    if a0.length_squared() < ON_EPSILON {
        return new_curve;
    };
    if a2.length_squared() < ON_EPSILON {
        return new_curve;
    };

    // Compute a0 Transpose and a2 Transpose
    let a0t = a0.cross(pln_norm).unitize();
    let a2t = a2.cross(pln_norm).unitize();
    if a0t.length_squared() < ON_EPSILON {
        return new_curve;
    };
    if a2t.length() < ON_EPSILON {
        return new_curve;
    };

    // Test for first case where all points are on same line (relative to offset plane
    // projection.
    let dist = offset;
    let s_q0 = s_p0 + dist * a0t;
    let s_q3 = s_p3 + dist * a2t;
    let s_q1: Point;
    let s_q2: Point;
    if dot(&a1, &a0t).abs() < ON_EPSILON && dot(&a2, &a0t).abs() < ON_EPSILON {
        // Have straight line.
        s_q1 = s_p1 + dist * a0t;
        s_q2 = s_p2 + dist * a2t;
    } else if dot(&a2, &a0t).abs() < ON_EPSILON {
        // Have case where end edges of control polygon are parallel
        s_q1 = s_p1 + dist * a0t + (8.0 * dist / 3.0) * a0 / (a0.length() + a2.length());
        s_q2 = s_p2 + dist * a2t - (8.0 * dist / 3.0) * a2 / (a0.length() + a2.length());
    } else {
        // Have standard Bezier offset case
        // Compute vec
        let a1a3 = a1 + a3;
        if a1a3.length_squared() < ON_EPSILON {
            return new_curve;
        }

        let vec = 2.0 * (a1 + a3) / a1a3.length() - a0 / a0.length() - a2 / a2.length();
        s_q1 = s_p1
            + dist * a0t
            + (4.0 * dist / 3.0) * ((dot(&vec, &a2)) / (dot(&a0, &(a2t * a2.length())))) * a0;
        s_q2 = s_p2
            + dist * a2t
            + (4.0 * dist / 3.0) * ((dot(&vec, &a0)) / (dot(&a2, &(a0t * a0.length())))) * a2;
    }

    let p1 = s_q0;
    let d1 = 3.0 * (s_q1 - s_q0);
    let p2 = s_q3;
    let d2 = 3.0 * (s_q3 - s_q2);
    let c = -3.0 * p1 - 2.0 * d1 + 3.0 * p2 - d2;
    let d = 2.0 * p1 + d1 - 2.0 * p2 + d2;

    new_curve.p1 = p1;
    new_curve.p2 = p2;
    new_curve.d1 = d1;
    new_curve.d2 = d2;
    new_curve.c = c.as_vector();
    new_curve.d = d;

    new_curve
}
```

## is_valid
```rust
#[inline]
pub fn is_valid(&self) -> bool {
    (self.dim == 2 || self.dim == 3)
        && (self.p1.is_valid()
            && self.p2.is_valid()
            && self.d1.is_valid()
            && self.d2.is_valid()
            && self.c.is_valid()
            && self.d.is_valid())
}
```

## control_bounding_box
```rust
fn control_bounding_box(&self) -> BoundingBox {
    let mut bb = BoundingBox::default();
    let pts = [
        &self.p1,
        &(self.p1 + self.d1 / 3.0),
        &(self.p2 - self.d2 / 3.0),
        &self.p2,
    ];
    for pt in pts {
        bb.grow_Point(pt);
    }
    bb
}
```

## to_bezier 
```rust
fn to_bezier(&self) -> Option<BezierCurve> {
    if !self.is_valid() {
        return None;
    };

    let pt1 = &self.p1;
    let pt2 = self.p1 + self.d1 / 3.0;
    let pt3 = self.p2 - self.d2 / 3.0;
    let pt4 = &self.p2;

    let pt_4d1 = Point4D::new(pt1.x, pt1.y, pt1.z, 1.0);
    let pt_4d2 = Point4D::new(pt2.x, pt2.y, pt2.z, 1.0);
    let pt_4d3 = Point4D::new(pt3.x, pt3.y, pt3.z, 1.0);
    let pt_4d4 = Point4D::new(pt4.x, pt4.y, pt4.z, 1.0);

    Some(BezierCurve::new(vec![pt_4d1, pt_4d2, pt_4d3, pt_4d4]))
}
```
## to_nurbs
```rust
fn to_nurbs(&self) -> Option<Curve> {
    if !self.is_valid() {
        return None;
    };

    if let Some(curve) = self.to_bezier() {
        return Some(curve.to_nurbs());
    }
    None
}
```

---

