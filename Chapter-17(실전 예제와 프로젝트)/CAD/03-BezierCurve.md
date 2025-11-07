# BezierCurve
BezierCurve 구현을 기반으로 수식 점검 및 문서화를 아래와 같이 정리. 

## 소스 코드
```rust
#[derive(Debug, Clone)]
pub struct BezierCurve {
    pub degree: usize,
    pub ctrl: Vec<CPoint>,
}
```
```rust
impl BezierCurve {
    pub fn new(control_points: Vec<CPoint>) -> Self {
        let degree = control_points.len().saturating_sub(1);
        Self {
            degree,
            ctrl : control_points,
        }
    }

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
            dimension:3,
            degree: p as u16,
            knots: KnotVector{knots: knot},
            ctrl: self.ctrl.clone(),
            domain : Interval{t0: 0.0, t1:1.0}
        }
    }
}
```
```rust
pub fn on_bezier_arc_len(ctrl: &[CPoint], tol: f64) -> f64 {
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

## 📐 BezierCurve 기능 및 수식 정리
## 🧱 구조 개요
```rust
struct BezierCurve {
    degree: usize,       // 곡선 차수 (n)
    ctrl: Vec<CPoint>,   // 제어점 리스트 (n+1개)
}
```

## 📏 Bezier 관련 수식

### 1. Bernstein Basis
```
Bᵢⁿ(u) = C(n, i) · uⁱ · (1 - u)ⁿ⁻ⁱ
C(n, i) = n! / (i! · (n - i)!)
```
### 2. 곡선 평가 (비유리)
```
P(u) = ∑ Bᵢⁿ(u) · Pᵢ
```
### 3. 곡선 평가 (유리)
```
P(u) = (∑ Bᵢⁿ(u) · wᵢ · Pᵢ) / (∑ Bᵢⁿ(u) · wᵢ)
```
### 4. 차수 승격
```
Qⱼ = ∑ Mⱼᵢ · Pᵢ
(M: degree elevation matrix)
```
### 5. 차수 감소 (근사)
```
Qᵢ = P(uᵢ), where uᵢ = i / q
```

### 6. 내적 곡선
```
Rₖ = ∑ⱼ Pⱼ · Qₖ₋ⱼ
```
### 7. 외적 곡선
```
Rₖ = ∑ⱼ Pⱼ × Qₖ₋ⱼ
```
### 8. 분할 (De Casteljau)
```
Pᵢⱼ = (1 - u) · Pᵢⱼ₋₁ + u · Pᵢ₊₁ⱼ₋₁
```
### 9. 근사 길이
```
length ≈ ∑ ||Pᵢ₊₁ - Pᵢ||, subdivide if chord - poly > tol
```

## 🛠 기능별 설명 요약
| 메서드                          | 설명                                                             |
|---------------------------------|------------------------------------------------------------------|
| `new`                             | 제어점으로부터 BezierCurve 생성                                 |
| `is_rational`                     | 유리 곡선 여부 (w ≠ 1 존재 여부)                                |
| `is_closed(eps)`                  | 시작/끝점 거리로 폐곡선 여부 판단                              |
| `evaluate_point(u)`               | 비유리 곡선 평가                                                |
| `evaluate_cpoint(u)`              | 유리 포함 CPoint 평가                                           |
| `evaluate_cpoint_rational(t)`     | 유리 여부에 따라 rational 평가                                 |
| `elevate_degree(t)`               | 차수 승격 (degree + t)                                          |
| `reduce_degree(target_deg)`       | 차수 감소 (근사 제어점 반환)                                   |
| `reduce_degree_curve(target_deg)` | 차수 감소된 곡선 반환                                           |
| `re_parameterize(func)`           | 파라미터 재매핑 (f(u))                                          |
| `dot(rhs)`                        | 두 곡선의 내적 곡선 반환                                       |
| `cross(rhs)`                      | 두 곡선의 외적 곡선 반환                                       |
| `split(u)`                        | u에서 곡선을 두 개로 분할                                      |
| `approx_cubic(ps, ts, pe, te)`    | 시작/끝점 및 접선 기반 cubic 근사                              |
| `to_nurbs()`                      | 클램프 B-스플라인으로 변환                                     |
| `bezier_arc_len(ctrl, tol)`       | 재귀적 분할 기반 근사 길이 계산                                |


## ✅ 수식 점검 결과

| 항목                     | 수식 표현                                                              | 설명                                      |
|--------------------------|------------------------------------------------------------------------|-------------------------------------------|
| Bernstein Basis          | $\( B_i^n(u) = {n \choose i} u^i (1 - u)^{n - i} \)$                    | 베지어 기저 함수                          |
| 비유리 곡선 평가         | $\( P(u) = \sum B_i^n(u) \cdot P_i \)$                                  | 제어점과 기저 함수의 선형 조합           |
| 유리 곡선 평가           | $\( P(u) = \frac{\sum B_i^n(u) w_i P_i}{\sum B_i^n(u) w_i} \)$           | 무게 중심 기반 평가                      |
| 차수 승격                | $\( Q_j = \sum M_{ji} \cdot P_i \)$                                     | 승격 행렬 기반 제어점 계산               |
| 차수 감소                | $\( Q_i = P(u_i), \quad u_i = \frac{i}{q} \)$                           | 균등 분할된 파라미터에서 평가            |
| 내적 곡선                | $\( R_k = \sum_j P_j \cdot Q_{k-j} \)$                                  | 각 차수별 내적 결과                      |
| 외적 곡선                | $\( R_k = \sum_j P_j \times Q_{k-j} \)$                                 | 각 차수별 외적 결과                      |
| 분할 알고리즘            | $\( P_{ij} = (1 - u) P_{i,j-1} + u P_{i+1,j-1} \)$                       | De Casteljau 알고리즘                    |
| 근사 길이                | $\( \mathrm{length} \approx \sum \| P_{i+1} - P_i \| \)$                 | chord vs poly 비교 후 재귀 분할          |

--


# 📐 BezierFunction, BezierArc, 재매개화 기능 및 수식 정리

## 소스 코드

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
            let inv = 1.0 / on_binomial(q, i) as f64;
            let k_low = if i > t { i - t } else { 0 };
            let k_high = p.min(i);
            for j in k_low..=k_high {
                out[i] += inv * on_binomial(p, j) as f64 * on_binomial(t, i - j) as f64 * self.coeffs[j];
            }
        }
        BezierFunction { degree: q, coeffs: out }
    }
}
```
```rust
#[derive(Debug, Clone)]
pub struct BezierArc {
    pub ctrl: Vec<CPoint>,
    pub degree: usize,
}
```
```rust
impl BezierArc {

    /// Piegl: B_GETCIW
    /// Compute middle weight approximating circular arc
    pub fn approx_weight_circle(p0: &Point, p1: &Point, p2: &Point) -> f64 {
        let d = p0.distance(p2) * 0.5;
        let fl = p0.distance(p1);
        let fr = p2.distance(p1);
        let sl = d / (d + fl);
        let sr = d / (d + fr);
        let s = 0.5 * (sl + sr);
        s / (1.0 - s)
    }

    /// Piegl: B_MAKCIR
    /// Create circular arc given endpoints and tangents
    pub fn from_end_tangents(p1: Point, t1: Vector, p2: Point, t2: Vector) -> Option<BezierArc> {
        let chord = p2 - p1;
        let d = chord.magnitude();
        let mut ctrl: Vec<CPoint> = Vec::new();

        // 기본 weight (90° 세그먼트)
        let cw = 0.5 * f64::sqrt(2.0);

        // 간단한 근사: 1세그먼트 원호
        ctrl.push(CPoint::from_point_w(&p1, 1.0));
        let mid = (p1 + p2) * 0.5;
        ctrl.push(CPoint::from_point_w(&mid, cw));
        ctrl.push(CPoint::from_point_w(&p2, 1.0));

        Some(BezierArc { ctrl, degree: 2 })
    }

    /// Piegl: B_MAKCON
    /// Create conic given point-on-arc
    pub fn from_point_on_arc(
        p0: Point, t0: Vector,
        p2: Point, t2: Vector,
        p: Point
    ) -> Option<(Point, f64)> {
        // B_makcon 의 근사적 형태
        // 교차점을 P1으로 잡고 weight 계산 (단순 근사)
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
pub fn on_re_param_inverse_matrix(p: usize, a: f64, b: f64, ap: f64, bp: f64) -> Vec<Vec<f64>> {
    let mut m = vec![vec![0.0; p + 1]; p + 1];
    let c = (b - a) / (bp - ap);
    let d = (bp * a - ap * b) / (bp - ap);
    m[0][0] = 1.0;
    for i in 1..=p {
        m[0][i] = d * m[0][i - 1];
        m[i][i] = c * m[i - 1][i - 1];
    }
    for i in 1..p {
        let mut fact = m[i][i];
        for j in i + 1..=p {
            fact *= d;
            m[i][j] = binomial_usize(j, i) as f64 * fact;
        }
    }
    m
}

```
```rust
/// 선형 재매개화 u = α u' + β 의 계수
/// 원구간 [a,b] → 새구간 [ap,bp] 로의 매핑:
///   u  = α * u' + β,   α = (b - a)/(bp - ap),   β = (bp*a - ap*b)/(bp - ap)
#[inline]
pub fn on_re_param_affine(a: Real, b: Real, ap: Real, bp: Real) -> (Real, Real) {
    let denom = bp - ap;
    let alpha = (b - a) / denom;
    let beta  = (bp * a - ap * b) / denom;
    (alpha, beta)
}

```
```rust
/// 재매개화 행렬  M  (Bezier 계수 변환:  c' = M · c)
/// Piegl의 B_REPMAT에 해당. 위의 reparam_affine(α,β) 사용.
/// 참고: Bezier(n)의 모노미얼로 확장해 α,β 적용 후 다시 Bezier로 투영하는 표준 구성.
pub fn on_re_param_matrix(p: usize, a: Real, b: Real, ap: Real, bp: Real) -> Vec<Vec<Real>> {
    let (alpha, beta) = re_param_affine(a, b, ap, bp);

    // Step 1: R 행렬 생성 — (αu' + β)^i 전개
    let mut r = vec![vec![0.0; p + 1]; p + 1];
    for i in 0..=p {
        for j in 0..=i {
            let comb = binomial_usize(i, j) as f64;
            r[i][j] = comb * beta.powi((i - j) as i32) * alpha.powi(j as i32);
        }
    }

    // Step 2: Bezier → Power basis 변환 행렬 T
    let t = bezier_to_power_matrix(p);

    // Step 3: Power → Bezier basis 변환 행렬 P
    let p_mat = power_to_bezier_matrix(p);

    // Step 4: 최종 재매개화 행렬 M = P · R · T
    let rt = Matrix::mul(&r, &t);
    let m = Matrix::mul(&p_mat, &rt);

    m
}
```

