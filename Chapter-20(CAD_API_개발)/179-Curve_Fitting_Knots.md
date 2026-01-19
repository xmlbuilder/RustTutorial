# Curve Fitting Knots
## 📘 on_crv_fitting_knots 
- Curve Fitting Knot Vector Generation
- Averaging Method for B‑Spline / NURBS Curve Fitting

### 1. 함수 목적 (What this function does)
- 이 함수는 곡선 피팅(curve fitting) 에서 사용되는 표준 Knot Vector 생성 알고리즘이다.
- 입력으로 주어진 파라미터 값 $u_0$, $u_1,\dots ,u_k$ 를 기반으로  
  차수 p 의 B‑Spline/NURBS 곡선을 만들기 위한 clamped knot vector를 생성한다.
- 이 방식은 NURBS 교과서에서 **Averaging Method** 또는 **Curve Fitting Knot Vector** 라고 부르는 방식이다.

### 2. Knot Vector 구조
- 차수 p, 데이터 포인트 개수 k+1 일 때:
- 제어점 개수:
```math
n=k+2
```
- Knot vector 길이:
```math
m=n+p+1
```
```math
\mathrm{knots.len()}=m+1=k+p+4
```
- Rust 코드에서 need_len = k + p + 4 로 체크하는 이유가 이것이다.

### 3. Knot Vector 구성 방식
- ✔ 1) 양 끝 클램프 (End Clamping)
```math
U_0=U_1=\cdots =U_p=u_0
```
```math
U_{m-p}=\cdots =U_m=u_k
```
- 코드:
```rust
for i in 0..=p {
    knots[i] = u[0];
    knots[n + i + 1] = u[k];
}
```
- ✔ 2) 내부 Knot 계산 (Averaging Method)
- 내부 knot는 다음 수식으로 계산된다:
```math
U_{i+p+1}=\frac{1}{p}\sum _{j=i}^{i+p-1}u_j
```
```math
i=0,1,\dots ,k-p+1
```
- 즉, 길이 p 의 슬라이딩 윈도우 평균이다.
- 코드:
```rust
for i in 0..=k-p+1 {
    let mut sum = 0.0;
    for j in i..=(i + p - 1) {
        sum += u[j];
    }
    knots[i + p + 1] = sum / (p as Real);
}
```

### 4. 전체 Knot Vector 형태
- 최종 knot vector는 다음과 같은 구조를 가진다:


### 5. 입력/출력 정의
- 입력 설명

| 이름           | 타입           | 의미                                      |
|----------------|----------------|-------------------------------------------|
| `u`            | `&[Real]`      | 파라미터 값 배열 (단조 증가해야 함)       |
| `k`            | `usize`        | 마지막 인덱스 (`u.len() == k + 1`)        |
| `p`            | `usize`        | 곡선 차수 (degree)                        |
| `knt`          | `&mut KnotVector` | 결과 knot vector 저장 공간               |

- 출력
  - 성공 시 Ok(())
  - 실패 시 Err(String) (입력 오류 메시지 포함)

### 6. 사용 목적 (When to use this)
- 이 함수는 다음 상황에서 사용된다:
- ✔ Curve Fitting (곡선 피팅)
- 주어진 데이터 포인트에 대해 least‑squares 방식으로 B‑Spline/NURBS 곡선을 만들 때  
  필수적으로 필요한 knot vector를 생성한다.
- ✔ Parameterization이 이미 주어진 경우
  - 예: chord-length, centripetal, uniform 등으로 u[]를 먼저 만든 뒤  
    그 u[]를 기반으로 knot vector를 만들 때 사용.
- ✔ CAD/CAM/CAE에서 표준 방식
  - Rhino, Siemens NX, CATIA, OpenNURBS 등에서  
    curve fitting 시 내부적으로 사용하는 방식과 동일한 구조.

### 7. 장점
- 매우 안정적
- 단조 증가 파라미터만 있으면 항상 유효한 knot 생성
- clamped B‑Spline의 표준 형태
- least‑squares fitting과 완벽히 호환
- 구현이 간단하고 빠름

### 8. 주의사항
- ⚠ u[]는 반드시 단조 증가해야 한다
  - 비단조 증가일 경우 내부 knot가 뒤집힐 수 있다.
