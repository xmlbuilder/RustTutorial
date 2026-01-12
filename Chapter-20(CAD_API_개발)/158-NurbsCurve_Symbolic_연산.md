# Symbolic  연산

- 이번 세 개는 **“symbolic 연산 계열”** 이라 수식적 의미를 잡아두면, 이후 커널 설계할 때 엄청 도움이 된다.

## 1. sym_sum_difference — 두 NURBS 곡선의 합/차 (rational 지원)
```rust
/// sum/difference of two curves (handles rational via num/den)
pub fn sym_sum_difference(cur_p: &NurbsCurve, cur_q: &NurbsCurve, plus: bool, tol: Real) -> Result<NurbsCurve>
```

### 1.1 기능 요약
- 두 곡선 C_p(u),C_q(u)에 대해
- plus = true  → $C_{\mathrm{out}}(u)=C_p(u)+C_q(u)$
- plus = false → $C_{\mathrm{out}}(u)=C_p(u)-C_q(u)$
- Rational / non-rational 혼합도 지원:
- 둘 중 하나라도 rational이면 num/den 분해 후 정확한 rational 합/차 계산
- 둘 다 non-rational이면 degree align + knot refinement 후 control point-wise sum/diff

### 1.2 Rational 케이스 수식
- 두 rational curve:
```math
C_p(u)=\frac{N_p(u)}{D_p(u)},\quad C_q(u)=\frac{N_q(u)}{D_q(u)}
```
- 여기서
    - $N_p(u)$, $N_q(u)$: numerator curve (벡터값)
    - $D_p(u)$, $D_q(u)$: scalar denominator function
- 합/차:
```math
C_{\mathrm{out}}(u)=C_p(u)\pm C_q(u)=\frac{N_p}{D_p}\pm \frac{N_q}{D_q}=\frac{N_pD_q\pm N_qD_p}{D_pD_q}
```
- 코드 흐름이 정확히 이 수식을 구현:
- rational 여부 판단:
```rust
let rat = cur_p.is_rational() || cur_q.is_rational();
```

- 각 곡선을 (num, den)으로 변환
    - 이미 rational이면: extract_num_den()
    - non-rational이면: numerator = 자기 자신, denominator = constant 1
- 수식적으로:
    - non-rational C(u)는 \frac{C(u)}{1}로 표현
    - 공통 denominator:
```math
D(u)=D_p(u)\cdot D_q(u)
```
```rust
let den = den_p.sym_product(&den_q, tol)?;
```

- numerator:
```math
N_{\mathrm{out}}(u)=\left\{ \, \begin{array}{ll}\textstyle N_pD_q+N_qD_p&\textstyle (\mathrm{plus\  =\  true})\\ \textstyle N_pD_q-N_qD_p&\textstyle (\mathrm{plus\  =\  false})\end{array}\right. 
```
```rust
let t1 = NurbsCurve::sym_product_function_curve(&den_q, &num_p, tol)?;
let t2 = NurbsCurve::sym_product_function_curve(&den_p, &num_q, tol)?;
let num = NurbsCurve::sym_sum_difference(&t1, &t2, plus, tol)?;
```

- 최종 rational curve 생성:
```rust
return NurbsCurve::from_num_den(&num, Some(&den));
```


### 1.3 Non-rational 케이스 수식
- 두 non-rational B-spline 곡선:
```math
C_p(u)=\sum _iN_i^p(u)P_i,\quad C_q(u)=\sum _iN_i^p(u)Q_i
```
- degree align + knot refinement 후에는 동일한 basis 함수 $N_i^p(u)$ 집합을 공유하게 된다.
- 그 상태에서:
```math
C_{\mathrm{out}}(u)=\sum _iN_i^p(u)(P_i\pm Q_i)
```
- 즉,
    - control point-wise sum/diff로 구현 가능.
- 코드:
- degree align:
```rust
if p.degree != q.degree {
    if p.degree < q.degree { p.degree_elevate(...); } else { q.degree_elevate(...); }
}
```

- knot 벡터 병합 후 refine:
```rust
let mut kvs = vec![p.kv.clone(), q.kv.clone()];
let inserts = on_merge_knot_vectors_with_tolerance(&mut kvs, tol)?;
p.refine_knot_vector(inserts[0].knots.as_slice());
q.refine_knot_vector(inserts[1].knots.as_slice());
```

- control point-wise 연산:
```rust
if plus {
    out.ctrl[i] += q.ctrl[i];
} else {
    out.ctrl[i] -= q.ctrl[i];
}
```


