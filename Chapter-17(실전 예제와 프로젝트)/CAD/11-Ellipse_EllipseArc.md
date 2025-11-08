# 📘 Ellipse 구조 문서 요약

## 수식 정리

아래는 Ellipse 구조체에서 사용된 주요 수학적 수식들을 설명과 함께 정리한 내용입니다.  
CAD나 곡선 모델링에서 타원을 다룰 때 핵심이 되는 수식들을 직관적으로 이해할 수 있도록 구성했습니다.

## 📐 타원 관련 수학 수식 설명
### 1. 타원 위의 점 계산

$$
E(t)=O+rx\cdot \cos (t)\cdot X+ry\cdot \sin (t)\cdot Y
$$

- O: 중심점
- X, Y: 평면의 x축, y축
- rx, ry: 반지름
- t: 파라미터 (0 ~ 2π)  
👉 point_at(t)에서 사용됨. 타원 위의 점을 파라미터 t로 계산합니다.

### 2. 도함수 패턴 (주기 4)
- 1차 도함수:

$$
E'(t)=-rx\cdot \sin (t)\cdot X+ry\cdot \cos (t)\cdot Y
$$

- 2차 도함수:

$$
E''(t)=-rx\cdot \cos (t)\cdot X-ry\cdot \sin (t)\cdot Y
$$


👉 derivative_at(k, t)와 normal_at(t)에서 사용됨.  
k차 도함수는 4주기 패턴으로 반복됩니다.

### 3. 곡률 계산

$$
\kappa (t)=\frac{|\mathbf{r}'(t)\times \mathbf{r}''(t)|}{|\mathbf{r}'(t)|^3}
$$

- r'(t): 1차 도함수
- r''(t): 2차 도함수
- ×: 벡터 외적
- ·: 벡터 길이  
👉 curvature_value_at(t)에서 사용됨.  
타원 곡선의 곡률을 계산합니다.

### 4. Ramanujan 근사식 (둘레 길이)

$$
L\approx \pi (a+b)\left( 1+\frac{3h}{10+\sqrt{4-3h}}\right) \quad \mathrm{where}\quad h=\frac{(a-b)^2}{(a+b)^2}
$$

- a, b: rx, ry 중 큰 값과 작은 값  
👉 length()에서 사용됨.  
정확도 높고 계산 빠른 근사식입니다.  

### 5. 초점 거리 계산

$$
c=a\cdot \sqrt{1-\left( \frac{b}{a}\right) ^2}\quad \mathrm{where}\quad a\geq b
$$

👉 focal_distance()에서 사용됨.  
두 초점 사이의 거리입니다.  

### 6. 타원 내부 포함 판정
- 점 P가 타원 내부에 있는지 확인:

$$
\left( \frac{x}{rx}\right) ^2+\left( \frac{y}{ry}\right) ^2<1
$$

👉 is_point_inside(p)에서 사용됨.  
평면에 투영된 좌표 기준으로 계산합니다.  

### 7. Lagrange 최적화 기반 최단점 계산
- 목적 함수:

$$
F(\lambda )=\frac{rx^2\cdot px^2}{(\lambda +rx^2)^2}+\frac{ry^2\cdot py^2}{(\lambda +ry^2)^2}-1
$$

👉 closest_param(p)에서 사용됨.  
내부/외부 점에 대해 λ를 이분법으로 찾아 최단점을 계산합니다.  

### 8. 면적 계산

$$
A=\pi \cdot rx\cdot ry
$$

👉 area()에서 사용됨.  
타원의 면적 공식입니다.  

## 🔧 구조 정의
```rust
pub struct Ellipse {
    pub plane: Plane,                      // 타원이 놓인 평면
    pub radius_x: f64,                     // X축 반지름
    pub radius_y: f64,                     // Y축 반지름
    pub edge_index: i32,                   // 엣지 인덱스 (CAD 연동용)
    pub from_boolean_intersection: bool,   // 불리언 연산 결과 여부
}
```


## 🏗️ 생성자 및 유효성 검사
| 메서드명                                 | 설명                                       |
|------------------------------------------|--------------------------------------------|
| `new(center, rx, ry)`                    | 중심점과 반지름으로 타원 생성               |
| `new_on_plane(plane, center, rx, ry)`    | 주어진 평면 위에 중심점과 반지름으로 생성   |
| `from_xy(x, y, z, rx, ry)`               | 좌표 기반으로 타원 생성                    |
| `from_plane(plane, rx, ry)`              | 평면과 반지름으로 타원 생성                |
| `validate()` / `is_valid()`              | 평면과 반지름이 유효한지 검사              |
| `is_circle()`                            | 타원이 원에 가까운지 여부 확인             |
| `center()`                               | 타원의 중심점 반환                         |


## 📐 기하 속성
| 메서드명           | 설명                                 |
|--------------------|--------------------------------------|
| `focal_distance()` | 두 초점 사이의 거리 계산              |
| `foci()`           | 타원의 두 초점 좌표 반환              |


## 📊 평가 함수
| 메서드명                                 | 설명                                           |
|------------------------------------------|------------------------------------------------|
| `point_at(t)`                            | 파라미터 t에 해당하는 타원 위의 점 반환         |
| `derivative_at(k, t)`                    | k차 도함수 벡터 계산                           |
| `tangent_at(t)`                          | 접선 벡터 계산                                 |
| `normal_at(t)`                           | 법선 벡터 계산 (2차 도함수 기반)               |
| `curvature_value_at(t)`                  | 곡률 값 계산                                   |
| `derive_at_static(k, t, plane, r0, r1)`  | 정적 도함수 계산 (평면과 반지름 직접 지정)     |

## 📏 길이 및 면적
| 메서드명               | 설명                                       |
|------------------------|--------------------------------------------|
| `length()`             | 전체 타원의 둘레 길이 계산 (Ramanujan 근사) |
| `arc_length(t0, t1)`   | 파라미터 구간 [t0, t1]에 해당하는 호의 길이 |
| `area()`               | 타원의 면적 계산                           |


## 📌 거리 및 최단점 계산
| 메서드명                         | 설명                                                   |
|----------------------------------|--------------------------------------------------------|
| `closest_param(p)`              | 주어진 점에 가장 가까운 타원 파라미터 t 계산            |
| `closest_point(p)`              | 주어진 점에 가장 가까운 타원 위의 점 반환               |
| `project(point, &mut t)`        | 평면에 점을 투영하여 타원 파라미터 t 추정               |
| `param_from_polar(rx, ry, θ)`   | 극각 θ를 타원 파라미터 t로 변환                         |



## 📐 도메인 및 분할
| 메서드명                             | 설명                                                   |
|--------------------------------------|--------------------------------------------------------|
| `domain()`                           | 타원의 파라미터 도메인 [0, 2π] 반환                     |
| `split_at(t)`                        | 파라미터 t를 기준으로 두 개의 타원 호로 분할            |
| `sub_curve(t0, t1)`                  | 파라미터 구간 [t0, t1]에 해당하는 타원 호 반환          |
| `start_point()` / `end_point()`      | 타원의 시작점과 끝점 반환 (t = 0, t = 2π)               |
| `start_tangent()` / `end_tangent()`  | 시작점과 끝점에서의 접선 벡터 반환                      |



## 🔄 변환 및 평면 판정
| 메서드명                     | 설명                                                             |
|------------------------------|------------------------------------------------------------------|
| `transform_by(xform)`        | 평면과 반지름을 주어진 변환 행렬에 따라 변환                      |
| `reverse()`                  | 타원의 파라미터 진행 방향을 반전 (Y축 방향 반전)                  |
| `is_planar(tol)`             | 타원이 평면 객체인지 여부 반환                                   |
| `is_in_plane(plane, tol)`    | 주어진 평면에 타원이 포함되는지 여부 검사                         |



## 🧪 포함 판정 및 길이 ↔ 파라미터 변환
| 메서드명                         | 설명                                                       |
|----------------------------------|------------------------------------------------------------|
| `is_point_inside(p)`            | 주어진 점이 타원 내부에 포함되는지 여부 판단                |
| `is_linear(tol)`                | 타원이 선형 객체인지 여부 반환 (항상 false)                |
| `get_param_from_length(len)`    | 전체 길이 중 일부 길이에 해당하는 파라미터 t 계산           |
| `get_length_from_param(u)`      | 파라미터 u에 해당하는 시작점부터의 길이 계산                |

## 🔁 NURBS 변환
| 메서드명       | 설명                                      |
|----------------|-------------------------------------------|
| `to_nurbs()`   | 타원을 전체 NURBS 곡선으로 변환            |


