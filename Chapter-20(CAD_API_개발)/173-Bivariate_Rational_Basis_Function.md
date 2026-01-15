# bivariate rational basis function

- **bivariate rational basis function** 이라는 말은 NURBS 곡면(surface)에서 핵심 개념,  
- 이걸 정확히 이해하면 NURBS surface 평가, 미분, 곡률, symbolic 연산까지 전부 자연스럽게 풀린다.
- 아래에서 수식 → 의미 → 왜 필요한가 순서로 깔끔하게 정리.

## 1. **Bivariate** 의 수학적 의미
- **bivariate** 는 말 그대로 두 개의 변수(variable) 를 가진다는 뜻.
    - univariate: 변수 1개 → u
    - bivariate: 변수 2개 → u,v
- 즉, bivariate basis function은:
```math
N(u,v)
```
- 처럼 두 변수에 의존하는 함수를 말한다.
- NURBS 곡선은 univariate, NURBS 곡면(surface)은 bivariate 구조를 가진다.

## 2. Bivariate B-spline basis function의 수식
- NURBS surface의 기본은 tensor product B-spline basis다.
- 두 개의 univariate B-spline basis:
```math
N_{i,p}(u),\quad M_{j,q}(v)
```
- 이 있으면, 곡면의 bivariate basis는 다음과 같이 정의된다:
```math
B_{i,j}(u,v)=N_{i,p}(u)\, M_{j,q}(v)
```
- 즉, 두 개의 1D basis를 곱해서 2D basis를 만든 것이다.
- 이게 바로 **bivariate** 의 핵심.

## 3. Rational bivariate basis function
- NURBS는 rational 구조를 가지므로,
- bivariate rational basis는 다음과 같이 정의된다:
```math
R_{i,j}(u,v)=\frac{N_{i,p}(u)\, M_{j,q}(v)\, w_{i,j}}{\displaystyle \sum _{a=0}^{n_u}\sum _{b=0}^{n_v}N_{a,p}(u)\, M_{b,q}(v)\, w_{a,b}}
```
- 여기서:
    - $w_{i,j}$: control point weight
    - 분모는 전체 basis의 가중합 → normalization 역할
- 즉, rational bivariate basis는:
    - 두 방향의 B-spline basis를 곱하고
    - weight를 곱한 뒤
    - 전체 weight 합으로 나누어 정규화한 것

## 4. 의미적으로 해석하면
### 1) 곡면(surface)은 2D parameter domain을 가진다
- 곡선은 u 하나만 필요하지만, 곡면은 (u,v)라는 2D 좌표가 필요하다.
- 그래서 basis도 2D가 된다.
### 2) tensor product 구조
- bivariate basis는 사실상 다음과 같은 의미를 가진다:
    - u 방향으로 p-degree B-spline
    - v 방향으로 q-degree B-spline
    - 두 방향의 영향이 독립적이면서 결합됨
- 그래서 곡면의 한 점은:
```math
S(u,v)=\sum _{i,j}R_{i,j}(u,v)\, P_{i,j}
```
- 이렇게 계산된다.
### 3) rational 구조는 곡면을 더 유연하게 만든다
- rational이 아니면 원뿔, 구, 토러스 같은 곡면을 정확히 표현할 수 없다.
- rational bivariate basis는 이런 곡면을 정확하게 표현할 수 있게 해준다.

## 5. Evaluate bivariate rational basis function의 의미
- **bivariate rational basis function을 평가한다** 는 말은:
    - 주어진 (u,v)에서
    - $R_{i,j}(u,v)$ 값을 계산한다는 뜻이다.
- 즉, 곡면의 한 점을 계산하기 위해 각 basis의 기여도를 구하는 과정.
- 이 과정은 다음 단계로 이루어진다:
    - $N_{i,p}(u)$ 계산
    - $M_{j,q}(v)$ 계산
    - 곱해서 $N_{i,p}(u)M_{j,q}(v)$
    - weight 곱하기
    - 전체 weight 합으로 나누기

## 6. 왜 중요한가?
- bivariate rational basis는 NURBS surface의 모든 연산의 핵심이다.
    - surface point evaluation
    - surface derivative
    - surface normal
    - curvature
    - trimming
    - subdivision
    - symbolic numerator/denominator 분리
    - surface-surface intersection
    - surface refinement
    - surface degree elevation
- 이 모든 연산이 결국 bivariate rational basis를 정확히 계산하는 것에서 출발한다.

## 7. 요약
- bivariate rational basis function은:
    - 두 변수 u,v를 가지며
    - 두 방향의 B-spline basis를 곱한 뒤
    - weight로 rational 정규화한 함수
- 수식:
```math
R_{i,j}(u,v)=\frac{N_{i,p}(u)\, M_{j,q}(v)\, w_{i,j}}{\displaystyle \sum _{a,b}N_{a,p}(u)\, M_{b,q}(v)\, w_{a,b}}
```
- 의미:
    - NURBS surface의 한 점을 결정하는 “기여도 함수”
    - 2D parameter domain에서 작동
    - rational 구조로 정확한 기하 표현 가능

---
