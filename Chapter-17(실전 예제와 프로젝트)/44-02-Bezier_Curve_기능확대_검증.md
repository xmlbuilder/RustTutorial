## Bezier curve implementation overview
- 각 함수의 목적과 단계를 설명합니다.

### Data structures and core conventions
- BezierCurve:
    - degree: 곡선 차수 p (제어점 개수 (n=p+1))
    - ctrl: 제어점 배열. Point4D 형태로 저장되며, 합리적(유리) 표현을 위해 $(xw,yw,zw,w)$ 규칙을 따릅니다.
- Rational convention:
    - Label: 저장 규칙
    - 제어점은 일반적으로 $(x\cdot w,y\cdot w,z\cdot w,w)$ 로 저장됩니다.
    - 유리 평가 시 분자/분모 형태로 나누어 유클리드 좌표를 복원합니다.
- Support types: Point3D, Point4D, Vector3D, BezierFunction, BezierArc.

### Evaluation and rational point computation
- Bernstein basis and point evaluation
- Point evaluation (non-homogeneous):
    - 함수: point_at(u)
    - Formula:


$$
P(u)=\sum _{i=0}^pB_i^p(u)\, \mathbf{P_{\mathnormal{i}}}
$$


- 여기서 B_i^p(u)는 베르슈타인 기저, 

$$
\mathbf{P_{\mathnormal{i}}}=(x_i,y_i,z_i).
$$

- Homogeneous control evaluation:
    - 함수: point_at_rational(u)
    - Formula:

$$
C(u)=\sum _{i=0}^pB_i^p(u)\, \mathbf{C_{\mathnormal{i}}},\quad \mathbf{C_{\mathnormal{i}}}=(x_i,y_i,z_i,w_i)
$$

- Rational evaluation:
    - 함수: evaluate_cpoint_rational(t)
    - If rat = true:
        - Formula:

$$
X_w=\sum _iB_i^p(t) (x_iw_i),\quad Y_w=\sum _iB_i^p(t) (y_iw_i), \quad Z_w=\sum _iB_i^p(t) (z_iw_i),\quad W=\sum _iB_i^p(t) w_i
$$

  - 반환: $(X_w,Y_w,Z_w,W)$, 유클리드 변환은 $(X_w/W,Y_w/W,Z_w/W)$.

- If rat = false:
  - Formula:

$$
X=\sum _iB_i^p(t)\, x_i,\quad Y=\sum _iB_i^p(t) y_i,\quad Z=\sum _iB_i^p(t) z_i
$$


- 반환: $(X,Y,Z,1)$.

### Bernstein computation path:
- on_all_ber_1d(p, t) 또는 내부 de Casteljau 스타일 누적 로직으로 올바르게 계산합니다.
- Derivatives, tangent, curvature, torsionHomogeneous derivatives up to third order
  - Function: eval_h_ders(t, d_max)
  - Approach:
    - 제어점을 (xw,yw,zw,w)로 변환한 뒤, 미분 제어점 집합을 생성.
- First derivatives control points:

$$
\mathbf{Q_{\mathnormal{i}}}=p(\mathbf{P_{\mathnormal{i+1}}}-\mathbf{P_{\mathnormal{i}}}),\quad i=0,\ldots ,p-1
$$

- Second derivatives control points:

$$
\mathbf{R_{\mathnormal{i}}}=(p-1)(\mathbf{Q_{\mathnormal{i+1}}}-\mathbf{Q_{\mathnormal{i}}}),\quad i=0,\ldots ,p-2
$$

- Third derivatives control points:

$$
\mathbf{S_{\mathnormal{i}}}=(p-2)(\mathbf{R_{\mathnormal{i+1}}}-\mathbf{R_{\mathnormal{i}}}),\quad i=0,\ldots ,p-3
$$

- 각 집합을 동일한 베르슈타인 평가로 값 계산.

### Non-homogeneous derivatives reconstruction
- Function: eval_point_and_ders(t, ders_order)
- 0th:

$$
\mathbf{p}=\frac{(X_w,Y_w,Z_w)}{W}
$$

- 1st derivative:

$$
\mathbf{p}'=\frac{\mathbf{v_{\mathnormal{1}}}W-\mathbf{v_{\mathnormal{0}}}W'}{W^2}
$$

- 여기서 $\mathbf{v_{\mathnormal{k}}}=(X_k,Y_k,Z_k),\; W_k$ 는 k차 동차값의 w-성분.

- 2nd derivative:

$$
\mathbf{p}''=\frac{W^2\mathbf{v_{\mathnormal{2}}}-2WW'\mathbf{v_{\mathnormal{1}}}-\mathbf{v_{\mathnormal{0}}}(WW''-2(W')^2)}{W^3}
$$

- 3rd derivative:

$$
\mathbf{p}'''=\frac{W^3\mathbf{v_{\mathnormal{3}}}-3W^2W'\mathbf{v_{\mathnormal{2}}}-3W\mathbf{v_{\mathnormal{1}}}(WW''-2(W')^2)-\mathbf{v_{\mathnormal{0}}}(W^2W'''-6WW'W''+6(W')^3)}{W^4}
$$

- Tangent: tangent(t)은 

$$
\mathbf{p}'/\| \mathbf{p}'\| .
$$

- Curvature:

$$
\kappa (t)=\frac{\| \mathbf{p}'(t)\times \mathbf{p}''(t)\| }{\| \mathbf{p}'(t)\| ^3}
$$

- Torsion:

