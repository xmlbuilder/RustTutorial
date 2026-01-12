
# CFun 정리

- CFun이 하는 건 한 줄로 말하면:
    - **곡선 좌표 하나(x, y, z, w)만 떼서 보는 1D B-spline 함수의 symbolic 엔진**

- 스칼라 함수에 대한 같은 레벨의 symbolic 연산이라고 보면 된다.
- 아래에서 함수별로:
    - 수식
    - 알고리즘이 뭘 하는지
    - 어디에 쓰는지


## 1. CFun::new
```rust
pub fn new(p: Degree, knots: KnotVector, fu: Vec<Real>) -> Result<Self>
```

- 의미
    - 차수 p, knot vector U, control values $f_i$ 로 스칼라 B-spline 함수 f(u)를 만든다.
    - NurbsCurve에서의 control point 대신, 여기선 control value 하나짜리 곡선.
- 수식
    - B-spline 함수:
```math
f(u)=\sum _{i=0}^nN_i^p(u)f_i
```
- 여기서
- $N_i^p(u)$ : B-spline basis
- $f_i$ : 스칼라 control value
- $n+1=|fu|$
- check_degree_vs_cp로 위 수식이 잘 정의되는지 검증한다.

## 2. CFun::eval
```rust
pub fn eval(&self, u: Param, side: Side) -> Result<Real>
```

- 기능
    - 주어진 매개변수 u에서 B-spline 함수 값을 계산:
```math
f(u)=\sum _{j=0}^pN_{i-p+j}^p(u)\, f_{i-p+j}
```
- 여기서 $i=\mathrm{span\  index}$.
- 코드와 수식 대응
- span 찾기:
```rust
let span = on_find_span_left_eval(&self.knots, self.p, u)? as usize;
```

- $i=\mathrm{span}$
- basis 값:
```rust
let n = on_basis_funs_ret_vec(&self.knots.as_slice(), span, u, p);
```

- $N_j=N_{i-p+j}^p(u), j=0..p$
- 합산:
```rust
let start = span.saturating_sub(p);
for j in 0..=p {
    v += n[j] * self.fu[start + j];
}
```

- 정확히:
```math
f(u)=\sum _{j=0}^pN_j\cdot f_{start+j}
```
- 용도
    - rational NURBS에서 denominator w(u) 계산
    - numerator/den을 분리했을 때 scalar 쪽 평가
    - 곡선의 scalar field (예: 폭, 두께, 속성 함수) 표현

## 3. CFun::derivative_function
```rust
pub fn derivative_function(&self, d: usize) -> Result<CFun>
```

- 기능
    - B-spline 함수 f(u)의 d차 도함수도 B-spline 함수로 표현:
```math
f^{(d)}(u)=\sum _{i=0}^{n-d}N_i^{p-d}(u)\, f_i^{(d)}
```
- 여기서 $f_i^{(d)}$ 는 control value 차분으로 계산.
- 수식 (반복 차분)
- 원래 control values: $f_i^{(0)}=f_i$.
- k번째 단계에서:
```math
f_i^{(k)}=\frac{p-k+1}{U_{i+p+1}-U_{i+k}}\left( f_{i+1}^{(k-1)}-f_i^{(k-1)}\right) 
```
- d번 반복 후:
    - degree → p_d=p-d
    - control 개수 → n_d=n-d
- 코드와 수식 대응
```rust
for k in 1..=d {
    for i in 0..=(n - k) {
        let denom = u[i + p + 1] - u[i + k];
        let alpha = ((p - k + 1) as Real) / denom;
        ft[i] = alpha * (ft[i + 1] - ft[i]);
    }
}
```
- 완전히 위 수식 그대로.
- knot vector:
```math
V=[U_0\mathrm{\  반복\  }(p_d+1),\, U_{p+1},\dots ,U_n,\, U_m\mathrm{\  반복\  }(p_d+1)]
```
- 용도
    - rational curve 도함수에서 D'(u) 구할 때
    - scalar field의 미분 (속도, 변화율 등)
    - CFun을 이용한 더 복잡한 symbolic 연산의 기반

## 4. CFun::refine_with_knot_vector
```rust
pub fn refine_with_knot_vector(&self, knx: &KnotVector) -> Result<CFun>
```

- 기능
- 스칼라 B-spline 함수 f(u)에 대해  
    knx에 들어 있는 knot들을 모두 삽입하여
- 동일한 함수를 더 촘촘한 knot vector 위에서 표현.
- 수학적 의미
    - knot insertion은 곡선을 변형하지 않고 표현만 바꾸는 연산
    - 새로운 control value $f'_i$ 를 만들어서:
