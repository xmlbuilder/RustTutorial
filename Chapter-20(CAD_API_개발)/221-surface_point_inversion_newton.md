# on_surface_point_inversion_newton

## 1. 문제 정의
- 주어진 것:
    - NURBS 곡면 $S(u,v)\in \mathbb{R^{\mathnormal{3}}}$
    - 공간 점 $P\in \mathbb{R^{\mathnormal{3}}}$
- 하고 싶은 것:
$Q=S(u,v)$ 가 P 에 가장 가까운 점이 되도록
$(u,v)$ 를 찾는 것 (곡면 위 최근접점).

- 즉, 최소화 문제:
```math
\min _{u,v}\; F(u,v)=\frac{1}{2}\| S(u,v)-P\| ^2
```
- 여기서
```math
R(u,v)=S(u,v)-P
```
## 2. 1차 미분: Newton에서 쓰는 기본량
- 편미분:
```math
S_u=\frac{\partial S}{\partial u},\quad S_v=\frac{\partial S}{\partial v}
```
- 목적함수:
```math
F(u,v)=\frac{1}{2}\langle R,R\rangle
``` 
- 그래디언트:
```math
\frac{\partial F}{\partial u}=\langle R,S_u\rangle =:r_u=\nu 
```
```math
\frac{\partial F}{\partial v}=\langle R,S_v\rangle =:r_v=\nu _v
```
코드에서:
- $r = S - P$
- $nu = Su · r → r_u$
- $nv = Sv · r → r_v$
- 이 두 개가 **현재 점이 최적점에서 얼마나 벗어났는지** 를 나타내는 1차 정보.

## 3. 2차 미분: Newton 시스템 구성
- 2차 미분(헤시안 항):
```math
S_{uu}=\frac{\partial ^2S}{\partial u^2},\quad S_{uv}=\frac{\partial ^2S}{\partial u\partial v},\quad S_{vv}=\frac{\partial ^2S}{\partial v^2}
```
- F 의 2차 미분은:
```math
\frac{\partial ^2F}{\partial u^2}=\langle S_u,S_u\rangle +\langle R,S_{uu}\rangle
```

```math 
\frac{\partial ^2F}{\partial v^2}=\langle S_v,S_v\rangle +\langle R,S_{vv}\rangle
```

```math 
\frac{\partial ^2F}{\partial u\partial v}=\langle S_u,S_v\rangle +\langle R,S_{uv}\rangle 
```

코드에서:
- $du = |Su|$
- $dv = |Sv|$
- $ruu = R·Suu$
- $ruv = R·Suv$
- $rvv = R·Svv$
- $uv = Su·Sv$
- 그리고 이렇게 조합:
- $f_u=|S_u|^2+\langle R,S_{uu}\rangle$ 
- $g_v=|S_v|^2+\langle R,S_{vv}\rangle$ 
- $f_v=g_u=\langle S_u,S_v\rangle +\langle R,S_{uv}\rangle$ 
- 코드:
```rust
let fu = du * du + ruu;
let fv = uv + ruv;
let gu = fv;
let gv = dv * dv + rvv;
```

## 4. Newton 시스템
- Newton 방법은
```math
\nabla F(u,v)+H\cdot \left[ \begin{matrix}\Delta u\\ \Delta v\end{matrix}\right] =0
```
- 여기서
```math
H=\left[ \begin{matrix}f_u&f_v\\ g_u&g_v\end{matrix}\right] 
```
- 따라서 선형 시스템:
```math
\left[ \begin{matrix}f_u&f_v\\ g_u&g_v\end{matrix}\right]
\left[ \begin{matrix}\Delta u\\ \Delta v\end{matrix}\right] =-\left[ \begin{matrix}r_u\\ r_v\end{matrix}\right]
``` 
- 코드에서는 부호를 뒤집어서 다음과 같이 쓰고 있음:
```rust
// ru = nu, rv = nv
let ru = nu;
let rv = nv;

// den = fu*gv - fv*gu
// dlu = (rv*fv - ru*gv) / den
// dlv = (ru*gu - rv*fu) / den
```

- 이건 사실상 위 시스템을 풀어서 얻은 해:
```math
\Delta u=\frac{r_vf_v-r_ug_v}{\det H}
```
```math
\Delta v=\frac{r_ug_u-r_vf_u}{\det H}
```

## 5. 업데이트와 파라미터 보정
- Newton step:
```math
u_{\mathrm{new}}=u_{\mathrm{old}}+\Delta u
```
```math
v_{\mathrm{new}}=v_{\mathrm{old}}+\Delta v
```
- 코드:
```math
unew = uold + dlu;
vnew = vold + dlv;
```

- 그 다음:
    - 곡면이 닫혀 있으면 wrap (mod domain)
    - 아니면 clamp
```rust
on_clamp_or_wrap_param(&mut unew, umin, umax, uclosed);
on_clamp_or_wrap_param(&mut vnew, vmin, vmax, vclosed);
```