## 소스 코드
```rust

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub plane: Plane,
    pub radius_x: f64,
    pub radius_y: f64,
    pub edge_index: i32,
    pub from_boolean_intersection: bool,
}
```
```rust
impl Ellipse {
    // -------- Creator --------
    pub fn new(center: Point, rx: f64, ry: f64) -> Result<Self, &'static str> {
        let mut pl = Plane::new();
        pl.origin = center;
        pl.x_axis = Vector::world_x();
        pl.y_axis = Vector::world_y();
        pl.z_axis = Vector::world_z();
        pl.update_equation();
        Self::from_plane(pl, rx, ry)
    }
    pub fn new_on_plane(plane: Plane, center: Point, rx: f64, ry: f64) -> Option<Self> {
        if rx.is_finite() && ry.is_finite() && rx > 1e-12 && ry > 1e-12 {
            let mut pl = plane;
            pl.origin = center;
            pl.x_axis = Vector::world_x();
            pl.y_axis = Vector::world_y();
            pl.z_axis = Vector::world_z();
            Some(Self {
                plane: pl,
                radius_x: rx,
                radius_y: ry,
                edge_index: 0,
                from_boolean_intersection: false,
            })
        } else {
            None
        }
    }
    pub fn from_xy(x: f64, y: f64, z: f64, rx: f64, ry: f64) -> Result<Self, &'static str> {
        Self::new(Point::new(x, y, z), rx, ry)
    }

    pub fn from_plane(plane: Plane, rx: f64, ry: f64) -> Result<Self, &'static str> {
        if !Self::validate(&plane, rx, ry) {
            return Err("Invalid ellipse (plane or radii).");
        }
        Ok(Self {
            plane,
            radius_x: rx,
            radius_y: ry,
            edge_index: -1,
            from_boolean_intersection: false,
        })
    }

    #[inline]
    fn validate(plane: &Plane, rx: f64, ry: f64) -> bool {
        plane.is_valid() && rx > ON_TOL12 && ry > ON_TOL12
    }

    pub fn is_valid(&self) -> bool {
        Self::validate(&self.plane, self.radius_x, self.radius_y)
    }

    #[inline]
    pub fn is_circle(&self) -> bool {
        (self.radius_x - self.radius_y).abs() <= self.radius_x.abs() * ON_TOL12
    }

    #[inline]
    pub fn center(&self) -> Point {
        self.plane.origin
    }

    /// Focal length: when a ≥ b, c = a × sqrt(1 − (b/a)²)
    pub fn focal_distance(&self) -> f64 {
        let (a, b) = if self.radius_x.abs() >= self.radius_y.abs() {
            (self.radius_x.abs(), self.radius_y.abs())
        } else {
            (self.radius_y.abs(), self.radius_x.abs())
        };
        if a <= 0.0 {
            return 0.0;
        }
        a * (1.0 - (b / a) * (b / a)).sqrt()
    }

    // Assume the major axis of the ellipse is plane.x_axis and the minor axis is plane.y_axis
    pub fn foci(&self) -> (Point, Point) {
        let c = self.focal_distance();
        // 주축은 더 큰 반지름에 해당하는 축
        let major = if self.radius_x >= self.radius_y {
            self.plane.x_axis
        } else {
            self.plane.y_axis
        };
        let o = self.plane.origin;
        (o + (major * c).to_point(), o - (major * c).to_point())
    }

    // -------- 평가 (점/미분/접선/곡률) --------

    /// E(t) = O + (rx cos t) * X + (ry sin t) * Y
    #[inline]
    pub fn point_at(&self, t: f64) -> Point {
        self.plane
            .point_at(self.radius_x * t.cos(), self.radius_y * t.sin())
    }

    /// k-th derivative: follows the same 4-periodic pattern
    pub fn derivative_at(&self, k: i32, t: f64) -> Vector {
        let mut r0 = self.radius_x;
        let mut r1 = self.radius_y;
        match k.abs() % 4 {
            0 => {
                r0 *= t.cos();
                r1 *= t.sin();
            }
            1 => {
                r0 *= -t.sin();
                r1 *= t.cos();
            }
            2 => {
                r0 *= -t.cos();
                r1 *= -t.sin();
            }
            3 => {
                r0 *= t.sin();
                r1 *= -t.cos();
            }
            _ => unreachable!(),
        }
        self.plane.x_axis * r0 + self.plane.y_axis * r1
    }

    #[inline]
    pub fn tangent_at(&self, t: f64) -> Vector {
        // d/dt : (-rx sin t) * X + (ry cos t) * Y
        let v = self.plane.x_axis * (-self.radius_x * t.sin())
            + self.plane.y_axis * (self.radius_y * t.cos());
        v.unitize()
    }

    /// Curvature κ(t) = T × K / r′(t)³ (reference: OpenNURBS implementation)
    pub fn curvature_value_at(&self, t: f64) -> f64 {
        let rp = self.derivative_at(1, t);
        let rpp = self.derivative_at(2, t);
        let rp_len = rp.length();
        if rp_len <= 0.0 {
            return 0.0;
        }
        let num = rp.cross(&rpp).length();
        num / (rp_len * rp_len * rp_len)
    }

    // -------- Length/Area --------

    /// Total circumference: Ramanujan's second-order approximation — fast and highly accurate
    pub fn length(&self) -> f64 {
        // a = max(rx, ry), b = min(rx, ry)
        let (a, b) = if self.radius_x.abs() >= self.radius_y.abs() {
            (self.radius_x.abs(), self.radius_y.abs())
        } else {
            (self.radius_y.abs(), self.radius_x.abs())
        };
        let h = ((a - b) * (a - b)) / ((a + b) * (a + b));
        PI * (a + b) * (1.0 + 3.0 * h / (10.0 + (4.0 - 3.0 * h).sqrt()))
    }

    /// Arc length: Simpson’s integration
    pub fn arc_length(&self, t0: f64, t1: f64) -> f64 {
        fn integrand(t: f64, rx: f64, ry: f64) -> f64 {
            (rx * rx * t.sin() * t.sin() + ry * ry * t.cos() * t.cos()).sqrt()
        }
        // n은 짝수
        let mut n = 100usize;
        if n % 2 == 1 {
            n += 1;
        }
        let h = (t1 - t0) / (n as f64);
        let mut sum = integrand(t0, self.radius_x, self.radius_y)
            + integrand(t1, self.radius_x, self.radius_y);
        for i in 1..n {
            let t = t0 + (i as f64) * h;
            if i % 2 == 0 {
                sum += 2.0 * integrand(t, self.radius_x, self.radius_y);
            } else {
                sum += 4.0 * integrand(t, self.radius_x, self.radius_y);
            }
        }
        (h / 3.0) * sum
    }

    #[inline]
    pub fn area(&self) -> f64 {
        PI * self.radius_x * self.radius_y
    }

    /// f(t) = |E(t) - P|^2, f'(t) = 2*(dy*ry*cos t - dx*rx*sin t)
    #[allow(unused)]
    fn dist2_and_grad_in_plane(rx: f64, ry: f64, px: f64, py: f64, t: f64) -> (f64, f64) {
        let (st, ct) = t.sin_cos();
        let dx = rx * ct - px;
        let dy = ry * st - py;
        let f = dx * dx + dy * dy;
        let df = 2.0 * (dy * ry * ct - dx * rx * st);
        (f, df)
    }

    /// After projecting the point onto the plane, perform 1D minimization within a quadrant-based interval to find the closest parameter
    /// - Seed: atan2
    /// - Interval: [t0, t1] chosen based on the quadrant of the projected point
    /// - Method: simplified Brent’s algorithm with safeguard

    pub fn closest_param(&self, p: &Point) -> Option<f64> {
        // 1) 타원 평면으로 투영
        let uv = self.plane.project(p); // (u,v) in plane axes
        let (px, py) = (uv.x, uv.y);
        let (rx, ry) = (self.radius_x.abs(), self.radius_y.abs());

        // 특수 케이스: 중심
        if px == 0.0 && py == 0.0 {
            // 더 큰 반지름 방향의 축에서 시작
            return Some(if rx >= ry {
                0.0
            } else {
                std::f64::consts::FRAC_PI_2
            });
        }

        // 2) 점이 타원 위인지 빠르게 체크 (허용오차 포함)
        let on_ellipse = {
            let val = (px * px) / (rx * rx) + (py * py) / (ry * ry);
            (val - 1.0).abs() <= 1e-12
        };
        if on_ellipse {
            // 정확히 위라면 그냥 atan2로 파라미터 환원
            let t = (py / ry).atan2(px / rx);
            return Some(if t < 0.0 {
                t + 2.0 * std::f64::consts::PI
            } else {
                t
            });
        }

        // 3) Solve F(λ) = 0 using Lagrange multiplier λ (monotonic ⇒ bisection method)
        // F(λ) = (rx²·px²)/(λ + rx²)² + (ry²·py²)/(λ + ry²)² − 1

        let fx = rx * rx * px * px;
        let fy = ry * ry * py * py;
        let f = |lam: f64| -> f64 {
            fx / ((lam + rx * rx) * (lam + rx * rx)) + fy / ((lam + ry * ry) * (lam + ry * ry))
                - 1.0
        };

        // Interval setup
        let mut lo;
        let mut hi;
        if (px * px) / (rx * rx) + (py * py) / (ry * ry) > 1.0 {
            // Exterior point: λ ∈ 0, +∞)
            lo = 0.0;
            // Generously estimate hi using distance and radius scaling
            hi = (px.hypot(py) + rx.max(ry)).powi(2);
        } else {
            // Interior point: λ ∈ (−min(rx², ry²), 0)
            let m = rx.min(ry);
            lo = -(m * m) + 1e-16; // 특이점 회피
            hi = 0.0;
        }

        // In case the solution is already satisfied
        let f0 = f(0.0);
        if f0.abs() <= 1e-14 {
            // 매우 가까우면
            let x = rx * rx * px / (rx * rx + 0.0);
            let y = ry * ry * py / (ry * ry + 0.0);
            let t = (y / ry).atan2(x / rx);
            return Some(if t < 0.0 {
                t + 2.0 * std::f64::consts::PI
            } else {
                t
            });
        }

        // Bisection method (monotonic)
        let mut flo = f(lo);
        let mut fhi = f(hi);

        // Guarantee: If the signs are not opposite, expand hi or shrink lo to bracket the root
        let mut expand = 0;
        while flo * fhi > 0.0 && expand < 64 {
            if f0 > 0.0 {
                // Possibility of being outside → expand hi
                hi *= 2.0;
                fhi = f(hi);
            } else {
                // Possibility of being inside → lower lo further
                lo *= 2.0;
                flo = f(lo);
            }
            expand += 1;
        }

        if flo * fhi > 0.0 {
            // Abnormal case: return safely
            // Final fallback: use circular seed from atan2
            let mut t = (py / ry).atan2(px / rx);
            if t < 0.0 {
                t += 2.0 * std::f64::consts::PI;
            }
            return Some(t);
        }

        // Core of the bisection method
        let mut mid = 0.5 * (lo + hi);
        for _ in 0..80 {
            mid = 0.5 * (lo + hi);
            let fm = f(mid);
            if fm.abs() <= 1e-14 || (hi - lo).abs() <= 1e-14 {
                break;
            }
            if flo * fm < 0.0 {
                hi = mid;
                fhi = fm;
            } else {
                lo = mid;
                flo = fm;
            }
        }

        let lam = mid;

        // 4) Restore x, y → recover parameter
        let x = rx * rx * px / (lam + rx * rx);
        let y = ry * ry * py / (lam + ry * ry);
        let mut t = (y / ry).atan2(x / rx);
        if t < 0.0 {
            t += 2.0 * std::f64::consts::PI;
        }
        Some(t)
    }

    pub fn closest_point(&self, p: &Point) -> Point {
        match self.closest_param(p) {
            Some(t) => self.point_at(t),
            None => self.center(),
        }
    }

    #[inline]
    pub fn domain(&self) -> Interval {
        Interval::new(0.0, TAU)
    }

    /// Divide the domain [0, 2π] by parameter t (only for valid t values)
    pub fn split_at(&self, t: f64) -> Option<(EllipticalArc, EllipticalArc)> {
        if t <= 0.0 || t >= 2.0 * PI {
            return None;
        }
        let left = EllipticalArc::new(self.plane, self.radius_x, self.radius_y, 0.0, t);
        let right = EllipticalArc::new(self.plane, self.radius_x, self.radius_y, t, 2.0 * PI);
        Some((left, right))
    }

    pub fn sub_curve(&self, t0: f64, t1: f64) -> Option<EllipticalArc> {
        if (t1 - t0).abs() >= 2.0 * PI - 1e-15 {
            return None;
        }
        Some(EllipticalArc::new(
            self.plane,
            self.radius_x,
            self.radius_y,
            t0,
            t1,
        ))
    }

    // -------- Transformation --------
    /// Apply affine transformation:
    /// 1) Transform the plane, origin, and axes
    /// 2) Radius scaling:
    ///    - If uniformly scaled → apply the same factor to both rx and ry
    ///    - If non-uniform → compute scale factors sx and sy along the two in-plane axes
    ///      If sx ≈ sy, treat as quasi-uniform and scale by their average
    pub fn transform_by(&mut self, xform: &Transform) {
        // (1) 평면 변환
        let old = self.plane.clone();
        self.plane = self.plane.transform(xform); // Plane::transformed 구현에 맞게

        // (2) 스케일 판정
        let eps = 1e-12;
        if xform.is_uniform_scale(eps) {
            let s = xform.scale_factor_x(); // 임의축 동일
            self.radius_x *= s;
            self.radius_y *= s;
            return;
        }

        // Approximate using axis-aligned scaling on the plane
        // Transform the previous plane's axes and compare their lengths
        let sx = xform.apply_vector(old.x_axis).length();
        let sy = xform.apply_vector(old.y_axis).length();

        if (sx - sy).abs() <= 1e-9 {
            let s = 0.5 * (sx + sy);
            self.radius_x *= s;
            self.radius_y *= s;
        } else {
            // True anisotropic scale/shear → transforms the plane without changing the radius (may distort conic sections)
            // If needed, implement additional logic here to decompose a general 2D linear transform into radius and axis components
        }
    }

    pub fn is_planar(&self, _tol: f64) -> Option<Plane> {
        Some(self.plane)
    }

    pub fn is_in_plane(&self, test_plane: &Plane, tolerance: f64) -> bool {
        if !self.is_valid() {
            return false;
        }

        // Representative 3 points: t = 0, 2π⁄3, 4π⁄3
        let ts = [0.0, 2.0 * PI / 3.0, 4.0 * PI / 3.0];
        for &t in &ts {
            let p = self.point_at(t);
            let d = test_plane.distance_to(&p).abs();
            if d.abs() > tolerance {
                return false;
            }
        }
        true
    }

    pub fn is_point_inside(&self, test_point: &Point) -> bool {
        // 2D coordinates projected onto the plane (local to plane)
        let uv_start = self.plane.project(&self.start_point()); // StartPoint 투영
        let uv_p = self.plane.project(test_point);

        // u: (0,0)→StartPoint_proj, v: (0,0)→testPoint_proj
        let _u = Vector2::new(uv_start.x, uv_start.y); // 기준방향
        let v = Vector2::new(uv_p.x, uv_p.y); // 시험점 방향

        // θ = SignedAngle(u, v). Recover ellipse parameter t from θ:
        // Replace GEllipticalArc.GetParam(rx, ry, θ) with the following formula:
        // t = atan2(v.y / ry, v.x / rx)

        let t = (v.y / self.radius_y).atan2(v.x / self.radius_x);

        // Ellipse point a corresponding to the parameter
        let a = self.point_at(t);
        let o = self.center();

        // Compare v² with a − O²
        let len_v2 = v.x * v.x + v.y * v.y;
        let rad2 =
            (a.x - o.x) * (a.x - o.x) + (a.y - o.y) * (a.y - o.y) + (a.z - o.z) * (a.z - o.z);
        len_v2 < rad2
    }

    pub fn is_linear(&self, _tol: f64) -> (bool, Option<Segment3D>) {
        (false, None)
    }

    #[inline]
    pub fn point_on_ellipse_at(t: f64, plane: &Plane, r0: f64, r1: f64) -> Point {
        plane.point_at(r0 * t.cos(), r1 * t.sin())
    }

    #[inline]
    pub fn get_vector(t: f64, plane: &Plane, r0: f64, r1: f64) -> Vector {
        Self::derive_at_static(1, t, plane, r0, r1)
    }

    pub fn derive_at_static(k: i32, t: f64, plane: &Plane, mut r0: f64, mut r1: f64) -> Vector {
        match k.abs() % 4 {
            0 => {
                r0 *= t.cos();
                r1 *= t.sin();
            }
            1 => {
                r0 *= -t.sin();
                r1 *= t.cos();
            }
            2 => {
                r0 *= -t.cos();
                r1 *= -t.sin();
            }
            3 => {
                r0 *= t.sin();
                r1 *= -t.cos();
            }
            _ => {}
        }
        plane.x_axis * r0 + plane.y_axis * r1
    }

    pub fn normal_at(&self, t: f64) -> Vector {
        // 두 번째 도함수: (-rx cos t) * X + (-ry sin t) * Y
        let v2 = self.plane.x_axis * (-self.radius_x * t.cos())
            + self.plane.y_axis * (-self.radius_y * t.sin());
        v2.unitize()
    }

    pub fn start_tangent(&self) -> Vector {
        let mut v = Self::get_vector(0.0, &self.plane, self.radius_x, self.radius_y);
        let _ = v.normalize();
        v
    }
    pub fn end_tangent(&self) -> Vector {
        let mut v = Self::get_vector(0.0, &self.plane, self.radius_x, self.radius_y);
        let _ = v.normalize();
        v
    }
    #[inline]
    pub fn start_point(&self) -> Point {
        self.point_at(0.0)
    }
    #[inline]
    pub fn end_point(&self) -> Point {
        self.point_at(PI * 2.0)
    }

    /// 점을 평면에 사영 → 타원 파라미터 추정
    pub fn project(&self, point: Point, t_out: &mut f64) -> bool {
        // 평면으로 투영해서 (u,v) → 극각 theta → 타원파라미터
        let (u, v) = self.plane.project_st(point);
        if on_are_equal(u, 0.0, 0.0) && on_are_equal(v, 0.0, 0.0) {
            *t_out = 0.0;
            return true;
        }
        let mut theta = v.atan2(u); // -π..π
        theta = if theta < 0.0 { theta + TAU } else { theta };
        let t = Self::param_from_polar(self.radius_x, self.radius_y, theta);
        *t_out = t;
        true
    }

    /// 타원 θ(극각)→파라미터 t 변환. (C# GetParam 유사: 사분면 보정)
    pub fn param_from_polar(rx: f64, ry: f64, theta: f64) -> f64 {
        // t = atan( (rx/ry) * tan(theta) ), 사분면에 맞춰 보정
        let mut theta_n = theta;
        while theta_n < 0.0 {
            theta_n += TAU;
        }
        while theta_n > TAU {
            theta_n -= TAU;
        }

        let mut t = ((rx / ry) * theta_n.tan()).atan();
        // 축 위의 특수각은 그대로 사용
        if on_are_equal(theta_n, 0.0, 1e-15)
            || on_are_equal(theta_n, std::f64::consts::FRAC_PI_2, 1e-15)
            || on_are_equal(theta_n, std::f64::consts::PI, 1e-15)
            || on_are_equal(theta_n, 3.0 * std::f64::consts::FRAC_PI_2, 1e-15)
            || on_are_equal(theta_n, TAU, 1e-15)
        {
            t = theta_n;
        } else if theta_n > std::f64::consts::FRAC_PI_2
            && theta_n < 3.0 * std::f64::consts::FRAC_PI_2
        {
            t += std::f64::consts::PI;
        } else if theta_n > 3.0 * std::f64::consts::FRAC_PI_2 && theta_n < TAU {
            t += TAU;
        }
        t
    }

    /// length ∈ [0, total] → parameter u (증가 도메인 가정)
    pub fn get_param_from_length(&self, length: f64) -> (bool, f64) {
        let d = self.domain();
        let total = self.length();
        if on_are_equal_scaled(length, 0.0, total) {
            return (true, d.t0);
        }
        if on_are_equal_scaled(length, total, total) {
            return (true, d.t1);
        }
        if length < 0.0 || length > total {
            return (false, d.t0);
        }

        // 증가 도메인으로 고정해 놓고 이분 탐색 (비교 길이는 항상 'a->mid')
        let a = d.t0.min(d.t1);
        let b = d.t0.max(d.t1);
        let mut lo = a;
        let mut hi = b;

        // 허용오차 (길이 스케일 기준)
        let tol_len = 1e-12 * (1.0 + total.abs());

        for _ in 0..64 {
            let mid = 0.5 * (lo + hi);
            let lmid = self.arc_length(a, mid); // 항상 a에서 mid까지!

            if (lmid - length).abs() <= tol_len {
                // 감소 도메인이면 mid를 반사해서 되돌림
                let u = if d.t0 <= d.t1 { mid } else { a + b - mid };
                return (true, u);
            }
            if lmid < length {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        // 최대 반복 후 중간값 반환
        let mid = 0.5 * (lo + hi);
        let u = if d.t0 <= d.t1 { mid } else { a + b - mid };
        (true, u)
    }

    /// parameter u → length from domain.t0
    pub fn get_length_from_param(&self, u: f64) -> (bool, f64) {
        let dom = self.domain();
        if u < dom.t0 || u > dom.t1 {
            return (false, 0.0);
        }
        (true, self.arc_length(dom.t0, u))
    }

    /// 도메인 반전: 파라미터 진행 방향 역전
    pub fn reverse(&mut self) {
        self.plane.y_axis = -self.plane.y_axis;
        self.plane.update_equation();
    }

    pub fn to_nurbs(&self) -> Curve {
        ellipse_arc_to_nurbs(
            &self.plane,
            self.radius_x,
            self.radius_y,
            0.0,
            2.0 * PI,
            true,
        )
    }
}
```
---