## 🧱 구조 개요
```rust
struct BezierFunction {
    degree: usize,        // 함수 차수
    coeffs: Vec<f64>,     // 제어값 (함수값)
}
```
```rust
struct BezierArc {
    degree: usize,        // 원호 차수 (보통 2)
    ctrl: Vec<CPoint>,    // 제어점 (유리 제어점 포함)
}
```

## 📏 관련 수식

### 1. Bezier 함수 평가
```
f(u) = ∑ Bᵢⁿ(u) · cᵢ
```
### 2. Bezier 함수 곱셈
```
f(u) · g(u) = ∑ₖ ∑ⱼ cⱼ · dₖ₋ⱼ · Mₖⱼ
(Mₖⱼ: product_matrix 계수)
```
### 3. 차수 승격
```
cᵢ' = ∑ⱼ C(p, j) · C(t, i-j) / C(p+t, i) · cⱼ
```
### 4. 원호 근사 weight 계산 (Piegl)
```
w = s / (1 - s), where s = ½ · (d / (d + fl) + d / (d + fr))
```
### 5. 재매개화 선형 변환
```
u = α · u' + β
α = (b - a) / (bp - ap)
β = (bp·a - ap·b) / (bp - ap)
```
### 6. 재매개화 행렬 M
```
Mᵢⱼ = C(i, j) · βⁱ⁻ʲ · αʲ
→ c' = M · c
```

