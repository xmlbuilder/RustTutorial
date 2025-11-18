# Enterpolation

다양한 곡선 보간(interpolation) 방식들을 추상화한 인터페이스를 제공하며, Bezier, B-Spline, Linear 보간을 Interpolatable 타입에 대해 구현하고 있습니다.    
아래에 코드를 정리하고, 각 보간 방식의 수학적 원리를 설명.

## ✅ 코드 구조 정리

```rust
use crate::core::geom::Point2D;
use crate::core::prelude::{Point3D, Vector3D};
use enterpolation::Curve;
use enterpolation::bezier::Bezier;
use enterpolation::bspline::BSpline;
use enterpolation::linear::Linear;
use std::fmt::Debug;
use std::ops::{Add, Mul};
```
```rust
trait Interpolatable: Clone + Copy + Default + Add<Output = Self> + Mul<f64, Output = Self> {}
impl Interpolatable for Point2D {}
impl Interpolatable for Point3D {}
impl Interpolatable for Vector3D {}
```
```rust
pub fn on_bezier_interpolate<T: Interpolatable>(elements: &[T]) -> impl Curve<f64, Output = T> {
    Bezier::builder()
        .elements(elements)
        .normalized::<f64>()
        .dynamic()
        .build()
        .expect("Failed to build Bezier curve")
}
```
```rust
pub fn bspline_deg2<T: Interpolatable>(
    elements: &[T],
    knots: &[f64],
) -> impl Curve<f64, Output = T> {
    BSpline::builder()
        .elements(elements)
        .knots(knots)
        .constant::<2>()
        .build()
        .expect("Failed to build cubic BSpline")
}
```
```rust
pub fn bspline_deg3<T: Interpolatable>(
    elements: &[T],
    knots: &[f64],
) -> impl Curve<f64, Output = T> {
    BSpline::builder()
        .elements(elements)
        .knots(knots)
        .constant::<3>()
        .build()
        .expect("Failed to build cubic BSpline")
}
```
```rust
pub fn bspline_deg4<T: Interpolatable>(
    elements: &[T],
    knots: &[f64],
) -> impl Curve<f64, Output = T> {
    BSpline::builder()
        .elements(elements)
        .knots(knots)
        .constant::<4>()
        .build()
        .expect("Failed to build cubic BSpline")
}
```
```rust
pub fn bspline_deg5<T: Interpolatable>(
    elements: &[T],
    knots: &[f64],
) -> impl Curve<f64, Output = T> {
    BSpline::builder()
        .elements(elements)
        .knots(knots)
        .constant::<5>()
        .build()
        .expect("Failed to build cubic BSpline")
}
```
```rust
pub fn on_linear_interpolate<T: Interpolatable + Debug>(
    elements: &[T],
    knots: &[f64],
) -> impl Curve<f64, Output = T> {
    Linear::builder()
        .elements(elements)
        .knots(knots)
        .build()
        .expect("Failed to build Linear interpolation")
}
```
## 📐 수학적 설명
### 1️⃣ Bezier Curve
- 정의:

$$
B(t)=\sum _{i=0}^nB_i^n(t)\cdot P_i
$$

- 베르스타인 기저함수:

$$
B_i^n(t)={n \choose i}(1-t)^{n-i}t^i
$$

- 특징:
    - 제어점 P_i에 의해 곡선이 형성됨
    - $t\in [0,1]$ 에서 정의됨
    - 전체 곡선은 제어점의 볼록 껍질(convex hull) 안에 존재

### 2️⃣ B-Spline Curve
- 정의:

$$
  S(t)=\sum _{i=0}^nN_{i,k}(t)\cdot P_i
$$
  
- $N_{i,k}(t)$: B-Spline 기저함수 (degree k)
  
- 특징:
  - knots 배열에 따라 곡선의 형태가 결정됨
  - 국소 제어 가능 (local control)
  - 연속성과 부드러움 조절 가능 (degree에 따라 C^k 연속)