# 📘 EllipticalArc 구조 문서 요약
## 🔧 구조 정의
```rust
pub struct EllipticalArc {
    pub plane: Plane,       // 타원이 놓인 평면
    pub radius_x: f64,      // X축 반지름
    pub radius_y: f64,      // Y축 반지름
    pub t0: f64,            // 시작 파라미터
    pub t1: f64,            // 종료 파라미터
}
```

## 🏗️ 생성자
| 메서드명                         | 설명                                               |
|----------------------------------|----------------------------------------------------|
| `new(plane, rx, ry, t0, t1)`     | 평면과 반지름, 파라미터 구간으로 직접 생성         |
| `new_end(center, rx, ry, end)`   | 중심점과 반지름으로 생성 후 0~end까지 호 구성       |
| `new_on_plane(...)`              | 평면 기반으로 타원 생성 후 호 구성                 |
| `new_on_ellipse(ellipse, ang)`   | 기존 타원에서 파라미터 구간으로 호 생성            |
| `from_polar_angles(...)`         | 극각 기반으로 파라미터 변환 후 호 생성             |

## 📐 평가 및 속성
| 메서드명             | 설명                                           |
|----------------------|------------------------------------------------|
| `domain()`           | 파라미터 구간 (t0, t1) 반환                    |
| `point_at(t)`        | 파라미터 t에 해당하는 타원 호의 점 계산        |
| `tangent_at(t)`      | 접선 벡터 계산                                 |
| `normal_at(t)`       | 법선 벡터 계산                                 |
| `is_valid()`         | 반지름과 파라미터 구간의 유효성 검사           |