```math
f(u)\equiv \sum N_i^{p,U'}(u)f'_i
```
- 여기서 U'는 삽입된 knot vector.
- 알고리즘
- Piegl & Tiller의 knot insertion 알고리즘 그대로:
    - 삽입 knot X에 대해 span 찾기 (LEFT sense)
    - 바뀌지 않는 control 값/knots는 복사
    - 나머지 구간에서 역순으로 alpha blending:
```math
f_{\mathrm{new}}=(1-\alpha )f_{\mathrm{old,left}}+\alpha f_{\mathrm{old,right}}
```
- 코드의 핵심:
```rust
alf = (uq[k + l] - x[j]) / (uq[k + l] - up[i - p + l]);
fq[t - 1] = oma * fq[t] + alf * fq[t - 1];
```

- 용도
    - CFun을 NurbsCurve와 동일한 knot partition으로 맞출 때
    - sym_product에서 두 함수의 knot를 맞추기 전에 refine
    - Bezier 분해 전, 내부 knot multiplicity 조정

## 5. CFun::global_interpolate
```rust
pub fn global_interpolate(params: &[Real], values: &[Real], degree: Degree) -> Result<CFun>
```

- 기능
- 주어진 시료:
    - 매개변수 $u_k$
    - 값 $f_k$
- 에 대해,
- **모든 점을 통과하는 B-spline 함수 f(u)** 를 구성.
```math
f(u_k)=\sum _{j=0}^nN_j^p(u_k)f_j=\mathrm{given\  }values[k]
```
- 수식
    - knot vector는 on_compute_knots_for_global_interpolation로 생성
    - 각 $u_k$ 에 대해 행렬 A의 한 행:
```math
A_{k,j}=N_j^p(u_k)
```
- 선형 시스템:
```math
A\cdot F=V
```
- F: control values $f_j$
- V: 주어진 데이터 값 values[k]
- 해를 구하면:
```math
F=A^{-1}V
```
- 코드에서:
```rust
on_solve_linear_system_dense_mut_flat(&mut a, &mut b, n + 1);
CFun::new(degree, knots, b)
```

- 용도
    - 샘플링된 data → B-spline 함수로 fitting
    - rational curve의 denominator/numerator에 대한 스칼라 함수 fitting
    - 데이터 기반 설계, 공차 보정, 보간 기반 설계

## 6. CFun::sym_product
```rust
pub fn sym_product(&self, other: &CFun, knot_tol: Real) -> Result<CFun>
```

- 기능
    - 두 B-spline 함수 f(u),g(u)의 전역 곱 함수:
```math
h(u)=f(u)\cdot g(u)
```
- 를 정확한 B-spline 함수로 만든다.

- 수학적 구조
    - 같은 basis 위로 옮긴 뒤:
```math
f(u)=\sum N_i^p(u)f_i,\quad g(u)=\sum N_i^p(u)g_i
```
- Bezier segment 단위에서:
    - 각 구간에서 f,g는 Bezier 다항식이다.
```math    
f(u)=\sum _{i=0}^pB_i^p(u)f_i,\quad g(u)=\sum _{j=0}^qB_j^q(u)g_j
```
- 곱:
```math
h(u)=f(u)g(u)=\sum _{k=0}^{p+q}B_k^{p+q}(u)h_k
```
- 여기서 $h_k$ 는 convolution 방식으로 계산.
    - on_bezier_function_product_range 가 이 부분을 담당.
- 전체 구간에 대해 이 Bezier 곱들을 붙여 B-spline으로 재조합.
- 알고리즘 단계
- knot 병합 + refine
    - 두 함수의 knot를 tolerance 내에서 동일하게 만들기
    - 각 내부 knot를 full multiplicity(= degree)로
        - 각 span이 Bezier segment가 되도록 Bezierize
    - 각 Bezier segment에 대해:
```math
h_{\mathrm{seg}}=f_{\mathrm{seg}}\cdot g_{\mathrm{seg}}
```
  
  - 모든 Bezier segment를 이어붙이고,
  - degree p+q, full multiplicity knot vector 생성
- 용도
    - rational NURBS 연산에서 denominator product $D_pD_q$ 등
    - 스칼라 field 곱 (예: weight field, scaling field)
    - 더 복잡한 rational/symbolic 연산의 기반

## 7. CFun::derivative
```rust
pub fn derivative(&self, d: usize) -> Result<CFun>
```

- 이건 위의 derivative_function과 거의 같은 역할이지만,
    - 조금 더 범용 / 다른 경로에서 사용되는 버전이라고 보면 된다.
- 수식, 원리는 완전히 동일:
```math
f_i^{(k)}=\frac{p-k+1}{U_{i+p+1}-U_{i+k}}(f_{i+1}^{(k-1)}-f_i^{(k-1)})
```

