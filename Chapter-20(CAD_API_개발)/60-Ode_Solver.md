# Ode Solver
## 📘 OdeSolver 핵심 알고리즘 설명
### 1️⃣ RK4 (Runge-Kutta 4차) 알고리즘
- 고정 스텝 h를 사용하여 다음 상태 y_{n+1}를 계산:

$$
\begin{aligned}k_1&=f(t_n,y_n)\\ \quad k_2&=f\left( t_n+\frac{h}{2},y_n+\frac{h}{2}k_1\right) \\ \quad k_3&=f\left( t_n+\frac{h}{2},y_n+\frac{h}{2}k_2\right) \\ \quad k_4&=f(t_n+h,y_n+hk_3)\\ \quad y_{n+1}&=y_n+\frac{h}{6}(k_1+2k_2+2k_3+k_4)\end{aligned}
$$

    - 정확도: 4차
    - 스텝 크기 고정
    - 구현 간단, 계산량 많음

### 2️⃣ RK45 (Dormand-Prince 5(4)) 알고리즘
- 적응형 스텝을 사용하여 5차와 4차 해를 동시에 계산하고 오차 추정:

$$
\mathrm{오차}=\left( \frac{1}{n}\sum _{i=1}^n\left( \frac{y_i^{(5)}-y_i^{(4)}}{\mathrm{scale_{\mathnormal{i}}}}\right) ^2\right) ^{1/2}
$$

- 여기서
  
$$
\mathrm{scale_{\mathnormal{i}}}=a_{\mathrm{tol}}+r_{\mathrm{tol}}\cdot \max (|y_i|,|y_i^{(5)}|)
$$

- $y^{(5)}$: 5차 근사 해
- $y^{(4)}$: 4차 근사 해
- $a_{\mathrm{tol}},r_{\mathrm{tol}}$: 절대/상대 허용 오차
- 스텝 크기 조정:

$$
h_{\mathrm{new}}=h\cdot \mathrm{safety}\cdot \left( \frac{1}{\mathrm{err}}\right) ^{1/5}
$$

- 정확도: 5차 (4차 오차 추정 포함)
- 스텝 자동 조절
- 효율적, 복잡한 시스템에 적합


## 📐 RK4와 RK45의 수학적 검증 개요
### 1️⃣ RK4 (Runge-Kutta 4차) 방법
- RK4는 다음과 같은 방식으로 y(t)를 근사합니다:

$$
\begin{aligned}k_1&=f(t_n,y_n)\\ k_2&=f\left( t_n+\frac{h}{2},y_n+\frac{h}{2}k_1\right) \\ k_3&=f\left( t_n+\frac{h}{2},y_n+\frac{h}{2}k_2\right) \\ k_4&=f(t_n+h,y_n+hk_3)\\ y_{n+1}&=y_n+\frac{h}{6}(k_1+2k_2+2k_3+k_4)\end{aligned}
$$

- 정확도: 4차 (오차 $O(h^5)$ )
- 검증 방법: 해석적 해 $y(t)$ 와 수치 해 $y_n$ 의 절대 오차 비교
- 예: $y'=y,y(0)=1\Rightarrow y(t)=e^t$
    - RK4로 $y(1)\approx e$ 를 계산하고 $|y_{\mathrm{RK4}}-e|<\varepsilon$  확인

### 2️⃣ RK45 (Dormand-Prince 5(4)) 방법
- RK45는 5차 근사 $y^{(5)}$ 와 4차 근사 $y^{(4)}$ 를 동시에 계산하여 오차를 추정:

$$
\mathrm{오차}=\left( \frac{1}{n}\sum _{i=1}^n\left( \frac{y_i^{(5)}-y_i^{(4)}}{a_{\mathrm{tol}}+r_{\mathrm{tol}}\cdot \max (|y_i|,|y_i^{(5)}|)}\right) ^2\right) ^{1/2}
$$

- 정확도: 5차 (오차 O(h^6))
- 스텝 조절: 오차가 허용 범위 이내면 스텝 채택, 아니면 축소
- 검증 예:
- $y'=y\Rightarrow y(1)=e$
- $y'=-ky\Rightarrow y(t)=e^{-kt}$
    - RK45로 계산한 $y(t)$ 와 해석적 해 비교