## 📏 길이 및 면적
### 1. 호 길이 계산

$$
L=\int _{t_0}^{t_1}\sqrt{(rx\cdot \sin t)^2+(ry\cdot \cos t)^2}\, dt
$$

- arc_length(a, b)에서 사용됨
- length()는 t0 → t1 방향에 따라 자동 계산

### 2. 면적 계산 (선적분)

$$
A=\frac{1}{2}\int _{t_0}^{t_1}(x\cdot dy-y\cdot dx)\, dt
$$

- area()에서 사용됨
- Simpson's Rule로 근사 계산

## 🔁 길이 ↔ 파라미터 변환
| 메서드명                     | 설명                                               |
|------------------------------|----------------------------------------------------|
| `get_param_from_length(len)` | 주어진 길이에 해당하는 파라미터 t 계산 (이분 탐색) |
| `get_length_from_param(u)`   | 파라미터 u에 해당하는 시작점부터의 길이 계산       |


## ✂️ 분할 및 반전
| 메서드명             | 설명                                           |
|----------------------|------------------------------------------------|
| `split_at(t)`        | 파라미터 t 기준으로 두 개의 호로 분할          |
| `reverse()`          | 파라미터 방향 반전 및 평면 Y축 반전             |


## 🔄 NURBS 변환
| 메서드명       | 설명                                      |
|----------------|-------------------------------------------|
| `to_nurbs()`   | 타원 호를 NURBS 곡선으로 변환              |


## ✅ 수학적 특징 요약
- 타원 점 계산:

$$
P(t)=O+rx\cdot \cos (t)\cdot X+ry\cdot \sin (t)\cdot Y
$$

- 접선 벡터:

$$
T(t)=-rx\cdot \sin (t)\cdot X+ry\cdot \cos (t)\cdot Y
$$

- 법선 벡터:

$$
N(t)=-rx\cdot \cos (t)\cdot X-ry\cdot \sin (t)\cdot Y
$$

- 길이 적분:

$$
\int \sqrt{(rx\cdot \sin t)^2+(ry\cdot \cos t)^2}\, dt
$$


- 면적 적분:

$$
\frac{1}{2}\int (x\cdot dy-y\cdot dx)\, dt
$$


## 소스 코드
```rust
#[derive(Debug, Clone)]
pub struct EllipticalArc {
    pub plane: Plane,
    pub radius_x: f64,
    pub radius_y: f64,
    pub t0: f64,
    pub t1: f64,
}
```
```rust
impl EllipticalArc {
    pub fn new(plane: Plane, rx: f64, ry: f64, t0: f64, t1: f64) -> Self {
        Self {
            plane,
            radius_x: rx,
            radius_y: ry,
            t0,
            t1,
        }
    }

    pub fn new_end(center: Point, rx: f64, ry: f64, end_param: f64) -> Option<Self> {
        let el = Ellipse::new(center, rx, ry).unwrap();
        Self::new_on_ellipse(el, Interval::new(0.0, end_param))
    }

    pub fn new_on_plane(
        plane: Plane,
        center: Point,
        rx: f64,
        ry: f64,
        t0: f64,
        t1: f64,
    ) -> Option<Self> {
        let el = Ellipse::new_on_plane(plane, center, rx, ry)?;
        Self::new_on_ellipse(el, Interval::new(t0, t1))
    }

    pub fn new_on_ellipse(ellipse: Ellipse, mut ang: Interval) -> Option<Self> {
        if ang.is_decreasing() {
            ang.swap();
            let mut e = ellipse;
            e.reverse();
            let arc = Self {
                plane: e.plane,
                radius_x: e.radius_x,
                radius_y: e.radius_y,
                t0: ang.t0,
                t1: ang.t1,
            };
            return if arc.is_valid() { Some(arc) } else { None };
        }

        // 2π 초과 컷
        if ang.length() > TAU {
            ang.t1 = ang.t0 + TAU;
        }
        let arc = Self {
            plane: ellipse.plane,
            radius_x: ellipse.radius_x,
            radius_y: ellipse.radius_y,
            t0: ang.t0,
            t1: ang.t1,
        };
        if arc.is_valid() { Some(arc) } else { None }
    }

    #[inline]
    pub fn domain(&self) -> (f64, f64) {
        (self.t0, self.t1)
    }
    #[inline]
    pub fn point_at(&self, t: f64) -> Point {
        self.plane
            .point_at(self.radius_x * t.cos(), self.radius_y * t.sin())
    }
    pub fn length(&self) -> f64 {
        let (t0, t1) = (self.t0, self.t1);
        if on_are_equal(self.t0, self.t1, 0.0) {
            return 0.0;
        } // dummy to signal use; next line is real:

        if t1 >= t0 {
            self.arc_length(t0, t1)
        } else {
            self.arc_length(t1, t0)
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        let ang = Interval::new(self.t0, self.t1);
        self.radius_x > 1e-12
            && self.radius_y > 1e-12
            && self.radius_x.is_finite()
            && self.radius_y.is_finite()
            && ang.length() > 1e-12
            && ang.length() <= TAU + 1e-12
    }

    /// 구간 [a, b] 길이: ∫ sqrt((rx sin t)^2 + (ry cos t)^2) dt
    pub fn arc_length(&self, a: f64, b: f64) -> f64 {
        if !self.is_valid() {
            return 0.0;
        }
        let rx2 = self.radius_x * self.radius_x;
        let ry2 = self.radius_y * self.radius_y;
        let speed = |t: f64| (rx2 * t.sin().powi(2) + ry2 * t.cos().powi(2)).sqrt();
        let length = on_adaptive_simpson(&speed, a, b, 1e-10, 20);
        length
    }

    pub fn area(&self) -> f64 {
        // ∫ (xdy − ydx)/2 : can be improved using Gauss–Legendre; currently using Simpson’s rule
        fn integrand(theta: f64, a: f64, b: f64) -> f64 {
            let x = a * theta.cos();
            let y = b * theta.sin();
            let dx = -a * theta.sin();
            let dy = b * theta.cos();
            x * dy - y * dx
        }
        let mut n = 100usize;
        if n % 2 == 1 {
            n += 1;
        }
        let h = (self.t1 - self.t0) / (n as f64);
        let mut sum = integrand(self.t0, self.radius_x, self.radius_y)
            + integrand(self.t1, self.radius_x, self.radius_y);
        for i in 1..n {
            let t = self.t0 + (i as f64) * h;
            if i % 2 == 0 {
                sum += 2.0 * integrand(t, self.radius_x, self.radius_y);
            } else {
                sum += 4.0 * integrand(t, self.radius_x, self.radius_y);
            }
        }
        0.5 * (h / 3.0) * sum
    }

    #[inline]
    pub fn tangent_at(&self, t: f64) -> Vector {
        // d/dt : (-rx sin t) * X + (ry cos t) * Y
        let v = self.plane.x_axis * (-self.radius_x * t.sin())
            + self.plane.y_axis * (self.radius_y * t.cos());
        v.unitize()
    }

    pub fn normal_at(&self, t: f64) -> Vector {
        // 두 번째 도함수: (-rx cos t) * X + (-ry sin t) * Y
        let v2 = self.plane.x_axis * (-self.radius_x * t.cos())
            + self.plane.y_axis * (-self.radius_y * t.sin());
        v2.unitize()
    }

    pub fn get_param_from_length(&self, length: f64) -> (bool, f64) {
        let total = self.length();
        if on_are_equal_scaled(length, 0.0, total) {
            return (true, self.t0);
        }
        if on_are_equal_scaled(length, total, total) {
            return (true, self.t1);
        }
        if length < 0.0 || length > total {
            return (false, self.t0);
        }

        // 증가 도메인으로 고정해 놓고 이분 탐색 (비교 길이는 항상 'a->mid')
        let a = self.t0.min(self.t1);
        let b = self.t0.max(self.t1);
        let mut lo = a;
        let mut hi = b;

        // 허용오차 (길이 스케일 기준)
        let tol_len = 1e-12 * (1.0 + total.abs());

        for _ in 0..64 {
            let mid = 0.5 * (lo + hi);
            let lmid = self.arc_length(a, mid); // 항상 a에서 mid까지!

            if (lmid - length).abs() <= tol_len {
                // 감소 도메인이면 mid를 반사해서 되돌림
                let u = if self.t0 <= self.t1 { mid } else { a + b - mid };
                return (true, u);
            }
            if lmid < length {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        // 최대 반복 후 중간값 반환
        let mid = 0.5 * (lo + hi);
        let u = if self.t0 <= self.t1 { mid } else { a + b - mid };
        (true, u)
    }

    /// parameter u → length from domain.t0 along domain 방향
    pub fn get_length_from_param(&self, u: f64) -> (bool, f64) {
        let increasing = self.t1 >= self.t0;
        let in_domain = if increasing {
            u >= self.t0 && u <= self.t1
        } else {
            u <= self.t0 && u >= self.t1
        };
        if !in_domain {
            return (false, 0.0);
        }
        let len = if increasing {
            self.arc_length(self.t0, u)
        } else {
            self.arc_length(u, self.t0)
        };
        (true, len.abs())
    }

    pub fn split_at(&self, t: f64) -> Option<(EllipticalArc, EllipticalArc)> {
        let increasing = self.t1 >= self.t0;
        let in_domain = if increasing {
            t > self.t0 + ON_TOL12 && t < self.t1 - ON_TOL12
        } else {
            t < self.t0 - ON_TOL12 && t > self.t1 + ON_TOL12
        };
        if !in_domain {
            return None;
        }
        let a = EllipticalArc::new(self.plane, self.radius_x, self.radius_y, self.t0, t);
        let b = EllipticalArc::new(self.plane, self.radius_x, self.radius_y, t, self.t1);
        Some((a, b))
    }
    pub fn from_polar_angles(
        plane: Plane,
        center: Point,
        rx: f64,
        ry: f64,
        start_angle: f64,
        end_angle: f64,
        polar_angles: bool,
    ) -> Option<Self> {
        let el = Ellipse::new_on_plane(plane, center, rx, ry)?;
        if !polar_angles {
            return EllipticalArc::new_on_ellipse(el, Interval::new(start_angle, end_angle));
        }
        // polar → param
        let t0 = Ellipse::param_from_polar(rx, ry, start_angle);
        let t1 = Ellipse::param_from_polar(rx, ry, end_angle);

        let mut a = t0;
        let mut b = t1;
        let mut flipped = false;
        if b < a {
            std::mem::swap(&mut a, &mut b);
            flipped = true;
        }
        let mut ang = Interval::new(a, b);
        if ang.length() > TAU {
            ang.t1 = ang.t0 + TAU;
        }

        let mut arc = EllipticalArc {
            plane: el.plane,
            radius_x: el.radius_x,
            radius_y: el.radius_y,
            t0: ang.t0,
            t1: ang.t1,
        };
        if flipped {
            arc.reverse();
        }

        if arc.is_valid() { Some(arc) } else { None }
    }

    pub fn reverse(&mut self) {
        self.plane.y_axis = -self.plane.y_axis;
        self.plane.update_equation();
        mem::swap(&mut self.t0, &mut self.t1);
    }

    pub fn to_nurbs(&self) -> Curve {
        ellipse_arc_to_nurbs(
            &self.plane,
            self.radius_x,
            self.radius_y,
            self.t0,
            self.t1,
            false,
        )
    }
}

```




