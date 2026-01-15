# on_extract_crv_numerator_denominator
- on_extract_crv_numerator_denominator 함수의 수식, 개념 설명, 알고리즘, 테스트 의미를 모두 포함한 문서.
- Extracting the Numerator Curve and Denominator Function from a Rational NURBS Curve  
    이 문서는 NURBS 곡선에서 분자(Numerator) 곡선과 분모(Denominator) 함수를 분리하는 함수
- on_extract_crv_numerator_denominator 의 수학적 의미와 구현 목적을 설명한다.

## 1. 개요 (Overview)
- Rational NURBS 곡선은 다음과 같이 정의된다:
```math
C(u)=\frac{\sum _iN_{i,p}(u)\, w_i\, P_i}{\sum _iN_{i,p}(u)\, w_i}
```
- 여기서
    - $P_i=(x_i,y_i,z_i)$
    - $w_i$ 는 weight
    - $N_{i,p}(u)$ 는 B-spline basis function
- 즉, Rational NURBS는 분자 / 분모 구조를 가진다.
```math
C(u)=\frac{N(u)}{D(u)}
```
이 함수는 곡선을 다음 두 개의 객체로 분리한다:
- Numerator curve
```math
N(u)=\sum _iN_{i,p}(u)\, (x_iw_i,\, y_iw_i,\, z_iw_i)
```
- Denominator scalar function
```math
D(u)=\sum _iN_{i,p}(u)\, w_i
```
- 이 분리는 다음과 같은 알고리즘에서 매우 중요하다:
  - 곡선 미분 (rational derivative)
  - 곡률 계산
  - 곡선-곡선 교차
  - 곡선의 rational → polynomial 변환
  - 곡선의 수치적 안정성 분석

## 2. 수학적 배경 (Mathematical Formulation)
### 2.1 Rational NURBS Curve
```math
C(u)=\frac{\sum _iN_{i,p}(u)\, w_i\, P_i}{\sum _iN_{i,p}(u)\, w_i}
```
- 분자와 분모를 다음과 같이 정의한다:
    - Numerator curve
    ```math
    N(u)=\sum _iN_{i,p}(u)\, (x_iw_i,\, y_iw_i,\, z_iw_i)
    ```
    - Denominator function
    ```math
    D(u)=\sum _iN_{i,p}(u)\, w_i
    ```
- Rational curve reconstruction
```math
C(u)=\frac{N(u)}{D(u)}
```
## 3. 함수 설명 (Function Description)
```rust
pub fn on_extract_crv_numerator_denominator(cur: &NurbsCurve)
    -> Result<CurveNumeratorDenominator>
```

- 입력
    - cur: Rational 또는 Non-rational NURBS curve
- 출력
    - CurveNumeratorDenominator 구조체:
```rust
pub struct CurveNumeratorDenominator {
    pub num: NurbsCurve,   // numerator curve
    pub den: Option<CFun>, // denominator function (if rational)
}
```

- Rational curve일 때
  - num = weight가 모두 1인 NURBS curve
  - 제어점 = $(x_i,y_i,z_i,1)$
  - 실제 numerator는 내부적으로 basis × weight × xyz 로 계산됨
  - den = weight만을 제어점으로 갖는 scalar B-spline function
Non-rational curve일 때
    - num = 원래 curve와 동일
    - den = None

## 4. 알고리즘 (Algorithm)
- 곡선이 비어 있는지 검사
- 곡선이 rational인지 확인
- numerator curve control points 생성
- $(x_i, y_i, z_i, 1)$
- denominator control values 생성 (if rational)
- $(w_i)$
- numerator curve 구성
- denominator function 구성 (CFun)
- 결과 반환

## 5. 테스트 설명 (Unit Test Meaning)
- ✔ Test 1: Rational curve reconstruction
```rust
curnad_rational_reconstructs_curve_points
```

- 검증 내용:
    - Rational curve → numerator + denominator 로 분리
    - 모든 샘플 파라미터 u에 대해
    ```math
    C(u)\approx \frac{N(u)}{D(u)}
    ```
    - 즉, rational curve가 정확히 재구성되는지 확인한다.

- ✔ Test 2: Non-rational curve behavior
```rust
curnad_nonrational_returns_no_den_and_num_matches
```

- 검증 내용:
    - Non-rational curve는 weight가 모두 1
    - 따라서:
    ```math
    C(u)=N(u)
    ```
    - den = None 이어야 한다
    - numerator curve는 원래 curve와 동일해야 한다

## 6. 요약 (Summary)
| 항목 | 설명 |
|------|------|
| 목적 | Rational NURBS를 Numerator curve + Denominator function으로 분리 |
| Numerator | (xw, yw, zw) 형태의 비-rational NURBS curve |
| Denominator | weight만을 갖는 scalar B-spline function |
| Non-rational 처리 | numerator = 원래 curve, denominator = None |
| 형상 보존 | C(u) = N(u) / D(u) 관계가 항상 유지됨 |
| 활용 | 미분, 곡률, 교차, rational → polynomial 변환 등 |