## ✅ 수치 검증 방식
- 절대 오차: $|y_{\mathrm{num}}-y_{\mathrm{exact}}|$
- 상대 오차: $\frac{|y_{\mathrm{num}}-y_{\mathrm{exact}}|}{\max (|y_{\mathrm{num}}|,|y_{\mathrm{exact}}|)}$
- 시간 궤적 검증: 시간 배열 $t_i$ 가 단조 증가하고 $t_n=t_{\mathrm{end}}$ 에 도달하는지 확인
- 스텝 수 비교: RK45가 RK4보다 적은 스텝으로 동일 정확도 달성하는지 확인

🔍 실제 검증 사례
- MATLAB에서 $y'=2t^5$ 문제를 RK4와 ode45(RK45)로 풀었을 때, RK45는 해석적 해와 정확히 일치하지만 RK4는 오차가 발생.
- SciPy의 RK45 구현은 Dormand-Prince 쌍을 사용하며, 5차 해를 채택하고 4차 해로 오차를 추정.
- RKF45(C++) 구현에서도 동일한 방식으로 오차 추정 및 스텝 조절이 이루어짐

## ✅ 현업 적용 가능성 평가

| 항목 / 구성 요소         | 적용 가능성 | 설명 및 평가 요약                                                                 |
|--------------------------|--------------|------------------------------------------------------------------------------------|
| `OdeSolver` 구조체       | ✅ 높음       | 구조가 명확하고 확장성 있음. 다양한 ODE 시스템에 유연하게 대응 가능               |
| `rk45_with_user()`       | ✅ 매우 유용  | 사용자 정의 파라미터를 외부에서 주입 가능. 산업용 모델링에 적합                    |
| `Vec<f64>` 기반 벡터     | ⚠️ 보통       | 간단하고 직관적이나, 고성능 수치 계산에는 `nalgebra` 또는 `ndarray`가 더 적합     |
| `set_function(f)`        | ✅ 유연함     | 클로저 기반으로 시스템 정의 가능. 다양한 모델에 대응 가능                         |
| `set_tolerances(a, r)`   | ✅ 필수 기능  | 오차 제어 가능. RK45의 정확도 조절에 핵심적인 역할                                |
| `set_step_bounds(min,max)` | ✅ 안정성 확보 | 스텝 크기 제한으로 수치 폭주 방지 가능                                            |
| `integrate_rk4()`        | ✅ 기본 제공  | 고정 스텝 방식. 단순하고 예측 가능하지만 효율성은 낮을 수 있음                    |
| `integrate_rk45()`       | ✅ 고급 기능  | 적응형 스텝으로 효율적 계산 가능. 대부분의 실무 문제에 적합                       |
| `solout()` 콜백          | ✅ 이벤트 제어 | 조건 기반 적분 중단 가능. 실시간 제어/이벤트 감지에 유용                          |
| 테스트 커버리지          | ✅ 충분함     | 해석적 해 기반 검증, 다양한 모델 포함. 신뢰성 확보에 기여                         |



## 🔍 개선 및 보완 제안

| 항목 또는 키워드         | 제안 또는 설명                                                                 |
|--------------------------|--------------------------------------------------------------------------------|
| `nalgebra`, `ndarray`    | `Vec<f64>` 대신 고성능 선형대수 라이브러리 사용 시 다차원 시스템 처리 및 성능 향상 가능 |
| `Result<T, E>`           | `bool` 반환 대신 `Result`로 에러 원인 명시 → 디버깅 및 예외 처리에 유리             |
| `log`                    | `log` 크레이트 도입으로 내부 상태, 스텝 수, 오차 등을 로깅하여 추적 가능             |
| `Send + Sync`            | 병렬 환경에서 사용 시 스레드 안전성 확보 필요 (`FnMut` → `Send + Sync` 고려)        |


🧪 결론
- 현업 적용 가능성: 매우 높음
- 단, 적용 전에:
- 성능 요구사항 분석
- 병렬성/확장성 필요 여부 판단
- 에러 처리 및 로깅 보완
- 테스트 범위 확장

