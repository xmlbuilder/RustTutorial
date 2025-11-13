# Line Direction, Curvature

## 📐 수학적 설명 요약
### 1. 방향 판별 (2D/3D)
- on_line_point_side_2d:  

$$
z=(b-a)\times (q-a)=(b_x-a_x)(q_y-a_y)-(b_y-a_y)(q_x-a_x)\]
$$

- → $z>0: Left, z<0: Right$

- on_line_point_side_xy / on_line_point_side:  
    3D 점을 XY 또는 임의 평면에 투영 후 위 수식으로 방향 판별

### 2. 거리 및 회전 각도- on_point_distance:

$$
\| a-b\| =\sqrt{(a_x-b_x)^2+(a_y-b_y)^2+(a_z-b_z)^2}
$$

- on_turn_cosine:

$$
\cos \theta =\frac{(b-a)\cdot (c-b)}{\| b-a\| \cdot \| c-b\| }
$$

### 3. 곡선/곡면 평가 (Power Basis)
- on_eval_curve_power3d:

$$
C(t)=\sum _{i=0}^na_it^i
$$

- on_eval_curve_power4d:
    - Rational curve:

$$
C(t)=\frac{\sum a_it^i}{w(t)}\quad \mathrm{where\  }w(t)=\sum w_it^i
$$

- on_eval_surface_power3d / power4d:

$$
S(u,v)=\sum _{i=0}^n\sum _{j=0}^ma_{ij}u^iv^j
$$

### 4. 도함수 계산
- on_eval_curve_power3d_deriv:

$$
C'(t)=\sum _{i=1}^nia_it^{i-1}
$$

- on_eval_curve_power4d_deriv:  
    - Rational 도함수:

$$
C'(t)=\frac{w\cdot C'-C\cdot w'}{w^2}
$$

- on_eval_surface_power3d_d1:

$$
\frac{\partial S}{\partial u} ,\quad \frac{\partial S}{\partial v}
$$

### 5. 곡률 계산- on_curve_kappa:

$$
\kappa =\frac{\| d_1\times d_2\| }{\| d_1\| ^3}
$$

- on_surface_curvature:
    - First fundamental form: 

$$
E=\vec {s}_u\cdot \vec {s}_u ,\  F=\vec {s}_u\cdot \vec {s}_v ,\  G=\vec {s}_v\cdot \vec {s}_v
$$

    - Second form: 

$$
e=\vec {n}\cdot \vec {s}_{uu} ,\  f=\vec {n}\cdot \vec {s}_{uv} ,\  g=\vec {n}\cdot \vec {s}_{vv}
$$

- Gaussian:

$$
K=\frac{eg-f^2}{EG-F^2}
$$

- Mean:

$$
H=\frac{Eg-2Ff+Ge}{2(EG-F^2)}
$$

### 6. 원호 Bezier- on_make_quarter_arc_bezier:
- 0°, 45°, 90° 점과 가중치 

$$
w=[1,\frac{1}{\sqrt{2}},1]
$$

### 7. 평면 관련- on_plane_eval:
- 평면 방정식 $ax+by+cz+d=0$ 에 점 대입
- on_intersect_line_plane / on_shoot_to_plane:

$$
t=\frac{-(n\cdot p_0+d)}{n\cdot \vec {dir}}\Rightarrow p=p_0+t\cdot \vec {dir}
$$

- on_pass_plane_side:  
    평면 평가값 s에 따라 Side 판별

## 📊 기능별 정리 표

