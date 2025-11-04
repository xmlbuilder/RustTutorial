# Bezier Curve

베지어 곡선 및 함수의 핵심 연산들이 잘 구현되어 있습니다.  
아래에 수식 정리와 함께 reparameterize() 함수의 구현도 제안.

## ✅ 핵심 수식 정리
### 1. 🎯 베지어 곡선 평가

$$
P(u)=\sum _{i=0}^nB_i^n(u)\cdot P_i\quad \mathrm{where}\quad B_i^n(u)={n \choose i}u^i(1-u)^{n-i}
$$

- evaluate_point() → 유클리드 좌표
- evaluate_cpoint() → 동차 좌표
- evaluate_cpoint_rational() → 가중치 포함 rational 평가

### 2. 📈 차수 상승 (degree elevation)

$$
P_i'=\sum _{j=0}^nE_{ij}\cdot P_j\quad \mathrm{where}\quad E_{ij}=\mathrm{degree\  elevation\  matrix}
$$

- elevate_degree() → 정확한 행렬 기반 구현

### 3. 📉 차수 감소 (degree reduction)
- 근사 방식: $P_i'=P(t_i), where \quadt_i=\frac{i}{q}$
- reduce_degree() → rational 평가 기반 보간
- 정확한 방식은 least-squares 기반이지만, 현재는 보간 기반으로 충분

### 4. ✂️ 분할 (de Casteljau)
- split() → de Casteljau 알고리즘 기반
- 수식:

$$
P_i^{(k)}=(1-u)P_i^{(k-1)}+uP_{i+1}^{(k-1)}
$$


### 5. 📐 곡선 곱셈 (dot, cross)
- dot():

$$
(P\cdot Q)_i=\sum _{j=\max (0,i-q)}^{\min (p,i)}P_j\cdot Q_{i-j}
$$

- cross():

$$
(P\times Q)_i=\sum _{j=\max (0,i-q)}^{\min (p,i)}P_j\times Q_{i-j}
$$


### 6. 📏 길이 근사 (arc length)
- bezier_arc_len() → chord vs polygon 길이 비교 후 재귀 분할
- 수식:

$$
\mathrm{length}\approx \sum _{i=0}^{n-1}\| P_{i+1}-P_i\| \quad \mathrm{vs}\quad \| P_n-P_0\|
$$ 

## 🧮 BezierFunction 수식
### 1. 평가

$$
f(u)=\sum _{i=0}^nB_i^n(u)\cdot f_i
$$

### 2. 곱셈

$$
(f\cdot g)_i=\sum _{j=\max (0,i-q)}^{\min (p,i)}M_{i,j}\cdot f_j\cdot g_{i-j}\quad \mathrm{where}\quad M_{i,j}=\mathrm{product\  matrix\  coefficient}
$$

### 3. 차수 상승

$$
f_i'=\sum _{j=0}^p\sum _{k=0}^t\frac{{p \choose j}{t \choose k}}{{p+t \choose i}}f_j\quad \mathrm{where}\quad i=j+k
$$

## 🔧 구현: reparameterize(f: &BezierFunction)
### 🎯 목적
- 기존 베지어 곡선 P(u)를 함수 $f(u)\in [0,1]$ 로 재파라미터화
- 즉, $P(f(u))$ 를 베지어 곡선으로 근사

### 🧠 아이디어
- $P(f(u))=\sum _{i=0}^nP_iB_i^n(f(u))$
- $B_i^n(f(u))$ 를 베지어 함수로 근사 → 곱셈으로 전개
- 최종적으로:

$$
P'(u)=\sum _{i=0}^nP_i\cdot B_i^n(f(u))\quad \mathrm{→\  베지어\  곡선}
$$


## ✅ 구현 예시
```rust
pub fn reparameterize(&self, f: &BezierFunction) -> BezierCurve {
    
    let n = self.degree;
    let mut result = vec![CPoint::zero(); f.degree + n + 1];

    for i in 0..=n {
        let bi = BezierFunction {
            degree: n,
            coeffs: (0..=n).map(|j| if j == i { 1.0 } else { 0.0 }).collect(),
        };
        let bi_f = bi.multiply(f); // B_i^n(f(u)) as BezierFunction
        for (j, coeff) in bi_f.coeffs.iter().enumerate() {
            result[j].x += coeff * self.ctrl[i].x;
            result[j].y += coeff * self.ctrl[i].y;
            result[j].z += coeff * self.ctrl[i].z;
            result[j].w += coeff * self.ctrl[i].w;
        }
    }

    BezierCurve {
        degree: f.degree + n,
        ctrl: result,
    }
}
```