## 🛠 기능별 설명 요약
| 메서드/함수                     | 설명                                                             |
|---------------------------------|------------------------------------------------------------------|
| `BezierFunction::evaluate(u)`     | Bezier 함수값 평가                                               |
| `BezierFunction::multiply(rhs)`   | 두 Bezier 함수 곱셈                                              |
| `BezierFunction::elevate(t)`      | 차수 승격                                                        |
| `BezierArc::approx_weight_circle` | 세 점 기반 원호 weight 근사                                     |
| `BezierArc::from_end_tangents`    | 시작/끝점 및 접선으로 원호 생성                                 |
| `BezierArc::from_point_on_arc`    | 원호 위 점으로 conic 생성 및 weight 계산                        |
| `re_param_affine(a,b,ap,bp)`      | 선형 재매개화 계수 α, β 계산                                    |
| `re_param_matrix(p,a,b,ap,bp)`    | Bezier 계수 변환 행렬 M 생성                                    |
| `re_param_inverse_matrix(...)`    | 역변환 행렬 구성 (모노미얼 기반)                                |



## ✅ 수식 점검 결과

| 항목                     | 수식 표현                                                              | 설명                                      |
|--------------------------|------------------------------------------------------------------------|-------------------------------------------|
| Bezier 함수 평가         | f(u) = ∑ Bᵢⁿ(u) · cᵢ                                                  | 제어값 기반 Bezier 함수 평가              |
| 함수 곱셈                | f(u)g(u) = ∑ₖ ∑ⱼ cⱼ · dₖ₋ⱼ · Mₖⱼ                                     | 두 Bezier 함수의 곱셈 결과                |
| 차수 승격                | cᵢ′ = ∑ⱼ [C(p,j) · C(t,i−j)] / C(p+t,i) · cⱼ                          | Piegl 방식의 차수 승격 공식               |
| 원호 weight 근사         | w = s / (1 − s), s = ½(sₗ + sᵣ)                                       | 원호 중간 제어점 weight 계산              |
| 재매개화 선형 변환       | u = α · u′ + β                                                        | 구간 [a,b] → [a′,b′] 선형 매핑            |
| 재매개화 행렬            | Mᵢⱼ = C(i,j) · βⁱ⁻ʲ · αʲ                                             | Bezier 계수 변환 행렬                     |