- ⚠ knt.knots 길이는 반드시 k+p+4 이어야 한다
  - 그렇지 않으면 오류 반환.
- ⚠ p ≥ 1
  - p=0은 B‑Spline이 아님.
- ⚠ k+1 ≥ p
  - 데이터 포인트가 차수보다 적으면 fitting 불가능.

### 9. 예제
```rust
let u = vec![0.0, 0.2, 0.5, 0.8, 1.0];
let k = u.len() - 1;
let p = 3;

let mut knt = KnotVector { knots: vec![0.0; k + p + 4] };

on_crv_fitting_knots(&u, k, p, &mut knt).unwrap();

println!("Knot Vector = {:?}", knt.knots);
```

### 10. 이 함수가 전체 피팅 파이프라인에서 차지하는 위치
```
Data Points → Parameterization(u[]) → on_crv_fitting_knots → Knot Vector
```
- 그 다음 단계는:
  - Basis matrix 구성
  - Least‑squares로 control points 계산
  - 최종 B‑Spline/NURBS 곡선 생성
- 즉, 곡선 피팅의 핵심 첫 단계를 담당하는 함수다.

```rust
pub fn on_crv_fitting_knots(u: &[Real], k: usize, p: usize, knt: &mut KnotVector)
  -> Result<(), String> {
    if u.is_empty() {
        return Err("INP_ERR: u empty".into());
    }
    if u.len() != k + 1 {
        return Err(format!(
            "INP_ERR: u.len mismatch: u.len={} but k+1={}",
            u.len(),
            k + 1
        ));
    }
    if p == 0 {
        return Err("INP_ERR: degree p must be >= 1".into());
    }
    // C에서 내부 loop: i=0..=k-p+1 이므로 k >= p-1 필요
    if k + 1 < p {
        return Err(format!(
            "INP_ERR: not enough params for degree: k+1={} < p={}",
            k + 1,
            p
        ));
    }

    // C: n = k+2
    let n = k + 2;

    // -> highest index m = n+p+1 이 되도록 정의
    // -> knots length = (n+p+1)+1 = n+p+2
    let need_len = n + p + 2; // == k + p + 4
    if knt.knots.len() != need_len {
        return Err(format!(
            "INP_ERR: knt size wrong. need {} (k+p+4) but got {}",
            need_len,
            knt.knots.len()
        ));
    }

    let knots = &mut knt.knots;

    // --- end clamping: for i=0..=p ---
    // U[i]     = u[0]
    // U[n+i+1] = u[k]
    for i in 0..=p {
        knots[i] = u[0];
        knots[n + i + 1] = u[k];
    }

    // --- internal knots ---
    // for i=0..=k-p+1:
    //   sum = Σ_{j=i}^{i+p-1} u[j]
    //   U[i+p+1] = sum/p
    let last_i = k - p + 1;
    for i in 0..=last_i {
        let mut sum = 0.0;
        for j in i..=(i + p - 1) {
            sum += u[j];
        }
        knots[i + p + 1] = sum / (p as Real);
    }

    Ok(())
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests_fitting_knots {
    use nurbslib::core::types::Real;
    use nurbslib::core::knot::KnotVector;
    use nurbslib::core::math_extensions::on_crv_fitting_knots;

    fn is_nondecreasing(a: &[Real]) -> bool {
        a.windows(2).all(|w| w[0] <= w[1])
    }

    fn is_clamped(knots: &[Real], deg: usize) -> bool {
        if knots.is_empty() { return false; }
        if knots.len() < 2 * (deg + 1) { return false; }
        let us = knots[0];
        let ue = *knots.last().unwrap();
        (0..=deg).all(|i| knots[i] == us && knots[knots.len() - 1 - i] == ue)
    }

    fn make_knt_for_fitting_knots(k: usize, p: usize) -> KnotVector {
        // need len = k+p+4
        KnotVector { knots: vec![0.0; k + p + 4] }
    }
```
```rust
    #[test]
    fn fitting_knots_basic_known_values() {
        // u: 0..5 (k=5), p=3
        // n=k+2=7, need_len=k+p+4=12, m=n+p+1=11 => len 12
        let u: Vec<Real> = (0..=5).map(|x| x as Real).collect();
        let k = 5usize;
        let p = 3usize;

        let mut knt = make_knt_for_fitting_knots(k, p);
        on_crv_fitting_knots(&u, k, p, &mut knt).unwrap();

        // ends clamped
        assert!(is_clamped(&knt.knots, p));

        // nondecreasing
        assert!(is_nondecreasing(&knt.knots));

        // internal knots:
        // i=0..=k-p+1=3
        // U[i+p+1] = avg(u[i..i+p-1])
        // U[4]=avg(u0..u2)=1
        // U[5]=avg(u1..u3)=2
        // U[6]=avg(u2..u4)=3
        // U[7]=avg(u3..u5)=4
        assert_eq!(knt.knots[4], 1.0);
        assert_eq!(knt.knots[5], 2.0);
        assert_eq!(knt.knots[6], 3.0);
        assert_eq!(knt.knots[7], 4.0);

        // and the clamped blocks:
        // first p+1 = 4 entries are u[0]=0
        assert_eq!(&knt.knots[0..4], &[0.0, 0.0, 0.0, 0.0]);
        // last p+1 = 4 entries are u[k]=5
        assert_eq!(&knt.knots[8..12], &[5.0, 5.0, 5.0, 5.0]);
    }
```
```rust
    #[test]
    fn fitting_knots_nondecreasing_for_irregular_params() {
        // irregular but nondecreasing params
        let u: Vec<Real> = vec![0.0, 0.1, 0.4, 0.41, 0.9, 1.0, 1.0, 1.2];
        let k = u.len() - 1;
        let p = 3usize;

        let mut knt = make_knt_for_fitting_knots(k, p);
        on_crv_fitting_knots(&u, k, p, &mut knt).unwrap();

        assert!(is_clamped(&knt.knots, p));
        assert!(is_nondecreasing(&knt.knots));

        // internal knots must lie within [u0, uk]
        let us = u[0];
        let ue = u[k];
        for &x in &knt.knots {
            assert!(x >= us - 1e-12 && x <= ue + 1e-12);
        }
    }
```
```rust
    #[test]
    fn fitting_knots_length_must_match_k_p() {
        let u: Vec<Real> = vec![0.0, 0.2, 0.7, 1.0];
        let k = 3usize;
        let p = 2usize;

        // wrong length on purpose
        let mut knt = KnotVector { knots: vec![0.0; 999] };
        let err = on_crv_fitting_knots(&u, k, p, &mut knt).unwrap_err();
        assert!(err.contains("knt size wrong"));
    }
```
```rust
    #[test]
    fn fitting_knots_rejects_p_zero() {
        let u: Vec<Real> = vec![0.0, 0.5, 1.0];
        let k = 2usize;
        let p = 0usize;

        let mut knt = KnotVector { knots: vec![0.0; k + p + 4] };
        let err = on_crv_fitting_knots(&u, k, p, &mut knt).unwrap_err();
        assert!(err.contains("degree p must be"));
    }
```
```rust
    #[test]
    fn fitting_knots_rejects_not_enough_params_for_degree() {
        // k+1 < p
        let u: Vec<Real> = vec![0.0, 1.0, 2.0]; // k=2 => k+1=3
        let k = 2usize;
        let p = 4usize;

        let mut knt = KnotVector { knots: vec![0.0; k + p + 4] };
        let err = on_crv_fitting_knots(&u, k, p, &mut knt).unwrap_err();
        assert!(err.contains("not enough params"));
    }
}
```
---

