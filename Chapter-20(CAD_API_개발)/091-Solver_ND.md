## Solver ND

- **N차원 비선형 방정식 시스템을 뉴턴 방법으로 푸는 로컬 솔버(LocalSolveNd)** 입니다. 

## 🔎 핵심 개념
- 문제 형태

$$
F(x)=0,\quad F:\mathbb{R^{\mathnormal{n}}}\rightarrow \mathbb{R^{\mathnormal{n}}}
$$

- 여기서 $x$ 는 n-차원 벡터, $F(x)$ 는 n-차원 함수값 벡터입니다.
- 뉴턴 반복
각 단계에서 자코비안 $J(x)$ 을 이용해 선형 시스템을 풉니다:

$$
J(x)\cdot \Delta x=-F(x)
$$

$$
x\leftarrow x+\Delta x
$$

- 수렴 조건
    - $\| F(x)\|$ 가 원하는 허용 오차 이하일 때 종료
    - 스텝 크기가 너무 작아 더 이상 개선이 없을 때 종료
    - 최대 반복 횟수 초과 시 종료


## 🔎 전체 구조
- 목적: $F(x)=0$ 형태의 N차원 비선형 시스템을 풀기 위한 반복 알고리즘.
- 방법: Newton-Raphson 방식 + 자코비안 행렬을 이용한 선형 시스템 풀이.
- 부가 기능:
    - 수렴 판정 (desired_tol, acceptable_tol)
    - 스텝 크기 제한 (trust-region 비슷한 scaling)
    - 경계(bounds) 클램프
    - 종료 이유 기록 (TerminationReason)

## 📚 주요 타입 및 함수
### 1. EvalFunctionNd 트레이트
```rust
pub trait EvalFunctionNd {
    fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()>;
}
```

- 역할: 사용자가 정의하는 함수 인터페이스.
- 입력: 현재 추정치 $x\in \mathbb{R^{\mathnormal{n}}}$.
- 출력:
    - $f$: 함수값 F(x) (길이 n).
    - $jac$: 자코비안 행렬 J(x) (길이 n*n, row-major).
    - $converged$: 내부적으로 이미 충분히 수렴했다고 판단한 경우 true.

### 2. NdTerminationReason 열거형
```rust
pub enum NdTerminationReason {
    NotStarted,
    Converged,       // ||F(x)|| <= desired_tol
    Close,           // ||F(x)|| <= acceptable_tol
    BadJacobian,     // 자코비안이 특이(singular)
    OutOfBounds,     // bounds 밖으로 나감
    MaxIterations,   // 반복 초과
    EvaluationFailed // 함수 평가 실패
}
```
- 역할: 반복 종료 원인을 기록.

### ✅ LocalSolveNd가 하는 일
- 사용자 정의 함수 평가
  - EvalFunctionNd 트레이트를 통해 $F(x)$ 와 $J(x)$ 를 계산.
- 뉴턴/가우스-뉴턴 방식으로 해 찾기
  - 선형 시스템 $J\Delta x=-F$ 풀기
  - 스텝 크기 제한(trust region 비슷한 scaling)
- bounds(변수 범위) 클램프
- 종료 이유 기록
  - Converged (정밀 수렴)
  - Close (허용 오차 내 수렴)
  - BadJacobian (자코비안 특이)
  - OutOfBounds (경계 밖)
  - MaxIterations (반복 초과)
  - EvaluationFailed (함수 평가 실패)


### 3. LocalSolveNd 구조체
```rust
pub struct LocalSolveNd<F: EvalFunctionNd> {
    func: F,
    dim: usize,
    desired_tol: f64,
    acceptable_tol: f64,
    max_iter: usize,
    bounds: Option<Vec<(f64, f64)>>,
    found_accuracy: f64,
    term_reason: NdTerminationReason,
}
```
- 역할: N차원 뉴턴 솔버 본체.
- 필드:
    - func: 실제 함수 구현체.
    - dim: 차원 수.
    - desired_tol: 강한 수렴 기준.
    - acceptable_tol: 느슨한 수렴 기준.
    - max_iter: 최대 반복 횟수.
    - bounds: 각 변수의 최소/최대 범위.
    - found_accuracy: 마지막 반복에서의 잔차 노름.
    - term_reason: 종료 이유.


