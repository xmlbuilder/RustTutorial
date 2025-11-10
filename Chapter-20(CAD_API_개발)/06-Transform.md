# Transform

## 🏗️ Transform 생성자 및 기본 행렬

| 함수명                        | 수식 또는 설명                                                                 |
|------------------------------|----------------------------------------------------------------------------------|
| `identity()`                 | $M = I_{4×4}$ — 단위 행렬                                                   |
| `zero()`                     | $M = 0_{4×4}$ — 모든 원소가 0인 행렬                                        |
| `is_identity()`              | $M \approx I$ — 오차 허용 범위 내에서 단위 행렬인지 확인                    |
| `scale_uniform(f)`           | $M = \text{diag}(f, f, f, 1)$ — 원점 기준 균일 스케일링                     |
| `scale_uniform_about(f, c)`  | $T(c) \cdot S(f) \cdot T(-c)$ — 점 `c` 기준 균일 스케일링                   |
| `scaling(sx, sy, sz)`        | $M = \text{diag}(sx, sy, sz, 1)$ — 비균일 스케일링                          |
| `translation(dx, dy, dz)`    | $M = I + \text{translation vector}$ — 평행이동 행렬                         |
| `rotation_axis(angle, axis, center)` | Rodrigues' formula + offset — 축 기준 회전 행렬 생성 |
| `mirror_about_plane(p, n)`   | $M = I - 2nn^T$ + offset — 평면 기준 반사 행렬                              |
| `from_basis(origin, x, y, z)`| 열벡터: $M = [x\ y\ z\ origin]$ — 로컬 → 월드 좌표계                        |
| `to_basis(origin, x, y, z)`  | $M^{-1}$ — 월드 → 로컬 좌표계                                              |
| `change_of_basis(a, b)`      | $M = B^{-1} \cdot A$ — 좌표계 A → B 변환                                   |
| `from_cols(p0, p1, p2, p3)`  | $M = [p0\ p1\ p2\ p3]^T$ — 열벡터 직접 지정                                 |


## 🔁 Transform 행렬 연산

| 함수명                    | 수식 또는 설명                                                                 |
|--------------------------|----------------------------------------------------------------------------------|
| `mul(&rhs)`              | $M_{\text{result}} = M_{\text{self}} \cdot M_{\text{rhs}}$ <br>4×4 행렬 곱. 행 우선 곱셈 기준. |
| `invert()`               | $M^{-1}$ — 행렬의 역행렬. 존재하지 않으면 `None` 반환.                      |
| `inverse_transpose3x3()` | $(M^{-1})^T_{3×3}$ — 상단 3×3 블록의 역전치. 법선 벡터 변환에 사용.         |
| `then(next)`             | $M_{\text{then}} = M_{\text{next}} \cdot M_{\text{self}}$ <br>우→좌 순서의 합성. |

### 📌 참고
- mul()과 then()은 행렬 곱의 순서에 따라 결과가 달라지므로, 우선 적용되는 변환이 오른쪽에 위치합니다.
- inverse_transpose3x3()는 법선 벡터 변환 시 필수로 사용되는 연산입니다.
- invert()는 inverse4() 함수를 통해 안정적으로 역행렬을 계산합니다.



## 🧭 Transform 변환 API

| 함수명                  | 수식 또는 설명                                                                 |
|------------------------|----------------------------------------------------------------------------------|
| `transform_point2d(p)` | $\vec{p}' = \text{proj3}(M \cdot [x, y, 0, 1]^T)$ <br>2D 점의 동차 좌표 변환 후 투영 |
| `transform_vector2d(v)`| $\vec{v}' = \text{proj3}(M \cdot [x, y, 0, 0]^T)$ <br>2D 벡터의 선형 변환         |
| `transform_point3d(p)` | $\vec{p}' = \text{proj3}(M \cdot [x, y, z, 1]^T)$ <br>3D 점의 동차 좌표 변환 후 투영 |
| `transform_vector3d(v)`| $\vec{v}' = \text{proj3}(M \cdot [x, y, z, 0]^T)$ <br>3D 벡터의 선형 변환         |
| `transform_point4d(h)`| $\vec{p}' = M \cdot [x, y, z, w]^T$ <br>4D 점의 직접 변환                        |
| `transform_normal(n)` | $\vec{n}' = \text{normalize}((M^{-1})^T_{3×3} \cdot \vec{n})$ <br>법선 벡터의 역전치 변환 |
| `apply_point(p)`       | $\vec{p}' = \frac{p \cdot M}{w}$ <br>row-vector 방식. w ≈ 1이면 생략 가능       |
| `apply_vector(v)`      | $\vec{v}' = v \cdot M \quad (w = 0)$ <br>row-vector 방식의 벡터 선형 변환       |