# 🧠 전체 흐름 요약
```
on_ellipse_arc_to_nurbs()
 ├─ on_build_circle_arc_quadratic_segments() → 단위 원호를 rational Bézier로 분할
 ├─ on_embed_ellipse_on_plane() → 단위 원을 (rx, ry) 스케일 후 평면에 임베딩
 └─ Curve 생성 → NURBS 곡선으로 반환
```


## 🔢 1. on_build_circle_arc_quadratic_segments()
### 🎯 목적
단위 원호 구간 [θ₀, θ₁]을 rational quadratic Bézier 곡선들로 근사하여 NURBS 구성 요소 생성
### 📐 수학적 배경
- Rational Bézier로 원호 근사
    - 원호는 일반 Bézier로는 정확히 표현 불가
    - 하지만 rational quadratic Bézier는 다음 조건에서 정확히 표현 가능:
    - 각 세그먼트의 중심각 Δ ≤ π⁄2
    - 중심점 weight = cos(Δ⁄2)
- 제어점 구성
    - Δ = θ₂ − θ₀
    - θ₁ = (θ₀ + θ₂)⁄2
    - weight(P₁) = cos(Δ⁄2)
### 🧮 Knot Vector 구성
- degree = 2 → quadratic
- n개 세그먼트 → 내부 knot 중복으로 연결
- 형태: [0,0,0, t₁,t₁, t₂,t₂, ..., 1,1,1]

## 🧭 2. on_embed_ellipse_on_plane()
### 🎯 목적
단위 원상의 제어점들을 (rx, ry) 스케일 후 주어진 평면에 3D로 임베딩
### 📐 수학적 배경
- 단위 원 점 (x, y) → 타원 점 (rx·x, ry·y)
- 평면 임베딩:

$$
P=O+rx\cdot x\cdot \vec {X}+ry\cdot y\cdot \vec {Y}
$$

- O: 평면의 origin
- X, Y: 평면의 x축, y축
- w: rational weight 유지

## 🧩 3. on_ellipse_arc_to_nurbs()
### 🎯 목적
타원호를 NURBS 곡선으로 변환
### 📐 수학적 배경
- Rational Bézier → NURBS로 표현 가능
- 타원은 원의 affine 변환이므로:
- 단위 원호 → rational Bézier로 근사
- (rx, ry) 스케일 → 타원
- 평면에 임베딩 → 3D 곡선

### ✅ 결과 구조: Curve
Curve {
    dimension: 3,
    degree: 2,
    knots: KnotVector{...},
    ctrl: Vec<CPoint>, // 제어점 + weight
    domain: Interval { t0, t1 },
}
- 정확한 타원호 표현 가능
- CAD/NURBS 시스템에서 호를 부드럽게 연결 가능

## 소스 코드
```rust

fn on_build_circle_arc_quadratic_segments(
    theta0: f64,
    theta1: f64,
) -> (Vec<f64>, Vec<[f64; 3]>, Vec<f64>, usize) {
    // 원호를 Δ<=π/2 조각으로 분할하여 quadratic rational Bezier 들을 이어붙임
    let mut th0 = theta0;
    let mut th1 = theta1;
    if th1 < th0 {
        std::mem::swap(&mut th0, &mut th1);
    }
    let total = th1 - th0;
    let n_segs = (total / (PI / 2.0)).ceil() as usize;
    let n_segs = n_segs.max(1);
    let dtheta = total / (n_segs as f64);

    // degree = 2
    let p = 2;

    // Knot vector: [0,0,0, t1,t1, t2,t2, ..., 1,1,1] (내부 각 ti=(i/n_segs))
    let mut knots = Vec::with_capacity(3 + 2 * (n_segs - 1) + 3);
    knots.extend([0.0, 0.0, 0.0]);
    if n_segs > 1 {
        for i in 1..n_segs {
            let ti = (i as f64) / (n_segs as f64);
            knots.push(ti);
            knots.push(ti);
        }
    }
    knots.extend([1.0, 1.0, 1.0]);

    // Control points & weights in 2D (XY-plane, z=0): 각 세그먼트마다 2*1+1=3개의 베지어 CP
    // 연결 시 내부 점은 공유되므로 총 CP 개수 = 2 * n_segs + 1
    let mut cpts_xy: Vec<[f64; 3]> = Vec::with_capacity(2 * n_segs + 1);
    let mut wts: Vec<f64> = Vec::with_capacity(2 * n_segs + 1);

    let mut a0 = th0;
    for s in 0..n_segs {
        let a2 = if s + 1 == n_segs {
            th1
        } else {
            th0 + (s as f64 + 1.0) * dtheta
        };
        let a1 = 0.5 * (a0 + a2);
        let w = (0.5 * (a2 - a0)).cos(); // cos(Δ/2)

        let p0 = [a0.cos(), a0.sin(), 0.0];
        let pm = [a1.cos() / w, a1.sin() / w, 0.0]; // Cartesian control = unit circle / w
        let p2 = [a2.cos(), a2.sin(), 0.0];

        if s == 0 {
            cpts_xy.push(p0);
            wts.push(1.0);
        }
        cpts_xy.push(pm);
        wts.push(w);
        cpts_xy.push(p2);
        wts.push(1.0);

        a0 = a2;
    }
    (knots, cpts_xy, wts, p)
}

fn on_embed_ellipse_on_plane(
    plane: &Plane,
    rx: f64,
    ry: f64,
    c_pts: &[[f64; 3]],
    wts: &[f64],
) -> Vec<CPoint> {
    // (x,y,0) -> (rx*x, ry*y, 0) -> plane 임베딩
    let mut out = Vec::with_capacity(c_pts.len());
    for (i, p) in c_pts.iter().enumerate() {
        let x = rx * p[0];
        let y = ry * p[1];
        let world = plane.origin + (plane.x_axis * x + plane.y_axis * y).to_point();
        out.push(CPoint {
            x: world.x,
            y: world.y,
            z: world.z,
            w: wts[i],
        });
    }
    out
}


pub fn on_ellipse_arc_to_nurbs(
    plane: &Plane,
    rx: f64,
    ry: f64,
    t0: f64,
    t1: f64,
    _is_closed: bool,
) -> Curve {
    // 1) 원 단위에서 원호 NURBS 를 만들고
    let (knots, c_pts, wts, degree) = on_build_circle_arc_quadratic_segments(t0, t1);
    // 2) (rx, ry) 스케일 후 plane 에 임베딩
    let ctrl = on_embed_ellipse_on_plane(plane, rx, ry, &c_pts, &wts);

    Curve {
        dimension: 3,
        degree : degree as Degree,
        knots: KnotVector{knots},
        ctrl,
        domain: Interval { t0, t1 },
    }
}

```

# 수식 검증

전체 Ellipse, EllipticalArc, 그리고 on_ellipse_arc_to_nurbs() 관련 소스를 수학적으로 검토한 결과를 아래에 정리했습니다.  
주요 수식, 알고리즘, 근사 방식, 경계 조건 등을 기준으로 정확성, 안정성, 경계 처리, 수치적 타당성을 평가했습니다.  

## ✅ 전반적 평가
- 전체적으로 수학적으로 정확하고 안정적인 구현입니다.
- 타원 및 타원호의 기하적 정의, 도함수, 곡률, 길이, 면적, 최단점 계산, NURBS 변환 등에서 정확한 수식과 알고리즘이 사용되었습니다.
- 특히 closest_param()의 Lagrange multiplier 기반 최적화, arc_length()의 Simpson 적분, ellipse_arc_to_nurbs()의 rational Bézier 근사 등은 고급 수치 기법을 잘 활용하고 있습니다.

## 🔍 세부 검증 결과
### 1. 타원 점/도함수/곡률 계산
- point_at(t), derivative_at(k, t), curvature_value_at(t) 등은 모두 수학적으로 정확합니다.
- 곡률 공식:

$$
\kappa (t)=\frac{|\mathbf{r}'(t)\times \mathbf{r}''(t)|}{|\mathbf{r}'(t)|^3}
$$

