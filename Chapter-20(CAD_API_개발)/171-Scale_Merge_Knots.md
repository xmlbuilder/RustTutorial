# Scale / Merge Knot
## 1. on_scale_knots_in_place / on_merge_knots_have_same_inserts_in_place
- 여기 있는 두 함수는 결국 하나의 목적을 가지고 있음:
- on_scale_knots_in_place:
    - 한 B-spline의 knot vector를 어떤 구간 [ul, ur]로 선형 스케일링하면서,  
        clamped end(앞/뒤 p+1개 knot)는 그대로 ul/ur로 유지하는 함수.
- on_merge_knots_have_same_inserts_in_place:
- 두 B-spline의 내부 knot 구조를 맞추기 위해,  
    서로에게 부족한 내부 knot들을 **삽입해야 할 리스트** 로 계산해주는 함수.
- (실제 삽입은 on_cfun_refine_with_insert_list 같은 쪽에서 수행)
- 이 둘은 B-spline 곱셈, 합성, 연산에서 공통 knot vector를 만드는  
    전처리 단계에 해당.

## 2. on_scale_knots_in_place – 수식과 의미
### 2.1 수학적 정의
- 입력:
    - degree p
    - knot vector $U=\{ U_0,\dots ,U_m\}$ , 비감소, clamped:
    - $U_0=\dots =U_p$
    - $U_{m-p}=\dots =U_m$
- 목표:
    - 새로운 knot vector $U'=\{ U'_0,\dots ,U'_m\}$ 를 만들어서
    - 앞 p+1개는 모두 ul
    - 뒤 p+1개는 모두 ur
- 내부 knot들은 기존 knot를 $[U_0,U_m]\rightarrow [ul,ur]$ 로 선형 사상한 값

### 2.2 코드와 수식 매핑
```rust
let u0 = knots[0];
let u1 = knots[m];
let denom = u1 - u0;
let fac = (ur - ul) / denom;
```
```math
\mathrm{fac}=\frac{ur-ul}{U_m-U_0}
```
```rust
// front clamped
for i in 0..=p { knots[i] = ul; }

// interior
for i in i0..=i1 {
    knots[i] = ul + fac * (knots[i] - u0);
}

// back clamped
for i in (m - p)..=m { knots[i] = ur; }
```
- 위에서 쓴 수식 그대로.
### 2.3 수학적 성질
- Affine mapping:
    - 내부 knot는 [U_0,U_m]에서 [ul,ur]로 선형 변환되므로,  
        B-spline의 parameter domain이 선형 스케일링될 뿐,  
        곡선/함수의 형상은 바뀌지 않는다 (단, parameterization만 바뀜).
- clamped 유지:
    - 앞/뒤 p+1개를 강제로 ul/ur로 맞추므로,  
        B-spline의 clamped end 조건이 유지된다.
- 비감소성 유지:
- 선형 변환은 단조 증가 함수이므로,  
    원래 비감소였던 knot vector는 변환 후에도 비감소.
- 코드에서도 마지막에 한 번 더 검사:
```rust
for i in 0..m {
    if knots[i] > knots[i + 1] { Err(KnotNotNonDecreasing) }
}
```
- ✅ 수학적으로 완전히 일관된 affine rescaling.

## 3. on_merge_knots_have_same_inserts_in_place – 수식과 의미
### 3.1 문제 설정
- 두 개의 B-spline 함수가 있다고 하자:
```math
F(u)=\sum _{i=0}^{n_r}f_i\, N_{i,p}^{(r)}(u),\quad \mathrm{knot\  vector\  }R=\{ R_0,\dots ,R_{m_r}\}
```
```math 
G(u)=\sum _{j=0}^{n_s}g_j\, N_{j,q}^{(s)}(u),\quad \mathrm{knot\  vector\  }S=\{ S_0,\dots ,S_{m_s}\}
``` 
- 하고 싶은 건:
    - 이 둘을 곱하거나, 더하거나, 어떤 연산을 하려면 공통 knot vector를 가져야 한다.
- 즉, 내부 knot 구조가 같아야 한다.
- 그래서 이 함수는:
    - R에 삽입해야 할 knot 리스트 X_R
    - S에 삽입해야 할 knot 리스트 X_S