| 기능 범주               | 함수 또는 개념 설명                         | 수학적 핵심 수식 또는 표현                                   |
|------------------------|---------------------------------------------|--------------------------------------------------------------|
| 방향 판별 (2D)          | 선분 AB 기준으로 점 Q의 방향 판별           | $z = (b - a) \times (q - a)$                            |
| 거리 계산              | 두 점 사이 거리                             | $\| a - b \|$                                           |
| 회전 각도              | 벡터 사이의 회전 각도 코사인                 | $\cos \theta = \frac{v_1 \cdot v_2}{\|v_1\| \cdot \|v_2\|}$|
| 곡선 평가 (3D)         | Power basis 곡선                            | $C(t) = \sum a_i t^i$                                  |
| 곡선 평가 (4D, Rational)| Rational curve 평가                         | $C(t) = \frac{\sum a_i t^i}{w(t)}$                      |
| 곡면 평가              | Power basis 곡면                            | $S(u,v) = \sum a_{ij} u^i v^j$                          |
| 곡선 도함수            | Power basis 곡선 도함수                     | $C'(t) = \sum i a_i t^{i-1}$                            |
| Rational 곡선 도함수   | Rational 도함수                             | $C'(t) = \frac{w C' - C w'}{w^2}$                       |
| 곡면 도함수            | 곡면의 1차 도함수                           | $\frac{\partial S}{\partial u},\ \frac{\partial S}{\partial v}$|
| 곡률 계산 (곡선)       | 곡선의 곡률                                 | $\kappa = \frac{\| d_1 \times d_2 \|}{\| d_1 \|^3}$     |
| 원호 Bezier            | 1/4 원호 라셔널 Bezier 점 + 가중치          | $w = [1,\ \frac{1}{\sqrt{2}},\ 1]$                      |
| 평면 방정식 평가       | 평면 위 점의 위치 평가                      | $ax + by + cz + d$                                     |
| 직선-평면 교차점       | 직선과 평면의 교차점 계산                   | $p = p_0 + t \cdot \vec{dir}$                           |
| 평면 방향 판별         | 평면 기준으로 점의 방향 판별                | $s \geq 0 \Rightarrow \mathrm{Left},\ s \leq 0 \Rightarrow \mathrm{Right}$|


