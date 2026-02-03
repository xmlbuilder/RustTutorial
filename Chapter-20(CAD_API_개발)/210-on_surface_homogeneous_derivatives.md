# on_surface_homogeneous_derivatives


## 1. 이 함수가 하는 일 한 줄 요약
- on_surface_homogeneous_derivatives는:
- NURBS 표면 S(u,v)의 **동차 좌표계(homogeneous space)** 에서
```math
\frac{\partial ^{k+l}}{\partial u^k\partial v^l}
```
- S(u,v) 를 $0\leq k\leq udr$, $0\leq l\leq vdr$ 범위로 전부 계산해서  
    sdw[k][l]에 담아 반환하는 함수.

- 여기서 “homogeneous”라는 건:
    - 점을 (x,y,z)가 아니라 (xw,yw,zw,w) 형태의 4D로 다루고
    - 마지막에 필요하면 to_euclidean()으로 (x,y,z)로 나중에 변환하는 방식
- 이 함수는 그 변환을 일부러 하지 않고,  
    **동차 공간에서의 미분 결과** 만 계산해서 넘겨준다.

## 2. 수식 구조부터 정리해보자
- 주석에 이미 핵심이 다 들어있음:
- SDw[k][l] = Σ_i Σ_j BD[k][l][i][j] * Pw[usp-p+i, vsp-q+j]


- 이걸 수식으로 쓰면:
```math
SD_w^{(k,l)}(u,v)=\sum _{i=0}^p\sum _{j=0}^qBD_{i,j}^{(k,l)}(u,v)\cdot P_{(u\_ sp-p+i,\  v\_ sp-q+j)}^w
```
여기서:
- $p=\mathrm{sur.pu}$ : u 방향 차수
- $q=\mathrm{sur.pv}$ : v 방향 차수
- $BD_{i,j}^{(k,l)}(u,v)$ :
```math
\frac{\partial ^k}{\partial u^k}N_i(u)\cdot \frac{\partial ^l}{\partial v^l}N_j(v)
```
- 즉, u, v 방향 B-spline basis의 k, l차 미분의 곱
- $P_{(i,j)}^w$ : 동차 좌표계의 control point (Point4D)
- $u\_ sp,v\_ sp$ : u, v에서의 span index (해당 파라미터가 속한 knot 구간 인덱스)
- 즉:
    - 각 (k, l)에 대해,
    - 주변의 $(p+1)\times (q+1)$ 개 control point를
    - 해당 위치에서의 basis 미분값으로 가중합한 결과가 SDw[k][l]이다.


## 3. 함수 인자 의미 정리
```rust
pub fn on_surface_homogeneous_derivatives(
    sur: &NurbsSurface, // 대상 NURBS 표면
    u: Real,            // 평가할 u 파라미터
    v: Real,            // 평가할 v 파라미터
    ufl: Side,          // u 방향에서 좌/우/중앙 평가 플래그 (knot 경계 처리용)
    vfl: Side,          // v 방향에서 좌/우/중앙 평가 플래그
    mfl_upper_half_only_when_equal: bool, // udr == vdr일 때 상삼각만 계산할지 여부
    udr: usize,         // u 방향 미분 차수 최대값 (0..=udr)
    vdr: usize,         // v 방향 미분 차수 최대값 (0..=vdr)
) -> Result<Vec<Vec<Point4D>>>
```

- 반환값:
    - sdw[k][l]
    - $\frac{\partial ^{k+l}}{\partial u^k\partial v^l}S^w(u,v)$  
        (동차 좌표계에서의 4D 점)

## 4. 전처리: 유효성 검사와 파라미터 범위 체크
```rust
if !sur.is_valid() { ... }
if sur.ctrl.is_empty() || sur.ku.knots.is_empty() || sur.kv.knots.is_empty() { ... }
```
- 표면 정의가 정상인지
- control net, knot vector가 비어 있지 않은지
```rust
let u_min = sur.ku.knots[0];
let u_max = *sur.ku.knots.last().unwrap();
let v_min = sur.kv.knots[0];
let v_max = *sur.kv.knots.last().unwrap();

on_check_param_range(u, u_min, u_max)?;
on_check_param_range(v, v_min, v_max)?;
```