### 3️⃣ Linear Interpolation
- 정의:

$$
L(t)=(1-r)\cdot P_i+r\cdot P_{i+1}, where r=\frac{t-t_i}{t_{i+1}-t_i}
$$

- 특징:
  - 두 점 사이를 직선으로 연결
  - 가장 단순하고 빠른 보간 방식
  - 곡선이 아닌 선분으로 구성됨

## 🧠 요약 비교
| 보간 방식     | 수식 표현                         | 특징 요약                     |
|---------------|-----------------------------------|-------------------------------|
| Bezier        | ∑ Bᵢⁿ(t) · Pᵢ                     | 전체 제어점에 의해 곡선 결정 |
| B-Spline      | ∑ Nᵢ,ₖ(t) · Pᵢ                    | 국소 제어, 연속성 조절 가능  |
| Linear        | (1 - r) · Pᵢ + r · Pᵢ₊₁           | 두 점 사이 직선 보간         |

---

# ✅ 테스트 코드 범주별 정리

## 1️⃣ 기본 보간 함수 테스트
| 테스트 함수 이름             | 실행 조건 또는 설명             |
|------------------------------|----------------------------------|
| test_linear_interpolate      | on_linear_interpolate            |
| test_bezier_interpolate      | on_bezier_interpolate            |
| test_bspline_deg2            | on_bspline_deg2_interpolate      |
| test_bspline_deg5            | on_bspline_deg5_interpolate      |



## 2️⃣ Bezier 고급 기능 테스트
| 테스트 항목            | 설명 또는 테스트 목적                          |
|------------------------|-----------------------------------------------|
| elements_with_weights  | 가중치 기반 보간이 정확히 적용되는지 확인       |
| bezier_errors          | Bezier 보간의 오차 및 경계 조건 테스트          |
| extrapolation          | 보간 범위를 벗어난 외삽 결과의 안정성 확인      |
| constant               | constant() 설정 시 곡선이 고정되는지 테스트     |
| deriatives             | 도함수 및 접선 계산이 정확히 수행되는지 확인    |
| partial_eq             | 보간 객체 간의 동등성 비교가 올바르게 작동하는지 |


## 3️⃣ Stepper 기능 테스트
| 테스트 항목 | 설명 또는 테스트 목적                          |
|-------------|-----------------------------------------------|
| stepper     | 등간격 또는 사용자 정의 구간에서의 샘플링 정확성 확인 |


## 4️⃣ B-Spline 수치 검증 테스트
| 테스트 항목                   | 설명 또는 테스트 목적                          |
|------------------------------|-----------------------------------------------|
| linear_bspline               | degree 1 B-Spline의 선형 보간 정확성 확인       |
| quadratic_bspline            | degree 2 B-Spline의 곡선 형태 및 연속성 검증    |
| cubic_bspline                | degree 3 B-Spline의 부드러운 곡선 및 도함수 확인 |
| quartic_bspline              | degree 4 B-Spline의 고차 보간 안정성 테스트     |
| quartic_bspline_f64          | f64 기반 고정밀 quartic B-Spline 테스트        |



## 5️⃣ Linear 보간 고급 테스트
| 테스트 항목         | 설명 또는 테스트 목적                             |
|----------------------|--------------------------------------------------|
| linear_equidistant   | 등간격 knot 기반 선형 보간 정확성 확인             |
| linear               | 일반적인 선형 보간 동작 테스트                     |
| extrapolation2       | 보간 범위 외의 extrapolation 동작 검증             |
| weights              | 가중치 기반 선형 보간이 정확히 적용되는지 확인     |
| const_creation       | ConstEquidistantLinear 생성 및 동작 테스트         |
| borrow_creation      | borrow된 데이터로 보간 객체 생성 테스트            |
| partial_eq2          | 선형 보간 객체 간의 동등성 비교 테스트             |


