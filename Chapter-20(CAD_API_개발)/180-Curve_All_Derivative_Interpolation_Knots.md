# 📘 on_curve_all_derivative_interpolation_knots 
- All‑Derivative Interpolation Knot Vector

## 1. 목적 (What this function does)
- 이 함수는 모든 도함수(all derivatives) 보간을 수행할 때 필요한 특수 Knot Vector를 생성한다.
- 즉, 다음 조건을 만족하는 B‑Spline/NURBS 곡선을 만들기 위한 knot vector:
  - 위치(position)
  - 1차 도함수(tangent)
  - 2차 도함수(curvature)
  - … 모든 도함수
- 이 모두가 정확히 보간(interpolation) 되도록 설계된 knot 패턴이다.
- 이 방식은 일반적인 fitting/approximation과는 완전히 다르다.

## 2. 제약 조건
- p MUST BE 2 OR 3
- 즉, 2차 또는 3차 B‑Spline에서만 사용 가능하다.


### 3. on_curve_all_derivative_interpolation_knots 함수 입력

| 이름   | 타입            | 의미 |
|--------|-----------------|--------------------------------------------------------------|
| `u`    | `&[Real]`       | 파라미터 배열 (단조 증가, 길이 = k+1)                       |
| `k`    | `usize`         | u[]의 마지막 인덱스                                         |
| `p`    | `usize`         | 곡선 차수 (반드시 2 또는 3)                                 |
| `knt`  | `&mut KnotVector` | 결과 knot vector 저장 공간 (길이 = 2k+p+3 이어야 함)     |


## 4. Knot Vector 크기
- 코드 정의:
```math
n=2k+1
```
- knot vector 길이:
```math
\mathrm{len}=n+p+2=2k+p+3
```
- 코드에서 need_len = 2k + p + 3 체크하는 이유가 이것.

## 5. 알고리즘 단계별 설명

- ✔ 1) End Clamp
```math
U_0=U_1=\cdots =U_p=u_0
```
```math
U_{n+1}=\cdots =U_{n+p+1}=u_k
```
- 코드:
```rust
for i in 0..=p {
    U[i] = u[0];
    U[n + i + 1] = u[k];
}
```

- ✔ 2) 내부 Knot 패턴 생성 (핵심)
- 여기서부터가 on_curve_all_derivative_interpolation_knots 만의 특수 규칙이다.
- p = 2 (Quadratic)
  - 내부 knot 패턴:
    - 첫 내부 knot
    ```math
    U_{p+1}=\frac{u_0+u_1}{2}
    ```
    - 그 다음 반복 패턴
      - 각 i = 1..k-1 에 대해:
    ```math
    U_{j+1}=u_i
    ```
    ```math
    U_{j+2}=\frac{u_i+u_{i+1}}{2}
    ```
    - 즉, u[i] 와 (u[i]+u[i+1])/2 를 번갈아 배치한다.

- p = 3 (Cubic)
    - 내부 knot 패턴:
      - 첫 내부 knot
      ```math
      U_{p+1}=\frac{u_0+u_1}{2}
      ```
      - 마지막 내부 knot
      ```math
      U_n=\frac{u_{k-1}+u_k}{2}
      ```
      - 중간 패턴 (i = 1..k-2)
      ```math
      U_{j+1}=\frac{2}{3}u_i+\frac{1}{3}u_{i+1}
      ```
      ```math
      U_{j+2}=\frac{1}{3}u_i+\frac{2}{3}u_{i+1}
      ```

      - 즉, 1/3–2/3 가중 평균 두 개를 번갈아 배치한다.

## 6. 왜 이런 특수 패턴을 쓰는가?
0 이 knot 패턴은 Hermite‑style interpolation을 B‑Spline으로 구현하기 위한 것이다.
- 즉:
  - 위치
  - 1차 도함수
  - 2차 도함수
  - … 모든 도함수
- 이 정확히 보간되도록 knot vector를 구성해야 한다.
- 이를 위해:
  - 각 데이터 포인트 u[i] 주변에
  - 여러 개의 knot를 배치하여
  - 도함수 조건을 만족시키는 basis function을 확보한다.
- 그래서:
  - p=2 → 각 구간에 2개의 내부 knot
  - p=3 → 각 구간에 2개의 내부 knot (1/3, 2/3 분할)
