# Principal curvature, directions

- 주만곡값/방향(Principal curvature, directions) 구하는 코드.
- 수식 자체는 전형적인 Weingarten map S=I^{-1}II 고유값/고유벡터 계산 사용
- 아래에:


## 1. 수식 간단 점검
- 입력:
- dU, dV : 접선 벡터 $\partial f/\partial u,\partial f/\partial v$
- dUU, dUV, dVV : 2차 미분 $\partial ^2f/\partial u^2,\partial ^2f/\partial uv,\partial ^2f/\partial v^2$
- surfNorm : 표면 법선 (단위벡터라고 가정)
- 제1기본형 계수
```math
E=a=dU\cdot dU,\quad F=b=dU\cdot dV,\quad G=c=dV\cdot dV
```
- 제2기본형 계수(법선 방향으로 투영)
```math
e=fx=n\cdot dUU,\quad f=fy=n\cdot dUV,\quad g=fz=n\cdot dVV
```
- Weingarten map S=I^{-1}II 의 trace/det
```math
\Delta =EG-F^2=ac-b^2
```
```math
\mathrm{detTerm}=\det S=\frac{eg-f^2}{\Delta }=\frac{fx\cdot fz-fy^2}{ac-b^2}
```
```math
\mathrm{trace}=\mathrm{tr}(S)=\frac{Ge-2Ff+Eg}{\Delta }=\frac{cfx-2bfy+afz}{ac-b^2}
```
- 고유값(주만곡)
```math
\lambda _{1,2}=\frac{\mathrm{trace}\pm \sqrt{\mathrm{trace^{\mathnormal{2}}}-4\det }}{2}
```
- 코드는 이 공식을 그대로 쓰되,
- 판별식이 작을 때 수치적 안정성 확보
- $|\lambda _1|\geq |\lambda _2|$ 되도록 재정렬
- 고유벡터 계산 실패/불안정 시 fallback 을 추가한 구조.