--- 
## 소스 코드
```rust
pub fn on_extract_crv_numerator_denominator(cur: &NurbsCurve) 
-> Result<CurveNumeratorDenominator> {
    if cur.ctrl.is_empty() {
        return Err(NurbsError::InvalidArgument { msg: "empty curve control points".into() });
    }

    let is_rat = cur.is_rational();

    // numerator: (xw, yw, zw, 1)
    let mut num_ctrl = Vec::with_capacity(cur.ctrl.len());

    // denominator function control values: w
    let mut den_fu = if is_rat {
        Some(Vec::with_capacity(cur.ctrl.len()))
    } else {
        None
    };

    for cp in &cur.ctrl {
        num_ctrl.push(Point4D { x: cp.x, y: cp.y, z: cp.z, w: 1.0 });
        if let Some(ref mut fu) = den_fu {
            fu.push(cp.w);
        }
    }

    let num = NurbsCurve {
        dimension: cur.dimension,
        degree: cur.degree,
        ctrl: num_ctrl,
        kv: cur.kv.clone(),
        domain: cur.domain,
    };

    let den = if let Some(fu) = den_fu {
        Some(CFun::new(cur.degree, cur.kv.clone(), fu)?)
    } else {
        None
    };

    Ok(CurveNumeratorDenominator { num, den })
}
```
---
### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::nurbs_curve::NurbsCurve;
    use nurbslib::core::prelude::Degree;
    use nurbslib::core::basis::Side;
    use nurbslib::core::cfun::on_extract_crv_numerator_denominator;
    use nurbslib::core::geom::{Point4D};

    fn assert_near(a: f64, b: f64, eps: f64, msg: &str) {
        let d = (a - b).abs();
        assert!(d <= eps, "{}: |{}-{}|={}", msg, a, b, d);
    }
```
```rust
    #[test]
    fn ecnd_rational_reconstructs_curve_points() {
        // make a rational curve (weights not all 1)
        let p: Degree = 3;
        let ctrl = vec![
            Point4D { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            Point4D { x: 1.0, y: 2.0, z: 1.0, w: 0.5 },
            Point4D { x: 2.0, y: 0.5, z: 2.0, w: 2.0 },
            Point4D { x: 3.0, y: 1.5, z: -1.0, w: 1.2 },
            Point4D { x: 4.0, y: 0.0, z: 0.0, w: 0.8 },
            Point4D { x: 5.0, y: 1.0, z: 1.0, w: 1.0 },
        ];
        let mut cur = NurbsCurve::from_ctrl_clamped_uniform(p, ctrl);
        cur.dimension = 3;
        assert!(cur.is_rational());

        let nd = on_extract_crv_numerator_denominator(&cur).unwrap();
        assert!(nd.den.is_some());

        let den = nd.den.as_ref().unwrap();

        let us = cur.uniform_params(40);
        let eps = 1e-9;

        for &u in &us {
            let c = cur.point_at(u);

            // numerator homogeneous accumulation (xw,yw,zw,w=1 accumulated)
            let nh = nd.num.point_at_h(u);      // should be (Nx,Ny,Nz, 1*sum_basis) but we only need xyz
            let d = den.eval(u, Side::Left).unwrap();

            // reconstruct: (xw/d, yw/d, zw/d)
            let rx = nh.x / d;
            let ry = nh.y / d;
            let rz = nh.z / d;

            assert_near(rx, c.x, eps, "x");
            assert_near(ry, c.y, eps, "y");
            assert_near(rz, c.z, eps, "z");
        }
    }
```
```rust
    #[test]
    fn ecnd_nonrational_returns_no_den_and_num_matches() {
        let p: Degree = 2;
        let ctrl = vec![
            Point4D { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
            Point4D { x: 1.0, y: 2.0, z: 0.0, w: 1.0 },
            Point4D { x: 3.0, y: 1.0, z: 0.0, w: 1.0 },
            Point4D { x: 4.0, y: 0.0, z: 0.0, w: 1.0 },
        ];
        let mut cur = NurbsCurve::from_ctrl_clamped_uniform(p, ctrl);
        cur.dimension = 3;
        assert!(!cur.is_rational());

        let nd = on_extract_crv_numerator_denominator(&cur).unwrap();
        assert!(nd.den.is_none());

        let us = cur.uniform_params(25);
        let eps = 1e-12;

        for &u in &us {
            let c0 = cur.point_at(u);
            let c1 = nd.num.point_at(u);
            assert_near(c0.x, c1.x, eps, "x");
            assert_near(c0.y, c1.y, eps, "y");
            assert_near(c0.z, c1.z, eps, "z");
        }
    }
}
```
---




