# on_extract_curve_segment

- 이 함수는 NURBS 커브의 부분 구간 [ul, ur]만 정확하게 잘라내는 알고리즘인데,  
    일반적인 **split → trim** 방식이 아니라 로컬 knot-insertion 방식으로 직접 control net을 재구성하는 고급 커널 루틴.

## 🎯 함수의 목적 (수학적 관점)
- 주어진 NURBS 곡선:
```math
C(u)=\frac{\sum _{i=0}^nN_{i,p}(u)\, w_i\, P_i}{\sum _{i=0}^nN_{i,p}(u)\, w_i}
```
- 이 곡선의 부분 구간:
```math
C(u),\quad u\in [u_l,u_r]
```
- 을 새로운 NURBS 곡선으로 만들고 싶다.
- 즉, 새로운 곡선:
```math
\tilde {C}(u)=C(u),\quad u\in [u_l,u_r]
```
을 정확하게 표현하는 control points + knot vector를 생성하는 것이 목표.

## 🌟 핵심 아이디어
- NURBS 곡선은 knot vector에 의해 구간이 정의되므로,
- 부분 구간을 정확히 추출하려면:
    - ul을 knot vector에 삽입
    - ur을 knot vector에 삽입
    - 해당 구간에 해당하는 control points만 추출
    - 새로운 knot vector를 구성
- 즉, knot insertion을 두 번 수행한 뒤 control net을 잘라내는 과정이다.
- 이 함수는 split()을 쓰지 않고, knot insertion 수식을 직접 구현한 것이다.

### 📌 1. Knot insertion 수식
- NURBS knot insertion의 기본 수식:
- 기존 control points $P_i$ 에서
- 새 knot $u^*$ 를 삽입할 때:
```math
P_i'=\alpha _iP_i+(1-\alpha _i)P_{i-1}
```
- 여기서:
```math
\alpha _i=\frac{u^*-U_i}{U_{i+p+1}-U_i}
```
- 이 함수는 이 수식을 그대로 사용한다.

### 📌 2. ul 삽입 (Left insertion)
- Rust 코드:
```rust
let alf = (ul - left) / den;
let oma = 1.0 - alf;
qw[j] = alf * qw[j+1] + oma * qw[j];
```

- 수식 그대로:
```math
Q_j^{(new)}=\alpha Q_{j+1}+(1-\alpha )Q_j
```
- 여기서:
```math
\alpha =\frac{u_l-U_{ll+i+j}}{U_{spl+j+1}-U_{ll+i+j}}
```
- 이 과정은 ul을 knot vector에 삽입하는 것과 동일하다.

## 📌 3. ur 삽입 (Right insertion)
- Rust 코드:
```rust
let alf = (ur - left) / den;
qw[k] = alf * qw[k] + oma * qw[k - 1];
```

- 수식:
```math
Q_k^{(new)}=\alpha Q_k+(1-\alpha )Q_{k-1}
```
- 여기서:
```math
\alpha =\frac{u_r-U_{lr+i+j}}{U_{spr+j+1}-U_{lr+i+j}}
```
- 이것은 ur을 knot vector에 삽입하는 과정이다.

## 📌 4. Control point 범위 계산
- ul 삽입 후, ur 삽입 후
- 유효한 control point index 범위는:
```math
i_s=spl-p
```
```math
i_e=spr-mlr
```
- 즉:
- $i_s = ul$ 삽입 후 시작 control index
- $i_e = ur$ 삽입 후 끝 control index
- Rust 코드:
```rust
let is = spl_usize - p_usize;
let ie = spr_usize - mlr_usize;
```

- 이 범위의 control points만 남기면  
    부분 구간을 정확히 표현할 수 있다.

## 📌 5. 새로운 knot vector 구성
- 새 knot vector는 다음 구조를 가진다:
```math
U[spl+1],U[spl+2],\dots ,U[spr-mlr]
```
- Rust 코드:
```rust
for i=0..p: uq[j++] = ul
for i=spl+1..spr-mlr: uq[j++] = U[i]
for i=0..p: uq[j++] = ur
```

