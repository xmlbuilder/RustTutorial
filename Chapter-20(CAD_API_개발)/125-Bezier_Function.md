## 📘 Bezier Function 문서 (정리본)
- Bezier Function: 정의, 수식, 의미, 활용

## 1. 개요
- Bezier Function은 Bezier Curve의 스칼라 버전이다.
- Bezier Curve: 점(Point4D)을 반환
- Bezier Function: 실수값 f(t)를 반환
- 즉,
```math
C(t)=\sum P_iB_{i,p}(t)
```
- 처럼 점을 만드는 대신,
```math
f(t)=\sum f_iB_{i,p}(t)
```
- 처럼 스칼라 값을 만드는 Bezier 형태의 함수다.
- Bezier Function은 CAD/NURBS 엔진에서 다음과 같은 곳에 사용된다:
  - 곡선/곡면의 reparameterization
  - rational curve의 분모 W(t) 표현
  - 곡선 × 함수의 product curve 생성
  - weight function, blending function, trimming function 등

## 2. 정의
- Bezier Function of degree p:
```math
f(t)=\sum _{i=0}^pf_iB_{i,p}(t)
```
- 여기서:
  - $f_i$ : Bezier control values (스칼라)
  - $B_{i,p}(t)$ : Bernstein basis function
```math
B_{i,p}(t)={p \choose i}t^i(1-t)^{p-i}
```
- Bezier Curve와 완전히 동일한 구조지만
- **control point 대신 control value(스칼라)** 를 사용한다.

## 3. 성질
- ✔ 1) Convex combination
```math
f(t)=\sum f_iB_{i,p}(t)
```
- Bernstein basis는 항상:
  - $B_{i,p}(t)\geq 0$
  - $\sum B_{i,p}(t)=1$
- 따라서:
```math
\min (f_i)\leq f(t)\leq \max (f_i)
```
- 즉, Bezier Function은 control value의 convex hull 안에 있다.

- ✔ 2) End-point interpolation
```math
f(0)=f_0,\quad f(1)=f_p
```
- Bezier Curve와 동일.

- ✔ 3) 미분
```math
f'(t)=p\sum _{i=0}^{p-1}(f_{i+1}-f_i)B_{i,p-1}(t)
```
즉, **차분(difference)** 이 derivative control value가 된다.

## 4. Power Basis와의 관계
- Bezier Function은 power basis로 변환 가능:
```math
f(t)=a_0+a_1t+a_2t^2+\dots +a_pt^p
```
- 변환은 다음과 같은 매트릭스를 통해 이루어진다:
```math
a_k=\sum _{i=0}^pf_i\cdot M_{k,i}
```
- 엔진에는 이미 다음 함수들이 존재한다:
  - on_power_to_bezier_deg2
  - on_power_to_bezier_deg3
  - on_power_to_bezier_deg4
- 즉, power → Bezier 변환은 이미 구현되어 있고 Bezier → power 변환도 쉽게 구현 가능하다.

## 5. Product with Bezier Curve
- Bezier Function f(t) 와 Bezier Curve C(t) 의 곱:
```math
Q(t)=f(t)C(t)
```
- f: degree p
- C: degree q
- Q: degree p+q
- 수식:
```math
Q(t)=\sum _{i=0}^pf_iB_{i,p}(t)\cdot \sum _{j=0}^qP_jB_{j,q}(t)
```
- Bernstein basis의 곱은 다시 Bernstein basis로 표현 가능:
```math
B_{i,p}(t)B_{j,q}(t)=\frac{{p \choose i}{q \choose j}}{{p+q \choose i+j}}B_{i+j,p+q}(t)
```
- 따라서 control point는:
```math
Q_k=\sum _{i+j=k}\frac{{p \choose i}{q \choose j}}{{p+q \choose k}}f_iP_j
```
- 엔진에서는 이 공식을 그대로 구현한 것:
- on_product_matrix(p, q, i, j)
- multiply_bezier_function()

## 6. Rational Function과의 관계
- Bezier Function은 rational curve의 분모 W(t)를 표현하는 데 사용된다:
```math
C(t)=\frac{\sum P_iw_iB_{i,p}(t)}{\sum w_iB_{i,p}(t)}
```
- 여기서:
```math
W(t)=\sum w_iB_{i,p}(t)
```
- 즉, W(t)는 Bezier Function이다.
- 따라서 rational curve의 모든 미분, 곡률, 비틀림 계산에서 Bezier Function이 핵심 역할을 한다.