```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}
```
```rust
pub fn on_line_point_side_2d(a: Point2, b: Point2, q: Point2, eps: f64) -> Side {
    let vx = b.x - a.x;
    let vy = b.y - a.y;
    let dx = q.x - a.x;
    let dy = q.y - a.y;
    let z = vx * dy - vy * dx;
    if z >= eps { Side::Left } else { Side::Right }
}
```
```rust
pub fn on_line_point_side_xy(a: Point, b: Point, q: Point, eps: f64) -> Side {
    on_line_point_side_2d(
        Point2::new(a.x, a.y),
        Point2::new(b.x, b.y),
        Point2::new(q.x, q.y),
        eps,
    )
}
```
```rust
/// Projects a 3D input onto a 2D plane defined by an arbitrary reference plane,
/// then determines left/right orientation.
/// If `ref_plane` is `None`, the XY plane is used by default.
pub fn on_line_point_side(line: &Segment3D, q: Point, ref_plane: Option<&Plane>, eps: f64) -> Side {
    if let Some(p) = ref_plane {
        // P의 로컬 (s,t)로 투영
        let proj = |x: Point| -> Point2 {
            let d = Vector::from_points(&p.origin, &x);
            Point2::new(Vector::dot(&d, &p.x_axis), Vector::dot(&d, &p.y_axis))

        };
        let a2 = proj(line.p0);
        let b2 = proj(line.p1);
        let q2 = proj(q);
        return on_line_point_side_2d(a2, b2, q2, eps);
    }
    on_line_point_side_xy(line.p0, line.p1, q, eps)
}
```
```rust
#[inline]
pub fn on_point_distance(a: &Point, b: &Point) -> f64 {
    a.distance(b)
}
```
```rust
/// Cosine of the rotation angle (cos θ) between vectors a→b and b→c.
/// Returns `None` if any segment has zero length.
pub fn on_turn_cosine(a: &Point, b: &Point, c: &Point) -> Option<f64> {
    let v1 = *b - *a;
    let v2 = *c - *b;
    let d1 = v1.length();
    let d2 = v2.length();
    if d1 <= 0.0 || d2 <= 0.0 {
        return None;
    }
    Some(Point::dot(&v1, &v2) / (d1 * d2))
}
```
```rust
// ------------------------------
// Power basis evaluation (curve/surface) + derivatives / rational transformation
// ------------------------------
pub fn on_eval_curve_power3d(a: &[Point], degree: usize, t: f64) -> Point {
    let mut s = Point::new(0.0, 0.0, 0.0);
    let mut tp = 1.0;
    for i in 0..=degree {
        s.x += a[i].x * tp;
        s.y += a[i].y * tp;
        s.z += a[i].z * tp;
        tp *= t;
    }
    s
}
```
```rust
pub fn on_eval_curve_power4d(a: &[CPoint], degree: usize, t: f64) -> CPoint {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut w = 0.0;
    let mut tp = 1.0;
    for i in 0..=degree {
        x += a[i].x * tp;
        y += a[i].y * tp;
        z += a[i].z * tp;
        w += a[i].w * tp;
        tp *= t;
    }
    if w.abs() < 1e-14 {
        CPoint::new(x, y, z, 1.0)
    } else {
        CPoint::new(x, y, z, w)
    }
}
```
```rust
pub fn on_eval_surface_power3d(a: &[Vec<Point>], n: usize, m: usize, u: f64, v: f64) -> Point {
    let mut s = Point::new(0.0, 0.0, 0.0);
    for i in 0..=n {
        let ui = u.powi(i as i32);
        for j in 0..=m {
            let c = ui * v.powi(j as i32);
            s.x += a[i][j].x * c;
            s.y += a[i][j].y * c;
            s.z += a[i][j].z * c;
        }
    }
    s
}
```
```rust
pub fn on_eval_surface_power4d(a: &[Vec<CPoint>], n: usize, m: usize, u: f64, v: f64) -> Point {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut w = 0.0;
    for i in 0..=n {
        let ui = u.powi(i as i32);
        for j in 0..=m {
            let c = ui * v.powi(j as i32);
            x += a[i][j].x * c;
            y += a[i][j].y * c;
            z += a[i][j].z * c;
            w += a[i][j].w * c;
        }
    }
    if w.abs() < 1e-14 {
        Point::new(x, y, z)
    } else {
        Point::new(x / w, y / w, z / w)
    }
}
```
```rust
pub fn on_eval_curve_power3d_deriv(a: &[Point], n: usize, t: f64) -> Vector {
    let mut d = Vector::new(0.0, 0.0, 0.0);
    for i in 1..=n {
        let c = (i as f64) * t.powi((i - 1) as i32);
        d.x += a[i].x * c;
        d.y += a[i].y * c;
        d.z += a[i].z * c;
    }
    d
}
```
```rust
pub fn on_eval_curve_power4d_deriv(a: &[CPoint], n: usize, t: f64) -> Vector {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut w = 0.0;
    let mut xd = 0.0;
    let mut yd = 0.0;
    let mut zd = 0.0;
    let mut wd = 0.0;
    for i in 0..=n {
        let ti = t.powi(i as i32);
        x += a[i].x * ti;
        y += a[i].y * ti;
        z += a[i].z * ti;
        w += a[i].w * ti;
        if i >= 1 {
            let c = (i as f64) * t.powi((i - 1) as i32);
            xd += a[i].x * c;
            yd += a[i].y * c;
            zd += a[i].z * c;
            wd += a[i].w * c;
        }
    }
    let w2 = w * w;
    if w2.abs() < 1e-14 {
        return Vector::new(0.0, 0.0, 0.0);
    }
    Vector::new(
        (xd * w - x * wd) / w2,
        (yd * w - y * wd) / w2,
        (zd * w - z * wd) / w2,
    )
}
```
```rust
pub struct Eval3dD1 {
    pub s: Point,
    pub su: Vector,
    pub sv: Vector,
}
```
```rust
pub struct Eval3dD2 {
    pub s: Point,
    pub su: Vector,
    pub sv: Vector,
    pub suu: Vector,
    pub suv: Vector,
    pub svv: Vector,
}
```
```rust
pub fn on_eval_surface_power3d_d1(a: &[Vec<Point>], n: usize, m: usize, u: f64, v: f64) -> Eval3dD1 {
    let mut s = Point::new(0.0, 0.0, 0.0);
    let mut su = Vector::new(0.0, 0.0, 0.0);
    let mut sv = Vector::new(0.0, 0.0, 0.0);

    let mut up = vec![1.0; n + 1];
    let mut vp = vec![1.0; m + 1];
    for i in 1..=n {
        up[i] = up[i - 1] * u;
    }
    for j in 1..=m {
        vp[j] = vp[j - 1] * v;
    }

    let mut dup = vec![0.0; n + 1];
    let mut dvp = vec![0.0; m + 1];
    for i in 1..=n {
        dup[i] = (i as f64) * u.powi((i - 1) as i32);
    }
    for j in 1..=m {
        dvp[j] = (j as f64) * v.powi((j - 1) as i32);
    }

    for i in 0..=n {
        for j in 0..=m {
            let c = up[i] * vp[j];
            let cu = dup[i] * vp[j];
            let cv = up[i] * dvp[j];
            let aij = a[i][j];
            s.x += aij.x * c;
            s.y += aij.y * c;
            s.z += aij.z * c;
            su.x += aij.x * cu;
            su.y += aij.y * cu;
            su.z += aij.z * cu;
            sv.x += aij.x * cv;
            sv.y += aij.y * cv;
            sv.z += aij.z * cv;
        }
    }
    Eval3dD1 { s, su, sv }
}
```
```rust
pub fn on_curve_kappa(d1: Vector, d2: Vector) -> f64 {
    let c = d1.cross(&d2);
    let n = c.length();
    let s = d1.length();
    if s <= 0.0 { 0.0 } else { n / (s * s * s) }
}
```
```rust
pub fn on_surface_curvature(
    su: Vector,
    sv: Vector,
    suu: Vector,
    suv: Vector,
    svv: Vector,
) -> Option<(f64, f64, f64, f64)> {
    let e = su.dot(&su);
    let f = su.dot(&sv);
    let g = sv.dot(&sv);
    let n = su.cross(&sv);
    let n_len = n.length();
    if n_len == 0.0 {
        return None;
    }
    let nh = n / n_len;
    let ee = nh.dot(&suu);
    let ff = nh.dot(&suv);
    let gg = nh.dot(&svv);
    let eg_f2 = e * g - f * f;
    if eg_f2 == 0.0 {
        return None;
    }
    let k = (ee * gg - ff * ff) / eg_f2;
    let h = (e * gg - 2.0 * f * ff + g * ee) / (2.0 * eg_f2);
    let disc = (h * h - k).max(0.0);
    let s = disc.sqrt();
    let k1 = h + s;
    let k2 = h - s;
    Some((k, h, k1, k2))
}
```
```rust
// ------------------------------
// 1/4 원호 Bezier(라셔널)용 포인트 + weight
// ------------------------------
pub fn on_make_quarter_arc_bezier(r: f64, z: f64) -> ([Point; 3], [f64; 3]) {
    // 0°, 45°, 90° (중간 가중치 = 1/√2)
    let w_mid = (0.5f64).sqrt();
    let row = [
        Point::new(r, 0.0, z),
        Point::new(r, r, z),
        Point::new(0.0, r, z),
    ];
    let w = [1.0, w_mid, 1.0];
    (row, w)
}
```
```rust
pub fn on_plane_eval(pl: &Plane, p: Point) -> f64 {
    // Plane 은 이미 equation 을 유지한다고 가정
    pl.equation.value_at_point(p)
}
```
```rust
pub fn on_intersect_line_plane(line_from: Point, line_to: Point, pl: &Plane) -> Option<Point> {
    let n = pl.normal();
    let d = -(n.x * pl.origin.x + n.y * pl.origin.y + n.z * pl.origin.z);
    let p0 = line_from;
    let dir = line_to - line_from;
    let denom = Vector::dot(&n, &dir);
    if denom.abs() < 1e-14 {
        return None;
    }
    let t = -(n.x * p0.x + n.y * p0.y + n.z * p0.z + d) / denom;
    Some(p0 + dir * t)
}
```
```rust
pub fn on_shoot_to_plane(q0: Point, w: Vector, pl: &Plane) -> Option<Point> {
    let n = pl.normal();
    let d = -(n.x * pl.origin.x + n.y * pl.origin.y + n.z * pl.origin.z);
    let denom = Vector::dot(&n, &w);
    if denom.abs() < 1e-14 {
        return None;
    }
    let t = -(n.x * q0.x + n.y * q0.y + n.z * q0.z + d) / denom;
    Some(q0 + w * t)
}
```
```rust
pub fn on_pass_plane_side(pl: &Plane, p: Point, side: Side) -> bool {
    let s = on_plane_eval(pl, p);
    match side {
        Side::Left => s >= 0.0,
        Side::Right => s <= 0.0,
    }
}
```

