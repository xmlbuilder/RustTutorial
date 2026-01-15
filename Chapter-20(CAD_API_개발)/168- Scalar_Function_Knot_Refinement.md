# B-spline scalar function knot refinement
- 이 문서는 CFun::cfun_refine_with_insert_list와 그에 대한 테스트들을 기반으로,  
    스칼라 B-spline 함수의 knot refinement 연산을 수학적으로 정리한 것이다.

## 1. 개요
- CFun은 1D B-spline 스칼라 함수:
```math
F(u)=\sum _{i=0}^nf_i\, N_{i,p}(u)
```
- p: 차수 (degree)
- $f_i$: control value
- $N_{i,p}(u)$: B-spline basis
- $knot vector: \{ U_0,\dots ,U_m\}$
- cfun_refine_with_insert_list는 주어진 knot vector 내부에  
    새로운 knot 집합 $X=\{ x_0,\dots ,x_r\}$ 를 삽입하여
```math
F'(u)=\sum _{i=0}^{n+r}f'_i\, N'_{i,p}(u)
```
- 을 구성한다.
- 함수 값 F(u)는 모든 u에서 그대로 유지되고, 표현만 더 세밀한 B-spline으로 바뀐다.

## 2. 수학적 정의
### 2.1 입력과 출력
- 입력:
    - 차수 p
    - control 값 $\{ f_i\} _{i=0}^n$
    - knot vector $\{ U_j\} _{j=0}^m$
    - 삽입 knot 집합 $X=\{ x_0,\dots ,x_r\}$ , 정렬된 상태, 모두 내부: $U_0<x_k<U_m$

