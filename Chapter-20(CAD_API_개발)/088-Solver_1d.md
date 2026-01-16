# Solver 1D

## 🔎 Solver 1D 수학적 원리
### 1. 뉴턴 방법 (Newton-Raphson)
- 반복식:

$$
t_{n+1}=t_n-\frac{f(t_n)}{f'(t_n)}
$$


- 장점: 빠른 수렴 (근처에서 2차 수렴)
- 단점: 도함수가 0에 가까우면 불안정

### 2. 이분법 (Bisection)
- 브라켓 [a,b]에서 f(a)\cdot f(b)<0이면 근 존재
- 반복식:

$$
m=\frac{a+b}{2},\quad f(m)\mathrm{\  평가\  후\  부호에\  따라\  구간\  축소}
$$


- 장점: 항상 수렴
- 단점: 느림 (선형 수렴)

### 3. 브렌트 방법 (Brent’s Method)
- 뉴턴, 이분법, 보간법을 혼합하여 안정성과 속도를 동시에 확보
- 극값 찾기(최대/최소):
- 후보 구간 [a,b,c]에서 함수값 비교
- 보간식으로 새로운 후보 u 계산
- 반복적으로 갱신하여 수렴

### 4. 주기적 구간 (Periodic Interval)
- 구간 [min,max]에서 값이 벗어나면 wrap-around:

```math
x\mapsto x+k\cdot (max-min),\quad \mathrm{단\  }min\leq x\leq max
```

### 5. 다항식 해법
- 2차 방정식:
```math
ax^2+bx+c=0
```

- 판별식:
```math
\Delta =b^2-4ac
```
- 3차 방정식:
```math
ax^3+bx^2+cx+d=0
```
  - 카르다노 공식 사용:
  
  ```math
  p=\frac{3ac-b^2}{3a^2},\quad q=\frac{2b^3-9abc+27a^2d}{27a^3}
  \Delta =\left( \frac{q}{2}\right) ^2+\left( \frac{p}{3}\right) ^3
  ```

## 함수 정리

| 함수/메서드                   | 기능 설명                                      |
|-------------------------------|-----------------------------------------------|
| LocalSolve1D::solve           | 뉴턴 반복으로 근을 찾고, 필요 시 Brent/Bisection 보정 |
| LocalSolve1D::brent_refine    | 브렌트 알고리즘으로 극값(최대/최소) 정밀화           |
| LocalSolve1D::bisection_refine| 브라켓 구간에서 이분법으로 근 정밀화                 |
| Interval1D::periodic_wrap     | 주기 구간 내 값으로 wrap-around 처리               |
| RealRootSolver::solve_quadratic | 2차 방정식 해 구하기                           |
| RealRootSolver::solve_cubic     | 3차 방정식 해 구하기                           |
| EvalFunction1D::evaluate      | 함수 f(t), 도함수 f'(t) 평가                     |



## 🧠 소스가 복잡한 이유
### 1. 수렴 실패 대비 보정 로직 포함
- solve()는 단순 뉴턴 반복만 하는 게 아니라:
- 브라켓 업데이트
- 브렌트 보정 (최대/최소)
- 이분법 보정
- 경계 처리
- 주기 wrap
- 즉, 수렴 실패를 대비한 모든 전략을 한 함수에 통합하고 있어요.
### 2. 상태 추적을 위한 반복 상태 관리
- st_prev2, st_prev1, st_cur 등 3단계 상태를 유지하면서:
- 진동/정체 감지
- 극값 판단
- 브렌트 보정 조건 확인
- 이게 없으면 극값 보정이 불가능.
### 3. 경계 및 주기 처리
- Interval1D를 통해:
- 값이 구간 밖으로 나가면 clamp 또는 wrap
- 경계 반복 횟수 초과 시 종료
- 이 부분도 수치 안정성을 위한 필수 처리입니다.
### 4. 다양한 종료 조건
- TerminationReason1D는 다음을 구분:
- Converged
- Close
- OutOfBounds
- BadJacobian
- UnableToConverge
- 즉, 단순히 “안 됨”이 아니라 왜 안 됐는지를 명확히 기록합니다.
### 5. 단일 진입점 설계
- solve() 하나로 모든 알고리즘을 처리하게 설계되어 있어서:
- 외부에서 알고리즘을 바꿔도 인터페이스는 동일
- 내부에서 알고리즘 분기 (BrentMaximize, BrentMinimize, Bisection, Newton)
- 이건 API 일관성을 위한 전략적 선택이에요.

## ✅ 요약
| 구성 요소                  | 역할 설명                                      |
|---------------------------|------------------------------------------------|
| LocalSolve1D::solve       | 뉴턴 반복으로 근 탐색, 실패 시 Brent/Bisection 보정 |
| LocalSolve1D::brent_refine| 극값(최대/최소) 정밀화 알고리즘 (Brent 방식)         |
| LocalSolve1D::bisection_refine | 브라켓 구간에서 이분법으로 근 정밀화             |
| Interval1D::periodic_wrap | 주기 구간 내 값으로 wrap-around 처리               |
| RealRootSolver::solve_quadratic | 2차 방정식 해 구하기                         |
| RealRootSolver::solve_cubic     | 3차 방정식 해 구하기                         |
| EvalFunction1D::evaluate  | 함수 $f(t)$, 도함수 


- Solver1D::solve()는 단순한 반복 함수가 아니라, 답을 찾기 위해 여러 전략을 내부적으로 시도하는 복합 알고리즘.  
- 핵심은 **어떻게든 답을 찾는다** 는 목표에 집중한 구조입니다.

## 🔁 내부에서 시도하는 전략들
| 단계                     | 전략 또는 함수                             | 설명 또는 수식                                |
|--------------------------|--------------------------------------------|-----------------------------------------------|
| 초기 평가                | `evaluate(t₀)`                             | 함수값 f(t₀), 도함수 f′(t₀), 수렴 여부 평가     |
| 뉴턴 반복                | `solve()` 내부                             | $t_{n+1} = t_n - \frac{f(t_n)}{f'(t_n)}$   |
| 브렌트 보정              | `brent_refine()`                           | 극값 후보 구간에서 정밀 보정                   |
| 이분법 보정              | `bisection_refine()`                       | 브라켓 구간에서 부호 변화 기반 보정            |
| 주기 wrap 처리           | `periodic_wrap()`                          | 구간 밖 값 → 주기적으로 감싸기                 |
| 종료 판단                | `TerminationReason1D`                      | Converged, Close, OutOfBounds 등 종료 사유 기록 |


## 🎯 왜 이렇게 여러 시도를 하나요?
- 실제 함수는 예측 불가능: 도함수가 0에 가까울 수도 있고, 진동하거나 경계에 걸릴 수도 있음.
- 수치 안정성 확보: 단일 알고리즘만 쓰면 실패율이 높아짐 → 다양한 전략을 조합
- 자동 보정: 사용자가 직접 알고리즘을 바꾸지 않아도 내부에서 상황에 따라 적절한 방법을 선택

## ✅ 요약
- solve()는 단순한 반복이 아니라 답을 찾기 위한 전략 모음집
- Newton → 실패 시 Brent/Bisection → 경계/주기 처리 → 종료 판단까지
- 이 모든 걸 하나의 함수에서 처리하는 이유는 사용자에게 단순한 인터페이스를 제공하면서도  
  내부적으로는 매우 똑똑하게 동작하기 위해서입니다


## 🔎 왜 Solver1D가 Point_Inversion에 적합한가?
- Newton 시도 → 실패 시 보정  
  - 좌표 변환 과정에서 Jacobian이 불안정하거나 0에 가까울 때 Newton은 잘 안 풀립니다.   
  - Solver1D는 자동으로 Brent/Bisection을 시도해 답을 찾습니다.  
- 구간 제약 및 주기 처리  
  - 좌표가 특정 구간(예: [0, 2π])에 있어야 할 때 Interval1D::periodic_wrap으로 안전하게 처리합니다.
- TerminationReason 기록  
  - 단순히 “실패”가 아니라 왜 실패했는지 알려주므로 디버깅과 안정성 확보에 유리합니다.
- 답을 찾는 데 집중  
  - 시간이 조금 더 걸리더라도, 여러 전략을 시도해서 결국 답을 찾아내는 구조라서 안정성이 높습니다.

## ⚖️ 장점 vs 단점
| 장점                                      | 단점                                      |
|-------------------------------------------|-------------------------------------------|
| 다양한 전략을 자동으로 시도하여 실패율 낮음 | Newton 단독보다 반복 횟수가 많아질 수 있음 |
| 주기 wrap, 브라켓, 보정 등 안전장치 풍부   | 복잡한 함수일수록 계산 비용 증가           |
| TerminationReason으로 원인 추적 가능       | 코드 구조가 길고 복잡해짐                  |
| 답을 찾는 데 집중 → 안정성 높음            | 속도보다 안정성을 우선시하여 시간이 더 걸릴 수 있음 |

## ✅ 요약
- Solver1D는 “답을 찾는 것”을 최우선 목표로 설계된 알고리즘 모음집입니다.
- Point_Inversion처럼 난해한 문제에서도 Newton → Brent → Bisection → Periodic Wrap을 거쳐  
  결국 해를 찾아내는 안정성이 강점입니다.
- 단점은 시간이 더 걸릴 수 있다는 점이지만, 정확한 답을 보장하는 안정성이 필요한 상황에서는 훨씬 유리합니다.

---

# EvalFunction1D

- Solver1D 전체 구조에서 핵심 함수 인터페이스는 바로 EvalFunction1D입니다.  
- 이 트레이트(인터페이스)가 있어야 LocalSolve1D가 어떤 함수 `f(t)` 와 그 도함수 `f'(t)` 를 평가할 수 있고,  
- `Newton/Brent/Bisection` 같은 알고리즘을 적용할 수 있습니다.

## 🔎 EvalFunction1D의 역할
```rust
pub trait EvalFunction1D {
    /// 반환값: Ok((f, fp, found)) / Err(())
    fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()>;
}
```

- 입력: 실수 t
- 출력:
  - $f(t)$: 함수값
  - $f'(t)$: 도함수 값
- found: 수렴 여부 힌트 (예: f(t)가 충분히 작으면 true)

### ✅ 왜 중요한가?
- Newton 방법은 $\frac{f(t)}{f'(t)}$ 를 써야 하므로 반드시 함수값과 도함수가 필요합니다.
- Brent/Bisection은 함수값만으로도 동작하지만, 도함수가 있으면 더 빠르고 안정적으로 수렴합니다.
- found 플래그는 “이미 해를 찾았다”는 힌트를 줘서 불필요한 반복을 줄여줍니다.

### 📌 예시
- 근 찾기: $f(t)=t^2-2,f'(t)=2t$
- Point_Inversion: $f(t)=d/dt(||C(t)-P||^2),f'(t)=d^2/dt^2(||C(t)-P||^2)$
- 최적화: $f(t)=-(t-1)^2+1,f'(t)=-2(t-1)$

- 👉 정리하면, Solver1D는 내부적으로 여러 전략을 시도하지만,    
    **그 모든 전략이 의존하는 함수 인터페이스는 EvalFunction1D** 입니다.
- 즉, 사용자가 어떤 문제를 풀고 싶든, 그 문제를 EvalFunction1D::evaluate로 정의해주면  
    Solver1D가 알아서 Newton → Brent → Bisection 등을 시도해 답을 찾아줍니다.

---

# 도함수 문제
- Solver1D에서 사용하는 도함수 값 `f'(t)` 은 내부에서 자동으로 차분법으로 구하지 않습니다.  
- 핵심은 EvalFunction1D 트레이트에 있습니다:

```rust
pub trait EvalFunction1D {
    fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()>;
}
```

- 여기서 반환하는 (f, fp, found) 중 fp가 바로 도함수 값입니다.
- 즉, 사용자가 직접 도함수를 정의해서 제공해야 Newton 방식이 제대로 작동합니다.

## 🔎 왜 내부에서 차분법을 쓰지 않나?
- 정확도 문제: 수치 미분(차분법)은 오차가 크고, 특히 근 근처에서 불안정합니다.
- 성능 문제: 매번 함수값을 여러 번 평가해야 하므로 계산 비용이 증가합니다.
- 유연성 확보: 사용자가 해석적 도함수를 제공하면 훨씬 빠르고 안정적입니다.

### ✅ 어떻게 해야 하나?
- Newton 기반으로 쓰려면 evaluate()에서 **함수값 f(t)와 도함수 f′(t)** 를 직접 계산해서 반환해야 합니다.
- 만약 도함수를 구하기 어려운 경우:
- SolverAlgorithm1D::Bisection이나 Brent 같은 도함수 불필요한 알고리즘을 선택하면 됩니다.
- 이 경우 fp는 더미 값(예: 0.0)으로 반환해도 됩니다.

## 📌 요약
- Solver1D는 내부에서 차분법으로 도함수를 구하지 않습니다.
- Newton을 쓰려면 사용자가 EvalFunction1D 에서 도함수를 직접 제공해야 합니다.
- 도함수를 제공하기 어려운 경우, Brent/Bisection 같은 도함수 불필요한 알고리즘을 선택하면 됩니다.

## 차분법
- 해석적 도함수를 알기 어려울 때는 **차분법(finite difference)** 으로 근사해서 EvalFunction1D에 구현할 수 있습니다.  


## 🧩 예제: 차분법으로 도함수 근사하기
- 여기서는 $f(t)=t^2-2$ 의 근을 찾는 문제를 예로 들겠습니다.
- 실제 도함수는 f'(t)=2t이지만, 이를 직접 쓰지 않고 중앙 차분법으로 근사합니다:

$$
f'(t)\approx \frac{f(t+h)-f(t-h)}{2h}
$$

### EvalFunction1D 구현
```rust
use nurbslib::core::solver_1d::EvalFunction1D;
struct FiniteDiffFunction;
impl EvalFunction1D for FiniteDiffFunction {
    fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()> {
        // 원래 함수 f(t) = t^2 - 2
        let f = t * t - 2.0;

        // 중앙 차분법으로 도함수 근사
        let h = 1.0e-6;
        let f_plus = (t + h) * (t + h) - 2.0;
        let f_minus = (t - h) * (t - h) - 2.0;
        let fp = (f_plus - f_minus) / (2.0 * h);

        // 수렴 여부 힌트
        let found = f.abs() < 1.0e-12;
        Ok((f, fp, found))
    }
}
```
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nurbslib::core::solver_1d::{Interval1D, LocalSolve1D};

    #[test]
    fn test_finite_diff_solver() {
        let mut f = FiniteDiffFunction;
        let interval = Interval1D::new(-10.0, 10.0);

        let (ok, root) = LocalSolve1D::new(&mut f)
            .set_solution_interval(interval)
            .set_desired_accuracy(1.0e-12)
            .set_acceptable_accuracy(1.0e-9)
            .set_max_iterations(100)
            .solve(1.0, 1.0e-9) // 초기 추정치 t=1
            .unwrap();

        assert!(ok, "solver did not converge");

        // 실제 근은 sqrt(2) ≈ 1.414213562...
        let expected = 2.0_f64.sqrt();
        assert!(
            (root - expected).abs() < 1.0e-6,
            "root={} expected={}",
            root,
            expected
        );
    }
}
```
- EvalFunction1D에서 도함수를 직접 제공하지 않고, 중앙 차분법으로 근사 가능.
- 이렇게 하면 해석적 도함수를 모르는 함수에도 Newton 기반 Solver1D를 적용할 수 있음.
- 테스트에서는 $t^2-2=0$ 의 근을 잘 찾아서 sqrt(2) 근사값을 얻습니다.


### ✅ 코드 스케치
```rust
/// Curve 기반 Point Inversion (이미 구현된 구조)
struct ClosestPointOnCurve<'a, C: Curve> {
    curve: &'a C,
    p: Vec2,
}
```
```rust
impl<'a, C: Curve> EvalFunction1D for ClosestPointOnCurve<'a, C> {
    fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()> {
        let c  = self.curve.point(t);          // C(t)
        let c1 = self.curve.derivative(t);     // C'(t)
        let c2 = self.curve.second_derivative(t); // C''(t)

        let r = c - self.p;
        let f  = 2.0 * r.dot(c1);
        let fp = 2.0 * (c1.dot(c1) + r.dot(c2));
        let found = f.abs() < 1.0e-14;

        Ok((f, fp, found))
    }
}
```
```rust
/// Surface 기반 Point Inversion (확장 구조)
struct ClosestPointOnSurface<'a, S: Surface> {
    surface: &'a S,
    p: Vec3,
}
```
```rust
impl<'a, S: Surface> ClosestPointOnSurface<'a, S> {
    fn evaluate(&self, u: f64, v: f64) -> (f64, Vec2, [[f64;2];2]) {
        let s   = self.surface.point(u, v);      // S(u,v)
        let su  = self.surface.derivative_u(u,v); // S_u
        let sv  = self.surface.derivative_v(u,v); // S_v
        let suu = self.surface.second_derivative_uu(u,v);
        let suv = self.surface.second_derivative_uv(u,v);
        let svv = self.surface.second_derivative_vv(u,v);

        let r = s - self.p;
        let f = r.norm2(); // 거리 제곱

        // Gradient (∂F/∂u, ∂F/∂v)
        let grad_u = 2.0 * r.dot(su);
        let grad_v = 2.0 * r.dot(sv);
        let grad = Vec2::new(grad_u, grad_v);

        // Hessian (Newton 2D용)
        let h11 = 2.0 * (su.dot(su) + r.dot(suu));
        let h12 = 2.0 * (su.dot(sv) + r.dot(suv));
        let h22 = 2.0 * (sv.dot(sv) + r.dot(svv));
        let hessian = [[h11, h12],[h12, h22]];

        (f, grad, hessian)
    }
}
```
## 🎯 요약
- Curve: EvalFunction1D로 충분 (단일 변수 t).
- Surface: (u,v) 두 변수를 다루므로 EvalFunction2D 같은 구조가 필요.
- Surface에서는 Gradient와 Hessian을 계산해 Newton 2D 최적화로 확장 가능.
- 이렇게 하면 Curve와 Surface 모두 “Point Inversion” 문제를 Solver 구조로 풀 수 있습니다.


## 🧪 테스트 코드 설명
### 테스트 모듈은 각 기능을 검증합니다:
- 뉴턴 방법 테스트
- QuadMinusTwo: $f(t)=t^2-2$, 근은 $\pm \sqrt{2}$
- SinFunction: $f(t)=\sin (t)$, 근은 $k\pi$ 
    - solve로 올바른 근을 찾는지 확인
- 브렌트 최대화 테스트
- ParabolaMax: $f(t)=-(t-1)^2+1$, 최대값은 $t=1,f=1$
    - brent_refine로 극값을 정확히 찾는지 확인
- 이분법 테스트
    - 구간 $[1,2]$ 에서 $\sqrt{2}$ 근을 감싸고 bisection_refine으로 수렴 확인
- RealRootSolver 테스트
    - 2차: $x^2-3x+2=0$ → 근 1,2
    - 2차 중근: $(x-1)^2=0$ → 근 1
    - 3차: $(x-1)(x-2)(x-3)=0$ → 근 1,2,3
    - 3차 퇴화: $a\approx 0$ → 2차로 처리

## ✅ 요약
- Solver1D는 뉴턴, 이분법, 브렌트 알고리즘을 조합하여 근과 극값을 안정적으로 찾음
- Interval1D는 구간 제약과 주기 wrap 기능 제공
- RealRootSolver는 2차/3차 방정식 해법을 수식 기반으로 구현
- 테스트 코드로 각 알고리즘의 정확성과 안정성을 검증


```rust
use std::f64;

