# on_eval_surface_grid_points
- **표면 위의 격자점(grid)을 효율적으로 평가하는 2D 곱셈 구조** 를 그대로 구현.

## 1. 함수의 목적
- on_eval_surface_grid_points는:
    - NURBS 표면 S(u,v) 위에서
    - 주어진 파라미터 배열 $u[0..n],v[0..m]$ 에 대해
    - 모든 격자점 $S(u_k,v_l) (k=0..n, l=0..m)$ 를 한 번에 계산하는 함수다.
- 즉:
```math
\mathrm{out}[k][l]=S(u_k,v_l)
```
- 이걸 직접 이중 루프 + evaluator 호출로 하면 비효율적이라,  
    basis precompute + 2단계 곱셈으로 최적화한 버전이다.

## 2. NURBS 표면 수식 복습
- NURBS 표면의 homogeneous 표현은:
```math
S(u,v)=\frac{\sum _{i=0}^{n_u}\sum _{j=0}^{m_v}N_i^p(u)\, M_j^q(v)\, w_{ij}\, P_{ij}}{\sum _{i=0}^{n_u}\sum _{j=0}^{m_v}N_i^p(u)\, M_j^q(v)\, w_{ij}}
```
- homogeneous 좌표 $P_{ij}^w=(xw,yw,zw,w)$ 로 쓰면:
```math
S^w(u,v)=\sum _{i=0}^{n_u}\sum _{j=0}^{m_v}N_i^p(u)\, M_j^q(v)\, P_{ij}^w
```
- 마지막에 Euclidean 변환:
```math
S(u,v)=\left( \frac{x_w}{w},\frac{y_w}{w},\frac{z_w}{w}\right) 
```
## 3. 격자 평가의 구조
- 우리가 하고 싶은 건:
```math
S_{k,l}=S(u_k,v_l)
```
- 즉:
```math
S_{k,l}^w=\sum _{i=0}^{n_u}\sum _{j=0}^{m_v}N_i^p(u_k)\, M_j^q(v_l)\, P_{ij}^w
```
- 여기서 핵심은:
- $u_k$ 마다 $N_i^p(u_k)$ 는 p+1개만 non-zero
- $v_l$ 마다 $M_j^q(v_l)$ 도 q+1개만 non-zero
- 그래서 실제 합은:
```math
S_{k,l}^w=\sum _{i=0}^p\sum _{j=0}^qN_{i'}^p(u_k)\, M_{j'}^q(v_l)\, P_{i'j'}^w
```
여기서 $i'=\mathrm{usp}[k]-p+i, j'=\mathrm{vsp}[l]-q+j$.

## 4. basis precompute: on_all_basis_func_at_params
- 코드에서:
```rust
let (nu_basis, usp) = on_all_basis_func_at_params(&sur.ku, sur.pu, u, ufl)?;
let (nv_basis, vsp) = on_all_basis_func_at_params(&sur.kv, sur.pv, v, vfl)?;
```

- 이게 하는 일:
    - 각 u_k에 대해:
    - span index: usp[k]
    - non-zero basis: nu_basis[k][0..=p]
        - $N_i^p(u_k), i=0..p$ (실제 index는 usp[k]-p + i)
- 각 v_l에 대해:
    - span index: vsp[l]
    - non-zero basis: nv_basis[l][0..=q]
        - $M_j^q(v_l), j=0..q$ (실제 index는 vsp[l]-q + j)
- 즉, 모든 u, v에 대한 basis를 한 번에 미리 계산해둔다.