## 소스 코드
```rust
/// Compute principal curvatures and directions from first/second derivatives.
///
/// d_u, d_v   : first derivatives (tangent basis)
/// d_uu,d_uv,d_vv : second derivatives
/// surf_norm  : surface normal (should be unit)
///
/// Outputs:
///   det_term  = det(S) = k1 * k2
///   trace_half = 0.5 * tr(S) = 0.5 * (k1 + k2)
///   prin_val1, prin_val2 : principal curvatures (|prin_val1| >= |prin_val2|)
///   prin_dir1, prin_dir2 : corresponding principal directions (tangent to surface)
pub fn on_compute_principal_directions_and_values(
    d_u: Vector3D,
    d_v: Vector3D,
    d_uu: Vector3D,
    d_uv: Vector3D,
    d_vv: Vector3D,
    surf_norm: Vector3D,
    det_term: &mut Real,
    trace_half: &mut Real,
    prin_val1: &mut Real,
    prin_val2: &mut Real,
    prin_dir1: &mut Vector3D,
    prin_dir2: &mut Vector3D,
) -> bool {
    // 1) First fundamental form coefficients
    let a = d_u.dot(&d_u); // E
    let b = d_u.dot(&d_v); // F
    let c = d_v.dot(&d_v); // G

    // 2) Second fundamental form coefficients (projected on normal)
    let fx = surf_norm.dot(&d_uu); // e
    let fy = surf_norm.dot(&d_uv); // f
    let fz = surf_norm.dot(&d_vv); // g

    *det_term = 0.0;
    *trace_half = 0.0;
    *prin_val1 = 0.0;
    *prin_val2 = 0.0;
    *prin_dir1 = Vector3D::UNSET_VECTOR;
    *prin_dir2 = Vector3D::UNSET_VECTOR;

    // denominator = EG - F^2
    let denominator = a * c - b * b;
    if denominator == 0.0 {
        return false;
    }

    let inv_det = 1.0 / denominator;

    // det(S) and trace(S)
    let det = (fx * fz - fy * fy) * inv_det;
    let trace = (c * fx - 2.0 * b * fy + a * fz) * inv_det;

    *det_term = det;
    *trace_half = 0.5 * trace;

    let trace_squared = trace * trace;

    let mut lambda1: Real;
    let mut lambda2: Real;

    // 3) Eigenvalues: cases for numerical stability
    if trace_squared < 4.0 * det {
        // Discriminant < 0 (complex) → degenerate; treat as flat if det small
        if det > 1e-15 {
            return false;
        }
        lambda1 = 0.0;
        lambda2 = 0.0;
        *det_term = 0.0;
        *trace_half = 0.0;
    } else if trace_squared == 0.0 {
        // trace = 0 → symmetric ±√(-det)
        if det > 0.0 {
            return false;
        }
        lambda1 = (-det).sqrt();
        lambda2 = -lambda1;
    } else {
        // General case: numerically stable formula
        let root = (1.0 - (4.0 * det / trace_squared)).sqrt();
        lambda1 = 0.5 * trace.abs() * (1.0 + root);
        if trace < 0.0 {
            lambda1 = -lambda1;
        }
        lambda2 = det / lambda1;
    }

    // Ensure |lambda1| >= |lambda2|
    if lambda1.abs() > lambda2.abs() {
        *prin_val1 = lambda1;
        *prin_val2 = lambda2;
    } else {
        *prin_val1 = lambda2;
        *prin_val2 = lambda1;
        std::mem::swap(&mut lambda1, &mut lambda2);
    }

    let mut fallback = true;

    // 4) Try to compute principal directions if eigenvalues sufficiently distinct
    if (lambda1 - lambda2).abs() > 1e-6 * (lambda1.abs() + lambda2.abs()) {
        fallback = false;

        // Matrix representation of S in {d_u, d_v} basis
        let m00 = (c * fx - b * fy) * inv_det;
        let m01 = (c * fy - b * fz) * inv_det;
        let m10 = (a * fy - b * fx) * inv_det;
        let m11 = (a * fz - b * fy) * inv_det;

        let mut directions = [Vector3D::UNSET_VECTOR; 2];
        let mut magnitudes = [0.0_f64; 2];

        for i in 0..2 {
            let lambda = if i == 0 { *prin_val1 } else { *prin_val2 };

            // Heuristic for eigenvector to avoid degeneracy
            let cond =
                (m00 - lambda) * m10 + m01 * (m11 - lambda) >= 0.0;

            let (x, y) = if cond {
                // (m00 - λ + m10, m01 + m11 - λ)
                (m00 - lambda + m10, m01 + m11 - lambda)
            } else {
                // (m00 - λ - m10, m01 - m11 + λ)
                (m00 - lambda - m10, m01 - m11 + lambda)
            };

            // dir = -y * d_u + x * d_v  (stays in tangent plane)
            let mut dir = (-y) * d_u.to_point() + x * d_v.to_point();
            let len = dir.length();
            if len > 0.0 {
                dir = dir / len;
            }

            directions[i] = dir.to_vec();
            magnitudes[i] = len;
        }

        *prin_dir1 = directions[0];
        *prin_dir2 = directions[1];

        // Check validity w.r.t surface normal
        let mut valid1 = prin_dir1.dot(&surf_norm).abs() >= 0.0001;
        let mut valid2 = prin_dir2.dot(&surf_norm).abs() >= 0.0001;

        // If both invalid but directions not orthogonal, pick the stronger one
        if !valid1 && !valid2 && prin_dir1.dot(prin_dir2).abs() >= 0.0001 {
            if magnitudes[0] < magnitudes[1] {
                valid1 = true;
            } else {
                valid2 = true;
            }
        }

        if valid1 || valid2 {
            if valid1 && valid2 {
                // both directions acceptable → still fall back to robust basis
                fallback = true;
            } else if valid1 {
                // construct orthogonal partner
                let mut d = Vector3D::cross_vec(&prin_dir2, &surf_norm);
                if d.normalize() {
                    *prin_dir1 = d;
                }
            } else {
                let mut d = Vector3D::cross_vec(&surf_norm, &prin_dir1);
                if d.normalize() {
                    *prin_dir2 = d;
                }
            }
        }
    }

    // 5) Fallback: use d_u, d_v as basis
    if fallback {
        if a >= c {
            *prin_dir1 = d_u;
        } else {
            *prin_dir1 = d_v;
        }
        prin_dir1.normalize();
        let mut d = Vector3D::cross_vec(&surf_norm, prin_dir1);
        if !d.normalize() {
            // 극단적 퇴화 케이스에서 방어
            d = Vector3D::perpendicular_to(&surf_norm);
            d.normalize();
        }
        *prin_dir2 = d;
    }

    true
}
```
---