/// 1D 솔버 알고리즘
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverAlgorithm1D {
    Newton,
    Bisection,
    BrentMaximize,
    BrentMinimize,
}
```
```rust
/// 종료 이유
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationReason1D {
    Converged,
    Close,
    OutOfBounds,
    BadJacobian,
    UnableToConverge,
}
```
```rust
/// t -> f, f'(t) 를 평가하는 인터페이스
///
/// C++
///   struct IEvalFunction {
///     virtual int evaluate(double t, double& f, double& fp, bool& found);
///   };
pub trait EvalFunction1D {
    /// 반환값: Ok((f, fp, found)) / Err(())  (에러코드는 필요하면 바꿔도 됨)
    fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()>;
}
```
```rust
/// 반복 상태 (C++ IterationState)
#[derive(Debug, Clone, Copy)]
struct IterationState1D {
    t: f64,
    f: f64,
    fp: f64,
}
```
```rust
impl IterationState1D {
    fn new() -> Self {
        Self { t: 0.0, f: 0.0, fp: 0.0 }
    }
}
``
```rust
/// 단순 구간 구조체 (C++ ON_Interval 대체)
#[derive(Debug, Clone, Copy)]
pub struct Interval1D {
    pub min: f64,
    pub max: f64,
}
```
```rust
impl Interval1D {
    pub fn new(a: f64, b: f64) -> Self {
        if a <= b {
            Self { min: a, max: b }
        } else {
            Self { min: b, max: a }
        }
    }

    #[inline]
    pub fn include(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    /// 주기 wrap (C++ ON_Interval::PeriodicWrap 비슷한 기능)
    pub fn periodic_wrap(&self, mut x: f64) -> f64 {
        let len = self.max - self.min;
        if len <= 0.0 {
            return self.min;
        }
        while x < self.min {
            x += len;
        }
        while x > self.max {
            x -= len;
        }
        x
    }
}
```
```rust
pub struct LocalSolve1D<'a, F: EvalFunction1D + ?Sized> {
    pub func: &'a mut F,

    use_interval: bool,
    interval: Interval1D,
    solver_algo: SolverAlgorithm1D,

    desired_acc: f64,
    acceptable_acc: f64,
    max_iters: usize,
    max_boundary_hits: usize,
    is_periodic: bool,

    // 브라켓
    has_lower_bracket: bool,
    lower_bracket_f: f64,
    lower_bracket_t: f64,
    has_upper_bracket: bool,
    upper_bracket_f: f64,
    upper_bracket_t: f64,

    // 반복 상태
    st_prev2: IterationState1D,
    st_prev1: IterationState1D,
    st_cur: IterationState1D,
    cur_iter: usize,

    // 결과/통계
    found_accuracy: f64,
    lower_bound_hits: usize,
    upper_bound_hits: usize,
    term_reason: TerminationReason1D,
}
```
```rust
impl<'a, F: EvalFunction1D + ?Sized> LocalSolve1D<'a, F> {
    /// func 참조를 받아 초기화 (구간/옵션은 기본값)
    pub fn new(func: &'a mut F) -> Self {
        Self {
            func,
            use_interval: false,
            interval: Interval1D::new(0.0, 1.0),
            solver_algo: SolverAlgorithm1D::Newton,
            desired_acc: 1.0e-12,
            acceptable_acc: 1.0e-9,
            max_iters: 100,
            max_boundary_hits: 2,
            is_periodic: false,

            has_lower_bracket: false,
            lower_bracket_f: 0.0,
            lower_bracket_t: 0.0,
            has_upper_bracket: false,
            upper_bracket_f: 0.0,
            upper_bracket_t: 0.0,

            st_prev2: IterationState1D::new(),
            st_prev1: IterationState1D::new(),
            st_cur: IterationState1D::new(),
            cur_iter: 0,

            found_accuracy: 0.0,
            lower_bound_hits: 0,
            upper_bound_hits: 0,
            term_reason: TerminationReason1D::Converged,
        }
    }
```
```rust
    /// 구간을 지정 (해는 이 구간 안이라고 가정)
    pub fn set_solution_interval(mut self, interval: Interval1D) -> Self {
        self.interval = interval;
        self.use_interval = true;
        self
    }
```
```rust
    pub fn set_periodic(mut self, periodic: bool) -> Self {
        self.is_periodic = periodic;
        self
    }
```
```rust
    pub fn set_solver_algorithm(mut self, algo: SolverAlgorithm1D) -> Self {
        self.solver_algo = algo;
        self
    }
```
```rust
    pub fn set_desired_accuracy(mut self, eps: f64) -> Self {
        self.desired_acc = eps;
        self
    }
```
```rust
    pub fn set_acceptable_accuracy(mut self, eps: f64) -> Self {
        self.acceptable_acc = eps;
        self
    }
``
```rust
    pub fn set_max_iterations(mut self, n: usize) -> Self {
        self.max_iters = n.max(1);
        self
    }