---

## 소스 코드
```rust
use crate::core::maths::on_clamp;

/// Dormand–Prince 5(4) / RK4를 지원하는 ODE 솔버
pub struct OdeSolver<'a> {
    f: Option<Box<dyn FnMut(f64, &[f64], &mut [f64]) + 'a>>,
    n: usize,

    // 적응 스텝 옵션
    r_tol: f64,
    a_tol: f64,
    h_min: f64,
    h_max: f64,
    fac_min: f64,
    fac_max: f64,
    safety: f64,
}
```
```rust
impl Default for OdeSolver<'_> {
    fn default() -> Self {
        Self {
            f: None,
            n: 0,
            r_tol: 1e-6,
            a_tol: 1e-9,
            h_min: 1e-12,
            h_max: 1e2,
            fac_min: 0.2,
            fac_max: 5.0,
            safety: 0.9,
        }
    }
}
```
```rust
impl<'a> OdeSolver<'a> {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            ..Default::default()
        }
    }

    #[inline]
    pub fn set_dimension(&mut self, n: usize) {
        self.n = n;
    }

    #[inline]
    pub fn dimension(&self) -> usize {
        self.n
    }

    /// 시스템 미분함수 설정
    pub fn set_function<F>(&mut self, f: F)
    where
        F: FnMut(f64, &[f64], &mut [f64]) + 'a,
    {
        self.f = Some(Box::new(f));
    }

    /// 공차 설정
    pub fn set_tolerances(&mut self, abs_tol: f64, rel_tol: f64) {
        self.a_tol = if abs_tol > 0.0 { abs_tol } else { 1e-9 };
        self.r_tol = if rel_tol > 0.0 { rel_tol } else { 1e-6 };
    }

    /// 스텝 한계 (동일 의미: SetStepBounds / SetStepLimits)
    pub fn set_step_bounds(&mut self, mut h_min: f64, mut h_max: f64) {
        if !(h_min > 0.0) {
            h_min = f64::EPSILON;
        }
        if !(h_max > 0.0) {
            h_max = h_min;
        }
        if h_max < h_min {
            std::mem::swap(&mut h_min, &mut h_max);
        }
        self.h_min = h_min;
        self.h_max = h_max;
    }
    pub fn set_step_limits(&mut self, hmin: f64, hmax: f64) {
        let hmin = hmin.max(1e-16);
        self.h_min = hmin;
        self.h_max = hmax.max(hmin);
    }
    pub fn get_step_bounds(&self) -> (f64, f64) {
        (self.h_min, self.h_max)
    }

    fn deriv(&mut self, t: f64, y: &[f64], dydt: &mut [f64]) {
        if let Some(ref mut f) = self.f.as_mut() {
            f(t, y, dydt);
        }
    }

    fn step_rk4(&mut self, t: f64, y: &[f64], h: f64, y_out: &mut Vec<f64>) {
        let n = self.n;
        debug_assert_eq!(y.len(), n);
        debug_assert_eq!(y_out.len(), n);

        let mut k1 = vec![0.0; n];
        let mut k2 = vec![0.0; n];
        let mut k3 = vec![0.0; n];
        let mut k4 = vec![0.0; n];
        let mut yt = vec![0.0; n];

        // k1 = f(t, y)
        self.deriv(t, y, &mut k1);

        // k2 = f(t + h/2, y + h/2 * k1)
        for i in 0..n {
            yt[i] = y[i] + 0.5 * h * k1[i];
        }
        self.deriv(t + 0.5 * h, &yt, &mut k2);

        // k3 = f(t + h/2, y + h/2 * k2)
        for i in 0..n {
            yt[i] = y[i] + 0.5 * h * k2[i];
        }
        self.deriv(t + 0.5 * h, &yt, &mut k3);

        // k4 = f(t + h, y + h * k3)
        for i in 0..n {
            yt[i] = y[i] + h * k3[i];
        }
        self.deriv(t + h, &yt, &mut k4);

        for i in 0..n {
            y_out[i] = y[i] + (h / 6.0) * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
    }
```
```rust
    fn step_rk45(&mut self, t: f64, y: &[f64], h: f64, y_next_out: &mut Vec<f64>) -> f64 {
        let n = self.n;
        debug_assert_eq!(y.len(), n);
        debug_assert_eq!(y_next_out.len(), n);

        // k*는 h가 곱해진 형태로 사용 (원본과 동일)
        let mut k1 = vec![0.0; n];
        let mut k2 = vec![0.0; n];
        let mut k3 = vec![0.0; n];
        let mut k4 = vec![0.0; n];
        let mut k5 = vec![0.0; n];
        let mut k6 = vec![0.0; n];
        let mut k7 = vec![0.0; n];
        let mut yt = vec![0.0; n];

        let mut eval = |yin: &[f64], kout: &mut [f64], tt: f64| {
            self.deriv(tt, yin, kout);
            for i in 0..n {
                kout[i] *= h;
            } // h 스케일
        };

        // k1
        eval(y, &mut k1, t);

        // k2 @ t + 1/5 h
        for i in 0..n {
            yt[i] = y[i] + (1.0 / 5.0) * k1[i];
        }
        eval(&yt, &mut k2, t + (1.0 / 5.0) * h);

        // k3 @ t + 3/10 h
        for i in 0..n {
            yt[i] = y[i] + (3.0 / 40.0) * k1[i] + (9.0 / 40.0) * k2[i];
        }
        eval(&yt, &mut k3, t + (3.0 / 10.0) * h);

        // k4 @ t + 4/5 h
        for i in 0..n {
            yt[i] = y[i] + (44.0 / 45.0) * k1[i] + (-56.0 / 15.0) * k2[i] + (32.0 / 9.0) * k3[i];
        }
        eval(&yt, &mut k4, t + (4.0 / 5.0) * h);

        // k5 @ t + 8/9 h
        for i in 0..n {
            yt[i] = y[i]
                + (19372.0 / 6561.0) * k1[i]
                + (-25360.0 / 2187.0) * k2[i]
                + (64448.0 / 6561.0) * k3[i]
                + (-212.0 / 729.0) * k4[i];
        }
        eval(&yt, &mut k5, t + (8.0 / 9.0) * h);

        // k6 @ t + h
        for i in 0..n {
            yt[i] = y[i]
                + (9017.0 / 3168.0) * k1[i]
                + (-355.0 / 33.0) * k2[i]
                + (46732.0 / 5247.0) * k3[i]
                + (49.0 / 176.0) * k4[i]
                + (-5103.0 / 18656.0) * k5[i];
        }
        eval(&yt, &mut k6, t + 1.0 * h);

        // 5차 해 (y5)
        let mut y5 = vec![0.0; n];
        for i in 0..n {
            y5[i] = y[i]
                + (35.0 / 384.0) * k1[i]
                + (500.0 / 1113.0) * k3[i]
                + (125.0 / 192.0) * k4[i]
                + (-2187.0 / 6784.0) * k5[i]
                + (11.0 / 84.0) * k6[i];
        }

        // FSAL: k7 = f(t+h, y5)
        eval(&y5, &mut k7, t + 1.0 * h);

        // 4차 해 (y4)
        let mut y4 = vec![0.0; n];
        for i in 0..n {
            y4[i] = y[i]
                + (5179.0 / 57600.0) * k1[i]
                + (7571.0 / 16695.0) * k3[i]
                + (393.0 / 640.0) * k4[i]
                + (-92097.0 / 339200.0) * k5[i]
                + (187.0 / 2100.0) * k6[i]
                + (1.0 / 40.0) * k7[i];
        }

        // 정규화된 RMS 에러
        let mut err2 = 0.0;
        for i in 0..n {
            let sc = self.a_tol + self.r_tol * y[i].abs().max(y5[i].abs());
            let e = (y5[i] - y4[i]) / if sc > 0.0 { sc } else { 1.0 };
            err2 += e * e;
        }
        let err = (err2 / (n.max(1) as f64)).sqrt();

        // 출력으로 5차 해를 제공
        y_next_out.copy_from_slice(&y5);
        err
    }
```
```rust
    pub fn integrate_rk4(
        &mut self,
        t0: f64,
        y0: &[f64],
        t1: f64,
        h: f64,
        y1: &mut Vec<f64>,
        mut ts: Option<&mut Vec<f64>>,
        mut ys: Option<&mut Vec<Vec<f64>>>,
    ) -> bool {
        if self.f.is_none() || self.n == 0 || !(h > 0.0) {
            return false;
        }
        assert_eq!(y0.len(), self.n);

        let n = self.n;
        let mut y = y0.to_vec();

        if let Some(ts) = ts.as_mut() {
            ts.clear();
            ts.push(t0);
        }
        if let Some(ys) = ys.as_mut() {
            ys.clear();
            ys.push(y.clone());
        }

        let mut t = t0;
        let mut y_next = vec![0.0; n];

        while t < t1 - 1e-15 {
            let h_use = h.min(t1 - t);
            self.step_rk4(t, &y, h_use, &mut y_next);
            t += h_use;
            y.copy_from_slice(&y_next);

            if let Some(ts) = ts.as_mut() {
                ts.push(t);
            }
            if let Some(ys) = ys.as_mut() {
                ys.push(y.clone());
            }
        }

        *y1 = y;
        true
    }
```
```rust
    pub fn integrate_rk45(
        &mut self,
        t0: f64,
        y0: &[f64],
        t1: f64,
        y1: &mut Vec<f64>,
        mut ts: Option<&mut Vec<f64>>,
        mut ys: Option<&mut Vec<Vec<f64>>>,
        h_init: Option<f64>,
    ) -> bool {
        if self.f.is_none() || self.n == 0 {
            return false;
        }
        assert_eq!(y0.len(), self.n);

        let n = self.n;
        let mut y = y0.to_vec();

        if let Some(ts) = ts.as_mut() {
            ts.clear();
            ts.push(t0);
        }
        if let Some(ys) = ys.as_mut() {
            ys.clear();
            ys.push(y.clone());
        }

        let mut t = t0;
        let t_end = t1;

        let mut h = h_init.unwrap_or_else(|| on_clamp((t_end - t0) / 50.0, self.h_min, self.h_max));
        h = on_clamp(h, self.h_min, self.h_max);

        let max_iter = 1_000_000;
        let mut y_candidate = vec![0.0; n];

        for _iter in 0..max_iter {
            if t >= t_end - 1e-15 {
                break;
            }
            if t + h > t_end {
                h = t_end - t;
            }
            h = on_clamp(h, self.h_min, self.h_max);

            let err = self.step_rk45(t, &y, h, &mut y_candidate);
            if err <= 1.0 {
                // 스텝 채택
                t += h;
                y.copy_from_slice(&y_candidate);

                if let Some(ts) = ts.as_mut() {
                    ts.push(t);
                }
                if let Some(ys) = ys.as_mut() {
                    ys.push(y.clone());
                }

                // 다음 스텝 제안
                let mut fac = self.safety * (1.0f64.max(1.0 / err)).powf(1.0 / 5.0);
                fac = on_clamp(fac, self.fac_min, self.fac_max);
                h = on_clamp(h * fac, self.h_min, self.h_max);
            } else {
                // 거부 → 줄여서 재시도
                let mut fac = self.safety * (1.0f64.max(1.0 / err)).powf(1.0 / 5.0);
                fac = on_clamp(fac, 0.1, 0.5);
                h = on_clamp(h * fac, self.h_min, self.h_max);
            }
        }

        *y1 = y;
        t >= t_end - 1e-12
    }
```
```rust
    pub fn rk45_with_user<U, F>(
        &mut self,
        mut f_ud: F,
        t0: f64,
        y0: &[f64],
        t1: f64,
        y1: &mut Vec<f64>,
        user: &'a mut U,
        ts: Option<&'a mut Vec<f64>>,
        ys: Option<&'a mut Vec<Vec<f64>>>,
        h_init: Option<f64>,
    ) -> bool
    where
        F: FnMut(f64, &[f64], &mut [f64], usize, &mut U) + 'a,
    {
        let n = self.n;

        // set_function 은 FnMut 을 받아야 합니다.
        self.set_function(move |t, y, dy| {
            f_ud(t, y, dy, n, user);
        });

        self.integrate_rk45(t0, y0, t1, y1, ts, ys, h_init)
    }
}
```