## 📐 테스트 코드 설명 및 수학적 의미
### 🔹 Bezier 보간
- Bezier 곡선은 제어점과 베르스타인 기저함수로 구성됨
- 테스트는 곡선의 평가, 도함수, 외삽, 상수 곡선, 가중치 적용 등 다양한 기능을 검증
### 🔹 B-Spline 보간
- B-Spline은 knot 벡터와 degree에 따라 곡선이 결정됨
- 테스트는 다양한 degree에 대해 곡선의 형태와 수치적 정확성을 검증
### 🔹 Linear 보간
- 선형 보간은 두 점 사이를 직선으로 연결
- 테스트는 등간격, 사용자 지정 knot, 외삽, 가중치 적용 등 다양한 시나리오를 포함
### 🔹 Stepper
- Stepper는 일정한 간격으로 값을 생성하는 유틸리티
- 테스트는 정규화된 구간과 사용자 지정 구간에서 정확한 샘플링을 검증

## ✅ 결론
- 테스트 코드는 보간 알고리즘의 정확성, 안정성, 예외 처리, 외삽, 도함수 계산 등 다양한 측면을 포괄적으로 검증합니다.
- 수학적으로도 Bezier, B-Spline, Linear 보간은 모두 정당한 방식으로 구현되어 있으며, 테스트는 그 수치적 신뢰성을 보장합니다.

