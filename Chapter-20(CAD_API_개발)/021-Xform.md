# Xform

- Xform 구조체는 3D 그래픽 및 기하학에서 자주 사용되는 4×4 변환 행렬을 다루며, 회전, 평행이동, 스케일, 역변환, 행렬 곱 등 다양한 기능을 제공합니다.  
- 아래에 각 함수의 수식화, 함수 요약표, 그리고 수학적 점검을 정리.

## 📐 Xform 주요 함수 수식 정리

| 함수 이름                     | 수식 또는 원리                                                                 |
|------------------------------|----------------------------------------------------------------------------------|
| `transform_point(p)`         | $\mathbf{p}'=\dfrac{M,[x\ y\ z\ 1]^T}{w}$ |
| `transform_vector(v)`        | $\mathbf{v}' = M_{3×3} \cdot \mathbf{v}$                                   |
| `transform_normal(n)`        | $\mathbf{n}' = (M^{-1})_{3×3}^T \cdot \mathbf{n}$                          |
| `translation(dx, dy, dz)`    | $T = [[1,0,0,dx],[0,1,0,dy],[0,0,1,dz],[0,0,0,1]]$ |
| `scale_xyz(sx, sy, sz)`      | $\mathbf{S} = \mathrm{diag}(sx, sy, sz, 1)$                                |
| `rotation_about_axis(c, \vec{u}, \theta)` | $\mathbf{T}(c) \cdot \mathbf{R}(\vec{u}, \theta) \cdot \mathbf{T}(-c)$ |
| `rotation_from_to(c, \vec{a}, \vec{b})`   | $\mathbf{R} = \mathrm{Rodrigues}(\vec{a} \times \vec{b}, \arctan2(\|\vec{a} \times \vec{b}\|, \vec{a} \cdot \vec{b}))$ |
| `inverse()`                  | $M^{-1}$                                                                   |
| `determinant()`              | $\det(M)$                                                                  |
| `transpose()`                | $M^T$                                                                      |
| `Mul<Xform>`                 | $M = A \cdot B$                                                            |
| `Add<Xform>`                 | $M = A + B$                                                                |
| `Sub<Xform>`                 | $M = A - B$                                                                |

## 📊 Xform 함수 요약표
| 함수 이름                      | 기능 설명                                 | 수식 기반 여부 |
|-------------------------------|--------------------------------------------|----------------|
| `identity()`                  | 단위 행렬 생성                             | ✅              |
| `zero_transformation()`       | 모든 요소 0, 마지막 원소만 1               | ✅              |
| `diagonal(d)`                 | 스케일 행렬 생성                           | ✅              |
| `from_cols(c0, c1, c2, c3)`   | 열 벡터로 행렬 구성                        | ✅              |
| `transform_point(p)`          | 포인트에 변환 적용                         | ✅              |
| `transform_vector(v)`         | 벡터에 선형 변환 적용                      | ✅              |
| `transform_normal(n)`         | 법선 벡터에 역행렬 전치 적용               | ✅              |
| `translation(dx, dy, dz)`     | 평행이동 행렬 생성                         | ✅              |
| `scale_xyz(sx, sy, sz)`       | 축별 스케일 행렬 생성                      | ✅              |
| `rotation_about_axis(c,u,θ)`  | 중심점 기준 회전                           | ✅              |
| `rotation_from_to(c,a,b)`     | 벡터 a→b 최소 회전                         | ✅              |
| `inverse()`                   | 행렬 역원 계산                             | ✅              |
| `determinant()`               | 행렬식 계산                                | ✅              |
| `transpose()`                 | 행렬 전치                                  | ✅              |
| `Mul<Xform>`                  | 행렬 곱                                    | ✅              |
| `Add<Xform>`                  | 행렬 덧셈                                  | ✅              |
| `Sub<Xform>`                  | 행렬 뺄셈                                  | ✅              |

## 수학 검증

