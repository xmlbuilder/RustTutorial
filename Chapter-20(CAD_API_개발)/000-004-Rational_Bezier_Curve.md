# 📘 Rational Bézier Curves: 수학적 필요성과 정의
## 🔷 1. 왜 rational이 필요한가?
- ✔ 문제 제기: 원(circle)을 다항식으로 표현할 수 있는가?
- 원 위의 점은 다음 조건을 만족해야 한다:
```math
x(u)^2+y(u)^2=1
```
- 만약 x(u),y(u)가 다항식이라면:
```math
x(u)=a_0+a_1u+\cdots +a_nu^n,\quad y(u)=b_0+b_1u+\cdots +b_nu^n
```
- 그러면:
```math
x(u)^2+y(u)^2-1=0
```
- 이건 항등식이어야 하므로, 모든 계수가 0이어야 한다.

## ✔ 전개 결과:
```math
x(u)^2+y(u)^2-1=\sum _{k=0}^{2n}c_ku^k
```
- 이때:
- $c_{2n}=a_n^2+b_n^2=0\Rightarrow a_n=b_n=0$
- $c_{2n-1}=2a_{n-1}a_n+2b_{n-1}b_n=0\Rightarrow a_{n-1}=b_{n-1}=0$
- … 반복하면 모든 계수가 0이어야 함
- 결국:
```math
x(u)=a_0,\quad y(u)=b_0,\quad \mathrm{상수\  함수}
```
- 원은 다항식으로 표현 불가능

## 🔷 2. 해결책: Rational 표현
- ✔ 정의
```math
x(u)=\frac{X(u)}{W(u)},\quad y(u)=\frac{Y(u)}{W(u)}
```
- 여기서 X(u),Y(u),W(u)는 다항식
    - rational 함수로 표현하면 원도 가능

- ✔ 예시: 단위원
```math
x(u)=\frac{1-u^2}{1+u^2},\quad y(u)=\frac{2u}{1+u^2}
```
- 이 곡선은 $x(u)^2+y(u)^2=1$ 을 만족함

## 🔷 3. Rational Bézier 곡선의 정의
- ✔ 일반형
```math
C(u)=\frac{\sum _{i=0}^nB_{i,n}(u)\, P_i\, w_i}{\sum _{i=0}^nB_{i,n}(u)\, w_i}\quad 0\leq u\leq 1
```
- $P_i=(x_i,y_i,z_i)$: 컨트롤 포인트
- $w_i$: weight
- $B_{i,n}(u)$: Bernstein basis

- ✔ Rational basis 함수
```math
R_{i,n}(u)=\frac{B_{i,n}(u)\, w_i}{\sum _{j=0}^nB_{j,n}(u)\, w_j}
```
- 그러면:
```math
C(u)=\sum _{i=0}^nR_{i,n}(u)\, P_i
```

## 🔷 4. Rational basis의 성질
- P1.8 비음수성
```math
R_{i,n}(u)\geq 0
```
- P1.9 Partition of unity
```math
\sum _{i=0}^nR_{i,n}(u)=1
```
- P1.10 끝점 조건
```math
R_{0,n}(0)=1,\quad R_{n,n}(1)=1
```
- P1.11 최대값 위치
```math
R_{i,n}(u)\  \mathrm{는}\  u=\frac{i}{n}\  \mathrm{에서\  최대값을\  가짐}
```
- P1.12 특수 경우
```math
w_i=1\  \forall i\Rightarrow R_{i,n}(u)=B_{i,n}(u)
```

## 🔷 5. Rational Bézier 곡선의 기하학적 성질
- P1.13 Convex hull property
    - 곡선은 weighted convex hull 안에 있음
- P1.14 변환 불변성
    - 회전, 이동, 스케일은 컨트롤 포인트에 적용하면 곡선에도 적용됨
- P1.15 Variation diminishing property
    - 곡선은 컨트롤 폴리라인보다 덜 진동함
- P1.16 끝점 보간
```math
C(0)=P_0,\quad C(1)=P_n
```
- P1.17 도함수 방향
```math
C'(0)\parallel P_1-P_0,\quad C'(1)\parallel P_n-P_{n-1}
```
- P1.18 Polynomial Bézier는 특수한 경우
    - w_i=1이면 rational Bézier는 polynomial Bézier와 동일