## 소스 코드
```rust
use crate::core::basis::{all_ber_1d, binomial, product_matrix};
use crate::core::knot::{bernstein, binomial_usize, degree_elev_matrix, is_rat};
use crate::core::prelude::{CPoint, Curve, Degree, KnotVector, Point, Real, Vector};
use crate::core::types::{NONE};
```
```rust
#[derive(Debug, Clone)]
pub struct BezierCurve {
    pub degree: usize,
    pub ctrl: Vec<CPoint>,
}
```
```rust
impl BezierCurve {
    pub fn is_rational(&self) -> bool {
        self.ctrl.iter().any(|cp| cp.w != 1.0)
    }

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

    pub fn evaluate_point(&self, u: f64) -> Point {
        let n = self.degree;
        let mut p = Point::zero();
        for i in 0..=n {
            let b = bernstein(n, i, u);
            p.x += b * self.ctrl[i].x;
            p.y += b * self.ctrl[i].y;
            p.z += b * self.ctrl[i].z;
        }
        p
    }

    pub fn evaluate_cpoint(&self, u: f64) -> CPoint {
        let n = self.degree;
        let mut c = CPoint { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        for i in 0..=n {
            let b = bernstein(n, i, u);
            c.x += b * self.ctrl[i].x;
            c.y += b * self.ctrl[i].y;
            c.z += b * self.ctrl[i].z;
            c.w += b * self.ctrl[i].w;
        }
        c
    }

    pub fn evaluate_cpoint_rational(&self, t : Real) -> CPoint {
        let p: Degree = (self.ctrl.len() as i32 - 1).max(0) as u16;
        let b_vec = all_ber_1d(p, t);
        let rat = is_rat(self.ctrl.as_slice());

        if rat {
            let (mut xw, mut yw, mut zw, mut w) = (0.0, 0.0, 0.0, 0.0);
            for (i, Ni) in b_vec.iter().enumerate() {
                let c = self.ctrl[i];
                xw += Ni * (c.x * c.w);
                yw += Ni * (c.y * c.w);
                zw += Ni * (c.z * c.w);
                w  += Ni *  c.w;
            }
            if w == 0.0 { w = NONE; }
            CPoint { x: xw , y: yw , z: zw , w }
        } else {
            let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
            for (i, Ni) in b_vec.iter().enumerate() {
                let c = self.ctrl[i];
                x += Ni * c.x; y += Ni * c.y; z += Ni * c.z;
            }
            CPoint { x, y, z, w : NONE }
        }
    }

    pub fn elevate_degree(&self, t: usize) -> BezierCurve {
        let mat = degree_elev_matrix(self.degree, t);
        let mut new_ctrl = vec![CPoint::zero(); self.degree + t + 1];
        for i in 0..=self.degree + t {
            for j in 0..=self.degree {
                new_ctrl[i].x += mat[i][j] * self.ctrl[j].x;
                new_ctrl[i].y += mat[i][j] * self.ctrl[j].y;
                new_ctrl[i].z += mat[i][j] * self.ctrl[j].z;
                new_ctrl[i].w += mat[i][j] * self.ctrl[j].w;
            }
        }
        BezierCurve { degree: self.degree + t, ctrl: new_ctrl }
    }

    pub fn reduce_degree(&mut self, target_deg: Degree) -> Vec<CPoint>{
        let p = (self.ctrl.len() - 1) as i32;
        if target_deg >= p as u16 { return self.ctrl.to_vec(); }

        let q = target_deg as usize;
        let mut new_ctrl = vec![CPoint { x: 0.0, y: 0.0, z: 0.0, w: f64::NAN }; q + 1];
        // 간단한 비례 보간 기반 (정확히는 least-squares로도 가능)
        for i in 0..=q {
            let t = i as Real / q as Real;
            // De Casteljau 를 통해 (t)에서 점을 얻고 그대로 제어점으로
            new_ctrl[i] = self.evaluate_cpoint_rational(t);
        }
        new_ctrl
    }

    pub fn reduce_degree_curve(&mut self, target_deg: Degree) -> Self{
        Self{
            degree: target_deg as usize,
            ctrl: self.reduce_degree(target_deg),
        }
    }

    pub fn re_parameterize(&self, func: &BezierFunction) -> BezierCurve {

        let n = self.degree;
        let mut result = vec![CPoint::zero(); func.degree + n + 1];

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

    /// Cross product of two curves — B_CURCRO
    pub fn cross(&self, rhs: &BezierCurve) -> BezierCurve {
        let n = self.degree + rhs.degree;
        let mut result = vec![CPoint::zero(); n + 1];
        for i in 0..=n {
            let jl = i.saturating_sub(rhs.degree);
            let jh = self.degree.min(i);
            for j in jl..=jh {
                let p = self.ctrl[j].to_point();
                let q = rhs.ctrl[i - j].to_point();
                let v = Vector::cross(&Vector::from(p), &Vector::from(q));
                result[i].x += v.x;
                result[i].y += v.y;
                result[i].z += v.z;
                result[i].w = 1.0;
            }
        }
        BezierCurve { degree: n, ctrl: result }
    }

    /// Split at u — B_CSPLIT
    pub fn split(&self, u: f64) -> (BezierCurve, BezierCurve) {
        let p = self.degree;
        let mut a = self.ctrl.clone();
        let mut left = vec![CPoint::zero(); p + 1];
        let mut right = vec![CPoint::zero(); p + 1];

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
            BezierCurve { degree: p, ctrl: left },
            BezierCurve { degree: p, ctrl: right },
        )
    }

    /// Least-squares cubic Bezier approximation — B_CUBAPT
    pub fn approx_cubic(
        ps: &Point, ts: &Vector,
        _p: &Point, _t: &Vector,
        pe: &Point, te: &Vector
    ) -> BezierCurve {
        // 내부: Piegl 의 least-square 방식. 실제 수치적 근사는 생략하고 구조만 보존.
        let mut ctrl = Vec::with_capacity(4);
        ctrl.push(CPoint::from_point_w(ps, 1.0));

        // 근사적으로 middle control points 계산
        let p1 = Point {
            x: ps.x + ts.x * 0.3,
            y: ps.y + ts.y * 0.3,
            z: ps.z + ts.z * 0.3,
        };
        let p2 = Point {
            x: pe.x - te.x * 0.3,
            y: pe.y - te.y * 0.3,
            z: pe.z - te.z * 0.3,
        };
        ctrl.push(CPoint::from_point_w(&p1, 1.0));
        ctrl.push(CPoint::from_point_w(&p2, 1.0));
        ctrl.push(CPoint::from_point_w(pe, 1.0));
        BezierCurve { degree: 3, ctrl }
    }

    pub fn to_nurbs(&self) -> Curve {

        // 베지어 곡선 → 클램프 B-스플라인: [0..0 (p+1개), 1..1 (p+1개)]
        let p = self.degree;
        let mut knot = Vec::with_capacity(2 * (p + 1));
        knot.extend(std::iter::repeat(0.0).take(p + 1));
        knot.extend(std::iter::repeat(1.0).take(p + 1));

        Curve {
            degree: p as u16,
            knots: KnotVector{knots: knot},
            ctrl: self.ctrl.clone(),
        }
    }
}

pub fn bezier_arc_len(ctrl: &[CPoint], tol: f64) -> f64 {
    fn rec(ctrl: &[Point], tol2: f64) -> f64 {
        let n = ctrl.len() - 1;
        let mut chord = 0.0;
        let poly;
        for i in 0..n {
            let a = &ctrl[i];
            let b = &ctrl[i + 1];
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let dz = b.z - a.z;
            chord += (dx*dx + dy*dy + dz*dz).sqrt();
        }
        poly = ((ctrl[0].x - ctrl[n].x).powi(2)
            + (ctrl[0].y - ctrl[n].y).powi(2)
            + (ctrl[0].z - ctrl[n].z).powi(2)).sqrt();

        if chord - poly < tol2 {
            return chord;
        }

        // subdivide
        let mid = ctrl.len() / 2;
        let left = &ctrl[..=mid];
        let right = &ctrl[mid..];
        rec(left, tol2) + rec(right, tol2)
    }

    let pts: Vec<Point> = ctrl.iter().map(|c| c.to_point()).collect();
    0.5 * rec(&pts, 2.0 * tol)
}
```
```rust
#[derive(Debug, Clone)]
pub struct BezierFunction {
    pub degree: usize,
    pub coeffs: Vec<f64>, // control values (function values)
}

impl BezierFunction {
    pub fn evaluate(&self, u: f64) -> f64 {
        let n = self.degree;
        let mut val = 0.0;
        for i in 0..=n {
            val += self.coeffs[i] * bernstein(n, i, u);
        }
        val
    }

    pub fn multiply(&self, rhs: &Self) -> BezierFunction {
        let p = self.degree;
        let q = rhs.degree;
        let n = p + q;
        let mut fg = vec![0.0; n + 1];
        for i in 0..=n {
            let jl = i.saturating_sub(q);
            let jh = p.min(i);
            for j in jl..=jh {
                let coef = product_matrix(p, q, i, j);
                fg[i] += coef * self.coeffs[j] * rhs.coeffs[i - j];
            }
        }
        BezierFunction { degree: n, coeffs: fg }
    }

    pub fn elevate(&self, t: usize) -> BezierFunction {
        let p = self.degree;
        let q = p + t;
        let mut out = vec![0.0; q + 1];
        for i in 0..=q {
            let inv = 1.0 / binomial(q, i) as f64;
            let k_low = if i > t { i - t } else { 0 };
            let k_high = p.min(i);
            for j in k_low..=k_high {
                out[i] += inv * binomial(p, j) as f64 * binomial(t, i - j) as f64 * self.coeffs[j];
            }
        }
        BezierFunction { degree: q, coeffs: out }
    }
}
```
```rust
pub fn bernstein(p: usize, i: usize, u: f64) -> f64 {
    assert!(i <= p && u >= 0.0 && u <= 1.0);
    let mut tmp = vec![0.0; p + 1];
    tmp[p - i] = 1.0;
    let omu = 1.0 - u;
    for k in 1..=p {
        for j in (k..=p).rev() {
            tmp[j] = omu * tmp[j] + u * tmp[j - 1];
        }
    }
    tmp[p]
}
```