- u,v가 knot 범위 안에 있는지 확인
- C 코드의 E_parval 역할

## 5. basis 함수와 그 미분값 테이블 계산
```rust
let p = sur.pu as usize;
let q = sur.pv as usize;

let (bd, usp, vsp) = on_knot_nonrational_bivariate_basis_ders(
    &sur.ku,
    &sur.kv,
    sur.pu,
    sur.pv,
    u,
    v,
    ufl,
    vfl,
    mfl_upper_half_only_when_equal,
    udr,
    vdr,
)?;
```

- 여기서 on_knot_nonrational_bivariate_basis_ders는:
    - u 방향 B-spline basis $N_i(u)$ 와 그 미분들
    - v 방향 B-spline basis $M_j(v)$ 와 그 미분들
    - 그리고 그 곱 $BD[k][l][i][j]=N_i^{(k)}(u)\cdot M_j^{(l)}(v)$
    - u, v에서의 span index usp, vsp
- 를 한 번에 계산해서 넘겨주는 루틴이라고 보면 된다.
- 이후:
```rust
let usp = usp as usize;
let vsp = vsp as usize;
```
- usp, vsp는 현재 (u, v)가 속한 knot span의 인덱스
- control point 인덱스를 잡을 때 기준점 역할

## 6. control net 인덱싱 준비
```rust
let nu = sur.nu;
let nv = sur.nv;
if nu == 0 || nv == 0 { ... }

let mut sdw = vec![vec![Point4D::zero(); vdr + 1]; udr + 1];
```

- nu, nv: control net의 u, v 방향 개수
- sdw[k][l]를 담을 2D 배열 생성
```rust
let upper_half = mfl_upper_half_only_when_equal && (udr == vdr);
```

- udr == vdr이고, 플래그가 켜져 있으면
    - (k, l) 중에서 상삼각(예: k + l <= udr)만 계산하는 최적화 모드

## 7. (k, l) 루프: 각 차수의 혼합 미분 계산
```rust
for k in 0..=udr {
    let lmax = if upper_half {
        std::cmp::min(vdr, udr.saturating_sub(k))
    } else {
        vdr
    };

    for l in 0..=lmax {
        let mut acc = Point4D::zero();
        ...
        sdw[k][l] = acc;
    }
}
```
- k: u 방향 미분 차수
- l: v 방향 미분 차수
- upper_half가 true면, 예를 들어 udr = vdr = 2일 때:
    - (0,0), (0,1), (0,2)
    - (1,0), (1,1)
    - (2,0) 이런 식으로 상삼각만 계산 (대칭성 활용 가능할 때)
- 각 (k, l)에 대해:
- acc는 $\sum _i\sum _jBD[k][l][i][j]\cdot P^w$ 를 누적하는 누산기

## 8. (i, j) 루프: basis × control point 합산
```rust
for i in 0..=p {
    let iu = usp
        .checked_sub(p)
        .and_then(|t| t.checked_add(i))
        .ok_or_else(|| NurbsError::InternalError { ... })?;

    for j in 0..=q {
        let iv = vsp
            .checked_sub(q)
            .and_then(|t| t.checked_add(j))
            .ok_or_else(|| NurbsError::InternalError { ... })?;

        if iu >= nu || iv >= nv {
            return Err(NurbsError::InternalError { ... });
        }

        let a = bd[k][l][i][j];
        let cp = &sur.ctrl[on_idx_row_major(nu, iu, iv)];
        on_add_scaled_point4d(&mut acc, a, cp);
    }
}
```