## 5. 두 가지 곱셈 순서
- 표면 평가의 이중 합:
```math
S_{k,l}^w=\sum _i\sum _jN_i^p(u_k)\, M_j^q(v_l)\, P_{ij}^w
```
- 이걸 두 가지 순서로 쪼갤 수 있다.
### 5.1. Branch A: 먼저 v 방향으로 축소, 그 다음 u
- v 방향 축소:
```math
T_i(v_l)=\sum _{j=0}^qM_j^q(v_l)\, P_{i,\, s_0+j}^w
```
- u 방향 축소:
```math
S_{k,l}^w=\sum _{i=0}^pN_i^p(u_k)\, T_{r_0+i}(v_l)
```
- 이게 코드의 이 부분:
```rust
// Tw[i] = sum_{j=0..q} NV[l][j] * Pw[i][s0+j]
for i in i0..=i1 {
    acc = 0
    for j in 0..=q {
        a = nv_basis[l][j];
        cp = Pw[i][s0+j];
        acc += a * cp;
    }
    tw[i] = acc;
}
```
```rust
// Sw = sum_{i=0..p} NU[k][i]*Tw[r0+i]
for k in 0..=n {
    sw = 0
    for i in 0..=p {
        a = nu_basis[k][i];
        cp = tw[r0+i];
        sw += a * cp;
    }
    out[k][l] = euclid(sw);
}
```

### 5.2. Branch B: 먼저 u 방향으로 축소, 그 다음 v
- u 방향 축소:
```math
T_j(u_k)=\sum _{i=0}^pN_i^p(u_k)\, P_{r_0+i,\, j}^w
```
- v 방향 축소:
```math
S_{k,l}^w=\sum _{j=0}^qM_j^q(v_l)\, T_{s_0+j}(u_k)
```
- 이게 코드의 이 부분:
```rust
// Tw[j] = sum_{i=0..p} NU[k][i]*Pw[r0+i][j]
for j in j0..=j1 {
    acc = 0
    for i in 0..=p {
        a = nu_basis[k][i];
        cp = Pw[r0+i][j];
        acc += a * cp;
    }
    tw[j] = acc;
}
```
```rust
// Sw = sum_{j=0..q} NV[l][j]*Tw[s0+j]
for l in 0..=m {
    sw = 0
    for j in 0..=q {
        a = nv_basis[l][j];
        cp = tw[s0+j];
        sw += a * cp;
    }
    out[k][l] = euclid(sw);
}
```

## 6. 비용 비교 수식 (C의 r, s)
- 코드:
```rust
let r_cost = m * (nu*q + n*p);
let s_cost = n * (mv*p + m*q);
```

- 이건 대략적인 연산량 추정이다.
    - Branch A (v 먼저, 그 다음 u):
- v 방향 축소:
    각 l에 대해, i 범위 ~nu, j 범위 ~q
    - 대략 $m\cdot (nu\cdot q)$
- u 방향 축소:
    - 각 l에 대해, k 범위 ~n, i 범위 ~p
    → 대략 $m\cdot (n\cdot p)$
- 합치면:
```math
r=m\cdot (nu\cdot q+n\cdot p)
```
- Branch B (u 먼저, 그 다음 v):
- u 방향 축소:
- 각 k에 대해, j 범위 ~mv, i 범위 ~p
    - $n\cdot (mv\cdot p)$
- v 방향 축소:
    - 각 k에 대해, l 범위 ~m, j 범위 ~q
    - $n\cdot (m\cdot q)$
- 합치면:
```math
s=n\cdot (mv\cdot p+m\cdot q)
```
- 그래서:
```rust
if r_cost < s_cost { branch A } else { branch B }
```
- 더 싼 곱셈 순서를 자동으로 선택.

## 7. 인덱스와 span 처리
- u 방향
    - $usp[k] = u_k$ 가 속한 span index
    - non-zero basis는 i=0..p에 대해
    - 실제 control index: $i' = usp[k] - p + i$
- 코드:
```rust
let r0 = usp[k].checked_sub(p)?; // r0 = usp[k] - p
// Tw[r0+i] 또는 Pw[r0+i][...]
```

- v 방향
    - $vsp[l] = v_l$ 가 속한 span index
    - non-zero basis는 j=0..q에 대해
    - 실제 control index: $j' = vsp[l] - q + j$
- 코드:
```rust
let s0 = vsp[l].checked_sub(q)?; // s0 = vsp[l] - q
// Tw[s0+j] 또는 Pw[...][s0+j]
```


## 8. 최종 Euclidean 변환
- 각 Sw는 Point4D (xw, yw, zw, w)이고:
```rust
out[k][l] = on_euclid(sw);
```