$$
\tau (t)=\frac{\det (\mathbf{p}',\mathbf{p}'',\mathbf{p}''')}{\| \mathbf{p}'\times \mathbf{p}''\| ^2}=\frac{(\mathbf{p}'\times \mathbf{p}'')\cdot \mathbf{p}'''}{\| \mathbf{p}'\times \mathbf{p}''\| ^2}
$$


- Notes: 극소 $\| \mathbf{p}'\|$  또는 $\| \mathbf{p}'\times \mathbf{p}''\|$ 에서는 수치적으로 불안정할 수 있어 None 반환 보호가 적절합니다.

### Construction, conversion, and manipulationCreation and rational toggling
- Create: new(ctrl)는 자동으로 degree 계산, create(degree, ctrl)는 검증 후 생성.
- Rational checks:
    - Label: 판정
        - is_rational()은 가중치 배열로 합리 곡선 여부 판정.
    - Label: 비합리화
        - make_non_rational()은 모든 제어점의 w를 1로 만들고 좌표를 나눠 유클리드로 변환.
    - Label: 합리화
        - make_rational_with_weights(weights)는 길이 검증 후 새로운 w를 적용하고 좌표를 $(x,y,z)\mapsto (xw,yw,zw)$ 로 갱신.

### Degree elevation and reduction

- Elevation: elevate_degree(t)는 표준 승차수 행렬로 새 제어점을 계산.
- Formula:
    - 승차수 행렬 E를 사용해 

$$    
    \mathbf{C}'=E\mathbf{C}.
$$

- Reduction (approximate): reduce_degree(target_deg)
- Method: 균등 파라미터 $t_i=i/q$ 에서 유리 점을 샘플링 후 제어점으로 사용.
- Note: 수치적으로 **근사** 입니다. 최소제곱 기반의 베지어 축소와는 다릅니다. 정밀도가 필요하면 LSQ로 대체 권장.


### Reparameterization

- Linear affine mapping: on_re_param_affine(a,b,ap,bp)
- Formula:

$$
\alpha =\frac{b-a}{bp-ap},\quad \beta =\frac{bp\cdot a-ap\cdot b}{bp-ap},\quad u=\alpha u'+\beta 
$$

- Matrix M: on_re_param_matrix(p, a,b, ap,bp)
- Steps:
- Label: R 행렬
    - $(\alpha u'+\beta )^i$ 전개로 R 구성.
- Label: Basis transforms
    - T= Bezier→Power, P= Power→Bezier.
- Label: Final

$$
M=P\cdot R\cdot T
$$

- 반환된 M을 사용해 계수 변환 $c'=Mc$.
- Inverse matrix: on_re_param_inverse_matrix(...)
- Formula:

$$
M^{-1}=T^{-1}\cdot R^{-1}\cdot P^{-1}
$$

- 여기서 $R^{-1}$ 은 $(\alpha t+\beta )^i$ 의 역방향 계수. nalgebra로 곱한 뒤 2D 벡터로 복원.


### NURBS conversion
- Function: to_nurbs()
    - Knot vector: 클램프형: $[0,0,\ldots ,0,1,1,\ldots ,1]$ with multiplicity p+1.
    - Degree: 동일한 p.
    - Domain: [0,1].
    - 합리 베지어는 그대로 합리 NURBS로 표현할 수 있습니다.

### Splitting (de Casteljau)
- Function: split(u)
    - Method: 표준 de Casteljau 반복으로 왼쪽·오른쪽 베지어 제어점 생성.
    - Result: 두 개의 동일 차수 베지어 곡선 반환.

### Arc, length, Hermite cubic
- Conic arc to Bezier: conic_arc_to_bezier(...)
- on_make_bezier_conic_arc 결과로 중간 제어점과 weight을 받아 2차 합리 베지어를 구성합니다. 
- Arc length (approximation): on_bezier_arc_len(ctrl, tol)
    - Method:
    - Label: chord vs polyline

### 폴리라인 길이 합(제어 폴리곤 간 거리 합)과 chord 길이의 차가 임계 이하이면 종료.
- Label: recursion
- 아니면 분할 후 재귀.
- Note: 표준 근사 방식이지만, 튜닝을 위해 tol 스케일링과 분할 기준을 손볼 수 있습니다.
- Hermite to cubic Bezier: CubicBezier::from_hermite(...)
  - Formula:

$$
\mathbf{b_{\mathnormal{0}}}=p_0,\quad \mathbf{b_{\mathnormal{3}}}=p_1,\quad \mathbf{b_{\mathnormal{1}}}=p_0+\frac{h}{3}t_0,\quad \mathbf{b_{\mathnormal{2}}}=p_1-\frac{h}{3}t_1
$$

- Point evaluation: 표준 cubic 베지어 식을 정확히 사용합니다.

### Dot and cross between curves
- Dot product curve coefficients: dot(rhs)
  - Formula: convolution 형태로 계수 계산:

$$
d_i=\sum _j\langle \mathbf{P_{\mathnormal{j}}},\mathbf{Q_{\mathnormal{i-j}}}\rangle 
$$

- 결과는 스칼라 베지어 계수 벡터입니다.
- Cross product curve: cross(rhs)
  - Formula:

$$
\mathbf{R_{\mathnormal{i}}}=\sum _j(\mathbf{P_{\mathnormal{j}}}\times \mathbf{Q_{\mathnormal{i-j}}})
$$

- 결과는 벡터값 베지어를 Point4D로 저장하고 w=1로 설정합니다.

### Utilities and miscellaneous

- Bounding box: bounding_box(samples)는 샘플링 기반.
- Reverse: 제어점 역순.
- Translate/Scale/transform_linear: 합리 좌표를 고려해 w를 곱하거나 유지하는 방식으로 안전하게 변환.
- Degree-3 chord parameters: on_compute_bezier_degree3chord_parameters(q)는 chord-length 기반 내부 매개변수 추정으로 타당합니다. 예외 시 1/3, 2/3 기본값.

---
## 소스 코드
```rust
use crate::core::basis::{on_all_ber_1d, on_bernstein, on_bezier_to_power_matrix, on_binomial_usize, on_degree_elevation_matrix, on_fit_cubic_bezier_through4points, on_power_to_bezier_vec, on_product_matrix};
use crate::core::curve_utils::is_rational_ctrl_array;
use crate::core::domain::Interval;
use crate::core::geom_trait::Curve;
use crate::core::knot::on_is_rat;
use crate::core::maths::on_make_bezier_conic_arc;
use crate::core::matrix::Matrix;
use crate::core::prelude::{Degree, KnotVector, NurbsCurve, Point3D, Point4D, Real, Vector3D};
use nalgebra::{DMatrix, DVector};
use crate::core::types::ON_TOL12;

#[derive(Debug, Clone)]
pub struct BezierCurve {
    pub degree: usize,
    pub ctrl: Vec<Point4D>,
}
```
```rust
impl Curve for BezierCurve {}
```
```rust
impl BezierCurve {

    pub fn new(control_points: Vec<Point4D>) -> Self {
        let degree = control_points.len().saturating_sub(1);
        Self {
            degree,
            ctrl: control_points,
        }
    }
```
```rust
    pub fn create(degree: usize, control_points: Vec<Point4D>) -> Option<Self> {
        if control_points.len().saturating_sub(1) != degree {
            return None;
        }
        Some(Self {
            degree,
            ctrl: control_points
        })
    }
```
```rust
    pub fn is_rational(&self) -> bool {
        is_rational_ctrl_array(&self.ctrl)
    }
```
```rust
    pub fn is_closed(&self, eps: f64) -> bool {
        if self.ctrl.len() < 2 {
            return false;
        }
        let p0 = self.ctrl.first().unwrap().to_point();
        let p1 = self.ctrl.last().unwrap().to_point();

        let dx = p0.x - p1.x;
        let dy = p0.y - p1.y;
        let dz = p0.z - p1.z;
        dx * dx + dy * dy + dz * dz <= eps * eps
    }
```
```rust
    pub fn point_at(&self, u: Real) -> Point3D {
        let n = self.degree;
        let mut p = Point3D::zero();
        for i in 0..=n {
            let b = on_bernstein(n, i, u);
            p.x += b * self.ctrl[i].x;
            p.y += b * self.ctrl[i].y;
            p.z += b * self.ctrl[i].z;
        }
        p
    }
```
```rust
    pub fn point_at_rational(&self, t: Real) -> Option<Point3D> {
        let c = self.evaluate_cpoint_rational(t);
        if !c.w.is_finite() || c.w.abs() < 1e-15 {
            return None; // avoid division by zero
        }
        Some(Point3D::new(c.x / c.w, c.y / c.w, c.z / c.w))
    }
```
```rust
    pub fn cpoint_at(&self, u: f64) -> Point4D {
        let n = self.degree;
        let mut c = Point4D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        for i in 0..=n {
            let b = on_bernstein(n, i, u);
            c.x += b * self.ctrl[i].x;
            c.y += b * self.ctrl[i].y;
            c.z += b * self.ctrl[i].z;
            c.w += b * self.ctrl[i].w;
        }
        c
    }
```
```rust
    pub fn evaluate_cpoint_rational(&self, t: Real) -> Point4D {
        let p: Degree = (self.ctrl.len() as i32 - 1).max(0) as u16;
        let b_vec = on_all_ber_1d(p, t);
        let rat = on_is_rat(self.ctrl.as_slice());

        if rat {
            let (mut xw, mut yw, mut zw, mut w) = (0.0, 0.0, 0.0, 0.0);
            for (i, ni) in b_vec.iter().enumerate() {
                let c = self.ctrl[i];
                xw += ni * (c.x * c.w);
                yw += ni * (c.y * c.w);
                zw += ni * (c.z * c.w);
                w += ni * c.w;
            }
            if w == 0.0 {
                w = 1.0;
            }
            Point4D {
                x: xw,
                y: yw,
                z: zw,
                w,
            }
        } else {
            let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
            for (i, ni) in b_vec.iter().enumerate() {
                let c = self.ctrl[i];
                x += ni * c.x;
                y += ni * c.y;
                z += ni * c.z;
            }
            Point4D { x, y, z, w: 1.0 }
        }
    }
```
```rust
    pub fn elevate_degree(&self, t: usize) -> BezierCurve {
        let mat = on_degree_elevation_matrix(self.degree, t);
        let mut new_ctrl = vec![Point4D::zero(); self.degree + t + 1];
        for i in 0..=self.degree + t {
            for j in 0..=self.degree {
                new_ctrl[i].x += mat[i][j] * self.ctrl[j].x;
                new_ctrl[i].y += mat[i][j] * self.ctrl[j].y;
                new_ctrl[i].z += mat[i][j] * self.ctrl[j].z;
                new_ctrl[i].w += mat[i][j] * self.ctrl[j].w;
            }
        }
        BezierCurve {
            degree: self.degree + t,
            ctrl: new_ctrl,
        }
    }
```
```rust
    pub fn reduce_degree(&mut self, target_deg: Degree) -> Vec<Point4D> {
        let p = (self.ctrl.len() - 1) as i32;
        if target_deg >= p as u16 {
            return self.ctrl.to_vec();
        }

        let q = target_deg as usize;
        let mut new_ctrl = vec![
            Point4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0
            };
            q + 1
        ];
        // Simple proportional interpolation basis (can also be done with least-squares)
        for i in 0..=q {
            let t = i as Real / q as Real;
            // Obtain the point at parameter (t) using De Casteljau’s algorithm,
            // and use it directly as a control point
            new_ctrl[i] = self.evaluate_cpoint_rational(t);
        }
        new_ctrl
    }
```
```rust
    pub fn reduce_degree_curve(&mut self, target_deg: Degree) -> Self {
        Self {
            degree: target_deg as usize,
            ctrl: self.reduce_degree(target_deg),
        }
    }
```
```rust
    pub fn re_parameterize(&self, func: &BezierFunction) -> BezierCurve {
        let n = self.degree;
        let mut result = vec![Point4D::zero(); func.degree + n + 1];

        for i in 0..=n {
            let bi = BezierFunction {
                degree: n,
                coeffs: (0..=n).map(|j| if j == i { 1.0 } else { 0.0 }).collect(),
            };
            let bi_f = bi.multiply(func); // B_i^n(f(u)) as BezierFunction
            for (j, coeff) in bi_f.coeffs.iter().enumerate() {
                result[j].x += coeff * self.ctrl[i].x;
                result[j].y += coeff * self.ctrl[i].y;
                result[j].z += coeff * self.ctrl[i].z;
                result[j].w += coeff * self.ctrl[i].w;
            }
        }
        BezierCurve {
            degree: func.degree + n,
            ctrl: result,
        }
    }
```
```rust
    pub fn dot(&self, rhs: &BezierCurve) -> Vec<f64> {
        let n = self.degree + rhs.degree;
        let mut result = vec![0.0; n + 1];
        for i in 0..=n {
            let jl = i.saturating_sub(rhs.degree);
            let jh = self.degree.min(i);
            for j in jl..=jh {
                let p = &self.ctrl[j];
                let q = &rhs.ctrl[i - j];
                result[i] += p.x * q.x + p.y * q.y + p.z * q.z;
            }
        }
        result
    }
```
```rust
    /// Cross product of two curves
    pub fn cross(&self, rhs: &BezierCurve) -> BezierCurve {
        let n = self.degree + rhs.degree;
        let mut result = vec![Point4D::zero(); n + 1];
        for i in 0..=n {
            let jl = i.saturating_sub(rhs.degree);
            let jh = self.degree.min(i);
            for j in jl..=jh {
                let p = self.ctrl[j].to_point();
                let q = rhs.ctrl[i - j].to_point();
                let v = Vector3D::cross(&Vector3D::from(p), &Vector3D::from(q));
                result[i].x += v.x;
                result[i].y += v.y;
                result[i].z += v.z;
                result[i].w = 1.0;
            }
        }
        BezierCurve {
            degree: n,
            ctrl: result,
        }
    }
```
```rust
    /// Split at u
    pub fn split(&self, u: f64) -> (BezierCurve, BezierCurve) {
        let p = self.degree;
        let mut a = self.ctrl.clone();
        let mut left = vec![Point4D::zero(); p + 1];
        let mut right = vec![Point4D::zero(); p + 1];

        left[0] = a[0];
        right[p] = a[p];
        for k in 1..=p {
            for i in 0..=(p - k) {
                a[i] = a[i].lerp(&a[i + 1], u);
            }
            left[k] = a[0];
            right[p - k] = a[p - k];
        }
        (
            BezierCurve {
                degree: p,
                ctrl: left,
            },
            BezierCurve {
                degree: p,
                ctrl: right,
            },
        )
    }
```
```rust
    /// Least-squares cubic Bezier approximation
    pub fn approx_cubic(
        ps: &Point3D,
        ts: &Vector3D,
        _p: &Point3D,
        _t: &Vector3D,
        pe: &Point3D,
        te: &Vector3D,
    ) -> BezierCurve {

        // Internal: Piegl's least-squares method. Numerical approximation omitted, structure only preserved.
        let mut ctrl = Vec::with_capacity(4);
        ctrl.push(Point4D::from_point_w(ps, 1.0));

        // Approximately compute middle control points
        let p1 = Point3D {
            x: ps.x + ts.x * 0.3,
            y: ps.y + ts.y * 0.3,
            z: ps.z + ts.z * 0.3,
        };
        let p2 = Point3D {
            x: pe.x - te.x * 0.3,
            y: pe.y - te.y * 0.3,
            z: pe.z - te.z * 0.3,
        };
        ctrl.push(Point4D::from_point_w(&p1, 1.0));
        ctrl.push(Point4D::from_point_w(&p2, 1.0));
        ctrl.push(Point4D::from_point_w(pe, 1.0));

        BezierCurve { degree: 3, ctrl }
    }
```
```rust
    pub fn to_nurbs(&self) -> NurbsCurve {
        // Bézier curve → clamped B-spline: [0..0 (p+1 times), 1..1 (p+1 times)]

        let p = self.degree;
        let mut knot = Vec::with_capacity(2 * (p + 1));
        knot.extend(std::iter::repeat(0.0).take(p + 1));
        knot.extend(std::iter::repeat(1.0).take(p + 1));

        NurbsCurve {
            dimension: 3,
            degree: p as u16,
            kv: KnotVector { knots: knot },
            ctrl: self.ctrl.clone(),
            domain: Interval { t0: 0.0, t1: 1.0 },
        }
    }
}
```
```rust
impl BezierCurve {
    /// Internal: Evaluate Bezier values from homogeneous Bezier control points in the form (x*w, y*w, z*w, w).
    /// Ignoring rationality, evaluate only as a simple polynomial Bezier.
    fn eval_h_bezier(ctrl: &[Point4D], t: Real) -> Point4D {
        let n = ctrl.len();
        debug_assert!(n >= 1);
        let p = n - 1;

        // Compute all Bernstein basis functions (degree p, indices 0..=p)
        let mut b = vec![0.0; n];
        // Simple de Casteljau-style Bernstein value computation
        // (If on_bernstein_all_clamped already exists, you can use that instead)
        b[0] = 1.0;
        let u = t;
        let v = 1.0 - u;
        for k in 1..=p {
            let mut saved = 0.0;
            for j in 0..k {
                let temp = b[j];
                b[j] = saved + v * temp;
                saved = u * temp;
            }
            b[k] = saved;
        }

        let mut r = Point4D::new(0.0, 0.0, 0.0, 0.0);
        for i in 0..n {
            let w = b[i];
            r.x += w * ctrl[i].x;
            r.y += w * ctrl[i].y;
            r.z += w * ctrl[i].z;
            r.w += w * ctrl[i].w;
        }
        r
    }
```
```rust
    /// Internal: Compute 0th, 1st, 2nd, and 3rd order values/derivatives of a homogeneous Bezier curve at once.
    /// Returns: [C(t), C'(t), C''(t), C'''(t)] in R^4 (Xw, Yw, Zw, W)
    fn eval_h_ders(&self, t: Real, d_max: usize) -> Option<[Point4D; 4]> {
        let n = self.ctrl.len();
        if n == 0 {
            return None;
        }
        let p = n - 1;
        if p == 0 {
            // 상수 곡선: 도함수는 0
            let c = {
                let c0 = self.ctrl[0];
                let w = if c0.w.is_finite() && c0.w != 0.0 {
                    c0.w
                } else {
                    1.0
                };
                Point4D::new(c0.x * w, c0.y * w, c0.z * w, w)
            };
            return Some([
                c,
                Point4D::new(0.0, 0.0, 0.0, 0.0),
                Point4D::new(0.0, 0.0, 0.0, 0.0),
                Point4D::new(0.0, 0.0, 0.0, 0.0),
            ]);
        }

        let d_max = d_max.min(3);

        // 1) Convert to homogeneous control points in the form (x*w, y*w, z*w, w)
        let mut homo = Vec::with_capacity(n);
        for c in &self.ctrl {
            let w = if c.w.is_finite() && c.w != 0.0 {
                c.w
            } else {
                1.0
            };
            homo.push(Point4D::new(c.x * w, c.y * w, c.z * w, w));
        }

        // Initialize result array
        let mut out = [
            Point4D::new(0.0, 0.0, 0.0, 0.0),
            Point4D::new(0.0, 0.0, 0.0, 0.0),
            Point4D::new(0.0, 0.0, 0.0, 0.0),
            Point4D::new(0.0, 0.0, 0.0, 0.0),
        ];

        // 0th order: original Bezier
        out[0] = Self::eval_h_bezier(&homo, t);

        if d_max >= 1 {
            // First-order control points: Q_i = p * (P_{i+1} - P_i), i = 0..p-1
            let mut d1 = Vec::with_capacity(p);
            let s1 = p as Real;
            for i in 0..p {
                let a = &homo[i];
                let b = &homo[i + 1];
                d1.push(Point4D::new(
                    s1 * (b.x - a.x),
                    s1 * (b.y - a.y),
                    s1 * (b.z - a.z),
                    s1 * (b.w - a.w),
                ));
            }
            out[1] = Self::eval_h_bezier(&d1, t);

            if d_max >= 2 && p >= 2 {
                // Second-order control points: R_i = (p-1) * (Q_{i+1} - Q_i), i = 0..p-2
                let mut d2 = Vec::with_capacity(p - 1);
                let s2 = (p - 1) as Real;
                for i in 0..(p - 1) {
                    let a = &d1[i];
                    let b = &d1[i + 1];
                    d2.push(Point4D::new(
                        s2 * (b.x - a.x),
                        s2 * (b.y - a.y),
                        s2 * (b.z - a.z),
                        s2 * (b.w - a.w),
                    ));
                }
                out[2] = Self::eval_h_bezier(&d2, t);

                if d_max >= 3 && p >= 3 {
                    // Third-order control points: S_i = (p-2) * (R_{i+1} - R_i), i = 0..p-3
                    let mut d3 = Vec::with_capacity(p - 2);
                    let s3 = (p - 2) as Real;
                    for i in 0..(p - 2) {
                        let a = &d2[i];
                        let b = &d2[i + 1];
                        d3.push(Point4D::new(
                            s3 * (b.x - a.x),
                            s3 * (b.y - a.y),
                            s3 * (b.z - a.z),
                            s3 * (b.w - a.w),
                        ));
                    }
                    out[3] = Self::eval_h_bezier(&d3, t);
                }
            }
        }
        Some(out)
    }