--- 

# 테스트 코드

각 함수에 대한 Rust 테스트 코드 예시입니다.  
간단한 입력을 통해 함수의 동작을 검증할 수 있도록 구성했습니다.

## 📊 함수별 테스트 코드 요약 표

| 함수 이름                        | 테스트 목적                         | 검증 내용 또는 기대 결과                                 |
|----------------------------------|-------------------------------------|----------------------------------------------------------|
| `on_line_point_side_2d`          | 2D 방향 판별                        | 점이 선분 왼쪽/오른쪽에 있는지 확인                      |
| `on_point_distance`              | 두 점 거리 계산                     | 두 점 사이 거리 = 5.0                                    |
| `on_turn_cosine`                | 회전 각도 코사인 계산               | $\cos(45^\circ) \approx 0.707$                       |
| `on_eval_curve_power3d`         | 3D 곡선 평가                        | $C(2) = (2, 4, 0)$                                   |
| `on_eval_curve_power3d_deriv`   | 3D 곡선 도함수                      | $C'(2) = (1, 8, 0)$                                  |
| `on_curve_kappa`                | 곡선 곡률 계산                      | 직교 벡터 → $\kappa = 1.0$                           |
| `on_intersect_line_plane`       | 직선과 평면 교차점 계산            | 평면 z=0과 교차점 z=0 확인                               |
| `on_pass_plane_side`            | 평면 기준 방향 판별                | 위쪽 → Left, 아래쪽 → Right                              |
| `on_make_quarter_arc_bezier`    | 1/4 원호 Bezier 점 및 가중치 생성  | 중간 가중치 = $\frac{1}{\sqrt{2}}$                   |