- 이건 clamped NURBS의 부분 구간 knot vector 표준 형태다.

## 📌 6. 최종적으로 얻는 곡선
- 결과 곡선:
    - degree = p
    - control points = $Q_{i_s},\dots ,Q_{i_e}$
    - knot vector = 위에서 구성한 uq
- 이 곡선은 원래 곡선의 정확한 부분 구간이다.

## 🎯 전체 알고리즘 요약 (수식 기반)
- ul의 knot span spl과 multiplicity mll 계산
- ur의 knot span spr과 multiplicity mlr 계산
- ul을 knot insertion 수식으로 삽입
- ur을 knot insertion 수식으로 삽입
- control point 범위 $[i_s,i_e]$ 추출
    - 새로운 knot vector 구성
    - 새로운 NURBS 곡선 생성

🌟 이 함수가 중요한 이유
- split()을 쓰지 않고 정확한 부분 구간 추출 가능
- CAD 커널에서 trimmed curve 생성 시 필수
- 곡면 trimming, intersection curve trimming 등에서 핵심
- NURBS의 수학적 정의에 100% 부합하는 방식

```rust
/// Extract curve segment [ul, ur] from a clamped NURBS curve
/// using local knot-insertion style (NO split()).
///
/// - ur must be > ul
/// - ul, ur must satisfy U[0] <= ul and ur <= U[m]
/// - uses N_KNTFSM(..., LEFT) for both ul and ur
/// - special-case: if ur == U[m-p], then spr=m, mlr=p+1
pub fn on_extract_curve_segment(
    cur_p: &NurbsCurve,
    ul: Real,
    ur: Real,
) -> Result<NurbsCurve> {
    // ---- local notation (C: U_curbre, U_curknp) ----
    let n_p = cur_p.degree as usize;
    let up = cur_p.kv.as_slice();
    let pw = &cur_p.ctrl;

    if ur <= ul {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: ur must be > ul".into(),
        });
    }
    if up.is_empty() {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: empty knot vector".into(),
        });
    }

    let m_full = up.len() - 1;

    // requirement: U[0] <= ul and ur <= U[m]
    if ul < up[0] || ur > up[m_full] {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: ul/ur outside [U0..Um]".into(),
        });
    }
    if n_p + 1 > up.len() {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: invalid degree vs knot vector".into(),
        });
    }

    // ---- Find knot spans and multiplicities ----
    let (mut spl, mut mll) = on_find_knot_span_and_multiplicity(&cur_p.kv, cur_p.degree, ul, Side::Left)?;
    let (mut spr, mut mlr) = on_find_knot_span_and_multiplicity(&cur_p.kv, cur_p.degree, ur, Side::Left)?;

    let n_spl = spl as usize;
    let mut n_spr = spr as usize;
    let n_mll = mll as usize;
    let mut n_mlr = mlr as usize;

    // is = spl - p
    if n_spl < n_p {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: invalid left span (spl < p)".into(),
        });
    }
    let is = n_spl - n_p;

    // if( ur == UP[m-p] ) { spr = m; mlr = p+1; }
    // NOTE: use same tol policy as your N_KNTFSM (1e-14)
    if (ur - up[m_full - n_p]).abs() < 1e-14 {
        n_spr = m_full;
        n_mlr = n_p + 1;
    }

    // ie = spr - mlr
    if n_spr < n_mlr {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: invalid right span/mult (spr < mlr)".into(),
        });
    }
    let ie = n_spr - n_mlr;

    if ie < is || ie >= pw.len() {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: computed control range out of bounds".into(),
        });
    }

    // New n,m in are "highest indexes" (not counts)
    // n = ie - is
    // m = spr - spl - mlr + 2*p + 1
    let n_hi = ie - is;
    let m_hi = n_spr
        .checked_sub(n_spl).ok_or_else(|| NurbsError::InvalidArgument{ msg: "on_extract_curve_segment: bad index arithmetic".into() })?
        .checked_sub(n_mlr).ok_or_else(|| NurbsError::InvalidArgument{ msg: "on_extract_curve_segment: bad index arithmetic".into() })?
        + 2 * n_p
        + 1;

    // Allocate output arrays (counts = highest+1)
    let mut qw = vec![Point4D::default(); n_hi + 1];
    let mut uq = vec![0.0; m_hi + 1];

    // ---- Copy initial control points ----
    for (dst, src) in (0..=n_hi).zip(is..=ie) {
        qw[dst] = pw[src];
    }

    // Helper: A_comcpt(alf, A, oma, B) => alf*A + oma*B
    #[inline]
    fn comb(alf: Real, a: Point4D, oma: Real, b: Point4D) -> Point4D {
        Point4D {
            x: alf * a.x + oma * b.x,
            y: alf * a.y + oma * b.y,
            z: alf * a.z + oma * b.z,
            w: alf * a.w + oma * b.w,
        }
    }

    // ---- Insert the left knot ----
    // ll = spl - p
    let ll = n_spl - n_p;
    // for i = 1..= p - mll
    if n_mll > n_p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: left multiplicity > p+1".into(),
        });
    }
    for i in 1..=(n_p.saturating_sub(n_mll)) {
        // for j=0..=p-i-mll
        let jmax = n_p
            .checked_sub(i).ok_or_else(|| NurbsError::InvalidArgument{ msg: "N_toocsg: bad left loop".into() })?
            .checked_sub(n_mll).ok_or_else(|| NurbsError::InvalidArgument{ msg: "N_toocsg: bad left loop".into() })?;
        for j in 0..=jmax {
            let left = up[ll + i + j];
            let den = up[n_spl + j + 1] - left;
            if den.abs() < 1e-18 {
                return Err(NurbsError::InvalidArgument {
                    msg: "on_extract_curve_segment: zero denominator in left insertion".into(),
                });
            }
            let alf = (ul - left) / den;
            let oma = 1.0 - alf;
            // Qw[j] = alf*Qw[j+1] + oma*Qw[j]
            let newp = comb(alf, qw[j + 1], oma, qw[j]);
            qw[j] = newp;
        }
    }

    // ---- Insert the right knot ----
    // lr = spr - p; lk = n - p + mlr
    if n_spr < n_p {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: spr < p".into(),
        });
    }
    let lr = n_spr - n_p;
    let lk = n_hi
        .checked_sub(n_p).ok_or_else(|| NurbsError::InvalidArgument{ msg: "on_extract_curve_segment: bad lk arithmetic".into() })?
        + n_mlr;

    if n_mlr > n_p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: right multiplicity > p+1".into(),
        });
    }

    for i in 1..=(n_p.saturating_sub(n_mlr)) {
        // for j = p-i-mlr down to 0
        let start = n_p
            .checked_sub(i).ok_or_else(|| NurbsError::InvalidArgument{ msg: "on_extract_curve_segment: bad right loop".into() })?
            .checked_sub(n_mlr).ok_or_else(|| NurbsError::InvalidArgument{ msg: "on_extract_curve_segment: bad right loop".into() })?;

        for j in (0..=start).rev() {
            let k = lk + i + j;

            let mut left = up[lr + i + j];
            if left < ul {
                left = ul; // C: if(left LT ul) left=ul
            }

            let den = up[n_spr + j + 1] - left;
            if den.abs() < 1e-18 {
                return Err(NurbsError::InvalidArgument {
                    msg: "on_extract_curve_segment: zero denominator in right insertion".into(),
                });
            }

            let alf = (ur - left) / den;
            let oma = 1.0 - alf;

            // Qw[k] = alf*Qw[k] + oma*Qw[k-1]
            if k == 0 || k >= qw.len() {
                return Err(NurbsError::InvalidArgument {
                    msg: "on_extract_curve_segment: control index out of bounds in right insertion".into(),
                });
            }
            let newp = comb(alf, qw[k], oma, qw[k - 1]);
            qw[k] = newp;
        }
    }

    // ---- Load the knot vector (C 그대로) ----
    // j=-1;
    // for i=0..p: UQ[++j]=ul
    // for i=spl+1..spr-mlr: UQ[++j]=UP[i]
    // for i=0..p: UQ[++j]=ur
    let mut jj: isize = -1;

    for _ in 0..=n_p {
        jj += 1;
        uq[jj as usize] = ul;
    }

    let mid_end = n_spr - n_mlr;
    if n_spl + 1 <= mid_end {
        for i in (n_spl + 1)..=mid_end {
            jj += 1;
            uq[jj as usize] = up[i];
        }
    }

    for _ in 0..=n_p {
        jj += 1;
        uq[jj as usize] = ur;
    }

    if (jj as usize) != m_hi {
        return Err(NurbsError::InvalidArgument {
            msg: "on_extract_curve_segment: knot fill count mismatch".into(),
        });
    }

    // ---- Build output curve ----
    // domain은 from_rational_control_points가 knot 양끝으로 맞춤
    let out = NurbsCurve::from_rational_control_points(
        cur_p.degree,
        qw,
        KnotVector { knots: uq },
    )?;

    Ok(out)
}
```
---
## 구체적 설명
- 아래는 on_extract_curve_segment(… ul, ur …) 알고리즘을  
    직관적으로 이해할 수 있는 단계 설명.