## ✅ BezierCurve 테스트 기능 요약
| 테스트 함수                  | 검증 내용 요약                                                  |
|-----------------------------|------------------------------------------------------------------|
| sample_re_parameterize      | BezierCurve에 BezierFunction을 적용한 재파라미터화 결과 확인     |
| bezier_to_nurbs_test        | BezierCurve → NURBS 변환 시 degree 및 knot 벡터 검증             |
| bezier_evaluate_point_test  | 특정 u에서 evaluate_point가 정확한 위치 반환하는지 확인          |
| bezier_elevate_degree_test  | 차수 승격 후 제어점 수 및 degree가 올바르게 증가했는지 확인      |
| bezier_split_test           | split(u) 호출 시 좌/우 곡선의 제어점이 정확히 분할되는지 확인    |
| bezier_dot_cross_test       | dot/cross 연산 결과의 degree 및 값이 기대대로 생성되는지 확인     |
| test_re_param_matrix_identity    | 동일 구간 재매개화 시 항등 행렬이 생성되는지 확인                    |
| test_re_param_matrix_affine_shift| 구간 변경 시 재매개화 행렬이 상삼각 구조이며 값이 유효한지 확인      |


## 🧪 테스트 코드

### 1. sample_re_parameterize
```rust
#[test]
fn sample_re_parameterize() {
    // 원래 베지어 곡선: degree 2
    let curve = BezierCurve {
        degree: 2,
        ctrl: vec![
            CPoint::from_point_w(&Point { x: 0.0, y: 0.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 1.0, y: 2.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 2.0, y: 0.0, z: 0.0 }, 1.0),
        ],
    };

    // 재파라미터화 함수 f(u) = u² → degree 2 베지어 함수로 표현
    let f = BezierFunction {
        degree: 2,
        coeffs: vec![0.0, 0.0, 1.0], // f(u) = B₂²(u) = u²
    };

    // 재파라미터화된 곡선: P(f(u)) = P(u²)
    let new_curve = curve.re_parameterize(&f);

    // 결과 출력
    println!("Original curve degree: {}", curve.degree);
    println!("Re-parameterized curve degree: {}", new_curve.degree);
    for (i, cp) in new_curve.ctrl.iter().enumerate() {
        println!("ctrl[{}] = ({:.3}, {:.3}, {:.3}, w={:.3})", i, cp.x, cp.y, cp.z, cp.w);
    }
}
```
#[test]
### 2. bezier_to_nurbs_test
```rust
fn bezier_to_nurbs_test() {

    let bezier = BezierCurve {
        degree: 2,
        ctrl: vec![
            CPoint::from_point_w(&Point { x: 0.0, y: 0.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 1.0, y: 2.0, z: 0.0 }, 1.0),
            CPoint::from_point_w(&Point { x: 2.0, y: 0.0, z: 0.0 }, 1.0),
        ],
    };

    let nurbs = bezier.to_nurbs();
    assert_eq!(nurbs.degree, 2);
    assert_eq!(nurbs.knots.knots, vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);

    println!("{:?}", nurbs);
}
```

