# Input KnotVector(=U_new) 개념 정리: 
- `on_unclamp_with_knot_vector`

## 0. 한 문장 요약
- `on_unclamp_with_knot_vector`의 **input knot vector(U_new)** 는  
  기존 clamped knot vector(U_old)와 **내부 구간(U[p..n+1])이 완전히 동일** 하고,  
  끝 부분(좌측 p개, 우측 p개)만 바깥쪽으로 펼쳐서(end multiplicity를 낮추기 위해)  
  값이 달라진 knot vector 를 의미합니다.

- 즉, **multiplicity만 조정** 하는 용도이며,   
  **내부 knot를 새로 만들어 재분배/삽입하는 함수가 아닙니다.**

---

## 1. 기초 표기
- 차수: `p`
- 컨트롤 포인트 개수: `n+1` (최고 인덱스 `n`)
- knot vector 길이: `m+1`, `m = n + p + 1`
- knot 배열: `U[0..m]`
- “곡선의 기본 평가 도메인”: `[U[p], U[n+1]]`

- B-spline(또는 homogeneous 누적 포함) 곡선은

```math
C(u) = \sum_{i=0}^{n} N_{i,p}(u;U)\,P_i
```

- 으로 정의되고, **basis 함수 $N_{i,p}$ 는 knot vector U가 바뀌면 완전히 바뀝니다.**

---

## 2. clamped(open clamped)와 unclamped의 차이

### 2.1 clamped(open clamped)
- 좌측 끝값이 `p+1`번 반복: `U[0]=...=U[p]`
- 우측 끝값이 `p+1`번 반복: `U[n+1]=...=U[n+p+1]`
- 이때는 끝점에서
  - `C(U[p]) = P0`
  - `C(U[n+1]) = Pn`
  이 성질이 성립합니다.

### 2.2 unclamped
- 끝 중복도가 `p+1`이 아니므로 위의 “끝점 통과” 성질이 일반적으로 깨집니다.
- 따라서 평가 함수에서 **endpoint snap을 clamped일 때만 적용** 해야 합니다.

---

## 3. `on_unclamp_with_knot_vector`가 보장하는 것과 전제

- 이 함수는 다음을 목표로 합니다.
  - 입력: `(U_old, P)`
  - 출력: `(U_new, P')`
  - 목표: 같은 `u`에 대해 `C_old(u) == C_new(u)`

- 그런데 이 목표가 **항상** 가능한 게 아니라, **특정 형태의 U_new(호환 가능한 U_new)** 에  
  대해서만 확실히 성립합니다.

- 그 “호환 가능한 U_new”의 핵심 조건이 바로:
  - **내부 구간 `U[p..n+1]` 은 절대로 바뀌면 안 된다**
- 입니다.

---

## 4. 왜 내부 knot를 바꾸면 안 되나?

### 4.1 내부 knot를 바꾸면 basis가 전체적으로 바뀜
- `U`가 바뀌면 `N_{i,p}(u;U)`가 바뀌고,
- 그 결과 **같은 컨트롤 포인트로는 같은 곡선을 만들 수 없습니다.**

- 더 중요한 점은:
  - “새 U_new에서 같은 곡선을 표현하는 P'가 존재하느냐?”는
    일반적으로 **항상 보장되지 않습니다**.

### 4.2 이 루틴은 ‘끝 부분 변화’만 처리하는 특수 변환
- `on_unclamp_with_knot_vector`는 구조적으로
  - 좌측 끝에서 몇 단계
  - 우측 끝에서 몇 단계
- 처럼 **끝쪽 컨트롤 포인트들을 연쇄적으로 재계산(back-substitution)** 하는 방식입니다.

- 즉, 처리 가능한 변화는 **끝의 clamping을 푸는 변화** 이고,  
  내부 knot까지 바꿔버리면 변화가 **전체 구간** 으로 퍼져서 이 방식으로는 보존이 성립하지 않습니다.

---

## 5. 헷갈린 지점

### 5.1 원래 clamped
```
0, 0, 0, 0, 0.3, 0.6, 1, 1, 1, 1
```