- 이건:
```math
(x,y,z)=\left( \frac{x_w}{w},\frac{y_w}{w},\frac{z_w}{w}\right)
``` 
- 을 수행하는 함수다.

## 9. 이 함수가 좋은 이유
- 모든 $(u_k, v_l)$ 를 한 번에 처리
    → evaluator를 n×m번 호출하는 것보다 훨씬 효율적
- basis를 미리 계산
    - 같은 $u_k, v_l$ 에 대해 basis를 재사용
- 두 가지 곱셈 순서 중 더 싼 쪽을 자동 선택
    - 큰 그리드에서 성능 차이가 꽤 난다


## 10. 한 줄 요약
- 이 함수는:
- NURBS 표면의 격자점 $S(u_k,v_l)$ 를  
    basis precompute + 2단계 곱셈 + 비용 최소화 순서 선택으로 효율적으로 계산
- 수식으로는:
```math
S_{k,l}^w=\sum _{i=0}^p\sum _{j=0}^qN_i^p(u_k)\, M_j^q(v_l)\, P_{i'j'}^w
```
- 를 v→u 또는 u→v 순서로 축소해서 계산하는 구조다.

---

## 1. 우리가 실제로 하고 싶은 계산
- 표면의 homogeneous 점:
```math
S^w(u_k,v_l)=\sum _i\sum _jN_i^p(u_k)\, M_j^q(v_l)\, P_{ij}^w
```
- 여기서:
    - $N_i^p(u_k)$: u 방향 B-spline basis
    - $M_j^q(v_l)$: v 방향 B-spline basis
    - $P_{ij}^w$: control net (homogeneous)
- 즉, i, j에 대한 이중 합.
- 코드로 쓰면 이런 느낌:
```rust
Sw = 0;
for i in ... {
    for j in ... {
        Sw += N_i(u_k) * M_j(v_l) * Pw[i][j];
    }
}
```
- 근데 이걸 그대로 하면 $(u_k, v_l)$ 마다 이중 루프를 돌려야 해서 너무 비쌈.
- 즉, 이중 합을 두 번의 1중 합으로 나누어 계산하는 구조.

## 2. 핵심 아이디어: “한 방향을 먼저 다 처리해놓고, 나머지 방향으로 한 번 더”
- 이중 합:
```math
\sum _i\sum _jN_i(u_k)\, M_j(v_l)\, P_{ij}
```
- 을 두 가지 방식으로 쪼갤 수 있dma.

### 방식 A: 먼저 v 방향으로 축소, 그 다음 u (v→u)
#### 1단계: v 방향으로 먼저 합치기
```math
T_i(v_l)=\sum _jM_j(v_l)\, P_{ij}
```
- 이건 각 i에 대해, v 방향 basis로 control point들을 한 번 모아서 $T_i$ 를 만든다 는 뜻.
#### 2단계: u 방향으로 $T_i$ 들을 다시 합치기
```math
S^w(u_k,v_l)=\sum _iN_i(u_k)\, T_i(v_l)
```
- 즉:
    - 먼저 v 방향으로 control net을 **압축** 해서 $T_i$ 를 만들고
    - 그 $T_i$ 들을 u 방향 basis로 다시 합쳐서 최종 Sw를 만든다
- 코드로 쓰면:
```rust
// 1단계: v 방향 축소 → T[i]
for i in ... {
    T[i] = 0;
    for j in ... {
        T[i] += M_j(v_l) * Pw[i][j];
    }
}
```
```rust
// 2단계: u 방향 축소 → Sw
Sw = 0;
for i in ... {
    Sw += N_i(u_k) * T[i];
}
```

- 이게 내가 말한 **v→u 순서로 축소”**.

