# Decompose NurbsSurface To BezierPatch
## 1. 전체 알고리즘 개요
- 목표는:
- 하나의 NURBS 곡면 S(u,v) 를  
    서로 겹치지 않는 Bézier 패치들의 격자로 분해하는 것.

- 즉,
- 원래 곡면:
```math
S(u,v)=\sum _{i=0}^n\sum _{j=0}^mN_{i,p}(u)\, M_{j,q}(v)\, P_{ij}
```
- 여기서 $N_{i,p}$, $M_{j,q}$ 는 B-spline basis, $P_{ij}$ 는 (동차) 제어점
- 이를 각 knot span마다 Bézier 패치로 쪼갬:
```math
S(u,v)=\bigcup _{a,b}S_{ab}(u,v)
```
- 각 $S_{ab}$ 는 $[u_a,u_{a+1}]\times [v_b,v_{b+1}]$ 위의 Bézier 곡면.
- 이걸 하는 표준 방식:
    - U 방향으로 먼저 Bézier strip으로 분해
    - 각 strip을 V 방향으로 다시 Bézier 패치들로 분해

## 2. knot_nonzero_span_count: 
- 유효 span 개수 계산
```rust
fn knot_nonzero_span_count(knots: &[Real], p: usize) -> usize {
    let m = knots.len().saturating_sub(1);
    if m == 0 || m < 2 * p + 1 {
        return 0;
    }
    let mut nsp = 0usize;
    for i in p..(m - p) {
        if knots[i] != knots[i + 1] {
            nsp += 1;
        }
    }
    nsp
}
```
- 수식적으로는:
- 유효한 내부 span은
```math
[U_i,U_{i+1}]\quad \mathrm{with\  }U_i\neq U_{i+1},\  i=p,\dots ,m-p-1
```
- 그 개수를 세는 함수.
    - 이게 U 방향 Bézier strip 개수 = nsu,
    - V 방향 Bézier strip 개수 = nsv가 된다.

## 3. Bezier 패치의 구조
```rust
fn make_empty_bezier_patch(p: usize, q: usize) -> NurbsSurface {
    let nu = p + 1;
    let nv = q + 1;

    let ctrl = vec![Point4D::default(); nu * nv];
    let ku = KnotVector::new(vec![0.0; 2 * (p + 1)]).unwrap(); // len = 2p+2
    let kv = KnotVector::new(vec![0.0; 2 * (q + 1)]).unwrap(); // len = 2q+2

    NurbsSurface::new_from_flat_ctrl(p, q, nu, nv, ctrl, ku, kv).unwrap()
}
```
- Bézier 곡면은 각 방향에서 degree = p, control point 개수 = p+1,  
    knot vector 길이 = 2p+2.
- 여기서는 일단 0으로 채워진 knot vector를 만들고,
- 나중에 실제 분해 과정에서 각 패치마다 ku, kv를 덮어쓴다.
- 즉, 이 함수는 **빈 Bézier 패치 껍데기**를 만드는 역할.
## 4. U 방향 분해: Bw / NBw 와 가상 knot 삽입핵심 버퍼:
- bw: 크기 $(p+1)\times (m+1)$
    - U 방향으로 degree+1개의 control row, V 방향 전체
- nbw: 다음 strip 준비용 버퍼
- 초기화:
```rust
for i in 0..=p {
    for l in 0..=m {
        bw[bw_idx(i, l, p, m)] = *self.ctrl_ref(i, l);
    }
}
```
### 4.1. U 방향 knot multiplicity와 가상 삽입
```rust
while ieu < r {
    iq += 1;
    // ...
    while ieu < r && up[ieu] == up[ieu + 1] {
        ieu += 1;
    }
    let mlu = ieu - i + 1;      // 현재 knot의 multiplicity
    let ru = p.saturating_sub(mlu); // 필요한 삽입 횟수
```
- 수식적으로:
    - 어떤 내부 knot U_k의 multiplicity = m_u
    - Bézier 분해를 위해서는 각 내부 knot의 multiplicity가 degree와 같아야 함
    - 부족한 만큼 r_u=p-m_u 번 knot 삽입을 해야 함