### 5.2 (잘못된) 새 unclamped 제안
```
0, 0, 0.3, 0.375, 0.45, 0.525, 0.6, 0.7, 1, 1
```

- 여기서 문제는 사용자가 말한 그대로:
  - 원래 내부: `0.3, 0.6`
  - 새 내부: `0.3, 0.375, 0.45, 0.525, 0.6, 0.7`

즉 **내부 knot가 완전히 재구성**되었습니다.

- 이건 **끝 multiplicity만 줄인 unclamp** 가 아니라
  - knot refinement(새 knot 삽입)
  - 혹은 knot redistribution(재분배)
  - 혹은 파라미터 재설계
- 같은 다른 종류의 작업입니다.
- ➡️ 그래서 이런 U_new는 `on_unclamp_with_knot_vector`의 입력으로 **부적합** 합니다.

---

## 6. 올바른 U_new의 형태 (정확한 규칙)

### 6.1 반드시 만족해야 하는 조건
- 다음을 만족해야 합니다.

#### 1) 길이 동일
- `len(U_new) == len(U_old) == n+p+2`

#### 2) non-decreasing

#### 3) 가장 중요한 조건: 내부 구간 일치
- **모든 `i ∈ [p .. n+1]`에 대해**
  - `U_new[i] == U_old[i]` (허용오차 포함)

#### 4) 바뀌어도 되는 곳은 딱 2군데
- 좌측: `U_new[0 .. p-1]`
- 우측: `U_new[n+2 .. n+p+1]`

- 요약하면:
  - **내부(interior)는 그대로**
  - **끝에서만 펼친다**
- 입니다.

### 6.2 직관적인 그림
- clamped:
```
| u0 u0 u0 u0 | 0.3 | 0.6 | u1 u1 u1 u1 |
```
- 호환 가능한 unclamped:
```
| x  y  z  u0 | 0.3 | 0.6 | u1 a  b  c |
```
- 가운데 `u0=U[p]`, `0.3`, `0.6`, `u1=U[n+1]`는 동일
- 좌측 3개, 우측 3개만 바깥쪽으로 새 값

---

## 7. 그럼 “multiplicity만 조정하고 내부는 interpolation”이 맞는 말인가?

- 문장 자체는 두 가지 의미로 해석될 수 있습니다.

### (A) 올바른 의미(이 함수에 해당)
- 내부 knot 값/배치는 유지한다
- 끝쪽 `p`개 knot만 바깥쪽으로 생성(외삽/규칙 기반 생성)

- 이 의미에서는 “끝 부분만 생성한다”는 점에서 ‘보간/외삽’이라고 표현할 수는 있지만,  
  **내부 knot를 보간해서 새 interior를 만들면 안 됩니다.**

### (B) 착각하기 쉬운 의미(이 함수와 무관)
- 내부 knot를 촘촘히 새로 만들고(예: 0.375, 0.45, ...)
- 전체 knot 구조를 재설계

- 이건 `on_unclamp_with_knot_vector`가 아닌
  - global knot redistribution
  - knot insertion/refinement
  - reparameterization
- 쪽의 문제입니다.


## 8. 실전용 호환성 체크(강추)
-  아래 체크가 통과하면 `on_unclamp_with_knot_vector` 입력으로 “개념상” 맞습니다.

```rust
fn compatible_for_unclamp(u_old: &[Real], u_new: &[Real], p: usize, n: usize, eps: Real)
  -> bool {
    if u_old.len() != u_new.len() { return false; }
    // 내부 [p..=n+1] 반드시 동일
    for i in p..=n+1 {
        if (u_old[i] - u_new[i]).abs() > eps { return false; }
    }
    true
}
```

- 추가로(강추):
  - `u_new`가 nondecreasing인지
  - 좌/우 끝쪽에 너무 붙은 값이 없는지(분모가 0에 가까워지는 위험) 도 같이 검사.


## 9. 마지막으로: 왜 평가 함수에서 endpoint snap이 문제였나?