### 방식 B: 먼저 u 방향으로 축소, 그 다음 v (u→v)
#### 1단계: u 방향으로 먼저 합치기
```math
T_j(u_k)=\sum _iN_i(u_k)\, P_{ij}
```
#### 2단계: v 방향으로 T_j들을 다시 합치기
```math
S^w(u_k,v_l)=\sum _jM_j(v_l)\, T_j(u_k)
```
- 코드로 쓰면:
```rust
// 1단계: u 방향 축소 → T[j]
for j in ... {
    T[j] = 0;
    for i in ... {
        T[j] += N_i(u_k) * Pw[i][j];
    }
}
```
```rust
// 2단계: v 방향 축소 → Sw
Sw = 0;
for j in ... {
    Sw += M_j(v_l) * T[j];
}
```

- 이게 **u→v 순서로 축소**.

## 3. 왜 두 가지 순서가 생기냐?
- 이유는 간단:
    - 이중 합은 어느 방향을 먼저 더해도 결과가 같다
    - 하지만 계산 비용은 다르다
- 표면 전체 그리드 $(u_k,v_l)$ 에 대해 계산할 때:
- u 방향으로 먼저 축소하면
- u 관련 계산을 재사용하기 좋고
- v 방향으로 먼저 축소하면
- v 관련 계산을 재사용하기 좋다

```rust
r = m*(nu*q + n*p);
s = n*(mv*p + m*q);
if (r < s) { v→u } else { u→v }
```

- 이렇게 어느 방향을 먼저 축소하는 게 더 싼지를 비교해서 선택.
```rust
let r_cost = m * (nu*q + n*p);
let s_cost = n * (mv*p + m*q);

if r_cost < s_cost {
    // branch A: v→u
} else {
    // branch B: u→v
}
```

## 4. 네 코드에 대입해서 다시 보자
- Branch A (v→u):
```rust
// 1단계: v 방향 축소 → Tw[i]
for i in i0..=i1 {
    acc = 0;
    for j in 0..=q {
        a = nv_basis[l][j];      // M_j(v_l)
        cp = Pw[i][s0+j];        // P_{i, j'}
        acc += a * cp;
    }
    tw[i] = acc;                 // T_i(v_l)
}
```
```rust
// 2단계: u 방향 축소 → Sw
for k in 0..=n {
    sw = 0;
    for i in 0..=p {
        a = nu_basis[k][i];      // N_i(u_k)
        cp = tw[r0+i];           // T_{i'}(v_l)
        sw += a * cp;
    }
    out[k][l] = euclid(sw);
}
```
- 먼저 v 방향으로 control net을 압축해서 Tw[i] 만들고,  
    그걸 u 방향 basis로 다시 합쳐서 Sw를 만드는 구조.

- Branch B (u→v):
```rust
// 1단계: u 방향 축소 → Tw[j]
for j in j0..=j1 {
    acc = 0;
    for i in 0..=p {
        a = nu_basis[k][i];      // N_i(u_k)
        cp = Pw[r0+i][j];        // P_{i', j}
        acc += a * cp;
    }
    tw[j] = acc;                 // T_j(u_k)
}
```
```rust
// 2단계: v 방향 축소 → Sw
for l in 0..=m {
    sw = 0;
    for j in 0..=q {
        a = nv_basis[l][j];      // M_j(v_l)
        cp = tw[s0+j];           // T_{j'}(u_k)
        sw += a * cp;
    }
    out[k][l] = euclid(sw);
}
```

- 먼저 u 방향으로 control net을 압축해서 Tw[j] 만들고,  
    그걸 v 방향 basis로 다시 합쳐서 Sw를 만드는 구조.

## 5. 아주 작은 예로 직관 잡기
- control net이 2×2라고 가정 (i=0,1 / j=0,1).
- 표면 점은:
```math
S^w(u,v)=\sum _{i=0}^1\sum _{j=0}^1N_i(u)\, M_j(v)\, P_{ij}
```
- v→u:
### 1단계:
```math
T_0(v)=M_0(v)P_{00}+M_1(v)P_{01}
```
```math
T_1(v)=M_0(v)P_{10}+M_1(v)P_{11}
```
### 2단계:
```math
S^w(u,v)=N_0(u)T_0(v)+N_1(u)T_1(v)
```
- u→v:
### 1단계:
```math
T_0(u)=N_0(u)P_{00}+N_1(u)P_{10}
```
```math
T_1(u)=N_0(u)P_{01}+N_1(u)P_{11}
```
### 2단계:
```math
S^w(u,v)=M_0(v)T_0(u)+M_1(v)T_1(u)
```
- 결과는 똑같지만,  
    어떤 방향을 먼저 계산하느냐에 따라 중간 결과(T)의 재사용 패턴과 연산량이 달라진다.