## on_curve_fitting_knot_with_boundary 
- EndWhere(Start/End) 기반 Knot Vector
- ✔ 이 함수는 특정 끝에서 derivative 조건을 만족시키기 위한 knot vector를 만든다.
- on_curve_fitting_knot_with_boundary()는  
  곡선의 시작 또는 끝에서 1차/2차 미분값을 지정하는 fitting을 위해 존재한다.
- 그래서 내부 knot 계산 방식이 달라진다.

### 1) 두 함수의 내부 knot 계산 방식 비교
- 🔵 on_crv_fitting_knots (표준 averaging)
```math
U_{i+p+1}=\frac{1}{p}\sum _{j=i}^{i+p-1}u_j
```
- i = 0..k-p+1
- 완전한 sliding window
- symmetric
- 양쪽 끝 동일한 방식

- 🔴 on_curve_fitting_knot_with_boundary (derivative boundary 조건용)
- EndWhere::Start
```math
U[p+1]=\frac{1}{p}\sum _{j=0}^{p-1}u_j
```
```math
U[i+p+1]=\frac{1}{p}\sum _{j=i}^{i+p-1}u_j,\quad i=1..k-p
```
- 즉, 첫 번째 내부 knot만 특별 취급한다.
- EndWhere::End
```math
U[n]=\frac{1}{p}\sum _{j=k-p+1}^ku_j
```
```math
U[i+p]=\frac{1}{p}\sum _{j=i}^{i+p-1}u_j,\quad i=1..k-p
```
- 즉, 마지막 내부 knot만 특별 취급한다.