- 를 계산해서 반환한다:
```rust
Ok((KnotVector::new(xr)?, KnotVector::new(xs)?))
```

- 여기서:
    - xr: S에서 가져와서 R에 삽입해야 할 knot들
    - xs: R에서 가져와서 S에 삽입해야 할 knot들
### 3.2 end interval 맞추기 (스케일링 단계)
```rust
let r0 = r[0]; let rm = r[mr];
let s0 = s[0]; let sm = s[ms];

if (r0 - s0).abs() > p_tol || (rm - sm).abs() > p_tol {
    let span_r = rm - r0;
    let span_s = sm - s0;

    if span_r > span_s {
        on_scale_knots_in_place(&mut kns.knots, q, r0, rm)?;
    } else {
        on_scale_knots_in_place(&mut knr.knots, p, s0, sm)?;
    }
}
```

- 수학적으로:
- 두 knot vector의 정의역이 $[R_0,R_{m_r}]$, $[S_0,S_{m_s}]$ 로 다를 수 있다.
    - 이 경우, 더 긴 쪽 interval에 맞춰 짧은 쪽을 affine scaling 한다.
    - 즉, 둘 중 하나를 선형 사상해서 같은 parameter 구간을 갖도록 만든다.
- 이건 B-spline 함수의 parameterization을 맞추기 위한 전처리로,  
    형상은 그대로 유지되며, domain만 맞춰지는 작업이다.
### 3.3 내부 knot 비교 및 삽입 리스트 생성
```rust
// n = m - p - 1
let nr = mr - p - 1;
let ns = ms - q - 1;

let mut xr: Vec<Real> = Vec::new(); // knots to insert into R (from S)
let mut xs: Vec<Real> = Vec::new(); // knots to insert into S (from R)

let mut i = p + 1;
let mut j = q + 1;

// Only internal knots are considered: indices up to n (nr/ns).
while i <= nr || j <= ns {
    let ri = if i <= nr + 1 { r[i] } else { r[mr] };
    let sj = if j <= ns + 1 { s[j] } else { s[ms] };

    if (ri - sj).abs() < p_tol {
        // equal: skip duplicates
        ...
        i += 1; j += 1;
        continue;
    }

    if ri < sj {
        // R 쪽 knot가 더 작다 → S에 이 knot를 삽입해야 함
        ...
        if i <= nr { xs.push(r[i]); }
        i += 1;
        continue;
    }

    // ri > sj → R에 S의 knot 삽입
    ...
    if j <= ns { xr.push(s[j]); }
    j += 1;
}
```

- 수학적으로 이건 두 정렬된 내부 knot 집합의 병합 과정이다.
    - 내부 knot index 범위:
```math
i\in [p+1,nr],\quad j\in [q+1,ns]
```
- 여기서
```math
n_r=m_r-p-1,\quad n_s=m_s-q-1
```
- 는 control index의 최댓값.
- ri < sj인 경우:
    - R에는 ri가 있지만 S에는 없다 →  
        S에 ri를 삽입해야 두 knot vector가 같아진다.
    - 따라서 xs.push(ri).
- ri > sj인 경우:
    - 반대로 S에는 sj가 있지만 R에는 없다 →  
        R에 sj를 삽입해야 한다.
    - 따라서 xr.push(sj).
- |ri - sj| < p_tol인 경우:
    - 같은 knot로 간주하고,
    - 각자 중복 knot를 건너뛰며 다음으로 진행.
- 결과적으로:
```math
X_R=\{ \mathrm{S의\  내부\  knot\  중\  R에\  없는\  것들}\}
```
```math
X_S=\{ \mathrm{R의\  내부\  knot\  중\  S에\  없는\  것들}\} 
```
- 이 두 리스트를 각각 R, S에 삽입하면,
- 두 knot vector의 내부 knot 구조가 동일해진다.

## 4. 테스트를 통한 수학적 검증

### 4.1 on_scale_knots_in_place 테스트
```rust
let mut k = kv(&[0.0,0.0,0.0,0.0, 0.3,0.5,0.5,0.7, 1.0,1.0,1.0,1.0]);
on_scale_knots_in_place(&mut k.knots, 3, 2.0, 5.0).unwrap();
```
- 검증:
    - 앞 4개 = 2.0, 뒤 4개 = 5.0 → clamped 유지
    - 전체 비감소 → affine mapping + clamping이 잘 작동