```
```rust
    pub fn eval_point_and_ders(
        &self,
        t: Real,
        ders_order: usize,
    ) -> Option<(Point3D, Vec<Vector3D>)> {
        let d = ders_order.min(3);
        let hd = self.eval_h_ders(t, d)?;
        let c0 = hd[0];
        if !c0.w.is_finite() || c0.w.abs() < 1e-15 {
            return None;
        }

        let w0 = c0.w;
        let p = Point3D::new(c0.x / w0, c0.y / w0, c0.z / w0);

        let mut ders = Vec::new();
        if d >= 1 {
            let c1 = hd[1];
            let w1 = c1.w;
            // P' = (X1*W0 - X0*W1) / W0^2
            let v0 = Vector3D::new(c0.x, c0.y, c0.z);
            let v1 = Vector3D::new(c1.x, c1.y, c1.z);
            let num = v1 * w0 - v0 * w1;
            let denom = w0 * w0;
            ders.push(num / denom);
        }
        if d >= 2 {
            let c1 = hd[1];
            let c2 = hd[2];
            let w1 = c1.w;
            let w2 = c2.w;
            let v0 = Vector3D::new(c0.x, c0.y, c0.z);
            let v1 = Vector3D::new(c1.x, c1.y, c1.z);
            let v2 = Vector3D::new(c2.x, c2.y, c2.z);

            // P'' = (W0^2*X2 - 2 W0 W1 X1 - X0 (W0 W2 - 2 W1^2)) / W0^3
            let term1 = v2 * (w0 * w0);
            let term2 = v1 * (2.0 * w0 * w1);
            let term3 = v0 * (w0 * w2 - 2.0 * w1 * w1);
            let num = term1 - term2 - term3;
            let denom = w0 * w0 * w0;
            ders.push(num / denom);
        }
        if d >= 3 {
            let c1 = hd[1];
            let c2 = hd[2];
            let c3 = hd[3];
            let w1 = c1.w;
            let w2 = c2.w;
            let w3 = c3.w;

            let v0 = Vector3D::new(c0.x, c0.y, c0.z);
            let v1 = Vector3D::new(c1.x, c1.y, c1.z);
            let v2 = Vector3D::new(c2.x, c2.y, c2.z);
            let v3 = Vector3D::new(c3.x, c3.y, c3.z);

            // P''' = (W0^3 X3 -3 W0^2 W1 X2 -3 W0 X1 (W0 W2 - 2 W1^2)
            //         - X0 (W0^2 W3 - 6 W0 W1 W2 + 6 W1^3)) / W0^4
            let term1 = v3 * (w0 * w0 * w0);
            let term2 = v2 * (3.0 * w0 * w0 * w1);
            let term3 = v1 * (3.0 * w0 * (w0 * w2 - 2.0 * w1 * w1));
            let term4 = v0 * (w0 * w0 * w3 - 6.0 * w0 * w1 * w2 + 6.0 * w1 * w1 * w1);
            let num = term1 - term2 - term3 - term4;
            let denom = w0 * w0 * w0 * w0;
            ders.push(num / denom);
        }

        Some((p, ders))
    }
