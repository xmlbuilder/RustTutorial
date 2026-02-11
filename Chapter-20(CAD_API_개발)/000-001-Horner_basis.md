
# Horner Function
## 📘 Horner’s Method for Power Basis Curve Evaluation
- n차 power basis 곡선이 다음과 같이 주어졌다고 하자:
```math
\mathbf{C}(u)=\sum _{i=0}^n\mathbf{a_{\mathnormal{i}}}u^i
```
- Horner 방식은 이를 다음과 같이 재귀적 형태로 변환한다:

✔ Horner Form
```math
\mathbf{C}(u)=(((\mathbf{a_{\mathnormal{n}}}u+\mathbf{a_{\mathnormal{n-1}}})u+\mathbf{a_{\mathnormal{n-2}}})u+\cdots +\mathbf{a_{\mathnormal{1}}})u+\mathbf{a_{\mathnormal{0}}}
```

- ✔ 행렬/벡터 형태 (이미지 스타일로 정리)
```math
\mathbf{C}(u)=\mathbf{a_{\mathnormal{0}}}+u\left( \mathbf{a_{\mathnormal{1}}}+u\left( \mathbf{a_{\mathnormal{2}}}+u\left( \cdots +u\mathbf{a_{\mathnormal{n}}}\right) \right) \right) 
```

- ✔ 재귀적 정의
```math
\mathbf{b_{\mathnormal{n}}}=\mathbf{a_{\mathnormal{n}}}
```
```math
\mathbf{b_{\mathnormal{k}}}=\mathbf{a_{\mathnormal{k}}}+u\, \mathbf{b_{\mathnormal{k+1}}}\quad (k=n-1,n-2,\dots ,0)
```
- 최종적으로:
```math
\mathbf{C}(u)=\mathbf{b_{\mathnormal{0}}}
```
- ✔ 스칼라 성분별 표현
```math
x(u)=(((x_nu+x_{n-1})u+x_{n-2})u+\cdots +x_1)u+x_0
```
```math
y(u)=(((y_nu+y_{n-1})u+y_{n-2})u+\cdots +y_1)u+y_0
```
```math
z(u)=(((z_nu+z_{n-1})u+z_{n-2})u+\cdots +z_1)u+z_0
```

## 📘 요약
- Power basis 곡선은 $u^i$ 항을 직접 계산하면 비효율적
- Horner 방식은 곱셈/덧셈 횟수를 최소화
- 교재 스타일의 “중첩된 괄호” 형태로 표현 가능
- 벡터/행렬 형태로도 자연스럽게 표현됨


## 📘 2D Power Basis Surface — Horner Form (정석 2D Horner)
- 2D power basis surface:
```math
\mathbf{S}(u,v)=\sum _{i=0}^p\sum _{j=0}^q\mathbf{a_{\mathnormal{ij}}}u^iv^j
```
- 2D Horner는 v 방향 Horner → u 방향 Horner 순서로 계산하는 것이 정석이다.