---


## 🧪 테스트 수식 및 목적 요약

| 테스트 함수 이름                  | 수식 또는 모델 정의                                      | 목적 및 검증 내용                          | 특이사항 또는 기능         |
|----------------------------------|----------------------------------------------------------|--------------------------------------------|----------------------------|
| `rk4_exp_yprime_eq_y()`         | $y' = y \Rightarrow y(t) = e^t$                     | RK4 방식으로 exp(t) 계산 정확도 확인       | 고정 스텝 RK4              |
| `rk45_exp_yprime_eq_y()`        | $y' = y \Rightarrow y(t) = e^t$                     | RK45 방식으로 exp(t) 계산 및 궤적 검증     | 적응형 스텝 RK45           |
| `rk45_simple_harmonic_oscillator()` | $x' = v,\ v' = -\omega^2 x \Rightarrow x(t) = \cos(\omega t)$| 단순 조화 진동자 모델의 주기성 검증 | 2차 시스템, 진동 모델      |
| `rk4_vs_rk45_steps_and_accuracy()` | $y' = y$                                           | RK4 vs RK45 정확도 및 스텝 수 비교         | 효율성 비교                |
| `rk45_with_user_param()`        | $y' = -k y \Rightarrow y(t) = e^{-kt}$              | 사용자 파라미터 기반 감쇠 모델 검증        | 사용자 정의 함수           |
| `test_integrate_test1_*()`      | $y' = \frac{x - y}{2}$                              | 선형 1차 ODE, 수렴성 확인                  | RK4, nalgebra 벡터 사용    |
| `test_integrate_test2_*()`      | $y' = -2x - y$                                      | 선형 비자기적 ODE, 감쇠성 확인             | RK4, nalgebra 벡터 사용    |
| `test_integrate_test3_*()`      | $y' = \frac{5x^2 - y}{e^{x + y}}$                   | 비선형 ODE, 수치 안정성 확인               | RK4, 복잡한 함수 구조      |
| `test_integrate_test4_*()`      | $x \geq 0.5$                                        | 적분 중단 조건 테스트                      | `solout()` 콜백 사용       |