```
```rust
    /// Unit tangent vector
    pub fn tangent(&self, t: Real) -> Option<Vector3D> {
        let (_, ders) = self.eval_point_and_ders(t, 1)?;
        let d1 = ders[0];
        let len = d1.length();
        if len <= 1e-15 {
            return None;
        }
        Some(d1 / len)
    }
```
```rust
    /// Curvature κ(t) = ||C'(t) × C''(t)|| / ||C'(t)||^3
    pub fn curvature(&self, t: Real) -> Option<Real> {
        let (_, ders) = self.eval_point_and_ders(t, 2)?;
        let d1 = ders[0];
        let d2 = ders[1];
        let v = d1.cross(&d2);
        let num = v.length();
        let denom = d1.length();
        if denom <= 1e-15 {
            return None;
        }
        let denom3 = denom * denom * denom;
        if denom3 <= 1e-30 {
            return None;
        }
        Some(num / denom3)
    }
```
```rust
    /// Torsion τ(t) = det(C', C'', C''') / ||C' × C''||^2
    /// det(C',C'',C''') = (C' × C'') · C'''
    pub fn torsion(&self, t: Real) -> Option<Real> {
        let (_, ders) = self.eval_point_and_ders(t, 3)?;
        let d1 = ders[0];
        let d2 = ders[1];
        let d3 = ders[2];

        let cross12 = d1.cross(&d2);
        let denom = cross12.length_squared();
        if denom <= 1e-24 {
            // In cases that are nearly planar or have very little curvature
            return None;
        }
        let num = cross12.dot(&d3);
        Some(num / denom)
    }
}
```
```rust