- 여기서 하는 일은 정확히 이것:
- control point 인덱스 계산
- u 방향:
```math
iu=(u\_ sp-p)+i
```
- v 방향:
```math
iv=(v\_ sp-q)+j
```
- 이건 **현재 span 주변의 (p+1)개, (q+1)개 control point** 를 잡는 전형적인 B-spline 인덱싱 방식.
- checked_sub, checked_add를 쓰는 이유는:
- underflow/overflow를 방지해서 인덱스 버그를 Rust에서 안전하게 막기 위함
- 인덱스 범위 검사
```rust
if iu >= nu || iv >= nv {
    return Err(...);
}
```
- control net 범위를 벗어나면 내부 오류로 처리
- basis 값과 control point를 곱해서 누적
```rust
let a = bd[k][l][i][j];
let cp = &sur.ctrl[on_idx_row_major(nu, iu, iv)];
on_add_scaled_point4d(&mut acc, a, cp);
```
- 수식으로 쓰면:
- 여기서 on_add_scaled_point4d(acc, a, cp)는:
```math
acc\leftarrow acc+a\cdot cp
```
- 을 수행하는 헬퍼 함수라고 보면 된다.
- 루프가 끝나면:
```rust
sdw[k][l] = acc;
```
- 즉:
```math
SD_w^{(k,l)}(u,v)=acc
```

## 9. homogeneous space에서 끝내는 이유
- 주석 명시:
```
/// - 이 루틴은 "homogeneous space" 결과이므로 to_euclidean() 호출하지 않는다.
/// - non-rational / rational 모두 동일한 방식으로 homogeneous 합산 가능.
///   (네 규약상 w=1.0 고정이므로 사실상 non-rational처럼 동작)
```
- rational NURBS든 non-rational B-spline이든  
    동차 좌표계에서는 동일한 방식으로 합산할 수 있다.
- rational의 경우:
    - 나중에 (X,Y,Z,W)에서 (X/W,Y/W,Z/W)로 나누면 되고
- non-rational의 경우:
    - 네 규약상 w=1.0이므로
- 사실상 그냥 3D B-spline과 동일하게 동작
- 이 함수는: **동차 좌표계에서의 미분 결과** 만 책임지고,  
    **유클리드 공간으로의 변환(to_euclidean)** 은  
    호출자가 필요할 때 따로 하도록 설계된 것.

## 10. 전체 흐름을 한 번에 요약하면
- 표면 유효성, 파라미터 범위 체크
    - u, v 방향 차수 p,q 가져오기
- on_knot_nonrational_bivariate_basis_ders로:
    - basis 미분값 테이블 bd[k][l][i][j]
    - span index usp, vsp 를 얻는다.
    - sdw[k][l] 배열을 (udr+1) × (vdr+1) 크기로 준비
    - 각 (k, l)에 대해:
    - 주변 (p+1) × (q+1) control point를 돌면서
    - acc += bd[k][l][i][j] * Pw[iu, iv]
    - 최종 acc를 sdw[k][l]에 저장
    - 최종적으로 sdw를 반환
        - 동차 좌표계에서의 모든 혼합 미분 결과
---

## on_knot_nonrational_bivariate_basis_ders
- on_surface_homogeneous_derivatives가 표면 레벨의 합산 루틴,
- on_knot_nonrational_bivariate_basis_ders는 그 밑에서 돌아가는 순수 basis × 미분 테이블 생성기.
- 이 함수 하나를 정확히 이해하면:
    - u, v 방향 B-spline basis가 어떻게 미분되는지
    - 1D 미분이 어떻게 2D로 확장되는지
    - bd[k][l][i][j]가 정확히 무엇을 의미하는지가 전부 명확해진다.

### 1. 이 함수가 하는 일 한 줄 요약
```math
bd[k][l][i][j] = (d^k/du^k N_{ku-p+i,p}(u)) * (d^l/dv^l N_{kv-q+j,q}(v))
```
- where i=0..p, j=0..q
- usp, vsp: knot span indices


- 수식으로 쓰면:
```math
BD_{i,j}^{(k,l)}(u,v)=\frac{\partial ^k}{\partial u^k}N_i^{(p)}(u)\cdot \frac{\partial ^l}{\partial v^l}M_j^{(q)}(v)
```
- 여기서:
- $N_i^{(p)}(u)$ : u 방향 B-spline basis (차수 p)
- $M_j^{(q)}(v)$ : v 방향 B-spline basis (차수 q)
- k=0..udr, l=0..vdr
- i=0..p, j=0..q
- usp, vsp: u, v에서의 span index
- 즉:
    - 이 함수는 u, v 방향 1D basis 미분 테이블을 먼저 만들고,
    - 그걸 곱해서 **2D bivariate basis 미분 테이블 bd[k][l][i][j]** 를 만든다.