## 🔷 6. 한 문장 요약
- Rational Bézier 곡선은 다항식 Bézier로는 표현할 수 없는 원, 타원, 쌍곡선 같은 곡선을  
    rational basis를 통해 정확하게 표현할 수 있으며,
- 기존 Bézier의 모든 기하학적 성질을 유지하면서 더 넓은 표현력을 제공한다.

```rust
#[derive(Debug, Clone)]
pub struct BezierCurve {
    pub dim: usize,
    pub degree: usize,
    pub ctrl: Vec<Point4D>,
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

        let mut r = Point4D::zero();
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
    pub fn eval_h_ders(&self, t: Real, d_max: usize) -> Option<[Point4D; 4]> {
        let n = self.ctrl.len();
        if n == 0 {
            return None;
        }
        let p = n - 1;

        let d_max = d_max.min(3);

        let mut out = [
            Point4D::zero(),
            Point4D::zero(),
            Point4D::zero(),
            Point4D::zero(),
        ];

        // degree 0: constant
        if p == 0 {
            out[0] = self.ctrl[0]; // already homogeneous
            return Some(out);
        }

        // 0th: original Bezier in homogeneous
        out[0] = Self::eval_h_bezier(&self.ctrl, t);

        if d_max >= 1 {
            // Q_i = p * (P_{i+1} - P_i), i=0..p-1   (all in homogeneous)
            let s1 = p as Real;
            let mut d1 = Vec::with_capacity(p);
            for i in 0..p {
                let a = self.ctrl[i];
                let b = self.ctrl[i + 1];
                d1.push(Point4D::new(
                    s1 * (b.x - a.x),
                    s1 * (b.y - a.y),
                    s1 * (b.z - a.z),
                    s1 * (b.w - a.w),
                ));
            }
            out[1] = Self::eval_h_bezier(&d1, t);

            if d_max >= 2 && p >= 2 {
                // R_i = (p-1) * (Q_{i+1} - Q_i), i=0..p-2
                let s2 = (p - 1) as Real;
                let mut d2 = Vec::with_capacity(p - 1);
                for i in 0..(p - 1) {
                    let a = d1[i];
                    let b = d1[i + 1];
                    d2.push(Point4D::new(
                        s2 * (b.x - a.x),
                        s2 * (b.y - a.y),
                        s2 * (b.z - a.z),
                        s2 * (b.w - a.w),
                    ));
                }
                out[2] = Self::eval_h_bezier(&d2, t);

                if d_max >= 3 && p >= 3 {
                    // S_i = (p-2) * (R_{i+1} - R_i), i=0..p-3
                    let s3 = (p - 2) as Real;
                    let mut d3 = Vec::with_capacity(p - 2);
                    for i in 0..(p - 2) {
                        let a = d2[i];
                        let b = d2[i + 1];
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
---

## 📘 1. Homogeneous Bézier Curve 평가 수식
- (eval_h_bezier)
- 동차 Bézier 곡선:
```math
C^w(t)=\sum _{i=0}^pB_{i,p}(t)\, P_i^w
```
- 여기서
```math
P_i^w=(x_iw_i,\; y_iw_i,\; z_iw_i,\; w_i)
```
- 코드에서 하는 일:
    - Bernstein basis B_{i,p}(t) 계산
    - 선형 결합
    - 결과는 4D 점 (X(t),Y(t),Z(t),W(t))
- ✔ Bernstein 계산 수식
- 코드는 다음 재귀식을 사용:
```math
B_{0,0}=1
```
```math
B_{j,k}(t)=(1-t)B_{j,k-1}(t)+tB_{j-1,k-1}(t)
```
- 이걸 1D 배열로 구현한 것이:
```rust
b[j] = saved + v * temp;
saved = u * temp;
```

- 즉, de Casteljau의 Bernstein 버전.
- ✔ 최종 4D 점
```math
C^w(t)=\sum _{i=0}^pB_{i,p}(t)\, P_i^w
```
## 📘 2. Bézier 곡선의 동차 도함수 수식
- (eval_h_ders)
- Bézier 곡선의 k차 도함수는 다음과 같다.

- ✔ 1차 도함수
```math
C^{w'}(t) = \sum _{i=0}^{p-1}B_{i,p-1}(t) * Q_i
```
- 여기서
```math
Q_i=p(P_{i+1}^w-P_i^w)
```
- 코드:
```rust
Q_i = p * (P[i+1] - P[i])
```


- ✔ 2차 도함수
```math
C^{w''}(t)=\sum _{i=0}^{p-2}B_{i,p-2}(t)\, R_i
```
여기서
```math
R_i=(p-1)(Q_{i+1}-Q_i)
```
- 코드:
```rust
R_i = (p-1) * (Q[i+1] - Q[i])
```


- ✔ 3차 도함수
```math
C^{w'''}(t)=\sum _{i=0}^{p-3}B_{i,p-3}(t)\, S_i
```
- 여기서
```math
S_i=(p-2)(R_{i+1}-R_i)
```
- 코드:
```rust
S_i = (p-2) * (R[i+1] - R[i])
```


## 📘 3. Rational Curve 도함수 수식
- (eval_point_and_ders)
- 유클리드 점:
```math
C(t)=\frac{C^w(t)}{W(t)}
```
- 여기서
```math
C^w(t)=(X(t),Y(t),Z(t),W(t))
```
- ✔ 1차 도함수
- Rational curve의 도함수는 다음 공식:
```math
C'(t)=\frac{WC^{w'}-C^wW'}{W^2}
```
- 코드:
```rust
num = v1 * w0 - v0 * w1;
den = w0^2;
```


- ✔ 2차 도함수
- 정확한 수식:
```math
C''(t)=\frac{W^2C^{w''}-2WW'C^{w\, '}-C^w(WW''-2W'^2)}{W^3}
```
- 코드:
```rust
term1 = v2 * (w0*w0)
term2 = v1 * (2 w0 w1)
term3 = v0 * (w0 w2 - 2 w1^2)
num = term1 - term2 - term3
den = w0^3
```


- ✔ 3차 도함수
- 정확한 수식:
```math
C'''(t) =
( W^3 * X_{3}
  - 3 W^2 W_{1} * X_{2}
  - 3 W * X1 * ( W W_{2} - 2 W1^2 )
  - X_{0} * ( W^2 W_{3} - 6 W W_{1} W_{2} + 6 W_{1}^3 )
) / W^4
```
- 코드:
```rust
term1 = v3 * (w0^3)
term2 = v2 * (3 w0^2 w1)
term3 = v1 * (3 w0 (w0 w2 - 2 w1^2))
term4 = v0 * (w0^2 w3 - 6 w0 w1 w2 + 6 w1^3)
num = term1 - term2 - term3 - term4
den = w0^4
```


## 📘 4. Tangent, Curvature, Torsion 수식

- ✔ 단위 접선 벡터
```math
T(t)=\frac{C'(t)}{\| C'(t)\| }
```
- ✔ 곡률
```math
\kappa (t)=\frac{\| C'(t)\times C''(t)\| }{\| C'(t)\| ^3}
```
✔ 비틀림 (torsion)
```math
\tau (t)=\frac{\det (C',C'',C''')}{\| C'\times C''\| ^2}
```
- 코드:
```rust
let d1 = ders[0];
let v = d1.cross(&d2);
let num = v.length();
let denom = d1.length();
if denom <= 1e-15 {
    return None;
}
let denom3 = denom * denom * denom;
```

## 📘 5. 전체 구조 요약
- 이 Rust 구현은 다음 수학적 구조를 그대로 따른다:

| Component                     | Mathematical Expression                                                |
|------------------------------|-------------------------------------------------------------------------|
| Homogeneous curve            | $C^w(t) = \sum_{i=0}^{p} B_{i,p}(t)\, P_i^w$                    |
| Homogeneous derivatives      | $C^{w\,'}(t),\; C^{w\,''}(t),\; C^{w\,'''}(t)$                     |
| Rational tangent             | $T(t) = \frac{C'(t)}{\|C'(t)\|}$                                 |
| Curvature                    | $\kappa(t) = \frac{\| C'(t) \times C''(t) \|}{\|C'(t)\|^3}$       |
| Torsion                      | $\tau(t) = \frac{\det(C',C'',C''')}{\| C' \times C'' \|^2}$     |


- 즉:
  - Piegl & Tiller의 Rational Bézier 곡선 이론을 100% 수식 그대로 Rust로 옮긴 완전한 구현이다.
---
