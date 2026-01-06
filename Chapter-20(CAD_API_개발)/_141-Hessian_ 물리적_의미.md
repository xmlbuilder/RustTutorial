# Hessian
## 1. Hessian이 뭔지부터 정리
- 지금 만든 compute_surface_2nd_derivative_matrix 는 사실 표면과 어떤 기준점(ref_point) 사이의 거리 함수에 대한 Hessian(2차 미분 행렬) 을 계산.
### 1.1. 스칼라 함수의 Hessian
- 스칼라 함수 $f(x), x\in \mathbb{R^{\mathnormal{n}}}$ 에서:
- Gradient(그라디언트):
```math
\nabla f(x)=\left[ \begin{matrix}\frac{\partial f}{\partial x_1}\\ \vdots \\ \frac{\partial f}{\partial x_n}\end{matrix}\right] 
```
- Hessian(헤시안):
```math
H(x) = 
\begin{bmatrix}
\frac{\partial^2 f}{\partial x_1^2} 
    & \frac{\partial^2 f}{\partial x_1 \partial x_2} 
    & \cdots \\[8pt]
\frac{\partial^2 f}{\partial x_2 \partial x_1} 
    & \frac{\partial^2 f}{\partial x_2^2} 
    & \cdots \\[8pt]
\vdots 
    & \vdots 
    & \ddots
\end{bmatrix}
``` 
- Hessian은 “곡률과 2차 형상”을 담고 있는 행렬이고,
- 쉽게 말하면 주변에서 함수가 어떤 모양의 **그릇 / 안장 / 평면** 인지를 표현한다.

```rust

pub fn on_compute_surface_2nd_derivative_matrix(
    surface: &dyn SurfaceGeom,
    u: f64,
    v: f64,
    ref_point: Point3D,
    hessian: &mut Matrix,
) -> bool {
    // ders[du][dv]
    let ders = surface.evaluate_derivatives_nder(u, v, 2);
    if ders.is_empty() || ders[0].is_empty() {
        return false;
    }

    // Extract derivatives (OpenNURBS ordering)
    let position = ders[0][0]; // S
    let du       = ders[1][0]; // S_u
    let dv       = ders[0][1]; // S_v
    let d2u      = ders[2][0]; // S_uu
    let dudu     = ders[1][1]; // S_uv
    let d2v      = ders[0][2]; // S_vv

    // diff = f(u,v) - refPoint
    let diff = position - ref_point.to_vec();

    // off-diagonal term
    let off_diag = du.dot(&dv) + diff.dot(&dudu);

    // Fill Hessian
    hessian.resize(2, 2);
    hessian[(0, 0)] = du.dot(&du) + diff.dot(&d2u);
    hessian[(0, 1)] = off_diag;
    hessian[(1, 0)] = off_diag;
    hessian[(1, 1)] = dv.dot(&dv) + diff.dot(&d2v);

    true
}
```

## 2. 이 코드에서의 Hessian: Surface + Point
- 지금 함수는 이런 형태의 스칼라 함수를 암묵적으로 다룬다:
```math
F(u,v)=\frac{1}{2}\| S(u,v)-P\| ^2
```
- S(u,v): NURBS 표면
- P: ref_point (공간에서 고정된 어떤 점)
- 즉, **파라미터 (u,v) 에서 표면 점과 ref_point 사이의 거리 제곱** 이다.
- 이 함수의 gradient = 1차 미분, Hessian = 2차 미분은:
    - Gradient: Newton iteration 에서 방향/스텝 계산
    - Hessian: 곡률/2차 정보로 뉴턴 스텝을 더 정확하게 만든다
- 그리고 Rust 코드:
```rust
H(0,0) = du·du + diff·d2u;
H(0,1) = du·dv + diff·dudu;
H(1,1) = dv·dv + diff·d2v;
```
- 이 정확히 F(u,v) 의 2차 미분에 해당한다.

## 3. 수식으로 확인해보기
```math
F(u,v)=\frac{1}{2}\| S(u,v)-P\| ^2=\frac{1}{2}(S-P)\cdot (S-P)
```
- 여기서 $\mathrm{diff}=S-P$ 라 두면:
### 4.1. 1차 미분
```math
F_u=\frac{\partial F}{\partial u}=\mathrm{diff}\cdot S_u
```
```math
F_v=\frac{\partial F}{\partial v}=\mathrm{diff}\cdot S_v
```
### 4.2. 2차 미분
```math
F_{uu}=\frac{\partial }{\partial u}(\mathrm{diff}\cdot S_u)=S_u\cdot S_u+\mathrm{diff}\cdot S_{uu}
```
```math
F_{vv}=S_v\cdot S_v+\mathrm{diff}\cdot S_{vv}
```
```math
F_{uv}=\frac{\partial }{\partial v}(\mathrm{diff}\cdot S_u)=S_v\cdot S_u+\mathrm{diff}\cdot S_{uv}
```
- 이게 코드와 1:1 매핑된다:
    - $du = S_u$
    - $dv = S_v$
    - $d2u = S_{uu}$
    - $dudu = S_{uv}$
    - $d2v = S_{vv}$
    - $diff = S-P$