## 6. 한 줄로 정리하면
- **v→u로 축소한다** 는 말은,  
    먼저 v 방향 basis로 control net을 압축해서 중간 결과 Tw를 만들고,  
    그 다음 u 방향 basis로 Tw를 다시 합쳐서 최종 표면 점을 만드는 구조라는 뜻이다.

- **u→v로 축소한다** 는 말은 그 반대 순서로 같은 일을 한다는 뜻.

---
## 소스 코드
```rust
/// Compute a grid of points on a NURBS surface at parameter arrays u[0..=n], v[0..=m].
///
/// - u, v must be non-decreasing arrays.
/// - S[k][l] corresponds to (u[k], v[l]).
///
/// Notes:
///   - on_all_basis_func_at_params
///   - on_euclid(Point4D)->Point3D
///   - on_add_scaled_point4d(dst, a, cp)
///
/// Returns: grid as Vec<Vec<Point3D>> sized (n+1) x (m+1).
pub fn on_eval_surface_grid_points(
    sur: &NurbsSurface,
    u: &[Real], // length n+1
    v: &[Real], // length m+1
    ufl: Side,
    vfl: Side,
) -> Result<Vec<Vec<Point3D>>> {
    const RNAME: &str = "on_eval_surface_grid_points";

    if u.is_empty() || v.is_empty() {
        return Ok(Vec::new());
    }

    let n = u.len() - 1;
    let m = v.len() - 1;

    // ---- local notation ----
    let p = sur.pu as usize;
    let q = sur.pv as usize;

    let nu = sur.nu as usize; // count in u
    let mv = sur.nv as usize; // count in v
    if nu == 0 || mv == 0 {
        return Err(NurbsError::InvalidArgument {
            msg: format!("{RNAME}: empty control net"),
        });
    }


    let n_u = nu - 1;
    let m_v = mv - 1;

    // ---- parameter order checks ----
    for w in u.windows(2) {
        if w[0] > w[1] {
            return Err(NurbsError::InvalidInput {
                msg: format!("{RNAME}: u params must be non-decreasing"),
            });
        }
    }
    for w in v.windows(2) {
        if w[0] > w[1] {
            return Err(NurbsError::InvalidInput {
                msg: format!("{RNAME}: v params must be non-decreasing"),
            });
        }
    }

    on_ensure_param_in_knot_domain(&sur.ku, u[0])?;
    on_ensure_param_in_knot_domain(&sur.ku, u[n])?;
    on_ensure_param_in_knot_domain(&sur.kv, v[0])?;
    on_ensure_param_in_knot_domain(&sur.kv, v[m])?;

    // ---- basis at params ----
    let (nu_basis, usp) = on_all_basis_func_at_params(&sur.ku, sur.pu, u, ufl)?;
    let (nv_basis, vsp) = on_all_basis_func_at_params(&sur.kv, sur.pv, v, vfl)?;

    // output grid S[k][l] (k: u-index, l: v-index)
    let mut out = vec![vec![Point3D { x: 0.0, y: 0.0, z: 0.0 }; m + 1]; n + 1];

    // temporary point4 workspace Tw[] length = max(nu, mv)
    let tw_len = std::cmp::max(nu, mv);
    let mut tw = vec![Point4D { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }; tw_len];

    let r_cost = (m as i64) * ((nu as i64) * (q as i64) + (n as i64) * (p as i64));
    let s_cost = (n as i64) * ((mv as i64) * (p as i64) + (m as i64) * (q as i64));

    // index helpers (row-major ctrl storage)
    #[inline]
    fn pw_at(sur: &NurbsSurface, uu: usize, vv: usize) -> Point4D {
        *sur.ctrl_at_as_ref(uu, vv)
    }

    if r_cost < s_cost {
        // ---- branch A: first contract in v, then in u ----
        for l in 0..=m {
            let s0 = vsp[l]
                .checked_sub(q)
                .ok_or_else(|| NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: v span underflow (vsp[l]={}, q={})", vsp[l], q),
                })?;

            // Tw[i] for i in [usp[0]-p .. usp[n]]
            let i0 = usp[0]
                .checked_sub(p)
                .ok_or_else(|| NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: u span underflow (usp[0]={}, p={})", usp[0], p),
                })?;
            let i1 = usp[n];

            if i1 > n_u {
                return Err(NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: usp[n]={} exceeds max u index {}", i1, n_u),
                });
            }
            if s0 + q > m_v {
                return Err(NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: v-span block exceeds ctrl-net (s0={}, q={}, max_v={})", s0, q, m_v),
                });
            }

            for i in i0..=i1 {
                // Tw[i] = sum_{j=0..q} NV[l][j] * Pw[i][s0+j]
                let mut acc = Point4D { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
                for j in 0..=q {
                    let a = nv_basis[l][j];
                    let cp = pw_at(sur, i, s0 + j);
                    on_add_scaled_point4d(&mut acc, a, &cp);
                }
                // store into tw at index i (safe because tw_len=max(nu,mv) and i<=n_u<nu<=tw_len)
                tw[i] = acc;
            }

            for k in 0..=n {
                let r0 = usp[k]
                    .checked_sub(p)
                    .ok_or_else(|| NurbsError::InvalidArgument {
                        msg: format!("{RNAME}: u span underflow at k={} (usp={}, p={})", k, usp[k], p),
                    })?;

                // Sw = sum_{i=0..p} NU[k][i]*Tw[r0+i]
                let mut sw = Point4D { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
                for i in 0..=p {
                    let a = nu_basis[k][i];
                    let cp = tw[r0 + i];
                    on_add_scaled_point4d(&mut sw, a, &cp);
                }
                out[k][l] = on_euclid(sw);
            }
        }
    } else {
        // ---- branch B: first contract in u, then in v ----
        for k in 0..=n {
            let r0 = usp[k]
                .checked_sub(p)
                .ok_or_else(|| NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: u span underflow at k={} (usp={}, p={})", k, usp[k], p),
                })?;

            let j0 = vsp[0]
                .checked_sub(q)
                .ok_or_else(|| NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: v span underflow (vsp[0]={}, q={})", vsp[0], q),
                })?;
            let j1 = vsp[m];

            if j1 > m_v {
                return Err(NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: vsp[m]={} exceeds max v index {}", j1, m_v),
                });
            }
            if r0 + p > n_u {
                return Err(NurbsError::InvalidArgument {
                    msg: format!("{RNAME}: u-span block exceeds ctrl-net (r0={}, p={}, max_u={})", r0, p, n_u),
                });
            }

            for j in j0..=j1 {
                // Tw[j] = sum_{i=0..p} NU[k][i]*Pw[r0+i][j]
                let mut acc = Point4D { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
                for i in 0..=p {
                    let a = nu_basis[k][i];
                    let cp = pw_at(sur, r0 + i, j);
                    on_add_scaled_point4d(&mut acc, a, &cp);
                }
                tw[j] = acc;
            }

            for l in 0..=m {
                let s0 = vsp[l]
                    .checked_sub(q)
                    .ok_or_else(|| NurbsError::InvalidArgument {
                        msg: format!("{RNAME}: v span underflow at l={} (vsp={}, q={})", l, vsp[l], q),
                    })?;

                // Sw = sum_{j=0..q} NV[l][j]*Tw[s0+j]
                let mut sw = Point4D { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
                for j in 0..=q {
                    let a = nv_basis[l][j];
                    let cp = tw[s0 + j];
                    on_add_scaled_point4d(&mut sw, a, &cp);
                }
                out[k][l] = on_euclid(sw);
            }
        }
    }
    Ok(out)
}
```
---
