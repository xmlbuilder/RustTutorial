# 📘 NURBS 2차(Quadratic) Conic Utilities
- 이 문서는 원래의 conic_shape_invariance / conic_implicit_coeffs함수들의  
    수학적 의미와 기능을 설명한다.
- 모든 함수는 다음 조건을 전제로 한다.
    - 곡선은 단일 세그먼트
    - 차수(degree) = 2
    - 정확히 3개의 제어점
    - 곡선은 XY 평면(z=0) 위에 존재
    - Rational conic 또는 Non-rational conic 모두 지원

## 1. conic_shape_invariance()
- Conic Shape Invariance Factor k 계산
- 목적
- 2차 Rational NURBS 곡선이 **원뿔곡선(conic section)** 인지 판별하고,  
    그 형태를 결정하는 불변량(invariant) k 값을 계산한다.
- 수식
  - Rational conic의 경우:

```math
k=\frac{w_0\cdot w_2}{w_1^2}
```
  
  - Non-rational(모든 weight ≈ 1)인 경우:
  - k=1
  - 여기서
  - $w_0$, $w_1$, $w_2$ 는 3개의 제어점의 weight
  - $w_1$ 은 중간 제어점의 weight이며, 0에 가까우면 conic이 정의되지 않음

- 반환값
    - 성공: Ok(k)
    - 실패: Err(NurbsError)
    - degree ≠ 2
    - control point 개수 ≠ 3
    - weight가 유효하지 않음
    - rational인데 $w_1\approx 0$
- 의미
    - k=1 → 포물선(parabola)
    - k<1 → 타원(ellipse)
    - k>1 → 쌍곡선(hyperbola)

## 2. conic_implicit_coeffs()
- Implicit Conic Equation의 계수 계산
- 목적
    - NURBS 2차 원뿔곡선을 다음과 같은 Implicit Form으로 변환한다.
    ```math
    c_0x^2+c_1y^2+c_2xy+c_3x+c_4y+c_5=0
    ```
    - 이 식은 CAD 커널에서:
        - 교차 검사
        - 트리밍
        - 충돌 검사
        - 곡선-곡선 교차
        - 곡선-곡면 교차
    - 등에 사용되는 표준 형태다.
- 입력 조건
    - degree = 2
    - control point = 3
    - XY 평면(z=0)
    - Rational 또는 Non-rational conic
    - shape invariance k가 유효해야 함
- 내부 계산 과정 요약
    - 1) Shape invariance k 계산
        - 앞서 정의한:
        ```math
        k=\frac{w_0w_2}{w_1^2}
        ```
        - 이후 알고리즘에서는 역수를 사용:
        ```math
        k\leftarrow \frac{1}{k}
        ```

    - 2) 제어점을 유클리드 좌표로 변환
        ```math
        P_i=(x_i,y_i,z_i)
        ```
        - z는 반드시 0이어야 한다.
    - 3) 보조 변수 계산
        ```math
        h_0=x_0-x_1,\quad h_1=x_2-x_1
        ```
        ```math
        h_2=y_0-y_1,\quad h_3=y_2-y_1
        ```
        ```math
        g_1=h_2h_1-h_0h_3
        ```
        ```math
        s_1=h_0-h_1,\quad s_2=h_3-h_2
        ```

    - 4) 최종 계수 계산
        ```math
        c_0=s_2^2+4kh_2h_3
        ```
        ```math
        c_1=s_1^2+4kh_0h_1
        ```
        ```math
        c_2=2(s_1s_2-2k(h_0h_3+h_1h_2))
        ```
        ```math
        c_3=2(g_1s_2-c_0x_1-c_2y_1)
        ```
        ```math
        c_4=2(g_1s_1-c_1y_1-c_2x_1)
        ```
        ```math
        c_5=c_0x_1^2+c_1y_1^2+2c_2x_1y_1-2g_1(s_1y_1+s_2x_1)+g_1^2
        ```
- 반환값
    - 성공: [c0, c1, c2, c3, c4, c5]
    - 실패:
    - degree ≠ 2
    - control point 개수 ≠ 3
    - XY 평면이 아님
    - shape invariance k가 너무 작음

## 3. eval_conic_implicit_xy()
- Implicit conic 식을 평가하는 유틸리티
- 목적
    - Implicit conic 식:
    ```math
    F(x,y)=c_0x^2+c_1y^2+c_2xy+c_3x+c_4y+c_5
    ```
    - 을 특정 (x, y)에서 계산한다.
- 의미
    - F(x,y)=0 → 점이 conic 위에 있음
    - F(x,y)<0 → conic 내부
    - F(x,y)>0 → conic 외부
- CAD 커널에서 교차 검사에 필수.

## 전체 요약
| Function                  | Description / Formula                     |
|---------------------------|--------------------------------------------|
| conic_shape_invariance()  | k = (w0 * w2) / (w1 * w1)                  |
| conic_implicit_coeffs()   | Returns implicit conic coefficients c0..c5 |
| eval_conic_implicit_xy()  | F(x,y) = c0 x² + c1 y² + c2 xy + c3 x + c4 y + c5 |