### 1.4 용도
- 곡선 간 벡터 합/차: offset curve, blending, deformation 조합
- rational/non-rational 섞인 상태에서도 정확한 연산 가능
- symbolic 연산 기반 고급 CAD 기능의 building block

## 2. sym_derivative_rational — rational 곡선의 1차 도함수
```rust
/// first derivative curve of a rational curve.
pub fn sym_derivative_rational(cur: &NurbsCurve, tol: Real) -> Result<NurbsCurve>
```

### 2.1 기능 요약
- Rational NURBS curve  의 1차 도함수 곡선 C'(u)를 계산한다.
- Non-rational이면 그냥 sym_derivative_non_rational(1)로 위임.

### 2.2 수식
- 기본:
```math
C(u)=\frac{N(u)}{D(u)}
```
- 미분하면:
```math
C'(u)=\frac{N'(u)D(u)-N(u)D'(u)}{D(u)^2}
```
- 코드가 이걸 그대로 구현하고 있다.
- numerator / denominator 분해:
```rust
let (n, d) = cur.extract_num_den()?;
```
- denominator 제곱:
```math
D_{\mathrm{out}}(u)=D(u)^2
```
```rust
let den = d.sym_product(&d, tol)?;
```

- 도함수들:
```math
N'(u),\quad D'(u)
```
```rust
let np = n.sym_derivative_non_rational(1)?;
let dp = d.derivative(1)?;
```

- numerator:
```math
N'(u)D(u)-N(u)D'(u)
```
```rust
let npd = NurbsCurve::sym_product_function_curve(&d, &np, tol)?;
let ndp = NurbsCurve::sym_product_function_curve(&dp, &n, tol)?;
let mut num = NurbsCurve::sym_sum_difference(&npd, &ndp, false, tol)?;
```

- legacy degree elevate:
```rust
num.degree_elevate(1);
```

- 이는 기존 코드와 형식을 맞추기 위한 호환성 조치.  
    (수식적으로는 필수는 아니지만, basis 차수와 표현력을 확보하기 위한 것.)
- 결과 curve 생성:
```rust
let mut out = NurbsCurve::from_num_den(&num, Some(&den))?;
out.domain = cur.domain;
```


### 2.3 용도
- Rational curve의 정확한 기하학적 도함수 곡선
- tangent, curvature, normal 계산의 기반
- 고급 기하 연산(예: curvature matching, fairing, fillet 설계 등)

## 3. sym_derivative_non_rational — 비유리 곡선의 d차 도함수
```rust
/// d-th derivative of a **non-rational** curve.
pub fn sym_derivative_non_rational(&self, d: usize) -> Result<NurbsCurve>
```

### 3.1 기능 요약
- 비유리(non-rational) NURBS 곡선의 d차 도함수 곡선을 만든다.
- Piegl & Tiller의 control point differencing 공식을 d회 반복 적용.

### 3.2 수식 (반복 도함수)
- 원 곡선:
```math
C(u)=\sum _{i=0}^nN_i^p(u)P_i
```
- 1차 도함수:
```math
C'(u)=\sum _{i=0}^{n-1}N_i^{p-1}(u)P_i^{(1)}
```
- 여기서
```math
P_i^{(1)}=\frac{p}{U_{i+p+1}-U_{i+1}}(P_{i+1}-P_i)
```
- d차 도함수는 이 공식을 k = 1..d 만큼 반복 적용하는 것:
```math
P_i^{(k)}=\frac{p-(k-1)}{U_{i+p+1}-U_{i+k}}\left( P_{i+1}^{(k-1)}-P_i^{(k-1)}\right)
``` 
- 최종적으로:
    - degree: p_d=p-d
    - control point 개수: n_d=n-d

### 3.3 코드 검증
- 핵심 루프:
```rust
for k in 1..=d {
    let pk = (p - (k - 1)) as Real;
    for i in 0..=(n - k) {
        let denom = u[i + p + 1] - u[i + k];
        let alf = pk / denom;
        tmp[i] = alf*(tmp[i+1] - tmp[i]);
    }
}
```

- 이는 수식:
```math
\alpha =\frac{p-(k-1)}{U_{i+p+1}-U_{i+k}}
```
```math
P_i^{(k)}=\alpha (P_{i+1}^{(k-1)}-P_i^{(k-1)})
```
- 과 정확히 일치한다.

### 3.4 knot vector 재구성
- 도함수 후 degree p_d=p-d, control point 개수 n_d=n-d.
- clamped form에서 사용하는 knot는:
```math
V=[U_0\mathrm{\  반복\  }(p_d+1),\; U_{p+1},\ldots ,U_n,\; U_m\mathrm{\  반복\  }(p_d+1)]
```
- 코드:
```rust
for _ in 0..=(p - d) { v.push(u[0]); }
for i in (p + 1)..=n { v.push(u[i]); }
for _ in 0..=(p - d) { v.push(u[m]); }
```