- 그리고 이 bd가 바로 앞에서 봤던
- on_surface_homogeneous_derivatives의 핵심 입력이 된다.

## 2. 함수 시그니처 의미
```rust
pub fn on_knot_nonrational_bivariate_basis_ders(
    ku: &KnotVector,   // u 방향 knot vector
    kv: &KnotVector,   // v 방향 knot vector
    p: Degree,         // u 방향 차수
    q: Degree,         // v 방향 차수
    u: Param,          // 평가할 u 파라미터
    v: Param,          // 평가할 v 파라미터
    ufl: Side,         // u 방향 좌/우/중앙 평가 플래그
    vfl: Side,         // v 방향 좌/우/중앙 평가 플래그
    mfl_upper_half_only_when_equal: bool, // udr == vdr일 때 상삼각만 계산할지
    udr: usize,        // u 방향 미분 차수 최대값
    vdr: usize,        // v 방향 미분 차수 최대값
) -> Result<(Vec<Vec<Vec<Vec<Real>>>>, Index, Index)>
```

- 반환값:
- bd[k][l][i][j]
```math
\frac{\partial ^k}{\partial u^k}N_i^{(p)}(u)\cdot \frac{\partial ^l}{\partial v^l}M_j^{(q)}(v)
```
- usp, vsp
    - u, v에서의 span index

## 3. 초반 전처리: 차수, knot 길이, 파라미터 범위
```rust
let p = p as usize;
let q = q as usize;

if ku.knots.is_empty() || kv.knots.is_empty() {
    return Err(NurbsError::InvalidArgument { ... });
}
let mu = ku.knots.len() - 1;
let mv = kv.knots.len() - 1;
```

- mu, mv는 C 코드에서 자주 쓰는 “마지막 인덱스” 개념  
    (보통 U[0..m]에서 m = mu)
```rust
if u < ku.knots[0] || u > ku.knots[mu] || v < kv.knots[0] || v > kv.knots[mv] {
    return Err(NurbsError::InvalidArgument { ... });
}
```

- $u\in [U_0,U_m], v\in [V_0,V_m]$ 범위 체크

## 4. span index 계산
```rust
let usp = on_find_span_left_right(ku, p as Degree, u, ufl)?;
let vsp = on_find_span_left_right(kv, q as Degree, v, vfl)?;
```
- 주어진 u, v에 대해:
    - 어느 knot 구간 $[U_i,U_{i+1})$ 에 속하는지
- 경계에서 좌/우/중앙을 어떻게 처리할지 (ufl, vfl)
- 이 usp, vsp는 이후:
    - 1D basis 계산에서 **활성 basis index** 를 정하는 기준
    - 나중에 control point 인덱싱에서 (usp - p + i) 같은 형태로 쓰임

## 5. 1D basis + 미분 테이블 계산
```rust
let du = on_all_non_vanishing_basis_and_ders_1d(ku, p, u, usp, udr)?;
let dv = on_all_non_vanishing_basis_and_ders_1d(kv, q, v, vsp, vdr)?;
```

- 여기서 du, dv의 의미는 보통 이렇게 된다:
- du[k][i]
```math
\frac{\partial ^k}{\partial u^k}N_i^{(p)}(u), k=0..udr, i=0..p
```
- dv[l][j]
```math
\frac{\partial ^l}{\partial v^l}M_j^{(q)}(v), l=0..vdr, j=0..q
```
- 즉:
    - u 방향에서 (p+1)개의 non-zero basis와 그 미분들
    - v 방향에서 (q+1)개의 non-zero basis와 그 미분들을
- 각각 1D로 계산해둔 테이블.

이제 이 둘을 곱해서 2D bivariate basis를 만들 차례다.

## 6. 4D 배열 bd 할당
```rust
let mut bd = vec![
    vec![vec![vec![0.0 as Real; q + 1]; p + 1]; vdr + 1];
    udr + 1
];
```

- 구조는:
    - bd[k][l][i][j]
- 크기:
    - k: 0..=udr → udr + 1
    - l: 0..=vdr → vdr + 1
    - i: 0..=p   → p + 1
    - j: 0..=q   → q + 1