### 1. 원래 NURBS 곡선 구조
- degree: p
- control points: $P_0,P_1,\dots ,P_n$
- knot vector: $U_0,U_1,\dots ,U_m$
- Control points:
```
P0 --- P1 --- P2 --- ... --- Pn
```
- Knot vector (clamped):
```
U0 = ... = U_p      ...      U_{m-p} = ... = U_m
|----|----|----|----|----|----|----|----|----|
  0    1    2         ...              m
```

- 우리가 하고 싶은 것:
- 원래 곡선 C(u) 중에서
    - u ∈ [ul, ur] 구간만 정확하게 잘라서 새로운 NURBS 곡선 C_seg(u)로 만들기


### 2. ul, ur이 knot vector 상에서 어디에 있는지 찾기
- 먼저 N_kntfsm(…, LEFT)로 span과 multiplicity를 구한다:
    - ul → span = spl, multiplicity = mll
    - ur → span = spr, multiplicity = mlr
- 도식:
```
U:
|----|----|----|----|----|----|----|----|----|
  0    1    2   ... spl ... spr ...       m
              ^          ^
             ul         ur
```

- 여기서:
    - spl = ul이 속한 knot 구간 인덱스
    - spr = ur이 속한 knot 구간 인덱스
    - mll, mlr = 이미 그 위치에 존재하는 ul, ur의 중복도

