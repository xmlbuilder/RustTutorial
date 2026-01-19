# evaluate_rational_basis_and_derivatives
- 이 함수 evaluate_rational_basis_and_derivatives는 NURBS 엔진에서 가장 중요한 핵심 루틴 중 하나.
- 왜냐하면:
  - Rational basis function $R_i(u)$
  - 그리고 그 모든 도함수 $R_i^{(k)}(u)$
- 를 정확하게 계산하는 함수이기 때문.
- 코드의 구조는 다음과 같아:
  - Numerator:
```math
A_k=\frac{d^k}{du^k}N_i(u)
```
  - (B-spline basis function의 도함수)
- Denominator:
```math
d_k=\frac{d^k}{du^k}W(u)
```
- (가중치 함수의 도함수)
- Rational basis function:
```math
R_i(u)=\frac{w_iA_0}{d_0}
```
- Higher derivatives:
- Pascal triangle을 이용한 일반적인 quotient rule:
```math
R^{(k)}=\frac{1}{d_0}\left( w_iA_k-\sum _{j=1}^k{k \choose j}d_jR^{(k-j)}\right)
```
---