#[derive(Debug, Clone)]
pub struct BezierFunction {
    pub degree: usize,
    pub coeffs: Vec<f64>, // control values (function values)
}
```
```rust
impl BezierFunction {
    pub fn evaluate(&self, u: f64) -> f64 {
        let n = self.degree;
        let mut val = 0.0;
        for i in 0..=n {
            val += self.coeffs[i] * on_bernstein(n, i, u);
        }
        val
    }
```
```rust
    pub fn multiply(&self, rhs: &Self) -> BezierFunction {
        let p = self.degree;
        let q = rhs.degree;
        let n = p + q;
        let mut fg = vec![0.0; n + 1];
        for i in 0..=n {
            let jl = i.saturating_sub(q);
            let jh = p.min(i);
            for j in jl..=jh {
                let coef = on_product_matrix(p, q, i, j);
                fg[i] += coef * self.coeffs[j] * rhs.coeffs[i - j];
            }
        }
        BezierFunction {
            degree: n,
            coeffs: fg,
        }
    }
```
```rust
    pub fn elevate(&self, t: usize) -> BezierFunction {
        let p = self.degree;
        let q = p + t;
        let mut out = vec![0.0; q + 1];
        for i in 0..=q {
            let inv = 1.0 / on_binomial_usize(q, i) as f64;
            let k_low = if i > t { i - t } else { 0 };
            let k_high = p.min(i);
            for j in k_low..=k_high {
                out[i] += inv
                    * on_binomial_usize(p, j) as f64
                    * on_binomial_usize(t, i - j) as f64
                    * self.coeffs[j];
            }
        }
        BezierFunction {
            degree: q,
            coeffs: out,
        }
    }
}
```
```rust
#[derive(Debug, Clone)]
pub struct BezierArc {
    pub ctrl: Vec<Point4D>,
    pub degree: usize,
}
```
```rust
impl BezierArc {
    /// Compute middle weight approximating circular arc
    pub fn approx_weight_circle(p0: &Point3D, p1: &Point3D, p2: &Point3D) -> f64 {
        let d = p0.distance(p2) * 0.5;
        let fl = p0.distance(p1);
        let fr = p2.distance(p1);
        let sl = d / (d + fl);
        let sr = d / (d + fr);
        let s = 0.5 * (sl + sr);
        s / (1.0 - s)
    }
```
```rust
    /// Create circular arc given endpoints and tangents
    pub fn from_end_tangents(
        p1: Point3D,
        _t1: Vector3D,
        p2: Point3D,
        _t2: Vector3D,
    ) -> Option<BezierArc> {
        let mut ctrl: Vec<Point4D> = Vec::new();

        // Default weight (90° segment)
        let cw = 0.5 * f64::sqrt(2.0);

        // Simple approximation: one-segment circular arc
        ctrl.push(Point4D::from_point_w(&p1, 1.0));
        let mid = (p1 + p2) * 0.5;
        ctrl.push(Point4D::from_point_w(&mid, cw));
        ctrl.push(Point4D::from_point_w(&p2, 1.0));

        Some(BezierArc { ctrl, degree: 2 })
    }
```
```rust
    /// Create conic given point-on-arc
    pub fn from_point_on_arc(
        p0: Point3D,
        _t0: Vector3D,
        p2: Point3D,
        _t2: Vector3D,
        p: Point3D,
    ) -> Option<(Point3D, f64)> {
        // Take the intersection point as P1 and compute the weight (simple approximation)
        let chord = p2 - p0;
        let mid = (p0 + p2) * 0.5;
        let v = (p - mid).magnitude();
        let d = chord.magnitude() * 0.5;
        let w = v / d;
        Some((mid, w))
    }
}
```
```rust
impl BezierCurve {
    pub fn ev_point(&self, t: Real) -> Option<Point3D> {
        self.eval_point_and_ders(t, 0).map(|(p, _)| p)
    }
```
```rust
    pub fn ev1_der(&self, t: Real) -> Option<(Point3D, Vector3D)> {
        let (p, ders) = self.eval_point_and_ders(t, 1)?;
        let d1 = ders[0];
        Some((p, d1))
    }
```
```rust
    pub fn ev2_der(&self, t: Real) -> Option<(Point3D, Vector3D, Vector3D)> {
        let (p, ders) = self.eval_point_and_ders(t, 2)?;
        let d1 = ders[0];
        let d2 = ders[1];
        Some((p, d1, d2))
    }
```
```rust
    pub fn ev3_der(&self, t: Real) -> Option<(Point3D, Vector3D, Vector3D, Vector3D)> {
        let (p, ders) = self.eval_point_and_ders(t, 3)?;
        let d1 = ders[0];
        let d2 = ders[1];
        let d3 = ders[2];
        Some((p, d1, d2, d3))
    }
```
```rust
    pub fn ev_tangent(&self, t: Real) -> Option<(Point3D, Vector3D)> {
        let (p, ders) = self.eval_point_and_ders(t, 1)?;
        let d1 = ders[0];
        let len = d1.length();
        if len <= 1e-15 {
            return None;
        }
        Some((p, d1 / len))
    }
```
```rust
    pub fn ev_curvature(&self, t: Real) -> Option<(Point3D, Real)> {
        let (p, _) = self.eval_point_and_ders(t, 0)?;
        let k = self.curvature(t)?;
        Some((p, k))
    }
```
```rust
    pub fn ev_torsion(&self, t: Real) -> Option<(Point3D, Real)> {
        let (p, _) = self.eval_point_and_ders(t, 0)?;
        let tau = self.torsion(t)?;
        Some((p, tau))
    }
}
```
```rust
impl BezierCurve {
    /// Returns the d-th derivative (1–3) at t.
    /// Derivative in R^3 (non-homogeneous).
    pub fn derivative_at(&self, t: Real, order: usize) -> Option<Vector3D> {
        if order == 0 {
            // 0th order = just a point. If needed, convert to Point3D here or move to a separate function.
            let (p, _) = self.eval_point_and_ders(t, 0)?;
            return Some(Vector3D::new(p.x, p.y, p.z));
        }

        let max_order = order.min(3);
        let (_, ders) = self.eval_point_and_ders(t, max_order)?;

        // ders[0] = first derivative, ders[1] = second derivative, ders[2] = third derivative (if available)
        let idx = max_order.saturating_sub(1);
        ders.get(idx).cloned()
    }
```
```rust
    pub fn evaluate_derives(&self, t: Real) -> Option<Point3D> {
        let (p, _) = self.eval_point_and_ders(t, 0)?;
        Some(p)
    }
```
```rust
    pub fn tangent_at(&self, t: Real) -> Option<Vector3D> {
        self.tangent(t)
    }
```
```rust
    pub fn curvature_at(&self, t: Real) -> Option<Real> {
        self.curvature(t)
    }
```
```rust
    pub fn torsion_at(&self, t: Real) -> Option<Real> {
        self.torsion(t)
    }
```
```rust
    pub fn bounding_box(&self, samples: usize) -> Option<(Point3D, Point3D)> {
        if self.ctrl.is_empty() || samples < 2 {
            return None;
        }

        let mut min = Point3D::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3D::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        let n = samples - 1;
        for i in 0..=n {
            let t = (i as Real) / (n as Real);
            let p = self.point_at(t);
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);

            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }
        Some((min, max))
    }