```
```rust
    pub fn set_max_boundary_hits(mut self, n: usize) -> Self {
        self.max_boundary_hits = n.max(1);
        self
    }
```
```rust
    #[inline]
    fn signed_like(a: f64, b: f64) -> f64 {
        if b >= 0.0 { a.abs() } else { -a.abs() }
    }
```
```rust
    pub fn found_accuracy(&self) -> f64 {
        self.found_accuracy
    }
```
```rust    
    pub fn termination_reason(&self) -> TerminationReason1D {
        self.term_reason
    }
```
```rust    
    pub fn lower_bound_hits(&self) -> usize {
        self.lower_bound_hits
    }
```
```rust    
    pub fn upper_bound_hits(&self) -> usize {
        self.upper_bound_hits
    }

    /// 브렌트 보정 (최소/최대)
    ///
    /// ax, bx, cx: 극값 후보 구간 (bx 주변)
    pub fn brent_refine(
        &mut self,
        ax: f64,
        bx: f64,
        cx: f64,
        tol: f64,
        maximize: bool,
    ) -> Result<(f64, f64), ()> {
        let mut a = if ax < cx { ax } else { cx };
        let mut b = if ax > cx { ax } else { cx };
        let mut w = bx;
        let mut v = bx;
        let mut x = bx;
        let mut fw = 0.0;
        let mut fv = 0.0;
        let mut fx = 0.0;
        let mut dw = 0.0;
        let mut dv = 0.0;
        let mut dx = 0.0;
        let mut e = 0.0;
        let mut d = 0.0;

        let (mut f_init, mut fp_init, mut found) = self.func.evaluate(x)?;
        if maximize {
            f_init = -f_init;
            fp_init = -fp_init;
        }
        fx = f_init;
        fw = f_init;
        fv = f_init;
        dx = fp_init;
        dw = fp_init;
        dv = fp_init;

        for _iter in 1..=self.max_iters {
            let xm = 0.5 * (a + b);
            let tol1 = tol * x.abs() + f64::EPSILON;
            let tol2 = 2.0 * tol1;

            if (x - xm).abs() <= (tol2 - 0.5 * (b - a)) {
                let out_f = if maximize { -fx } else { fx };
                return Ok((x, out_f));
            }

            if (e as f64).abs() > tol1 {
                let mut d1 = 2.0 * (b - a);
                let mut d2 = d1;

                if (dw - dx).abs() > 0.0 {
                    d1 = (w - x) * dx / (dx - dw);
                }
                if (dv - dx).abs() > 0.0 {
                    d2 = (v - x) * dx / (dx - dv);
                }

                let u1 = x + d1;
                let u2 = x + d2;

                let ok1 = (a - u1) * (u1 - b) > 0.0 && dx * d1 <= 0.0;
                let ok2 = (a - u2) * (u2 - b) > 0.0 && dx * d2 <= 0.0;
                let olde = e;
                e = d;

                if ok1 || ok2 {
                    if ok1 && ok2 {
                        d = if d1.abs() < d2.abs() { d1 } else { d2 };
                    } else if ok1 {
                        d = d1;
                    } else {
                        d = d2;
                    }

                    if d.abs() <= 0.5 * olde.abs() {
                        // accept
                    } else {
                        d = 0.5 * if dx >= 0.0 { a - x } else { b - x };
                        e = d;
                    }
                } else {
                    d = 0.5 * if dx >= 0.0 { a - x } else { b - x };
                    e = d;
                }
            } else {
                d = 0.5 * if dx >= 0.0 { a - x } else { b - x };
                e = d;
            }

            let u = if d.abs() >= tol1 {
                x + d
            } else {
                x + Self::signed_like(tol1, d)
            };

            let (mut fu, mut fpu, _) = self.func.evaluate(u)?;
            if maximize {
                fu = -fu;
                fpu = -fpu;
            }

            if fu <= fx {
                if u >= x { a = x; } else { b = x; }
                v = w; fv = fw; dv = dw;
                w = x; fw = fx; dw = dx;
                x = u; fx = fu; dx = fpu;
            } else {
                if u < x { a = u; } else { b = u; }
                if fu <= fw || (w - x).abs() < f64::EPSILON {
                    v = w; fv = fw; dv = dw;
                    w = u; fw = fu; dw = fpu;
                } else if fu <= fv || (v - x).abs() < f64::EPSILON ||
                    (v - w).abs() < f64::EPSILON {
                    v = u; fv = fu; dv = fpu;
                }
            }
        }

        // 반복 한계 → 현재 값 반환
        let out_f = if maximize { -fx } else { fx };
        Ok((x, out_f))
    }