### 3. ul 기준으로 “왼쪽에서 잘라낼 준비” (is, ll)
```rust
is = spl - p
ll = spl - p
```
- 직관적으로:
    - is = ul 이후에 남길 control point들의 시작 인덱스
    - ll = ul 삽입 시 참조할 knot의 시작 인덱스
- 도식:
```
P0   P1   ...  P_is   ...   P_ie   ...   Pn
           ^                 ^
          시작               끝
```

### 4. ur 기준으로 “오른쪽에서 잘라낼 준비” (ie, lr, lk)
```rust
ie = spr - mlr
lr = spr - p
lk = n - p + mlr
```
- ie = ur 이전에 남길 control point들의 끝 인덱스
- lr = ur 삽입 시 참조할 knot의 시작 인덱스
- lk = 오른쪽 삽입 시 사용할 control index 기준점
- 도식:
```
P0   ...   P_is   ...   P_ie   ...   Pn
           ^                 ^
          시작               끝
```

- 결국 우리가 쓸 control point 범위는:
```rust
Q[0]   ...   Q[n_seg]
  =   Pw[is..=ie]
```


### 5. ul 삽입 도해 (왼쪽에서 knot insertion)
- 왼쪽 삽입 루프:
```rust
for i in 1..=(p - mll) {
    for j in 0..=p-i-mll {
        Qw[j] = α * Qw[j+1] + (1-α) * Qw[j];
    }
}
```

- 도식으로 보면:
- 초기 Qw (복사된 부분):
```
Q0   Q1   Q2   ...   Qk
```
- ul을 삽입할수록,  
    왼쪽 끝 쪽 control들이 점점 ul에 맞게 "당겨짐":