- clamped일 때만
  - `C(U[p]) = P0`, `C(U[n+1]) = Pn`
- 이 성립합니다.

- unclamped 후에는 이 성질이 일반적으로 깨지므로
  - `u<=U[p]`이면 `ctrl[0]`을 강제로 반환
- 같은 처리를 하면 형상이 깨집니다.

- 그래서 endpoint snap을
  - `if self.is_clamped_knot() { ... }`
- 로 제한한 것이 정답입니다.

---

## 핵심만 다시 요약
- **input knot vector(U_new)는 내부가 그대로인 채로 끝만 바뀐 knot vector**
- 내부를 새로 만든 knot(0.375, 0.45, ...)는 이 함수 입력이 아님
- 그런 작업은 다른 알고리즘 영역(재분배/삽입/재파라미터화)

## 소스 코드
```rust
/// - "새로운 unclamped knot vector(knt)"를 입력으로 받아
/// - control points를 그 knot에 맞게 재계산한 뒤
/// - curve의 knot vector를 knt로 교체한다.
///
/// 주의:
/// - interior/끝 knot 값들이 old/new가 “호환”되는 것을 가정한다.
/// - 여기서는 최소한의 크기/인덱스 검증만 한다.
///
/// 전제(원본 의미):
/// - degree p >= 2
/// - curve knot length == knt knot length == (n+p+2)
pub fn on_unclamp_with_knot_vector(cur: &mut NurbsCurve, knt: &KnotVector) -> Result<()> {
    let p = cur.degree as usize;
    if p < 2 {
        return Err(NurbsError::InvalidArgument { msg: "unclamp: p<2".into() });
    }

    let n = cur.ctrl.len().saturating_sub(1);
    let uc_len = cur.kv.knots.len();
    if knt.knots.len() != uc_len {
        return Err(NurbsError::InvalidArgument
          { msg: "unclamp: knot len mismatch".into() });
    }
    if !knt.is_non_decreasing() {
        return Err(NurbsError::InvalidArgument
          { msg: "unclamp: new knots not nondecreasing".into() });
    }

    // ✅ 필수 호환성 체크: interior span은 반드시 동일해야 함 (최소 조건)
    // old/new must agree on [p..=n+1]
    for i in p..=n + 1 {
        if (cur.kv.knots[i] - knt.knots[i]).abs() > 1e-12 {
            return Err(NurbsError::InvalidArgument {
                msg: format!("unclamp: incompatible knot at i={} old={} new={}",
                  i, cur.kv.knots[i], knt.knots[i]),
            });
        }
    }

    let u = &knt.knots;
    let pw = &mut cur.ctrl;

    #[inline]
    fn comb(alf: Real, a: Point4D, bet: Real, b: Point4D) -> Point4D {
        Point4D { x: alf*a.x + bet*b.x, y: alf*a.y + bet*b.y,
          z: alf*a.z + bet*b.z, w: alf*a.w + bet*b.w }
    }

    // LEFT
    for i in 0..=(p - 2) {
        for j in (0..=i).rev() {
            let a = p + j - i - 1;
            let b = p + j + 1;

            let denom = u[b] - u[a];
            if denom.abs() < 1e-18 {
                return Err(NurbsError::NumericError
                  { msg: format!("unclamp(left): denom~0 i={} j={}", i, j) });
            }
            let alf0 = (u[p] - u[a]) / denom;

            let denom2 = alf0 - 1.0;
            if denom2.abs() < 1e-18 {
                return Err(NurbsError::NumericError
                  { msg: format!("unclamp(left): alf0-1~0 i={} j={}", i, j) });
            }
            let bet = alf0 / denom2;
            let alf = 1.0 - bet;

            // LOG
            eprintln!(
                "[unclamp:left] i={} j={} a={} b={} U[p]={:.17} U[a]={:.17} U[b]={:.17} alf0={:.17} bet={:.17} alf={:.17}",
                i, j, a, b, u[p], u[a], u[b], alf0, bet, alf
            );

            let before = pw[j];
            pw[j] = comb(alf, pw[j], bet, pw[j + 1]);
            let after = pw[j];

            eprintln!(
                "[unclamp:left] Pw[{}] before=({:.6},{:.6},{:.6},{:.6}) after=({:.6},{:.6},{:.6},{:.6})",
                j, before.x, before.y, before.z, before.w, after.x, after.y, after.z, after.w
            );
        }
    }

    // RIGHT
    for i in 0..=(p - 2) {
        for j in (0..=i).rev() {
            let a = n - j;
            let b = n - j + i + 2;

            let denom = u[b] - u[a];
            if denom.abs() < 1e-18 {
                return Err(NurbsError::NumericError { msg: format!("unclamp(right): denom~0 i={} j={}", i, j) });
            }
            let alf0 = (u[n + 1] - u[a]) / denom;

            if alf0.abs() < 1e-18 {
                return Err(NurbsError::NumericError { msg: format!("unclamp(right): alf0~0 i={} j={}", i, j) });
            }
            let bet = (alf0 - 1.0) / alf0;
            let alf = 1.0 - bet;

            eprintln!(
                "[unclamp:right] i={} j={} a={} b={} U[n+1]={:.17} U[a]={:.17} U[b]={:.17} alf0={:.17} bet={:.17} alf={:.17}",
                i, j, a, b, u[n+1], u[a], u[b], alf0, bet, alf
            );

            let before = pw[a];
            pw[a] = comb(alf, pw[a], bet, pw[a - 1]);
            let after = pw[a];

            eprintln!(
                "[unclamp:right] Pw[{}] before=({:.6},{:.6},{:.6},{:.6}) after=({:.6},{:.6},{:.6},{:.6})",
                a, before.x, before.y, before.z, before.w, after.x, after.y, after.z, after.w
            );
        }
    }

    // copy knots
    cur.kv.knots.copy_from_slice(u);
    Ok(())
}
```