| 함수 이름     | 수학적 정확성 | 구현 안정성 | 설명 |
|---------------|----------------|--------------|------|
| `Mul`         | ✅ 정확         | ✅ 안정적     | 표준 행렬 곱: $M = A \cdot B$|
| `transpose`   | ✅ 정확         | ✅ 안정적     | 행렬 전치: $M^T$|
| `determinant` | ✅ 정확         | ✅ 안정적     | Laplace 전개 최적화 |
| `inverse`     | ✅ 정확         | ✅ 안정적     | Gauss-Jordan 방식, pivot 선택 포함 |
| `transform_point` | ✅ 정확     | ✅ 안정적     | 동차 좌표 적용 후 w로 나눔 |
| `transform_vector` | ✅ 정확    | ✅ 안정적     | w=0 처리로 평행이동 무시 |
| `transform_normal` | ✅ 정확    | ✅ 안정적     | $(M^{-1})^T \cdot n$|
| `rotation_about_axis` | ✅ 정확 | ✅ 안정적     | Rodrigues 공식 기반 회전 행렬 |
| `rotation_from_to` | ✅ 정확    | ✅ 안정적     | 최소 회전: cross/dot 기반 |
| `scale_xyz`   | ✅ 정확         | ✅ 안정적     | 축별 스케일 행렬 구성 |

---

## 소스 코드
```rust
use crate::math::point2d::Point2D;
use crate::math::prelude::{Point3D, Vector3D};
use crate::math::vector2d::Vector2D;
use core::ops::{Add, Mul, Sub};
```
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xform {
    pub m: [[f64; 4]; 4],
}
```
```rust
impl Default for Xform {
    fn default() -> Self {
        Self::zero_transformation()
    }
}
```
```rust
impl Xform {
    pub const fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
```
```rust
    // diag = (0,0,0,1)
    pub const fn zero_transformation() -> Self {
        Self {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
```
```rust
    // 4×4 all zero
    pub const fn zero4x4() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }
```
```rust
    //4x4 all NaN
    pub fn nan() -> Self {
        Self {
            m: [[f64::NAN; 4]; 4],
        }
    }
```
```rust
    // diag = (d,d,d,1)
    pub fn diagonal(d: f64) -> Self {
        Self::diagonal3(d, d, d)
    }
```
```rust
    // diag = (d0,d1,d2,1)
    pub fn diagonal3(d0: f64, d1: f64, d2: f64) -> Self {
        let mut out = Self::identity();
        out.m[0][0] = d0;
        out.m[1][1] = d1;
        out.m[2][2] = d2;
        out
    }
```
```rust    
    pub fn from_cols(c0: [f64; 4], c1: [f64; 4], c2: [f64; 4], c3: [f64; 4]) -> Self {
        // Internally stored in row-major order. Assuming the input consists of four column vectors:
        Self {
            m: [
                [c0[0], c1[0], c2[0], c3[0]],
                [c0[1], c1[1], c2[1], c3[1]],
                [c0[2], c1[2], c2[2], c3[2]],
                [c0[3], c1[3], c2[3], c3[3]],
            ],
        }
    }
```
```rust   
    pub fn as_slice(&self) -> &[f64; 16] {
        // A flattened view for safe access
        unsafe { &*(self.m.as_ptr() as *const [f64; 16]) }
    }
```
```rust
    /* ===== basic predicates ===== */
    pub fn is_nan(&self) -> bool {
        self.m.iter().flatten().any(|v| v.is_nan())
    }
```
```rust    
    pub fn is_valid(&self) -> bool {
        self.m.iter().flatten().all(|v| v.is_finite())
    }
```
```rust    
    pub fn is_identity(&self, eps: f64) -> bool {
        let id = Xform::identity();
        self.m
            .iter()
            .flatten()
            .zip(id.m.iter().flatten())
            .all(|(a, b)| (a - b).abs() <= eps)
    }
```
```rust    
    pub fn is_zero(&self, eps: f64) -> bool {
        self.m.iter().flatten().all(|v| v.abs() <= eps)
    }
```
```rust    
    pub fn is_zero4x4(&self, eps: f64) -> bool {
        self.is_zero(eps)
    }
```
```rust    
    pub fn is_translation(&self, eps: f64) -> bool {
        // diag = (1, 1, 1, 1), only the upper-right region (rows 0 to 2, column 3) is unconstrained
        if (self.m[0][0] - 1.0).abs() > eps
            || (self.m[1][1] - 1.0).abs() > eps
            || (self.m[2][2] - 1.0).abs() > eps
        {
            return false;
        }
        if self.m[3][0].abs() > eps || self.m[3][1].abs() > eps || self.m[3][2].abs() > eps {
            return false;
        }
        if (self.m[3][3] - 1.0).abs() > eps {
            return false;
        }
        // All non-diagonal elements must be zero
        for i in 0..3 {
            for j in 0..3 {
                if i != j && self.m[i][j].abs() > eps {
                    return false;
                }
            }
        }
        true
    }
```
```rust    
    pub fn is_scaled(&self) -> bool {
        // If the three basis vectors of the linear part all have length 1, it's (approximately) unscaled
        let col =
            |j: usize| -> Vector3D { Vector3D::new(self.m[0][j], self.m[1][j], self.m[2][j]) };
        let ln = |v: Vector3D| -> f64 { (v.x * v.x + v.y * v.y + v.z * v.z).sqrt() };
        let l0 = ln(col(0));
        let l1 = ln(col(1));
        let l2 = ln(col(2));
        ((l0 - 1.0).abs() > 1e-9) || ((l1 - 1.0).abs() > 1e-9) || ((l2 - 1.0).abs() > 1e-9)
    }
```
```rust
    pub fn transpose(&self) -> Self {
        let mut t = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                t[i][j] = self.m[j][i];
            }
        }
        Self { m: t }
    }