### 2) 왜 이런 차이가 생기나?
- ✔ on_curve_fitting_knot_with_boundary는 **한쪽 끝에서 derivative 조건을 만족시키기 위해**  
  내부 knot의 첫 번째 또는 마지막 값을 특별하게 조정해야 한다.
- 예를 들어:
  - 시작점에서 tangent(1차 미분)를 고정하고 싶다 → Start 모드
  - 끝점에서 curvature(2차 미분)를 고정하고 싶다 → End 모드
- 이런 경우,
  - 첫 번째 또는 마지막 내부 knot의 위치가 곡선의 shape에 직접적인 영향을 준다.
  - 그래서 on_curve_fitting_knot_with_boundary는:
    - 첫 내부 knot 또는 마지막 내부 knot을
    - 특별한 averaging 방식으로 계산한다.

### 3) 용도 차이 요약
- 🧮 두 함수의 용도 차이 요약


| 함수 이름     | 내부 knot 계산 방식       | 목적                  | 특징                 | 사용 상황          |
|--------------|-------------------------|----------------------|----------------------------|---------------|
|`on_crv_fitting_knots`|완전한 슬라이딩 평균 (`i=0..k-p+1`)|일반적인 곡선 피팅|양쪽 끝 대칭, 모든 내부 knot 동일 방식|대부분의 least-squares fitting|
|`on_curve_fitting_knot_with_boundary`| 한쪽 끝 knot 특별 처리 (`Start/End`)|미분값(derivative) 조건을 만족시키는 피팅|첫 또는 마지막 내부 knot만 특별 계산|tangent/curvature 조건 있는 경우|

### 4) 실제 예시로 보면 더 명확해짐
- 예: p=3, u=[0,1,2,3,4,5]
- on_crv_fitting_knots
- 내부 knot = [1,2,3,4]  
    (완전한 sliding average)
- on_curve_fitting_knot_with_boundary(Start)
  - 내부 knot = [avg(0,1,2), avg(1,2,3), avg(2,3,4)]
  - 즉, 첫 knot가 특별 처리됨
- on_curve_fitting_knot_with_boundary(End)
  - 내부 knot = [avg(1,2,3), avg(2,3,4), avg(3,4,5)]
  - 즉, 마지막 knot가 특별 처리됨

### 🎯 최종 정리
- ✔ on_crv_fitting_knots
  - 표준 curve fitting knot vector
  - symmetric averaging
  - derivative 조건 없음
  - 가장 일반적인 방식
- ✔ on_curve_fitting_knot_with_boundary
  - derivative boundary 조건을 만족시키기 위한 특수 knot vector
  - 첫 내부 knot 또는 마지막 내부 knot을 특별 처리
  - tangent/curvature fitting에 사용

