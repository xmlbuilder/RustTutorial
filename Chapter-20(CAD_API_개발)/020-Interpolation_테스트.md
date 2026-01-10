# Interpolation 테스트

## 🧪 테스트 함수 요약

| 테스트 이름                     | 대상 모듈 / 기능                     | 설명                                      |
|----------------------------------|--------------------------------------|-------------------------------------------|
| `data_interp_d`                 | `DataInterpolatord`                 | f64 기반 선형 보간 테스트                 |
| `data_interp_f`                 | `DataInterpolatorf`                 | f32 기반 선형 보간 테스트                 |
| `scalar_interpolator`          | `Interpolator`                      | 스칼라 시퀀스 보간 (linear, cubic)        |
| `point_interp`                 | `PointInterpolator`                 | 2D/3D 포인트 보간                         |
| `cc_polynomials`               | `Integrator`                        | Clenshaw-Curtis 다항식 적분 정확도 검증   |
| `cc_against_gauss_legendre_exp`| `Integrator`                        | CC vs Gauss-Legendre 비교                 |
| `cc_abs_nonsmooth_fixed_n`     | `Integrator`                        | 비매끈 함수 적분 (고정 N)                 |
| `cc_abs_nonsmooth_adaptive`    | `Integrator`                        | 비매끈 함수 적분 (적응형 N)               |
| `easing_functions_test`        | `InterpolatorEase`                  | 다양한 이징 함수의 값 검증                |

## 🎯 back_out(t) 함수의 원리
```rust
pub fn back_out(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let s = 1.70158;
    let p = t - 1.0;
    p * p * ((s + 1.0) * p + s) + 1.0
}
```

## 📐 수식

```math
f(t)=(t-1)^2\cdot ((s+1)(t-1)+s)+1\quad \mathrm{where}\quad s=1.70158
```

- $t ∈ [0, 1]$ 일 때도 결과값은 1.0을 초과할 수 있음
- s는 overshoot 강도를 조절하는 상수
- t = 0.5일 때 f(t) ≈ 1.0876975는 정상


## 📌 Easing 함수 결과 범위 요약

| 함수 이름      | 설명                         | 예상 결과 범위 |
|----------------|------------------------------|----------------|
| `back_out(t)`  | Overshoot 후 안정화 효과     | 0.0 ~ 1.1      |