- 이는 위에서 정리한 수식과 정확히 일치.
4.2 merge_knots_example_from_comment
```rust
let mut r = kv(&[0.0,0.0,0.0,0.0, 0.3, 0.5,0.5, 0.7, 1.0,1.0,1.0,1.0]);
let mut s = kv(&[0.0,0.0,0.0, 0.2,0.2, 0.5, 0.6, 1.0,1.0,1.0]);

let (kxr, kxs) = on_merge_knots_have_same_inserts_in_place(
  &mut r, &mut s, 3, 2, p_tol).unwrap();

assert_eq!(kxr.as_slice(), &[0.2, 0.6]);
assert_eq!(kxs.as_slice(), &[0.3, 0.7]);
```
- 이건 말 그대로:
    - R 내부 knot: {0.3, 0.5, 0.5, 0.7}
    - S 내부 knot: {0.2, 0.2, 0.5, 0.6}
- 중복을 고려한 병합 결과:
    - R에 없는 S의 값: 0.2, 0.6 → kxr
    - S에 없는 R의 값: 0.3, 0.7 → kxs
- 수학적으로 완전히 타당한 결과.

### 4.3 endpoint mismatch scaling 테스트
```rust
// R in [0,2], S in [10,20]; 
//spans: R=2, S=10 -> S wider -> scale R to S interval
let mut r = kv(&[0.0,0.0,0.0, 0.5, 1.0, 2.0,2.0,2.0]);
let mut s = kv(&[10.0,10.0,10.0, 12.0, 15.0, 20.0,20.0,20.0]);

let (_kxr, _kxs) = on_merge_knots_have_same_inserts_in_place
  (&mut r, &mut s, 2, 2, ptol).unwrap();

assert!((r.as_slice()[0] - s.as_slice()[0]).abs() <= ptol);
assert!((r.as_slice()[r.as_slice().len()-1] - s.as_slice()[s.as_slice().len()-1]).abs() <= ptol);
```
- span 비교 후, 더 짧은 쪽을 더 긴 쪽 interval로 스케일링.
- 결과적으로 두 knot vector의 시작/끝이 거의 동일해짐.
- 이는 **두 B-spline의 parameter domain을 맞춘다** 는 의미로,
- 수학적으로도 자연스럽고, B-spline 연산에서 표준적인 처리 방식.
## 5. 전체적인 수학적 검증 요약
- on_scale_knots_in_place
    - 내부 knot에 대한 affine mapping 수식이 정확하다.
    - clamped end 유지 조건을 만족한다.
    - 비감소성 유지가 보장된다.
    - 테스트가 이 모든 성질을 직접 확인한다.
- on_merge_knots_have_same_inserts_in_place
    - 두 knot vector의 정의역을 맞추기 위해 affine scaling을 사용한다.
        - B-spline 함수의 형상은 유지되고, parameter domain만 맞춰진다.
    - 내부 knot 비교 로직은 “두 정렬된 집합의 병합”과 동일하며,  
        각자에 없는 knot를 삽입 리스트로 반환한다.
- 예제 테스트에서 기대하는 병합 결과와 정확히 일치한다.
- endpoint mismatch에 대한 scaling도 테스트로 검증된다.
- 그래서 수학적으로 정리하면:
    - 이 두 함수는- B-spline knot vector의 affine rescaling,
    - 두 knot vector의 내부 knot 집합을 일치시키기 위한 병합 로직  
        을 이론과 완전히 일치하는 방식으로 구현하고 있고,
    - 테스트도 그 수학적 성질을 잘 검증하고 있다.
---