## 6. 수렴 조건들
- (1) 점 일치 (distance 기준)
```math
\| R\| =\| S(u,v)-P\| \leq \mathrm{top}
```
    - 충분히 가까우면 종료.

- (2) “zero cosine” 조건 (법선 방향 정렬)
- 여기서 핵심은:
- $\nu =S_u\cdot R$
- $\nu _v=S_v\cdot R$
- 이걸 정규화해서 “각도”를 보는 것:
```math
z_{cu}=\frac{|\nu |}{\| S_u\| \cdot \| R\| }
```
```math
z_{cv}=\frac{|\nu _v|}{\| S_v\| \cdot \| R\| }
```
- 이 값이 충분히 작으면,  
    R 가 곡면의 접평면에 거의 수직(=법선 방향)이라는 뜻 
    - 최적점 근처.
- 코드:
```rust
let zcu = (nu.abs()) / (du * dis);
let zcv = (nv.abs()) / (dv * dis);
if zcu <= toc && zcv <= toc { ... }
```


- (3) 파라미터 변화에 대응하는 3D 이동량
- Newton step이 파라미터 공간에서 너무 작아져서  
    더 이상 의미 있는 3D 이동이 없으면 종료.
```math
A=\Delta u\cdot S_u+\Delta v\cdot S_v
```
```math
\| A\| \leq \mathrm{top}
```
- 코드:
```rust
let a = su * (unew - uold) + sv * (vnew - vold);
let mag = a.length();
if mag <= top { ... }
```


## 7. 전체 알고리즘 흐름 (요약)
- 초기 (u_0,v_0) 를 도메인 안으로 clamp/wrap
- 반복:
    - $S,S_u,S_v,S_{uu},S_{uv},S_{vv}$ 계산
    - R=S-P, 거리, nu, nv, du, dv 계산
    - “best” (가장 작은 거리) 갱신
    - 거리 기준 수렴 체크
    - zero cosine 기준 수렴 체크
    - Newton 시스템 구성 → \Delta u,\Delta v 계산
    - 파라미터 업데이트 + clamp/wrap
    - 3D 이동량 기준 수렴 체크
    - IT_LIM 도달 시: 에러 리턴하지만, best는 구조체에 담아서 돌려줌

## 8. 이 루틴이 좋은 이유
- 단순히 **직교 투영** 이 아니라,
    곡면 위 최근접점을 찾는 진짜 최적화 문제를 푸는 것.
- 1차/2차 미분을 모두 사용한 Newton-Raphson in parameter space.
- 기하학적으로:
    - 최적점에서는 R 이 곡면의 접평면에 수직
    - 즉, 곡면 법선 방향과 R 이 정렬되는 지점을 찾는 것.

---