## 7. 활용 요약
| 활용 분야                   | 설명                                                         |
|-----------------------------|--------------------------------------------------------------|
| Rational Curve 분모 W(t)    | NURBS 곡선의 분모 W(t)를 Bezier Function으로 표현            |
| Reparameterization          | 곡선/곡면의 파라미터를 f(u)로 재매핑할 때 사용               |
| Product Curve               | f(t) * C(t) 형태의 곡선 곱셈에서 스칼라 함수 역할            |
| Weight Blending             | 가중치(weight) 보간, blending function으로 활용              |
| Surface Trimming            | 트리밍 곡선의 스칼라 필드 표현(inside/outside test)          |
| Approximation / Fitting     | 스칼라 필드 근사, 곡선 fitting 시 보조 함수로 사용           |
| Constraint Functions        | 곡선/곡면 제약 조건을 스칼라 함수로 표현                     |


- Bezier Function은 CAD/NURBS 엔진에서 스칼라 기반의 모든 Bezier 연산의 기본 단위다.

## 8. Rust 구조와의 매핑
- BezierFunction은:
```rust
pub struct BezierFunction {
    pub degree: usize,
    pub coeffs: Vec<f64>,
}
```
  - coeffs[i] = fᵢ
  - degree = p
  - evaluate() = Σ fᵢ Bᵢ,ₚ(t)
  - multiply() = Bezier convolution
  - elevate() = degree elevation
  - to_power_basis() = power basis 변환
  - from_power_basis() = power → Bezier 변환
- BezierCurve와 동일한 구조를 갖고 있어서 모든 연산이 자연스럽게 확장된다.

## 🔥 결론
- Bezier Function은:
  - Bezier Curve의 스칼라 버전
  - Bernstein basis로 표현되는 스칼라 함수
  - rational curve의 분모 W(t)
  - reparameterization, product, blending 등에서 핵심 역할
  - Bezier Curve와 동일한 성질(Convex, End-point, Derivative)
 
---
## 소스
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
```
```rust
    /// Convert Bernstein/Bezier coefficients b[i] (degree p) -> Power coefficients a[k]
    /// so that P(t) = sum_{k=0..p} a[k] t^k.
    pub fn to_power_basis(&self) -> Vec<f64> {
        let p = self.degree;
        assert_eq!(self.coeffs.len(), p + 1, "BezierFunction invariant violated");

        let b = &self.coeffs;
        let mut a = vec![0.0f64; p + 1];

        // a_k = Σ_{i=0..k} b_i * C(p,i)*C(p-i,k-i)*(-1)^(k-i)
        for k in 0..=p {
            let mut sum = 0.0;
            for i in 0..=k {
                let sign = if ((k - i) & 1) == 0 { 1.0 } else { -1.0 };
                let c = on_binomial_real(p, i) * on_binomial_real(p - i, k - i) * sign;
                sum += b[i] * c;
            }
            a[k] = sum;
        }
        a
    }
```
```rust
    /// Construct Bernstein/Bezier coefficients from power coefficients a[k]
    /// given P(t)=Σ a[k] t^k, returns b[i] such that P(t)=Σ b[i] B_i^p(t).
    pub fn from_power_basis(power: &[f64]) -> Self {
        let p = power.len().saturating_sub(1);
        assert!(!power.is_empty(), "power basis coeffs must be non-empty");

        // b_i = Σ_{k=0..i} a_k * C(i,k)/C(p,k)
        let mut b = vec![0.0f64; p + 1];
        for i in 0..=p {
            let mut sum = 0.0;
            for k in 0..=i {
                let denom = on_binomial_real(p, k);
                // denom=0 only if k>p (불가능) 이라서 안전
                sum += power[k] * on_binomial_real(i, k) / denom;
            }
            b[i] = sum;
        }

        BezierFunction { degree: p, coeffs: b }
    }
}
```
```rust
impl BezierCurve {
    /// B_cfncu4와 동일한 형태:
    /// out[i] (i=s..=e)만 채운다. 나머지는 건드리지 않는다.
    pub fn multiply_bezier_function_range_inplace(
        &self,
        f: &BezierFunction,
        s: usize,
        e: usize,
        out: &mut [Point4D], // 길이 >= (p+q+1)
    ) -> Result<(), NurbsError> {
        let p = f.degree;
        let q = self.degree;
        let n = p + q;

        if self.ctrl.len() != q + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!("BezierCurve ctrl len must be degree+1 (len={}, q={})",
                  self.ctrl.len(), q),
            });
        }
        if f.coeffs.len() != p + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!("BezierFunction coeffs len must be degree+1 (len={}, p={})",
                  f.coeffs.len(), p),
            });
        }
        if out.len() < n + 1 {
            return Err(NurbsError::InvalidArgument {
                msg: format!("out length too small: out.len()={} need {}", out.len(), n + 1),
            });
        }
        if s > e || e > n {
            return Err(NurbsError::InvalidArgument {
                msg: format!("range [s,e]=[{},{}] must satisfy 0<=s<=e<=p+q={}", s, e, n),
            });
        }

        for i in s..=e {
            let jl = i.saturating_sub(q);
            let jh = (p).min(i);

            let mut qi = Point4D::zero();

            for j in jl..=jh {
                // coef = A[i][j] * f[j]
                let coef = on_product_matrix(p, q, i, j) * f.coeffs[j];
                let pw = &self.ctrl[i - j];

                qi.x += coef * pw.x;
                qi.y += coef * pw.y;
                qi.z += coef * pw.z;
                qi.w += coef * pw.w;
            }

            out[i] = qi;
        }

        Ok(())
    }