## 8. CFun::refine_with_insert_list
```rust
pub fn refine_with_insert_list(&self, insert_vec: &[Real]) -> Result<CFun>
```

- 기능
    - insert_vec에 주어진 knot들을 삽입해,
    - 새로운 knot vector와 control values를 생성한다.
    - refine_with_knot_vector와 구조는 비슷하지만, **삽입 리스트** 를 직접 받아서 쓰는 버전.
- 용도
    - 이미 외부에서 **삽입할 값 리스트** 를 계산해둔 경우
    - NurbsCurve의 refinement와 동일한 패턴으로 CFun도 refine

## 9. cfun_derivatives
```rust
pub fn cfun_derivatives(cfn: &CFun, u: Param, side: Side, der: usize) -> Result<Vec<Real>>
```

- 기능
    - 하나의 u에서:
```math
f(u),f'(u),f''(u),\dots ,f^{(der)}(u)
```
- 를 한 번에 모두 계산.
- 즉, “점에서의 값 + 도함수들”을 벡터로 반환.
- 수식
    - B-spline 함수:
```math
f(u)=\sum _iN_i^p(u)f_i
```
- k차 도함수:
```math
f^{(k)}(u)=\sum _iN_i^{p,(k)}(u)f_i
```
- 여기서 $N_i^{p,(k)}(u)$ 는 basis의 k차 도함수.
- 코드:
- span 찾기:
```rust
let span = on_find_span_left_right(&cfn.knots, cfn.p, u, side)?;
```

- basis 도함수 행렬:
```rust
let bd = on_basis_ders_at_span(&cfn.knots, p, u, span, der);
```

- bd[k][j] = k차 도함수에서 j번째 local basis
- 각 차수별로 합산:
```rust
for k in 0..=der {
    for j in 0..=p {
        s += cfn.fu[i0 + j] * bd[k][j];
    }
    out[k] = s;
}
```

```math
f^{(k)}(u)=\sum _{j=0}^pf_{i_0+j}\cdot N_{i_0+j}^{(k)}(u)
```

- 용도
    - 점에서의 값 + 도함수들을 동시에 얻고 싶을 때
    - tangent, curvature, higher-order 정보 계산
    - rational curve에서 N(u),D(u) 각각에 대한 도함수 필요할 때

- 한 줄 요약
- CFun = 스칼라 B-spline 함수
- 이 파일은:
    - 값 평가 (eval)
    - d차 도함수 (derivative / derivative_function / cfun_derivatives)
    - knot refinement (refine_with_knot_vector / refine_with_insert_list)
    - global interpolation (global_interpolate)
    - 두 함수 곱 (sym_product) 까지 다 갖춘 symbolic scalar B-spline 엔진,
    - NurbsCurve 쪽 rational 연산의 기반.

----
# CFun + NurbsCurve

- NurbsCurve + CFun 구조는 사실 “Rational NURBS 커널의 핵심 아키텍처”.
- 이 둘이 어떻게 상호작용하는지 한눈에 보이도록 구조도(Architecture Diagram) 형태로 정리.
- 아래 내용은 CAD 커널 설계 문서에 그대로 들어갈 수 있는 수준으로 정리.

## 🧩 전체 구조도: NurbsCurve ↔ CFun 상호작용
```
┌──────────────────────────────────────────────────────────────┐
│                        NurbsCurve (Vector)                   │
│  - degree p                                                  │
│  - knot vector U                                             │
│  - control points P_i = (x_i, y_i, z_i, w_i)                 │
│                                                              │
│  Rational curve:                                             │
│      C(u) = ( Σ N_i(u) * w_i * P_i ) / ( Σ N_i(u) * w_i )    │
│                                                              │
│  Uses CFun for:                                              │
│      • numerator curve N(u) = Σ N_i(u) * (w_i * P_i)         │
│      • denominator D(u) = Σ N_i(u) * w_i                     │
│      • symbolic operations (sum, diff, product, derivative)  │
└──────────────────────────────────────────────────────────────┘
                     ▲                           ▲
                     │                           │
                     │ uses                      │ uses
                     │                           │
┌──────────────────────────────────────────────────────────────┐
│                          CFun (Scalar)                       │
│  - degree p                                                  │
│  - knot vector U                                             │
│  - control values f_i (scalar)                               │
│                                                              │
│  Represents scalar B-spline function:                        │
│      f(u) = Σ N_i(u) * f_i                                   │
│                                                              │
│  Provides symbolic operations:                               │
│      • eval(u)                                               │
│      • derivative(d)                                         │
│      • refine_with_knot_vector                               │
│      • sym_product(f, g)                                     │
│      • global_interpolate                                    │
│                                                              │
│  Used by NurbsCurve to handle:                               │
│      • rational numerator/denominator                        │
│      • rational derivative                                   │
│      • rational sum/difference                               │
│      • rational product                                      │
└──────────────────────────────────────────────────────────────┘
```

