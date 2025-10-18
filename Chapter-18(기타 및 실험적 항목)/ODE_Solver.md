# ODE(Ordinary Differential Equation) 수치해석 방법 비교: RK4 vs RK45
## ⚙️ RK4 (고전적 4차 Runge-Kutta)
- 고정 스텝 방식: 시간 간격(Δt)을 일정하게 유지
- 정확도: 4차 정확도 (오차 ~ O(h^4))
- 장점:
- 구현이 간단하고 안정적
- 대부분의 비강성 ODE에 충분히 정확함
- 단점:
- 스텝 크기를 자동으로 조절하지 않음 → 급격한 변화가 있는 시스템에 비효율적


### 📐 RK4의 slope 계산식
```
k₁ = f(tₙ, yₙ)
k₂ = f(tₙ + ½·h, yₙ + ½·k₁·h)
k₃ = f(tₙ + ½·h, yₙ + ½·k₂·h)
k₄ = f(tₙ + h, yₙ + k₃·h)
yₙ₊₁ = yₙ + (1/6)·h·(k₁ + 2·k₂ + 2·k₃ + k₄)
```

### 🔄 RK4 계산 흐름도
```
tₙ ──► k₁ ──┐
             ├─► k₂ (tₙ + ½·h, yₙ + ½·k₁·h) ──┐
             │                                ├─► k₃ (tₙ + ½·h, yₙ + ½·k₂·h) ──┐
             │                                │                                ├─► k₄ (tₙ + h, yₙ + k₃·h)
             │                                │                                │
             │                                │                                ▼
             │                                │                          Combine slopes:
             │                                │                          yₙ₊₁ = yₙ + (1/6)·h·(k₁ + 2·k₂ + 2·k₃ + k₄)
             │                                │                                │
             └─────────────────────────────────────────────────────────────────► tₙ₊₁
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use crate::rk4::Rk4;
    use crate::{DVector, OVector, System, Vector1};
    use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

    struct Test1 {}
    impl<D: Dim> System<f64, OVector<f64, D>> for Test1
    where
        DefaultAllocator: Allocator<f64, D>,
    {
        fn system(&self, x: f64, y: &OVector<f64, D>, dy: &mut OVector<f64, D>) {
            dy[0] = (x - y[0]) / 2.;
        }
    }

    struct Test2 {}
    impl<D: Dim> System<f64, OVector<f64, D>> for Test2
    where
        DefaultAllocator: Allocator<f64, D>,
    {
        fn system(&self, x: f64, y: &OVector<f64, D>, dy: &mut OVector<f64, D>) {
            dy[0] = -2. * x - y[0];
        }
    }

    struct Test3 {}
    impl<D: Dim> System<f64, OVector<f64, D>> for Test3
    where
        DefaultAllocator: Allocator<f64, D>,
    {
        fn system(&self, x: f64, y: &OVector<f64, D>, dy: &mut OVector<f64, D>) {
            dy[0] = (5. * x * x - y[0]) / (x + y[0]).exp();
        }
    }

    // Same as Test3, but aborts after x is greater/equal than 0.5
    struct Test4 {}
    impl<D: Dim> System<f64, OVector<f64, D>> for Test4
    where
        DefaultAllocator: Allocator<f64, D>,
    {
        fn system(&self, x: f64, y: &OVector<f64, D>, dy: &mut OVector<f64, D>) {
            dy[0] = (5. * x * x - y[0]) / (x + y[0]).exp();
        }

        fn solout(&mut self, x: f64, _y: &OVector<f64, D>, _dy: &OVector<f64, D>) -> bool {
            return x >= 0.5;
        }
    }

    #[test]
    fn test_integrate_test1_svector() {
        let system = Test1 {};
        let mut stepper = Rk4::new(system, 0., Vector1::new(1.), 0.2, 0.1);
        let _ = stepper.integrate();
        let x_out = stepper.x_out();
        let y_out = stepper.y_out();
        assert!((*x_out.last().unwrap() - 0.2).abs() < 1.0E-8);
        assert!((&y_out[1][0] - 0.95369).abs() < 1.0E-5);
        assert!((&y_out[2][0] - 0.91451).abs() < 1.0E-5);
    }

    #[test]
    fn test_integrate_test2_svector() {
        let system = Test2 {};
        let mut stepper = Rk4::new(system, 0., Vector1::new(-1.), 0.5, 0.1);
        let _ = stepper.integrate();
        let x_out = stepper.x_out();
        let y_out = stepper.y_out();
        assert!((*x_out.last().unwrap() - 0.5).abs() < 1.0E-8);
        assert!((&y_out[3][0] + 0.82246).abs() < 1.0E-5);
        assert!((&y_out[5][0] + 0.81959).abs() < 1.0E-5);
    }

    #[test]
    fn test_integrate_test3_svector() {
        let system = Test3 {};
        let mut stepper = Rk4::new(system, 0., Vector1::new(1.), 1., 0.1);
        let _ = stepper.integrate();
        let out = stepper.y_out();
        assert!((&out[5][0] - 0.913059839).abs() < 1.0E-9);
        assert!((&out[8][0] - 0.9838057659).abs() < 1.0E-9);
        assert!((&out[10][0] - 1.0715783953).abs() < 1.0E-9);
        assert_eq!(out.len(), 11);
    }

    #[test]
    fn test_integrate_test4_svector() {
        let system = Test4 {};
        let mut stepper = Rk4::new(system, 0., Vector1::new(1.), 1., 0.1);
        let _ = stepper.integrate();

        let x = stepper.x_out();
        assert!((*x.last().unwrap() - 0.5).abs() < 1.0E-9);

        let out = stepper.y_out();
        assert!((&out[5][0] - 0.913059839).abs() < 1.0E-9);
        assert_eq!(out.len(), 6);
    }

    #[test]
    fn test_integrate_test1_dvector() {
        let system = Test1 {};
        let mut stepper = Rk4::new(system, 0., DVector::from(vec![1.]), 0.2, 0.1);
        let _ = stepper.integrate();
        let x_out = stepper.x_out();
        let y_out = stepper.y_out();
        assert!((*x_out.last().unwrap() - 0.2).abs() < 1.0E-8);
        assert!((&y_out[1][0] - 0.95369).abs() < 1.0E-5);
        assert!((&y_out[2][0] - 0.91451).abs() < 1.0E-5);
    }

    #[test]
    fn test_integrate_test2_dvector() {
        let system = Test2 {};
        let mut stepper = Rk4::new(system, 0., DVector::from(vec![-1.]), 0.5, 0.1);
        let _ = stepper.integrate();
        let x_out = stepper.x_out();
        let y_out = stepper.y_out();
        assert!((*x_out.last().unwrap() - 0.5).abs() < 1.0E-8);
        assert!((&y_out[3][0] + 0.82246).abs() < 1.0E-5);
        assert!((&y_out[5][0] + 0.81959).abs() < 1.0E-5);
    }

    #[test]
    fn test_integrate_test3_dvector() {
        let system = Test3 {};
        let mut stepper = Rk4::new(system, 0., DVector::from(vec![1.]), 1., 0.1);
        let _ = stepper.integrate();
        let out = stepper.y_out();
        assert!((&out[5][0] - 0.913059839).abs() < 1.0E-9);
        assert!((&out[8][0] - 0.9838057659).abs() < 1.0E-9);
        assert!((&out[10][0] - 1.0715783953).abs() < 1.0E-9);
    }
}
```