### 4.2. alpha 계산 (de Boor 스타일)
```rust
let num = up[ieu] - up[isu];
for ii in (mlu + 1..=p).rev() {
    let denom = up[isu + ii] - up[isu];
    let alpha = if denom != 0.0 { num / denom } else { 0.0 };
    let k = ii - mlu - 1;
    uals[k] = alpha;
    omus[k] = 1.0 - alpha;
}
```
수식:
- 일반적인 knot 삽입에서 control point 업데이트는:
```math
P_i^{(r+1)}=\alpha _iP_i^{(r)}+(1-\alpha _i)P_{i-1}^{(r)}
```
- 여기서
```math
\alpha _i=\frac{U^*-U_i}{U_{i+p-r}-U_i}
```
- 코드에서는:
    - $U^*=U[\mathrm{ieu}]$
    - 시작 index = isu
    - ii가 control index 역할
### 4.3. Bw 업데이트와 NBw 저장
```rust
for step in 1..=ru {
    let su = mlu + step;
    let save = ru - step;

    for j in (su..=p).rev() {
        let a = uals[j - su];
        let b = omus[j - su];
        for l in 0..=m {
            let cur = bw_idx(j, l, p, m);
            let prev = bw_idx(j - 1, l, p, m);
            let out = a * bw[cur] + b * bw[prev];
            bw[cur] = out;
        }
    }

    if ieu < r {
        for l in 0..=m {
            nbw[bw_idx(save, l, p, m)] = bw[bw_idx(p, l, p, m)];
        }
    }
}
```
의미:- 각 step마다 knot 삽입을 한 번씩 수행
    - 그 과정에서 마지막 row (Bw[p][*])가 하나의 Bézier strip의 끝 control row가 됨
    - 그걸 NBw[save][*]에 저장해서 다음 strip의 시작으로 사용
- 마지막에:
```rust
// 다음 strip 준비
if ieu < r {
    for irow in ru..=p {
        let src_u = ieu - p + irow;
        for l in 0..=m {
            nbw[bw_idx(irow, l, p, m)] = *self.ctrl_ref(src_u, l);
        }
    }
}
isu = ieu;
ieu += 1;
std::mem::swap(&mut bw, &mut nbw);
````
## 5. V 방향 분해: 
- 각 U-strip을 Bézier 패치들로
    - U-strip 하나에 대해:
    - 먼저 sur_a[iq][0]에 첫 번째 V 패치의 초기 control을 채움:
```rust
let patch = &mut sur_a[iq_usize][0];
for j in 0..=q {
    for k in 0..=p {
        let src = bw[bw_idx(k, j, p, m)];
        let dst_i = Self::patch_ctrl_index(nu_patch, k, j);
        patch.ctrl[dst_i] = src;
    }
}
```

- 그 다음 while iev < s 루프에서 V 방향도 U와 같은 방식으로:
    - knot multiplicity mlv
    - 필요한 삽입 횟수 rv = q - mlv
    - vals, omvs에 alpha, 1-alpha 저장
    - de Boor 스타일로 control point 업데이트
    - 마지막 column을 다음 패치의 시작 column으로 복사
- 여기서 중요한 부분:
```rust
let row = &mut sur_a[iq_usize];
let (left, right) = row.split_at_mut(jq_usize + 1);

let patch = &mut left[jq_usize];
let mut next_patch_opt: Option<&mut NurbsSurface> =
    if has_next { Some(&mut right[0]) } else { None };