- 따라서:
```rust
H(0,0) = du·du + diff·d2u  ≡ F_uu
H(1,1) = dv·dv + diff·d2v  ≡ F_vv
H(0,1) = du·dv + diff·dudu ≡ F_uv
H(1,0) = H(0,1)
```

- 즉, 이 Hessian은 **(u,v)에서 거리 제곱 함수의 2차 미분** 이다.

## 5. Point Inversion에서의 역할 (왜 중요한지)
- Point Inversion = **공간의 점 P가 주어졌을 때, 표면 S(u,v) 위에서 P와 가장 가까운 점의 파라미터 (u*, v*)를 찾는 문제**
- 이건 최적화 문제:
```math
\min _{u,v}F(u,v)=\frac{1}{2}\| S(u,v)-P\| ^2
```
- 이걸 푸는 대표적인 알고리즘이 Newton-Raphson이고, 뉴턴 한 스텝은:
```math
\left[ \begin{matrix}u_{k+1}\\ v_{k+1}\end{matrix}\right] =\left[ \begin{matrix}u_k\\ v_k\end{matrix}\right] -H^{-1}(u_k,v_k)\, \nabla F(u_k,v_k)
```
- 여기서:
- $\nabla F=[F_u,F_v]^T$
- H = 지금 만든 Hessian(2×2)
- 즉, Point Inversion은 Gradient + Hessian 둘 다 필요하고,
- 그 중에서도 Hessian은 **이 방향으로 얼마나 세게 휘고 있는지** 를 말해주는 정보다.
- 왜 Hessian이 좋냐면:
    - Gradient만 쓰는 방법(steepest descent)는 수렴이 느리고 방향이 비틀린다.
    - Hessian을 쓰면 **곡률을 고려한 최적의 로컬 이동 방향과 크기”** 를 주기 때문에
    - 적은 스텝으로 원하는 파라미터에 더 정확히 도달할 수 있다.
- 그래서 상용 CAD/CAE 커널들은 Point Inversion에서 대부분 Hessian 기반 뉴턴을 쓴다.

## 6. 물리적인 의미
### 6.1. F(u,v) = 거리 제곱 함수의 모양
- 표면 위에서 (u,v)를 살짝 움직이면, ref_point까지의 거리가 변한다.
    - Gradient: **어느 방향으로 움직이면 거리(에너지)가 줄어드는지** 를 알려줌
    - Hessian: 그 주변에서 함수가 볼록한지, 안장인지, 평평한지를 알려줌
- 좀 더 물리적인 비유로:
    - 표면 위에 고무막이 있고, ref_point와 그 지점 사이의 스프링 에너지가 F(u,v)라고 가정.
    - Hessian은 그 지점 근처에서 그 에너지 풍경이 어떤 곡률을 가지는지 알려준다.
### 6.2. 최소 거리점에서의 Hessian
- P에서 표면까지의 최근접점(최소거리점)에 도달하면:
    - Gradient = 0 (더 내려갈 곳 없음)
    - Hessian은 그 지점이 최소인지, 최대인지, 안장인지를 판별한다.
- 거리 제곱 함수는 대개 최소만 가지기 때문에,
    - Hessian은 **이 최소가 얼마나 튼튼한(강한) 최소인지** 를 알려준다.
    - 고르게 완만한 곡률 → Hessian 작은 값
    - 아주 빡세게 구겨진 곳(곡률 큼) → Hessian 큰 값
- 그래서 Point Inversion에서 Hessian이 크면:
    - **조금만 움직여도 거리가 확 늘어난다**
    - 민감한 영역, 곡률이 큰 곳