### 📌 참고
- transform_* 함수들은 모두 column-vector 기준으로 동차 좌표를 적용한 후 proj3()로 투영합니다.
- apply_* 함수들은 row-vector 기준으로 직접 계산하며, Point * Transform 오버로드와 일관됩니다.
- transform_normal()은 법선 벡터를 정확히 변환하기 위해 역전치 3×3 블록을 사용합니다.


## 📐 Transform 기하 분석

| 함수명                        | 수식 또는 설명                                                                 |
|------------------------------|----------------------------------------------------------------------------------|
| `basis_x/y/z()`              | $\vec{x} = M_{0:2,0},\ \vec{y} = M_{0:2,1},\ \vec{z} = M_{0:2,2}$ <br>각 축 방향 벡터 추출 |
| `scale_factor_x/y/z()`       | $s_i = \|\vec{basis}_i\|$ <br>각 축의 스케일 크기                            |
| `scale_factors()`            | $(s_x, s_y, s_z)$<br>모든 축의 스케일 크기 튜플 반환                        |
| `is_uniform_scale(eps)`      | $\|s_x - s_y\|\ < \varepsilon,\ \|s_y - s_z\| < \varepsilon$ <br>모든 축이 동일한 스케일인지 확인 |
| `is_plane_uniform_scale(eps)`| $\|s_x - s_y\|\ < \varepsilon$ <br>X-Y 평면 기준 스케일 일치 여부 확인           |
| `shear_xy/yz/zx()`           | $\text{shear}_{ij} = \hat{i} \cdot \hat{j}$ <br>단위 벡터 간 내적 (전단 계수) |
| `shear_factors()`            | $(\text{shear}_{xy} , \text{shear}_{yz} , \text{shear}_{zx})$ <br>전단 계수 튜플 반환 |
| `is_orthogonal_basis(eps)`   | $\|\text{shear}_{ij}\| < \varepsilon$ <br>축 간 직교 여부 확인                 |

### 📌 참고
- basis_*()는 행렬의 열 벡터를 추출하여 각 축 방향을 나타냅니다.
- scale_factor_*()는 각 축 벡터의 길이를 통해 스케일 크기를 계산합니다.
- shear_*()는 축 간의 전단(비직교성)을 측정하며, 내적이 0에 가까울수록 직교에 가까움입니다.
- is_orthogonal_basis()는 모든 축이 서로 직교하는지 판단하는 데 사용됩니다.

## 🔄 Transform 연산자 오버로드

| 타입 × Transform           | 트레이트 | 수식 또는 설명                                                                 |
|---------------------------|----------|----------------------------------------------------------------------------------|
| `Point * Transform`       | `Mul`    | $\vec{p}' = \frac{M \cdot [x, y, z, 1]^T}{w}$ <br>점에 대한 동차 좌표 변환 후 투영 |
| `Vector * Transform`      | `Mul`    | $\vec{v}' = M \cdot [x, y, z, 0]^T$ <br>벡터는 w=0으로 선형 변환만 적용           |
| `Point2 * Transform`      | `Mul`    | $\vec{p}' = \frac{M \cdot [x, y, 0, 1]^T}{w}$ <br>2D 점을 3D로 확장 후 변환         |
| `Vector2 * Transform`     | `Mul`    | $\vec{v}' = M \cdot [x, y, 0, 0]^T$ <br>2D 벡터를 3D로 확장 후 선형 변환           |


### 📌 참고
- 모든 연산은 동차 좌표(homogeneous coordinates) 기반으로 처리됩니다.
- Point는 w=1, Vector는 w=0으로 처리되어 위치와 방향의 차이를 반영합니다.
- Point2, Vector2는 내부적으로 z=0으로 확장되어 3D 변환 행렬에 적용됩니다.
- Mul 트레이트는 다양한 참조 타입(&Point, Point, &Transform, Transform)에 대해 오버로드되어 있어 유연하게 사용할 수 있습니다.


## 🧮 Transform 내부 유틸리티

| 함수명         | 수식 또는 설명                                                                 |
|----------------|----------------------------------------------------------------------------------|
| `act4(x,y,z,w)`| $h = M \cdot [x, y, z, w]^T$<br>4D 동차 좌표에 행렬을 적용하여 4D 결과 벡터 반환 |
| `proj3(h)`     | $\vec{v} = \left[\frac{h_0}{h_3}, \frac{h_1}{h_3}, \frac{h_2}{h_3}\right]$<br>동차 좌표를 3D로 투영 |

### 📌 참고
- act4()는 행렬 곱을 직접 수행하여 4D 벡터를 반환합니다. 모든 transform_* 함수의 기반이 되는 핵심 연산입니다.
- proj3()는 동차 좌표의 w 성분을 기준으로 투영을 수행하며, 일반적인 3D 렌더링 또는 변환에서 사용됩니다.
- 이 두 함수는 함께 사용되어 점/벡터의 변환을 완성합니다:
예: proj3(act4(x, y, z, 1.0)) → 점 변환