```
- split_at_mut로 현재 패치와 다음 패치를 겹치지 않는 두 &mut로 분리
- Rust의 aliasing 규칙을 깨지 않으면서 복수 접근을 재현한 것.
- 그리고:
```rust
if iev < s {
    if let Some(next_patch) = next_patch_opt.as_deref_mut() {
        for k in 0..=p {
            let src_i = Self::patch_ctrl_index(nu_patch, k, q);
            let dst_i = Self::patch_ctrl_index(nu_patch, k, save);
            next_patch.ctrl[dst_i] = patch.ctrl[src_i];
        }
    }
}
```
- V 방향에서도 U와 동일하게, 마지막 column이 다음 패치의 시작 부분이 된다.
- 마지막에 각 패치의 knot/domain 설정:
```rust
for t in 0..=p {
    patch.ku.knots[t] = up[isu];
    patch.ku.knots[t + p + 1] = up[ieu];
}
for t in 0..=q {
    patch.kv.knots[t] = vp[isv];
    patch.kv.knots[t + q + 1] = vp[iev];
}
patch.domain_u = Interval { t0: up[isu], t1: up[ieu] };
patch.domain_v = Interval { t0: vp[isv], t1: vp[iev] };
```
- Bézier 패치의 knot vector는 항상:
- U: $[u_a,\dots ,u_a,u_b,\dots ,u_b]$ (각각 p+1번)
- V: $[v_c,\dots ,v_c,v_d,\dots ,v_d]$
## 6. breaks 계산
```rust
for i in 0..nsu {
    let ku = &sur_a[i][0].ku.knots;
    let a = ku[0];
    let b = ku[ku.len() - 1];
    if i == 0 {
        u_breaks.push(a);
    }
    u_breaks.push(b);
}
```
- 각 U-strip의 첫 패치에서 [a,b]를 읽어와서
    u_breaks = [u0, u1, u2, ...] 형태로 만듦.
- V도 동일.
- 이건 원래 NURBS 곡면의 파라미터 영역을 Bézier 패치 경계로 나눈 분할점 리스트.

---

## 소스 코드

```rust
impl NurbsSurface {