```rust

/// Surface point inversion/projection using Newton's method.
///
/// Finds (u,v) such that Q = S(u,v) is closest to P.
/// Returns the **best** (u,v,Q,dist) found even if convergence fails.
///
/// Notes / policy alignment:
/// - Uses your `on_surface_derivatives(.., udr=2, vdr=2, mfl=true)`
/// - Uses `Side::Left` for both (matches C call N_surder(...,LEFT,LEFT,...))
/// - Handles closed surfaces in U/V by wrapping parameters; otherwise clamps.
/// - Convergence checks follow the C routine:
///   1) point coincidence (dist <= top)
///   2) zero cosine (|nu|/(|Su|*dist) <= toc and |nv|/(|Sv|*dist) <= toc)
///   3) parameter range (wrap/clamp)
///   4) parameter change mapped to 3D (|dlu*Su + dlv*Sv| <= top)
///
/// Return:
/// - Ok(best) on convergence
/// - Err(Convergence) if ITLIM reached, but best is still returned in the struct.
#[derive(Debug, Clone, Copy)]
pub struct SurfaceProjectionResult {
    pub u: Real,
    pub v: Real,
    pub q: Point3D,
    pub dist: Real,
    pub iters: usize,
}

pub fn on_surface_point_inversion_newton(
    sur: &NurbsSurface,
    p: Point3D,
    u0: Real,
    v0: Real,
    top: Real, // point coincidence tolerance
    toc: Real, // zero cosine tolerance
) -> Result<SurfaceProjectionResult> {

    // ---- knot domain ----
    let u_knots = sur.ku.as_slice();
    let v_knots = sur.kv.as_slice();
    if u_knots.is_empty() || v_knots.is_empty() {
        return Err(NurbsError::InvalidArgument {
            msg: "on_surface_project_newton: empty knot vector".into(),
        });
    }
    let umin = u_knots[0];
    let umax = u_knots[u_knots.len() - 1];
    let vmin = v_knots[0];
    let vmax = v_knots[v_knots.len() - 1];

    // ---- closed flags (use your own helpers if you have them) ----
    let uclosed = sur.is_closed_u();
    let vclosed = sur.is_closed_v();

    // ---- initial ----
    let mut unew = u0;
    let mut vnew = v0;

    // bring initial into range (same policy as iteration range check)
    on_clamp_or_wrap_param(&mut unew, umin, umax, uclosed);
    on_clamp_or_wrap_param(&mut vnew, vmin, vmax, vclosed);

    let mut best_u = unew;
    let mut best_v = vnew;
    let mut best_q = sur.eval_point(best_u, best_v);
    let mut best_dis = (best_q - p).length();

    // ---- iteration ----
    let mut k = 0usize;
    while k < (IT_LIM as usize) {
        // derivatives up to 2nd order
        // D[0][0]=S, D[1][0]=Su, D[0][1]=Sv, D[2][0]=Suu, D[1][1]=Suv, D[0][2]=Svv
        let d = on_surface_derivatives(sur, unew, vnew, Side::Left, Side::Left, true, 2, 2)?;

        let s00 = d[0][0]; // Point3D
        let su = d[1][0].to_vector();
        let sv = d[0][1].to_vector();
        let suu = d[2][0].to_vector();
        let suv = d[1][1].to_vector();
        let svv = d[0][2].to_vector();

        // R = S(u,v) - P
        let r = (s00 - p).to_vector();

        // nu = Su · R, nv = Sv · R
        let nu = Vector3D::dot(&su, &r);
        let nv = Vector3D::dot(&sv, &r);

        // |Su|, |Sv|
        let du = su.length();
        let dv = sv.length();

        // dist = |R|
        let dis = r.length();

        // update best
        if dis < best_dis {
            best_dis = dis;
            best_u = unew;
            best_v = vnew;
            best_q = s00;
        }

        // Check #1: point coincidence
        if dis <= top {
            return Ok(SurfaceProjectionResult {
                u: best_u,
                v: best_v,
                q: best_q,
                dist: best_dis,
                iters: k,
            });
        }

        // Check #2: zero cosine
        // zcu = |nu|/(|Su|*|R|), zcv = |nv|/(|Sv|*|R|)
        let denom_u = du * dis;
        let denom_v = dv * dis;

        if denom_u.abs() <= ON_ZERO_TOL || denom_v.abs() <= ON_ZERO_TOL {
            // matches C: ERROR(CON_ERR) in division check paths
            return Err(NurbsError::ConvergenceError {
                msg: "on_surface_project_newton: zero denominator in cosine test".into(),
            });
        }

        let zcu = (nu.abs()) / denom_u;
        let zcv = (nv.abs()) / denom_v;
        if zcu <= toc && zcv <= toc {
            return Ok(SurfaceProjectionResult {
                u: best_u,
                v: best_v,
                q: best_q,
                dist: best_dis,
                iters: k,
            });
        }

        // ---- Newton step ----
        // ru = nu, rv = nv
        let ru = nu;
        let rv = nv;

        // ruu = R·Suu, ruv = R·Suv, rvv = R·Svv
        let ruu = Vector3D::dot(&r, &suu);
        let ruv = Vector3D::dot(&r, &suv);
        let rvv = Vector3D::dot(&r, &svv);

        // uv = Su·Sv
        let uv = Vector3D::dot(&su, &sv);

        // fu = |Su|^2 + ruu
        // fv = (Su·Sv) + ruv
        // gu = fv
        // gv = |Sv|^2 + rvv
        let fu = du * du + ruu;
        let fv = uv + ruv;
        let gu = fv;
        let gv = dv * dv + rvv;

        let den = fu * gv - fv * gu;

        if den.abs() <= ON_ZERO_TOL {
            return Err(NurbsError::ConvergenceError {
                msg: "on_surface_project_newton: singular Newton system (den≈0)".into(),
            });
        }

        // dlu = (rv*fv - ru*gv) / den
        // dlv = (ru*gu - rv*fu) / den
        let dlu = (rv * fv - ru * gv) / den;
        let dlv = (ru * gu - rv * fu) / den;

        let uold = unew;
        let vold = vnew;

        unew = uold + dlu;
        vnew = vold + dlv;

        // Check #3: parameter range (wrap/clamp)
        on_clamp_or_wrap_param(&mut unew, umin, umax, uclosed);
        on_clamp_or_wrap_param(&mut vnew, vmin, vmax, vclosed);

        // Check #4: parameter change mapped to 3D
        // A = dlu*Su + dlv*Sv, mag = |A|
        let a = su * (unew - uold) + sv * (vnew - vold);
        let mag = a.length();
        if mag <= top {
            return Ok(SurfaceProjectionResult {
                u: best_u,
                v: best_v,
                q: best_q,
                dist: best_dis,
                iters: k + 1,
            });
        }

        k += 1;
    }

    // no convergence -> return error but best is known
    Err(NurbsError::ConvergenceError {
        msg: format!(
            "on_surface_project_newton: did not converge (ITLIM={}), best_dist={}",
            IT_LIM, best_dis
        ),
    })
}
```
---