### ✅ 방향 판별 함수
```rust
#[test]
fn test_on_line_point_side_2d() {
    let a = Point2::new(0.0, 0.0);
    let b = Point2::new(1.0, 0.0);
    let q_left = Point2::new(0.5, 1.0);
    let q_right = Point2::new(0.5, -1.0);
    assert_eq!(on_line_point_side_2d(a, b, q_left, 1e-12), Side::Left);
    assert_eq!(on_line_point_side_2d(a, b, q_right, 1e-12), Side::Right);
}
```


### ✅ 거리 및 회전 각도
```rust
#[test]
fn test_on_point_distance() {
    let a = Point::new(0.0, 0.0, 0.0);
    let b = Point::new(3.0, 4.0, 0.0);
    assert!((on_point_distance(&a, &b) - 5.0).abs() < 1e-12);
}
```
```rust
#[test]
fn test_on_turn_cosine() {
    let a = Point::new(0.0, 0.0, 0.0);
    let b = Point::new(1.0, 0.0, 0.0);
    let c = Point::new(1.0, 1.0, 0.0);
    let cos = on_turn_cosine(&a, &b, &c).unwrap();
    assert!((cos - 0.70710678).abs() < 1e-6); // cos(45°)
}
```

### ✅ 곡선/곡면 평가
```rust
#[test]
fn test_on_eval_curve_power3d() {
    let a = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ];
    let p = on_eval_curve_power3d(&a, 2, 2.0);
    assert!((p.x - 2.0).abs() < 1e-12);
    assert!((p.y - 4.0).abs() < 1e-12);
}
```

### ✅ 곡선 도함수
```rust
#[test]
fn test_on_eval_curve_power3d_deriv() {
    let a = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 2.0, 0.0),
    ];
    let d = on_eval_curve_power3d_deriv(&a, 2, 2.0);
    assert!((d.x - 1.0).abs() < 1e-12);
    assert!((d.y - 8.0).abs() < 1e-12);
}
```


### ✅ 곡률 계산
```rust
#[test]
fn test_on_curve_kappa() {
    let d1 = Vector::new(1.0, 0.0, 0.0);
    let d2 = Vector::new(0.0, 1.0, 0.0);
    let kappa = on_curve_kappa(d1, d2);
    assert!((kappa - 1.0).abs() < 1e-12);
}
```


### ✅ 평면 교차 및 방향
```rust
#[test]
fn test_on_intersect_line_plane() {
    let p0 = Point::new(0.0, 0.0, -1.0);
    let p1 = Point::new(0.0, 0.0, 1.0);
    let pl = Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let ip = on_intersect_line_plane(p0, p1, &pl).unwrap();
    assert!((ip.z).abs() < 1e-12);
}
```
```rust
#[test]
fn test_on_pass_plane_side() {
    let pl = Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let above = Point::new(0.0, 0.0, 1.0);
    let below = Point::new(0.0, 0.0, -1.0);
    assert!(on_pass_plane_side(&pl, above, Side::Left));
    assert!(on_pass_plane_side(&pl, below, Side::Right));
}
```

### ✅ 원호 Bezier 생성
```rust
#[test]
fn test_on_make_quarter_arc_bezier() {
    let (pts, weights) = on_make_quarter_arc_bezier(1.0, 0.0);
    assert_eq!(pts[0], Point::new(1.0, 0.0, 0.0));
    assert_eq!(weights[1], 0.5f64.sqrt());
}
```