```rust
#[cfg(test)]
mod encapsulation_tests {
    use assert_float_eq::assert_float_absolute_eq;
    use enterpolation::bezier::{Bezier, BezierBuilder, BezierDirector};
    use enterpolation::bspline::BSpline;
    use enterpolation::{Signal, Stepper};
    use nurbslib::core::enterpolator::{
        bspline_deg2, bspline_deg3, bspline_deg4, bspline_deg5, on_bezier_interpolate,
        on_linear_interpolate,
    };
    use nurbslib::core::prelude::Point3D;
    use std::iter;

    #[test]
    fn test_linear_interpolate() {
        let elements = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 1.0,
                y: 2.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            Point3D {
                x: 3.0,
                y: 1.0,
                z: 2.0,
            },
        ];

        let knots = vec![0.0, 0.3, 0.6, 1.0];
        let t = 0.5;

        // 사용 예
        let curve = on_linear_interpolate(&elements, &knots);
        println!("Linear: {:?}", curve.eval(t));
        println!("Linear: {:?}", curve.eval(0.0));
        println!("Linear: {:?}", curve.eval(1.0));
    }
```
```rust
    #[test]
    fn test_bspline_deg2() {
        let elements = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 1.0,
                y: 2.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            Point3D {
                x: 3.0,
                y: 1.0,
                z: 2.0,
            },
        ];

        let knots = vec![0.0, 0.3, 0.6, 1.0];
        let t = 0.5;
        let curve = bspline_deg2(&elements, &knots);
        println!("BSpline Deg2: {:?}", curve.eval(t));
        println!("BSpline Deg2: {:?}", curve.eval(0.0));
        println!("BSpline Deg2: {:?}", curve.eval(1.0));
    }
```
```rust
    #[test]
    fn test_bspline_deg3() {
        let elements = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 1.0,
                y: 2.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            Point3D {
                x: 3.0,
                y: 1.0,
                z: 2.0,
            },
        ];

        let knots = vec![0.0, 0.3, 0.6, 1.0];
        let t = 0.5;
        let curve = bspline_deg3(&elements, &knots);
        println!("BSpline Deg3: {:?}", curve.eval(t));
        println!("BSpline Deg3: {:?}", curve.eval(0.0));
        println!("BSpline Deg3: {:?}", curve.eval(1.0));
    }
```
```rust
    #[test]
    fn test_bspline_deg4() {
        let elements = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 1.0,
                y: 2.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            Point3D {
                x: 3.0,
                y: 1.0,
                z: 2.0,
            },
        ];

        let knots = vec![0.0, 0.3, 0.6, 1.0];
        let t = 0.5;
        let curve = bspline_deg4(&elements, &knots);
        println!("BSpline Deg3: {:?}", curve.eval(t));
        println!("BSpline Deg3: {:?}", curve.eval(0.0));
        println!("BSpline Deg3: {:?}", curve.eval(1.0));
    }
```
```rust
    #[test]
    fn test_bspline_deg5() {
        let elements = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 1.0,
                y: 2.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            Point3D {
                x: 3.0,
                y: 1.0,
                z: 2.0,
            },
        ];

        let knots = vec![0.0, 0.3, 0.6, 1.0];
        let t = 0.5;
        let curve = bspline_deg5(&elements, &knots);
        println!("BSpline Deg5: {:?}", curve.eval(t));
        println!("BSpline Deg5: {:?}", curve.eval(0.0));
        println!("BSpline Deg5: {:?}", curve.eval(1.0));
    }
```
```rust
    #[test]
    fn test_bezier_interpolate() {
        let elements = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 1.0,
                y: 2.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
            Point3D {
                x: 3.0,
                y: 1.0,
                z: 2.0,
            },
        ];
        let t = 0.5;
        let curve = on_bezier_interpolate(&elements);
        println!("Bezier: {:?}", curve.eval(t));
        println!("Bezier: {:?}", curve.eval(0.0));
        println!("Bezier: {:?}", curve.eval(1.0));
    }

    fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }
```
```rust
    #[test]
    fn elements_with_weights() {
        let a = BezierBuilder::new()
            .elements_with_weights([(1.0, 1.0), (2.0, 2.0), (3.0, 0.0)])
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();
        println!("{:?}", a.sample(iter::once(0.5)));

        BezierBuilder::new()
            .elements_with_weights([1.0, 2.0, 3.0].stack([1.0, 2.0, 0.0]))
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();

        BezierBuilder::new()
            .elements_with_weights([
                Homogeneous::new(1.0),
                Homogeneous::weighted_unchecked(2.0, 2.0),
                Homogeneous::infinity(3.0),
            ])
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();

        BezierBuilder::new()
            .elements([0.1, 0.2, 0.3])
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();

        assert!(
            BezierBuilder::new()
                .elements::<[f64; 0]>([])
                .normalized::<f64>()
                .constant()
                .build()
                .is_err()
        );
    }
```
```rust
    #[test]
    fn bezier_errors() {
        assert!(BezierDirector::new().elements::<[f32; 0]>([]).is_err());
        assert!(BezierDirector::new().elements([1.0]).is_ok());
    }
```
```rust
    #[test]
    fn extrapolation() {
        let bez = Bezier::builder()
            .elements([20.0, 0.0, 200.0])
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();
        assert_eq!(bez.eval(2.0), 820.0);
        assert_eq!(bez.eval(-1.0), 280.0);
    }
```
```rust
    #[test]
    fn constant() {
        let bez = Bezier::builder()
            .elements([5.0])
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();
        let res = bez.gen_with_tangent(0.25);
        assert_eq!(res[0], 5.0);
        assert_eq!(res[1], 0.0);
        let res = bez.gen_with_tangent(0.75);
        assert_eq!(res[0], 5.0);
        assert_eq!(res[1], 0.0);
    }
```
```rust
    #[test]
    fn deriatives() {
        let bez = Bezier::builder()
            .elements([1.0, 2.0, 3.0])
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();
        let res = bez.gen_with_tangent(0.5);
        assert_eq!(res[0], 2.0);
        assert_eq!(res[1], 2.0);
        let res = bez.gen_with_deriatives::<3>(0.5);
        assert_eq!(res[0], 2.0);
        assert_eq!(res[1], 2.0);
        assert_eq!(res[2], 0.0);
        // check if asking of to many derivatives does not panic
        let res = bez.gen_with_deriatives::<5>(0.5);
        assert_eq!(res[0], 2.0);
        assert_eq!(res[1], 2.0);
        assert_eq!(res[2], 0.0);
        assert_eq!(res[3], 0.0);
        assert_float_absolute_eq!(res[4], 0.0, ON_ZERO_TOL);
    }
```
```rust
    #[test]
    fn partial_eq() {
        let bez = Bezier::builder()
            .elements([20.0, 0.0, 200.0])
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();
        let bez2 = Bezier::builder()
            .elements([20.0, 0.0, 200.0])
            .normalized::<f64>()
            .constant()
            .build()
            .unwrap();
        assert_eq!(bez, bez2);
    }
```
```rust
    #[test]
    fn stepper() {
        let mut stepper = Stepper::<f64>::normalized(11);
        let res = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        for val in res {
            assert!(approx_equal(val, stepper.next().unwrap(), ON_ZERO_TOL));
        }

        let mut stepper = Stepper::new(5, 3.0, 5.0);
        let res = [3.0, 3.5, 4.0, 4.5, 5.0];
        for val in res {
            assert!(approx_equal(val, stepper.next().unwrap(), ON_ZERO_TOL));
        }
    }
```
```rust
    #[test]
    fn linear_bspline() {
        let expect = [
            (-1.0, -1.0),
            (0.0, 0.0),
            (0.2, 0.2),
            (0.4, 0.4),
            (0.6, 0.6),
            (0.8, 0.8),
            (1.0, 1.0),
            (2.0, 2.0),
        ];
        let points = [0.0f32, 1.0];
        let knots = [0.0f32, 1.0];
        let spline = BSpline::builder()
            .elements(points)
            .knots(knots)
            .constant::<2>()
            .build()
            .unwrap();
        for (input, output) in expect {
            assert_float_absolute_eq!(spline.eval(input), output);
        }
    }
```
```rust
    #[test]
    fn quadratic_bspline() {
        let expect = [
            (0.0, 0.0),
            (0.5, 0.125),
            (1.0, 0.5),
            (1.4, 0.74),
            (1.5, 0.75),
            (1.6, 0.74),
            (2.0, 0.5),
            (2.5, 0.125),
            (3.0, 0.0),
        ];
        let points = [0.0f32, 0.0, 1.0, 0.0, 0.0];
        let knots = [0.0f32, 0.0, 1.0, 2.0, 3.0, 3.0];
        let spline = BSpline::builder()
            .elements(points)
            .knots(knots)
            .constant::<3>()
            .build()
            .unwrap();
        for (input, output) in expect {
            assert_float_absolute_eq!(spline.eval(input), output);
        }
    }
```
```rust
    #[test]
    fn cubic_bspline() {
        let expect = [
            (-2.0, 0.0),
            (-1.5, 0.125),
            (-1.0, 1.0),
            (-0.6, 2.488),
            (0.0, 4.0),
            (0.5, 2.875),
            (1.5, 0.12500001),
            (2.0, 0.0),
        ];
        let points = [0.0f32, 0.0, 0.0, 6.0, 0.0, 0.0, 0.0];
        let knots = [-2.0f32, -2.0, -2.0, -1.0, 0.0, 1.0, 2.0, 2.0, 2.0];
        let spline = BSpline::builder()
            .elements(points)
            .knots(knots)
            .constant::<4>()
            .build()
            .unwrap();
        for (input, output) in expect {
            assert_float_absolute_eq!(spline.eval(input), output);
        }
    }
```
```rust
    #[test]
    fn quartic_bspline() {
        let expect = [
            (0.0, 0.0),
            (0.4, 0.0010666668),
            (1.0, 0.041666668),
            (1.5, 0.19791667),
            (2.0, 0.4583333),
            (2.5, 0.5989583),
            (3.0, 0.4583333),
            (3.2, 0.35206667),
            (4.1, 0.02733751),
            (4.5, 0.002604167),
            (5.0, 0.0),
        ];
        let points = [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let knots = [0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0];
        let spline = BSpline::builder()
            .elements(points)
            .knots(knots)
            .constant::<5>()
            .build()
            .unwrap();
        for (input, output) in expect {
            assert_float_absolute_eq!(spline.eval(input), output);
        }
    }
```
```rust
    #[test]
    fn quartic_bspline_f64() {
        let expect = [
            (0.0, 0.0),
            (0.4, 0.001066666666666667),
            (1.0, 0.041666666666666664),
            (1.5, 0.19791666666666666),
            (2.0, 0.45833333333333337),
            (2.5, 0.5989583333333334),
            (3.0, 0.4583333333333333),
            (3.2, 0.3520666666666666),
            (4.1, 0.027337500000000046),
            (4.5, 0.002604166666666666),
            (5.0, 0.0),
        ];
        let points = [0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let knots = [0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 5.0, 5.0];
        let spline = BSpline::builder()
            .elements(points)
            .knots(knots)
            .constant::<5>()
            .build()
            .unwrap();
        for (input, output) in expect {
            assert_float_absolute_eq!(spline.eval(input), output, ON_ZERO_TOL);
        }
    }
    use enterpolation::Curve;
    use enterpolation::linear::{ConstEquidistantLinear, Linear};
    use enterpolation::weights::Homogeneous;
    use nurbslib::core::types::ON_ZERO_TOL;
```
```rust
    #[test]
    fn linear_equidistant() {
        let lin = Linear::builder()
            .elements([20.0, 100.0, 0.0, 200.0])
            .equidistant::<f64>()
            .normalized()
            .build()
            .unwrap();
        let expected = [20.0, 60.0, 100.0, 50.0, 0.0, 100.0, 200.0];
        let mut iter = lin.take(expected.len());
        for val in expected {
            assert_float_absolute_eq!(val, iter.next().unwrap());
        }
    }
```
```rust
    #[test]
    fn linear() {
        //DynamicLinear
        let lin = Linear::builder()
            .elements([20.0, 100.0, 0.0, 200.0])
            .knots([0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0])
            .build()
            .unwrap();
        let expected = [20.0, 60.0, 100.0, 50.0, 0.0, 100.0, 200.0];
        let mut iter = lin.take(expected.len());
        for val in expected {
            assert_float_absolute_eq!(val, iter.next().unwrap());
        }
    }
```
```rust
    #[test]
    fn extrapolation2() {
        let lin = Linear::builder()
            .elements([20.0, 100.0, 0.0, 200.0])
            .knots([1.0, 2.0, 3.0, 4.0])
            .build()
            .unwrap();
        assert_float_absolute_eq!(lin.eval(1.5), 60.0);
        assert_float_absolute_eq!(lin.eval(2.5), 50.0);
        assert_float_absolute_eq!(lin.eval(-1.0), -140.0);
        assert_float_absolute_eq!(lin.eval(5.0), 400.0);
    }
```
```rust
    #[test]
    fn weights() {
        let lin = Linear::builder()
            .elements_with_weights([(0.0, 9.0), (1.0, 1.0)])
            .equidistant::<f64>()
            .normalized()
            .build()
            .unwrap();
        assert_float_absolute_eq!(lin.eval(0.5), 0.1);
        // const LIN : Linear<f64,f64,ConstEquidistant<f64>,CollectionWrapper<[f64;4],f64>> = Linear::new_equidistant_unchecked([20.0,100.0,0.0,200.0]);
    }
```
```rust
    #[test]
    fn const_creation() {
        const LIN: ConstEquidistantLinear<f64, f64, 4> =
            ConstEquidistantLinear::equidistant_unchecked([20.0, 100.0, 0.0, 200.0]);
        // const LIN : Linear<f64,f64,ConstEquidistant<f64>,CollectionWrapper<[f64;4],f64>> = Linear::new_equidistant_unchecked([20.0,100.0,0.0,200.0]);
        let expected = [20.0, 60.0, 100.0, 50.0, 0.0, 100.0, 200.0];
        let mut iter = LIN.take(expected.len());
        for val in expected {
            assert_float_absolute_eq!(val, iter.next().unwrap());
        }
    }
```
```rust
    #[test]
    fn borrow_creation() {
        let elements = [20.0, 100.0, 0.0, 200.0];
        let knots = [0.0, 1.0, 2.0, 3.0];
        let samples = [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0];
        let linear = Linear::builder()
            .elements(&elements)
            .knots(&knots)
            .build()
            .unwrap();
        let expected = [20.0, 60.0, 100.0, 50.0, 0.0, 100.0, 200.0];
        let mut iter = linear.sample(samples);
        for val in expected {
            assert_float_absolute_eq!(val, iter.next().unwrap());
        }
    }
```
```rust
    #[test]
    fn partial_eq2() {
        let linear = Linear::builder()
            .elements([20.0, 100.0, 0.0, 200.0])
            .knots([0.0, 1.0, 2.0, 3.0])
            .build()
            .unwrap();
        let linear2 = Linear::builder()
            .elements([20.0, 100.0, 0.0, 200.0])
            .knots([0.0, 1.0, 2.0, 3.0])
            .build()
            .unwrap();
        assert_eq!(linear, linear2);
    }
}
```
---