```
```rust
    /// 이분법 보정 (브라켓 [lower_t, upper_t] 에서 f=0 근사)
    pub fn bisection_refine(
        &mut self,
        depth: usize,
        mut lower_t: f64,
        mut upper_t: f64,
        mut lower_f: f64,
        mut upper_f: f64,
    ) -> Result<(f64, f64), ()> {
        let mut mid_t = lower_t;
        let mut mid_f = lower_f;

        for _ in 0..depth {
            mid_t = 0.5 * (lower_t + upper_t);
            let (f_mid, _, _) = self.func.evaluate(mid_t)?;
            mid_f = f_mid;

            if lower_f * mid_f <= 0.0 {
                upper_t = mid_t;
                upper_f = mid_f;
            } else {
                lower_t = mid_t;
                lower_f = mid_f;
            }
        }

        Ok((mid_t, mid_f))
    }
```
```rust
    /// 메인 solve (뉴턴 + 브렌트/이분법 보정 포함)
    ///
    /// 반환: Ok((found, t_solution))
    pub fn solve(&mut self, guess_t: f64, acceptable_accuracy: f64) -> Result<(bool, f64), ()> {
        self.acceptable_acc = acceptable_accuracy;
        self.term_reason = TerminationReason1D::UnableToConverge;
        self.cur_iter = 0;
        self.lower_bound_hits = 0;
        self.upper_bound_hits = 0;
        self.has_lower_bracket = false;
        self.has_upper_bracket = false;

        self.st_prev2 = IterationState1D::new();
        self.st_prev1 = IterationState1D::new();
        self.st_cur = IterationState1D::new();

        // 초기 t
        let mut t0 = guess_t;
        if self.use_interval {
            if self.is_periodic {
                t0 = self.interval.periodic_wrap(t0);
            } else {
                t0 = t0.clamp(self.interval.min, self.interval.max);
            }
        }

        // 초기 평가
        let (f0, fp0, mut found) = self.func.evaluate(t0)?;
        self.st_cur.t = t0;
        self.st_cur.f = f0;
        self.st_cur.fp = fp0;
        self.found_accuracy = f0;

        if found || f0.abs() < self.desired_acc {
            self.term_reason = TerminationReason1D::Converged;
            return Ok((true, t0));
        }

        let mut best_abs = f0.abs();
        let mut stagnation_count = 0;

        for iter in 0..self.max_iters {
            self.cur_iter = iter;

            if self.st_cur.f.abs() < self.desired_acc {
                self.term_reason = TerminationReason1D::Converged;
                self.found_accuracy = self.st_cur.f;
                return Ok((true, self.st_cur.t));
            }

            // 브라켓 업데이트 (부호 변화)
            if self.st_cur.f < 0.0 {
                if !self.has_lower_bracket || self.st_cur.f.abs() < self.lower_bracket_f.abs() {
                    self.has_lower_bracket = true;
                    self.lower_bracket_f = self.st_cur.f;
                    self.lower_bracket_t = self.st_cur.t;
                }
            } else {
                if !self.has_upper_bracket || self.st_cur.f.abs() < self.upper_bracket_f.abs() {
                    self.has_upper_bracket = true;
                    self.upper_bracket_f = self.st_cur.f;
                    self.upper_bracket_t = self.st_cur.t;
                }
            }

            if self.has_lower_bracket && self.has_upper_bracket {
                if (self.upper_bracket_t - self.lower_bracket_t).abs() < f64::EPSILON {
                    self.term_reason = TerminationReason1D::Converged;
                    self.found_accuracy = self.st_cur.f;
                    return Ok((true, self.st_cur.t));
                }
            }

            // 뉴턴 스텝
            let mut fp = self.st_cur.fp;
            if fp.abs() <= f64::EPSILON {
                fp = if fp < 0.0 { -1.0 } else { 1.0 };
            }
            let mut d_t = -self.st_cur.f / fp;

            // 스텝 제한 (구간 사용 시)
            if self.use_interval {
                let mut guard = 0;
                let half = 0.5 * (self.interval.max - self.interval.min).abs();
                while d_t.abs() > half + f64::EPSILON {
                    d_t /= 1.98765432;
                    guard += 1;
                    if guard > 100 {
                        break;
                    }
                }
            }

            let mut new_t = self.st_cur.t + d_t;

            // 경계/주기 처리
            if self.use_interval && !self.interval.include(new_t) {
                if self.is_periodic {
                    new_t = self.interval.periodic_wrap(new_t);
                } else {
                    if new_t < self.interval.min {
                        new_t = self.interval.min;
                        self.lower_bound_hits += 1;
                        if self.lower_bound_hits > self.max_boundary_hits {
                            self.term_reason = TerminationReason1D::OutOfBounds;
                            return Ok((false, new_t));
                        }
                    }
                    if new_t > self.interval.max {
                        new_t = self.interval.max;
                        self.upper_bound_hits += 1;
                        if self.upper_bound_hits > self.max_boundary_hits {
                            self.term_reason = TerminationReason1D::OutOfBounds;
                            return Ok((false, new_t));
                        }
                    }
                }
            }

            // 스텝 적용
            self.st_prev2 = self.st_prev1;
            self.st_prev1 = self.st_cur;
            self.st_cur.t = new_t;

            let (f_new, fp_new, fa) = self.func.evaluate(self.st_cur.t)?;
            self.st_cur.f = f_new;
            self.st_cur.fp = fp_new;

            // 정체/진동 보호
            if self.st_cur.f.abs() >= best_abs {
                stagnation_count += 1;
                if stagnation_count == 4 && self.st_cur.f.abs() < self.acceptable_acc {
                    break;
                }
            } else {
                stagnation_count = 0;
                best_abs = self.st_cur.f.abs();
            }

            if fa {
                self.found_accuracy = self.st_cur.f;
                self.term_reason = TerminationReason1D::Converged;
                return Ok((true, self.st_cur.t));
            }

            // 브렌트 보정: 최대화
            if self.solver_algo == SolverAlgorithm1D::BrentMaximize && iter >= 1 {
                if self.st_cur.f > self.st_prev1.f && self.st_cur.f > self.st_prev2.f {
                    let a = self.st_prev1.t;
                    let c = self.st_prev2.t;
                    let b = self.st_cur.t;
                    let (t_b, f_b) = self.brent_refine(a, b, c, self.desired_acc, true)?;
                    self.found_accuracy = f_b;
                    self.term_reason = TerminationReason1D::Converged;
                    return Ok((true, t_b));
                }
            }

            // 브렌트 보정: 최소화
            if self.solver_algo == SolverAlgorithm1D::BrentMinimize && iter >= 1 {
                if self.st_cur.f < self.st_prev1.f && self.st_cur.f < self.st_prev2.f {
                    let a = self.st_prev1.t;
                    let c = self.st_prev2.t;
                    let b = self.st_cur.t;
                    let (t_b, f_b) = self.brent_refine(a, b, c, self.desired_acc, false)?;
                    self.found_accuracy = f_b;
                    self.term_reason = TerminationReason1D::Converged;
                    return Ok((true, t_b));
                }
            }

            // 수렴 정체 + 브라켓 보유 → 이분법 보정으로 탈출
            if self.st_cur.f.abs() >= self.st_prev1.f.abs()
                && self.st_cur.f * self.st_prev1.f > 0.0
            {
                if self.has_lower_bracket && self.has_upper_bracket {
                    break;
                }
                if d_t.abs() < f64::EPSILON {
                    let ok = self.st_cur.f.abs() < self.acceptable_acc;
                    self.found_accuracy = self.st_cur.f;
                    self.term_reason = TerminationReason1D::Converged;
                    return Ok((ok, self.st_cur.t));
                }
            }
        }

        // 종료 직전 세이프가드: 브라켓 있으면 이분법 한 번 수행
        if self.has_lower_bracket && self.has_upper_bracket {
            let (t_mid, f_mid) = self.bisection_refine(
                20,
                self.lower_bracket_t,
                self.upper_bracket_t,
                self.lower_bracket_f,
                self.upper_bracket_f,
            )?;
            self.st_cur.t = t_mid;
            self.st_cur.f = f_mid;
        }

        if self.st_cur.f.abs() < self.acceptable_acc
            || self.solver_algo == SolverAlgorithm1D::Bisection
        {
            self.found_accuracy = self.st_cur.f;
            self.term_reason = if self.st_cur.f.abs() < self.desired_acc {
                TerminationReason1D::Converged
            } else {
                TerminationReason1D::Close
            };
            return Ok((true, self.st_cur.t));
        }

        self.found_accuracy = self.st_cur.f;
        self.term_reason = TerminationReason1D::UnableToConverge;
        Ok((false, self.st_cur.t))
    }
}
```
```rust
// =====================================
// C++ ON_RealRootSolver → Rust 변환
// =====================================