---

## 소스 코드
```rust
// ---------------------------------------------------------------------------
// - conic curve lies in XY plane
// - single segment, degree 2
// - exactly 3 control points
// ---------------------------------------------------------------------------

impl NurbsCurve {
    ///   k = (w0*w2)/(w1*w1)
    /// Non-rational (all weights ~1) => k = 1.
    pub fn conic_shape_invariance(&self) -> Result<Real> {
        if !self.is_valid() {
            return Err(NurbsError::InvalidInput {
                msg: "conic_shape_invariance: invalid curve".into(),
            });
        }

        if self.degree != 2 {
            return Err(NurbsError::InvalidInput {
                msg: format!(
                    "conic_shape_invariance: degree must be 2 (got {})",
                    self.degree
                ),
            });
        }
        if self.ctrl.len() != 3 {
            return Err(NurbsError::InvalidInput {
                msg: format!(
                    "conic_shape_invariance: conic must have exactly 3 control points (got {})",
                    self.ctrl.len()
                ),
            });
        }

        // Match original "E_curwei" intent: weights must be finite and positive
        let w0 = self.ctrl[0].w;
        let w1 = self.ctrl[1].w;
        let w2 = self.ctrl[2].w;

        if !w0.is_finite() || !w1.is_finite() || !w2.is_finite() || w0 <= 0.0 || w1 <= 0.0 || w2 <= 0.0 {
            return Err(NurbsError::InvalidInput {
                msg: "conic_shape_invariance: invalid weights".into(),
            });
        }

        if self.is_rational() {
            if w1.abs() <= ON_TOL14 {
                return Err(NurbsError::NumericError {
                    msg: "conic_shape_invariance: w1 too small".into(),
                });
            }
            Ok((w0 * w2) / (w1 * w1))
        } else {
            Ok(1.0)
        }
    }

    ///   c0*x^2 + c1*y^2 + c2*x*y + c3*x + c4*y + c5 = 0
    pub fn conic_implicit_coeffs(&self) -> Result<[Real; 6]> {
        if !self.is_valid() {
            return Err(NurbsError::InvalidInput {
                msg: "conic_implicit_coeffs: invalid curve".into(),
            });
        }
        if self.degree != 2 {
            return Err(NurbsError::InvalidInput {
                msg: format!(
                    "conic_implicit_coeffs: degree must be 2 (got {})",
                    self.degree
                ),
            });
        }
        if self.ctrl.len() != 3 {
            return Err(NurbsError::InvalidInput {
                msg: format!(
                    "conic_implicit_coeffs: conic must have exactly 3 control points (got {})",
                    self.ctrl.len()
                ),
            });
        }

        // shape invariance
        let mut k = self.conic_shape_invariance()?;
        if k.abs() <= ON_TOL14 {
            return Err(NurbsError::NumericError {
                msg: "conic_implicit_coeffs: k too small".into(),
            });
        }

        // Euclidean control points
        let p0 = self.ctrl[0].from_w();
        let p1 = self.ctrl[1].from_w();
        let p2 = self.ctrl[2].from_w();

        // must be in XY plane
        let z_tol = ON_TOL12;
        if p0.z.abs() > z_tol || p1.z.abs() > z_tol || p2.z.abs() > z_tol {
            return Err(NurbsError::InvalidInput {
                msg: "conic_implicit_coeffs: conic must lie in XY plane (z != 0)".into(),
            });
        }

        let x0 = p0.x; let y0 = p0.y;
        let x1 = p1.x; let y1 = p1.y;
        let x2 = p2.x; let y2 = p2.y;

        // original constants
        let h0 = x0 - x1;
        let h1 = x2 - x1;
        let h2 = y0 - y1;
        let h3 = y2 - y1;
        let g1 = h2 * h1 - h0 * h3;
        let s1 = h0 - h1;
        let s2 = h3 - h2;

        // k = 1/k
        k = 1.0 / k;

        // coefficients
        let c0 = s2 * s2 + 4.0 * k * h2 * h3;
        let c1 = s1 * s1 + 4.0 * k * h0 * h1;
        let c2 = 2.0 * (s1 * s2 - 2.0 * k * (h0 * h3 + h1 * h2));
        let c3 = 2.0 * (g1 * s2 - c0 * x1 - c2 * y1);
        let c4 = 2.0 * (g1 * s1 - c1 * y1 - c2 * x1);
        let c5 = c0 * x1 * x1
            + c1 * y1 * y1
            + 2.0 * c2 * x1 * y1
            - 2.0 * g1 * (s1 * y1 + s2 * x1)
            + g1 * g1;

        Ok([c0, c1, c2, c3, c4, c5])
    }

    /// Convenience: evaluate implicit polynomial at (x,y)
    #[inline]
    pub fn eval_conic_implicit_xy(c: &[Real; 6], x: Real, y: Real) -> Real {
        c[0] * x * x + c[1] * y * y + c[2] * x * y + c[3] * x + c[4] * y + c[5]
    }
}
```
---