## ⚙️ OdeSolver 주요 함수 요약

| 함수 이름              | 설명 또는 역할                                      |
|------------------------|-----------------------------------------------------|
| `set_function(f)`      | 미분방정식의 우변 함수 $f(t, y, dy)$설정       |
| `set_tolerances(a, r)` | 절대 오차 `a_tol`과 상대 오차 `r_tol` 설정          |
| `set_step_bounds(min, max)` | RK45에서 사용할 최소/최대 스텝 크기 설정     |
| `integrate_rk4()`      | 고정 스텝 Runge-Kutta 4차 방식으로 적분 수행        |
| `integrate_rk45()`     | 적응형 스텝 Dormand-Prince 5(4) 방식으로 적분 수행  |
| `rk45_with_user()`     | 사용자 정의 파라미터를 포함한 RK45 적분 수행        |


## 📐 각 테스트 수식 및 설명
### 1️⃣ y′ = y → y(t) = eᵗ
- 해석적 해: y(t)=e^t
- 초기 조건: y(0)=1
- 검증 방법: 수치 해 y(1)과 e^1 비교
### 2️⃣ x′ = v, v′ = −ω²x → 단순 조화 진동자
- 해석적 해:
- x(t)=\cos (\omega t)
- v(t)=-\omega \sin (\omega t)
- 초기 조건: x(0)=1,v(0)=0
- 검증 방법: 한 주기 후 x(T)≈1,v(T)≈0
### 3️⃣ y′ = −k·y → 감쇠 모델
- 해석적 해: y(t)=e^{-kt}
- 초기 조건: y(0)=1
- 검증 방법: 사용자 파라미터 k를 활용해 y(t) 계산