## 테스트 함수
```rust
#[cfg(test)]
mod tests {

    use nurbslib::core::geom::Point2;
    use nurbslib::core::integrator::Integrator;
    use nurbslib::core::interpolator::{DataInterpolatord, DataInterpolatorf, Interpolator,
        InterpolatorEase, PointInterpolator};
    use nurbslib::core::prelude::Point;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }
```
```rust
    #[test]
    fn data_interp_d() {
        let mut di = DataInterpolatord::new();
        di.add_source(0.0, 0.0);
        di.add_source(2.0, 4.0);
        di.add_source(1.0, 1.0);
        assert!(approx(di.get_value(-1.0), 0.0));
        assert!(approx(di.get_value(0.5), 0.5));
        assert!(approx(di.get_value(1.0), 1.0));
        assert!(approx(di.get_value(1.5), 2.5));
        assert!(approx(di.get_value(3.0), 4.0));
    }
```
```rust
    #[test]
    fn data_interp_f() {
        let mut di = DataInterpolatorf::new();
        di.add_source(0.0, 0.0);
        di.add_source(2.0, 4.0);
        di.add_source(1.0, 1.0);
        assert!((di.get_value(-1.0) - 0.0).abs() < 1e-6);
        assert!((di.get_value(1.5) - 2.5).abs() < 1e-6);
    }
```
```rust
    #[test]
    fn scalar_interpolator() {
        let itp = Interpolator::new(vec![0.0, 10.0, 20.0, 10.0], false);
        assert!(approx(itp.linear(0.0), 0.0));
        assert!(approx(itp.linear(0.5), 5.0));
        assert!(approx(itp.linear(1.0), 10.0));
        // cubic in middle
        let _c = itp.cubic(1.25);
    }
```
```rust
    #[test]
    fn point_interp() {
        let p0 = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let p1 = Point {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };
        let p = PointInterpolator::lerp_3d(&p0, &p1, 0.25);
        assert!(approx(p.x, 0.5) && approx(p.y, 0.0) && approx(p.z, 0.0));

        let arr = vec![
            Point2 { x: 0.0, y: 0.0 },
            Point2 { x: 1.0, y: 1.0 },
            Point2 { x: 2.0, y: 0.0 },
        ];
        let q = PointInterpolator::lerp_2d(&arr[0], &arr[1], 0.5);
        assert!(approx(q.x, 0.5) && approx(q.y, 0.5));
    }
```
```rust
    #[test]
    fn cc_polynomials() {
        let n = 128; // 짝수
        let s1 = Integrator::clenshaw_curtis_lobatto(|x| x, 0.0, 1.0, n);
        let s2 = Integrator::clenshaw_curtis_lobatto(|x| x * x, 0.0, 1.0, n);
        assert!((s1 - 0.5).abs() < 1e-14, "x:   got {}", s1);
        assert!((s2 - 1.0 / 3.0).abs() < 1e-14, "x^2: got {}", s2);
    }
```
```rust
    #[test]
    fn cc_against_gauss_legendre_exp() {
        let n = 256;
        let s_cc = Integrator::clenshaw_curtis_lobatto(|x| x.exp(), 0.0, 1.0, n);
        let s_gl = Integrator::gauss_legendre(|x| x.exp(), 0.0, 1.0);
        assert!((s_cc - s_gl).abs() < 1e-12, "cc={} vs gl={}", s_cc, s_gl);
    }
```
```rust
    #[test]
    fn cc_abs_nonsmooth_fixed_n() {
        // ∫_{-1}^{1} |x| dx = 1
        // 비매끈 → N=256에서 1e-8 정도 기대; 1e-10은 과한 기대치.
        let n = 256;
        let s = Integrator::clenshaw_curtis_lobatto(|x| x.abs(), -1.0, 1.0, n);
        assert!((s - 1.0).abs() < 1e-8, "got {}", s);
    }
```
```rust   
    fn cc_adaptive<F: Fn(f64) -> f64>(
        f: F,
        a: f64,
        b: f64,
        tol: f64,
        n0: usize,
        nmax: usize,
    ) -> f64 {
        let mut n = n0.max(8) & !1; // 짝수 보장
        let mut prev = Integrator::clenshaw_curtis_lobatto(&f, a, b, n);
        loop {
            let n2 = (n * 2).min(nmax);
            let cur = Integrator::clenshaw_curtis_lobatto(&f, a, b, n2);
            if (cur - prev).abs() <= tol || n2 == nmax {
                return cur;
            }
            n = n2;
            prev = cur;
        }
    }
```
```rust    
    #[test]
    fn cc_abs_nonsmooth_adaptive() {
        let s = cc_adaptive(|x| x.abs(), -1.0, 1.0, 1e-10, 64, 16384);
        assert!((s - 1.0).abs() < 1e-10, "got {}", s);
    }
```
```rust
    #[test]
    fn easing_functions_test() {

        let t = 0.5;
        let s = InterpolatorEase::smooth_step(t);
        let q_in = InterpolatorEase::quadratic_in(t);
        let q_out = InterpolatorEase::quadratic_out(t);
        let q_io = InterpolatorEase::quadratic_in_out(t);
        let c_in = InterpolatorEase::cubic_in(t);
        let c_out = InterpolatorEase::cubic_out(t);
        let c_io = InterpolatorEase::cubic_in_out(t);
        let e_in = InterpolatorEase::exponential_in(t);
        let e_out = InterpolatorEase::exponential_out(t);
        let e_io = InterpolatorEase::exponential_in_out(t);
        let b_out = InterpolatorEase::bounce_out(t);
        let el_out = InterpolatorEase::elastic_out(t);
        let back_out = InterpolatorEase::back_out(0.5);

        assert!(approx(s, 0.5));
        assert!(approx(q_in, 0.25));
        assert!(approx(q_out, 0.75));
        assert!(approx(q_io, 0.5));
        assert!(approx(c_in, 0.125));
        assert!(approx(c_out, 0.875));
        assert!(approx(c_io, 0.5));
        assert!(e_in > 0.0 && e_in < 1.0);
        assert!(e_out > 0.0 && e_out < 1.0);
        assert!(e_io > 0.0 && e_io < 1.0);
        assert!(b_out > 0.0 && b_out < 1.0);
        assert!(el_out > 0.0 && el_out < 1.0);
        assert!(back_out > 1.0); // overshoot 확인
    }
}
```
---