## 🎯 1. 표면에서 곡률을 구하는 기본 원리
- 어떤 매끄러운 표면 f(u,v) 에서:
    - 1차 미분:
        ```math
        f_u=dU,\quad f_v=dV
        ```
        - 표면의 접선 벡터
    - 2차 미분:
    ```math
    f_{uu}=dUU,\quad f_{uv}=dUV,\quad f_{vv}=dVV
    ```
    - 법선 벡터:
    ```math
    n=surfNorm
    ```
## 🎯 2. 제1기본형(First Fundamental Form)
- 표면의 metric(길이/각도)을 나타내는 행렬:
```math
I=\left[ \begin{matrix}E&F\\ F&G\end{matrix}\right] =\left[ \begin{matrix}dU\cdot dU&dU\cdot dV\\ dU\cdot dV&dV\cdot dV\end{matrix}\right] 
```
- 코드에서는:
```rust
double a = dU * dU; // E
double b = dU * dV; // F
double c = dV * dV; // G
```
![First Fundamental](/image/curvaute_first.png)

## 🎯 3. 제2기본형(Second Fundamental Form)
- 곡률을 나타내는 행렬:

```math
II=\left[ \begin{matrix}e&f\\ f&g\end{matrix}\right] =\left[ \begin{matrix}n\cdot f_{uu}&n\cdot f_{uv}\\ n\cdot f_{uv}&n\cdot f_{vv}\end{matrix}\right]
``` 

- 코드에서는:
```rust
double fx = surfNorm * dUU; // e
double fy = surfNorm * dUV; // f
double fz = surfNorm * dVV; // g
```

![Normal](/image/curvaute_normal.png)

![Second Fundamental](/image/curvaute_principal_curvature.png)


## 🎯 4. Shape Operator (Weingarten map)
- 곡률을 결정하는 핵심 연산:
- $S=I^{-1}II$ 이 행렬의 고유값이 바로:
    - $k_1$ (최대곡률)
    - $k_2$ (최소곡률)
- 그리고 고유벡터가:
- 주곡률 방향 e₁, e₂
## 🎯 5. 코드가 하는 일 = S의 고유값/고유벡터 계산

### 5.1 det(S) 계산
```math
\det (S)=\frac{eg-f^2}{EG-F^2}
```
- 코드:
```rust
double det = (fx * fz - fy * fy) * invDet;
```
### 5.2 trace(S) 계산
```math
\mathrm{tr}(S)=\frac{Ge-2Ff+Eg}{EG-F^2}
```
- 코드:
```rust
double trace = (c * fx - 2.0 * b * fy + a * fz) * invDet;
```
### 5.3 고유값(주만곡) 계산
```math
\lambda _{1,2}=\frac{\mathrm{tr}(S)\pm \sqrt{\mathrm{tr^{\mathnormal{2}}}-4\det }}{2}
```
- 코드:
```rust
double root = sqrt(1.0 - (4.0 * det / traceSquared));
lambda1 = 0.5 * fabs(trace) * (1.0 + root);
lambda2 = det / lambda1;
```
- 이게 바로 k₁, k₂.

## 🎯 6. 고유벡터(주곡률 방향) 계산고유값 λ에 대해:
- $(S-\lambda I)v=0$ 이걸 풀면 고유벡터 v가 나오는데,
- 코드는 수치적 안정성을 위해 다음과 같이 계산:
```rust
x = m00 - lambda ± m10
y = m01 ± (m11 - lambda)
dir = (-y) * dU + x * dV
```
- 즉,
    - 고유벡터는 dU, dV의 선형결합으로 표현
    - 표면 접평면 위에 존재
    - normalize 해서 방향 벡터로 만듦
- 이게 바로 principal direction.

## 🎯 7. fallback이 필요한 이유곡률이 거의 평평하거나,
- 두 고유값이 거의 같으면(= umbilic point),고유벡터가 불안정해짐.그래서 fallback:
- dU 또는 dV 중 더 안정적인 방향을 선택
- surfNorm과 cross 해서 직교 방향 생성

## 🎉 결론:
- 이 함수는 정확히 Shape operator $S = I⁻¹ II$ 의 고유값/고유벡터를 구한다.
- 그래서:
- prinVal1, prinVal2 = 주만곡값 k₁, k₂
- prinDir1, prinDir2 = 주곡률 방향 e₁, e₂
- 즉, 표면의 곡률을 결정하는 가장 기본적인 미분기하 공식을 그대로 구현한 것이다.

---