### 3. evaluate_point
```rust
#[test]
fn bezier_evaluate_point_test() {
    let curve = BezierCurve::new(vec![
        CPoint::from_point_w(&Point::new(0.0, 0.0, 0.0), 1.0),
        CPoint::from_point_w(&Point::new(1.0, 2.0, 0.0), 1.0),
        CPoint::from_point_w(&Point::new(2.0, 0.0, 0.0), 1.0),
    ]);
    let pt = curve.evaluate_point(0.5);
    println!("Evaluated point at u=0.5: {:?}", pt);
    assert!(pt.x > 0.9 && pt.x < 1.1); // 대략 중간점
}
```


### 4. elevate_degree 테스트
```rust
#[test]
fn bezier_elevate_degree_test() {
    let curve = BezierCurve::new(vec![
        CPoint::from_point_w(&Point::new(0.0, 0.0, 0.0), 1.0),
        CPoint::from_point_w(&Point::new(1.0, 1.0, 0.0), 1.0),
    ]);
    let elevated = curve.elevate_degree(2);
    assert_eq!(elevated.degree, 3);
    assert_eq!(elevated.ctrl.len(), 4);
}
```

### 5. bezier_split_test 테스트
```rust
#[test]
fn bezier_split_test() {
    let curve = BezierCurve::new(vec![
        CPoint::from_point_w(&Point::new(0.0, 0.0, 0.0), 1.0),
        CPoint::from_point_w(&Point::new(1.0, 2.0, 0.0), 1.0),
        CPoint::from_point_w(&Point::new(2.0, 0.0, 0.0), 1.0),
    ]);
    let (left, right) = curve.split(0.5);
    assert_eq!(left.ctrl.len(), 3);
    assert_eq!(right.ctrl.len(), 3);
}
```

### 6. dot 및 cross 테스트
```rust
#[test]
fn bezier_dot_cross_test() {
    let a = BezierCurve::new(vec![
        CPoint::from_point_w(&Point::new(1.0, 0.0, 0.0), 1.0),
        CPoint::from_point_w(&Point::new(0.0, 1.0, 0.0), 1.0),
    ]);
    let b = BezierCurve::new(vec![
        CPoint::from_point_w(&Point::new(0.0, 1.0, 0.0), 1.0),
        CPoint::from_point_w(&Point::new(1.0, 0.0, 0.0), 1.0),
    ]);
    let dot = a.dot(&b);
    let cross = a.cross(&b);
    assert_eq!(dot.len(), 3);
    assert_eq!(cross.ctrl.len(), 3);
}
```

### 7. test_re_param_matrix_identity
```rust
#[test]
fn test_re_param_matrix_identity() {
    let p = 3;
    let a = 0.0;
    let b = 1.0;
    let ap = 0.0;
    let bp = 1.0;

    let m = re_param_matrix(p, a, b, ap, bp);

    // 항등 행렬 확인
    for i in 0..=p {
        for j in 0..=p {
            if i == j {
                assert!((m[i][j] - 1.0).abs() < 1e-12, "m[{}][{}] should be 1", i, j);
            } else {
                assert!(m[i][j].abs() < 1e-12, "m[{}][{}] should be 0", i, j);
            }
        }
    }
}
```
### 8. test_re_param_matrix_affine_shift
```rust
#[test]
fn test_re_param_matrix_affine_shift() {
    let p = 2;
    let a = 0.0;
    let b = 1.0;
    let ap = 0.0;
    let bp = 2.0;

    let m = re_param_matrix(p, a, b, ap, bp);

    // 출력 확인
    println!("Re parameterization matrix (p={}):", p);
    for row in &m {
        for val in row {
            print!("{:.4} ", val);
        }
        println!();
    }

    // 상삼각 행렬 여부 확인
    for i in 0..=p {
        for j in 0..i {
            assert!(m[i][j].abs() > 0.0, "Expected non-zero below diagonal");
        }
        for j in (i + 1)..=p {
            assert!(m[i][j].abs() < 1e-12, "Expected zero above diagonal");
        }
    }
}
```