pub struct RealRootSolver;

impl RealRootSolver {
    /// 2차 방정식 a x^2 + b x + c = 0 의 실근
    ///
    /// coef[0] = c, coef[1] = b, coef[2] = a  (C++와 동일)
    pub fn solve_quadratic(coef: [f64; 3], zero_tol: f64) -> Vec<f64> {
        let mut roots = Vec::new();

        let a = coef[2];
        let b = coef[1];
        let c = coef[0];

        if a.abs() < zero_tol {
            if b.abs() < zero_tol {
                return roots;
            }
            roots.push(-c / b);
            return roots;
        }

        if c.abs() < zero_tol {
            roots.push(0.0);
            if b.abs() >= zero_tol {
                roots.push(-b / a);
            }
            return roots;
        }

        if b.abs() < zero_tol {
            let v = -c / a;
            if v < 0.0 {
                return roots;
            }
            let r = v.abs().sqrt();
            roots.push(-r);
            roots.push(r);
            return roots;
        }

        let mut disc = b * b - 4.0 * a * c;
        let sqrt_eps = f64::EPSILON.sqrt();
        if disc.abs() <= (b * b * sqrt_eps).abs() {
            disc = 0.0;
        }

        if disc < 0.0 {
            return roots;
        }

        let sqrt_disc = disc.sqrt();
        let q = if b > 0.0 {
            -0.5 * (b + sqrt_disc)
        } else {
            -0.5 * (b - sqrt_disc)
        };

        let mut r0 = q / a;
        let mut r1 = c / q;

        if r0 > r1 {
            std::mem::swap(&mut r0, &mut r1);
        }

        if (r0 - r1).abs() < zero_tol {
            roots.push(r0);
        } else {
            roots.push(r0);
            roots.push(r1);
        }
        roots
    }