## 📌 예시
- 비선형 방정식:

$$
f_1(x,y)=x^2+y^2-1=0
$$

$$
f_2(x,y)=x-y=0
$$

- LocalSolveNd는 위 시스템을 풀어  같은 해를 찾아냅니다.

### 4. solve 메서드
```rust
pub fn solve(&mut self, x0: &[f64]) -> Result<(bool, Vec<f64>), ()>
```

- 역할: 초기값 x0에서 시작해 Newton 반복을 수행.
- 절차:
- 초기값을 bounds 안으로 클램프.
- 반복 시작:
    - evaluate(x) 호출 → F(x),J(x).
    - 잔차 노름 \| F(x)\|  계산.
- 수렴 판정:
    - desired_tol 이하 → Converged.
    - acceptable_tol 이하 → Close.
    - Newton 스텝 계산: $J\cdot dx=-F(x)$.
    - 스텝 크기 제한: $\| dx\|$ 가 도메인 폭의 1/4 이상이면 스케일링.
    - $x\leftarrow x+dx$.
    - bounds 클램프.
    - 스텝이 너무 작으면 종료.
    - 반복 횟수 초과 시 MaxIterations 종료.

### 5. 보조 함수
- euclidean_norm
  - 역할: 벡터의 2-노름 $\| v\| _2=\sqrt{\sum v_i^2}$.
```rust
fn euclidean_norm(v: &[f64]) -> f64
```

- max_abs
  - 역할: 벡터의 무한 노름 $\| v\| _{\infty }=\max |v_i|$.****
```rust
fn max_abs(v: &[f64]) -> f64
```

- solve_linear_system_dense
  - 역할: 자코비안 행렬 J와 RHS 벡터를 받아 선형 시스템 Jdx=rhs를 푸는 함수.
  - 방법: 부분 피벗 Gaussian elimination.
  - 출력: 해 벡터 dx, 실패 시 None.

```rust
fn solve_linear_system_dense(jac: &[f64], rhs: &[f64], n: usize) -> Option<Vec<f64>>
```

## 🎯 정리
- 수식: Newton-Raphson 반복
  - 수렴 기준: \| F(x)\|  ≤ desired_tol 또는 acceptable_tol.
  - 보조 기능: 스텝 제한, bounds 클램프, 종료 이유 기록.
  - 핵심 함수:
      - evaluate: 사용자 함수 평가.
      - solve: Newton 반복.
      - solve_linear_system_dense: 선형 시스템 풀이.
      - euclidean_norm, max_abs: 노름 계산.

$$
J(x)\cdot dx=-F(x),\quad x\leftarrow x+dx
$$

---