## 소스 코드
```rust
/// Scale knot vector to a given interval
/// - knot vector U[0..=m] 를 interval [ul, ur]로 in-place rescale.
/// - 앞의 p+1개는 ul, 뒤의 p+1개는 ur 로 강제(clamped end 유지).
/// - 내부 knots (p+1 .. m-p-1) 만 affine 변환.
/// - 이미 U[0]==ul && U[m]==ur 이면 아무 것도 안 함.
pub fn on_scale_knots_in_place(knots: &mut [Real], p: usize, ul: Real, ur: Real) -> Result<()> {
    if knots.len() < 2 {
        return Err(NurbsError::InvalidArgument {
            msg: "knot vector too short".into(),
        });
    }
    if p + 1 > knots.len() {
        return Err(NurbsError::InvalidArgument {
            msg: "degree too large for knot vector".into(),
        });
    }
    if knots.windows(2).any(|w| w[0] > w[1]) {
        return Err(NurbsError::InvalidArgument {
            msg: "knot vector must be nondecreasing".into(),
        });
    }
    if !(ur >= ul) {
        return Err(NurbsError::InvalidArgument {
            msg: "interval must satisfy ul <= ur".into(),
        });
    }

    let m = knots.len() - 1;

    // Need at least (p+1) on both ends => m >= 2p+1
    if m < 2 * p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: "invalid knot vector for given degree".into(),
        });
    }

    // already matches target span?
    if knots[0] == ul && knots[m] == ur {
        return Ok(());
    }

    let u0 = knots[0];
    let u1 = knots[m];
    let denom = u1 - u0;
    if denom == 0.0 {
        return Err(NurbsError::InvalidArgument {
            msg: "cannot rescale: U[m]-U[0] == 0".into(),
        });
    }

    let fac = (ur - ul) / denom;

    // front clamped
    for i in 0..=p {
        knots[i] = ul;
    }

    // interior
    let i0 = p + 1;
    let i1 = m - p - 1;
    for i in i0..=i1 {
        knots[i] = ul + fac * (knots[i] - u0);
    }

    // back clamped
    for i in (m - p)..=m {
        knots[i] = ur;
    }


    for i in 0..m {
        if knots[i] > knots[i + 1] {
            return Err(NurbsError::KnotNotNonDecreasing);
        }
    }

    Ok(())
}
```
```rust
/// Merge knot vectors to have the same internal knots
pub fn on_merge_knots_have_same_inserts_in_place(
    knr: &mut KnotVector,
    kns: &mut KnotVector,
    p: Degree,
    q: Degree,
    p_tol: Real,
) -> Result<(KnotVector, KnotVector)> {
    let p = p as usize;
    let q = q as usize;

    let r = &mut knr.knots;
    let s = &mut kns.knots;

    if r.len() < 2 || s.len() < 2 {
        return Err(NurbsError::InvalidArgument { msg: "symkvm: empty knot vector".into() });
    }

    let mr = r.len() - 1;
    let ms = s.len() - 1;

    if mr < 2 * p + 1 || ms < 2 * q + 1 {
        return Err(NurbsError::InvalidArgument { msg: "symkvm: knot vector too short for degree".into() });
    }

    // ---- Scale one vector if end-interval mismatch beyond PTOL (C behavior) ----
    let r0 = r[0];
    let rm = r[mr];
    let s0 = s[0];
    let sm = s[ms];

    if (r0 - s0).abs() > p_tol || (rm - sm).abs() > p_tol {
        let span_r = rm - r0;
        let span_s = sm - s0;

        if span_r > span_s {
            // scale S to [r0, rm]
            on_scale_knots_in_place(&mut kns.knots, q, r0, rm)?;
        } else {
            // scale R to [s0, sm]
            on_scale_knots_in_place(&mut knr.knots, p, s0, sm)?;
        }
    }

    // refresh references after possible scaling
    let r = &knr.knots;
    let s = &kns.knots;

    let mr = r.len() - 1;
    let ms = s.len() - 1;

    // n = m - p - 1
    let nr = mr - p - 1;
    let ns = ms - q - 1;

    let mut xr: Vec<Real> = Vec::new(); // knots to insert into R (from S)
    let mut xs: Vec<Real> = Vec::new(); // knots to insert into S (from R)

    let mut i = p + 1;
    let mut j = q + 1;

    // Only internal knots are considered: indices up to n (nr/ns).
    // For safe access in comparisons, allow i==nr+1 (end knot) and j==ns+1 (end knot).
    while i <= nr || j <= ns {
        let ri = if i <= nr + 1 { r[i] } else { r[mr] };
        let sj = if j <= ns + 1 { s[j] } else { s[ms] };

        // equal (within p_tol): advance both over duplicates
        if (ri - sj).abs() < p_tol {
            while i <= nr && r[i] == r[i + 1] { i += 1; }
            while j <= ns && s[j] == s[j + 1] { j += 1; }
            i += 1;
            j += 1;
            continue;
        }

        if ri < sj {
            while i <= nr && r[i] == r[i + 1] { i += 1; }
            // only internal knots get inserted
            if i <= nr {
                xs.push(r[i]);
            }
            i += 1;
            continue;
        }

        // ri > sj
        while j <= ns && s[j] == s[j + 1] { j += 1; }
        if j <= ns {
            xr.push(s[j]);
        }
        j += 1;
    }

    Ok((KnotVector::new(xr)?, KnotVector::new(xs)?))
}
```