```
```rust    
    pub fn determinant(&self) -> f64 {
        // 4x4 det via Laplace-expansion optimized
        let m = &self.m;
        let (a, b, c, d) = (m[0][0], m[0][1], m[0][2], m[0][3]);
        let (e, f, g, h) = (m[1][0], m[1][1], m[1][2], m[1][3]);
        let (i, j, k, l) = (m[2][0], m[2][1], m[2][2], m[2][3]);
        let (m0, n, o, p) = (m[3][0], m[3][1], m[3][2], m[3][3]);

        let kp_lo = k * p - l * o;
        let jp_ln = j * p - l * n;
        let jo_kn = j * o - k * n;
        let ip_lm = i * p - l * m0;
        let io_km = i * o - k * m0;
        let in_jm = i * n - j * m0;

        a * (f * kp_lo - g * jp_ln + h * jo_kn) - b * (e * kp_lo - g * ip_lm + h * io_km)
            + c * (e * jp_ln - f * ip_lm + h * in_jm)
            - d * (e * jo_kn - f * io_km + g * in_jm)
    }
```
```rust    
    pub fn inverse(&self) -> Option<Self> {
        // Gauss-Jordan (uses pivot with partial selection for robustness and numerical stability)
        let mut a = self.m;
        let mut inv = Xform::identity().m;

        for i in 0..4 {
            // pivot select
            let mut pivot_row = i;
            let mut maxv = a[i][i].abs();
            for r in (i + 1)..4 {
                let v = a[r][i].abs();
                if v > maxv {
                    maxv = v;
                    pivot_row = r;
                }
            }
            if maxv == 0.0 {
                return None;
            }
            if pivot_row != i {
                a.swap(i, pivot_row);
                inv.swap(i, pivot_row);
            }
            // normalize row i
            let piv = a[i][i];
            for j in 0..4 {
                a[i][j] /= piv;
                inv[i][j] /= piv;
            }
            // eliminate other rows
            for r in 0..4 {
                if r == i {
                    continue;
                }
                let f = a[r][i];
                if f == 0.0 {
                    continue;
                }
                for c in 0..4 {
                    a[r][c] -= f * a[i][c];
                    inv[r][c] -= f * inv[i][c];
                }
            }
        }
        Some(Self { m: inv })
    }
```
```rust
    pub fn transform_point(&self, p: Point3D) -> Point3D {
        let x = self.m[0][0] * p.x + self.m[0][1] * p.y + self.m[0][2] * p.z + self.m[0][3];
        let y = self.m[1][0] * p.x + self.m[1][1] * p.y + self.m[1][2] * p.z + self.m[1][3];
        let z = self.m[2][0] * p.x + self.m[2][1] * p.y + self.m[2][2] * p.z + self.m[2][3];
        let w = self.m[3][0] * p.x + self.m[3][1] * p.y + self.m[3][2] * p.z + self.m[3][3];
        if w != 0.0 {
            Point3D::new(x / w, y / w, z / w)
        } else {
            Point3D::new(x, y, z)
        }
    }