    /// 3차 방정식 a x^3 + b x^2 + c x + d = 0 의 실근
    ///
    /// coef[0] = d, coef[1] = c, coef[2] = b, coef[3] = a
    pub fn solve_cubic(coef: [f64; 4], zero_tol: f64) -> Vec<f64> {
        let mut roots = Vec::new();

        let a = coef[3];
        let b = coef[2];
        let c = coef[1];
        let d = coef[0];

        if a.abs() < zero_tol {
            // 퇴화 → 2차
            let quad = [d, c, b];
            return Self::solve_quadratic(quad, zero_tol);
        }

        let a_inv = 1.0 / a;
        let a2 = b * a_inv;
        let a1 = c * a_inv;
        let a0 = d * a_inv;

        let sq_A = a2 * a2;
        let p = (1.0 / 3.0) * (-1.0 / 3.0 * sq_A + a1);
        let q = (1.0 / 2.0) * (2.0 / 27.0 * a2 * sq_A - 1.0 / 3.0 * a2 * a1 + a0);

        let cb_p = p * p * p;
        let mut D = q * q + cb_p;

        if D.abs() < zero_tol {
            D = 0.0;
        }

        if D < 0.0 {
            // 세 개의 서로 다른 실근
            let phi = (q / (-cb_p).sqrt()).acos();
            let t = 2.0 * (-p).sqrt();
            let r0 = -1.0 / 3.0 * a2 + t * (phi / 3.0).cos();
            let r1 = -1.0 / 3.0 * a2 + t * ((phi + 2.0 * std::f64::consts::PI) / 3.0).cos();
            let r2 = -1.0 / 3.0 * a2 + t * ((phi + 4.0 * std::f64::consts::PI) / 3.0).cos();

            roots.push(r0);
            roots.push(r1);
            roots.push(r2);
        } else {
            // 하나 또는 두 개의 실근
            let sqrt_D = D.sqrt();
            let u = (-q + sqrt_D).cbrt();
            let v = (-q - sqrt_D).cbrt();

            let y1 = u + v;
            let r1 = y1 - a2 / 3.0;
            roots.push(r1);

            if (u - v).abs() < zero_tol {
                // 중복근
            } else {
                let re = -0.5 * y1 - a2 / 3.0;
                roots.push(re);
            }
        }

        roots
    }
}
```
----

## 🧪 테스트 코드: Solver1D 전략 흐름 극대화
```rust
#[cfg(test)]
mod strategic_flow_tests {
    use super::*;
    use std::f64::consts::PI;

    /// f(t) = sin(t) - 0.1
    /// - 뉴턴 방법은 도함수 cos(t)가 0 근처에서 불안정
    /// - 브라켓 구간 [-PI, PI]에서 부호 변화 존재
    /// - 따라서 Newton 실패 → 브라켓 → Bisection 보정 흐름을 모두 거침
    struct SinShifted;

    impl EvalFunction1D for SinShifted {
        fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()> {
            let f = t.sin() - 0.1;
            let fp = t.cos();
            let found = f.abs() < 1.0e-14;
            Ok((f, fp, found))
        }
    }
```
```rust
    #[test]
    fn test_solver_strategic_flow() {
        let mut f = SinShifted;
        let interval = Interval1D::new(-PI, PI);

        let mut solver = LocalSolve1D::new(&mut f)
            .set_solution_interval(interval)
            .set_desired_accuracy(1.0e-12)
            .set_acceptable_accuracy(1.0e-9)
            .set_max_iterations(100);

        // 초기 guess를 도함수가 거의 0인 지점 근처로 설정 (Newton 실패 유도)
        let guess = PI / 2.0; // cos(PI/2)=0 → Newton 불안정
        let (ok, root) = solver.solve(guess, 1.0e-9).unwrap();

        assert!(ok, "solver did not converge");
        // 실제 근은 arcsin(0.1) ≈ 0.100167421...
        let expected = 0.1001674211615598;
        assert!(
            (root - expected).abs() < 1.0e-7,
            "root={} expected={}",
            root,
            expected
        );

        println!(
            "Termination reason: {:?}, root found at t={}, f(t)≈{}",
            solver.termination_reason(),
            root,
            solver.found_accuracy()
        );
    }
}
```


## 🔎 이 테스트의 특징
- Newton 실패 유도: 초기 guess를 $t=\pi /2$ 로 잡아 도함수 $f'(t)=\cos (t)$ 가 0 → Newton 불안정.
- 브라켓 활용: 구간 $[-π, π]$ 에서 $f(t)=\sin (t)-0.1$ 은 부호 변화가 있으므로 브라켓 생성.
- Bisection 보정: Newton이 실패하면 브라켓 기반 이분법으로 안정적 수렴.
- TerminationReason 기록: Converged로 종료되며, 내부적으로 여러 전략이 발동됨.

## ✅ 요약
이 테스트는 Solver1D의 전략적 흐름을 모두 거치도록 설계되어 있습니다.  
즉, Newton → 실패 → 브라켓 → Bisection → Converged 과정을 실제로 검증할 수 있어요.

- Rust 예제: Solver1D로 Point_Inversion (곡선 최근접점) 구하기  
  - 아래 예제는 2D 원형 곡선 $C(t) = (cos t, sin t)$ 위에서 임의의 점 P에 대한 최근접점을 찾는  
    Point_Inversion을 Solver1D로 해결합니다.  
  - 핵심 아이디어는 거리 제곱 함수 $D(t) = C(t) - P^2$ 의 도함수 $g(t) = dD/dt = 2 (C(t) - P) · C'(t)$ 를  
    0으로 만드는 $t$ 를 찾는 1D 근찾기 문제로 바꾸는 것입니다.  
  - 도함수 $g'(t)$ 도 함께 제공하여 뉴턴 방법의 안정성을 높입니다.  
    구간은 $[0, 2π]$ 로 두고, 주기 wrap을 활성화합니다.

```rust
use std::f64::consts::PI;