    #[inline]
    fn knot_nonzero_span_count(knots: &[Real], p: usize) -> usize {
        let m = knots.len().saturating_sub(1);
        if m == 0 || m < 2 * p + 1 {
            return 0;
        }
        let mut nsp = 0usize;
        for i in p..(m - p) {
            if knots[i] != knots[i + 1] {
                nsp += 1;
            }
        }
        nsp
    }
```
```rust
    #[inline]
    fn patch_ctrl_index(nu: usize, u: usize, v: usize) -> usize {
        // row-major by v: idx = u + nu*v
        u + nu * v
    }
```
```rust
    #[inline]
    fn make_empty_bezier_patch(p: usize, q: usize) -> NurbsSurface {
        let nu = p + 1;
        let nv = q + 1;

        let ctrl = vec![Point4D::default(); nu * nv];
        let ku = KnotVector::new(vec![0.0; 2 * (p + 1) + 0]).unwrap(); // len = 2p+2
        let kv = KnotVector::new(vec![0.0; 2 * (q + 1) + 0]).unwrap(); // len = 2q+2

        NurbsSurface::new_from_flat_ctrl(p, q, nu, nv, ctrl, ku, kv).unwrap()
    }
```
```rust
    /// 2D grid of Bezier patches (same layout as C: surQ[i][j]).
    /// Returns (patches, u_breaks, v_breaks).
    ///
    /// - patches[u_seg][v_seg]
    /// - u_breaks len = u_count+1, v_breaks len = v_count+1
    pub fn decompose_to_bezier_patches_no_refine_grid(
        &self,
    ) -> (Vec<Vec<NurbsSurface>>, Vec<Real>, Vec<Real>) {
        let p = self.pu as usize;
        let q = self.pv as usize;

        debug_assert!(p > 0 && q > 0, "degree must be >= 1");

        let (n_last, m_last, r_last, s_last) = self.indices();
        let n = n_last as usize;
        let m = m_last as usize;
        let r = r_last as usize;
        let s = s_last as usize;

        let up = &self.ku.knots;
        let vp = &self.kv.knots;

        let nsu = Self::knot_nonzero_span_count(up, p);
        let nsv = Self::knot_nonzero_span_count(vp, q);

        // If degenerate, just return self as a single patch (safe fallback)
        if nsu == 0 || nsv == 0  {
            let one = self.clone();
            return (vec![vec![one]],
                    vec![self.domain_u.t0, self.domain_u.t1],
                    vec![self.domain_v.t0, self.domain_v.t1]);
        }

        let mut sur_a: Vec<Vec<NurbsSurface>> = (0..nsu)
            .map(|_| (0..nsv).map(|_| Self::make_empty_bezier_patch(p, q)).collect())
            .collect();

        debug_assert!(sur_a.len() == nsu && sur_a[0].len() == nsv);

        let mut bw: Vec<Point4D> = vec![Point4D::default(); (p + 1) * (m + 1)];
        let mut nbw: Vec<Point4D> = vec![Point4D::default(); (p + 1) * (m + 1)];

        // uals/omus size p, vals/omvs size q (C allocates p or q)
        let mut uals: Vec<Real> = vec![0.0; p.max(1)];
        let mut omus: Vec<Real> = vec![0.0; p.max(1)];
        let mut vals: Vec<Real> = vec![0.0; q.max(1)];
        let mut omvs: Vec<Real> = vec![0.0; q.max(1)];

        // helpers to index Bw arrays (row-major by v-column)
        let bw_idx = |uu: usize, vv: usize| -> usize {
            uu + (p + 1) * vv
        };

        // -----------------------------
        // Initialize for u decomposition
        // -----------------------------
        let mut isu: usize = p;
        let mut ieu: usize = p + 1;
        let mut iq: isize = -1;

        // Bw[i][l] = Pw[i][l] for i=0..p, l=0..m
        for i in 0..=p {
            for l in 0..=m {
                bw[bw_idx(i, l)] = *self.ctrl_ref(i, l);
            }
        }

        // ---------------------------------------
        // Decompose in U direction into Bezier strips
        // ---------------------------------------
        while ieu < r {
            iq += 1;
            let iq_usize = iq as usize;

            // knot multiplicity at UP[ieu]
            let start  = ieu;
            while ieu < r && up[ieu] == up[ieu + 1] {
                ieu += 1;
            }
            let mlu = ieu - start  + 1;
            let ru = p.saturating_sub(mlu);

            // Insert (extract) knot virtually if needed (mlu < p)
            if mlu < p {
                debug_assert!(isu + p <= r, "isu+p out of knot range");
                let num = up[ieu] - up[isu];

                for ii in (mlu + 1..=p).rev() {
                    let denom = up[isu + ii] - up[isu];
                    let alpha = if denom != 0.0 { num / denom } else { 0.0 };
                    let k = ii - mlu - 1;
                    uals[k] = alpha;
                    omus[k] = 1.0 - alpha;
                }

                for step in 1..=ru {
                    let su = mlu + step;
                    let save = ru - step;

                    for j in (su..=p).rev() {
                        let a = uals[j - su];
                        let b = omus[j - su];
                        for l in 0..=m {
                            let cur = bw_idx(j, l);
                            let prev = bw_idx(j - 1, l);
                            let out = a * bw[cur] + b * bw[prev];
                            bw[cur] = out;
                        }
                    }

                    // NBw[save][l] = Bw[p][l]
                    if ieu < r {
                        for l in 0..=m {
                            nbw[bw_idx(save, l)] = bw[bw_idx(p, l)];
                        }
                    }
                }
            }

            // ---------------------------------------
            // For this U-strip, decompose in V direction into patches
            // ---------------------------------------
            let mut isv: usize = q;
            let mut iev: usize = q + 1;
            let mut jq: isize = -1;

            // First patch control init:
            // Qw[k][j] = Bw[k][j] for j=0..q, k=0..p
            {
                let patch = &mut sur_a[iq_usize][0];
                let nu_patch = (p + 1) as usize;
                for j in 0..=q {
                    for k in 0..=p {
                        debug_assert!(j <= m, "Bw j out of range: j={j}, m={m}");
                        let src = bw[bw_idx(k, j)];
                        let dst_i = Self::patch_ctrl_index(nu_patch, k, j);
                        patch.ctrl[dst_i] = src;
                    }
                }
            }

            while iev < s {
                jq += 1;
                let jq_usize = jq as usize;

                // current patch is sur_a[iq][jq]
                let row_len = sur_a[iq_usize].len();
                let has_next = jq_usize + 1 < row_len;

                // knot multiplicity at VP[iev]
                let start_v = iev;
                while iev < s && vp[iev] == vp[iev + 1] {
                    iev += 1;
                }
                let mlv = iev - start_v + 1;
                let rv = q.saturating_sub(mlv);

                // Work on the *current* patch control net in-place
                {
                    let row = &mut sur_a[iq_usize];
                    debug_assert!(jq_usize < row.len());
                    debug_assert!(jq_usize + 1 <= row.len());

                    let (left, right) = row.split_at_mut(jq_usize + 1);

                    let patch = &mut left[jq_usize];
                    let mut next_patch_opt: Option<&mut NurbsSurface> =
                        if has_next { Some(&mut right[0]) } else { None };

                    let nu_patch = p + 1;

                    if mlv < q {
                        debug_assert!(isv + q <= s, "isv+q out of knot range");
                        let num = vp[iev] - vp[isv];

                        for jj in (mlv + 1..=q).rev() {
                            let denom = vp[isv + jj] - vp[isv];
                            let alpha = if denom != 0.0 { num / denom } else { 0.0 };
                            let k = jj - mlv - 1;
                            vals[k] = alpha;
                            omvs[k] = 1.0 - alpha;
                        }

                        for step in 1..=rv {
                            let sv = mlv + step;
                            let save = rv - step;

                            for j in (sv..=q).rev() {
                                let a = vals[j - sv];
                                let b = omvs[j - sv];

                                for k in 0..=p {
                                    let cur_i = Self::patch_ctrl_index(nu_patch, k, j);
                                    let prev_i = Self::patch_ctrl_index(nu_patch, k, j - 1);
                                    let out = a * patch.ctrl[cur_i] + b * patch.ctrl[prev_i];
                                    patch.ctrl[cur_i] = out;
                                }
                            }
                            if iev < s {
                                if let Some(next_patch) = next_patch_opt.as_deref_mut() {
                                    for k in 0..=p {
                                        let src_i = Self::patch_ctrl_index(nu_patch, k, q);
                                        let dst_i = Self::patch_ctrl_index(nu_patch, k, save);
                                        next_patch.ctrl[dst_i] = patch.ctrl[src_i];
                                    }
                                }
                            }
                        }
                    }

                    // knots/domain 세팅은 patch만
                    for t in 0..=p {
                        patch.ku.knots[t] = up[isu];
                        patch.ku.knots[t + p + 1] = up[ieu];
                    }
                    for t in 0..=q {
                        patch.kv.knots[t] = vp[isv];
                        patch.kv.knots[t + q + 1] = vp[iev];
                    }
                    patch.domain_u = Interval { t0: up[isu], t1: up[ieu] };
                    patch.domain_v = Interval { t0: vp[isv], t1: vp[iev] };
                }

                // Prepare next patch initial columns from Bw (C: for i=rv..q, NQw[k][i]=Bw[k][iev-q+i])
                if iev < s && has_next {
                    let next_patch = &mut sur_a[iq_usize][jq_usize + 1];
                    let nu_patch = (p + 1) as usize;
                    let base = iev.checked_sub(q).expect("iev-q underflow");
                    for irow in rv..=q {
                        let src_v = base + irow;
                        debug_assert!(src_v <= m, "Bw src_v out of range: src_v={src_v}, m={m}");
                        for k in 0..=p {
                            let src = bw[bw_idx(k, src_v)];
                            let dst_i = Self::patch_ctrl_index(nu_patch, k, irow);
                            next_patch.ctrl[dst_i] = src;
                        }
                    }
                }

                isv = iev;
                iev += 1;
            }

            // ---------------------------------------
            // Prepare for next U strip (C: NBw[i][l] = Pw[ieu-p+i][l])
            // ---------------------------------------
            if ieu < r {
                let base_u = ieu.checked_sub(p).expect("ieu-p underflow");
                for irow in ru..=p {
                    let src_u = base_u + irow;
                    debug_assert!(src_u <= n, "Pw src_u out of range: src_u={src_u}, n={n}");
                    for l in 0..=m {
                        nbw[bw_idx(irow, l)] = *self.ctrl_ref(src_u, l);
                    }
                }
            }

            isu = ieu;
            ieu += 1;

            // swap Bw / NBw
            swap(&mut bw, &mut nbw);
        }

        // breaks (domain boundaries)
        let mut u_breaks: Vec<Real> = Vec::with_capacity(nsu + 1);
        let mut v_breaks: Vec<Real> = Vec::with_capacity(nsv + 1);

        // Use knot endpoints we wrote on patches
        // u_breaks from [i][0]
        for i in 0..nsu {
            let ku = &sur_a[i][0].ku.knots;
            let a = ku[0];
            let b = ku[ku.len() - 1];
            if i == 0 {
                u_breaks.push(a);
            }
            u_breaks.push(b);
        }

        for j in 0..nsv {
            let kv = &sur_a[0][j].kv.knots;
            let a = kv[0];
            let b = kv[kv.len() - 1];
            if j == 0 {
                v_breaks.push(a);
            }
            v_breaks.push(b);
        }

        (sur_a, u_breaks, v_breaks)
    }
```
```rust
    /// Flat list (row-major: u-major, then v) – equivalent to old decompose() but no refinement.
    pub fn decompose_to_bezier_patches_no_refine(&self) -> Vec<NurbsSurface> {
        let (grid, _ub, _vb) = self.decompose_to_bezier_patches_no_refine_grid();
        let mut out = Vec::new();
        for row in grid {
            for p in row {
                out.push(p);
            }
        }
        out
    }
}
```

----
### 테스트 코드
```rust
use nurbslib::core::geom::{Point4D, Point3D};
use nurbslib::core::knot::{KnotVector, on_span_count};
use nurbslib::core::nurbs_surface::NurbsSurface;
use nurbslib::core::types::Real;