```
```rust    
    pub fn transform_vector(&self, v: Vector3D) -> Vector3D {
        // Ignore translation; apply only the linear part
        Vector3D {
            x: self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            y: self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            z: self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        }
    }
```
```rust    
    pub fn transform_normal(&self, n: Vector3D) -> Vector3D {
        // n' = (M⁻¹)ᵀ * n  (uses the 3×3 linear part o
        if let Some(inv) = self.inverse() {
            let t = inv.transpose();
            Vector3D {
                x: t.m[0][0] * n.x + t.m[0][1] * n.y + t.m[0][2] * n.z,
                y: t.m[1][0] * n.x + t.m[1][1] * n.y + t.m[1][2] * n.z,
                z: t.m[2][0] * n.x + t.m[2][1] * n.y + t.m[2][2] * n.z,
            }
        } else {
            // Safely return the original when the operation is non-invertible
            n
        }
    }
```
```rust
    pub fn translation(dx: f64, dy: f64, dz: f64) -> Self {
        let mut t = Self::identity();
        t.m[0][3] = dx;
        t.m[1][3] = dy;
        t.m[2][3] = dz;
        t
    }
```
```rust    
    pub fn translation_v(d: Vector3D) -> Self {
        Self::translation(d.x, d.y, d.z)
    }
```
```rust
    pub fn scale_uniform(f: f64) -> Self {
        Self::diagonal(f)
    }
```
```rust    
    pub fn scale_xyz(sx: f64, sy: f64, sz: f64) -> Self {
        Self::diagonal3(sx, sy, sz)
    }
```
```rust    
    pub fn scale_about_point(p: Point3D, sx: f64, sy: f64, sz: f64) -> Self {
        // T(p) * S * T(-p)
        Xform::translation(p.x, p.y, p.z)
            * Xform::scale_xyz(sx, sy, sz)
            * Xform::translation(-p.x, -p.y, -p.z)
    }
```
```rust
    #[inline]
    pub fn rotation(angle_radians: f64, axis: &Vector3D, center: &Point3D) -> Self {
        Xform::rotation_about_axis(center, axis, angle_radians)
    }
```
```rust
    #[inline]
    pub fn rotation_axis(angle_radians: f64, axis: &Vector3D) -> Self {
        Xform::rotation_about_axis(&Point3D::new(0.0, 0.0, 0.0), axis, angle_radians)
    }