```
```rust
    pub fn reverse(&mut self) {
        self.ctrl.reverse();
        // Degree is the same, only the control point order is reversed
    }
```
```rust
    pub fn translate(&mut self, dx: Real, dy: Real, dz: Real) {
        for c in &mut self.ctrl {
            c.x += dx * c.w;
            c.y += dy * c.w;
            c.z += dz * c.w;
        }
    }
```
```rust
    pub fn scale(&mut self, sx: Real, sy: Real, sz: Real) {
        for c in &mut self.ctrl {
            c.x *= sx;
            c.y *= sy;
            c.z *= sz;
        }
    }
```
```rust
    /// You can also use this to scale control points based on a line.
    pub fn transform_linear<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Point4D),
    {
        for c in &mut self.ctrl {
            f(c);
        }
    }
}
```
```rust
impl BezierCurve {
    /// Number of control points
    pub fn cv_count(&self) -> usize {
        self.ctrl.len()
    }

    /// i번째 weight (C++ Weight(i))
    pub fn weight(&self, i: usize) -> Option<Real> {
        self.ctrl.get(i).map(|cp| cp.w)
    }
```
```rust
    /// Set the i-th weight
    /// Keeps the coordinates unchanged and modifies only w.
    pub fn set_weight(&mut self, i: usize, w: Real) -> bool {
        if let Some(cp) = self.ctrl.get_mut(i) {
            cp.w = w;
            true
        } else {
            false
        }
    }
```
```rust
    /// Set the i-th control vertex (coordinates + weight together)
    pub fn set_cv(&mut self, i: usize, p: Point3D, w: Real) -> bool {
        if let Some(cp) = self.ctrl.get_mut(i) {
            cp.x = p.x * w;
            cp.y = p.y * w;
            cp.z = p.z * w;
            cp.w = w;
            true
        } else {
            false
        }
    }
```
```rust
    /// Full de-rationalization: set all control point weights to w = 1 and divide xyz accordingly.
    pub fn make_non_rational(&mut self) {
        for cp in &mut self.ctrl {
            if cp.w != 0.0 && cp.w.is_finite() && (cp.w - 1.0).abs() > 1e-15 {
                let w = cp.w;
                cp.x /= w;
                cp.y /= w;
                cp.z /= w;
                cp.w = 1.0;
            }
        }
    }
```
```rust
    /// Rationalize with the given weight array (use only if necessary).
    /// weights.len() must be equal to cv_count().
    pub fn make_rational_with_weights(&mut self, weights: &[Real]) -> bool {
        if weights.len() != self.ctrl.len() {
            return false;
        }
        for (cp, &w) in self.ctrl.iter_mut().zip(weights.iter()) {
            if !w.is_finite() || w == 0.0 {
                return false;
            }
            // Assume the current control point is stored as "xyz * previous w".
            // If non-rational, treat w_old = 1 and adjust accordingly.
            let px = cp.x / cp.w.max(1.0);
            let py = cp.y / cp.w.max(1.0);
            let pz = cp.z / cp.w.max(1.0);

            cp.x = px * w;
            cp.y = py * w;
            cp.z = pz * w;
            cp.w = w;
        }
        true
    }
```
```rust
    /// Control polygon length
    /// Compute the sum of distances after converting to Euclidean coordinates.
    pub fn control_polygon_length(&self) -> Real {
        if self.ctrl.len() < 2 {
            return 0.0;
        }
        let mut len = 0.0;
        for k in 0..(self.ctrl.len() - 1) {
            let a = &self.ctrl[k];
            let b = &self.ctrl[k + 1];

            let wa = if a.w != 0.0 { a.w } else { 1.0 };
            let wb = if b.w != 0.0 { b.w } else { 1.0 };

            let pa = Point3D::new(a.x / wa, a.y / wa, a.z / wa);
            let pb = Point3D::new(b.x / wb, b.y / wb, b.z / wb);
            len += (pb - pa).length();
        }
        len
    }
```
```rust
    /// Trim the curve to the interval [t0, t1] and return a new BezierCurve (C++ Trim).
    ///
    /// The parameter domain of the returned curve is remapped to [0, 1].
    pub fn trim(&self, mut t0: Real, mut t1: Real) -> Option<BezierCurve> {
        // Simple normalization and clamping
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t1 <= 0.0 || t0 >= 1.0 {
            return None; // Completely outside
        }
        t0 = t0.clamp(0.0, 1.0);
        t1 = t1.clamp(0.0, 1.0);
        if (t1 - t0).abs() < 1e-15 {
            return None;
        }

        // 1) Trim to [0, t1]
        let (c0, _) = self.split(t1);

        if t0 <= 0.0 {
            // Need the entire interval [0, t1]
            return Some(c0);
        }

        // 2) Extract only the [t0, t1] segment:
        //    The parameter of c0 is still [0, 1],
        //    and within it, [t0/t1, 1] corresponds to the original [t0, t1].
        let local = t0 / t1;
        let (_, c_trim) = c0.split(local);
        Some(c_trim)
    }
}
```
```rust
impl BezierCurve {
    /// Least-squares degree reduction to target_deg
    pub fn reduce_degree_ls(&self, target_deg: usize, samples: usize) -> Option<BezierCurve> {
        let p = self.degree;
        if target_deg >= p {
            return Some(self.clone());
        }

        // 1. 샘플링
        let mut ts = Vec::with_capacity(samples);
        for k in 0..samples {
            ts.push(k as f64 / (samples as f64 - 1.0));
        }

        // 2. 행렬 A (samples x (target_deg+1))
        let mut a = DMatrix::zeros(samples, target_deg + 1);
        for (row, &t) in ts.iter().enumerate() {
            for j in 0..=target_deg {
                a[(row, j)] = on_bernstein(target_deg, j, t);
            }
        }

        // 3. 벡터 y (samples x 3) : 원래 곡선 점
        let mut yx = DVector::zeros(samples);
        let mut yy = DVector::zeros(samples);
        let mut yz = DVector::zeros(samples);
        for (row, &t) in ts.iter().enumerate() {
            let pnt = self.point_at_rational(t).unwrap_or(Point3D::zero());
            yx[row] = pnt.x;
            yy[row] = pnt.y;
            yz[row] = pnt.z;
        }

        // 4. 정규방정식 풀기
        let ata = &a.transpose() * &a;
        let atx = &a.transpose() * yx;
        let aty = &a.transpose() * yy;
        let atz = &a.transpose() * yz;

        let qx = ata.clone().lu().solve(&atx)?;
        let qy = ata.clone().lu().solve(&aty)?;
        let qz = ata.clone().lu().solve(&atz)?;

        // 5. 제어점 구성
        let mut new_ctrl = Vec::new();
        for i in 0..=target_deg {
            new_ctrl.push(Point4D::new(qx[i], qy[i], qz[i], 1.0));
        }

        Some(BezierCurve {
            degree: target_deg,
            ctrl: new_ctrl,
        })
    }
}
```
```rust
pub struct CubicBezier {
    pub(crate) b0: Point3D,
    pub(crate) b1: Point3D,
    pub(crate) b2: Point3D,
    pub(crate) b3: Point3D,
}
```
```rust
impl CubicBezier {
    pub fn from_hermite(p0: Point3D, p1: Point3D, t0: Vector3D, t1: Vector3D, h: f64) -> Self {
        let b0 = p0;
        let b3 = p1;
        let b1 = p0 + (t0.to_point() * (h / 3.0));
        let b2 = p1 - (t1.to_point() * (h / 3.0));
        CubicBezier { b0, b1, b2, b3 }
    }
    pub fn cubic_bezier_point(&self, t: f64) -> Point3D {
        let t1 = 1.0 - t;
        let term1 = self.b0 * (t1 * t1 * t1); // (1-t)^3 * P0
        let term2 = self.b1 * (3.0 * t1 * t1 * t); // 3*(1-t)^2*t * P1
        let term3 = self.b2 * (3.0 * t1 * t * t); // 3*(1-t)*t^2 * P2
        let term4 = self.b3 * (t * t * t); // t^3 * P3
        term1 + term2 + term3 + term4
    }
}
```
```rust
pub fn conic_arc_to_bezier(
    p0: Point3D,
    t0: Vector3D,
    p2: Point3D,
    t2: Vector3D,
    p: Point3D,
) -> BezierCurve {
    let out = on_make_bezier_conic_arc(p0, t0, p2, t2, p).expect("should build");

    let pts = vec![
        p0.to_homogeneous(),
        Point4D::from_point_w(&out.0, out.1),
        p2.to_homogeneous(),
    ];

    BezierCurve::new(pts)
}
```
```rust
pub fn on_bezier_arc_len(ctrl: &[Point4D], tol: f64) -> f64 {
    fn rec(ctrl: &[Point3D], tol2: f64) -> f64 {
        let n = ctrl.len() - 1;
        let mut chord = 0.0;
        let poly;
        for i in 0..n {
            let a = &ctrl[i];
            let b = &ctrl[i + 1];
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let dz = b.z - a.z;
            chord += (dx * dx + dy * dy + dz * dz).sqrt();
        }
        poly = ((ctrl[0].x - ctrl[n].x).powi(2)
            + (ctrl[0].y - ctrl[n].y).powi(2)
            + (ctrl[0].z - ctrl[n].z).powi(2))
            .sqrt();

        if chord - poly < tol2 {
            return chord;
        }

        // subdivide
        let mid = ctrl.len() / 2;
        let left = &ctrl[..=mid];
        let right = &ctrl[mid..];
        rec(left, tol2) + rec(right, tol2)
    }

    let pts: Vec<Point3D> = ctrl.iter().map(|c| c.to_point()).collect();
    0.5 * rec(&pts, 2.0 * tol)
}
```
```rust
pub fn on_re_param_inverse_matrix(p: usize, a: f64, b: f64, ap: f64, bp: f64) -> Vec<Vec<f64>> {
    // 1. Affine inverse transformation coefficients
    let (alpha, beta) = on_re_param_affine(ap, bp, a, b); // 역방향

    // 2. Construct the R⁻¹ matrix: expansion of (αt + β)^i
    let mut r_inv = vec![vec![0.0; p + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=i {
            let comb = on_binomial_usize(i, j) as f64;
            r_inv[i][j] = comb * beta.powi((i - j) as i32) * alpha.powi(j as i32);
        }
    }

    // 3. T⁻¹ = power_to_bezier_matrix(p)
    let t_inv = on_power_to_bezier_vec(p);

    // 4. P⁻¹ = bezier_to_power_matrix(p)
    let p_inv = on_bezier_to_power_matrix(p);

    // 5. Matrix multiplication with nalgebra: T⁻¹ · R⁻¹ · P⁻¹
    let r_na = DMatrix::from_row_slice(p + 1, p + 1, &r_inv.concat());
    let t_na = DMatrix::from_row_slice(p + 1, p + 1, &t_inv.concat());
    let p_na = DMatrix::from_row_slice(p + 1, p + 1, &p_inv.concat());

    let m_inv = t_na * r_na * p_na;

    // 6. DMatrix → Vec<Vec<f64>>
    let mut result = vec![vec![0.0; p + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=p {
            result[i][j] = m_inv[(i, j)];
        }
    }
    result
}
```
```rust
/// Coefficients of linear reparameterization u = α u' + β
/// Mapping from original interval [a, b] → new interval [ap, bp]:
///   u = α * u' + β,   α = (b - a) / (bp - ap),   β = (bp*a - ap*b) / (bp - ap)
#[inline]
pub fn on_re_param_affine(a: Real, b: Real, ap: Real, bp: Real) -> (Real, Real) {
    let denom = bp - ap;
    let alpha = (b - a) / denom;
    let beta = (bp * a - ap * b) / denom;
    (alpha, beta)
}
```
```rust
/// Reparameterization matrix M (Bezier coefficient transformation: c' = M · c)
/// Note: Standard construction by expanding Bezier(n) into monomials, applying α, β,
///       and then projecting back to Bezier form.
pub fn on_re_param_matrix(p: usize, a: Real, b: Real, ap: Real, bp: Real) -> Vec<Vec<Real>> {
    let (alpha, beta) = on_re_param_affine(a, b, ap, bp);

    // Step 1: Construct the R matrix — expansion of (αu' + β)^i
    let mut r = vec![vec![0.0; p + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=i {
            let comb = on_binomial_usize(i, j) as f64;
            r[i][j] = comb * beta.powi((i - j) as i32) * alpha.powi(j as i32);
        }
    }

    // Step 2: Bezier → Power basis transformation matrix T
    let t = on_bezier_to_power_matrix(p);

    // Step 3: Power → Bezier basis transformation matrix P
    let p_mat = on_power_to_bezier_vec(p);

    // Step 4: Final reparameterization matrix M = P · R · T
    let rt = Matrix::mul(&r, &t);
    let m = Matrix::mul(&p_mat, &rt);
    m
}
```
```rust
pub fn on_compute_bezier_degree3chord_parameters(q: &[Point3D; 4]) -> [Real; 4]
{
    let mut u = [0.0; 4];
    u[0] = 0.0;
    u[3] = 1.0;

    let d1 = q[0].distance(&q[1]);
    let d2 = q[1].distance(&q[2]);
    let d3 = q[2].distance(&q[3]);
    let sum = d1 + d2 + d3;

    if !(sum > ON_TOL12) || !(d1 / sum >= 0.0) || !(d2 / sum >= 0.0)
    {
        u[1] = 1.0 / 3.0;
        u[2] = 2.0 / 3.0;
    }
    else {
        u[1] = u[0] + d1 / sum;
        u[2] = u[1] + d2 / sum;
    }
    u
}
```
```rust
pub fn on_make_cubic_interpolant_bezier(q: &[Point3D; 4]) -> Option<BezierCurve> {
    on_fit_cubic_bezier_through4points(q).map(|pts| {
        let pts4d: Vec<_> = pts.iter().map(|pt| pt.to_homogeneous()).collect();
        BezierCurve::new(pts4d)
    })
}
```