```rust

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndWhere {
    Start,
    End,
}

/// - u: parameters, len = k+1
/// - k: highest index in u
/// - p: degree
/// - whr: Start or End (which end derivative is specified)
/// - knt: output knot vector (must be allocated by caller)
pub fn on_curve_fitting_knot_with_boundary(u: &[Real], k: usize, p: usize,
  whr: EndWhere, knt: &mut KnotVector) -> Result<(), String> {
    // ---- error check (C: p > k+1 OR whr invalid) ----
    if p > k + 1 {
        return Err("INP_ERR: p > k+1".into());
    }

    if u.is_empty() || u.len() != k + 1 {
        return Err(format!("INP_ERR: u.len mismatch (u.len={} k+1={})", u.len(), k + 1));
    }

    // ---- n = k+1 ----
    let n = k + 1;
    let need_len = n + p + 2; // = k+p+3
    if knt.knots.len() != need_len {
        return Err(format!(
            "INP_ERR: knt size wrong. need {} (k+p+3) got {}",
            need_len,
            knt.knots.len()
        ));
    }

    let U = &mut knt.knots;

    // ---- clamp ends: U[0..p]=u0, U[n+1..n+p+1]=uk ----
    for i in 0..=p {
        U[i] = u[0];
        U[n + i + 1] = u[k];
    }

    // ---- if p < n then compute internal knots ----
    // (C: if (p LT n))
    if p < n {
        match whr {
            EndWhere::Start => {
                // for i=1..=k-p:
                //   U[i+p+1] = avg(u[i..i+p-1])
                // then special: U[p+1] = avg(u[0..p-1])
                if k >= p {
                    for i in 1..=(k - p) {
                        let mut sum = 0.0;
                        for j in i..=(i + p - 1) {
                            sum += u[j];
                        }
                        U[i + p + 1] = sum / (p as Real);
                    }
                }

                // U[p+1]
                let mut sum = 0.0;
                for j in 0..=(p - 1) {
                    sum += u[j];
                }
                U[p + 1] = sum / (p as Real);
            }

            EndWhere::End => {
                // for i=1..=k-p:
                //   U[i+p] = avg(u[i..i+p-1])
                // then special: U[n] = avg(u[k-p+1..k])
                if k >= p {
                    for i in 1..=(k - p) {
                        let mut sum = 0.0;
                        for j in i..=(i + p - 1) {
                            sum += u[j];
                        }
                        U[i + p] = sum / (p as Real);
                    }
                }
                // U[n]
                let mut sum = 0.0;
                for j in (k - p + 1)..=k {
                    sum += u[j];
                }
                U[n] = sum / (p as Real);
            }
        }
    }
    Ok(())
}
```

## 📘 on_curve_approximation_knots 
- Curve Approximation Knot Vector (Clustering + Averaging Method)
### 1. 이 함수가 하는 일 (핵심 요약)
- 이 함수는 다음 상황에서 사용된다:
- 데이터 포인트 개수 (m+1) > 제어점 개수 (n+1)
- 즉, **근사(approximation)** 를 해야 하는 경우.

- 그래서:
  - u[] 파라미터를 n+1개의 그룹으로 클러스터링하고
  - 각 그룹의 대표값 uk[i] 를 만든 뒤
  - uk[] 를 기반으로 averaging knot vector를 만든다.
- 이 방식은 곡선 근사(approximation) 에서 매우 널리 쓰이는 방식이며,  
  곡선 피팅(fitting)과는 목적이 다르다.

### 2. 입력 파라미터

| 이름   | 타입            | 의미 |
|--------|-----------------|-----------------------------------------------|
| `u`  |`&[Real]`       |파라미터 배열 (단조 증가, 길이 = m+1)  |
| `m`  |`usize`         |u[]의 마지막 인덱스       |
| `n`  |`usize`         |제어점 마지막 인덱스 (n+1개의 제어점, n ≤ m)|
| `p`  |`usize`         |곡선 차수 (p < n 이어야 함) |
| `knt`|`&mut KnotVector`|결과 knot vector 저장 공간 (길이 = n+p+2 이어야 함)|


### 3. 알고리즘 단계별 설명
- ✔ 1) End Clamp
- 양 끝 knot는 다음과 같이 p+1번 반복된다.
```math
U_0=U_1=\cdots =U_p=u_0
```
```math
U_{n+1}=\cdots =U_{n+p+1}=u_m
```

- ✔ 2) 파라미터 u[] 를 (n+1)개의 그룹으로 클러스터링
- 코드의 핵심:
```math
d=\frac{m+1}{n+1}
```
```math
l=-1
```
```math
l=l+d
```
```math
ih=\mathrm{round}(l)
```
각 i에 대해:
  - 그룹 시작 = il
  - 그룹 끝 = ih
  - 그룹 대표값 uk[i] = 평균(u[il..ih])
- 즉,
```math
uk[i]=\frac{1}{ih-il+1}\sum _{j=il}^{ih}u_j
```
- 이 uk[] 배열이 제어점 파라미터의 대표값이 된다.