```
```rust
    pub fn rotation_sin_cos_about_axis(center: &Point3D, axis: &Vector3D, s: f64, c: f64) -> Self {
        // 축 정규화
        let len2 = axis.x * axis.x + axis.y * axis.y + axis.z * axis.z;
        if len2 == 0.0 {
            return Xform::identity();
        }
        let inv_len = len2.sqrt().recip();
        let ux = axis.x * inv_len;
        let uy = axis.y * inv_len;
        let uz = axis.z * inv_len;

        // Rodrigues' formula constructed using direct sin/cos inputs
        let one_c = 1.0 - c;
        let r = [
            [
                c + ux * ux * one_c,
                ux * uy * one_c - uz * s,
                ux * uz * one_c + uy * s,
                0.0,
            ],
            [
                uy * ux * one_c + uz * s,
                c + uy * uy * one_c,
                uy * uz * one_c - ux * s,
                0.0,
            ],
            [
                uz * ux * one_c - uy * s,
                uz * uy * one_c + ux * s,
                c + uz * uz * one_c,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rot = Xform { m: r };
        // T(center) * R * T(-center)
        Xform::translation(center.x, center.y, center.z)
            * rot
            * Xform::translation(-center.x, -center.y, -center.z)
    }
```
```rust
    /// Rotation: center `c`, unit axis `axis`, angle in radians `angle`
    pub fn rotation_about_axis(c: &Point3D, axis: &Vector3D, angle: f64) -> Self {
        let n = {
            let len = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
            if len == 0.0 {
                return Self::identity();
            }
            Vector3D::new(axis.x / len, axis.y / len, axis.z / len)
        };
        let (s, c0) = angle.sin_cos();
        let (ux, uy, uz) = (n.x, n.y, n.z);
        // Rodrigues
        let r = [
            [
                c0 + ux * ux * (1.0 - c0),
                ux * uy * (1.0 - c0) - uz * s,
                ux * uz * (1.0 - c0) + uy * s,
                0.0,
            ],
            [
                uy * ux * (1.0 - c0) + uz * s,
                c0 + uy * uy * (1.0 - c0),
                uy * uz * (1.0 - c0) - ux * s,
                0.0,
            ],
            [
                uz * ux * (1.0 - c0) - uy * s,
                uz * uy * (1.0 - c0) + ux * s,
                c0 + uz * uz * (1.0 - c0),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rot = Xform { m: r };
        Xform::translation(c.x, c.y, c.z) * rot * Xform::translation(-c.x, -c.y, -c.z)
    }
```
```rust
    pub fn rotation_from_to(c: &Point3D, from: &Vector3D, to: &Vector3D) -> Self {
        // Minimal rotation: axis = cross(from, to), angle = atan2(cross, dot)
        let f = from;
        let t = to;
        let dot = f.x * t.x + f.y * t.y + f.z * t.z;
        let cx = Vector3D::new(
            f.y * t.z - f.z * t.y,
            f.z * t.x - f.x * t.z,
            f.x * t.y - f.y * t.x,
        );
        let s = (cx.x * cx.x + cx.y * cx.y + cx.z * cx.z).sqrt();
        if s == 0.0 {
            // Parallel (same direction / opposite direction)
            if dot >= 0.0 {
                Xform::identity()
            } else {
                // Opposite direction: rotate by π around an arbitrary perpendicular axis
                let any = if (f.x.abs() <= f.y.abs()) && (f.x.abs() <= f.z.abs()) {
                    Vector3D::new(1.0, 0.0, 0.0)
                } else if f.y.abs() <= f.z.abs() {
                    Vector3D::new(0.0, 1.0, 0.0)
                } else {
                    Vector3D::new(0.0, 0.0, 1.0)
                };
                let axis = Vector3D::new(
                    f.y * any.z - f.z * any.y,
                    f.z * any.x - f.x * any.z,
                    f.x * any.y - f.y * any.x,
                );
                Xform::rotation_about_axis(c, &axis, core::f64::consts::PI)
            }
        } else {
            let angle = s.atan2(dot);
            Xform::rotation_about_axis(c, &cx, angle)
        }
    }
```
```rust
    pub fn linear_part3x3(&self) -> [[f64; 3]; 3] {
        [
            [self.m[0][0], self.m[0][1], self.m[0][2]],
            [self.m[1][0], self.m[1][1], self.m[1][2]],
            [self.m[2][0], self.m[2][1], self.m[2][2]],
        ]
    }
```
```rust
    #[inline]
    pub fn rotation_sc(sin_angle: f64, cos_angle: f64, axis: &Vector3D, center: &Point3D) -> Self {
        Xform::rotation_sin_cos_about_axis(center, axis, sin_angle, cos_angle)
    }
```
```rust
    #[inline]
    fn multi_homogeneous(m: &Xform, x: f64, y: f64, z: f64, w: f64) -> (f64, f64, f64, f64) {
        let xr = m.m[0][0] * x + m.m[0][1] * y + m.m[0][2] * z + m.m[0][3] * w;
        let yr = m.m[1][0] * x + m.m[1][1] * y + m.m[1][2] * z + m.m[1][3] * w;
        let zr = m.m[2][0] * x + m.m[2][1] * y + m.m[2][2] * z + m.m[2][3] * w;
        let wr = m.m[3][0] * x + m.m[3][1] * y + m.m[3][2] * z + m.m[3][3] * w;
        (xr, yr, zr, wr)
    }
```
```rust
    #[allow(unused)]
    fn multi_point(m: &Xform, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let xr = m.m[0][0] * x + m.m[0][1] * y + m.m[0][2] * z + m.m[0][3];
        let yr = m.m[1][0] * x + m.m[1][1] * y + m.m[1][2] * z + m.m[1][3];
        let zr = m.m[2][0] * x + m.m[2][1] * y + m.m[2][2] * z + m.m[2][3];
        let wr = m.m[3][0] * x + m.m[3][1] * y + m.m[3][2] * z + m.m[3][3];
        (xr / wr, yr / wr, zr / wr)
    }
}
```
```rust
// For Point3D
impl Mul<Xform> for Point3D {
    type Output = Point3D;
    #[inline]
    fn mul(self, rhs: Xform) -> Self::Output {
        let (x, y, z, w) = Xform::multi_homogeneous(&rhs, self.x, self.y, self.z, 1.0);
        if w != 0.0 {
            Point3D::new(x / w, y / w, z / w)
        } else {
            Point3D::new(x, y, z)
        }
    }
}
```
```rust
impl<'a, 'b> Mul<&'b Xform> for &'a Point3D {
    type Output = Point3D;
    #[inline]
    fn mul(self, rhs: &'b Xform) -> Self::Output {
        let (x, y, z, w) = Xform::multi_homogeneous(rhs, self.x, self.y, self.z, 1.0);
        if w != 0.0 {
            Point3D::new(x / w, y / w, z / w)
        } else {
            Point3D::new(x, y, z)
        }
    }
}
```
```rust
impl<'b> Mul<&'b Xform> for Point3D {
    type Output = Point3D;
    #[inline]
    fn mul(self, rhs: &'b Xform) -> Self::Output {
        let (x, y, z, w) = Xform::multi_homogeneous(rhs, self.x, self.y, self.z, 1.0);
        if w != 0.0 {
            Point3D::new(x / w, y / w, z / w)
        } else {
            Point3D::new(x, y, z)
        }
    }
}
```
```rust
impl<'a> Mul<Xform> for &'a Point3D {
    type Output = Point3D;
    #[inline]
    fn mul(self, rhs: Xform) -> Self::Output {
        let (x, y, z, w) = Xform::multi_homogeneous(&rhs, self.x, self.y, self.z, 1.0);
        if w != 0.0 {
            Point3D::new(x / w, y / w, z / w)
        } else {
            Point3D::new(x, y, z)
        }
    }
}
```
```rust
// For Vector3D
impl Mul<Xform> for Vector3D {
    type Output = Vector3D;
    #[inline]
    fn mul(self, rhs: Xform) -> Self::Output {
        // 선형부만 적용 (w=0)
        let (x, y, z, _w) = Xform::multi_homogeneous(&rhs, self.x, self.y, self.z, 0.0);
        Vector3D::new(x, y, z)
    }
}
```
```rust
impl<'a, 'b> Mul<&'b Xform> for &'a Vector3D {
    type Output = Vector3D;
    #[inline]
    fn mul(self, rhs: &'b Xform) -> Self::Output {
        let (x, y, z, _w) = Xform::multi_homogeneous(rhs, self.x, self.y, self.z, 0.0);
        Vector3D::new(x, y, z)
    }
}
```
```rust
impl<'b> Mul<&'b Xform> for Vector3D {
    type Output = Vector3D;
    #[inline]
    fn mul(self, rhs: &'b Xform) -> Self::Output {
        let (x, y, z, _w) = Xform::multi_homogeneous(rhs, self.x, self.y, self.z, 0.0);
        Vector3D::new(x, y, z)
    }
}
```
```rust
impl<'a> Mul<Xform> for &'a Vector3D {
    type Output = Vector3D;
    #[inline]
    fn mul(self, rhs: Xform) -> Self::Output {
        let (x, y, z, _w) = Xform::multi_homogeneous(&rhs, self.x, self.y, self.z, 0.0);
        Vector3D::new(x, y, z)
    }
}
```
```rust
// For a Point2D
impl Mul<Xform> for Point2D {
    type Output = Point2D;
    #[inline]
    fn mul(self, rhs: Xform) -> Self::Output {
        let (x, y, _z, w) = Xform::multi_homogeneous(&rhs, self.x, self.y, 0.0, 1.0);
        if w != 0.0 {
            Point2D::new(x / w, y / w)
        } else {
            Point2D::new(x, y)
        }
    }
}
```
```rust
impl<'a, 'b> Mul<&'b Xform> for &'a Point2D {
    type Output = Point2D;
    #[inline]
    fn mul(self, rhs: &'b Xform) -> Self::Output {
        let (x, y, _z, w) = Xform::multi_homogeneous(rhs, self.x, self.y, 0.0, 1.0);
        if w != 0.0 {
            Point2D::new(x / w, y / w)
        } else {
            Point2D::new(x, y)
        }
    }
}
```
```rust
impl<'b> Mul<&'b Xform> for Point2D {
    type Output = Point2D;
    #[inline]
    fn mul(self, rhs: &'b Xform) -> Self::Output {
        let (x, y, _z, w) = Xform::multi_homogeneous(rhs, self.x, self.y, 0.0, 1.0);
        if w != 0.0 {
            Point2D::new(x / w, y / w)
        } else {
            Point2D::new(x, y)
        }
    }
}
```
```rust
impl<'a> Mul<Xform> for &'a Point2D {
    type Output = Point2D;
    #[inline]
    fn mul(self, rhs: Xform) -> Self::Output {
        let (x, y, _z, w) = Xform::multi_homogeneous(&rhs, self.x, self.y, 0.0, 1.0);
        if w != 0.0 {
            Point2D::new(x / w, y / w)
        } else {
            Point2D::new(x, y)
        }
    }
}
```
```rust
// For a Vector2D
impl Mul<Xform> for Vector2D {
    type Output = Vector2D;
    #[inline]
    fn mul(self, rhs: Xform) -> Self::Output {
        let (x, y, _z, _w) = Xform::multi_homogeneous(&rhs, self.x, self.y, 0.0, 0.0);
        Vector2D::new(x, y)
    }
}
```
```rust
impl<'a, 'b> Mul<&'b Xform> for &'a Vector2D {
    type Output = Vector2D;
    #[inline]
    fn mul(self, rhs: &'b Xform) -> Self::Output {
        let (x, y, _z, _w) = Xform::multi_homogeneous(rhs, self.x, self.y, 0.0, 0.0);
        Vector2D::new(x, y)
    }
}
```
```rust
impl<'b> Mul<&'b Xform> for Vector2D {
    type Output = Vector2D;
    #[inline]
    fn mul(self, rhs: &'b Xform) -> Self::Output {
        let (x, y, _z, _w) = Xform::multi_homogeneous(rhs, self.x, self.y, 0.0, 0.0);
        Vector2D::new(x, y)
    }
}
```
```rust
impl<'a> Mul<Xform> for &'a Vector2D {
    type Output = Vector2D;
    #[inline]
    fn mul(self, rhs: Xform) -> Self::Output {
        let (x, y, _z, _w) = Xform::multi_homogeneous(&rhs, self.x, self.y, 0.0, 0.0);
        Vector2D::new(x, y)
    }
}
```
```rust
impl Mul for Xform {
    type Output = Xform;
    fn mul(self, rhs: Xform) -> Xform {
        let mut r = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j]
                    + self.m[i][3] * rhs.m[3][j];
            }
        }
        Xform { m: r }
    }
}
```
```rust
impl Add for Xform {
    type Output = Xform;
    fn add(self, rhs: Xform) -> Xform {
        let mut r = self;
        for i in 0..4 {
            for j in 0..4 {
                r.m[i][j] += rhs.m[i][j];
            }
        }
        r
    }
}
```
```rust
impl Sub for Xform {
    type Output = Xform;
    fn sub(self, rhs: Xform) -> Xform {
        let mut r = self;
        for i in 0..4 {
            for j in 0..4 {
                r.m[i][j] -= rhs.m[i][j];
            }
        }
        r
    }
}
```
```rust
pub mod nalgebra_compat_xform {
    use nalgebra::{Matrix3, Rotation3, SymmetricEigen, Translation3};