- 외적과 벡터 길이 계산이 올바르게 구현되어 있습니다. ✅ 정확

### 2. Ramanujan 둘레 근사
- 사용된 Ramanujan 근사식:

$$
L\approx \pi (a+b)\left( 1+\frac{3h}{10+\sqrt{4-3h}}\right) \quad \mathrm{where}\quad h=\frac{(a-b)^2}{(a+b)^2}
$$

- 고전적인 둘레 근사식으로 정확하고 빠르며, 수치적으로 안정적입니다. ✅ 정확

### 3. Simpson 적분 기반 길이/면적
- arc_length()와 area()는 Simpson's Rule을 사용하여 다음을 근사:

$$
L=\int _{t_0}^{t_1}\sqrt{(rx\cdot \sin t)^2+(ry\cdot \cos t)^2}\, dt
A=\frac{1}{2}\int (x\cdot dy-y\cdot dx)\, dt
$$

- 적분 함수와 가중치 적용이 정확하며, 짝수 분할 조건도 잘 처리됨. ✅ 정확

### 4. Lagrange 기반 최단점 계산
- closest_param()에서 사용된 목적 함수:

$$
F(\lambda )=\frac{rx^2\cdot px^2}{(\lambda +rx^2)^2}+\frac{ry^2\cdot py^2}{(\lambda +ry^2)^2}-1
$$

- 외접/내접 여부에 따라 λ의 탐색 구간을 설정하고, 이분법으로 해를 찾는 방식은 수치적으로 안정적이며 수학적으로 타당합니다. ✅ 정확

### 5. 극각 → 파라미터 변환
- param_from_polar()에서 사용된 변환:

$$
t=\tan ^{-1}\left( \frac{rx}{ry}\cdot \tan (\theta )\right)
$$

- 사분면 보정 및 특수각 처리도 잘 되어 있으며, 타원 위의 극각을 파라미터로 환산하는 방식으로 적절합니다. ✅ 정확

### 6. NURBS 변환
- on_build_circle_arc_quadratic_segments()에서 사용된 rational quadratic Bézier 근사:

$$
\mathrm{weight}=\cos (\Delta /2)
$$

- 중심각 Δ ≤ π/2 조건을 만족하도록 분할하고, 제어점과 weight 계산이 정확합니다.
- embed_ellipse_on_plane()에서 단위 원을 (rx, ry) 스케일 후 평면에 임베딩하는 방식도 수학적으로 타당합니다. ✅ 정확

## ⚠️ 경계 처리 및 개선 여지
| 함수명              | 문제 지점        | 설명                                                                 |
|---------------------|------------------|----------------------------------------------------------------------|
| `closest_param()`   | `expand` 루프    | λ 탐색 시 f(λ)의 부호가 같을 경우 최대 64회 반복 후 fallback 발생 가능성 있음 |
| `param_from_polar()`| 사분면 보정      | θ → t 변환 시 사분면별 보정 로직이 복잡하며, 일부 극각에서 불연속 가능성 있음 |
| `reverse()`         | 평면 반전 방식   | Y축 반전 + t0/t1 swap은 방향 반전에는 충분하지만, 평면 정규화가 필요한 경우 있음 |

## 🔧 개선 제안
### 1. closest_param() 개선
- 문제: expand 루프가 64회 제한으로 fallback 발생 가능
- 제안:
    - expand 시 log-scale로 hi 증가 (예: hi *= 1.5)
    - fallback 전에 f(λ)의 기울기 또는 근사 root 위치 예측 추가

### 2. param_from_polar() 개선
- 문제: 사분면 보정이 조건문으로 복잡하게 처리됨
- 제안:
    - atan2(ry·sinθ, rx·cosθ) 방식으로 직접 파라미터 계산 가능
    - 또는 Ellipse::closest_param() 내부 로직 재사용

### 3. reverse() 개선
- 문제: 평면의 Y축만 반전 → 일부 시스템에서 법선 벡터 정규화 필요
- 제안:
    - plane.normal도 함께 반전하거나 plane.update_equation()에서 보장
    - Plane::reverse() 같은 메서드로 캡슐화하면 명확성 향상

## ✅ 결론
전체적으로 수학적으로 정확하며, CAD/NURBS 시스템에서 안정적으로 동작할 수 있는 구현입니다.  
수치적 안정성, 경계 처리, 근사 방식 모두 잘 설계되어 있으며, 고급 기법들이 적절히 사용되었습니다.  


---
# 테스트 진행

✅ GEllipse & GEllipticalArc 테스트 요약
| 테스트 함수명                                      | 검증 대상 / 설명                                                   |
|---------------------------------------------------|--------------------------------------------------------------------|
| `construct_and_validity`                          | 타원 생성, 유효성, 중심 좌표 확인                                  |
| `point_and_tangent_param`                         | 점 평가 및 접선 방향 확인                                          |
| `perimeter_and_arc_length`                        | 전체 둘레와 호 길이 일치 여부                                      |
| `foci_sum_distance_constant`                      | 초점 거리 합이 2a인지 확인                                         |
| `closest_param_on_axes`                           | 축 위 점에 대한 파라미터 정확성 확인                               |
| `closest_point_distance_is_minimal`               | 최근접점이 실제 최소 거리인지 확인                                 |
| `split_and_sub_curve_length_consistency`          | 분할 후 길이 합과 부분 곡선 길이 정확성 확인                       |
| `transform_uniform_scale`                         | 균일 스케일 변환 후 반지름 및 중심 확인                            |
| `transform_translate_and_rotate_axis`             | 평면 이동 및 회전 후 중심과 반지름 유지 여부                      |
| `arc_area_simple_checks`                          | 타원호 면적이 전체 면적의 비율과 일치하는지 확인                  |
| `is_in_plane_basic`                               | 평면 포함 여부 및 공차에 따른 판정 확인                            |
| `is_point_inside_basic`                           | 점이 타원 내부에 포함되는지 여부 확인                              |
| `point_on_ellipse_and_get_vector_match_instance_impls` | 정적 API와 인스턴스 API 결과 일치 여부 확인              |
| `derive_at_static_cycle_matches`                  | 도함수 4주기 패턴 및 인스턴스 도함수와 일치 여부 확인             |
| `normal_at_and_tangents`                          | 법선 벡터 길이 및 접선과의 관계 확인                               |
| `is_point_inside_matches_implict_equation_sign`   | 암시적 타원 방정식과 포함 판정 결과 일치 여부 확인                |
| `ellipse_point_and_tangent_xy`                    | 타원 점과 접선 방향 확인 (XY 평면 기준)                            |
| `ellipse_project_basic`                           | 평면 투영 후 파라미터 추정 정확성 확인                            |
| `ellipse_length_param_roundtrip_mid`              | 길이 ↔ 파라미터 왕복 변환 정확성 확인                             |
| `arc_basic_and_endpoints`                         | 타원호 생성 및 끝점 정확성 확인                                    |
| `arc_param_length_forward`                        | 타원호 길이 ↔ 파라미터 왕복 변환 확인                             |
| `arc_param_length_reverse_direction`              | 감소 도메인 생성 후 자동 반전 및 길이/파라미터 왕복 확인          |
| `arc_split_at`                                    | 타원호 분할 후 길이 합 및 분할점 위치 일치 확인                    |
| `arc_from_polar_angles_matches_endpoints`         | 극각 기반 생성 시 끝점 위치 정확성 확인                           |
| `ellipse_area_vs_subcurve_area`           | 전체 타원 면적과 전체 호 면적이 거의 같음을 확인 (ε 보정 포함)   |
| `ellipse_transform_anisotropic_scale`      | 비등방성 스케일 시 반지름 유지 여부 확인                         |
| `arc_reverse_consistency`                  | 도메인 반전 후 끝점 위치가 평면 반전 기준으로 일치하는지 확인    |




### 1. construct_and_validity
```rust
fn approx(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}
```
```rust
#[test]
fn construct_and_validity() {
    let pl = Plane::xy(); // 필요시 여러분의 Plane 생성자에 맞게 수정
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();
    assert!(e.is_valid());
    assert!(!e.is_circle());
    assert!(
        approx(e.center().x, 0.0, 1e-12)
            && approx(e.center().y, 0.0, 1e-12)
            && approx(e.center().z, 0.0, 1e-12)
    );
}
```

### 2. point_and_tangent_param
```rust
#[test]
fn point_and_tangent_param() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    // t = 0 → (rx, 0)
    let p0 = e.point_at(0.0);
    assert!(approx(p0.x, 3.0, 1e-12) && approx(p0.y, 0.0, 1e-12));

    // t = π/2 → (0, ry)
    let p90 = e.point_at(0.5 * PI);
    assert!(approx(p90.x, 0.0, 1e-12) && approx(p90.y, 2.0, 1e-12));

    // 접선은 매개속도의 정규화
    let t0 = e.tangent_at(0.0);
    // x축 양의 방향에서 시작 → 접선은 +y 방향
    assert!(t0.y > 0.0 && approx(t0.x, 0.0, 1e-12));
}
```

### 3. perimeter_and_arc_length
```rust
#[test]
fn perimeter_and_arc_length() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 5.0, 2.0).unwrap();

    // 전체 길이와 호길이(0..2π)가 일치해야 함
    let l = e.length();
    let la = e.arc_length(0.0, 2.0 * PI);
    println!("{:e}, {:e}", l, la);
    assert!(approx(l, la, 1e-6 * l.max(1.0)));
}
```

### 4. foci_sum_distance_constant
```rust
#[test]
fn foci_sum_distance_constant() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 5.0, 3.0).unwrap();

    // 임의의 점에서 두 초점까지의 거리 합은 2a(주반지름*2)
    let (f1, f2) = e.foci();
    let a = 5.0;
    for k in 0..6 {
        let t = (k as f64) * (PI / 3.0);
        let p = e.point_at(t);
        let s = p.distance(&f1) + p.distance(&f2);
        assert!(approx(s, 2.0 * a, 1e-9));
    }
}
```