- ✔ 3) 내부 Knot 계산 (Averaging of uk[])
```math
U_{i+p}=\frac{1}{p}\sum _{j=i}^{i+p-1}uk[j]
```
```math
i=1..n-p
```
- 즉, uk[] 를 기반으로 한 슬라이딩 평균이다.

### 4. 이 함수의 용도 (왜 필요한가)
- 이 함수는 곡선 근사(approximation) 에서 사용된다.
  - ✔ 데이터 포인트가 많고
  - ✔ 제어점은 적게 쓰고 싶을 때
  - ✔ least-squares approximation을 하기 위한 knot vector 생성기
- 즉, 다음과 같은 상황:
  - 스캔 데이터가 10,000개
  - 제어점은 50개만 쓰고 싶다
  - 곡선을 “근사”해야 한다
  - 그때 필요한 knot vector가 바로 이 함수가 만드는 knot vector


### 5. 다른 함수들과의 차이

| 함수 이름                    | 목적             | 내부 knot 계산 방식               | 특징                         | 사용 상황   |
|----------------------------|--------------------------|----------------------------------|----------------------------|----------------------------|
|`on_crv_fitting_knots`           |곡선 피팅 (fitting)  |u[] 기반 완전한 슬라이딩 평균| 대칭적, 가장 일반적인 fitting 방식|데이터 포인트 수 = 제어점 수일 때|
|`on_curve_fitting_knots_boundary`|경계 미분 조건 있는 피팅|첫/마지막 내부 knot 특별 처리|tangent/curvature 조건 반영 가능|시작/끝에서 미분값을 지정해야 할 때|
|`on_curve_approximation_knots`   |곡선 근사 (approximation)|u[] 클러스터링 후 uk[] 기반 평균|데이터 >> 제어점일 때 안정적 근사|스캔 데이터 등 많은 점을 적은 CP로 근사 |

- 즉, on_curve_approximation_knots는 approximation 전용 알고리즘이다.