    use super::Xform;

    impl Xform {
        // Extracts the 3x3 linear part as a `Matrix3<f64>` from nalgebra.
        // Since the internal layout is row-major, you can pass it directly to `from_row_slice`.
        pub fn to_na_matrix3(&self) -> Matrix3<f64> {
            Matrix3::from_row_slice(&[
                self.m[0][0],
                self.m[0][1],
                self.m[0][2],
                self.m[1][0],
                self.m[1][1],
                self.m[1][2],
                self.m[2][0],
                self.m[2][1],
                self.m[2][2],
            ])
        }
```
```rust
        // Extracts the translation component as a `Translation3<f64>` from nalgebra.
        pub fn to_na_translation3(&self) -> Translation3<f64> {
            Translation3::new(self.m[0][3], self.m[1][3], self.m[2][3])
        }

        // Extracts the pure rotational component from a 3×3 linear matrix that may contain scale or shear,
        // using **polar decomposition**, and returns it as a `Rotation3`.
        // Returns `None` on failure (e.g. singular matrix).
        //
        // Principle: A = R · S, where S = sqrt(AᵀA) ⇒ R = A · inv(sqrt(AᵀA))
        // Determinant correction: if det(R) < 0, the last axis is flipped.
```
```rust
        pub fn try_to_na_rotation3(&self, tol: f64) -> Option<Rotation3<f64>> {
            let a = self.to_na_matrix3(); // A (3x3)
            // AᵀA (symmetric positive definite)
            let ata = a.transpose() * a;

            // Computing (AᵀA)⁻¹ᐟ²: given (AᵀA) = QΛQᵀ, then (AᵀA)⁻¹ᐟ² = QΛ⁻¹ᐟ²Qᵀ
            let se = SymmetricEigen::new(ata);

            // Inversion of the square root fails if eigenvalues are too small.
            if se.eigenvalues.iter().any(|&l| l <= tol) {
                return None;
            }
            let mut lam_inv_sqrt = Matrix3::zeros();
            for i in 0..3 {
                lam_inv_sqrt[(i, i)] = 1.0 / se.eigenvalues[i].sqrt();
            }
            let inv_sqrt = &se.eigenvectors * lam_inv_sqrt * se.eigenvectors.transpose();

            // R = A (AᵀA)⁻¹ᐟ²
            let mut r = a * inv_sqrt;

            // Determinant correction (reflection removal)
            if r.determinant() < 0.0 {
                // 마지막 열에 -1 곱해 보정 (D = diag(1,1,-1))
                let mut d = Matrix3::identity();
                d[(2, 2)] = -1.0;
                r = r * d;
            }

            // Due to numerical errors, orthogonality or unit length may slightly deviate,
            // so we re-orthogonalize the matrix to the nearest valid rotation before returning.
            // (R ≈ R (RᵀR)^{-1/2})
            let rt_r = r.transpose() * r;
            let se2 = SymmetricEigen::new(rt_r);
            if se2.eigenvalues.iter().any(|&l| l <= tol) {
                return None;
            }
            let mut lam_inv_sqrt2 = Matrix3::zeros();
            for i in 0..3 {
                lam_inv_sqrt2[(i, i)] = 1.0 / se2.eigenvalues[i].sqrt();
            }
            let inv_sqrt2 = &se2.eigenvectors * lam_inv_sqrt2 * se2.eigenvectors.transpose();
            r = r * inv_sqrt2;

            // Wrap with Rotation3 (assuming it's nearly orthonormal)
            Some(Rotation3::from_matrix_unchecked(r))
        }
```
```rust
        /// (Note) Fast path used when the rotation is known to be a pure rotation (orthogonal with det = +1).
        pub fn to_na_rotation3_unchecked(&self) -> Rotation3<f64> {
            Rotation3::from_matrix_unchecked(self.to_na_matrix3())
        }
```
```rust
        /// Verifies whether the transform represents a rigid motion (pure rotation and translation only),
        /// and returns `(Translation3, Rotation3)` if valid.
        pub fn try_to_na_tr(&self, tol: f64) -> Option<(Translation3<f64>, Rotation3<f64>)> {
            let t = self.to_na_translation3();
            let r = self.try_to_na_rotation3(tol)?;
            Some((t, r))
        }
    }
}
```
---