// Solver1D에서 가져오는 트레이트/타입들 (사용 중인 모듈 경로에 맞게 조정)
use nurbslib::core::solver_1d::{EvalFunction1D, Interval1D, LocalSolve1D, SolverAlgorithm1D};

#[cfg(test)]
mod point_inversion_tests
{
    use std::f64::consts::PI;

    // Solver1D에서 가져오는 트레이트/타입들 (사용 중인 모듈 경로에 맞게 조정)
    use nurbslib::core::solver_1d::{EvalFunction1D, Interval1D, LocalSolve1D, SolverAlgorithm1D};

    /// 2D 벡터 유틸
    #[derive(Debug, Clone, Copy)]
    struct Vec2 { x: f64, y: f64 }
    impl Vec2 {
        fn new(x: f64, y: f64) -> Self { Self { x, y } }
        fn dot(self, other: Vec2) -> f64 { self.x * other.x + self.y * other.y }
        fn norm2(self) -> f64 { self.dot(self) }
    }

    fn curve(t: f64) -> Vec2 { Vec2::new(t.cos(), t.sin()) }
    fn curve_d1(t: f64) -> Vec2 { Vec2::new(-t.sin(), t.cos()) }
    fn curve_d2(t: f64) -> Vec2 { Vec2::new(-t.cos(), -t.sin()) }

