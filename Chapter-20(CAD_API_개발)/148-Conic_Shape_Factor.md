## 🎯 compute_conic_shape_factor 의 역할
- NURBS conic(3개의 control point + weight)에서  
  그 conic이 **어떤 종류의 곡선인지** 결정하는 shape factor k 를 계산하는 함수.
- 이 k 값 하나로 conic의 종류가 결정됨.

## 📐 수학적 의미
- NURBS conic은 보통 3개의 control point와 3개의 weight로 표현돼:
- $P_0$, $P_1$, $P_2$
- $w_0$, $w_1$, $w_2$
- 이때 conic의 형태는 다음 불변량으로 결정된다:
```math
k=\frac{w_0w_2}{w_1^2}
```      
- 이 값이 바로 shape invariance factor.

## 📊 k 값에 따른 conic 종류

|   k 값   |      Conic 종류      |
|---------|------------------------|
|   k = 1 | 원(circle) / 타원(ellipse) |
|   k > 1 | 타원(ellipse)            |
|   k = 0 | 포물선(parabola)         |
| 0 < k < 1 | 쌍곡선(hyperbola)     |

- 즉, k는 conic의 타입을 결정하는 핵심 파라미터.

## 🧠 왜 weight로 conic 종류가 결정될까?
- NURBS conic은 다음과 같은 형태로 정의:
```math
C(u)=\frac{\sum _iB_i^2(u)w_iP_i}{\sum _iB_i^2(u)w_i}
```
- 여기서 weight 비율이 conic의 곡률과 형태를 결정한다.
- 특히 중간 weight $w_1$ 은 conic의 **굽힘 정도** 를 조절하는데, 양 끝 weight와의 비율이 conic의 종류를 결정하게 된다.
- 그래서:
```math
k=\frac{w_0w_2}{w_1^2}
```
- 이 값이 형태 불변량이 되는 것.

## 🛠 함수가 실제로 하는 일
- Rust 버전 기준으로 보면:
  - 곡선이 rational인지 확인
  - weight가 정상 범위인지 검사
  - 첫 3개의 control point weight를 읽음
- 다음 계산 수행:
```rust
k = (w0 * w2) / (w1 * w1)
```

- 그 값을 반환

## 📌 언제 쓰는 함수인가?
- conic이 원인지, 타원인지, 포물선인지, 쌍곡선인지 판정할 때
- conic을 재구성하거나 변환할 때
- conic의 기하학적 성질을 분석할 때
- CAD/NURBS 커널에서 conic validation 할 때
- 즉, NURBS conic의 타입을 판정하는 핵심 도구.

## 🎉 요약
- compute_conic_shape_factor는:
  - NURBS conic의 weight를 이용해
  - conic의 형태를 결정하는 불변량 k 를 계산하고
  - 그 값으로 conic의 종류(원/타원/포물선/쌍곡선)를 판정할 수 있게 해주는 함수

## Nurbs가 어떻게 정보를 가지는가 ?
- NURBS가 어떻게 원/타원/포물선/쌍곡선을 품고 있는지를 정리.

### 1. NURBS conic의 기본 구조
- 우리가 다루는 conic은 3개의 제어점과 3개의 weight로 표현되는 rational quadratic Bezier.
- 제어점:
```math
P_0,P_1,P_2\in \mathbb{R^{\mathnormal{2}}}\mathrm{\  또는\  }\mathbb{R^{\mathnormal{3}}}
```
- weight:
```math
w_0,w_1,w_2>0
```
- Bezier basis (degree 2):
```math
B_0(u)=(1-u)^2,\quad B_1(u)=2u(1-u),\quad B_2(u)=u^2,\quad u\in [0,1]
```
- Rational Bezier conic:
```math
C(u)=\frac{B_0(u)w_0P_0+B_1(u)w_1P_1+B_2(u)w_2P_2}{B_0(u)w_0+B_1(u)w_1+B_2(u)w_2}
```
- 이게 우리가 쓰는 NURBS conic의 기본 형태.