## 🚀 RK45 (Runge-Kutta-Fehlberg 또는 Dormand-Prince)
- 적응형 스텝 방식: 오차 추정값을 기반으로 스텝 크기를 자동 조절
- 정확도: 4차와 5차 결과를 동시에 계산 → 오차 추정 가능
- 장점:
- 계산 효율이 높음 (필요할 때만 작은 스텝 사용)
- 급격한 변화가 있는 시스템에 적합
- 단점:
- 구현이 복잡함
- 계산량이 RK4보다 많을 수 있음

### 📐 RK45의 slope 계산식
```
k₁ = f(tₙ, yₙ)
k₂ = f(tₙ + a₂·h, yₙ + b₂₁·k₁·h)
k₃ = f(tₙ + a₃·h, yₙ + b₃₁·k₁·h + b₃₂·k₂·h)
k₄ = f(tₙ + a₄·h, yₙ + b₄₁·k₁·h + b₄₂·k₂·h + b₄₃·k₃·h)
k₅ = f(tₙ + a₅·h, yₙ + b₅₁·k₁·h + b₅₂·k₂·h + b₅₃·k₃·h + b₅₄·k₄·h)
k₆ = f(tₙ + a₆·h, yₙ + b₆₁·k₁·h + b₆₂·k₂·h + b₆₃·k₃·h + b₆₄·k₄·h + b₆₅·k₅·h)
```