- 1차 삽입 후:
```
Q0'  Q1'  Q2'  ...
```
- 2차 삽입 후:
```
Q0'' Q1'' Q2'' ...
```
- 결과적으로:
    - ul에서 시작하는 부분 곡선에 맞는 control net으로 변형됨

- 이건 **곡선을 잘라내는 게 아니라, ul을 knot vector에 실제로 삽입해서 그에 맞는 control net으로 재구성하는 과정”**.

### 6. ur 삽입 도해 (오른쪽에서 knot insertion)
- 오른쪽 삽입 루프:
```rust
for i in 1..=(p - mlr) {
    for j in (0..=p-i-mlr).rev() {
        k = lk + i + j;
        Qw[k] = α * Qw[k] + (1-α) * Qw[k-1];
    }
}
```

- 도식:
```
Q0   Q1   ...   Qk-1   Qk   Qk+1 ...
```
- 오른쪽에서부터 ur을 삽입하면서,
- 우측 끝 control들이 점점 ur에 맞게 "당겨짐":

- 1차 삽입:
```
... Qk-1'  Qk'
```
- 2차 삽입:
```
... Qk-1'' Qk''
```
- 결과적으로:
    - ur에서 끝나는 부분 곡선에 맞는 control net으로 변형됨


- 즉, ul 쪽에서 한 번, ur 쪽에서 한 번
    - 양쪽에서 knot insertion을 해서  
    [ul, ur] 구간에 정확히 맞는 control net을 만든다.

### 7. 최종 control point 범위와 knot vector 도해
- 최종적으로 우리가 얻는 건:
- Control points:
```
Q0, Q1, ..., Q_{n_seg}
  = 변형된 Qw[0..=n_hi] 중에서 실제 [ul, ur] 구간에 해당하는 부분
```

- Knot vector:
```
[ul, ..., ul,  U_mid...,  ur, ..., ur]
  ^      ^      ^         ^      ^
  p+1개   |   중간 knot   |    p+1개
       spl+1..spr-mlr
```

- New knot vector UQ:
```
ul  ul  ul  ul  ...   U[spl+1] ... U[spr-mlr]   ur  ur  ur  ur
|---|---|---|---|      ...           ...        |---|---|---|---|
 0   1   2   3         ...                      m'-3 m'-2 m'-1 m'
```
- New control points Q:
```
Q0 --- Q1 --- Q2 --- ... --- Q_{n_seg}
(이 control net이 정확히 C(u), u ∈ [ul, ur]를 표현)
```

## 8. 한 줄 요약
- 이 알고리즘은  
    - ul, ur을 knot vector에 실제로 삽입하는 것과 동일한 연산을
- 좌/우에서 직접 수행해서,
    - [ul, ur] 구간만 정확히 표현하는 새로운 NURBS 곡선을 만들어내는 과정이다.

```rust
pub fn reparameterize_curve_to_unit_domain(
    curve: &NurbsCurve,
) -> Result<NurbsCurve> {
    let knots = curve.kv.as_slice();
    let ul = *knots.first().ok_or(NurbsError::InvalidArgument{ msg: "empty knots".into() })?;
    let ur = *knots.last().ok_or(NurbsError::InvalidArgument{ msg: "empty knots".into() })?;
    if ul >= ur {
        return Err(NurbsError::InvalidArgument{ msg: "invalid knot domain".into() });
    }

    let scale = ur - ul;
    let mut new_knots = Vec::with_capacity(knots.len());
    for &u in knots {
        new_knots.push((u - ul) / scale);
    }

    NurbsCurve::from_rational_control_points(
        curve.degree,
        curve.ctrl.clone(),
        KnotVector { knots: new_knots },
    )
}
```
```rust
pub fn on_extract_and_reparameterize_to_unit(
    curve: &NurbsCurve,
    ul: Real,
    ur: Real,
) -> Result<NurbsCurve> {
    // 1) 먼저 segment 추출 (N_toocsg 기반)
    let seg = on_extract_curve_segment(curve, ul, ur)?;

    // 2) seg 전체를 [0,1]로 리파라미터
    reparameterize_curve_to_unit_domain(&seg)
}
```
---