fn dist2(a: Point3D, b: Point3D) -> Real {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

fn in_interval(u: Real, a: Real, b: Real, tol: Real) -> bool {
    u >= a - tol && u <= b + tol
}
```
```rust
/// 내부 knot가 있는 (u,v 모두 2개 이상의 span) NURBS surface 생성
fn make_test_surface_with_internal_knots() -> NurbsSurface {
    // degree
    let pu = 2usize;
    let pv = 2usize;

    // control counts
    let nu = 5usize; // => ku len = nu+pu+1 = 8
    let nv = 4usize; // => kv len = nv+pv+1 = 7

    // knots (clamped, with internal knots)
    // U: [0,0,0, 0.5,0.5, 1,1,1] -> 2 spans
    let ku = KnotVector::new(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0]).unwrap();
    // V: [0,0,0, 0.5, 1,1,1] -> 2 spans
    let kv = KnotVector::new(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0]).unwrap();

    // ctrl: row-major (u + nu*v)
    let mut ctrl = Vec::with_capacity(nu * nv);

    for v in 0..nv {
        for u in 0..nu {
            let uu = u as Real;
            let vv = v as Real;

            // 약간 비선형 z, 약간 weight 변동(=rational 케이스도 커버)
            let z = 0.12 * uu * uu + 0.07 * vv + 0.03 * uu * vv;
            let w = 1.0 + 0.05 * (((u + 2 * v) % 3) as Real);

            ctrl.push(Point4D::from_hom(uu, vv, z, w));
        }
    }

    NurbsSurface::new_from_flat_ctrl(pu, pv, nu, nv, ctrl, ku, kv).unwrap()
}
```
```rust
fn assert_bezier_form_knots(knots: &[Real], p: usize, tol: Real) {
    // Bezier-form: first p+1 are equal, last p+1 are equal
    let a = knots[0];
    let b = knots[knots.len() - 1];

    for i in 0..=p {
        assert!((knots[i] - a).abs() <= tol, "start knot not clamped");
        assert!(
            (knots[knots.len() - 1 - i] - b).abs() <= tol,
            "end knot not clamped"
        );
    }
}
```
```rust
#[test]
fn decompose_plane_xy_single_patch() {
    let s = NurbsSurface::from_plane_xy().unwrap();

    let (grid, ub, vb) = s.decompose_to_bezier_patches_no_refine_grid();

    assert_eq!(grid.len(), 1);
    assert_eq!(grid[0].len(), 1);

    assert_eq!(ub.len(), 2);
    assert_eq!(vb.len(), 2);

    // 몇 점 샘플링 비교
    let samples = [0.0, 0.2, 0.5, 0.8, 1.0];
    for &u in &samples {
        for &v in &samples {
            let p0 = s.eval_point(u, v);
            let p1 = grid[0][0].eval_point(u, v);
            assert!(dist2(p0, p1) <= 1e-20);
        }
    }
}
```
```rust
#[test]
fn decompose_patch_count_matches_span_count() {
    let s = make_test_surface_with_internal_knots();

    let p = s.degree_u();
    let q = s.degree_v();
    let ucnt = on_span_count(&s.ku.knots, p);
    let vcnt = on_span_count(&s.kv.knots, q);

    let (grid, _ub, _vb) = s.decompose_to_bezier_patches_no_refine_grid();

    assert_eq!(grid.len(), ucnt, "u patch count mismatch");
    assert_eq!(grid[0].len(), vcnt, "v patch count mismatch");
}
```
```rust
#[test]
fn decompose_patches_are_bezier_form_knots_and_domains() {
    let s = make_test_surface_with_internal_knots();
    let p = s.degree_u();
    let q = s.degree_v();

    let (grid, _ub, _vb) = s.decompose_to_bezier_patches_no_refine_grid();

    let tol = 1e-12;

    for row in &grid {
        for patch in row {
            // 1) knot is Bezier form
            assert_bezier_form_knots(&patch.ku.knots, p, tol);
            assert_bezier_form_knots(&patch.kv.knots, q, tol);

            // 2) domain matches knot end values
            let u0 = patch.ku.knots[0];
            let u1 = patch.ku.knots[patch.ku.knots.len() - 1];
            let v0 = patch.kv.knots[0];
            let v1 = patch.kv.knots[patch.kv.knots.len() - 1];

            assert!((patch.domain_u.t0 - u0).abs() <= tol);
            assert!((patch.domain_u.t1 - u1).abs() <= tol);
            assert!((patch.domain_v.t0 - v0).abs() <= tol);
            assert!((patch.domain_v.t1 - v1).abs() <= tol);
        }
    }
}
```
```rust
#[test]
fn decompose_eval_matches_original_surface_on_samples() {
    let s = make_test_surface_with_internal_knots();
    let (grid, _ub, _vb) = s.decompose_to_bezier_patches_no_refine_grid();

    // 샘플 파라미터(각 patch 내부/경계 포함)
    let us = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0];
    let vs = [0.0, 0.12, 0.3, 0.5, 0.72, 0.88, 1.0];

    let tol = 1e-10;
    let tol2 = tol * tol;

    for &u in &us {
        for &v in &vs {
            // 원본
            let p0 = s.eval_point(u, v);

            // 해당 (u,v)를 포함하는 patch 찾기
            let mut found = None;
            for row in &grid {
                for patch in row {
                    if in_interval(u, patch.domain_u.t0, patch.domain_u.t1, 1e-12)
                        && in_interval(v, patch.domain_v.t0, patch.domain_v.t1, 1e-12)
                    {
                        found = Some(patch);
                        break;
                    }
                }
                if found.is_some() {
                    break;
                }
            }

            let patch = found.expect("no patch covers (u,v)");
            let p1 = patch.eval_point(u, v);

            let d2 = dist2(p0, p1);
            assert!(
                d2 <= tol2,
                "eval mismatch at (u,v)=({:.6},{:.6}): d2={:.3e}",
                u,
                v,
                d2
            );
        }
    }
}
```
---