    struct ClosestPointOnCurve { p: Vec2 }
    impl ClosestPointOnCurve { fn new(p: Vec2) -> Self { Self { p } } }
```
```rust
    impl EvalFunction1D for ClosestPointOnCurve {
        fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()> {
            let c = curve(t);
            let c1 = curve_d1(t);
            let c2 = curve_d2(t);
            let r = Vec2::new(c.x - self.p.x, c.y - self.p.y);

            let f = 2.0 * r.dot(c1);
            let fp = 2.0 * (c1.dot(c1) + r.dot(c2));
            let found = f.abs() < 1.0e-14;
            Ok((f, fp, found))
        }
    }
```
```rust
    #[test]
    fn brent_minimize() {
        let p = Vec2::new(1.2, 0.3);
        let mut f = ClosestPointOnCurve::new(p);
        let interval = Interval1D::new(0.0, 2.0 * PI);

        // 체이닝 방식으로 solver 사용
        let (ok, t_star) = LocalSolve1D::new(&mut f)
            .set_solution_interval(interval)
            .set_periodic(true)
            .set_desired_accuracy(1.0e-12)
            .set_acceptable_accuracy(1.0e-9)
            .set_max_iterations(100)
            .solve(p.y.atan2(p.x), 1.0e-9)
            .expect("solve failed");

        let c_star = curve(t_star);
        let dist2 = Vec2::new(c_star.x - p.x, c_star.y - p.y).norm2();

        println!("ok            = {}", ok);
        println!("t* (radians)  = {}", t_star);
        println!("C(t*)         = ({:.9}, {:.9})", c_star.x, c_star.y);
        println!("||C(t*)-P||^2 = {:.12}", dist2);

        // BrentMinimize로 재확인 (체이닝)
        let (t_min, _) = LocalSolve1D::new(&mut f)
            .set_solution_interval(interval)
            .set_periodic(true)
            .set_solver_algorithm(SolverAlgorithm1D::BrentMinimize)
            .brent_refine(interval.min, t_star, interval.max, 1.0e-10, false)
            .expect("brent refine failed");

        let c_min = curve(t_min);
        let dmin2 = Vec2::new(c_min.x - p.x, c_min.y - p.y).norm2();
        println!("Brent refine: t_min={}, D(t_min)={:.12}", t_min, dmin2);
    }
```
```rust
    #[test]
    fn point_inversion_on_circle() {
        let p = Vec2::new(1.2, 0.3);
        let interval = Interval1D::new(0.0, 2.0 * PI);

        // -------------------------------
        // 1) Newton 기반 solve (도함수=0)
        // -------------------------------
        let mut f_grad = ClosestPointOnCurve::new(p);

        let guess = p.y.atan2(p.x);
        
        let (ok, t_star) = LocalSolve1D::new(&mut f_grad)
            .set_solution_interval(interval)
            .set_periodic(true)
            .set_desired_accuracy(1.0e-12)
            .set_acceptable_accuracy(1.0e-9)
            .set_max_iterations(100)
            .solve(guess, 1.0e-9)
            .unwrap();

        assert!(ok, "solver did not converge");

        let c_star = curve(t_star);
        let dist2 = Vec2::new(c_star.x - p.x, c_star.y - p.y).norm2();
        assert!(dist2 >= 0.0);
        assert!(t_star >= 0.0 && t_star <= 2.0 * PI);

        // -----------------------------------------
        // 2) BrentMinimize 기반 최소화 (거리 제곱 직접)
        // -----------------------------------------
        struct DistanceSquared { p: Vec2 }
        impl EvalFunction1D for DistanceSquared {
            fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()> {
                let c = curve(t);
                let c1 = curve_d1(t);
                let r = Vec2::new(c.x - self.p.x, c.y - self.p.y);

                let f = r.norm2();             // 거리 제곱
                let fp = 2.0 * r.dot(c1);      // 도함수
                let found = fp.abs() < 1.0e-14;
                Ok((f, fp, found))
            }
        }

        let mut f_dist = DistanceSquared { p };
        let bx = t_star;
        let ax = interval.min;
        let cx = interval.max;
        let tol = 1.0e-10;

        let (t_min, f_min) = LocalSolve1D::new(&mut f_dist)
            .set_solution_interval(interval)
            .set_periodic(true)
            .set_solver_algorithm(SolverAlgorithm1D::BrentMinimize)
            .brent_refine(ax, bx, cx, tol, false)
            .unwrap();

        // -----------------------------------------
        // 3) 두 결과 비교
        // -----------------------------------------
        let delta = (t_min - t_star).abs();
        println!("t_star={}, t_min={}, f_min={}", t_star, t_min, f_min);

        // Newton과 Brent가 같은 최소점을 찾아야 함
        assert!(delta < 1.0e-5, "t_min={} vs t_star={}", t_min, t_star);
    }

}
```

## 설명
- 핵심 변환: Point_Inversion은 거리 제곱의 도함수가 0인 지점(국소 최소)을 찾는 문제입니다.  
    이는 EvalFunction1D에서 f, f'를 제공하면서 뉴턴을 적용할 수 있도록 해줍니다.
- 주기 처리: [0, 2π] 구간을 사용하고 set_periodic(true)로 각도 t가 구간을 벗어나도 안정적으로 wrap합니다.
- 보정 전략: 뉴턴이 불안정할 때 내부에서 브라켓/이분법/브렌트가 동원되어 답을 끝내 찾도록 돕습니다.  
    예제에서는 BrentMinimize로 거리 제곱 직접 최소화를 재확인하는 단계도 포함했습니다.  
    원 대신 일반 곡선 C(t)에도 동일하게 적용할 수 있습니다.  
    예를 들어 Bezier 곡선이라면 $C$, $C′$, $C″$ 를 정의하고 같은 방식으로 $f(t)$와 $f′(t)$ 를 구성하면 됩니다.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use nurbslib::core::solver_1d::{EvalFunction1D, Interval1D, LocalSolve1D, RealRootSolver};

    const EPS: f64 = 1.0e-8;

    // -----------------------------
    // LocalSolve1D 테스트용 함수들
    // -----------------------------

    /// f(t) = t^2 - 2, f'(t) = 2t
    /// root: ±sqrt(2)
    struct QuadMinusTwo;
```
```rust
    impl EvalFunction1D for QuadMinusTwo {
        fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()> {
            let f = t * t - 2.0;
            let fp = 2.0 * t;
            let found = f.abs() < 1.0e-14;
            Ok((f, fp, found))
        }
    }
```
```rust
    /// f(t) = sin(t), f'(t) = cos(t)
    /// root: kπ
    struct SinFunction;

    impl EvalFunction1D for SinFunction {
        fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()> {
            let f = t.sin();
            let fp = t.cos();
            let found = f.abs() < 1.0e-14;
            Ok((f, fp, found))
        }
    }
```
```rust
    /// f(t) = -(t-1)^2 + 1
    /// 최대값: t = 1, f(1) = 1
    struct ParabolaMax;

    impl EvalFunction1D for ParabolaMax {
        fn evaluate(&mut self, t: f64) -> Result<(f64, f64, bool), ()> {
            let f = -(t - 1.0) * (t - 1.0) + 1.0;
            let fp = -2.0 * (t - 1.0);
            // 극값 근처에서는 found = true 처리
            let found = fp.abs() < 1.0e-10;
            Ok((f, fp, found))
        }
    }
```
```rust
    // ---------------------------------
    // 1) 뉴턴 기반 LocalSolve1D: 간단한 근 찾기
    // ---------------------------------

    #[test]
    fn test_local_solve1d_newton_simple_quadratic() {
        let mut f = QuadMinusTwo;
        let interval = Interval1D::new(0.0, 4.0);

        let mut solver = LocalSolve1D::new(&mut f)
            .set_solution_interval(interval)
            .set_desired_accuracy(1.0e-12)
            .set_acceptable_accuracy(1.0e-9)
            .set_max_iterations(50);

        // sqrt(2) 근방에서 시작
        let guess = 1.0;
        let (ok, root) = solver.solve(guess, 1.0e-9).unwrap();

        assert!(ok, "solver did not converge");
        let expected = 2.0_f64.sqrt();
        assert!(
            (root - expected).abs() < 1.0e-7,
            "root={} expected={}",
            root,
            expected
        );
    }
```
```rust
    #[test]
    fn test_local_solve1d_newton_sin_near_zero() {
        let mut f = SinFunction;
        let interval = Interval1D::new(-PI, PI);

        let mut solver = LocalSolve1D::new(&mut f)
            .set_solution_interval(interval)
            .set_desired_accuracy(1.0e-12)
            .set_acceptable_accuracy(1.0e-9)
            .set_max_iterations(50);

        let guess = 0.3; // 0 근처
        let (ok, root) = solver.solve(guess, 1.0e-9).unwrap();
        assert!(ok, "solver did not converge");
        assert!(root.abs() < 1.0e-7, "root={} not close to 0", root);
    }
```
```rust
    // ---------------------------------
    // 2) 브렌트 Maximize 테스트
    // ---------------------------------
    #[test]
    fn test_brent_maximize_parabola() {
        let mut f = ParabolaMax;
        let mut solver = LocalSolve1D::new(&mut f);

        // 극값 근처의 세 점: 0,1,2
        let ax = 0.0;
        let bx = 1.0;
        let cx = 2.0;
        let tol = 1.0e-10;

        let (t_max, f_max) = solver
            .brent_refine(ax, bx, cx, tol, true)
            .expect("brent refine failed");

        assert!(
            (t_max - 1.0).abs() < 1.0e-6,
            "t_max={} expected=1.0",
            t_max
        );
        assert!(
            (f_max - 1.0).abs() < 1.0e-6,
            "f_max={} expected=1.0",
            f_max
        );
    }
```
```rust
    // ---------------------------------
    // 3) Bisection 보정 테스트
    // ---------------------------------

    #[test]
    fn test_bisection_refine_for_quadratic_root() {
        let mut f = QuadMinusTwo;
        let mut solver = LocalSolve1D::new(&mut f);

        // √2 근을 감싸는 브라켓 [1, 2]
        let lower_t = 1.0;
        let upper_t = 2.0;
        let (f1, _, _) = solver.func.evaluate(lower_t).unwrap();
        let (f2, _, _) = solver.func.evaluate(upper_t).unwrap();

        assert!(f1 * f2 < 0.0, "interval does not bracket root");

        let (tm, fm) = solver
            .bisection_refine(40, lower_t, upper_t, f1, f2)
            .expect("bisection refine failed");

        let expected = 2.0_f64.sqrt();
        assert!(
            (tm - expected).abs() < 1.0e-6,
            "tm={} expected={}",
            tm,
            expected
        );
        assert!(fm.abs() < 1.0e-6, "fm={} not close to 0", fm);
    }
```
```rust
    // ---------------------------------
    // RealRootSolver 테스트
    // ---------------------------------

    #[test]
    fn test_real_root_solver_quadratic_two_roots() {
        // x^2 - 3x + 2 = 0 → roots: 1, 2
        // coef[0]=c, coef[1]=b, coef[2]=a
        let coef = [2.0, -3.0, 1.0];
        let mut roots = RealRootSolver::solve_quadratic(coef, 1.0e-12);

        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(roots.len(), 2);
        assert!((roots[0] - 1.0).abs() < EPS, "root0 = {}", roots[0]);
        assert!((roots[1] - 2.0).abs() < EPS, "root1 = {}", roots[1]);
    }
```
```rust
    #[test]
    fn test_real_root_solver_quadratic_single_root() {
        // (x-1)^2 = x^2 - 2x + 1 = 0 → root: 1 (중근)
        let coef = [1.0, -2.0, 1.0];
        let roots = RealRootSolver::solve_quadratic(coef, 1.0e-12);

        // 구현에 따라 1개 또는 2개일 수 있지만, 모두 1 근처여야 한다.
        assert!(!roots.is_empty(), "no roots returned");
        for r in roots {
            assert!((r - 1.0).abs() < EPS, "r={} not near 1", r);
        }
    }
```
```rust
    #[test]
    fn test_real_root_solver_cubic_three_real_roots() {
        // (x-1)(x-2)(x-3) = x^3 - 6x^2 + 11x - 6 = 0
        // coef[0]=d, coef[1]=c, coef[2]=b, coef[3]=a
        let coef = [-6.0, 11.0, -6.0, 1.0];
        let mut roots = RealRootSolver::solve_cubic(coef, 1.0e-12);

        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!(roots.len() >= 3, "expected 3 real roots, got {:?}", roots);

        // 앞에서 3개만 사용 (중복/근접값이 있을 수 있음)
        let r0 = roots[0];
        let r1 = roots[1];
        let r2 = roots[2];

        assert!((r0 - 1.0).abs() < 1.0e-6, "r0 = {}", r0);
        assert!((r1 - 2.0).abs() < 1.0e-6, "r1 = {}", r1);
        assert!((r2 - 3.0).abs() < 1.0e-6, "r2 = {}", r2);
    }
```
```rust
    #[test]
    fn test_real_root_solver_cubic_degenerate_to_quadratic() {
        // a≈0 → 2차로 퇴화
        // 2x^2 - 5x + 2 = 0 → roots: (5 ± sqrt(25-16))/4 = (5 ± 3)/4 = 0.5, 2
        let coef = [2.0, -5.0, 2.0, 1.0e-16]; // 아주 작은 a
        let mut roots = RealRootSolver::solve_cubic(coef, 1.0e-12);
        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!(roots.len() >= 2, "expected at least 2 roots, got {:?}", roots);
        assert!((roots[0] - 0.5).abs() < 1.0e-6, "r0 = {}", roots[0]);
        assert!((roots[1] - 2.0).abs() < 1.0e-6, "r1 = {}", roots[1]);
    }
}
```
---
