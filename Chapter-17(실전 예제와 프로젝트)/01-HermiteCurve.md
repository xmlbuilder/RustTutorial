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
    p1: Point3D,
    d1: Vector3D,
    p2: Point3D,
    d2: Vector3D,
    c:  Vector3D,
    d:  Vector3D,
    dim: usize, // 2 or 3 (정보용)
}

impl HermiteCurve {
    pub fn new_3d(p1: Point3D, v1: Vector3D, p2: Point3D, v2: Vector3D) -> Self {
        let c = Vector3D {
            x: -3.0*p1.x - 2.0*v1.x + 3.0*p2.x - v2.x,
            y: -3.0*p1.y - 2.0*v1.y + 3.0*p2.y - v2.y,
            z: -3.0*p1.z - 2.0*v1.z + 3.0*p2.z - v2.z,
        };
        let d = Vector3D {
            x:  2.0*p1.x + v1.x - 2.0*p2.x + v2.x,
            y:  2.0*p1.y + v1.y - 2.0*p2.y + v2.y,
            z:  2.0*p1.z + v1.z - 2.0*p2.z + v2.z,
        };
        Self { p1, d1: v1, p2, d2: v2, c, d, dim: 3 }
    }

    pub fn new_2d(p1: Point2D, v1: Vector2D, p2: Point2D, v2: Vector2D) -> Self {
        let p1_3 = Point3D { x: p1.x, y: p1.y, z: 0.0 };
        let p2_3 = Point3D { x: p2.x, y: p2.y, z: 0.0 };
        let v1_3 = Vector3D { x: v1.x, y: v1.y, z: 0.0 };
        let v2_3 = Vector3D { x: v2.x, y: v2.y, z: 0.0 };
        let mut h = Self::new_3d(p1_3, v1_3, p2_3, v2_3);
        h.dim = 2;
        h
    }

    #[inline] pub fn is_valid(&self) -> bool {
        (self.dim == 2 || self.dim == 3)
    }

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

    /// 위치/도함수 평가. l_num_derivatives: 0..=3 지원
    pub fn evaluate(&self, t: f64, l_num_derivatives: usize) -> [Point3D; 4] {
        let u  = t;
        let a  = self.p1;
        let b  = self.d1;
        let c  = self.c;
        let d  = self.d;

        // P
        let p = Point3D {
            x: a.x + u*(b.x + u*(c.x + u*d.x)),
            y: a.y + u*(b.y + u*(c.y + u*d.y)),
            z: a.z + u*(b.z + u*(c.z + u*d.z)),
        };

        // 1st
        let dp = if l_num_derivatives >= 1 {
            Point3D {
                x: b.x + u*(2.0*c.x + 3.0*u*d.x),
                y: b.y + u*(2.0*c.y + 3.0*u*d.y),
                z: b.z + u*(2.0*c.z + 3.0*u*d.z),
            }
        } else { Point3D { x:0.0,y:0.0,z:0.0 } };

        // 2nd
        let ddp = if l_num_derivatives >= 2 {
            Point3D {
                x: 2.0*c.x + 6.0*u*d.x,
                y: 2.0*c.y + 6.0*u*d.y,
                z: 2.0*c.z + 6.0*u*d.z,
            }
        } else { Point3D { x:0.0,y:0.0,z:0.0 } };

        // 3rd
        let dddp = if l_num_derivatives >= 3 {
            Point3D { x: 6.0*d.x, y: 6.0*d.y, z: 6.0*d.z }
        } else { Point3D { x:0.0,y:0.0,z:0.0 } };

        [p, dp, ddp, dddp]
    }

    #[inline]
    pub fn evaluate_point(&self, t: f64) -> Point3D {
        self.evaluate(t, 0)[0]
    }
}

```