- 이런 특수한 패턴이 필요하다.

## 7. 다른 knot 생성 함수들과의 차이
### 🧮 함수 비교

| 함수 이름                         | 목적                         | 내부 knot 방식                         | 특징 |
|-----------------------------------|------------------------------|------------------------------------------|-------|
| on_crv_fitting_knots              | 일반 곡선 피팅               | sliding average                          | 가장 일반적인 fitting |
| on_curve_fitting_knots_boundary   | 경계 미분 조건 있는 피팅     | 첫/마지막 knot 특별 처리                 | tangent/curvature 조건 |
| on_curve_approximation_knots      | 곡선 근사 (approximation)    | u[] 클러스터링 후 uk[] 평균              | 데이터 >> 제어점 |
| **on_fitkad**                     | **모든 도함수 보간**         | **특수 패턴 (1/2, 1/3–2/3)**             | Hermite‑style interpolation |


---

## 소스 코드
```rust
/// 목적: "모든 도함수(all derivatives) 지정" 보간용 knot vector 생성.
/// 제약: p MUST BE 2 OR 3

/// 입력:
/// - u: parameters, len = k+1 (nondecreasing 가정)
/// - k: highest index in u
/// - p: degree (2 or 3 only)
/// - knt: output knot vector (caller alloc)

/// - n = 2*k + 1
/// - knots length must be (n+p+2) = (2k+1)+p+2 = 2k+p+3
/// - clamp: U[0..=p]=u0, U[n+1..=n+p+1]=uk
/// - then fill special interior pattern depending on p

pub fn on_curve_all_derivative_interpolation_knots(u: &[Real], k: usize, p: usize,
    knt: &mut KnotVector) -> Result<(), String> {
    if u.is_empty() || u.len() != k + 1 {
        return Err(format!("INP_ERR: u.len mismatch (u.len={} k+1={})", u.len(), k + 1));
    }
    if p != 2 && p != 3 {
        return Err("INP_ERR: on_curve_all_derivative_interpolation_knots requires p=2 or p=3".into());
    }

    let n = 2 * k + 1;

    // highest index = n+p+1 => len = n+p+2
    let need_len = n + p + 2; // = 2k+p+3
    if knt.knots.len() != need_len {
        return Err(format!(
            "INP_ERR: knt size wrong. need {} (2k+p+3) got {}",
            need_len,
            knt.knots.len()
        ));
    }

    let knots = &mut knt.knots;

    // clamp ends
    for i in 0..=p {
        knots[i] = u[0];
        knots[n + i + 1] = u[k];
    }

    // j = p+1
    let mut j = p + 1;

    match p {
        2 => {
            if k >= 1 {
                knots[p + 1] = 0.5 * (u[1] + u[0]);
            } else {
                knots[p + 1] = u[0];
            }

            for i in 1..k {
                j += 1;
                knots[j] = u[i];

                j += 1;
                knots[j] = 0.5 * (u[i] + u[i + 1]);
            }
        }

        3 => {
            if k > 1 {
                knots[p + 1] = 0.5 * (u[1] + u[0]);
                knots[n] = 0.5 * (u[k] + u[k - 1]);
            }
            let alf = 1.0 / 3.0;
            let bet = 2.0 / 3.0;

            if k >= 2 {
                for i in 1..(k - 1) {
                    j += 1;
                    knots[j] = bet * u[i] + alf * u[i + 1];

                    j += 1;
                    knots[j] = alf * u[i] + bet * u[i + 1];
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
```
### 테스트 코드
```rust
#[cfg(test)]
mod tests_fit_crv_all_ders_int_knots {
    use nurbslib::core::types::Real;
    use nurbslib::core::knot::KnotVector;
    use nurbslib::core::math_extensions::on_curve_all_derivative_interpolation_knots;

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

    fn make_knt_fit_crv_all_ders_int_knots(k: usize, p: usize) -> KnotVector {
        // need len = (2k+1) + p + 2 = 2k + p + 3
        KnotVector { knots: vec![0.0; 2 * k + p + 3] }
    }
```
```rust
    #[test]
    fn fit_crv_all_ders_int_knots_p2_known_pattern_small_k() {
        // k=3 => u has 4 entries
        // p=2 => n=2k+1=7 => len=2k+p+3=11
        let u: Vec<Real> = vec![0.0, 1.0, 2.0, 3.0];
        let k = 3usize;
        let p = 2usize;

        let mut knt = make_knt_fit_crv_all_ders_int_knots(k, p);
        on_curve_all_derivative_interpolation_knots(&u, k, p, &mut knt).unwrap();

        // clamped ends + monotone
        assert!(is_clamped(&knt.knots, p));
        assert!(is_nondecreasing(&knt.knots));
        assert!(in_range(&knt.knots, u[0], u[k]));


        assert_eq!(knt.knots[3], 0.5);
        assert_eq!(knt.knots[4], 1.0);
        assert_eq!(knt.knots[5], 1.5);
        assert_eq!(knt.knots[6], 2.0);
        assert_eq!(knt.knots[7], 2.5);
        assert_eq!(&knt.knots[8..11], &[3.0, 3.0, 3.0]);
    }
```
```rust
    #[test]
    fn fit_crv_all_ders_int_knots_p3_known_pattern_k4() {
        // k=4 => u has 5 entries: 0,1,2,3,4
        // p=3 => n=2k+1=9 => len=2k+p+3=14
        let u: Vec<Real> = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let k = 4usize;
        let p = 3usize;

        let mut knt = make_knt_fit_crv_all_ders_int_knots(k, p);
        on_curve_all_derivative_interpolation_knots(&u, k, p, &mut knt).unwrap();

        assert!(is_clamped(&knt.knots, p));
        assert!(is_nondecreasing(&knt.knots));
        assert!(in_range(&knt.knots, u[0], u[k]));


        let eps = 1e-12;
        assert!((knt.knots[4] - 0.5).abs() < eps);
        assert!((knt.knots[5] - (4.0 / 3.0)).abs() < eps);
        assert!((knt.knots[6] - (5.0 / 3.0)).abs() < eps);
        assert!((knt.knots[7] - (7.0 / 3.0)).abs() < eps);
        assert!((knt.knots[8] - (8.0 / 3.0)).abs() < eps);
        assert!((knt.knots[9] - 3.5).abs() < eps);

        // tail clamp: n+1=10..=13 are uk=4
        assert_eq!(&knt.knots[10..14], &[4.0, 4.0, 4.0, 4.0]);
        // head clamp: first 4 are u0=0
        assert_eq!(&knt.knots[0..4], &[0.0, 0.0, 0.0, 0.0]);
    }
```
```rust
    #[test]
    fn fit_crv_all_ders_int_knots_rejects_invalid_degree() {
        let u: Vec<Real> = vec![0.0, 1.0, 2.0, 3.0];
        let k = 3usize;
        let p = 4usize;

        let mut knt = make_knt_fit_crv_all_ders_int_knots(k, 3);
        let err = on_curve_all_derivative_interpolation_knots(&u, k, p, &mut knt).unwrap_err();
        assert!(err.contains("p=2 or p=3"));
    }
```
```rust
    #[test]
    fn fit_crv_all_ders_int_knots_knot_length_must_match() {
        let u: Vec<Real> = vec![0.0, 1.0, 2.0, 3.0];
        let k = 3usize;
        let p = 2usize;

        let mut bad = KnotVector { knots: vec![0.0; 999] };
        let err = on_curve_all_derivative_interpolation_knots(&u, k, p, &mut bad).unwrap_err();
        assert!(err.contains("knt size wrong"));
    }
```
```rust
    #[test]
    fn fit_crv_all_ders_int_knots_handles_k1_p2_safely() {
        // k=1 (u has 2 points) is borderline but should not panic in Rust.
        // C assumes meaningful input; we just ensure safety + monotone.
        let u: Vec<Real> = vec![0.0, 1.0];
        let k = 1usize;
        let p = 2usize;

        let mut knt = make_knt_fit_crv_all_ders_int_knots(k, p);
        on_curve_all_derivative_interpolation_knots(&u, k, p, &mut knt).unwrap();

        assert!(is_clamped(&knt.knots, p));
        assert!(is_nondecreasing(&knt.knots));
        assert!(in_range(&knt.knots, u[0], u[k]));
    }
}
```
---