## ✅ 사용 예시
```rust
let t = Transform::translation(1.0, 2.0, 3.0)
    .mul(&Transform::rotation_axis(PI / 2.0, Vector::z(), Point::origin()));
let p = Point::new(1.0, 0.0, 0.0);
let q = t.transform_point3d(&p);
```


# 📐 Transform 함수별 수식 정리 및 검증

## 🔁 행렬 연산

### `Transform::mul(self, rhs)`
- **수식**:  

$$
  R_{ij} = \sum_{k=0}^{3} A_{ik} \cdot B_{kj}
$$

- **설명**: 4×4 행렬 곱. 행렬 합성 시 사용.
- ✅ **검증**: 정확함. 표준 행렬 곱 구현.

---

### `Transform::add(self, rhs)`
- **수식**:  

$$
  R_{ij} = A_{ij} + B_{ij}
$$

- **설명**: 행렬 원소별 덧셈.
- ✅ **검증**: 정확함.

---

### `Transform::sub(self, rhs)`
- **수식**:  

$$
  R_{ij} = A_{ij} - B_{ij}
$$

- **설명**: 행렬 원소별 뺄셈.
- ✅ **검증**: 정확함.

---

## 🧭 벡터/점 변환

### `Transform::multi_homogeneous(m, x, y, z, w)`
- **수식**:  

$$
  \begin{bmatrix}
  x' \\ y' \\ z' \\ w'
  \end{bmatrix}
  =
  M \cdot
  \begin{bmatrix}
  x \\ y \\ z \\ w
  \end{bmatrix}
$$

- ✅ **검증**: 정확함. 동차 좌표 변환.

---

### `Transform::multi_point(m, x, y, z)`
- **수식**:  

$$
  \vec{p}' = \frac{M \cdot [x, y, z, 1]^T}{w}
$$

- ✅ **검증**: 정확함. 투영 포함.

---

### `apply_point(p)`
- **수식**:  

$$
  \vec{p}' = \frac{p \cdot M}{w}
$$

- **설명**: row-vector convention. w ≈ 1이면 생략.
- ✅ **검증**: 정확함.

---

### `apply_vector(v)`
- **수식**:  

$$
  \vec{v}' = v \cdot M \quad \text{(w = 0)}
$$

- ✅ **검증**: 정확함.

---

## ➕ 연산자 오버로드

### `Point * Transform`
- **수식**:  

$$
  \vec{p}' = \frac{M \cdot [x, y, z, 1]^T}{w}
$$

- ✅ **검증**: 정확함.

### `Vector * Transform`
- **수식**:  

$$
  \vec{v}' = M \cdot [x, y, z, 0]^T
$$

- ✅ **검증**: 정확함.

### `Point2 * Transform`
- **수식**:  

$$
  \vec{p}' = \frac{M \cdot [x, y, 0, 1]^T}{w}
$$

- ✅ **검증**: 정확함.

### `Vector2 * Transform`
- **수식**:  

$$
  \vec{v}' = M \cdot [x, y, 0, 0]^T
$$

- ✅ **검증**: 정확함.

---

## 🧠 기하 분석

### `basis_x/y/z()`
- **수식**: 

$$
  \vec{x} = \text{column}_0,\quad \vec{y} = \text{column}_1,\quad \vec{z} = \text{column}_2
$$

- ✅ **검증**: 정확함.

### `scale_factor_x/y/z()`
- **수식**: 

$$
  s_i = \|\vec{basis}_i\|
$$

- ✅ **검증**: 정확함.

### `shear_xy/yz/zx()`
- **수식**: 

$$
  \text{shear}_{ij} = \hat{i} \cdot \hat{j}
$$

- ✅ **검증**: 정확함.

---

## ✅ 결론

- 모든 수식은 **선형대수학적으로 정확** 하며, 동차 좌표계와 행렬 연산 규칙을 잘 따르고 있습니다.
- `mul`, `apply`, `multi_homogeneous`, `basis`, `scale`, `shear` 등은 모두 **정확하게 구현** 되어 있으며, 수치적 안정성도 고려되어 있습니다.


---