## 🔥 핵심 개념: NurbsCurve는 벡터, CFun은 스칼라
- ✔ NurbsCurve
    - 4D control point (x, y, z, w)
    - Rational curve
    - 벡터 연산 필요
    - 곡선 자체의 기하학적 정의
- ✔ CFun
    - 스칼라 B-spline 함수
    - control value는 단일 실수
    - rational 연산에서 numerator/denominator 역할
    - symbolic 연산의 기반

## 🎯 왜 CFun이 필요한가? (핵심 이유)
- Rational NURBS 곡선:
```math
C(u)=\frac{\sum N_i(u)w_iP_i}{\sum N_i(u)w_i}
```
- 여기서:
- Numerator:
```math
N(u)=\sum N_i(u)(w_iP_i)
```
- Denominator:
```math
D(u)=\sum N_i(u)w_i
```
- 즉, rational curve는 벡터 함수 / 스칼라 함수의 비율로 표현된다.
- 그래서:
    - Numerator → NurbsCurve (벡터)
    - Denominator → CFun (스칼라)
- 이렇게 분리해야 symbolic 연산이 가능해진다.

## 🧠 NurbsCurve가 CFun을 사용하는 흐름
### 1) Rational sum/difference (sym_sum_difference)
```
C = P/Q,  D = R/S
C ± D = (P*S ± R*Q) / (Q*S)
```
\
- numerator: NurbsCurve
- denominator: CFun
- product: CFun::sym_product
- sum/diff: NurbsCurve::sym_sum_difference

### 2) Rational derivative (sym_derivative_rational)
```
C(u) = N(u)/D(u)
C'(u) = (N' D - N D') / D²
```

- N' → NurbsCurve::sym_derivative_non_rational
- D' → CFun::derivative
- D² → CFun::sym_product
- numerator 조합 → NurbsCurve::sym_sum_difference

### 3) Rational product (내부적으로)
```
(N1/D1) * (N2/D2) = (N1*N2) / (D1*D2)
```
- numerator product → NurbsCurve::sym_product_function_curve
- denominator product → CFun::sym_product

### 4) Knot refinement
- NurbsCurve와 CFun은 동일한 knot vector를 가져야 한다.
- 그래서:
    - NurbsCurve::refine_knot_vector
    - CFun::refine_with_knot_vector
- 둘 다 같은 방식으로 knot insertion을 수행한다.

## 🧩 전체 상호작용 흐름도
```
                ┌──────────────────────────────┐
                │         NurbsCurve           │
                │  (vector numerator)          │
                └──────────────┬───────────────┘
                               │
                               │ uses
                               ▼
                ┌──────────────────────────────┐
                │            CFun              │
                │   (scalar denominator)       │
                └──────────────┬───────────────┘
                               │
                               │ symbolic ops
                               ▼
                ┌──────────────────────────────┐
                │   Knot refinement / product  │
                │   derivative / interpolation │
                └──────────────────────────────┘
```

## 🏁 요약
| Component     | Role / Meaning                                      | Mathematical Object                     | Used For                                      |
|---------------|------------------------------------------------------|-------------------------------------------|-----------------------------------------------|
| NurbsCurve    | Vector-valued NURBS curve                           | C(u) = (Σ N_i w_i P_i) / (Σ N_i w_i)      | Geometry, modeling, rational curve operations |
| CFun          | Scalar B-spline function                             | f(u) = Σ N_i(u) f_i                       | Numerator/denominator, symbolic operations    |
| Interaction   | NurbsCurve uses CFun for rational ops                | C(u) = Num(u) / Den(u)                    | Sum/diff, product, derivative, refinement     |
| Symbolic Ops  | Manipulate formulas, not sampled points              | Exact algebraic manipulation              | CAD kernel accuracy, rational math engine     |

---