# 추가적인 enterpolation 기능

## 🧪 샘플링 관련 기능 설명
### 1️⃣ take(n)
- 설명: 보간된 곡선에서 등간격으로 n개의 샘플을 생성합니다.
- 사용 대상: Linear, Bezier, BSpline 등 모든 Curve 타입
- 내부적으로 Stepper::normalized(n)을 사용하여 [0, 1] 구간을 n등분
#### 예제:
```rust
let curve = Linear::builder()
    .elements([0.0, 1.0])
    .normalized()
    .build()
    .unwrap();

let samples: Vec<_> = curve.take(5).collect();
// 결과: [0.0, 0.25, 0.5, 0.75, 1.0]에 해당하는 보간 값
```


### 2️⃣ sample(&[t₁, t₂, …])
- 설명: 지정된 위치 t 값들에 대해 곡선을 평가합니다.
- 장점: 불규칙한 위치에서도 샘플링 가능
#### 예제:
```rust
let curve = Bezier::builder()
    .elements([0.0, 1.0, 0.0])
    .normalized()
    .build()
    .unwrap();

let positions = [0.0, 0.25, 0.5, 0.75, 1.0];
let samples: Vec<_> = curve.sample(&positions).collect();
// 결과: 해당 t 값에서의 Bezier 곡선 값
```


