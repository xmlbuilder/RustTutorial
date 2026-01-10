# PointInterpolator
PointInterpolator 구조체에 정의된 다양한 보간 함수들을 정리한 것입니다.    
각 함수의 목적, 수식, 입력 타입 등을 포함해 깔끔하게 요약.

## 🧩 구조체 개요
```rust
pub struct PointInterpolator;
```
- 다양한 공간(Point, Point2, Vector, Vector2)에 대해 보간 기능을 제공하는 유틸리티 구조체입니다.
- 모든 함수는 static 방식으로 사용됩니다.

## 📐 선형 보간 (Linear Interpolation)
```rust
lerp_3d(p0: &Point, p1: &Point, t: f64) -> Point
lerp_2d(p0: &Point2, p1: &Point2, t: f64) -> Point2
```
수식:

$$
P(t)=(1-t)\cdot P_0+t\cdot P_1
$$

설명:
- 두 점 사이의 선형 보간
- $t ∈ [0, 1]$ 일 때 P0에서 P1까지의 중간 위치 반환

## 📚 다중 선형 보간
```rust
lerp_3d_many(points: &[Point], t: f64) -> Point
lerp_2d_many(points: &[Point2], t: f64) -> Point2
```

### 설명:
- 여러 점으로 구성된 경로에서 t에 해당하는 위치를 선형 보간
- $t ∈ [0, 1]$ 일 때 points[i]와 points[i+1] 사이 보간

## 🧮 Catmull-Rom 보간
```rust
catmull_rom_3d(p0, p1, p2, p3, t) -> Point
catmull_rom_2d(p0, p1, p2, p3, t) -> Point2
```
수식:

$$
P(t)=0.5\cdot \left( 2P_1+(-P_0+P_2)t+(2P_0-5P_1+4P_2-P_3)t^2+(-P_0+3P_1-3P_2+P_3)t^3\right)
$$

### 설명:
- 4개의 점을 기반으로 자연스러운 곡선 생성
- $t ∈ [0, 1]$ 일 때 p1과 p2 사이 보간

## 🎯 2차 베지어 곡선 (Quadratic Bézier)
quadratic_bezier_*
- quadratic_bezier_vec3d(t, p0, p1, p2) -> Vector
- quadratic_bezier_vec2d(t, p0, p1, p2) -> Vector2
- quadratic_bezier_Point(t, p0, p1, p2) -> Point
- quadratic_bezier_Point2(t, p0, p1, p2) -> Point2

### 수식:

$$
P(t)=(1-t)^2\cdot P_0+2(1-t)t\cdot P_1+t^2\cdot P_2
$$

### 설명:
- 3개의 제어점을 기반으로 곡선 생성
- $t ∈ [0, 1]$ 일 때 곡선상의 위치 반환

## 🧵 3차 베지어 곡선 (Cubic Bézier)
cubic_bezier_*
- cubic_bezier_vec3d(t, p0, p1, p2, p3) -> Vector
- cubic_bezier_vec2d(t, p0, p1, p2, p3) -> Vector2
- cubic_bezier_Point(t, p0, p1, p2, p3) -> Point
- cubic_bezier_Point2(t, p0, p1, p2, p3) -> Point2

### 수식:

$$
P(t)=(1-t)^3\cdot P_0+3(1-t)^2t\cdot P_1+3(1-t)t^2\cdot P_2+t^3\cdot P_3
$$


### 설명:
- 4개의 제어점을 기반으로 곡선 생성
- $t ∈ [0, 1]$ 일 때 곡선상의 위치 반환

## 🧭 Barycentric 좌표 계산
```rust
barycentric_Point2(p, a, b, c) -> (f64, f64, f64)
```
### 설명:
- 삼각형 abc 내에서 점 p의 위치를 (u, v, w)로 표현
- 면적 기반 가중치 계산

## 🧊 3D Trilinear 보간
```rust
tri_linear_point(f, tx, ty, tz) -> Point
```

### 수식:
- 3차원 격자 f[2][2][2]에서 tx, ty, tz 위치의 보간값 계산
- 선형 보간을 x → y → z 순으로 적용