### 🧮 다음 단계
이 slope들을 조합해 두 가지 근사값을 계산합니다:
```
yₙ₊₁⁽⁴⁾ = yₙ + h·(c₁·k₁ + c₂·k₂ + c₃·k₃ + c₄·k₄ + c₅·k₅)
yₙ₊₁⁽⁵⁾ = yₙ + h·(ĉ₁·k₁ + ĉ₂·k₂ + ĉ₃·k₃ + ĉ₄·k₄ + ĉ₅·k₅ + ĉ₆·k₆)
```

- 여기서 `yₙ₊₁⁽⁴⁾` 는 4차 근사값, `yₙ₊₁⁽⁵⁾` 는 5차 근사값입니다.
- 두 값의 차이를 통해 오차를 추정하고, 다음 스텝 크기 h를 조절합니다.

### 🔄 RK45 계산 흐름도
```
t_n ──► k1 ──┐
             ├─► k2 ──┐
             │        ├─► k3 ──┐
             │        │        ├─► k4 ──┐
             │        │        │        ├─► k5 ──┐
             │        │        │        │        ├─► k6 ──┐
             │        │        │        │        │        │
             │        │        │        │        │        ▼
             │        │        │        │        │     Combine slopes
             │        │        │        │        │        │
             │        │        │        │        │        ▼
             │        │        │        │        │     y_{n+1}^{(4)} and y_{n+1}^{(5)}
             │        │        │        │        │        │
             │        │        │        │        │        ▼
             │        │        │        │        │     Estimate error → adjust h
             │        │        │        │        │        │
             │        │        │        │        │        ▼
             └────────────────────────────────────────────► t_{n+1}

```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OVector, System, Vector1};
    use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

    // Same as Test3 from rk4.rs, but aborts after x is greater/equal than 0.5
    struct Test1 {}
    impl<D: Dim> System<f64, OVector<f64, D>> for Test1
    where
        DefaultAllocator: Allocator<f64, D>,
    {
        fn system(&self, x: f64, y: &OVector<f64, D>, dy: &mut OVector<f64, D>) {
            dy[0] = (5. * x * x - y[0]) / (x + y[0]).exp();
        }

        fn solout(&mut self, x: f64, _y: &OVector<f64, D>, _dy: &OVector<f64, D>) -> bool {
            return x >= 0.5;
        }
    }

    #[test]
    fn test_integrate_test1_svector() {
        let system = Test1 {};
        let mut stepper = Dopri5::new(system, 0., 1., 0.1, Vector1::new(1.), 1e-12, 1e-6);
        let _ = stepper.integrate();

        let x = stepper.x_out();
        assert!((*x.last().unwrap() - 0.5).abs() < 1.0E-9); //

        let out = stepper.y_out();
        assert!((&out[5][0] - 0.913059243).abs() < 1.0E-9);
    }
}
```


## ⚔️ RK4 vs RK45 비교표
| 항목         | RK4 (고전적 Runge-Kutta 4차) | RK45 (Runge-Kutta-Fehlberg 또는 Dormand-Prince) |
|--------------|-------------------------------|--------------------------------------------------|
| 정확도       | 4차 정확도                     | 4차 + 5차 정확도 (오차 추정 포함)               |
| 스텝 크기    | 고정                           | 자동 조절 (adaptive step size)                  |
| 오차 제어    | 없음                           | 내장 오차 추정으로 제어 가능                    |
| 계산 효율    | 단순하고 빠름                  | 효율적이지만 계산량이 더 많을 수 있음           |
| 사용 용도    | 단순한 ODE, 안정적 시스템      | 급격한 변화나 민감한 시스템에 적합              |
| 구현 난이도  | 쉬움                           | 중간~어려움                                      |


## ✨ 예시 상황
- RK4: 단순한 물리 시뮬레이션, 진자 운동 등
- RK45: 천체 궤도 계산, 화학 반응 속도, stiff 문제 등


---

## 8차 Runge-Kutta

DOP853은 고차 정확도를 갖는 8차 Runge-Kutta 방법으로, 특히 비강성(Non-stiff) ODE를 빠르고 정확하게 풀기 위해 설계된 알고리즘입니다.

### 🚀 DOP853이란?
- DOP는 Dormand-Prince 계열을 의미하고, 853은 8차 정확도와 5차 오차 추정, 3차 보간을 뜻합니다.
- Dormand & Prince가 개발한 고차 Runge-Kutta 방식 중 하나
- 적응형 스텝 크기 조절을 통해 오차를 제어하면서 효율적으로 적분

### 📐 핵심 특징
| 항목         | 설명                                   | 비고                                 |
|--------------|----------------------------------------|--------------------------------------|
| 정확도       | 8차 정확도                             | 매우 높은 정밀도                     |
| 오차 추정    | 5차 근사값과 비교하여 오차 계산        | 적응형 스텝 크기 조절에 사용         |
| 보간         | 3차 보간 함수 내장                     | 중간 시간점의 값 계산 가능           |
| 스텝 조절    | 자동 조절 (adaptive step size)         | 안정성과 효율성 향상                 |
| 사용 용도    | 비강성 ODE에 적합                      | 천체 궤도, 생물 모델, 기체 역학 등   |
| 계산량       | slope 12개 계산                        | RK4보다 많지만 효율적                |


### 🧮 계산 방식 요약
DOP853은 다음을 수행합니다:
- 12개의 slope k_1 ~ k_{12} 계산
- 8차 근사값과 5차 근사값을 비교하여 오차 추정
- 오차가 허용 범위 내면 다음 스텝으로 진행, 아니면 스텝 크기 조절
- 필요 시 3차 보간으로 중간 값 계산


### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OVector, System, Vector1};
    use nalgebra::{allocator::Allocator, DefaultAllocator, Dim};

    // Same as Test3 from rk4.rs, but aborts after x is greater/equal than 0.5
    struct Test1 {}
    impl<D: Dim> System<f64, OVector<f64, D>> for Test1
    where
        DefaultAllocator: Allocator<f64, D>,
    {
        fn system(&self, x: f64, y: &OVector<f64, D>, dy: &mut OVector<f64, D>) {
            dy[0] = (5. * x * x - y[0]) / (x + y[0]).exp();
        }

        fn solout(&mut self, x: f64, _y: &OVector<f64, D>, _dy: &OVector<f64, D>) -> bool {
            return x >= 0.5;
        }
    }

    #[test]
    fn test_integrate_test1_svector() {
        let system = Test1 {};
        let mut stepper = Dop853::new(system, 0., 1., 0.1, Vector1::new(1.), 1e-12, 1e-6);
        let _ = stepper.integrate();

        let x = stepper.x_out();
        assert!((*x.last().unwrap() - 0.5).abs() < 1.0E-9); //

        let out = stepper.y_out();
        assert!((&out[5][0] - 0.912968195).abs() < 1.0E-9);
    }
}

```