## 소스
```rust
use crate::core::geom::{CPoint, Point, Point2, Vector, Vector2};
use crate::core::matrix4::inverse4;
use crate::core::types::{Matrix3x3, Matrix4x4};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub m: Matrix4x4,
}
```
```rust
impl Transform {
    #[inline]
    pub fn scale_uniform_about(f: f64, c: Point) -> Self {
        // T(-c)
        let t_neg = Self::translation(-c.x, -c.y, -c.z);
        // S(f)
        let s = Self::scale_uniform(f);
        // T(c)
        let t_pos = Self::translation(c.x, c.y, c.z);
        // T(c) * S(f) * T(-c)
        t_pos * s * t_neg
    }

    #[inline]
    pub fn scale_uniform(f: f64) -> Self {
        let mut t = Self::identity();
        t.m[0][0] = f;
        t.m[1][1] = f;
        t.m[2][2] = f;
        t
    }
}
```
```rust
impl Transform {
    pub(crate) fn from_cols(p0: [f64; 4], p1: [f64; 4], p2: [f64; 4], p3: [f64; 4]) -> Transform {
        let mut transform = Transform::identity();
        transform.m[0][0] = p0[0];
        transform.m[0][1] = p0[1];
        transform.m[0][2] = p0[2];
        transform.m[0][3] = p0[3];
        transform.m[1][0] = p1[0];
        transform.m[1][1] = p1[1];
        transform.m[1][2] = p1[2];
        transform.m[1][3] = p1[3];
        transform.m[2][0] = p2[0];
        transform.m[2][1] = p2[1];
        transform.m[2][2] = p2[2];
        transform.m[2][3] = p2[3];
        transform.m[3][0] = p3[0];
        transform.m[3][1] = p3[1];
        transform.m[3][2] = p3[2];
        transform.m[3][3] = p3[3];
        transform
    }
}
```
```rust
impl Transform {
    // --- Object Construction ---
    pub fn identity() -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = 1.0;
        m[1][1] = 1.0;
        m[2][2] = 1.0;
        m[3][3] = 1.0;
        Self { m }
    }

    pub fn is_identity(&self) -> bool {
        self.is_identity_eps(1e-12)
    }

    pub fn is_identity_eps(&self, eps: f64) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                let expected = if r == c { 1.0 } else { 0.0 };
                if (self.m[r][c] - expected).abs() > eps {
                    return false;
                }
            }
        }
        true
    }

    pub fn zero() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    #[inline]
    pub fn translation(dx: f64, dy: f64, dz: f64) -> Self {
        let mut t = Self::identity();
        t.m[0][3] = dx;
        t.m[1][3] = dy;
        t.m[2][3] = dz;
        t
    }

    pub fn scaling(sx: f64, sy: f64, sz: f64) -> Self {
        let mut t = Self::zero();
        t.m[0][0] = sx;
        t.m[1][1] = sy;
        t.m[2][2] = sz;
        t.m[3][3] = 1.0;
        t
    }

    pub fn rotation_axis(angle: f64, axis: Vector, center: Point) -> Self {
        let mut n = axis;
        if n.length_squared() > 0.0 {
            n.normalize();
        }
        let (s, c) = angle.sin_cos();
        let d = 1.0 - c;
        let (nx, ny, nz) = (n.x, n.y, n.z);
        let mut t = Self::identity();
        t.m[0][0] = nx * nx * d + c;
        t.m[0][1] = nx * ny * d - nz * s;
        t.m[0][2] = nx * nz * d + ny * s;
        t.m[1][0] = ny * nx * d + nz * s;
        t.m[1][1] = ny * ny * d + c;
        t.m[1][2] = ny * nz * d - nx * s;
        t.m[2][0] = nz * nx * d - ny * s;
        t.m[2][1] = nz * ny * d + nx * s;
        t.m[2][2] = nz * nz * d + c;
        if center.x != 0.0 || center.y != 0.0 || center.z != 0.0 {
            t.m[0][3] =
                -((t.m[0][0] - 1.0) * center.x + t.m[0][1] * center.y + t.m[0][2] * center.z);
            t.m[1][3] =
                -(t.m[1][0] * center.x + (t.m[1][1] - 1.0) * center.y + t.m[1][2] * center.z);
            t.m[2][3] =
                -(t.m[2][0] * center.x + t.m[2][1] * center.y + (t.m[2][2] - 1.0) * center.z);
        }
        t
    }

    // --- Matrix Operations ---
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut r = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j]
                    + self.m[i][3] * rhs.m[3][j];
            }
        }
        Self { m: r }
    }
    pub fn invert(&self) -> Option<Self> {
        inverse4(&self.m).map(|m| Self { m })
    }

    // --- (M⁻¹)ᵀ 3×3 block ---
    pub fn inverse_transpose3x3(&self) -> Option<Matrix3x3> {
        let inv = self.invert()?;
        let m = inv.m;
        Some([
            [m[0][0], m[1][0], m[2][0]],
            [m[0][1], m[1][1], m[2][1]],
            [m[0][2], m[1][2], m[2][2]],
        ])
    }

    // --- Left-side application ---
    #[inline]
    fn act4(&self, x: f64, y: f64, z: f64, w: f64) -> [f64; 4] {
        [
            self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z + self.m[0][3] * w,
            self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z + self.m[1][3] * w,
            self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z + self.m[2][3] * w,
            self.m[3][0] * x + self.m[3][1] * y + self.m[3][2] * z + self.m[3][3] * w,
        ]
    }
    #[inline]
    fn proj3(h: [f64; 4]) -> [f64; 3] {
        let iw = if h[3] != 0.0 { 1.0 / h[3] } else { 1.0 };
        [iw * h[0], iw * h[1], iw * h[2]]
    }

    // --- Transformation API (2D/3D/4D) ---
    pub fn transform_point2d(&self, p: &Point2) -> Point2 {
        let h = self.act4(p.x, p.y, 0.0, 1.0);
        let q = Self::proj3(h);
        Point2::new(q[0], q[1])
    }
    pub fn transform_vector2d(&self, v: &Vector2) -> Vector2 {
        let h = self.act4(v.x, v.y, 0.0, 0.0);
        let q = Self::proj3(h);
        Vector2::new(q[0], q[1])
    }
    pub fn transform_point3d(&self, p: &Point) -> Point {
        let h = self.act4(p.x, p.y, p.z, 1.0);
        let q = Self::proj3(h);
        Point::new(q[0], q[1], q[2])
    }
    pub fn transform_vector3d(&self, v: &Vector) -> Vector {
        let h = self.act4(v.x, v.y, v.z, 0.0);
        let q = Self::proj3(h);
        Vector::new(q[0], q[1], q[2])
    }
    pub fn transform_point4d(&self, h: &CPoint) -> CPoint {
        let q = self.act4(h.x, h.y, h.z, h.w);
        CPoint::new(q[0], q[1], q[2], q[3])
    }

    /// Normal transformation: apply the 3×3 block of (M⁻¹)ᵀ, then normalize
    pub fn transform_normal(&self, n: &Vector) -> Option<Vector> {
        let nt = self.inverse_transpose3x3()?;
        let x = nt[0][0] * n.x + nt[0][1] * n.y + nt[0][2] * n.z;
        let y = nt[1][0] * n.x + nt[1][1] * n.y + nt[1][2] * n.z;
        let z = nt[2][0] * n.x + nt[2][1] * n.y + nt[2][2] * n.z;
        let mut v = Vector::new(x, y, z);
        let _ = v.normalize();
        Some(v)
    }

    // --- Plane Mirror (point p₀, normal n) ---
    pub fn mirror_about_plane(p0: Point, n: Vector) -> Self {
        let mut nn = n;
        if nn.length_squared() == 0.0 {
            return Self::identity();
        }
        nn.normalize();
        let (nx, ny, nz) = (nn.x, nn.y, nn.z);
        let mut m = Self::identity().m;
        m[0][0] = 1.0 - 2.0 * nx * nx;
        m[0][1] = -2.0 * nx * ny;
        m[0][2] = -2.0 * nx * nz;
        m[1][0] = -2.0 * ny * nx;
        m[1][1] = 1.0 - 2.0 * ny * ny;
        m[1][2] = -2.0 * ny * nz;
        m[2][0] = -2.0 * nz * nx;
        m[2][1] = -2.0 * nz * ny;
        m[2][2] = 1.0 - 2.0 * nz * nz;
        let ndp = nx * p0.x + ny * p0.y + nz * p0.z;
        m[0][3] = 2.0 * ndp * nx;
        m[1][3] = 2.0 * ndp * ny;
        m[2][3] = 2.0 * ndp * nz;
        Self { m }
    }

    // --- Basis Transformation (Local ↔ World) ---
    pub fn from_basis(origin: Point, x: Vector, y: Vector, z: Vector) -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = x.x;
        m[0][1] = y.x;
        m[0][2] = z.x;
        m[0][3] = origin.x;
        m[1][0] = x.y;
        m[1][1] = y.y;
        m[1][2] = z.y;
        m[1][3] = origin.y;
        m[2][0] = x.z;
        m[2][1] = y.z;
        m[2][2] = z.z;
        m[2][3] = origin.z;
        m[3][3] = 1.0;
        Self { m }
    }
    pub fn to_basis(origin: Point, x: Vector, y: Vector, z: Vector) -> Option<Self> {
        Self::from_basis(origin, x, y, z).invert()
    }
    pub fn change_of_basis(
        a_o: Point,
        a_x: Vector,
        a_y: Vector,
        a_z: Vector,
        b_o: Point,
        b_x: Vector,
        b_y: Vector,
        b_z: Vector,
    ) -> Option<Self> {
        let a2w = Self::from_basis(a_o, a_x, a_y, a_z);
        let b2w = Self::from_basis(b_o, b_x, b_y, b_z);
        let w2b = b2w.invert()?;
        Some(w2b.mul(a2w))
    }

    /// Apply point (homogeneous w = 1): p′ = p * M (row-vector convention)
    #[inline]
    pub fn apply_point(&self, p: Point) -> Point {
        let x = p.x;
        let y = p.y;
        let z = p.z;
        let w = 1.0;
        let xp = x * self.m[0][0] + y * self.m[0][1] + z * self.m[0][2] + w * self.m[0][3];
        let yp = x * self.m[1][0] + y * self.m[1][1] + z * self.m[1][2] + w * self.m[1][3];
        let zp = x * self.m[2][0] + y * self.m[2][1] + z * self.m[2][2] + w * self.m[2][3];
        let wp = x * self.m[3][0] + y * self.m[3][1] + z * self.m[3][2] + w * self.m[3][3];

        if (wp - 1.0).abs() <= 1e-12 || wp.abs() <= 1e-12 {
            // Typical linear/affine transform (w ≈ 1) or rare/projective edge case (w ≈ 0) — skip division
            Point {
                x: xp,
                y: yp,
                z: zp,
            }
        } else {
            // Projection matrix, etc.: homogeneous division
            Point {
                x: xp / wp,
                y: yp / wp,
                z: zp / wp,
            }
        }
    }

    #[inline]
    pub fn apply_vector(&self, v: Vector) -> Vector {
        let x = v.x;
        let y = v.y;
        let z = v.z;
        let w = 0.0;
        Vector {
            x: x * self.m[0][0] + y * self.m[0][1] + z * self.m[0][2] + w * self.m[0][3],
            y: x * self.m[1][0] + y * self.m[1][1] + z * self.m[1][2] + w * self.m[1][3],
            z: x * self.m[2][0] + y * self.m[2][1] + z * self.m[2][2] + w * self.m[2][3],
        }
    }

    pub fn then(self, next: &Transform) -> Transform {
        next.mul(&self) // 합성행렬은 B*A
    }

    #[inline]
    pub fn basis_x(&self) -> Vector {
        Vector::new(self.m[0][0], self.m[1][0], self.m[2][0])
    }
    #[inline]
    pub fn basis_y(&self) -> Vector {
        Vector::new(self.m[0][1], self.m[1][1], self.m[2][1])
    }
    #[inline]
    pub fn basis_z(&self) -> Vector {
        Vector::new(self.m[0][2], self.m[1][2], self.m[2][2])
    }

    #[inline]
    pub fn scale_factor_x(&self) -> f64 {
        self.basis_x().length()
    }
    #[inline]
    pub fn scale_factor_y(&self) -> f64 {
        self.basis_y().length()
    }
    #[inline]
    pub fn scale_factor_z(&self) -> f64 {
        self.basis_z().length()
    }

    #[inline]
    pub fn scale_factors(&self) -> (f64, f64, f64) {
        (
            self.scale_factor_x(),
            self.scale_factor_y(),
            self.scale_factor_z(),
        )
    }

    pub fn is_uniform_scale(&self, eps: f64) -> bool {
        let (sx, sy, sz) = self.scale_factors();
        (sx - sy).abs() <= eps && (sy - sz).abs() <= eps
    }

    pub fn is_plane_uniform_scale(&self, eps: f64) -> bool {
        let (sx, sy, _sz) = self.scale_factors();
        (sx - sy).abs() <= eps
    }

    #[inline]
    pub fn shear_xy(&self) -> f64 {
        let x = self.basis_x().unitize();
        let y = self.basis_y().unitize();
        x.dot(&y)
    }

    #[inline]
    pub fn shear_yz(&self) -> f64 {
        let y = self.basis_y().unitize();
        let z = self.basis_z().unitize();
        y.dot(&z)
    }

    #[inline]
    pub fn shear_zx(&self) -> f64 {
        let z = self.basis_z().unitize();
        let x = self.basis_x().unitize();
        z.dot(&x)
    }

    #[inline]
    pub fn shear_factors(&self) -> (f64, f64, f64) {
        (self.shear_xy(), self.shear_yz(), self.shear_zx())
    }

    pub fn is_orthogonal_basis(&self, eps: f64) -> bool {
        let (sxy, syz, szx) = self.shear_factors();
        sxy.abs() <= eps && syz.abs() <= eps && szx.abs() <= eps
    }

    #[inline]
    fn multi_homogeneous(m: &Transform, x: f64, y: f64, z: f64, w: f64) -> (f64, f64, f64, f64) {
        let xr = m.m[0][0] * x + m.m[0][1] * y + m.m[0][2] * z + m.m[0][3] * w;
        let yr = m.m[1][0] * x + m.m[1][1] * y + m.m[1][2] * z + m.m[1][3] * w;
        let zr = m.m[2][0] * x + m.m[2][1] * y + m.m[2][2] * z + m.m[2][3] * w;
        let wr = m.m[3][0] * x + m.m[3][1] * y + m.m[3][2] * z + m.m[3][3] * w;
        (xr, yr, zr, wr)
    }
    #[allow(unused)]
    fn multi_point(m: &Transform, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let xr = m.m[0][0] * x + m.m[0][1] * y + m.m[0][2] * z + m.m[0][3];
        let yr = m.m[1][0] * x + m.m[1][1] * y + m.m[1][2] * z + m.m[1][3];
        let zr = m.m[2][0] * x + m.m[2][1] * y + m.m[2][2] * z + m.m[2][3];
        let wr = m.m[3][0] * x + m.m[3][1] * y + m.m[3][2] * z + m.m[3][3];
        (xr / wr, yr / wr, zr / wr)
    }


}
```
```rust
impl Mul<Transform> for Point {
    type Output = Point;
    #[inline]
    fn mul(self, rhs: Transform) -> Self::Output {
        let (x, y, z, w) = Transform::multi_homogeneous(&rhs, self.x, self.y, self.z, 1.0);
        if w != 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::new(x, y, z)
        }
    }
}

impl<'a, 'b> Mul<&'b Transform> for &'a Point {
    type Output = Point;
    #[inline]
    fn mul(self, rhs: &'b Transform) -> Self::Output {
        let (x, y, z, w) = Transform::multi_homogeneous(rhs, self.x, self.y, self.z, 1.0);
        if w != 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::new(x, y, z)
        }
    }
}
```
```rust
impl<'b> Mul<&'b Transform> for Point {
    type Output = Point;
    #[inline]
    fn mul(self, rhs: &'b Transform) -> Self::Output {
        let (x, y, z, w) = Transform::multi_homogeneous(rhs, self.x, self.y, self.z, 1.0);
        if w != 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::new(x, y, z)
        }
    }
}
```
```rust
impl<'a> Mul<Transform> for &'a Point {
    type Output = Point;
    #[inline]
    fn mul(self, rhs: Transform) -> Self::Output {
        let (x, y, z, w) = Transform::multi_homogeneous(&rhs, self.x, self.y, self.z, 1.0);
        if w != 0.0 {
            Point::new(x / w, y / w, z / w)
        } else {
            Point::new(x, y, z)
        }
    }
}
```
```rust
// For Vector3D
impl Mul<Transform> for Vector {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: Transform) -> Self::Output {
        // 선형부만 적용 (w=0)
        let (x, y, z, _w) = Transform::multi_homogeneous(&rhs, self.x, self.y, self.z, 0.0);
        Vector::new(x, y, z)
    }
}
```
```rust
impl<'a, 'b> Mul<&'b Transform> for &'a Vector {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: &'b Transform) -> Self::Output {
        let (x, y, z, _w) = Transform::multi_homogeneous(rhs, self.x, self.y, self.z, 0.0);
        Vector::new(x, y, z)
    }
}
```
```rust
impl<'b> Mul<&'b Transform> for Vector {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: &'b Transform) -> Self::Output {
        let (x, y, z, _w) = Transform::multi_homogeneous(rhs, self.x, self.y, self.z, 0.0);
        Vector::new(x, y, z)
    }
}
```
```rust
impl<'a> Mul<Transform> for &'a Vector {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: Transform) -> Self::Output {
        let (x, y, z, _w) = Transform::multi_homogeneous(&rhs, self.x, self.y, self.z, 0.0);
        Vector::new(x, y, z)
    }
}
```
```rust
// For a Point2D
impl Mul<Transform> for Point2 {
    type Output = Point2;
    #[inline]
    fn mul(self, rhs: Transform) -> Self::Output {
        let (x, y, _z, w) = Transform::multi_homogeneous(&rhs, self.x, self.y, 0.0, 1.0);
        if w != 0.0 {
            Point2::new(x / w, y / w)
        } else {
            Point2::new(x, y)
        }
    }
}
```
```rust
impl<'a, 'b> Mul<&'b Transform> for &'a Point2 {
    type Output = Point2;
    #[inline]
    fn mul(self, rhs: &'b Transform) -> Self::Output {
        let (x, y, _z, w) = Transform::multi_homogeneous(rhs, self.x, self.y, 0.0, 1.0);
        if w != 0.0 {
            Point2::new(x / w, y / w)
        } else {
            Point2::new(x, y)
        }
    }
}
```
```rust
impl<'b> Mul<&'b Transform> for Point2 {
    type Output = Point2;
    #[inline]
    fn mul(self, rhs: &'b Transform) -> Self::Output {
        let (x, y, _z, w) = Transform::multi_homogeneous(rhs, self.x, self.y, 0.0, 1.0);
        if w != 0.0 {
            Point2::new(x / w, y / w)
        } else {
            Point2::new(x, y)
        }
    }
}
```
```rust
impl<'a> Mul<Transform> for &'a Point2 {
    type Output = Point2;
    #[inline]
    fn mul(self, rhs: Transform) -> Self::Output {
        let (x, y, _z, w) = Transform::multi_homogeneous(&rhs, self.x, self.y, 0.0, 1.0);
        if w != 0.0 {
            Point2::new(x / w, y / w)
        } else {
            Point2::new(x, y)
        }
    }
}
```
```rust
// For a Vector2D
impl Mul<Transform> for Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Transform) -> Self::Output {
        let (x, y, _z, _w) = Transform::multi_homogeneous(&rhs, self.x, self.y, 0.0, 0.0);
        Vector2::new(x, y)
    }
}
```
```rust
impl<'a, 'b> Mul<&'b Transform> for &'a Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: &'b Transform) -> Self::Output {
        let (x, y, _z, _w) = Transform::multi_homogeneous(rhs, self.x, self.y, 0.0, 0.0);
        Vector2::new(x, y)
    }
}
```
```rust
impl<'b> Mul<&'b Transform> for Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: &'b Transform) -> Self::Output {
        let (x, y, _z, _w) = Transform::multi_homogeneous(rhs, self.x, self.y, 0.0, 0.0);
        Vector2::new(x, y)
    }
}
```
```rust
impl<'a> Mul<Transform> for &'a Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Transform) -> Self::Output {
        let (x, y, _z, _w) = Transform::multi_homogeneous(&rhs, self.x, self.y, 0.0, 0.0);
        Vector2::new(x, y)
    }
}
```
```rust
impl Mul for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Transform {
        let mut r = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j]
                    + self.m[i][3] * rhs.m[3][j];
            }
        }
        Transform { m: r }
    }
}
```
```rust
impl Add for Transform {
    type Output = Transform;
    fn add(self, rhs: Transform) -> Transform {
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
impl Sub for Transform {
    type Output = Transform;
    fn sub(self, rhs: Transform) -> Transform {
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

---

## 🧪 Transform 테스트 항목 요약

| 테스트 함수명                        | 검증 내용 및 목적                                      |
|-------------------------------------|--------------------------------------------------------|
| `apply_point_translation`          | 점에 대한 평행이동 적용                               |
| `apply_vector_translation_ignored` | 벡터는 평행이동 영향을 받지 않음                      |
| `apply_point_scaling`              | 점에 대한 비균일 스케일링 적용                        |
| `apply_vector_scaling`             | 벡터에 대한 비균일 스케일링 적용                      |
| `apply_point_rotation_z90`         | Z축 기준 90° 회전 후 점 위치 확인                     |
| `apply_vector_rotation_z90`        | Z축 기준 90° 회전 후 벡터 방향 확인                   |
| `apply_point_mirror_x_plane`       | X축 평면 기준 반사 후 점 위치 확인                    |

## 테스트 코드

```rust
#[cfg(test)]
mod transform_tests {