---

## 소스

```rust
/// Point interpolation (assuming Point/Point2 support +, −, and *f64 operations)
pub struct PointInterpolator;
```
```rust
impl PointInterpolator {
    #[inline]
    pub fn lerp_3d(p0: &Point, p1: &Point, t: f64) -> Point {
        p0 * (1.0 - t) + p1 * t
    }
```
```rust    
    #[inline]
    pub fn lerp_2d(p0: &Point2, p1: &Point2, t: f64) -> Point2 {
        p0 * (1.0 - t) + p1 * t
    }
```
```rust
    pub fn lerp_3d_many(points: &[Point], t: f64) -> Point {
        if points.is_empty() {
            return Point::UNSET;
        }
        if points.len() == 1 {
            return points[0];
        }
        let pos = t * ((points.len() - 1) as f64);
        let i = pos.floor() as usize;
        let local = pos - (i as f64);
        if i >= points.len() - 1 {
            return points[points.len() - 1];
        }
        Self::lerp_3d(&points[i], &points[i + 1], local)
    }
```
```rust
    pub fn lerp_2d_many(points: &[Point2], t: f64) -> Point2 {
        if points.is_empty() {
            return Point2::UNSET;
        }
        if points.len() == 1 {
            return points[0];
        }
        let pos = t * ((points.len() - 1) as f64);
        let i = pos.floor() as usize;
        let local = pos - (i as f64);
        if i >= points.len() - 1 {
            return points[points.len() - 1];
        }
        Self::lerp_2d(&points[i], &points[i + 1], local)
    }
```
```rust
    pub fn catmull_rom_3d(p0: Point, p1: Point, p2: Point, p3: Point, t: f64) -> Point {
        let t2 = t * t;
        let t3 = t2 * t;
        0.5 * ((2.0 * p1)
            + (-p0 + p2) * t
            + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
            + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
    }
```
```rust
    pub fn catmull_rom_2d(p0: Point2, p1: Point2, p2: Point2, p3: Point2, t: f64) -> Point2 {
        let t2 = t * t;
        let t3 = t2 * t;
        0.5 * ((2.0 * p1)
            + (-p0 + p2) * t
            + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
            + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
    }
```
```rust
    pub fn quadratic_bezier_vec3d(t: f64, p0: Vector, p1: Vector, p2: Vector) -> Vector {
        let t1 = 1.0 - t;
        let term1 = p0 * (t1 * t1); // (1-t)^2 * P0
        let term2 = p1 * (2.0 * t1 * t); // 2*(1-t)*t * P1
        let term3 = p2 * (t * t); // t^2 * P2
        term1 + term2 + term3
    }
```
```rust
    pub fn quadratic_bezier_vec2d(t: f64, p0: Vector2, p1: Vector2, p2: Vector2) -> Vector2 {
        let t1 = 1.0 - t;
        let term1 = p0 * (t1 * t1); // (1-t)^2 * P0
        let term2 = p1 * (2.0 * t1 * t); // 2*(1-t)*t * P1
        let term3 = p2 * (t * t); // t^2 * P2
        term1 + term2 + term3
    }
```
```rust
    pub fn quadratic_bezier_Point(t: f64, p0: Point, p1: Point, p2: Point) -> Point {
        let t1 = 1.0 - t;
        let term1 = p0 * (t1 * t1); // (1-t)^2 * P0
        let term2 = p1 * (2.0 * t1 * t); // 2*(1-t)*t * P1
        let term3 = p2 * (t * t); // t^2 * P2
        term1 + term2 + term3
    }
```
```rust
    pub fn quadratic_bezier_Point2(t: f64, p0: Point2, p1: Point2, p2: Point2) -> Point2 {
        let t1 = 1.0 - t;
        let term1 = p0 * (t1 * t1); // (1-t)^2 * P0
        let term2 = p1 * (2.0 * t1 * t); // 2*(1-t)*t * P1
        let term3 = p2 * (t * t); // t^2 * P2
        term1 + term2 + term3
    }
```
```rust
    // Cubic Bézier interpolation 함수
    pub fn cubic_bezier_vec2d(
        t: f64,
        p0: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> Vector2 {
        let t1 = 1.0 - t;
        let term1 = p0 * (t1 * t1 * t1); // (1-t)^3 * P0
        let term2 = p1 * (3.0 * t1 * t1 * t); // 3*(1-t)^2*t * P1
        let term3 = p2 * (3.0 * t1 * t * t); // 3*(1-t)*t^2 * P2
        let term4 = p3 * (t * t * t); // t^3 * P3
        term1 + term2 + term3 + term4
    }
```
```rust
    pub fn cubic_bezier_vec3d(
        t: f64,
        p0: Vector,
        p1: Vector,
        p2: Vector,
        p3: Vector,
    ) -> Vector {
        let t1 = 1.0 - t;
        let term1 = p0 * (t1 * t1 * t1); // (1-t)^3 * P0
        let term2 = p1 * (3.0 * t1 * t1 * t); // 3*(1-t)^2*t * P1
        let term3 = p2 * (3.0 * t1 * t * t); // 3*(1-t)*t^2 * P2
        let term4 = p3 * (t * t * t); // t^3 * P3
        term1 + term2 + term3 + term4
    }
```
```rust
    pub fn cubic_bezier_Point2(
        t: f64,
        p0: Point2,
        p1: Point2,
        p2: Point2,
        p3: Point2,
    ) -> Point2 {
        let t1 = 1.0 - t;
        let term1 = p0 * (t1 * t1 * t1); // (1-t)^3 * P0
        let term2 = p1 * (3.0 * t1 * t1 * t); // 3*(1-t)^2*t * P1
        let term3 = p2 * (3.0 * t1 * t * t); // 3*(1-t)*t^2 * P2
        let term4 = p3 * (t * t * t); // t^3 * P3
        term1 + term2 + term3 + term4
    }
```
```rust
    pub fn cubic_bezier_Point(
        t: f64,
        p0: Point,
        p1: Point,
        p2: Point,
        p3: Point,
    ) -> Point {
        let t1 = 1.0 - t;
        let term1 = p0 * (t1 * t1 * t1); // (1-t)^3 * P0
        let term2 = p1 * (3.0 * t1 * t1 * t); // 3*(1-t)^2*t * P1
        let term3 = p2 * (3.0 * t1 * t * t); // 3*(1-t)*t^2 * P2
        let term4 = p3 * (t * t * t); // t^3 * P3
        term1 + term2 + term3 + term4
    }
```
```rust
    #[allow(unused)]
    fn barycentric_Point2(p: Point2, a: Point2, b: Point2, c: Point2) -> (f64, f64, f64) {
        let v0 = Point2 {
            x: b.x - a.x,
            y: b.y - a.y,
        };
        let v1 = Point2 {
            x: c.x - a.x,
            y: c.y - a.y,
        };
        let v2 = Point2 {
            x: p.x - a.x,
            y: p.y - a.y,
        };
        let d00 = v0.x * v0.x + v0.y * v0.y;
        let d01 = v0.x * v1.x + v0.y * v1.y;
        let d11 = v1.x * v1.x + v1.y * v1.y;
        let d20 = v2.x * v0.x + v2.y * v0.y;
        let d21 = v2.x * v1.x + v2.y * v1.y;
        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        (u, v, w)
    }
```
```rust
    #[allow(unused)]
    fn tri_linear_point(f: [[[Point; 2]; 2]; 2], tx: f64, ty: f64, tz: f64) -> Point {
        let c00 = f[0][0][0] * (1.0 - tx) + f[1][0][0] * tx;
        let c01 = f[0][0][1] * (1.0 - tx) + f[1][0][1] * tx;
        let c10 = f[0][1][0] * (1.0 - tx) + f[1][1][0] * tx;
        let c11 = f[0][1][1] * (1.0 - tx) + f[1][1][1] * tx;

        let c0 = c00 * (1.0 - ty) + c10 * ty;
        let c1 = c01 * (1.0 - ty) + c11 * ty;

        c0 * (1.0 - tz) + c1 * tz
    }
}
```