```rust
#[inline]
pub fn on_round_index(x: Real) -> usize {
    (x + 0.5) as usize
}
/// - u: parameter array, len = m+1 (nondecreasing 가정)
/// - m: highest index in u
/// - n: highest index of control point array of approximating curve (n <= m)
/// - p: degree (원문: p < n)
/// - knt: output knot vector (caller alloc)
/// - end clamp: U[0..=p]=u0, U[n+1..=n+p+1]=u[m]
/// - compute representatives uk[0..=n] by clustering u into (n+1) groups
/// - internal knots: for i=1..=n-p: U[i+p] = avg(uk[i..i+p-1])
pub fn on_curve_approximation_knots(u: &[Real], m: usize, n: usize, p: usize, knt: &mut KnotVector)
  -> Result<(), String> {
    // ---- checks (C는 ERROR(INP_ERR) 후 error=1) ----
    if u.is_empty() {
        return Err("INP_ERR: u empty".into());
    }
    if u.len() != m + 1 {
        return Err(format!("INP_ERR: u.len mismatch (u.len={} m+1={})", u.len(), m + 1));
    }
    if n > m {
        return Err("INP_ERR: n > m".into());
    }
    if p >= n {
        return Err("INP_ERR: require p < n".into());
    }

    // knot length must be n+p+2
    let need_len = n + p + 2;
    if knt.knots.len() != need_len {
        return Err(format!(
            "INP_ERR: knt size wrong. need {} (n+p+2) got {}",
            need_len,
            knt.knots.len()
        ));
    }

    let U = &mut knt.knots;

    // ---- end clamp ----
    for i in 0..=p {
        U[i] = u[0];
        U[n + i + 1] = u[m];
    }

    // ---- Compute representatives uk[0..=n] of clusters ----
    // C:
    // d=(m+1.0)/(n+1.0); il=0; ih=il; l=-1;
    // for i=0..=n:
    //   l=l+d; ih=ROUND(l);
    //   if il==ih => uk[i]=u[il]
    //   else avg(u[il..ih])
    //   il=ih+1
    let mut uk = vec![0.0; n + 1];

    let d = (m as Real + 1.0) / (n as Real + 1.0);
    let mut il: usize = 0;
    let mut ih: usize = il;
    let mut l: Real = -1.0;

    for i in 0..=n {
        l += d;
        ih = on_round_index(l);

        // 안전 클램프 (C는 인덱스가 유효하다고 가정)
        if ih > m { ih = m; }
        if il > m { il = m; }

        if il == ih {
            uk[i] = u[il];
        } else {
            let mut sum = 0.0;
            for j in il..=ih {
                sum += u[j];
            }
            uk[i] = sum / ((ih - il + 1) as Real);
        }

        if ih + 1 > m {
            // 다음 il이 범위를 넘어가면 이후 반복은 사실상 끝까지 고정될 수 있음
            il = m;
        } else {
            il = ih + 1;
        }
    }

    // ---- Now compute the knot vector ----
    // C:
    // for i=1..=n-p:
    //   sum = Σ_{j=i}^{i+p-1} uk[j]
    //   U[i+p] = sum/p
    for i in 1..=(n - p) {
        let mut sum = 0.0;
        for j in i..=(i + p - 1) {
            sum += uk[j];
        }
        U[i + p] = sum / (p as Real);
    }

    Ok(())
}
```
### 테스트 코드
```rust
#[cfg(test)]
mod tests_fit_knot_bound_approx {
    use nurbslib::core::types::Real;
    use nurbslib::core::knot::KnotVector;
    use nurbslib::core::math_extensions::{on_curve_approximation_knots,
        on_curve_fitting_knot_with_boundary, EndWhere};

    fn is_nondecreasing(a: &[Real]) -> bool {
        a.windows(2).all(|w| w[0] <= w[1])
    }

    fn is_clamped(knots: &[Real], deg: usize) -> bool {
        if knots.is_empty() { return false; }
        if knots.len() < 2 * (deg + 1) { return false; }
        let us = knots[0];
        let ue = *knots.last().unwrap();
        (0..=deg).all(|i| knots[i] == us && knots[knots.len() - 1 - i] == ue)
    }

    fn in_range(knots: &[Real], lo: Real, hi: Real) -> bool {
        knots.iter().all(|&x| x >= lo - 1e-12 && x <= hi + 1e-12)
    }

    fn make_knt_fit_knot_bound(n: usize, p: usize) -> KnotVector {
        // need len = n+p+2
        KnotVector { knots: vec![0.0; n + p + 2] }
    }

    fn make_knt_fit_approx_knt(k: usize, p: usize) -> KnotVector {
        // need len = (k+1)+p+2 = k+p+3
        KnotVector { knots: vec![0.0; k + p + 3] }
    }

    // ---------------------------
    // N_fitkna tests
    // ---------------------------
```
```rust
    #[test]
    fn fit_knot_bound_basic_monotone_and_clamped() {
        // u: 0..=9 (m=9)
        let u: Vec<Real> = (0..=9).map(|x| x as Real).collect();
        let m = 9usize;
        let n = 4usize; // control points highest index (=> n+1=5 reps)
        let p = 2usize;

        let mut knt = make_knt_fit_knot_bound(n, p);
        on_curve_approximation_knots(&u, m, n, p, &mut knt).unwrap();

        assert!(is_clamped(&knt.knots, p));
        assert!(is_nondecreasing(&knt.knots));
        assert!(in_range(&knt.knots, u[0], u[m]));

        // internal knots count = n-p (because i=1..=n-p)
        // indices filled: U[i+p], i=1..=n-p => U[p+1 .. p+(n-p)] = U[p+1 .. n]
        // we can at least sanity-check those are within range and nondecreasing
        for idx in (p + 1)..=n {
            assert!(knt.knots[idx] >= u[0] - 1e-12 && knt.knots[idx] <= u[m] + 1e-12);
        }
    }
```
```rust
    #[test]
    fn fit_knot_bound_rejects_bad_sizes() {
        let u: Vec<Real> = vec![0.0, 0.5, 1.0];
        let m = 2usize;
        let n = 3usize; // n > m => error
        let p = 1usize;

        let mut knt = make_knt_fit_knot_bound(n, p);
        let err = on_curve_approximation_knots(&u, m, n, p, &mut knt).unwrap_err();
        assert!(err.contains("n > m"));
    }
```
```rust
    #[test]
    fn fit_knot_bound_rejects_p_ge_n() {
        let u: Vec<Real> = (0..=10).map(|x| x as Real).collect();
        let m = 10usize;
        let n = 3usize;
        let p = 3usize; // p >= n => error

        let mut knt = make_knt_fit_knot_bound(n, p);
        let err = on_curve_approximation_knots(&u, m, n, p, &mut knt).unwrap_err();
        assert!(err.contains("p < n"));
    }
```
```rust
    #[test]
    fn fit_approx_knt_start_basic_known_values() {
        // u = 0..=6, k=6, p=3, n=k+1=7, len = k+p+3 = 12
        let u: Vec<Real> = (0..=6).map(|x| x as Real).collect();
        let k = 6usize;
        let p = 3usize;

        let mut knt = make_knt_fit_approx_knt(k, p);
        on_curve_fitting_knot_with_boundary(&u, k, p, EndWhere::Start, &mut knt).unwrap();

        assert!(is_clamped(&knt.knots, p));
        assert!(is_nondecreasing(&knt.knots));
        assert!(in_range(&knt.knots, u[0], u[k]));

        // START branch:
        // - U[p+1] = avg(u[0..p-1]) = avg(0,1,2)=1
        // - for i=1..=k-p (=3):
        //   U[i+p+1] = avg(u[i..i+p-1])
        //   i=1 -> U[5]=avg(1,2,3)=2
        //   i=2 -> U[6]=avg(2,3,4)=3
        //   i=3 -> U[7]=avg(3,4,5)=4
        assert_eq!(knt.knots[p + 1], 1.0);
        assert_eq!(knt.knots[5], 2.0);
        assert_eq!(knt.knots[6], 3.0);
        assert_eq!(knt.knots[7], 4.0);

        // ends: first p+1 are u0=0, last p+1 are uk=6
        assert_eq!(&knt.knots[0..4], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&knt.knots[8..12], &[6.0, 6.0, 6.0, 6.0]);
    }
```
```rust
    #[test]
    fn fit_approx_knt_end_basic_known_values() {
        // u = 0..=6, k=6, p=3
        let u: Vec<Real> = (0..=6).map(|x| x as Real).collect();
        let k = 6usize;
        let p = 3usize;

        let mut knt = make_knt_fit_approx_knt(k, p);
        on_curve_fitting_knot_with_boundary(&u, k, p, EndWhere::End, &mut knt).unwrap();

        assert!(is_clamped(&knt.knots, p));
        assert!(is_nondecreasing(&knt.knots));
        assert!(in_range(&knt.knots, u[0], u[k]));

        // END branch:
        // - for i=1..=k-p (=3):
        //   U[i+p] = avg(u[i..i+p-1])
        //   i=1 -> U[4]=avg(1,2,3)=2
        //   i=2 -> U[5]=avg(2,3,4)=3
        //   i=3 -> U[6]=avg(3,4,5)=4
        // - U[n] where n=k+1=7: avg(u[k-p+1..k]) = avg(u[4..6]) = avg(4,5,6)=5
        assert_eq!(knt.knots[4], 2.0);
        assert_eq!(knt.knots[5], 3.0);
        assert_eq!(knt.knots[6], 4.0);
        assert_eq!(knt.knots[k + 1], 5.0); // U[n]

        // ends: first p+1 are u0=0, last p+1 are uk=6
        assert_eq!(&knt.knots[0..4], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&knt.knots[8..12], &[6.0, 6.0, 6.0, 6.0]);
    }
```
```rust
    #[test]
    fn fit_approx_knt_rejects_bad_degree() {
        let u: Vec<Real> = vec![0.0, 0.5, 1.0];
        let k = 2usize;
        let p = 4usize; // p > k+1 => error

        let mut knt = make_knt_fit_approx_knt(k, p);
        let err = on_curve_fitting_knot_with_boundary(&u, k, p, EndWhere::Start, &mut knt).unwrap_err();
        assert!(err.contains("p > k+1"));
    }
```
```rust
    #[test]
    fn fit_approx_knt_length_must_match() {
        let u: Vec<Real> = vec![0.0, 0.2, 0.7, 1.0];
        let k = 3usize;
        let p = 2usize;

        let mut bad = KnotVector { knots: vec![0.0; 999] };
        let err = on_curve_fitting_knot_with_boundary(&u, k, p, EndWhere::End, &mut bad).unwrap_err();
        assert!(err.contains("knt size wrong"));
    }
}
```
---