## 7. Hessian이 다른 곳에서 쓰이는 예시들
- Point / Curve / Surface Inversion
- 거리 제곱 함수 최소화에서 뉴턴 스텝
- robust closest point 계산
- 곡률 기반 Feature 분석
- Surface Hessian은 제2기본형/Shape operator와 연결
- 주곡률, Mean curvature, Gaussian curvature 계산
- 곡률 큰 영역 → 메쉬 refinement, fillet, chamfer 설계 기준
- Optimization / Fitting
- NURBS surface fitting, least-squares에서
- 에너지 함수의 Hessian → quasi-Newton, Gauss-Newton 등
- Contact / Collision
- 물리 엔진에서 표면–점, 표면–표면 penetration depth 추정할 때
- local quadratic 모델링에 Hessian 사용

## 8. 정리
- 지금 on_compute_surface_2nd_derivative_matrix 는 $F(u,v) = ½‖S(u,v) − P‖²$ 의 정확한 Hessian을 계산하고 있다.
- 이 Hessian은:
    - Point Inversion에서 Newton step을 제대로 만드는 핵심
    - 거리 함수의 **국소적인 곡률** 을 표현
    - 물리적으로는 **그 점 주변에서 에너지 풍경이 얼마나 휘어져 있는지** 를 나타냄
- Matrix::resize 테스트까지 포함해서,
- 이 전체 체인은 NURBS geometry 엔진의 매우 정통적인 구성 요소를 그대로 구현하고 있는 상태다.


## ✅ 테스트용 SurfaceGeom 구현 (평면)
- 평면은 다음과 같이 정의하자:
```math
S(u,v)=(u,v,0)
```
그러면:
    - $S_u=(1,0,0)$
    - $S_v=(0,1,0)$
    - $S_{uu}=S_{uv}=S_{vv}=(0,0,0)$
- 그리고 거리 함수:
```math
F(u,v)=\frac{1}{2}\| S(u,v)-P\| ^2
```
- Hessian은:
```math
H=\left[ \begin{matrix}1&0\\ 0&1\end{matrix}\right]
``` 
- 즉, 항상 단위행렬이 되어야 한다.

```rust
#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Formatter};
    use nurbslib::core::domain::Interval;
    use nurbslib::core::math_extensions::on_compute_surface_2nd_derivative_matrix;
    use nurbslib::core::matrix::Matrix;
    use nurbslib::core::prelude::{Point3D, Real};
    use nurbslib::topology::geom_kernel::SurfaceGeom;
    use nurbslib::core::prelude::{Vector3D};

    /// 테스트용 평면 S(u,v) = (u, v, 0)
    struct TestPlane;

    impl Debug for TestPlane {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl SurfaceGeom for TestPlane {
        fn evaluate_derivatives_nder(&self, u: f64, v: f64, _d: usize) -> Vec<Vec<Vector3D>> {
            // ders[du][dv]
            // 0차
            let s = Vector3D::new(u, v, 0.0);
            // 1차
            let su = Vector3D::new(1.0, 0.0, 0.0);
            let sv = Vector3D::new(0.0, 1.0, 0.0);
            // 2차
            let suu = Vector3D::new(0.0, 0.0, 0.0);
            let suv = Vector3D::new(0.0, 0.0, 0.0);
            let svv = Vector3D::new(0.0, 0.0, 0.0);

            vec![
                vec![s, sv, svv],   // ders[0][0], ders[0][1], ders[0][2]
                vec![su, suv],      // ders[1][0], ders[1][1]
                vec![suu],          // ders[2][0]
            ]
        }

        fn domain_u(&self) -> Interval {
            todo!()
        }

        fn domain_v(&self) -> Interval {
            todo!()
        }

        fn eval_point(&self, u: Real, v: Real) -> Point3D {
            todo!()
        }
    }

    #[test]
    fn test_surface_2nd_derivative_matrix_plane() {
        let plane = TestPlane;

        let u = 1.2;
        let v = -0.7;
        let ref_point = Point3D::new(0.0, 0.0, 0.0);

        let mut hessian = Matrix::new();

        let ok = on_compute_surface_2nd_derivative_matrix(
            &plane,
            u,
            v,
            ref_point,
            &mut hessian,
        );

        assert!(ok);
        assert_eq!(hessian.rows(), 2);
        assert_eq!(hessian.cols(), 2);

        // 평면의 Hessian은 항상 단위행렬
        let h00 = hessian.get(0, 0);
        let h01 = hessian.get(0, 1);
        let h10 = hessian.get(1, 0);
        let h11 = hessian.get(1, 1);

        assert!((h00 - 1.0).abs() < 1e-12);
        assert!(h01.abs() < 1e-12);
        assert!(h10.abs() < 1e-12);
        assert!((h11 - 1.0).abs() < 1e-12);
    }
}
```
---