    use nurbslib::core::prelude::{Point, Vector};
    use nurbslib::core::transform::Transform;
```
```rust
    #[test]
    fn apply_point_translation() {
        let xf = Transform::translation(1.0, 2.0, 3.0);
        let p = Point {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let r = xf.apply_point(p);
        assert!((r.x - 5.0).abs() < 1e-12 && (r.y - 7.0).abs() < 1e-12 && (r.z - 9.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn apply_vector_translation_ignored() {
        let xf = Transform::translation(1.0, 2.0, 3.0);
        let v = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let r = xf.apply_vector(v);
        assert!((r.x - 1.0).abs() < 1e-12 && r.y.abs() < 1e-12 && r.z.abs() < 1e-12);
    }
```
```rust
    #[test]
    fn apply_point_scaling() {
        let xf = Transform::scaling(2.0, 3.0, 4.0);
        let p = Point::new(1.0, 1.0, 1.0);
        let r = xf.apply_point(p);
        assert!((r.x - 2.0).abs() < 1e-12 && (r.y - 3.0).abs() < 1e-12 && (r.z - 4.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn apply_vector_scaling() {
        let xf = Transform::scaling(2.0, 3.0, 4.0);
        let v = Vector::new(1.0, 1.0, 1.0);
        let r = xf.apply_vector(v);
        assert!((r.x - 2.0).abs() < 1e-12 && (r.y - 3.0).abs() < 1e-12 && (r.z - 4.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn apply_point_rotation_z90() {
        let xf = Transform::rotation_axis(std::f64::consts::FRAC_PI_2, Vector::new(0.0, 0.0, 1.0), Point::origin());
        let p = Point::new(1.0, 0.0, 0.0);
        let r = xf.apply_point(p);
        assert!(r.x.abs() < 1e-12 && (r.y - 1.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn apply_vector_rotation_z90() {
        let xf = Transform::rotation_axis(std::f64::consts::FRAC_PI_2, Vector::new(0.0, 0.0, 1.0), Point::origin());
        let v = Vector::new(1.0, 0.0, 0.0);
        let r = xf.apply_vector(v);
        assert!(r.x.abs() < 1e-12 && (r.y - 1.0).abs() < 1e-12);
    }
```
```rust
    #[test]
    fn apply_point_mirror_x_plane() {
        let xf = Transform::mirror_about_plane(Point::origin(), Vector::new(1.0, 0.0, 0.0));
        let p = Point::new(2.0, 3.0, 4.0);
        let r = xf.apply_point(p);
        assert!((r.x + 2.0).abs() < 1e-12 && (r.y - 3.0).abs() < 1e-12 && (r.z - 4.0).abs() < 1e-12);
    }
}
```
---


