# Symbloic 연산
- Symbolic 연산의 의미를 제대로 이해하면, NURBS 커널이 왜 이렇게 복잡한지,  
    그리고 왜 이 symbolic 단계가 핵심인지가 한 번에 정리됨.
- 아래는 **수학적 의미 → NURBS에서의 역할 → 왜 중요한가** 순서로 깊게 설명.

## 🌟 1. Symbolic 연산의 수학적 의미
- ✔ 1) **함수 자체를 다루는 연산**
- 일반적인 수치 연산(numerical)은 값(value) 을 다룬다.
- 예:
    - f(0.3) 계산
    - 곡선의 한 점을 평가
    - 미분값을 수치적으로 근사
- 반면 symbolic 연산(symbolic computation) 은  
    함수의 구조 자체를 다루는 연산이다.
- 즉,
    - **값을 계산하는 것** 이 아니라 **함수의 표현식을 변환하는 것**.
- 예:
    - 두 함수의 곱 F(u)G(u) 를 새로운 B-spline/NURBS로 표현
    - 함수의 분자/분모를 분리하여 symbolic rational form 구성
    - knot refinement를 통해 동일한 함수지만 다른 표현으로 변환
    - basis 변환 (Bezier ↔ B-spline)
    - degree elevation
    - reparameterization
- 이런 것들은 모두 함수의 표현식을 조작하는 symbolic 연산이다.

## 🌟 2. NURBS에서 symbolic 연산이 필수적인 이유
- NURBS는 다음 구조를 가진다:
```math
C(u)=\frac{N(u)}{D(u)}
```
- 여기서
    - N(u): B-spline numerator
    - D(u): B-spline denominator
    - 둘 다 piecewise polynomial
- 즉, NURBS는 rational + piecewise + polynomial 구조를 가진다.
- 이 구조 때문에 다음 연산들이 필요해진다:

- ✔ 1) Rational 연산을 polynomial 연산으로 바꾸기 위해
- 예를 들어 곡률(curvature) 계산:
```math
C'(u)=\frac{N'D-ND'}{D^2}
```
- 여기서 필요한 symbolic 연산:
    - $N'(u)$ (미분)
    - $D'(u)$
    - $N'D$ (곱)
    - $ND'$
    - $D^2$ (제곱)
    - 마지막에 다시 rational form으로 정리
- 즉, 모든 rational 연산은 polynomial symbolic 연산의 조합이다.

- ✔ 2) B-spline 곱셈은 symbolic 연산 없이는 불가능
- 두 B-spline 함수의 곱:
```math
F(u)G(u)
```
- 이걸 다시 B-spline으로 표현하려면:
    - knot merge
    - refine
    - Bezier decomposition
    - Bezier product (symbolic)
    - knot removal
    - 재조합
- 이 전체 과정이 symbolic pipeline이다.

- ✔ 3) 형상 보존(shape preservation)을 위해 NURBS는 어떤 연산을 하든 형상이 바뀌면 안 된다.
- 예:
    - reparameterization
    - knot refinement
    - degree elevation
    - knot merge
    - numerator/denominator 분리
- 이런 연산들은 모두:
```math
F(u)=F'(u)
```
- 을 만족해야 한다.
- 즉, symbolic 연산은 형상을 유지하면서 표현만 바꾸는 기술이다.

- ✔ 4) 수치적 안정성을 위해
- NURBS는 수치적으로 매우 민감하다.
    - basis가 좁아짐
    - knot multiplicity 증가
    - rational 구조
    - high-degree polynomial
    - floating-point cancellation
- 이런 문제를 피하려면 symbolic 변환을 통해 안정적인 형태로 바꿔야 한다.
- 예:
    - numerator/denominator 분리
    - Bezier product matrix
    - knot refinement
    - reparameterization
- 이런 symbolic 연산이 없으면 수치적 폭발이 일어난다.

## 3. Symbolic 연산이 중요한 이유 (핵심 요약)

| 이유 | 설명 |
|------|------|
| 1. 함수의 구조를 직접 다룸 | 값(value)이 아니라 함수 표현식을 변환하는 연산 |
| 2. NURBS의 rational 구조 처리 | 모든 rational 연산은 polynomial symbolic 연산의 조합 |
| 3. 형상 보존 (Shape Preservation) | 표현만 바뀌고 곡선/곡면의 형상은 그대로 유지 |
| 4. 수치 안정성 확보 | floating-point 오차 누적을 symbolic 변환으로 방지 |
| 5. 고급 연산의 기반 | 곱셈, 미분, 적분, 곡률, 최적화 등 모든 고급 기능의 핵심 |
| 6. B-spline의 piecewise 구조 처리 | Bezier 분해, knot merge, refine 등 필수 전처리 |
| 7. NURBS 커널의 두뇌 역할 | 전체 NURBS 엔진의 핵심 알고리즘을 구성 |



## 🌟 4. symbolic kernel이 완성되면:
- NURBS 곱셈
- rational 미분
- 곡률
- 적분
- 최적화
- 곡선/곡면 연산
- 이 모든 고급 기능이 자연스럽게 따라온다.

## 🌟 5. 결론
- Symbolic 연산은 NURBS의 **수학적 심장** 이다.
- rational 구조를 polynomial로 다루기 위해
- 형상을 보존하기 위해
- 수치적 안정성을 위해
- 고급 연산을 가능하게 하기 위해
- B-spline의 piecewise 구조를 다루기 위해

---