- 즉:
```math
BD\in \mathbb{R^{\mathnormal{(udr+1)\times (vdr+1)\times (p+1)\times (q+1)}}}
```

## 7. 상삼각 최적화 플래그
```rust
let jump_upper = mfl_upper_half_only_when_equal && (udr == vdr);
```
- udr == vdr이고, 플래그가 켜져 있으면:
- (k, l) 중에서 상삼각 영역만 계산
- 예: udr = vdr = 2일 때
    - (0,0), (0,1), (0,2)
    - (1,0), (1,1)
    - (2,0)
    - 이런 식으로 k + l <= udr 범위만 채우는 효과
- 이건 나중에 표면 미분에서 대칭성을 활용할 수 있을 때 계산량을 줄이기 위한 최적화.

## 8. 핵심 루프: 1D 미분의 곱으로 2D 테이블 채우기
```rust
for i in 0..=p {
    for j in 0..=q {
        for k in 0..=udr {
            if jump_upper {
                // l <= udr-k
                let lmax = (udr - k).min(vdr);
                for l in 0..=lmax {
                    bd[k][l][i][j] = du[k][i] * dv[l][j];
                }
            } else {
                for l in 0..=vdr {
                    bd[k][l][i][j] = du[k][i] * dv[l][j];
                }
            }
        }
    }
}
```
- 이 부분이 수식과 1:1 대응되는 핵심이다.
- 수식으로 쓰면:
```math
BD_{i,j}^{(k,l)}(u,v)=du[k][i]\cdot dv[l][j]=\left( \frac{\partial ^k}{\partial u^k}N_i^{(p)}(u)\right) \cdot \left( \frac{\partial ^l}{\partial v^l}M_j^{(q)}(v)\right)
``` 
- 즉:
- 1D basis 미분 테이블 du, dv를     
    단순 곱해서 2D bivariate basis 미분 테이블 bd를 만든다.

- jump_upper가 true일 때
```rust
let lmax = (udr - k).min(vdr);
for l in 0..=lmax {
    bd[k][l][i][j] = du[k][i] * dv[l][j];
}
```

- l <= udr - k
    - 즉, k + l <= udr
- udr == vdr일 때 상삼각 영역만 채우는 효과
- 이건 나중에:
- $\frac{\partial ^{k+l}}{\partial u^k\partial v^l}$ 에서  
    어떤 대칭성이나 중복을 활용할 수 있을 때 계산량을 줄이기 위한 옵션으로 보인다.

## 9. 반환
```rust
Ok((bd, usp, vsp))
```
- bd: bivariate basis 미분 테이블
- usp, vsp: span index
- 이제 이 결과는 바로:
```rust
let (bd, usp, vsp) = on_knot_nonrational_bivariate_basis_ders(...)?;
```
- 로 받아서,
- 앞에서 봤던 on_surface_homogeneous_derivatives에서:
```rust
let a = bd[k][l][i][j];
let cp = &sur.ctrl[on_idx_row_major(nu, iu, iv)];
on_add_scaled_point4d(&mut acc, a, cp);
```

- 이렇게 basis × control point 합산에 사용된다.



## 10. 전체 흐름을 다시 한 번 연결해보면
- on_knot_nonrational_bivariate_basis_ders
    - u, v에서의 span index usp, vsp 계산
    - u 방향 1D basis 미분 du[k][i]
    - v 방향 1D basis 미분 dv[l][j]
    - 이 둘을 곱해서 bd[k][l][i][j] 생성
- on_surface_homogeneous_derivatives
    - 각 (k, l)에 대해:
    - 주변 (p+1) × (q+1) control point를 돌면서
    acc += bd[k][l][i][j] * Pw[iu, iv]
    - 최종 acc를 sdw[k][l]에 저장
- 이렇게 해서:
- 표면의 모든 혼합 미분을  
    동차 좌표계에서 정확하게 계산하는 전체 파이프라인이 완성된다.