- 도메인 전달:
```rust
out.domain = self.domain;
```


### 3.5 용도
- 비유리 곡선의 고차 도함수 곡선 생성
- 곡률, torsion, 고급 분석
- higher-order continuity 조건 충족, matching, blending

## 전체 요약 (Markdown 테이블)
| Function                       | Mathematical Definition / Formula                                              | Purpose |
|--------------------------------|---------------------------------------------------------------------------------|---------|
| sym_sum_difference             | For rational: Cout = (Np Dq ± Nq Dp) / (Dp Dq). For non-rational: CP-wise sum/diff after degree & knot alignment. | Curve sum/difference (rational-aware) |
| sym_derivative_rational        | C(u) = N/D ⇒ C'(u) = (N' D − N D') / D²                                       | First derivative of a rational NURBS curve |
| sym_derivative_non_rational    | d-th derivative via repeated $P_i^{(k)} = α(P_{i+1}^{(k−1)} − P_i^{(k−1)})$, $α = (p−k+1)/(U[i+p+1] − U[i+k])$ | d-th derivative of non-rational curve |


---


## 🎯 symbolic의 의미 — 핵심 한 줄 요약
- 수치적으로 샘플링해서 계산하는 게 아니라,  
    곡선의 **수식 구조 자체** 를 직접 다루는 연산을 symbolic이라고 부른다.
- 즉:
    - 점을 찍어서 계산하는 것 → numerical
    - 수식/기저함수/제어점/weight를 직접 조작하는 것 → symbolic


## 🎨 비유적 설명
- ✔ Numerical 방식
    - **곡선을 1000개 점으로 샘플링해서, 그 점들을 더하거나 빼거나 미분한다.**
- 즉, 결과는 점들의 집합
    - 정확도는 샘플링에 따라 달라짐
    - CAD 커널에서는 절대 이렇게 안 함
- ✔ Symbolic 방식
    - **곡선을 정의하는 수식 자체를 변형한다.**
- 예:
    - control point를 직접 조합
    - weight를 조합
    - knot vector를 조합
    - basis function을 조합
    - numerator/denominator를 조합
    - 도함수 공식 그대로 적용
- 즉, 곡선의 기하학적 정의를 그대로 유지한 채 연산
    - 정확도 100%
    - CAD 커널에서 반드시 필요한 방식

## 🔥 예시로 이해해보자
### 1) 두 곡선을 더하는 경우
- Numerical 방식
    ```rust
    for u in 0..1 step 0.001:
        C_out(u) = C_p(u) + C_q(u)
    ```
    - 결과는 “점들의 집합”
    - 곡선이 아님
    - CAD에서는 쓸 수 없음

- Symbolic 방식
- Bezier/NURBS 수식 자체를 조합해서 새로운 NURBS 곡선으로 만든다.
    - 결과는 정확한 NURBS 곡선
    - CAD 커널에서 요구하는 방식

### 2) Rational 곡선의 도함수
- Numerical 방식
    ```math
    C'(u) ≈ (C(u+h) - C(u)) / h
    ```
    - 근사값
    - 정확도 떨어짐
- Symbolic 방식
    ```math
    C(u) = N(u)/D(u)
    ```
    ```math
    C'(u) = (N' D - N D') / D²
    ```

    - 완전한 기하학적 정확성
    - CAD 커널에서 필수

### 📌 그래서 함수 이름이 sym_... 인 이유
- sym_sum_difference
    - 곡선의 수식적 합/차를 만든다
- sym_derivative_rational
    - rational 곡선의 수식적 도함수를 만든다
- sym_derivative_non_rational
    - 비유리 곡선의 수식적 도함수를 만든다
- 즉,
    - 곡선을 점으로 샘플링하지 않고,
    - 곡선의 정의 자체를 조작하는 연산이기 때문에
    - symbolic이라는 이름이 붙은  것임.

### 📘 CAD 커널에서 symbolic이 중요한 이유
- 정확도 100%
- 곡선/곡면의 기하학적 성질 유지
- trimming, intersection, offset, blending 등에서 필수
- 수치적 오차 누적 방지
- 산업용 CAD(Parasolid, ACIS, OpenCascade) 모두 symbolic 기반


### 🎉 symbolic의 의미 요약
- symbolic = 곡선을 점으로 계산하지 않고,  
    곡선을 정의하는 수식(제어점, weight, knot)을 직접 조작하는 방식.