```rust
pub fn on_make_unclamped_knot_for_non_rational_from_clamped(old: &[Real], p: usize)
  -> Vec<Real> {
    // old must be clamped, len = n+p+2
    let len = old.len();
    let m = len - 1;
    let n = m - p - 1; // highest ctrl index

    let mut u = old.to_vec();

    // --- Left end extension (A12.1 style) ---
    // for i=0..=p-2: U[p-i-1] = U[p-i] - (U[n-i+1] - U[n-i])
    for i in 0..=(p - 2) {
        let idx = p - i - 1;
        u[idx] = u[idx + 1] - (u[n - i + 1] - u[n - i]);
    }
    // Set U[0] similarly (matches common implementation)
    u[0] = u[1] - (u[n - p + 2] - u[n - p + 1]);

    // --- Right end extension (A12.1 style) ---
    // for i=0..=p-2: U[n+i+2] = U[n+i+1] + (U[p+i+1] - U[p+i])
    for i in 0..=(p - 2) {
        let idx = n + i + 2;
        u[idx] = u[idx - 1] + (u[p + i + 1] - u[p + i]);
    }
    // Set last knot
    let last = n + p + 1;
    u[last] = u[last - 1] + (u[2 * p] - u[2 * p - 1]);

    u
}
```

```rust
/// - 내부는 기존 clamped 내부와 동일 유지
/// - 양끝은 기존 값에서 바깥으로 1 step 확장
pub fn on_make_unclamped_knot_from_clamped(clamped: &[Real], p: usize)
  -> Vec<Real> {
    let m = clamped.len() - 1;
    let n = m - p - 1; // highest ctrl index
    let mut u = clamped.to_vec();

    // left extension: U[p-1],U[p-2]... 를 U[p]에서 같은 간격으로 왼쪽 확장
    // stepL = U[p+1]-U[p] (가능하면)
    let step_l = if p + 1 <= m { u[p + 1] - u[p] } else { 1.0 };
    for i in 0..=p {
        // keep interior and ends same? 우리는 "끝 multiplicity 제거"가 목적이라
        // clamped[0..=p]를 재배치해야 하는데,
        // U[p]는 그대로 두고 U[p-1..0]을 감소시키는 형태를 만들자.
        if p >= 1 && i <= p - 1 {
            let idx = p - 1 - i;
            u[idx] = u[idx + 1] - step_l;
        }
    }
    // right extension: U[n+2..n+p+1]를 U[n+1]에서 오른쪽 확장
    let step_r = if n >= 1 { u[n + 1] - u[n] } else { step_l };
    for t in 0..=p {
        let idx = n + 2 + t;
        if idx <= m {
            u[idx] = u[idx - 1] + step_r;
        }
    }
    // interior should remain nondecreasing overall for typical step choices
    u
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests_unclamp_with_knot_vector {
    use nurbslib::core::geom::{Point3D, Point4D};
    use nurbslib::core::knot::KnotVector;
    use nurbslib::core::math_extensions::on_make_unclamped_knot_for_non_rational_from_clamped;
    use nurbslib::core::nurbs_curve::{on_unclamp_with_knot_vector, NurbsCurve};
    use nurbslib::core::types::{Real, Degree};

    fn max_abs(a: Real, b: Real) -> Real { if a.abs() > b.abs() { a.abs() } else { b.abs() } }

    fn dist3(a: Point3D, b: Point3D) -> Real {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn is_nondecreasing(a: &[Real]) -> bool {
        a.windows(2).all(|w| w[0] <= w[1])
    }

    fn is_clamped(knots: &[Real], p: usize) -> bool {
        if knots.len() < 2 * (p + 1) { return false; }
        let a = knots[0];
        let b = *knots.last().unwrap();
        (0..=p).all(|i| knots[i] == a && knots[knots.len() - 1 - i] == b)
    }

    fn all_finite(a: &[Real]) -> bool {
        a.iter().all(|x| x.is_finite())
    }

    fn make_random_ctrl(n_ctrl: usize, seed: &mut u64) -> Vec<Point4D> {
        fn next_u(seed: &mut u64) -> u64 {
            *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            *seed
        }
        fn next_f(seed: &mut u64) -> Real {
            let u = next_u(seed);
            // [0,1)
            (u >> 11) as Real / ((1u64 << 53) as Real)
        }

        let mut out = Vec::with_capacity(n_ctrl);
        for _ in 0..n_ctrl {
            let x = (next_f(seed) - 0.5) * 10.0;
            let y = (next_f(seed) - 0.5) * 10.0;
            let z = (next_f(seed) - 0.5) * 10.0;
            out.push(Point4D::homogeneous(x, y, z, 1.0));
        }
        out
    }

    fn curve_max_point_error(a: &NurbsCurve, b: &NurbsCurve, samples: usize) -> Real {
        // domain 기반으로 평가한다는 전제 (a.domain 사용)
        let t0 = a.domain.t0;
        let t1 = a.domain.t1;

        let mut max_err = 0.0;
        for i in 0..=samples {
            let s = (i as Real) / (samples as Real);
            let t = t0 + (t1 - t0) * s;
            let pa = a.eval_point(t);
            let pb = b.eval_point(t);
            let d = dist3(pa, pb);
            if d > max_err { max_err = d; }
        }
        max_err
    }

    fn assert_curves_shape_equal(a: &NurbsCurve, b: &NurbsCurve, samples: usize, tol: Real) {
        let e = curve_max_point_error(a, b, samples);
        assert!(e <= tol, "curve shape mismatch: max_err={} tol={}", e, tol);
    }
  ```
  ```rust
    #[test]
    fn unclamp_with_knot_vector_preserves_shape_non_rational() {
        let p = 3usize;
        let n_ctrl = 9usize;
        let mut seed = 0x1234_5678_9abc_def0u64;

        let ctrl = make_random_ctrl(n_ctrl, &mut seed);
        let cur0 = NurbsCurve::from_ctrl_clamped_uniform(p as Degree, ctrl);

        let new_knots = on_make_unclamped_knot_for_non_rational_from_clamped(&cur0.kv.knots, p);
        let knt = KnotVector { knots: new_knots };

        let mut cur1 = cur0.clone();
        on_unclamp_with_knot_vector(&mut cur1, &knt).unwrap();
        assert_eq!(cur1.kv.knots, knt.knots);

        // ✅ knot-domain에서 비교
        let n = cur0.ctrl.len() - 1;
        let u0 = cur0.kv.knots[p];
        let u1 = cur0.kv.knots[n + 1];

        let samples = 400usize;
        let mut max_err = 0.0;
        for s in 0..=samples {
            let a01 = (s as Real) / (samples as Real);
            let u = u0 + (u1 - u0) * a01;

            // 핵심: 둘 다 같은 "u"로 평가
            let a = cur0.point_at_h_knot(u).to_point();
            let b = cur1.point_at_h_knot(u).to_point();

            let d = dist3(a, b);
            if d > max_err { max_err = d; }
        }

        assert!(max_err < 1e-9, "unclamp changed shape too much: {}", max_err);
    }
  ```
  ```rust
    #[test]
    fn unclamp_with_knot_vector_works_for_rational_curve_too() {
        // simple rational curve: same xyz but varying weights
        let p = 3usize;
        let n_ctrl = 8usize;
        let mut seed = 0xdead_beef_cafe_babeu64;

        let mut ctrl = make_random_ctrl(n_ctrl, &mut seed);
        // inject weights
        for (i, cp) in ctrl.iter_mut().enumerate() {
            cp.w = if i % 2 == 0 { 2.0 } else { 0.5 };
            // keep homogeneous consistent in your representation; here Point4D is (x,y,z,w)
        }

        let cur0 = NurbsCurve::from_ctrl_clamped_uniform(p as Degree, ctrl);
        assert!(cur0.is_rational());

        let new_knots = on_make_unclamped_knot_for_non_rational_from_clamped(&cur0.kv.knots, p);
        println!("new_knots: {:?}", new_knots);
        println!("cur0.kv.knots: {:?}", cur0.kv.knots);

        let knt = KnotVector { knots: new_knots };

        let mut cur1 = cur0.clone();
        on_unclamp_with_knot_vector(&mut cur1, &knt).expect("unclamp failed");

        // knot vector replaced
        assert_eq!(cur1.kv.knots, knt.knots);

        // shape preservation in Euclidean space
        let samples = 200usize;
        let mut max_err = 0.0;
        for s in 0..=samples {
            let t = (s as Real) / (samples as Real);
            let a = cur0.eval_point(t);
            let b = cur1.eval_point(t);
            let d = dist3(a, b);
            if d > max_err { max_err = d; }
        }
        assert!(max_err < 1e-9, "rational unclamp changed shape: {}", max_err);
    }
  ```
  ```rust
    #[test]
    fn unclamp_with_knot_vector_rejects_bad_length() {
        let p = 3usize;
        let n_ctrl = 6usize;
        let mut seed = 111222333u64;

        let ctrl = make_random_ctrl(n_ctrl, &mut seed);
        let mut cur = NurbsCurve::from_ctrl_clamped_uniform(p as Degree, ctrl);

        let mut bad = cur.kv.knots.clone();
        bad.push(1.0); // wrong length

        let knt = KnotVector { knots: bad };
        let rc = on_unclamp_with_knot_vector(&mut cur, &knt);
        assert!(rc.is_err());
    }
  ```
  ```rust
    #[test]
    fn unclamp_with_knot_vector_rejects_degree_lt_2() {
        let p = 1usize;
        let ctrl = vec![
            Point4D::homogeneous(0.0, 0.0, 0.0, 1.0),
            Point4D::homogeneous(1.0, 0.0, 0.0, 1.0),
        ];
        let mut cur = NurbsCurve::from_ctrl_clamped_uniform(p as Degree, ctrl);

        // same length knot vector, but p<2 should fail
        let knt = KnotVector { knots: cur.kv.knots.clone() };
        let rc = on_unclamp_with_knot_vector(&mut cur, &knt);
        assert!(rc.is_err());
    }
}
```
---