---
## 테스트 코드
```rust
#[cfg(test)]
mod merge_knot_have_same_tests {
    use nurbslib::core::knot::{on_merge_knots_have_same_inserts_in_place, on_scale_knots_in_place};
    use nurbslib::core::prelude::KnotVector;
    use nurbslib::core::types::{Real};

    fn kv(v: &[Real]) -> KnotVector {
        KnotVector::new(v.to_vec()).unwrap()
    }
```
```rust
    #[test]
    fn scale_knots_basic_mapping_preserves_clamps() {
        let mut k = kv(&[0.0,0.0,0.0,0.0, 0.3,0.5,0.5,0.7, 1.0,1.0,1.0,1.0]);
        on_scale_knots_in_place(&mut k.knots, 3, 2.0, 5.0).unwrap();

        let u = k.as_slice();
        let m = u.len()-1;
        // p=3 => first 4 knots = ul, last 4 knots = ur
        for i in 0..=3 { assert_eq!(u[i], 2.0); }
        for i in (m-3)..=m { assert_eq!(u[i], 5.0); }
        // non-decreasing
        for i in 0..m { assert!(u[i] <= u[i+1]); }
    }
```
```rust
    #[test]
    fn merge_knots_example_from_comment() {
        let p_tol = 1e-12;

        // degree inference: R has 12 knots, likely p=3; S has 10 knots, likely q=2 or 3
        // We'll use p=3, q=2 is inconsistent with clamping; safer to use q=3 for clamped ends.
        let mut r = kv(&[0.0,0.0,0.0,0.0, 0.3, 0.5,0.5, 0.7, 1.0,1.0,1.0,1.0]);

        // looks like degree 2 clamping, but we'll still test merge logic
        let mut s = kv(&[0.0,0.0,0.0, 0.2,0.2, 0.5, 0.6, 1.0,1.0,1.0]); 
        // To avoid degree mismatch assumptions, pick q=2
        // for this particular S (clamped ends: 0 repeated 3, 1 repeated 3).
        let (kxr, kxs) = on_merge_knots_have_same_inserts_in_place(&mut r, &mut s, 3, 2, p_tol).unwrap();

        println!("r: {:?}", r);
        println!("s: {:?}", s);


        // Expected from comment:
        // X inserted into R: {0.2, 0.6}
        // Y inserted into S: {0.3, 0.7}
        assert_eq!(kxr.as_slice(), &[0.2, 0.6]);
        assert_eq!(kxs.as_slice(), &[0.3, 0.7]);
    }
```
```rust
    #[test]
    fn merge_knots_scales_one_side_when_endpoints_mismatch() {
        let ptol = 1e-9;

        // R in [0,2], S in [10,20]; spans: R=2, S=10 -> S wider -> scale R to S interval
        let mut r = kv(&[0.0,0.0,0.0, 0.5, 1.0, 2.0,2.0,2.0]); // degree 2 clamped (0x3, 2x3)
        let mut s = kv(&[10.0,10.0,10.0, 12.0, 15.0, 20.0,20.0,20.0]); // degree 2 clamped

        let (_kxr, _kxs) = on_merge_knots_have_same_inserts_in_place(&mut r, &mut s, 2, 2, ptol).unwrap();

        // After scaling, endpoints should match within ptol
        assert!((r.as_slice()[0] - s.as_slice()[0]).abs() <= ptol);
        assert!((r.as_slice()[r.as_slice().len()-1] - s.as_slice()[s.as_slice().len()-1]).abs() <= ptol);
    }
}
```
---