- 직관적으로 보면:
  - w_1 이 커지면 → 중간 제어점이 **덜 당겨짐** → 더 완만한 곡선 → ellipse 쪽
  - w_1 이 작아지면 → 중간 제어점이 **강하게 당겨짐** → hyperbola 쪽으로

### 2. 아주 간단한 도출 스케치
- conic은 일반적으로 2차 곡선:
```math
Ax^2+Bxy+Cy^2+Dx+Ey+F=0
```
- rational Bezier conic은 위 방정식을 만족하는 곡선의 한 부분이다.
- 제어점과 weight를 적절히 선택하면, 이 conic이 ellipse/parabola/hyperbola 중 하나가 된다.
- 이때, 중간 weight w_1 을 변화시키면 conic의 곡률과 형태가 바뀌는데,
- 그 변화가 끝 weight와의 비율로 정규화되면서 결국 다음 조합이 불변량이 된다:
```math
k=\frac{w_0w_2}{w_1^2}
```
- 이 k 값이 conic의 discriminant 역할을 하게 된다.

---

 ## 소스 코드
 ```rust
use crate::core::nurbs_curve::NurbsCurve;
use crate::core::prelude::NurbsError;
```

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConicType {
    CircleOrEllipse,
    Ellipse,
    Parabola,
    Hyperbola,
    Degenerate,
}
```

```rust
/// Check whether all weights of a NURBS curve are within [wmin, wmax].
/// Equivalent to C's E_curwei().
///
/// Returns:
/// - Ok(()) if all weights are valid
/// - Err(NurbsError::InvalidInput) if any weight is out of range
pub fn check_curve_weights(
    curve: &NurbsCurve,
    w_min: f64,
    w_max: f64,
    r_name: &str,   // name of calling routine (for debugging)
) -> Result<(), NurbsError> {
    // 1) Rational curve인지 확인
    //    (C의 U_iscurr(cur))
    if !curve.is_rational() {
        return Ok(()); // rational이 아니면 weight 검사 필요 없음
    }

    // 2) 모든 control point weight 검사
    for (i, cp) in curve.ctrl.iter().enumerate() {
        let w = cp.w;

        if w < w_min || w > w_max {
            return Err(NurbsError::InvalidInput {
                msg: format!(
                    "Weight out of range in {}: index={}, weight={}, allowed=[{}, {}]",
                    r_name, i, w, w_min, w_max
                ),
            });
        }
    }

    Ok(())
}
```
```rust
// Compute conic shape invariance factor k for a NURBS conic arc.
/// Equivalent to C's N_consha().
///
/// Assumes:
/// - curve is a single conic segment (degree 2 or rational quadratic)
/// - control points are in homogeneous form (Point4D)
///
/// Returns:
/// - Ok(k)  : shape invariance factor
/// - Err(..): weight error or invalid curve
pub fn compute_conic_shape_factor(
    curve: &NurbsCurve,
    wmin: f64,
    wmax: f64,
) -> Result<f64, NurbsError> {
    // 1) weight sanity check (C: E_curwei)
    check_curve_weights(curve, wmin, wmax, "compute_conic_shape_factor")?;

    // 2) default k = 1.0
    let mut k = 1.0;

    // 3) rational curve인지 확인 (C: U_iscurr)
    if curve.is_rational() {
        // conic은 반드시 3개의 CP (degree=2) 또는
        // 최소한 첫 3개 CP가 conic 정의에 사용됨
        if curve.ctrl.len() < 3 {
            return Err(NurbsError::InvalidInput {
                msg: "Conic curve must have at least 3 control points".into(),
            });
        }

        let w0 = curve.ctrl[0].w;
        let w1 = curve.ctrl[1].w;
        let w2 = curve.ctrl[2].w;

        // C 코드: k = (w0 * w2) / (w1 * w1)
        if w1.abs() < 1e-14 {
            return Err(NurbsError::InvalidInput {
                msg: "Invalid conic: middle weight is zero".into(),
            });
        }

        k = (w0 * w2) / (w1 * w1);
    }

    Ok(k)
}
```
```rust
pub fn classify_conic_by_k(k: f64, tol: f64) -> ConicType {
    if !k.is_finite() {
        return ConicType::Degenerate;
    }

    if (k - 1.0).abs() <= tol {
        ConicType::CircleOrEllipse
    } else if k > 1.0 {
        ConicType::Ellipse
    } else if k.abs() <= tol {
        ConicType::Parabola
    } else if k > 0.0 && k < 1.0 {
        ConicType::Hyperbola
    } else {
        ConicType::Degenerate
    }
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::check::{classify_conic_by_k, compute_conic_shape_factor, ConicType};
    use nurbslib::core::prelude::{Interval, KnotVector, NurbsCurve, Point4D};

    fn make_simple_conic(w0: f64, w1: f64, w2: f64) -> NurbsCurve {
        // 단순 2D conic: P0=(-1,0), P1=(0,1), P2=(1,0)
        let p0 = Point4D::homogeneous(-1.0, 0.0, 0.0, w0);
        let p1 = Point4D::homogeneous( 0.0, 1.0, 0.0, w1);
        let p2 = Point4D::homogeneous( 1.0, 0.0, 0.0, w2);

        let kv = KnotVector::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]).unwrap();

        NurbsCurve {
            dimension: 3,
            degree: 2,
            ctrl: vec![p0, p1, p2],
            kv,
            domain: Interval { t0: 0.0, t1: 1.0 },
        }
    }