### 3️⃣ Stepper::new(n, start, end)
- 설명: 사용자 지정 구간 [start, end]를 n등분하여 샘플링 위치 생성
- 사용 예: 외삽 포함한 구간에서 일정한 간격으로 평가할 때
#### 예제:
```rust
let mut stepper = Stepper::new(5, 2.0, 4.0);
for t in stepper {
    println!("t = {}", t); // 출력: 2.0, 2.5, 3.0, 3.5, 4.0
}
```


### 4️⃣ Stepper::normalized(n)
- 설명: [0.0, 1.0] 구간을 n등분하여 샘플링 위치 생성
- 자주 사용되는 기본 샘플링 방식
#### 예제:
```rust
let mut stepper = Stepper::normalized(4);
for t in stepper {
    println!("t = {}", t); // 출력: 0.0, 0.333..., 0.666..., 1.0
}
```

## 📌 요약 비교
| 기능 이름             | 설명                                | 샘플링 위치       | 사용 예시                    |
|-----------------------|-------------------------------------|-------------------|------------------------------|
| take(n)               | 등간격으로 n개의 샘플 생성           | [0.0, 1.0] 구간    | 빠른 시각화, 테스트용         |
| sample(&[t])          | 지정된 위치에서 샘플링               | 사용자 지정       | 정밀 평가, 분석용             |
| Stepper::new(n, a, b) | 사용자 지정 구간을 n등분하여 샘플링  | [a, b] 구간       | 외삽 포함한 평가              |
| Stepper::normalized   | [0.0, 1.0] 구간을 n등분하여 샘플링   | 정규화된 구간     | 일반적인 보간 샘플링          |

---