```
```rust
    /// full range (s=0..=p+q) 계산
    pub fn multiply_bezier_function(&self, f: &BezierFunction) -> Result<BezierCurve, NurbsError> {
        let p = f.degree;
        let q = self.degree;
        let n = p + q;

        let mut qctrl = vec![Point4D::zero(); n + 1];
        self.multiply_bezier_function_range_inplace(f, 0, n, &mut qctrl)?;

        Ok(BezierCurve {
            dim: self.dim,
            degree: n,
            ctrl: qctrl,
        })
    }
```
```rust
    pub fn multiply_bezier_function_range(
        &self,
        f: &BezierFunction,
        s: usize,
        e: usize,
    ) -> Result<Vec<Point4D>, NurbsError> {
        let p = f.degree;
        let q = self.degree;
        let n = p + q;

        let mut out = vec![Point4D::zero(); n + 1];
        self.multiply_bezier_function_range_inplace(f, s, e, &mut out)?;
        Ok(out)
    }
}
```
```rust
pub fn on_eval_power(a: &[f64], t: f64) -> f64 {
    // Horner
    let mut v = 0.0;
    for &c in a.iter().rev() {
        v = v * t + c;
    }
    v
}
```
```rust
pub fn on_eval_bernstein(b: &[f64], t: f64) -> f64 {
    // de Casteljau (수치 안정)
    let n = b.len() - 1;
    let mut tmp = b.to_vec();
    for r in 1..=n {
        for i in 0..=(n - r) {
            tmp[i] = (1.0 - t) * tmp[i] + t * tmp[i + 1];
        }
    }
    tmp[0]
}
```
### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::basis::on_product_matrix;
    use nurbslib::core::bezier_curve::{BezierCurve, BezierFunction};
    use nurbslib::core::prelude::{Point3D, Point4D};

    #[test]
    fn test_product_matrix_vandermonde_sum_to_one() {
        let p = 7usize;
        let q = 5usize;
        let n = p + q;

        for i in 0..=n {
            let jl = i.saturating_sub(q);
            let jh = p.min(i);

            let mut s = 0.0;
            for j in jl..=jh {
                s += on_product_matrix(p, q, i, j);
            }
            assert!((s - 1.0).abs() < 1e-12, "i={}, sum={}", i, s);
        }
    }
```
```rust
    #[test]
    fn test_bezier_function_curve_product() {
        // f(t) = 1(1-t) + 3t = 1 + 2t  (degree 1)
        let f = BezierFunction {
            degree: 1,
            coeffs: vec![1.0, 3.0],
        };

        // Quadratic Bezier curve C(t)
        // P0 = (0,0,0)
        // P1 = (1,2,0)
        // P2 = (2,0,0)
        let curve = BezierCurve {
            dim: 3,
            degree: 2,
            ctrl: vec![
                Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
                Point4D::homogeneous(1.0, 2.0, 0.0, 1.0),
                Point4D::homogeneous(2.0, 0.0, 0.0, 1.0),
            ],
        };

        // Compute product f(t) * C(t)
        let prod = curve.multiply_bezier_function(&f).expect("multiply bezier failed");

        assert_eq!(prod.degree, 3);
        assert_eq!(prod.ctrl.len(), 4);

        // Helper: evaluate product at t
        let eval = |t: f64| {
            let ft = 1.0 + 2.0 * t;
            let ct = curve.point_at(t);
            Point3D::new(ft * ct.x, ft * ct.y, ft * ct.z)
        };

        // Check t = 0
        let p0 = prod.point_at(0.0);
        let e0 = eval(0.0);
        assert!((p0.x - e0.x).abs() < 1e-12);
        assert!((p0.y - e0.y).abs() < 1e-12);
        assert!((p0.z - e0.z).abs() < 1e-12);

        // Check t = 0.5
        let p05 = prod.point_at(0.5);
        let e05 = eval(0.5);
        assert!((p05.x - e05.x).abs() < 1e-12);
        assert!((p05.y - e05.y).abs() < 1e-12);
        assert!((p05.z - e05.z).abs() < 1e-12);

        // Check t = 1
        let p1 = prod.point_at(1.0);
        let e1 = eval(1.0);
        assert!((p1.x - e1.x).abs() < 1e-12);
        assert!((p1.y - e1.y).abs() < 1e-12);
        assert!((p1.z - e1.z).abs() < 1e-12);
    }
```
```rust

    #[test]
    fn test_bezier_function_curve_product_sampling_consistency() {
        // degree-1 Bezier function in Bernstein form
        // f(t) = f0*(1-t) + f1*t
        let f = BezierFunction { degree: 1, coeffs: vec![1.0, 2.0] };

        // degree-2 Bezier curve (homogeneous w=1)
        let curve = BezierCurve {
            dim: 3,
            degree: 2,
            ctrl: vec![
                Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
                Point4D::homogeneous(1.0, 2.0, 0.0, 1.0),
                Point4D::homogeneous(2.0, 0.0, 0.0, 1.0),
            ],
        };

        let prod = curve.multiply_bezier_function(&f).expect("invalid bezier Curve");

        // Bernstein eval for degree-1 function
        let f_eval = |t: f64| -> f64 {
            f.coeffs[0] * (1.0 - t) + f.coeffs[1] * t
        };

        // sample points
        let ts = [0.0, 0.25, 0.5, 0.75, 1.0];

        for &t in &ts {
            let ft = f_eval(t);
            let c = curve.point_at(t);
            let expected = Point3D::new(ft * c.x, ft * c.y, ft * c.z);

            let got = prod.point_at(t);

            assert!((got.x - expected.x).abs() < 1e-12, "t={}", t);
            assert!((got.y - expected.y).abs() < 1e-12, "t={}", t);
            assert!((got.z - expected.z).abs() < 1e-12, "t={}", t);
        }
    }
```
```rust
    #[test]
    fn test_bezier_function_curve_product_constant_function() {
        // f(t) = 3 (degree 0)
        let f = BezierFunction { degree: 0, coeffs: vec![3.0] };

        let curve = BezierCurve {
            dim: 3,
            degree: 2,
            ctrl: vec![
                Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
                Point4D::homogeneous(1.0, 2.0, 0.0, 1.0),
                Point4D::homogeneous(2.0, 0.0, 0.0, 1.0),
            ],
        };

        let prod = curve.multiply_bezier_function(&f).expect("invalid bezier Curve");

        // control points should be scaled in homogeneous 4D
        assert_eq!(prod.degree, curve.degree + f.degree);

        for i in 0..=curve.degree {
            let a = &curve.ctrl[i];
            let b = &prod.ctrl[i]; // degree same shift: p=0이면 인덱스 그대로가 맞음
            assert!((b.x - 3.0 * a.x).abs() < 1e-12);
            assert!((b.y - 3.0 * a.y).abs() < 1e-12);
            assert!((b.z - 3.0 * a.z).abs() < 1e-12);
            assert!((b.w - 3.0 * a.w).abs() < 1e-12);
        }
    }
```
```rust
    #[test]
    fn test_bezier_function_curve_product_invalid_coeff_len_is_err() {
        let f = BezierFunction { degree: 1, coeffs: vec![1.0] }; // len should be 2
        let curve = BezierCurve {
            dim: 3,
            degree: 2,
            ctrl: vec![
                Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
                Point4D::homogeneous(1.0, 2.0, 0.0, 1.0),
                Point4D::homogeneous(2.0, 0.0, 0.0, 1.0),
            ],
        };

        assert!(curve.multiply_bezier_function(&f).is_err());
    }
}
```
---