### 5. closest_param_on_axes
```rust
#[test]
fn closest_param_on_axes() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 4.0, 2.0).unwrap();

    // 축 위 점들: 파라미터가 정해짐
    let t0 = e.closest_param(&Point::new(10.0, 0.0, 0.0)).unwrap(); // +x 쪽 → t=0
    assert!(approx(t0, 0.0, 1e-9) || approx(t0, 2.0 * PI, 1e-9));

    let t90 = e.closest_param(&Point::new(0.0, 10.0, 0.0)).unwrap(); // +y 쪽 → t=π/2
    assert!(approx(t90, 0.5 * PI, 1e-6));

    let t180 = e.closest_param(&Point::new(-10.0, 0.0, 0.0)).unwrap(); // -x → t=π
    assert!(approx(t180, PI, 1e-6));

    let t270 = e.closest_param(&Point::new(0.0, -10.0, 0.0)).unwrap(); // -y → t=3π/2
    assert!(approx(t270, 1.5 * PI, 1e-6));
}
```

### 6. closest_point_distance_is_minimal
```rust
#[test]
fn closest_point_distance_is_minimal() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    // 임의 점 P에 대해 최근접점 Q가 샘플 점들보다 항상 가깝거나 같아야 함
    let p = Point::new(4.0, 4.0, 0.0);
    let q = e.closest_point(&p);
    let d_min = p.distance(&q);
    let mut ds_min = f64::INFINITY;
    for i in 0..360 {
        let t = (i as f64) * (PI / 180.0);
        let s = e.point_at(t);
        let ds = p.distance(&s);
        ds_min = ds_min.min(ds);
        assert!(ds + 1e-9 >= d_min);
    }

    assert!(ds_min + 1e-9 >= d_min);
}
```

### 7. split_and_sub_curve_length_consistency
```rust
#[test]
fn split_and_sub_curve_length_consistency() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 6.0, 2.0).unwrap();

    let t = PI * 0.7;
    let (left, right) = e.split_at(t).expect("split failed");
    let l = e.length();
    let la = left.length() + right.length();
    assert!(approx(l, la, 1e-6 * l.max(1.0)));

    let arc = e.sub_curve(0.2 * PI, 1.3 * PI).unwrap();
    let lap = arc.length();
    let lap_ref = e.arc_length(0.2 * PI, 1.3 * PI);
    assert!(approx(lap, lap_ref, 1e-6 * lap_ref.max(1.0)));
}
```

### 8. transform_uniform_scale
```rust
#[test]
fn transform_uniform_scale() {
    let pl = Plane::xy();
    let mut e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    let s = 2.0;
    let xf = Transform::scaling(s, s, s);
    e.transform_by(&xf);

    // 반지름이 s배
    assert!(approx(e.radius_x, 6.0, 1e-12));
    assert!(approx(e.radius_y, 4.0, 1e-12));

    // 중심도 스케일됨(원점 기준 스케일이므로 0 그대로)
    let c = e.center();
    assert!(approx(c.x, 0.0, 1e-12) && approx(c.y, 0.0, 1e-12));
}
```

### 9. transform_translate_and_rotate_axis
```rust
#[test]
fn transform_translate_and_rotate_axis() {
    let pl = Plane::xy();
    let mut e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    // 원점 이동
    let xf_t = Transform::translation(10.0, -5.0, 2.0);
    e.transform_by(&xf_t);
    let c = e.center();
    assert!(approx(c.x, 10.0, 1e-12) && approx(c.y, -5.0, 1e-12) && approx(c.z, 2.0, 1e-12));

    // z축 회전 (면이 XY 면이라 회전해도 타원 자체는 동일 도메인)
    let xf_r = Transform::rotation_axis(PI / 3.0, Vector::new(0.0, 0.0, 1.0), c);
    e.transform_by(&xf_r);
    // 반지름은 변하지 않음(회전만)
    assert!(approx(e.radius_x, 3.0, 1e-12) && approx(e.radius_y, 2.0, 1e-12));
}
```
### 10. arc_area_simple_checks
```rust
#[test]
fn arc_area_simple_checks() {
    // 보조: GEllipticalArc의 면적 테스트 (부호 없는 값으로 비교)
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 5.0, 2.0).unwrap();

    let arc = e.sub_curve(0.0, PI).unwrap(); // 반타원
    let a_arc = arc.area().abs();
    // 전체 타원 면적의 절반 근처(좌표계 방향에 따라 부호 달라질 수 있으니 abs)
    let a_full = e.area();
    assert!(approx(a_arc, 0.5 * a_full, 1e-3 * a_full));
}
```

### 11. is_in_plane_basic
```rust
#[test]
fn is_in_plane_basic() {
    let pl = Plane::xy(); // 타원은 XY면에 생성
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    // 동일 평면 → true
    assert!(e.is_in_plane(&pl, 1e-9));

    // z로 1만큼 평행이동한 평면 → false (공차 작게)
    let mut pl_shift = pl.clone();
    pl_shift.origin.z += 1.0;
    pl_shift.update_equation();
    assert!(!e.is_in_plane(&pl_shift, 1e-6));

    // 아주 작은 공차면 false, 큰 공차면 true 확인
    assert!(e.is_in_plane(&pl, 1e-12));
    assert!(!e.is_in_plane(&pl_shift, 1e-12));
    assert!(e.is_in_plane(&pl_shift, 2.0)); // 공차 크게 주면 통과
}
```

### 12. is_point_inside_basic
```rust
#[test]
fn is_point_inside_basic() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    // 타원 위의 점(θ=0) → (3,0)
    let on = e.point_at(0.0);
    // 안쪽 샘플: 위 점을 80% 쪽으로 스케일 (로컬 좌표 기준)
    let inside = Point::new(on.x * 0.8, on.y * 0.8, on.z);
    assert!(e.is_point_inside(&inside));

    // 바깥 샘플: 위 점을 120%로 스케일
    let outside = Point::new(on.x * 1.2, on.y * 1.2, on.z);
    assert!(!e.is_point_inside(&outside));

    // 다른 각도(30°)에서도 동일 논리 확인
    let t = 30.0f64.to_radians();
    let p_on = e.point_at(t);
    let p_in = Point::new(p_on.x * 0.85, p_on.y * 0.85, p_on.z);
    let p_out = Point::new(p_on.x * 1.15, p_on.y * 1.15, p_on.z);
    assert!(e.is_point_inside(&p_in));
    assert!(!e.is_point_inside(&p_out));
}
```

### 13. is_point_inside_basic
```rust
#[test]
fn point_on_ellipse_and_get_vector_match_instance_impls() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    // 정적 API vs 인스턴스 API 결과 비교
    for k in 0..12 {
        let t = k as f64 * (PI / 6.0);
        let p_static = Ellipse::point_on_ellipse_at(t, &e.plane, e.radius_x, e.radius_y);
        let p_inst = e.point_at(t);
        assert!(approx(p_static.x, p_inst.x, 1e-12));
        assert!(approx(p_static.y, p_inst.y, 1e-12));
        assert!(approx(p_static.z, p_inst.z, 1e-12));

        let v_static = Ellipse::get_vector(t, &e.plane, e.radius_x, e.radius_y);
        let v_inst = e.derivative_at(1, t);
        assert!(approx(v_static.x, v_inst.x, 1e-12));
        assert!(approx(v_static.y, v_inst.y, 1e-12));
        assert!(approx(v_static.z, v_inst.z, 1e-12));
    }
}
```

### 14. derive_at_static_cycle_matches
```rust
#[test]
fn derive_at_static_cycle_matches() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    // k=0..3 4주기 패턴 확인
    for &t in &[0.0, PI / 6.0, PI / 3.0, PI / 2.0, PI] {
        let d0 = Ellipse::derive_at_static(0, t, &e.plane, e.radius_x, e.radius_y);
        let d1 = Ellipse::derive_at_static(1, t, &e.plane, e.radius_x, e.radius_y);
        let d2 = Ellipse::derive_at_static(2, t, &e.plane, e.radius_x, e.radius_y);
        let d3 = Ellipse::derive_at_static(3, t, &e.plane, e.radius_x, e.radius_y);

        // 4주기: d4 == d0 근사
        let d4 = Ellipse::derive_at_static(4, t, &e.plane, e.radius_x, e.radius_y);
        assert!(
            approx(d0.x, d4.x, 1e-12) && approx(d0.y, d4.y, 1e-12) && approx(d0.z, d4.z, 1e-12)
        );

        // 인스턴스 1차 미분과 d1 일치
        let rp = e.derivative_at(1, t);
        assert!(
            approx(rp.x, d1.x, 1e-12) && approx(rp.y, d1.y, 1e-12) && approx(rp.z, d1.z, 1e-12)
        );

        // d2는 2차 미분과 일치
        let rpp = e.derivative_at(2, t);
        assert!(
            approx(rpp.x, d2.x, 1e-12)
                && approx(rpp.y, d2.y, 1e-12)
                && approx(rpp.z, d2.z, 1e-12)
        );

        // d3는 3차 미분과 일치
        let rppp = e.derivative_at(3, t);
        assert!(
            approx(rppp.x, d3.x, 1e-12)
                && approx(rppp.y, d3.y, 1e-12)
                && approx(rppp.z, d3.z, 1e-12)
        );
    }
}
```

### 15. normal_at_and_tangents
```rust
#[test]
fn normal_at_and_tangents() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    // 시작/끝 탠전트는 동일 구현 (t=0)
    let t0 = e.start_tangent();
    let t1 = e.end_tangent();
    assert!(
        approx(t0.x, t1.x, 1e-12) && approx(t0.y, t1.y, 1e-12) && approx(t0.z, t1.z, 1e-12)
    );

    // normal_at은 2차 미분 정규화 (C#과 동일 스펙) — 길이 1 확인
    let n = e.normal_at(PI / 3.0);
    assert!(approx(n.length(), 1.0, 1e-12));

    // tangent 과 정확히 직교는 아님(정의상 2차미분 노말이므로), 다만 유한값 확인
    let tan = e.tangent_at(PI / 3.0);
    let dot = n.dot(&tan).abs();
    assert!(dot.is_finite());
}
```

### 16. is_point_inside_matches_implict_equation_sign
```rust
#[test]
fn is_point_inside_matches_implict_equation_sign() {
    // 보너스: 암시적식 x^2/rx^2 + y^2/ry^2 < 1 과 일치하는지 XY-면에서 빠른 샘플
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    for ix in -5..=5 {
        for iy in -5..=5 {
            let x = ix as f64;
            let y = iy as f64;
            let p = Point::new(x, y, 0.0);
            let implicit =
                (x * x) / (e.radius_x * e.radius_x) + (y * y) / (e.radius_y * e.radius_y);
            let inside_implicit = implicit < 1.0 - 1e-12;
            assert_eq!(
                inside_implicit,
                e.is_point_inside(&p),
                "x={x}, y={y}, implicit={implicit}"
            );
        }
    }
}
```