- ✔ Step 1 — v 방향 Horner (각 i에 대해)
```math
\mathbf{c_{\mathnormal{i}}}(v)=(((\mathbf{a_{\mathnormal{i,q}}}v+\mathbf{a_{\mathnormal{i,q-1}}})v+\mathbf{a_{\mathnormal{i,q-2}}})v+\cdots +\mathbf{a_{\mathnormal{i,1}}})v+\mathbf{a_{\mathnormal{i,0}}}
```
- 재귀적 정의:
```math
\mathbf{d_{\mathnormal{i,q}}}=\mathbf{a_{\mathnormal{i,q}}}
```
```math
\mathbf{d_{\mathnormal{i,j}}}=\mathbf{a_{\mathnormal{i,j}}}+v\, \mathbf{d_{\mathnormal{i,j+1}}}
```
```math
\mathbf{c_{\mathnormal{i}}}(v)=\mathbf{d_{\mathnormal{i,0}}}
```
- ✔ Step 2 — u 방향 Horner ( $c_i(v)$ 를 이용)
```math
\mathbf{S}(u,v)=(((\mathbf{c_{\mathnormal{p}}}(v)u+\mathbf{c_{\mathnormal{p-1}}}(v))u+\mathbf{c_{\mathnormal{p-2}}}(v))u+\cdots +\mathbf{c_{\mathnormal{1}}}(v))u+\mathbf{c_{\mathnormal{0}}}(v)
```
- 재귀적 정의:
```math
\mathbf{e_{\mathnormal{p}}}=\mathbf{c_{\mathnormal{p}}}(v)
```
```math
\mathbf{e_{\mathnormal{k}}}=\mathbf{c_{\mathnormal{k}}}(v)+u\, \mathbf{e_{\mathnormal{k+1}}}
```
```math
\mathbf{S}(u,v)=\mathbf{e_{\mathnormal{0}}}
```
## 📘 2D Horner 전체를 한 줄로 표현하면
```math
\mathbf{S}(u,v)=\sum _{i=0}^p\left( \sum _{j=0}^q\mathbf{a_{\mathnormal{ij}}}v^j\right) u^i
```
- 이를 Horner로 풀면:
```math
\mathbf{S}(u,v)=(((\mathbf{d_{\mathnormal{p}}}(v)u+\mathbf{d_{\mathnormal{p-1}}}(v))u+\mathbf{d_{\mathnormal{p-2}}}(v))u+\cdots +\mathbf{d_{\mathnormal{1}}}(v))u+\mathbf{d_{\mathnormal{0}}}(v)
```
- 여기서:
```math
\mathbf{d_{\mathnormal{i}}}(v)=(((\mathbf{a_{\mathnormal{i,q}}}v+\mathbf{a_{\mathnormal{i,q-1}}})v+\mathbf{a_{\mathnormal{i,q-2}}})v+\cdots +\mathbf{a_{\mathnormal{i,1}}})v+\mathbf{a_{\mathnormal{i,0}}}
```
## 📘 요약
- 1D Horner:
    - $(((a_nu+a_{n-1})u+\cdots )u+a_0)$
- 2D Horner:
    - 먼저 v 방향 Horner
    - 그 결과를 u 방향 Horner로 다시 평가
        - 2D 다항식을 가장 효율적으로 계산하는 방식
---
## 소스 코드
```rust
/// Horner 방법: 다항식 평가 (coefficients[0] + coefficients[1] t + ... + coefficients[n] t^n)
pub fn on_horner(coefficients: &[Real], t: Real) -> Real {
    let mut result = 0.0;
    for &c in coefficients.iter().rev() {
        result = result * t + c;
    }
    result
}
```
```rust
/// Horner 방법 (오름차순: a0 + a1 t + ... + an t^n)
pub fn on_horner_ascending(coefficients: &[Real], t: Real) -> Real {
    let mut s = 0.0;
    for i in (0..coefficients.len()).rev() {
        s = s * t + coefficients[i];
    }
    s
}
```
```rust
/// Horner 방법 (내림차순: an + ... + a0)
pub fn on_horner_descending(coefficients: &[Real], t: Real) -> Real {
    let mut s = 0.0;
    for &c in coefficients.iter() {
        s = s * t + c;
    }
    s
}
```
```rust
/// 2차원 Horner (예: Bezier surface 평가)
/// coefficients: 행렬 형태 [degree_u+1][degree_v+1]
pub fn on_horner_2d(coefficients: &[Vec<Real>], u: Real, v: Real) -> Real {
    let degree_u = coefficients.len() - 1;
    let mut temp = Vec::with_capacity(degree_u + 1);

    for row in coefficients {
        temp.push(on_horner(row, v));
    }
    on_horner(&temp, u)
}
```
```rust

/// Evaluate a polynomial curve in power basis using Horner's method.
/// aw[i] = coefficient of u^i
pub fn on_eval_power_basis_horner(aw: &[Point3D], u: Real) -> Point3D {
    let n = aw.len();
    if n == 0 {
        return Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    // Start with highest degree coefficient
    let mut c = aw[n - 1];

    // Horner iteration
    for i in (0..n - 1).rev() {
        c = c * u + aw[i];
    }

    c
}
```
---