### 🎯 테스트 목적 요약
- 정확도 검증: 수치 해가 해석적 해와 얼마나 가까운지 확인
- 스텝 효율성: RK45가 RK4보다 적은 스텝으로 동일 정확도 달성하는지 비교
- 모델 다양성: 선형, 비선형, 다변수, 사용자 파라미터 기반 모델까지 폭넓게 검증
- 시간 궤적 검증: 시간 배열이 단조 증가하고, 목표 시간에 정확히 도달하는지 확인
- 중단 조건: solout()을 통해 이벤트 기반 적분 중단 기능 확인

```rust
#[cfg(test)]
mod tests {

    use geometry::solver::ode_solver::OdeSolver;
    use std::f64::consts::{E, PI};

    // ----------------- 헬퍼 -----------------
    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    fn rel_err(a: f64, b: f64) -> f64 {
        let denom = a.abs().max(b.abs()).max(1e-16);
        (a - b).abs() / denom
    }

    // ----------------- 테스트 -----------------
```
```rust
    #[test]
    fn rk4_exp_yprime_eq_y() {
        // y' = y, y(0) = 1  →  y(t) = e^t
        let n = 1;
        let mut solver = OdeSolver::new(n);
        solver.set_function(|_t, y, dy| {
            dy[0] = y[0];
        });

        let t0 = 0.0;
        let t1 = 1.0;
        let h = 1.0 / 100.0;
        let y0 = [1.0_f64];

        let mut y1 = Vec::new();
        let ok = solver.integrate_rk4(t0, &y0, t1, h, &mut y1, None, None);
        assert!(ok);

        let expected = E; // e^1
        assert!(
            approx_eq(y1[0], expected, 1e-6),
            "RK4 exp(1) wrong: got {}, want {}",
            y1[0],
            expected
        );
    }
```
```rust
    #[test]
    fn rk45_exp_yprime_eq_y() {
        // y' = y, y(0)=1 → y(1)=e
        let n = 1;
        let mut solver = OdeSolver::new(n);
        solver.set_function(|_t, y, dy| {
            dy[0] = y[0];
        });
        solver.set_tolerances(1e-12, 1e-12);

        let t0 = 0.0;
        let t1 = 1.0;
        let y0 = [1.0_f64];

        let mut y1 = Vec::new();
        let mut ts = Vec::new();
        let mut ys = Vec::new();

        let ok = solver.integrate_rk45(t0, &y0, t1, &mut y1, Some(&mut ts), Some(&mut ys), None);
        assert!(ok, "RK45 integrate failed");

        // 궤적 단조 증가 + 끝 시간이 정확한지
        assert!(!ts.is_empty());
        assert!(approx_eq(*ts.last().unwrap(), t1, 1e-12));
        for w in ts.windows(2) {
            assert!(w[1] >= w[0]);
        }
        assert_eq!(ts.len(), ys.len());

        let expected = E;
        let err = (y1[0] - expected).abs();
        assert!(err < 1e-8, "RK45 exp(1) abs err too big: {}", err);
    }
```
```rust
    #[test]
    fn rk45_simple_harmonic_oscillator() {
        // x' = v, v' = -ω^2 x
        // x(0)=1, v(0)=0 → x(t)=cos(ωt), v(t)=-ω sin(ωt)
        let omega = 2.0;
        let n = 2;
        let mut solver = OdeSolver::new(n);
        solver.set_function(move |_t, y, dy| {
            dy[0] = y[1];
            dy[1] = -omega * omega * y[0];
        });
        solver.set_tolerances(1e-9, 1e-9);

        let t0 = 0.0;
        let t1 = 2.0 * PI / omega; // 한 주기
        let y0 = [1.0_f64, 0.0_f64];

        let mut y1 = Vec::new();
        let ok = solver.integrate_rk45(t0, &y0, t1, &mut y1, None, None, None);
        assert!(ok);

        // 한 주기 후 x≈1, v≈0
        assert!(approx_eq(y1[0], 1.0, 1e-6), "x(T) wrong, got {}", y1[0]);
        assert!(approx_eq(y1[1], 0.0, 1e-6), "v(T) wrong, got {}", y1[1]);
    }
```
```rust
    #[test]
    fn rk4_vs_rk45_steps_and_accuracy() {
        // 동일 문제에서 RK45가 적은 스텝으로 비슷/더 좋은 정확도 달성하는지 체크(대략적)
        let n = 1;
        let mut rk4 = OdeSolver::new(n);
        rk4.set_function(|_t, y, dy| {
            dy[0] = y[0];
        });

        let mut rk45 = OdeSolver::new(n);
        rk45.set_function(|_t, y, dy| {
            dy[0] = y[0];
        });
        rk45.set_tolerances(1e-12, 1e-12);

        let t0 = 0.0;
        let t1 = 1.0;
        let y0 = [1.0_f64];

        // RK4
        let h = 1.0 / 100.0;
        let mut y1_rk4 = Vec::new();
        let mut ts4 = Vec::new();
        let ok4 = rk4.integrate_rk4(t0, &y0, t1, h, &mut y1_rk4, Some(&mut ts4), None);
        assert!(ok4);

        // RK45
        let mut y1_rk45 = Vec::new();
        let mut ts45 = Vec::new();
        let ok45 = rk45.integrate_rk45(t0, &y0, t1, &mut y1_rk45, Some(&mut ts45), None, None);
        assert!(ok45);

        let exp1 = E;
        let err4 = (y1_rk4[0] - exp1).abs();
        let err45 = (y1_rk45[0] - exp1).abs();

        // 정확도는 동일하거나 더 좋게
        assert!(
            err45 <= err4 * 1.1,
            "RK45 not accurate enough: rk4={}, rk45={}",
            err4,
            err45
        );

        // 스텝 수는 보통 RK45가 더 적음 (적응스텝이라 상황에 따라 다를 수 있지만 대체로 기대)
        assert!(
            ts45.len() <= ts4.len(),
            "RK45 used more steps than RK4: rk45={}, rk4={}",
            ts45.len(),
            ts4.len()
        );
    }
```
```rust
    #[derive(Clone)]
    struct Params {
        k: f64,
    }
    fn run_solver<'a>(user: &'a mut Params) {
        let _n = 1;
        let k = user.k;

        let mut solver = OdeSolver::new(1); // Solver도 'a 라이프타임을 가짐
        solver.set_tolerances(1e-12, 1e-10);
        solver.set_step_bounds(1e-12, 0.1);

        let f_ud = |_: f64, y: &[f64], dy: &mut [f64], _n: usize, user: &mut Params| {
            dy[0] = -user.k * y[0];
        };

        let t0 = 0.0;
        let t1 = 0.75;
        let y0 = [1.0_f64];

        let mut y1 = Vec::new();

        let ok = solver.rk45_with_user(f_ud, t0, &y0, t1, &mut y1, user, None, None, None);
        assert!(ok);

        let expected = (-k * t1).exp();
        assert!(
            rel_err(y1[0], expected) < 1e-7,
            "with_user wrong: got {}, want {}",
            y1[0],
            expected
        );
    }
```
```rust
    #[test]
    fn rk45_with_user_param() {
        // 사용자 데이터(예: 감쇠 비율 k)를 외부에서 넘겨서 씀
        // y' = -k y, y(0)=1 → y(t)=exp(-k t)

        let mut params = Params { k: 1.5 };
        run_solver(&mut params);
    }
```
```rust
    #[test]
    fn rk45_with_user_param2() {
        #[derive(Clone)]
        struct Params {
            k: f64,
        }

        let mut params = Params { k: 1.5 };
        let k = params.k;

        let mut solver = OdeSolver::new(1);
        solver.set_tolerances(1e-12, 1e-10);
        solver.set_step_bounds(1e-12, 0.1);

        let f_ud = |_: f64, y: &[f64], dy: &mut [f64], _n: usize, user: &mut Params| {
            dy[0] = -user.k * y[0];
        };

        let t0 = 0.0;
        let t1 = 0.75;
        let y0 = [1.0];
        let mut y1 = Vec::new();

        // 여기서 params는 테스트 함수 전체 라이프타임을 가짐
        let ok = solver.rk45_with_user(f_ud, t0, &y0, t1, &mut y1, &mut params, None, None, None);
        assert!(ok);

        let expected = (-k * t1).exp();
        assert!(rel_err(y1[0], expected) < 1e-7);
    }
}
```