```rust
#[inline]
fn close(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

#[inline]
fn pclose(a: Point, b: Point, eps: f64) -> bool {
    (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps && (a.z - b.z).abs() <= eps
}
```

### 17. ellipse_point_and_tangent_xy
```rust
#[test]
fn ellipse_point_and_tangent_xy() {
    let center = Point::new(0.0, 0.0, 0.0);
    let e = Ellipse::new(center, 2.0, 1.0).expect("ellipse");
    // 기본 도메인 [0, 2π]
    let d = e.domain();
    assert!(close(d.t0, 0.0, 1e-14) && close(d.t1, TAU, 1e-14));

    // 점: t=0 -> (rx, 0), t=π/2 -> (0, ry)
    let p0 = e.point_at(0.0);
    assert!(pclose(p0, Point::new(2.0, 0.0, 0.0), 1e-12));
    let p90 = e.point_at(PI / 2.0);
    assert!(pclose(p90, Point::new(0.0, 1.0, 0.0), 1e-12));

    // 접선은 단위벡터
    let t0 = e.tangent_at(0.0);
    assert!(t0.is_finite());
    assert!(close(t0.length(), 1.0, 1e-12));
    // t=0의 접선 방향 = +Y
    let want = Vector::new(0.0, 1.0, 0.0);
    assert!(close(t0.dot(&want), 1.0, 1e-12));
}
```
### 18. ellipse_project_basic
```rust
#[test]
fn ellipse_project_basic() {
    let center = Point::new(0.0, 0.0, 0.0);
    let e = Ellipse::new(center, 2.0, 1.0).expect("ellipse");

    // (0,1,0)은 t≈π/2로 사영
    let mut t = -1.0;
    assert!(e.project(Point::new(0.0, 1.0, 0.0), &mut t));
    // 파라미터는 극각-사분면 변환을 거치므로 정확히 π/2에 매우 근접
    assert!(close(t, PI / 2.0, 1e-9));
    // 해당 파라미터의 점과도 일치
    let p = e.point_at(t);
    assert!(pclose(p, Point::new(0.0, 1.0, 0.0), 1e-9));
}
```

### 19. ellipse_length_param_roundtrip_mid
```rust
#[test]
fn ellipse_length_param_roundtrip_mid() {
    let e = Ellipse::new(Point::new(0.0, 0.0, 0.0), 2.0, 1.5).expect("ellipse");
    let total = e.length();
    assert!(total.is_finite() && total > 0.0);

    // 길이 절반 → 파라미터
    let (ok, u) = e.get_param_from_length(0.5 * total);
    assert!(ok);

    // 파라미터 → 길이 (도메인 시작으로부터)
    let (ok2, len_back) = e.get_length_from_param(u);
    assert!(ok2);
    assert!(close(len_back, 0.5 * total, 1e-7 * total)); // 수치적분 오차 허용
}
```


### 20. arc_basic_and_endpoints
```rust
#[test]
fn arc_basic_and_endpoints() {
    let plane = Plane::xy();
    let center = Point::new(0.0, 0.0, 0.0);
    let rx = 3.0;
    let ry = 1.5;
    let t0 = 0.2;
    let t1 = 1.7;
    let arc = EllipticalArc::new_on_plane(plane, center, rx, ry, t0, t1).expect("arc");

    // 끝점 확인
    let p_start = arc.point_at(t0);
    let p_end = arc.point_at(t1);
    assert!(pclose(p_start, arc.point_at(t0), 1e-12));
    assert!(pclose(p_end, arc.point_at(t1), 1e-12));

    // 길이>0
    let length = arc.length();
    assert!(length > 0.0);
}
```

### 21. arc_param_length_forward
```rust
#[test]
fn arc_param_length_forward() {
    let plane = Plane::xy();
    let arc =
        EllipticalArc::new_on_plane(plane, Point::new(0.0, 0.0, 0.0), 2.0, 1.0, 0.3, 2.2)
            .expect("arc");
    let total = arc.length();

    // 0 → t0
    let (ok0, u0) = arc.get_param_from_length(0.0);
    assert!(ok0 && close(u0, arc.t0, 1e-12));

    // total → t1
    let (ok1, u1) = arc.get_param_from_length(total);
    assert!(ok1 && close(u1, arc.t1, 1e-9));

    // 중간 왕복
    let (okm, um) = arc.get_param_from_length(0.5 * total);
    assert!(okm);
    let (okk, len) = arc.get_length_from_param(um);
    assert!(okk);
    println!("len {len}, total {total}, um {um}");

    assert!(close(len, 0.5 * total, 1e-7 * total));
}
```


### 22. arc_param_length_reverse_direction
```rust
#[test]
fn arc_param_length_reverse_direction() {
    // t1 < t0 로 생성 → 내부에서 reverse 처리되어 유효 영역
    let plane = Plane::xy();
    let arc =
        EllipticalArc::new_on_plane(plane, Point::new(0.0, 0.0, 0.0), 2.0, 1.0, 1.8, 0.7)
            .expect("arc-rev");

    let d = arc.domain();
    // 생성자가 감소 도메인을 받으면 반전하여 증가도메인으로 맞추도록 구현했으므로 d.t0 < d.t1 여야 함
    assert!(
        d.0 < d.1,
        "angle should be normalized to increasing interval"
    );

    let total = arc.length();

    // 0 → t0
    let (ok0, u0) = arc.get_param_from_length(0.0);
    assert!(ok0 && close(u0, d.0, 1e-12));

    // total → t1
    let (ok1, u1) = arc.get_param_from_length(total);
    assert!(ok1 && close(u1, d.1, 1e-9));

    // 중간 왕복
    let (okm, um) = arc.get_param_from_length(0.5 * total);
    assert!(okm);
    let (okl, lmid) = arc.get_length_from_param(um);
    assert!(okl && close(lmid, 0.5 * total, 1e-6 * total));
}
```

### 23. arc_split_at
```rust
#[test]
fn arc_split_at() {
    let plane = Plane::xy();
    let t0 = 0.2;
    let t1 = 1.9;
    let t_mid = 1.0;
    let arc =
        EllipticalArc::new_on_plane(plane, Point::new(0.0, 0.0, 0.0), 2.0, 1.0, t0, t1)
            .expect("arc");
    let (a, b) = arc.split_at(t_mid).expect("split");
    // 분할 결과 도메인 이어붙이면 원래와 동일한 길이
    let l0 = arc.length();
    let l1 = a.length() + b.length();
    assert!(close(l0, l1, 1e-9 * l0));
    // 분할점 위치 일치
    assert!(pclose(
        a.point_at(a.domain().1),
        b.point_at(b.domain().0),
        1e-12
    ));
}
```


### 24. arc_split_at
```rust
#[test]
fn arc_from_polar_angles_matches_endpoints() {
    // 극각(폴라)에서 파라미터로 변환해 생성하는 경우, 끝점은 대응하는 극각 방향에 있어야 함
    let plane = Plane::xy();
    let c = Point::new(0.0, 0.0, 0.0);
    let rx = 3.0;
    let ry = 1.0;

    // 극각 30° ~ 210°
    let s_ang = 30.0_f64.to_radians();
    let e_ang = 210.0_f64.to_radians();

    let arc = EllipticalArc::from_polar_angles(plane, c, rx, ry, s_ang, e_ang, true)
        .expect("polar-arc");

    // 시작/끝점이 극각 방향의 점과 (타원 파라미터 변환 후) 일치하는지 확인
    // 극각 -> 타원 파라미터 변환을 테스트하기 위해 같은 유틸리티를 재사용
    let t_s = Ellipse::param_from_polar(rx, ry, s_ang);
    let t_e = Ellipse::param_from_polar(rx, ry, e_ang);
    let ps = arc.point_at(arc.domain().0);
    let pe = arc.point_at(arc.domain().1);
    assert!(pclose(ps, arc.point_at(t_s), 1e-9));
    assert!(pclose(pe, arc.point_at(t_e), 1e-9));
}
```


### 25. ellipse_area_vs_subcurve_area
```rust
#[test]
fn ellipse_area_vs_subcurve_area() {
    let pl = Plane::xy();
    let e = Ellipse::from_plane(pl, 4.0, 2.0).unwrap();
    let full_area = e.area();

    // 아주 약간 줄여서 전체 호 생성
    let arc = e.sub_curve(0.0, 2.0 * std::f64::consts::PI - 1e-10).unwrap();
    let arc_area = arc.area();

    assert!(on_are_equal(full_area, arc_area, 1e-9 * full_area));
}
```

### 26. ellipse_transform_anisotropic_scale
```rust
#[test]
fn ellipse_transform_anisotropic_scale() {
    let pl = Plane::xy();
    let mut e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    let xf = Transform::scaling(2.0, 1.0, 1.0);
    e.transform_by(&xf);

    // 반지름은 유지됨 (비등방성 스케일은 평면만 변형)
    assert!(on_are_equal(e.radius_x, 3.0, 1e-12));
    assert!(on_are_equal(e.radius_y, 2.0, 1e-12));
}
```


### 27. ellipse_transform_anisotropic_scale_correct
```rust
#[test]
fn ellipse_transform_anisotropic_scale_correct() {
    let pl = Plane::xy();
    let mut e = Ellipse::from_plane(pl, 3.0, 2.0).unwrap();

    let xf = Transform::scaling(2.0, 1.0, 1.0); // x축 2배, y축 그대로
    e.transform_by(&xf);

    // 반지름은 그대로
    assert!(on_are_equal(e.radius_x, 3.0, 1e-12));
    assert!(on_are_equal(e.radius_y, 2.0, 1e-12));

    println!("x_axis {:?}", e.plane.x_axis);
    println!("y_axis {:?}", e.plane.y_axis);
    // 평면 축 길이 확인
    assert!(on_are_equal(e.plane.x_axis.length(), 2.0, 1e-12));
    assert!(on_are_equal(e.plane.y_axis.length(), 1.0, 1e-12));


    // 점 위치는 축 길이에 따라 변형됨
    let p = e.point_at(0.0); // 원래 (3, 0) → x축 2배 → (6, 0)
    assert!(on_are_equal(p.x, 6.0, 1e-12));
    assert!(on_are_equal(p.y, 0.0, 1e-12));
}
```

---