- 출력:
    - 새로운 control 값 $\{ f'_i\} _{i=0}^{n+r}$
    - 새로운 knot vector $\{ U'_j\} _{j=0}^{m+r+1}$
- 동일한 함수:
```math
F(u)=F'(u)\quad \forall u
```
### 2.2 단일 knot 삽입의 Boehm 공식
- B-spline 함수에 knot x를 한 번 삽입할 때,
- 해당 구간의 control 값은 다음과 같이 갱신된다:
```math
\alpha_i =
\begin{cases}
\dfrac{U_{i+p} - x}{\,U_{i+p} - U_i\,}, & U_i < U_{i+p} \\
0, & \text{degenerate case}
\end{cases}
``` 
```math
f'_i=\alpha _if_i+(1-\alpha _i)f_{i-1}
```
- 이 연산을 적절한 인덱스 범위에서 반복 적용하면,  
    knot 삽입 후에도 동일한 함수 F(u)를 유지할 수 있다.
- cfun_refine_with_insert_list는 이 Boehm 공식의 다중 knot 삽입 버전을  
    역순(reverse)으로 구현한 것이다.

## 3. 알고리즘 개요
### 3.1 전제 조건
```rust
// strict endpar policy
// U[0] < X[0] <= ... <= X[r] < U[m]
if !(u0 < xi && xi < um) { Err(...) }
```

- 모든 삽입 knot은 strict interior:
```math
U_0<x_k<U_m
```
- end knot multiplicity를 건드리지 않기 위한 정책.

### 3.2 출력 크기
- control 값 개수:
```math
n'=n+r+1
```
- knot 개수:
```math
m'=m+r+1
```
- 코드:
```rust
let mut fq = vec![0.0; (n + r + 1) + 1];
let mut uq = vec![0.0; (m + r + 1) + 1];
```


### 3.3 span 찾기
```rust
let a = on_find_span_left_slice(up, p, x[0])?;
let mut b = on_find_span_left_slice(up, p, x[r])?;
b += 1;
```

- a: 첫 삽입 knot $x_0$ 가 속한 span
- b: 마지막 삽입 knot $x_r$ 이후 span
- 수식적으로:
```math
U_a\leq x_0<U_{a+1},\quad U_b\leq x_r<U_{b+1}
```
### 3.4 삽입 구간 밖의 값 복사
```rust
for j in 0..=a { uq[j] = up[j]; }
for j in (b + p)..=m { uq[j + r + 1] = up[j]; }

for j in 0..=(a - p) { fq[j] = fp[j]; }
for j in (b - 1)..=n { fq[j + r + 1] = fp[j]; }
```
- 삽입 구간 밖의 knot와 control 값은 그대로 유지.

## 3.5 역순 knot 삽입 루프
- 핵심 부분:
```rust
let mut i = b + p - 1;
let mut k = b + p + r;

for jj in (0..=r).rev() {
    while x[jj] <= up[i] && i > a {
        fq[k - p - 1] = fp[i - p - 1];
        uq[k] = up[i];
        k -= 1;
        i -= 1;
    }

    fq[k - p - 1] = fq[k - p];

    for l in 1..=p {
        let t = k - p + l;
        let mut alf = uq[k + l] - x[jj];
        if alf == 0.0 {
            fq[t - 1] = fq[t];
        } else {
            alf = alf / (uq[k + l] - up[i - p + l]);
            let oma = 1.0 - alf;
            fq[t - 1] = oma * fq[t] + alf * fq[t - 1];
        }
    }

    uq[k] = x[jj];
    k -= 1;
}
```
- 이 루프는 다음을 수행한다:
    - 기존 knot를 뒤로 밀어내며 복사:
    ```math
    U'_k=U_i,\quad f'_{k-p-1}=f_{i-p-1}
    ```
    - Boehm 공식에 따른 control 값 갱신:
    ```math
    \alpha =\frac{U'_{k+l}-x_{jj}}{U'_{k+l}-U_{i-p+l}}f'_{t-1}=(1-\alpha )f'_t+\alpha f'_{t-1}
    ```
    - 새 knot 삽입:
    ```math
    U'_k=x_{jj}
    ```
    - 이 과정을 삽입 knot들을 역순으로 진행함으로써,  
        메모리 오염 없이 안정적으로 다중 삽입을 수행한다.

## 4. 테스트가 보장하는 것
- 테스트 모듈 refine_tests는 다음 성질들을 검증한다.
### 4.1 값 보존 (shape preservation)
```rust
for u in sample_params_unit(120) {
    let f0 = cfn.eval_val(u);
    let f1 = refined.eval_val(u);
    assert_near(f0, f1, 5e-12, ...);
}
```
- 모든 테스트에서 eval_val(u)가 원본과 refined에서 거의 동일함을 확인.
- 수학적으로:
```math
F(u)=F'(u)\quad \forall u\in [0,1]
```
### 4.2 knot vector 속성
- 길이 증가:
```math
|\mathrm{knots}'|=|\mathrm{knots}|+|X|
```
- 비감소성:
```rust
assert_non_decreasing(refined.knots.as_slice());
```
- 중복 삽입 시 multiplicity 증가:
```rust
assert!(count_value(uq, 0.4) >= 3);
```

### 4.3 경계 조건
- 정확한 경계 값 삽입은 거부:
```rust
assert!(on_cfun_refine_with_insert_list(&cfn, &[u0]).is_err());
assert!(on_cfun_refine_with_insert_list(&cfn, &[um]).is_err());
```
- 경계 근처(내부) 값은 허용:
```rust
let x = vec![0.0 + 10.0*eps, 1.0 - 10.0*eps];
```


### 4.4 조합 성질 (composition)
```rust
let c1 = refine(cfn, x1);
let c2 = refine(c1, x2);

let x12 = sorted(x1 ∪ x2);
let once = refine(cfn, x12);

eval(c2, u) ≈ eval(once, u)
```

- 여러 번 refinement를 적용한 결과와  
    한 번에 모든 knot을 삽입한 결과가 동일함을 검증.

### 4.5 랜덤 스트레스 테스트
- 다양한 degree, control 값, 삽입 knot에 대해  
    반복적으로 값 보존을 확인.
- 수치적 안정성과 구현의 일반성을 검증.

## 5. 요약
- cfun_refine_with_insert_list는 B-spline 스칼라 함수의 다중 knot 삽입 알고리즘이다.
- Boehm knot insertion 공식을 기반으로,
- 형상을 완전히 보존하면서 knot vector와 control 값을 재구성한다.
- 테스트들은:
    - 값 보존
    - knot 비감소성
    - multiplicity 증가
    - 경계 정책
    - 조합 성질
    - 랜덤 케이스
- 를 통해 수학적/구현적 정당성을 강하게 뒷받침한다.

---
## 소스 코드
```rust
pub fn on_cfun_refine_with_insert_list(cfn: &CFun, insert_vec: &[Real]) -> Result<CFun> {
    cfn.cfun_refine_with_insert_list(insert_vec)
}

pub fn cfun_refine_with_insert_list(&self, insert_vec: &[Real]) -> Result<CFun> {
    if insert_vec.is_empty() {
        return Ok(self.clone());
    }

    let p = self.p as usize;
    let fp = self.fu.as_slice();
    let n = fp.len() - 1;

    let up = self.knots.as_slice();
    let m = up.len() - 1;

    // X
    let x = insert_vec;
    let r = x.len() - 1;

    // ---- strict endpar policy (E_endpar equivalent) ----
    // N_TOOCFR assumes: U[0] < X[0] <= ... <= X[r] < U[m]
    // -> we enforce strict inside for all inserted knots to avoid boundary insertion.
    let u0 = up[0];
    let um = up[m];
    for &xi in x {
        if !(u0 < xi && xi < um) {
            return Err(NurbsError::InvalidArgument {
                msg: "refine knots must satisfy U0 < X < Um".into(),
            });
        }
    }

    // Allocate output arrays (sizes match C)
    let mut fq = vec![0.0; (n + r + 1) + 1];
    let mut uq = vec![0.0; (m + r + 1) + 1];

    // Find spans a,b (LEFT), and b++
    let a = on_find_span_left_slice(up, p, x[0])?;
    let mut b = on_find_span_left_slice(up, p, x[r])?;
    b += 1;

    // Initialize output knot vector
    for j in 0..=a {
        uq[j] = up[j];
    }
    for j in (b + p)..=m {
        uq[j + r + 1] = up[j];
    }

    // Save unaltered control values
    for j in 0..=(a - p) {
        fq[j] = fp[j];
    }
    for j in (b - 1)..=n {
        fq[j + r + 1] = fp[j];
    }

    // Refine (reverse insertion)
    let mut i = b + p - 1;
    let mut k = b + p + r;

    for jj in (0..=r).rev() {
        while x[jj] <= up[i] && i > a {
            fq[k - p - 1] = fp[i - p - 1]; // IMPORTANT: from original fp
            uq[k] = up[i];
            k -= 1;
            i -= 1;
        }

        fq[k - p - 1] = fq[k - p];

        for l in 1..=p {
            let t = k - p + l;
            let mut alf = uq[k + l] - x[jj];
            if alf == 0.0 {
                fq[t - 1] = fq[t];
            } else {
                alf = alf / (uq[k + l] - up[i - p + l]);
                let oma = 1.0 - alf;
                fq[t - 1] = oma * fq[t] + alf * fq[t - 1];
            }
        }

        uq[k] = x[jj];
        k -= 1;
    }

    Ok(CFun::new(self.p, KnotVector::new(uq)?, fq)?)
}

```
---
###  테스트 코드
```rust

#[cfg(test)]
mod refine_tests {
    use nurbslib::core::cfun::CFun;
    use nurbslib::core::types::{Real, Degree};
    use nurbslib::core::knot::KnotVector;
    use nurbslib::core::cfun::{on_cfun_refine_with_insert_list};

    fn make_open_uniform_clamped_knots(p: usize, n: usize) -> Vec<Real> {
        // n: highest ctrl index (ctrl.len() = n+1)
        // m = n + p + 1  => knots length = m+1 = n+p+2
        let m = n + p + 1;
        let mut u = vec![0.0; m + 1];

        // clamp ends
        for i in 0..=p {
            u[i] = 0.0;
            u[m - i] = 1.0;
        }

        // number of spans = n - p + 1  (>=1)
        let spans = n.saturating_sub(p) + 1;
        if spans <= 1 {
            // no internal knots (pure Bezier)
            return u;
        }

        // internal knots indices: (p+1) .. (m-p-1)
        // there are (spans - 1) internal unique knots
        for j in 1..spans {
            let val = (j as Real) / (spans as Real);
            u[p + j] = val;
        }

        u
    }


    fn make_test_cfun(p: usize, ctrl: &[Real]) -> CFun {
        let n = ctrl.len() - 1;
        let knots = make_open_uniform_clamped_knots(p, n);
        CFun::new(p as Degree, KnotVector::new(knots).unwrap(), ctrl.to_vec()).unwrap()
    }

    fn sample_params_unit(n: usize) -> Vec<Real> {
        (0..=n).map(|i| (i as Real)/(n as Real)).collect()
    }

    fn assert_near(a: Real, b: Real, tol: Real, msg: &str) {
        let d = (a - b).abs();
        assert!(d <= tol, "{} |a-b|={} a={} b={}", msg, d, a, b);
    }

    // 결과 knot가 비감소인지 체크
    fn assert_non_decreasing(u: &[Real]) {
        for i in 0..u.len()-1 {
            assert!(u[i] <= u[i+1], "knots not non-decreasing at {}: {} > {}", i, u[i], u[i+1]);
        }
    }

    // 특정 값이 knot에 최소 count번 존재하는지
    fn count_value(u: &[Real], v: Real) -> usize {
        u.iter().filter(|&&t| t == v).count()
    }
```
```rust
    #[test]
    fn refine_basic_preserves_values_and_properties() {
        let p = 3;
        let cfn = make_test_cfun(p, &[0.2, 1.1, -0.7, 0.9, 2.0, -1.5]); // n=5
        let x = vec![0.15, 0.4, 0.8];

        let refined = on_cfun_refine_with_insert_list(&cfn, &x).unwrap();

        // 길이 체크
        assert_eq!(refined.fu.len(), cfn.fu.len() + x.len());
        assert_eq!(refined.knots.as_slice().len(), cfn.knots.as_slice().len() + x.len());

        // knot 비감소
        assert_non_decreasing(refined.knots.as_slice());

        // 값 불변
        for u in sample_params_unit(120) {
            let f0 = cfn.eval_val(u);
            let f1 = refined.eval_val(u);
            assert_near(f0, f1, 5e-12, "refine changed function value");
        }
    }
```
```rust    
    #[test]
    fn refine_with_duplicates_preserves_values_and_increases_multiplicity() {
        let p = 3;
        let cfn = make_test_cfun(p, &[1.0, -2.0, 0.5, 3.0, -1.0, 2.2]);
        let x = vec![0.4, 0.4, 0.4]; // triple insert

        let refined = on_cfun_refine_with_insert_list(&cfn, &x).unwrap();

        // 0.4가 최소 3번 들어갔는지 (원래 knot에 있을 수도 있으니 >=)
        let uq = refined.knots.as_slice();
        assert!(count_value(uq, 0.4) >= 3, "expected inserted knot multiplicity >= 3");

        for u in sample_params_unit(120) {
            assert_near(cfn.eval_val(u), refined.eval_val(u), 5e-12, "duplicate refine changed value");
        }
    }
```
```rust    
    #[test]
    fn refine_rejects_exact_boundaries() {
        let p = 3;
        let cfn = make_test_cfun(p, &[0.2, 1.1, -0.7, 0.9, 2.0, -1.5]);
        let up = cfn.knots.as_slice();
        let u0 = up[0];
        let um = up[up.len()-1];

        assert!(on_cfun_refine_with_insert_list(&cfn, &[u0]).is_err());
        assert!(on_cfun_refine_with_insert_list(&cfn, &[um]).is_err());
    }
```
```rust    
    #[test]
    fn refine_accepts_near_boundaries_inside() {
        let p = 3;
        let cfn = make_test_cfun(p, &[0.2, 1.1, -0.7, 0.9, 2.0, -1.5]);
        let eps = 1e-14;

        let x = vec![0.0 + 10.0*eps, 1.0 - 10.0*eps];
        let refined = on_cfun_refine_with_insert_list(&cfn, &x).unwrap();

        for u in sample_params_unit(80) {
            assert_near(cfn.eval_val(u), refined.eval_val(u), 5e-12, "near boundary refine changed value");
        }
    }
```
```rust    
    #[test]
    fn refine_unsorted_x_should_error_or_be_documented() {
        let p = 3;
        let cfn = make_test_cfun(p, &[0.3, 1.2, -0.4, 2.0, 0.7, 1.1]);

        let x_unsorted = vec![0.8, 0.2, 0.5];

        let r = on_cfun_refine_with_insert_list(&cfn, &x_unsorted);

        // 여기서 "무조건 Err"로 만들고 싶으면 구현에서 정렬 검사 추가하면 됨.
        // 일단은 결과를 강제하지 않고, 현재 동작을 확인하는 용도.
        if let Ok(refined) = r {
            // 성공했다면, 최소한 값 불변은 만족해야 함
            for u in sample_params_unit(60) {
                assert_near(cfn.eval_val(u), refined.eval_val(u), 5e-12, "unsorted refine changed value");
            }
        }
    }
```
```rust    
    #[test]
    fn refine_inserting_existing_internal_knot() {
        let p = 3;
        let cfn = make_test_cfun(p, &[0.9, -0.2, 1.3, 0.1, -0.6, 2.0]);

        // clamped uniform이면 내부 knot 중 하나가 0.5 같은 값일 가능성이 큼
        // 확실히 내부 knot 하나를 뽑자
        let up = cfn.knots.as_slice();
        let mid = up[p + 1]; // 보통 내부 시작점
        // mid가 0이나 1일 수 있으면 조금 더 안쪽으로 이동
        let xi = if mid > 0.0 && mid < 1.0 { mid } else { 0.5 };

        let refined = on_cfun_refine_with_insert_list(&cfn, &[xi]).unwrap();
        for u in sample_params_unit(100) {
            assert_near(cfn.eval_val(u), refined.eval_val(u), 5e-12, "existing knot insertion changed value");
        }
    }
```
```rust    
    #[test]
    fn refine_twice_equals_refine_once_with_concatenated_x() {
        let p = 3;
        let cfn = make_test_cfun(p, &[0.1, 1.0, -0.3, 2.1, 0.7, -1.2]);

        let x1 = vec![0.2, 0.4];
        let x2 = vec![0.6, 0.6];

        let c1 = on_cfun_refine_with_insert_list(&cfn, &x1).unwrap();
        let c2 = on_cfun_refine_with_insert_list(&c1, &x2).unwrap();

        let mut x12 = x1.clone();
        x12.extend_from_slice(&x2);
        x12.sort_by(|a,b| a.partial_cmp(b).unwrap());

        let once = on_cfun_refine_with_insert_list(&cfn, &x12).unwrap();

        for u in sample_params_unit(120) {
            assert_near(c2.eval_val(u), once.eval_val(u), 1e-11, "refine composition mismatch");
        }
    }
```
```rust    
    fn lcg(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        *seed
    }
    fn rand01(seed: &mut u64) -> Real {
        let v = lcg(seed) >> 11;
        (v as Real) / ((1u64<<53) as Real)
    }
    #[test]
    fn refine_random_stress_preserves_values() {
        let mut seed = 1234567u64;

        for _case in 0..20 {
            let p = 2 + (lcg(&mut seed) as usize % 4); // 2..5
            let n_ctrl = 6 + (lcg(&mut seed) as usize % 8); // 6..13
            let mut ctrl = vec![0.0; n_ctrl];
            for c in &mut ctrl {
                *c = (rand01(&mut seed)*2.0 - 1.0) * 3.0;
            }
            let cfn = make_test_cfun(p, &ctrl);

            // X: 내부에서만 뽑기
            let r = 1 + (lcg(&mut seed) as usize % 8);
            let mut x = Vec::with_capacity(r);
            for _ in 0..r {
                let mut u = rand01(&mut seed);
                // strict inside
                if u <= 0.0 { u = 1e-12; }
                if u >= 1.0 { u = 1.0 - 1e-12; }
                x.push(u);
            }
            x.sort_by(|a,b| a.partial_cmp(b).unwrap());

            let refined = on_cfun_refine_with_insert_list(&cfn, &x).unwrap();
            for u in sample_params_unit(80) {
                assert_near(cfn.eval_val(u), refined.eval_val(u), 3e-11, "random refine changed value");
            }
        }
    }
}

```
---