## 소스 코드
```rust
pub use crate::core::basis::{Side, on_basis_ders_at_span, on_basis_funs, on_find_span_left_right};
use crate::core::basis::{on_basis_funs_ret_vec, on_find_span_left_eval};
pub use crate::core::knot::{on_ensure_param_in_knot_domain};
pub use crate::core::errors::NurbsError;
pub use crate::core::functions::on_compute_knots_for_global_interpolation;
pub use crate::core::matrix::{on_solve_linear_system_dense, on_solve_linear_system_dense_mut_flat};
pub use crate::core::prelude::*;
```
```rust

#[derive(Debug, Clone)]
pub struct CFun {
    pub p: Degree,
    pub knots: KnotVector,
    pub fu: Vec<Real>,
}
```
```rust

impl CFun {
    pub fn new(p: Degree, knots: KnotVector, fu: Vec<Real>) -> Result<Self> {
        knots.check_degree_vs_cp(p, fu.len())?;
        Ok(Self { p, knots, fu })
    }
```
```rust
    /// Evaluate the B-spline function at parameter `u`.
    pub fn eval(&self, u: Param, side: Side) -> Result<Real> {
        on_ensure_param_in_knot_domain(&self.knots, u)?;
        let p = self.p as usize;
        let span = on_find_span_left_eval(&self.knots, self.p, u)? as usize;
        let ders = 0usize;
        // Use basis functions (not derivatives) at the span.
        let n = on_basis_funs_ret_vec(&self.knots.as_slice(), span, u, self.p as usize);
        debug_assert_eq!(n.len(), p + 1);
        let mut v = 0.0;
        // control index range: (span-p)..=span
        let start = span.saturating_sub(p);
        for j in 0..=p {
            v += n[j] * self.fu[start + j];
        }
        Ok(v)
    }
```
```rust
    /// Symbolic derivative function (exact control-value derivative).
    ///
    pub fn derivative_function(&self, d: usize) -> Result<CFun> {
        let p = self.p as usize;
        if d > p {
            return Err(NurbsError::InvalidArgument { msg: "derivative order exceeds degree".into() });
        }
        if d == 0 {
            return Ok(self.clone());
        }

        let n = self.fu.len() - 1;
        let nd = n - d;
        let pd = (p - d) as Degree;

        // Copy control values into a working buffer (like ft[] in the C code).
        let mut ft = self.fu.clone();
        let u = self.knots.as_slice();

        for k in 1..=d {
            for i in 0..=(n - k) {
                let denom = u[i + p + 1] - u[i + k];
                if denom == 0.0 {
                    return Err(NurbsError::NumericalIssue { msg: "zero knot span during derivative".into() });
                }
                let alpha = ((p - k + 1) as Real) / denom;
                ft[i] = alpha * (ft[i + 1] - ft[i]);
            }
        }

        let mut fu_d = vec![0.0; nd + 1];
        fu_d.copy_from_slice(&ft[0..=nd]);

        // Build derivative knot vector V:
        // [U0 repeated (pd+1)] + U[p+1..=n] + [Um repeated (pd+1)]
        let u0 = u[0];
        let um = u[u.len() - 1];
        let mut v = Vec::with_capacity((nd + 1) + (pd as usize) + 1);
        for _ in 0..=(pd as usize) { v.push(u0); }
        for i in (p + 1)..=n { v.push(u[i]); }
        for _ in 0..=(pd as usize) { v.push(um); }

        Ok(CFun::new(pd, KnotVector::new(v)?, fu_d)?)
    }
```
```rust
    /// Refine this curve function by inserting knots from `knx`.
    ///
    /// The `knx` knot vector is interpreted as the list of knots to insert.
    pub fn refine_with_knot_vector(&self, knx: &KnotVector) -> Result<CFun> {
        let x = knx.as_slice();
        if x.is_empty() {
            return Ok(self.clone());
        }

        let p = self.p as usize;
        let up = self.knots.as_slice();
        let fp = &self.fu;

        let r = (x.len() as i32) - 1;
        if r < 0 {
            return Ok(self.clone());
        }

        // Basic endpoint checks: new knots must lie inside (U0, Um)
        let u0 = up[0];
        let um = up[up.len() - 1];
        if x[0] <= u0 || x[x.len() - 1] >= um {
            return Err(NurbsError::InvalidArgument { msg: "refine knots must satisfy U0 < X < Um".into() });
        }

        let n = fp.len() - 1;
        let m = up.len() - 1;
        let rr = x.len() - 1;

        // Output sizes follow the C routine: n+r+1 control values, m+r+1 knots.
        let nq = n + rr + 1;
        let mq = m + rr + 1;
        let mut fq = vec![0.0; nq + 1];
        let mut uq = vec![0.0; mq + 1];

        // Find knot spans (LEFT) for X[0] and X[r]
        let a = on_find_span_left_eval(&self.knots, self.p, x[0])? as usize;
        let mut b = on_find_span_left_eval(&self.knots, self.p, x[rr])? as usize;
        b += 1;

        // Initialize output knot vector
        for j in 0..=a { uq[j] = up[j]; }
        for j in (b + p)..=m { uq[j + rr + 1] = up[j]; }

        // Save unaltered control values
        for j in 0..=(a - p) { fq[j] = fp[j]; }
        for j in (b - 1)..=n { fq[j + rr + 1] = fp[j]; }

        // Now refine
        let mut i = b + p - 1;
        let mut k = b + p + rr;
        for j in (0..=rr).rev() {
            while x[j] <= up[i] && i > a {
                fq[k - p - 1] = fp[i - p - 1];
                uq[k] = up[i];
                k -= 1;
                i -= 1;
            }

            fq[k - p - 1] = fq[k - p];
            for l in 1..=p {
                let t = k - p + l;
                let mut alf = uq[k + l] - x[j];
                if alf.abs() <= 0.0 {
                    fq[t - 1] = fq[t];
                } else {
                    alf = alf / (uq[k + l] - up[i - p + l]);
                    let oma = 1.0 - alf;
                    fq[t - 1] = oma * fq[t] + alf * fq[t - 1];
                }
            }
            uq[k] = x[j];
            k -= 1;
        }

        Ok(CFun::new(self.p, KnotVector::new(uq)?, fq)?)
    }
```
```rust
    /// Global interpolation for a curve function (scalar B-spline), using the standard
    /// interpolation matrix solve.
    pub fn global_interpolate(params: &[Real], values: &[Real], degree: Degree) -> Result<CFun> {
        if params.len() != values.len() {
            return Err(NurbsError::InvalidArgument { msg: "params/values length mismatch".into() });
        }
        if params.len() < 2 {
            return Err(NurbsError::InvalidArgument { msg: "need at least 2 samples".into() });
        }

        let knots = on_compute_knots_for_global_interpolation(params, degree);
        let p = degree as usize;
        let n = params.len() - 1;

        // Build interpolation matrix A (size (n+1)x(n+1))
        let mut a = vec![0.0; (n + 1) * (n + 1)];
        let mut b = values.to_vec();
        for (row, &u) in params.iter().enumerate() {
            let span = on_find_span_left_eval(&knots, degree, u)? as usize;
            let n_vals = on_basis_funs_ret_vec(&knots.as_slice(), span, u, degree as usize);
            let start = span.saturating_sub(p);
            for j in 0..=p {
                let col = start + j;
                a[row * (n + 1) + col] = n_vals[j];
            }
        }

        on_solve_linear_system_dense_mut_flat(&mut a, &mut b, n + 1);
        CFun::new(degree, knots, b)
    }
```
```rust
    /// Symbolic product of two univariate B-spline functions.
    ///
    /// Rust port of the classic `N_SYMPFF` routine.
    ///
    /// Implementation strategy (robust + simple):
    /// 1) merge knot breakpoints (with tolerance) and refine both inputs so their knot vectors match;
    /// 2) further refine each input so every internal knot has full multiplicity (= its degree),
    ///    turning each span into a Bezier segment;
    /// 3) multiply corresponding Bezier segments and assemble an output Bezier-segmented B-spline.
    pub fn sym_product(&self, other: &CFun, knot_tol: Real) -> Result<CFun> {
        use crate::core::bezier_curve::on_bezier_function_product_range;
        use crate::core::knots_extensions::on_merge_knot_vectors_with_tolerance;

        let p = self.p;
        let q = other.p;
        let pq = p + q;

        // Handle constant-degree special cases cheaply.
        if p == 0 {
            // self is constant on its single control value.
            let c = self.fu[0];
            let mut out = other.clone();
            for v in &mut out.fu {
                *v *= c;
            }
            return Ok(out);
        }
        if q == 0 {
            let c = other.fu[0];
            let mut out = self.clone();
            for v in &mut out.fu {
                *v *= c;
            }
            return Ok(out);
        }

        // 1) Merge breakpoints and refine so both share the same knot vector (up to tolerance).
        let merged = on_merge_knot_vectors_with_tolerance(
            &mut [self.knots.clone(), other.knots.clone()],
            knot_tol,
        )?;
        let mut f = if merged[0].len() > 0 { self.refine_with_knot_vector(&merged[0])? } else { self.clone() };
        let mut g = if merged[1].len() > 0 { other.refine_with_knot_vector(&merged[1])? } else { other.clone() };

        // Sanity: after refinement, knot vectors should match (within tolerance we already applied).
        if f.knots.len() != g.knots.len() {
            return Err(NurbsError::InvalidState { msg: "sym_product: knot vectors mismatch after refinement".into() });
        }
        if f.knots.as_slice() != g.knots.as_slice() {
            return Err(NurbsError::InvalidState { msg: "sym_product: knot vectors not identical after refinement".into() });
        }

        // Helper: build insertion list to make every internal knot full multiplicity (=deg).
        fn insertion_to_bezierize(kv: &KnotVector, deg: Degree) -> Vec<Real> {
            let u = kv.as_slice();
            if u.len() < 2 { return vec![]; }
            let p = deg as usize;
            let m = u.len() - 1;
            let mut ins = Vec::<Real>::new();
            let mut i = p;
            while i < m.saturating_sub(p) {
                // start of a run
                let val = u[i];
                let mut j = i;
                while j < m && u[j + 1] == val { j += 1; }
                let mult = j - i + 1;
                if i > p && j < m - p {
                    // internal knot
                    if mult < p {
                        for _ in 0..(p - mult) {
                            ins.push(val);
                        }
                    }
                }
                i = j + 1;
            }
            ins
        }

        // 2) Bezierize both.
        let ins_f = insertion_to_bezierize(&f.knots, f.p);
        if !ins_f.is_empty() {
            f = f.refine_with_knot_vector(&KnotVector::new(ins_f)?)?;
        }
        let ins_g = insertion_to_bezierize(&g.knots, g.p);
        if !ins_g.is_empty() {
            g = g.refine_with_knot_vector(&KnotVector::new(ins_g)?)?;
        }

        // After full multiplicity insertion, breakpoints remain identical.
        // Count spans from the (common) knot vector.
        let u = f.knots.as_slice();
        let p_usize = f.p as usize;
        let m = u.len() - 1;
        let mut breaks: Vec<Real> = Vec::new();
        // unique breakpoints in the valid domain
        breaks.push(u[p_usize]);
        for i in (p_usize + 1)..=(m - p_usize) {
            if u[i] != u[i - 1] {
                breaks.push(u[i]);
            }
        }
        // breaks contains [start, ..., end]
        if breaks.len() < 2 {
            return Err(NurbsError::InvalidState { msg: "sym_product: invalid knot vector".into() });
        }
        let nsp = breaks.len() - 1;

        // Assemble output (Bezier segmented):
        // control count = nsp * pq + 1
        let mut h_ctrl = vec![0.0; nsp * (pq as usize) + 1];
        for s in 0..nsp {
            let f0 = s * (p as usize);
            let g0 = s * (q as usize);
            let f_seg = &f.fu[f0..(f0 + p as usize + 1)];
            let g_seg = &g.fu[g0..(g0 + q as usize + 1)];
            let h_seg = on_bezier_function_product_range(f_seg, p as usize, g_seg, q as usize, 0, pq as usize);

            let h0 = s * (pq as usize);
            h_ctrl[h0..(h0 + pq as usize + 1)].copy_from_slice(&h_seg);
        }

        // Build output knot vector with full multiplicity pq at internal breaks.
        let mut h_knots: Vec<Real> = Vec::new();
        let a = breaks[0];
        let b = *breaks.last().unwrap();
        for _ in 0..=(pq as usize) { h_knots.push(a); }
        for bi in breaks.iter().skip(1).take(breaks.len() - 2) {
            for _ in 0..(pq as usize) { h_knots.push(*bi); }
        }
        for _ in 0..=(pq as usize) { h_knots.push(b); }

        Ok(CFun::new(pq, KnotVector { knots : h_knots }, h_ctrl)?)
    }
}
```
```rust
impl CFun {
    /// d-th derivative of a curve function (scalar B-spline).
    /// Matches the classic control-value differencing formula.
    pub fn derivative(&self, d: usize) -> Result<CFun> {
        let p = self.p as usize;
        if d > p {
            return Err(NurbsError::InvalidArgument {
                msg: "CFun derivative order exceeds degree".into(),
            });
        }
        let n = self.fu.len().checked_sub(1).ok_or_else(|| NurbsError::InvalidArgument {
            msg: "CFun has no control values".into(),
        })?;

        let u = self.knots.as_slice();
        let mut tmp = self.fu.clone();

        for k in 1..=d {
            let pk = (p - (k - 1)) as Real;
            for i in 0..=(n - k) {
                let denom = u[i + p + 1] - u[i + k];
                if denom == 0.0 {
                    return Err(NurbsError::InvalidArgument {
                        msg: "zero knot interval in CFun derivative".into(),
                    });
                }
                let alf = pk / denom;
                tmp[i] = alf * (tmp[i + 1] - tmp[i]);
            }
        }

        let nd = n - d;
        let pd = (p - d) as i32;

        // Knot vector per N_SYMFDR:
        // V = [U0 repeated (p-d+1), U[p+1..n] , Um repeated (p-d+1)]
        let m = self.knots.len() - 1;
        let mut v = Vec::with_capacity((nd + (pd as usize) + 2));
        for _ in 0..=(p - d) { v.push(u[0]); }
        for i in (p + 1)..=n { v.push(u[i]); }
        for _ in 0..=(p - d) { v.push(u[m]); }

        let knots = KnotVector::new(v)?;
        Ok(CFun::new(pd as Degree, knots, tmp[..=nd].to_vec())?)
    }
```
```rust
    /// refine a curve function with an insertion list (sorted, may contain duplicates).
    /// This is a scalar version of curve refinement (same structure as the C code you pasted).
    pub fn refine_with_insert_list(&self, insert_vec: &[Real]) -> Result<CFun> {
        if insert_vec.is_empty() {
            return Ok(self.clone());
        }

        // local notation
        let p = self.p as usize;
        let n = self.fu.len() - 1;
        let up = self.knots.as_slice();
        let m = up.len() - 1;

        // X must be within (end knots)
        let x = insert_vec;
        let r = x.len() - 1;
        if r < 0_usize {
            return Err(NurbsError::InvalidArgument { msg: "empty insert_vec".into() });
        }

        // Allocate output
        let mut fq = vec![0.0; n + r + 2];        // (n+r+1)+1
        let mut uq = vec![0.0; m + r + 2];        // (m+r+1)+1

        // find spans a,b in LEFT sense
        // Use existing span finder in knot.rs if you have it; otherwise a simple linear scan works.
        let find_span_left = |u: Real| -> Result<usize> {
            // span k such that U[k] <= u < U[k+1], clamped
            if u <= up[p] { return Ok(p); }
            if u >= up[n+1] { return Ok(n); }
            for k in p..=n {
                if u >= up[k] && u < up[k + 1] { return Ok(k); }
            }
            Ok(n)
        };

        let a = find_span_left(x[0])?;
        let mut b = find_span_left(x[r])? + 1;

        // init output knot vector
        for j in 0..=a { uq[j] = up[j]; }
        for j in (b + p)..=m { uq[j + r + 1] = up[j]; }

        // save unaltered control values
        for j in 0..=(a.saturating_sub(p)) { fq[j] = self.fu[j]; }
        for j in (b.saturating_sub(1))..=n { fq[j + r + 1] = self.fu[j]; }

        // refinement (reverse insert)
        let mut i = b + p - 1;
        let mut k = b + p + r;

        for j in (0..=r).rev() {
            while x[j] <= up[i] && i > a {
                fq[k - p - 1] = self.fu[i - p - 1];
                uq[k] = up[i];
                k -= 1;
                i -= 1;
            }

            fq[k - p - 1] = fq[k - p];

            for l in 1..=p {
                let t = k - p + l;
                let mut alf = uq[k + l] - x[j];
                if alf == 0.0 {
                    fq[t - 1] = fq[t];
                } else {
                    alf = alf / (uq[k + l] - up[i - p + l]);
                    let oma = 1.0 - alf;
                    fq[t - 1] = oma * fq[t] + alf * fq[t - 1];
                }
            }
            uq[k] = x[j];
            k -= 1;
        }

        Ok(CFun::new(p as Degree, KnotVector::new(uq)?, fq)?)
    }
```
```rust
    #[allow(unused)]
    fn knots_insertions_for_bezier_degree(&self, degree: usize) -> KnotVector {
        let u = self.knots.as_slice();
        if u.len() < 2 { return KnotVector { knots: Vec::new() }; }
        let a = u[0];
        let b = *u.last().unwrap();
        let mut x = Vec::<f64>::new();
        let mut i = 0usize;
        while i < u.len() {
            let val = u[i];
            let mut j = i + 1;
            while j < u.len() && u[j] == val { j += 1; }
            let mult = j - i;
            if val > a && val < b {
                if mult < degree {
                    for _ in 0..(degree - mult) { x.push(val); }
                }
            }
            i = j;
        }
        // NOTE: This KnotVector is used as an *insertion list*; it is allowed to be empty.
        KnotVector { knots: x }
    }
}
```
```rust
pub fn cfun_derivatives(cfn: &CFun, u: Param, side: Side, der: usize) -> Result<Vec<Real>> {
    on_ensure_param_in_knot_domain(&cfn.knots, u)?;
    let p = cfn.p as usize;
    let span = on_find_span_left_right(&cfn.knots, cfn.p, u, side)?;

    let bd = on_basis_ders_at_span(&cfn.knots, p, u, span, der);
    let i0 = span - p;

    let mut out = vec![0.0; der + 1];
    for k in 0..=der {
        let mut s = 0.0;
        for j in 0..=p {
            s += cfn.fu[i0 + j] * bd[k][j];
        }
        out[k] = s;
    }
    Ok(out)
}
```

---