---
## 소스 코드
```rust
/// 1D helper: compute all non-vanishing basis functions and derivatives up to `d`
/// for span `i` (where u in [U[i],U[i+1]] depending on side).
/// Output DU[k][j] where:
/// - j=0..p corresponds to basis N_{i-p+j,p}(u)
/// - k=0..d derivatives
pub fn on_all_non_vanishing_basis_and_ders_1d(
    kv: &KnotVector,
    p: usize,
    u: Real,
    span: Index,
    d: usize,
) -> Result<Vec<Vec<Real>>> {
    let uvec = &kv.knots;
    let m = uvec.len() - 1;

    if p == 0 {
        // degree 0: only one basis = 1, all derivatives 0
        let mut out = vec![vec![0.0; 1]; d + 1];
        out[0][0] = 1.0;
        return Ok(out);
    }

    // span must be valid to access U[span + j]
    let i = span as usize;
    if i + p + 1 > m {
        return Err(NurbsError::InvalidArgument {
            msg: format!("basis_ders_1d: span out of range i={} p={} m={}", i, p, m),
        });
    }

    let dmax = d.min(p); // C: mder = MIN(p,udr)

    // ndu is (p+1)x(p+1)
    let mut ndu = vec![vec![0.0 as Real; p + 1]; p + 1];
    let mut left = vec![0.0 as Real; p + 1];
    let mut right = vec![0.0 as Real; p + 1];

    // a is 2 x (p+1)
    let mut a = vec![vec![0.0 as Real; p + 1]; 2];

    // DU is (d+1) x (p+1)
    let mut du = vec![vec![0.0 as Real; p + 1]; d + 1];

    // Basis functions (C: ndu)
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = u - uvec[i + 1 - j];
        right[j] = uvec[i + j] - u;

        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let denom = ndu[j][r];
            if denom.abs() < 1e-18 {
                return Err(NurbsError::NumericError {
                    msg: format!("basis_ders_1d: ndu denom ~0 at j={} r={}", j, r),
                });
            }
            let temp = ndu[r][j - 1] / denom;
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }

    // Load basis
    for j in 0..=p {
        du[0][j] = ndu[j][p];
    }

    // Derivatives
    for r in 0..=p {
        let mut s1 = 0usize;
        let mut s2 = 1usize;
        a[0][0] = 1.0;

        for k in 1..=dmax {
            let mut dval = 0.0;
            let rk = r as isize - k as isize;
            let pk = p as isize - k as isize;

            if r >= k {
                let denom = ndu[(pk + 1) as usize][rk as usize];
                if denom.abs() < 1e-18 {
                    return Err(NurbsError::NumericError {
                        msg: format!("basis_ders_1d: denom ~0 (r>=k) r={} k={}", r, k),
                    });
                }
                a[s2][0] = a[s1][0] / denom;
                dval = a[s2][0] * ndu[rk as usize][pk as usize];
            }

            let j1 = if rk >= -1 { 1 } else { (-rk) as usize };
            let j2 = if (r as isize - 1) <= pk {
                k - 1
            } else {
                (p - r) as usize
            };

            for j in j1..=j2 {
                let denom = ndu[(pk + 1) as usize][(rk + j as isize) as usize];
                if denom.abs() < 1e-18 {
                    return Err(NurbsError::NumericError {
                        msg: format!("basis_ders_1d: denom ~0 mid r={} k={} j={}", r, k, j),
                    });
                }
                a[s2][j] = (a[s1][j] - a[s1][j - 1]) / denom;
                dval += a[s2][j] * ndu[(rk + j as isize) as usize][pk as usize];
            }

            if r as isize <= pk {
                let denom = ndu[(pk + 1) as usize][r];
                if denom.abs() < 1e-18 {
                    return Err(NurbsError::NumericError {
                        msg: format!("basis_ders_1d: denom ~0 tail r={} k={}", r, k),
                    });
                }
                a[s2][k] = -a[s1][k - 1] / denom;
                dval += a[s2][k] * ndu[r][pk as usize];
            }

            du[k][r] = dval;
            std::mem::swap(&mut s1, &mut s2);
        }
    }

    // Multiply through by correct factors (C block)
    let mut rfac = p as Real;
    for k in 1..=dmax {
        for j in 0..=p {
            du[k][j] *= rfac;
        }
        rfac *= (p - k) as Real;
    }

    // For k > p, derivatives are 0 (already initialized)
    Ok(du)
}
```
---