## 소스
```rust
use std::f64::EPSILON;

/// N차원 함수 F: R^n -> R^n 평가 인터페이스
///
/// - 입력: x (길이 n)
/// - 출력:
///   - f (길이 n): F(x)
///   - jac (길이 n*n): 자코비안 J(x) 을 row-major 로 저장 (J[i,j] = jac[i*n + j])
///   - converged: 내부에서 "이미 충분히 해를 찾았다"고 판단한 경우 true (보통 f 노름 기준)
pub trait EvalFunctionNd {
    fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()>;
}
```
```rust
/// 종료 이유
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NdTerminationReason {
    NotStarted,
    Converged,       // ||F(x)|| <= desired_tol
    Close,           // ||F(x)|| <= acceptable_tol
    Stagnation,      // 잔차는 큼 + 스텝이 매우 작아 더 못 움직이는 상황
    BadJacobian,     // 자코비안이 거의 특이(singular) 또는 NaN/Inf
    OutOfBounds,     // bounds 밖으로 나가서 중단(엄격 제약일 때)
    MaxIterations,   // 최대 반복 초과
    EvaluationFailed // 함수 평가 실패
}
```
```rust
/// N차원 국부 해 찾기 (뉴턴/가우스-뉴턴)
///
/// F(x) = 0 형태의 시스템을 푼다.
/// - 자코비안이 제공된다고 가정(EvalFunctionNd 에서 함께 반환)
/// - 간단한 step scaling + 백트래킹 라인서치 포함
/// - 필요하면 bounds 로 각 변수의 최소/최대를 줄 수 있음
pub struct LocalSolveNd<F: EvalFunctionNd> {
    func: F,
    dim: usize,

    desired_tol: f64,
    acceptable_tol: f64,
    relative_tol: f64,
    step_tol: f64,
    max_iter: usize,

    // 선택: 변수별 경계 (None 이면 무제한)
    bounds: Option<Vec<(f64, f64)>>,

    // 라인서치 사용 여부
    use_line_search: bool,

    // 상태 조회용
    found_accuracy: f64,
    term_reason: NdTerminationReason,
}
```
```rust
impl<F: EvalFunctionNd> LocalSolveNd<F> {
    /// dim: 미지수의 차원 (x의 길이)
    pub fn new(func: F, dim: usize) -> Self {
        Self {
            func,
            dim,
            desired_tol: 1.0e-12,
            acceptable_tol: 1.0e-9,
            relative_tol: 0.0,     // 기본: 상대 기준 끔
            step_tol: 1.0e-12,     // 스텝 정지 기준(무한 노름)
            max_iter: 50,
            bounds: None,
            use_line_search: true, // 기본: 라인서치 사용
            found_accuracy: f64::INFINITY,
            term_reason: NdTerminationReason::NotStarted,
        }
    }
```
```rust
    /// 원하는 정확도(강한 기준)
    pub fn set_desired_accuracy(&mut self, tol: f64) -> &mut Self {
        self.desired_tol = tol.max(0.0);
        self
    }
```
```rust
    /// 허용 가능한 정확도(조금 느슨한 기준)
    pub fn set_acceptable_accuracy(&mut self, tol: f64) -> &mut Self {
        self.acceptable_tol = tol.max(0.0);
        self
    }
```
```rust
    /// 상대 잔차 기준 (초기 잔차 대비)
    pub fn set_relative_accuracy(&mut self, rel_tol: f64) -> &mut Self {
        self.relative_tol = rel_tol.max(0.0);
        self
    }
```
```rust
    /// 스텝 정지 기준 (무한 노름)
    pub fn set_step_tolerance(&mut self, step_tol: f64) -> &mut Self {
        self.step_tol = step_tol.max(0.0);
        self
    }
```
```rust
    /// 최대 반복 횟수
    pub fn set_max_iterations(&mut self, max_iter: usize) -> &mut Self {
        self.max_iter = max_iter.max(1);
        self
    }
```
```rust
    /// 변수별 경계 설정 (길이 dim 인 (min,max) 튜플 벡터)
    pub fn set_bounds(&mut self, bounds: Vec<(f64, f64)>) -> &mut Self {
        if bounds.len() == self.dim {
            self.bounds = Some(bounds);
        }
        self
    }
```
```rust
    /// 라인서치 on/off
    pub fn enable_line_search(&mut self, enable: bool) -> &mut Self {
        self.use_line_search = enable;
        self
    }
```
```rust
    /// 마지막 solve 에서의 종료 이유
    pub fn termination_reason(&self) -> NdTerminationReason {
        self.term_reason
    }
```
```rust
    /// 마지막 solve 에서의 ||F(x)|| 값
    pub fn found_accuracy(&self) -> f64 {
        self.found_accuracy
    }
```
```rust
    /// 메인 solve:
    /// - 입력: 초기값 x0 (길이 dim)
    /// - 출력: (성공 여부, 해 x)
    pub fn solve(&mut self, x0: &[f64]) -> Result<(bool, Vec<f64>), ()> {
        assert_eq!(
            x0.len(),
            self.dim,
            "LocalSolveNd::solve: x0 length must match dim"
        );

        let mut x = x0.to_vec();

        // 초기값을 bounds 안으로 클램프
        if let Some(bounds) = &self.bounds {
            for i in 0..self.dim {
                let (min_i, max_i) = bounds[i];
                x[i] = x[i].clamp(min_i.min(max_i), min_i.max(max_i));
            }
        }

        self.term_reason = NdTerminationReason::NotStarted;
        self.found_accuracy = f64::INFINITY;

        // 초기 평가
        let (mut f, mut jac, mut func_converged) = match self.func.evaluate(&x) {
            Ok(v) => v,
            Err(_) => {
                self.term_reason = NdTerminationReason::EvaluationFailed;
                return Ok((false, x));
            }
        };
        if any_nan_inf(&f) || any_nan_inf(&jac) {
            self.term_reason = NdTerminationReason::BadJacobian;
            return Ok((false, x));
        }

        assert_eq!(f.len(), self.dim, "EvalFunctionNd: f length != dim");
        assert_eq!(jac.len(), self.dim * self.dim, "EvalFunctionNd: jac length != dim*dim");

        let mut norm_f0 = euclidean_norm(&f);
        let mut norm_f = norm_f0;
        self.found_accuracy = norm_f;

        // 메인 반복
        for iter in 0..self.max_iter {
            // (1) 수렴 판정
            let residual_goal = self.desired_tol + self.relative_tol * norm_f0;
            if func_converged && norm_f <= self.acceptable_tol {
                self.term_reason = if norm_f <= residual_goal {
                    NdTerminationReason::Converged
                } else {
                    NdTerminationReason::Close
                };
                return Ok((true, x));
            }
            if norm_f <= residual_goal {
                self.term_reason = NdTerminationReason::Converged;
                return Ok((true, x));
            }
            if norm_f <= self.acceptable_tol {
                self.term_reason = NdTerminationReason::Close;
                return Ok((true, x));
            }

            // (2) Newton 스텝: J * dx = -f
            let mut rhs = vec![0.0; self.dim];
            for i in 0..self.dim {
                rhs[i] = -f[i];
            }
            let dx_opt = solve_linear_system_dense(&jac, &rhs, self.dim);
            let mut dx = match dx_opt {
                Some(v) => v,
                None => {
                    // 자코비안 특이 또는 비정상
                    self.term_reason = NdTerminationReason::BadJacobian;
                    return Ok((false, x));
                }
            };
            if any_nan_inf(&dx) {
                self.term_reason = NdTerminationReason::BadJacobian;
                return Ok((false, x));
            }

            // (3) 스텝 크기 제한 (trust-like scaling)
            let mut max_width = 1.0;
            if let Some(bounds) = &self.bounds {
                for i in 0..self.dim {
                    let w = (bounds[i].1 - bounds[i].0).abs();
                    if w > max_width {
                        max_width = w;
                    }
                }
            }
            let max_step = 0.25 * max_width;
            let dx_norm = euclidean_norm(&dx);
            if dx_norm > max_step && dx_norm > EPSILON {
                let scale = max_step / dx_norm;
                for i in 0..self.dim {
                    dx[i] *= scale;
                }
            }

            // (4) 정체 판정(스텝 매우 작은데 잔차 큼)
            let dx_norm_inf = max_abs(&dx);
            if dx_norm_inf <= self.step_tol && norm_f > self.acceptable_tol {
                self.term_reason = NdTerminationReason::Stagnation;
                return Ok((false, x));
            }

            // (5) 백트래킹 라인서치
            if self.use_line_search {
                let mut alpha = 1.0;
                let max_backtracks = 8;
                let mut accepted = false;

                for _ in 0..max_backtracks {
                    let mut x_trial = x.clone();
                    for i in 0..self.dim {
                        x_trial[i] += alpha * dx[i];
                    }
                    // 경계 클램프
                    if let Some(bounds) = &self.bounds {
                        for i in 0..self.dim {
                            let (min_i, max_i) = bounds[i];
                            x_trial[i] = x_trial[i].clamp(min_i.min(max_i), min_i.max(max_i));
                        }
                    }

                    let eval_trial = self.func.evaluate(&x_trial);
                    let (f_trial, jac_trial, found_trial) = match eval_trial {
                        Ok(v) => v,
                        Err(_) => break, // 평가 실패 → 라인서치 중단
                    };
                    if any_nan_inf(&f_trial) || any_nan_inf(&jac_trial) {
                        break;
                    }

                    let f_norm_trial = euclidean_norm(&f_trial);
                    if f_norm_trial <= norm_f {
                        // 수용
                        x = x_trial;
                        f = f_trial;
                        jac = jac_trial;
                        func_converged = found_trial;
                        norm_f = f_norm_trial;
                        self.found_accuracy = norm_f;
                        accepted = true;
                        break;
                    }
                    alpha *= 0.5;
                }

                // 라인서치 실패 시, 한 번은 원 스텝을 적용(정책에 따라 조정 가능)
                if !accepted {
                    for i in 0..self.dim {
                        x[i] += dx[i];
                    }
                    if let Some(bounds) = &self.bounds {
                        for i in 0..self.dim {
                            let (min_i, max_i) = bounds[i];
                            x[i] = x[i].clamp(min_i.min(max_i), min_i.max(max_i));
                        }
                    }
                    // 재평가
                    let eval_now = self.func.evaluate(&x);
                    let (f_now, jac_now, found_now) = match eval_now {
                        Ok(v) => v,
                        Err(_) => {
                            self.term_reason = NdTerminationReason::EvaluationFailed;
                            return Ok((false, x));
                        }
                    };
                    if any_nan_inf(&f_now) || any_nan_inf(&jac_now) {
                        self.term_reason = NdTerminationReason::BadJacobian;
                        return Ok((false, x));
                    }
                    f = f_now;
                    jac = jac_now;
                    func_converged = found_now;
                    norm_f = euclidean_norm(&f);
                    self.found_accuracy = norm_f;
                }
            } else {
                // 라인서치 비활성: 바로 업데이트
                for i in 0..self.dim {
                    x[i] += dx[i];
                }
                if let Some(bounds) = &self.bounds {
                    for i in 0..self.dim {
                        let (min_i, max_i) = bounds[i];
                        x[i] = x[i].clamp(min_i.min(max_i), min_i.max(max_i));
                    }
                }
                // 재평가
                let eval_now = self.func.evaluate(&x);
                let (f_now, jac_now, found_now) = match eval_now {
                    Ok(v) => v,
                    Err(_) => {
                        self.term_reason = NdTerminationReason::EvaluationFailed;
                        return Ok((false, x));
                    }
                };
                if any_nan_inf(&f_now) || any_nan_inf(&jac_now) {
                    self.term_reason = NdTerminationReason::BadJacobian;
                    return Ok((false, x));
                }
                f = f_now;
                jac = jac_now;
                func_converged = found_now;
                norm_f = euclidean_norm(&f);
                self.found_accuracy = norm_f;
            }

            // 다음 반복으로...
            if iter + 1 == self.max_iter {
                self.term_reason = NdTerminationReason::MaxIterations;
                return Ok((false, x));
            }
        }

        // 여기까지 올 일은 거의 없음
        self.term_reason = NdTerminationReason::MaxIterations;
        Ok((false, x))
    }
}
```
```rust
/// 유클리드 노름 ||v||
fn euclidean_norm(v: &[f64]) -> f64 {
    let mut s = 0.0;
    for &x in v {
        s += x * x;
    }
    s.sqrt()
}
```
```rust
/// 무한 노름 (최대 절대값)
fn max_abs(v: &[f64]) -> f64 {
    let mut m = 0.0;
    for &x in v {
        let ax = x.abs();
        if ax > m {
            m = ax;
        }
    }
    m
}
```
```rust
/// NaN/Inf 검사
fn any_nan_inf(v: &[f64]) -> bool {
    v.iter().any(|&x| !x.is_finite())
}
```
```rust
/// 단순 dense 선형시스템 풀이 (부분 피벗 Gaussian elimination)
///
/// jac: 길이 n*n (row-major)
/// rhs: 길이 n
fn solve_linear_system_dense(jac: &[f64], rhs: &[f64], n: usize) -> Option<Vec<f64>> {
    if jac.len() != n * n || rhs.len() != n {
        return None;
    }

    // A, b 복사
    let mut a = vec![0.0; n * n];
    a.copy_from_slice(jac);
    let mut b = rhs.to_vec();

    // forward elimination with partial pivoting
    for k in 0..n {
        // pivot row 선택
        let mut piv = k;
        let mut max_abs = a[k * n + k].abs();
        for i in (k + 1)..n {
            let val = a[i * n + k].abs();
            if val > max_abs {
                max_abs = val;
                piv = i;
            }
        }

        // 동적/완화된 임계값
        if !max_abs.is_finite() || max_abs <= 1.0e-14 {
            return None; // 거의 특이 또는 비정상
        }

        if piv != k {
            // 행 교환
            for j in 0..n {
                a.swap(k * n + j, piv * n + j);
            }
            b.swap(k, piv);
        }

        let akk = a[k * n + k];
        if !akk.is_finite() || akk.abs() <= 1.0e-14 {
            return None;
        }

        // 제거
        for i in (k + 1)..n {
            let factor = a[i * n + k] / akk;
            a[i * n + k] = factor;
            for j in (k + 1)..n {
                a[i * n + j] -= factor * a[k * n + j];
            }
            b[i] -= factor * b[k];
        }
    }

    // back substitution
    for i in (0..n).rev() {
        let mut s = b[i];
        for j in (i + 1)..n {
            s -= a[i * n + j] * b[j];
        }
        let diag = a[i * n + i];
        if !diag.is_finite() || diag.abs() <= 1.0e-14 {
            return None;
        }
        b[i] = s / diag;
    }

    Some(b)
}
```
```rust
#[cfg(test)]
mod tests {
    use super::*;

    /// 1) 선형 시스템 테스트
    struct Linear2D;
    impl EvalFunctionNd for Linear2D {
        fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()> {
            // A = [[3, 1],[1, 2]], b = [1, 0] → F(x) = A x - b
            let f0 = 3.0 * x[0] + 1.0 * x[1] - 1.0;
            let f1 = 1.0 * x[0] + 2.0 * x[1] - 0.0;
            let jac = vec![
                3.0, 1.0,
                1.0, 2.0,
            ];
            let norm = (f0 * f0 + f1 * f1).sqrt();
            Ok((vec![f0, f1], jac, norm < 1e-14))
        }
    }
```
```rust
    #[test]
    fn nd_linear_solves() {
        let mut solver = LocalSolveNd::new(Linear2D, 2);
        solver
            .set_desired_accuracy(1e-12)
            .set_acceptable_accuracy(1e-9)
            .set_step_tolerance(1e-12)
            .set_max_iterations(20)
            .enable_line_search(true);

        let (ok, x) = solver.solve(&[0.0, 0.0]).unwrap();
        assert!(ok, "linear solve did not converge: {:?}", solver.termination_reason());
        // Exact solution: x = [0.2, -0.1]
        assert!((x[0] - 0.2).abs() < 1e-12);
        assert!((x[1] + 0.1).abs() < 1e-12);
    }

    /// 2) 비선형 시스템 + 경계 + 라인서치
    struct CircleLine;
    impl EvalFunctionNd for CircleLine {
        fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()> {
            let (u, v) = (x[0], x[1]);
            // F = [u^2 + v^2 - 1, u - v]
            let f0 = u * u + v * v - 1.0;
            let f1 = u - v;
            let jac = vec![
                2.0 * u, 2.0 * v,
                1.0,     1.0,
            ];
            let norm = (f0 * f0 + f1 * f1).sqrt();
            Ok((vec![f0, f1], jac, norm < 1e-14))
        }
    }
```
```rust
    #[test]
    fn nd_nonlinear_with_bounds() {
        let mut solver = LocalSolveNd::new(CircleLine, 2);
        solver
            .set_desired_accuracy(1e-10)
            .set_acceptable_accuracy(1.0e-8)
            .set_step_tolerance(1e-12)
            .set_relative_accuracy(0.0)
            .set_max_iterations(50)
            .set_bounds(vec![(-2.0, 2.0), (-2.0, 2.0)])
            .enable_line_search(true);

        let (ok, x) = solver.solve(&[0.9, 0.1]).unwrap();
        assert!(ok, "nonlinear solve did not converge: {:?}", solver.termination_reason());
        // Solutions: (√(1/2), √(1/2)) or negatives; near 0.9,0.1 we expect positive
        let s = (0.5f64).sqrt();
        assert!((x[0] - s).abs() < 1e-6, "x0={} expected~{}", x[0], s);
        assert!((x[1] - s).abs() < 1e-6, "x1={} expected~{}", x[1], s);
    }
```
```rust
    /// 3) 특이 자코비안 케이스
    struct Singular;
    impl EvalFunctionNd for Singular {
        fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()> {
            // F = [x0^2, 0]; Jacobian singular at x0=0
            let f0 = x[0] * x[0];
            let f1 = 0.0;
            let jac = vec![
                2.0 * x[0], 0.0,
                0.0,         0.0,
            ];
            Ok((vec![f0, f1], jac, f0.abs() < 1e-14))
        }
    }
```
```rust
    #[test]
    fn nd_bad_jacobian() {
        let mut solver = LocalSolveNd::new(Singular, 2);
        solver.set_max_iterations(5);

        let (ok, _x) = solver.solve(&[0.0, 0.0]).unwrap();
        assert!(!ok, "expected failure on singular Jacobian");
        assert_eq!(solver.termination_reason(), NdTerminationReason::BadJacobian);
    }
```
```rust
    /// 4) 정체(Stagnation) 케이스: 잔차가 큰데 스텝이 매우 작은 상황
    struct FlatResidual;
    impl EvalFunctionNd for FlatResidual {
        fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()> {
            // 거의 평탄한 함수: F = [10.0, 10.0] (상수), J = 작은 값
            let f = vec![10.0, 10.0];
            let jac = vec![
                1.0e-16, 0.0,
                0.0,     1.0e-16,
            ];
            Ok((f, jac, false))
        }
    }
```
```rust
    #[test]
    fn nd_stagnation_detected() {
        let mut solver = LocalSolveNd::new(FlatResidual, 2);
        solver
            .set_desired_accuracy(1e-12)
            .set_acceptable_accuracy(1e-9)
            .set_step_tolerance(1e-12)
            .set_max_iterations(10)
            .enable_line_search(true);

        let (ok, _x) = solver.solve(&[0.0, 0.0]).unwrap();
        assert!(!ok, "expected stagnation/failure");
        assert_eq!(solver.termination_reason(), NdTerminationReason::Stagnation);
    }
}
```
```rust
// 1) 선형 2D
struct Linear2D;
impl EvalFunctionNd for Linear2D {
    fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()> {
        let f0 = 3.0*x[0] + 1.0*x[1] - 1.0;
        let f1 = 1.0*x[0] + 2.0*x[1] - 0.0;
        let jac = vec![3.0, 1.0, 1.0, 2.0];
        let norm = (f0*f0 + f1*f1).sqrt();
        Ok((vec![f0, f1], jac, norm < 1e-14))
    }
}
```
```rust
#[test]
fn nd_linear_solves() {
    let mut solver = LocalSolveNd::new(Linear2D, 2)
        .set_desired_accuracy(1e-12)
        .set_acceptable_accuracy(1e-9)
        .set_max_iterations(20);
    let (ok, x) = solver.solve(&[0.0, 0.0]).unwrap();
    assert!(ok);
    assert!((x[0] - 0.2).abs() < 1e-12);
    assert!((x[1] + 0.1).abs() < 1e-12);
}
```
```rust
// 2) 비선형 + 경계
struct CircleLine;
impl EvalFunctionNd for CircleLine {
    fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()> {
        let (u, v) = (x[0], x[1]);
        let f0 = u*u + v*v - 1.0;
        let f1 = u - v;
        let jac = vec![2.0*u, 2.0*v, 1.0, 1.0];
        let norm = (f0*f0 + f1*f1).sqrt();
        Ok((vec![f0, f1], jac, norm < 1e-14))
    }
}
```
```rust
#[test]
fn nd_nonlinear_with_bounds() {
    let mut solver = LocalSolveNd::new(CircleLine, 2)
        .set_desired_accuracy(1e-10)
        .set_acceptable_accuracy(1.0e-8)
        .set_max_iterations(50)
        .set_bounds(vec![(-2.0, 2.0), (-2.0, 2.0)]);
    let (ok, x) = solver.solve(&[0.9, 0.1]).unwrap();
    assert!(ok);
    let s = (0.5f64).sqrt();
    assert!((x[0] - s).abs() < 1e-6);
    assert!((x[1] - s).abs() < 1e-6);
}
```
```rust
// 3) 특이 자코비안
struct Singular;
impl EvalFunctionNd for Singular {
    fn evaluate(&mut self, x: &[f64]) -> Result<(Vec<f64>, Vec<f64>, bool), ()> {
        let f0 = x[0]*x[0];
        let f1 = 0.0;
        let jac = vec![2.0*x[0], 0.0, 0.0, 0.0];
        Ok((vec![f0, f1], jac, f0.abs() < 1e-14))
    }
}
```
```rust
#[test]
fn nd_bad_jacobian() {
    let mut solver = LocalSolveNd::new(Singular, 2)
        .set_max_iterations(5);
    let (ok, _x) = solver.solve(&[0.0, 0.0]).unwrap();
    assert!(!ok);
    assert_eq!(solver.termination_reason(), NdTerminationReason::BadJacobian);
}
```
---