```
```rust
    #[test]
    fn test_conic_shape_factor_and_classification() {
        let wmin = 1e-6;
        let wmax = 1e6;
        let tol  = 1e-6;

        // 1) k = 1 → circle/ellipse
        let c1 = make_simple_conic(1.0, 1.0, 1.0);
        let k1 = compute_conic_shape_factor(&c1, wmin, wmax).unwrap();
        let t1 = classify_conic_by_k(k1, tol);
        println!("k1 = {}, type = {:?}", k1, t1);
        assert!( (k1 - 1.0).abs() < tol );
        assert!( matches!(t1, ConicType::CircleOrEllipse | ConicType::Ellipse) );

        // 2) k > 1 → ellipse
        let c2 = make_simple_conic(2.0, 1.0, 2.0); // k = (2*2)/(1*1) = 4
        let k2 = compute_conic_shape_factor(&c2, wmin, wmax).unwrap();
        let t2 = classify_conic_by_k(k2, tol);
        println!("k2 = {}, type = {:?}", k2, t2);
        assert!(k2 > 1.0);
        assert!(matches!(t2, ConicType::Ellipse));

        // 3) 0 < k < 1 → hyperbola
        let c3 = make_simple_conic(1.0, 2.0, 1.0); // k = (1*1)/(2*2) = 0.25
        let k3 = compute_conic_shape_factor(&c3, wmin, wmax).unwrap();
        let t3 = classify_conic_by_k(k3, tol);
        println!("k3 = {}, type = {:?}", k3, t3);
        assert!(k3 > 0.0 && k3 < 1.0);
        assert!(matches!(t3, ConicType::Hyperbola));

        // 4) k ≈ 0 → parabola (w1 → ∞에 가까운 상황을 흉내)
        let c4 = make_simple_conic(1.0, 1e3, 1.0); // k ≈ (1*1)/(1e3^2) ≈ 1e-6
        let k4 = compute_conic_shape_factor(&c4, wmin, wmax).unwrap();
        let t4 = classify_conic_by_k(k4, tol);
        println!("k4 = {}, type = {:?}", k4, t4);
        assert!(k4 < 1e-3);
        assert!(matches!(t4, ConicType::Parabola | ConicType::Degenerate));
    }
}
```
---

