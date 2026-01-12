## NurbsCurve 함수 설명

## 📘 1. knot_nonzero_span_count()
- ✔ 기능
- Clamped knot vector에서 non-zero span의 개수를 계산한다.
- 이는 곡선을 Bezier 조각으로 분해할 때 필요한 조각 개수와 동일하다.
- ✔ 수식
- Span [U_i,U_{i+1}] 가 non-zero span이 되려면:
```math
U_i\neq U_{i+1}
```
- Clamped knot vector에서 유효한 i 범위는:
```math
i=p\ldots (m-p-1)
```
- 여기서
    - p = degree
    - m = knot count - 1
- ✔ 알고리즘 검증
```rust
Rust 코드:
for i in p..(m.saturating_sub(p)) {
    if u[i] != u[i + 1] {
        nsp += 1;
    }
}
```

- 이는 Piegl & Tiller의 수식과 완전히 동일한 루프다.
- ✔ 용도
    - Bezier 분해 시 조각 개수를 결정
    - Knot vector의 구조 분석
    - 곡선이 단일 Bezier인지, 여러 segment인지 판단
 
## 소스 코드
```rust
/// number of non-zero spans in a clamped knot vector.
/// (C: count i=p..m-p-1 where U[i] != U[i+1])
pub fn knot_nonzero_span_count(&self) -> usize {
    let p = self.degree as usize;
    let u = &self.kv.knots;
    if u.len() < 2 * (p + 1) + 1 {
        return 0;
    }
    let m = u.len() - 1; // highest index
    let mut nsp = 0usize;
    // i = p .. m-p-1  (C: i<p..m-p; i++ with m as highest index)
    for i in p..(m.saturating_sub(p)) {
        if u[i] != u[i + 1] {
            nsp += 1;
        }
    }
    nsp
}
```

## 📘 2. to_bezier_pieces_no_refine()
- ✔ 기능
- NURBS 곡선을 Bezier 조각들로 분해한다.
    - 단, global knot refinement 없이, Piegl & Tiller의 알고리즘을 그대로 구현한 버전.
- ✔ 수학적 원리
- Bezier 분해는 다음 조건에서 segment가 끊어진다:
```math
U_i\neq U_{i+1}
```
- 즉, non-zero span마다 하나의 Bezier segment가 존재한다.
- 각 Bezier segment는:
    - degree = p
    - control points = p+1
    - knot vector =
```math
[a,a,...,a,b,b,...,b]\quad (p+1\mathrm{번씩})
```
- ✔ 알고리즘 핵심
- 첫 segment의 control point 초기화
```math
Q_0=P_0,\ldots ,Q_p=P_p
```
- 각 knot span마다 multiplicity 검사
```math
\mathrm{mlt}=\mathrm{multiplicity\  of\  }U_{ie}
```
- 필요하면 local knot insertion 수행
- Piegl & Tiller의 alpha blending:
```math
Q_j=\alpha Q_j+(1-\alpha )Q_{j-1}
```
- Bezier knot vector 생성
```math
U_q=[U_{is},...,U_{is},U_{ie},...,U_{ie}]
```
- 다음 segment의 초기 control point 설정
- ✔ 검증
- 이 구현은 Piegl & Tiller의 알고리즘과 1:1 대응한다.
- 특히:
    - alfs / omas 배열
    - backward blending
    - next.ctrl[save] = qw[p]
    - segment knot clamping
- 모두 원전 알고리즘과 동일.
- ✔ 용도
    - 곡선을 Bezier 조각으로 분해
    - GPU tessellation
    - CAD 커널의 trimming, intersection
    - 곡선 subdivision
 
### 소스 코드
```rust
/// Decompose curve into Bezier pieces (without global knot refinement).
/// Returns Bezier segments represented as NurbsCurve objects.
/// Each piece has:
/// - degree = p
/// - ctrl count = p+1
/// - knot count = 2p+2 (clamped): [a..a] (p+1 times), [b..b] (p+1 times)
pub fn to_bezier_pieces_no_refine(&self) -> Result<Vec<NurbsCurve>> {
    if !self.is_valid() {
        return Err(NurbsError::InvalidInput {
            msg: "to_bezier_pieces_no_refine: invalid curve".into(),
        });
    }

    let p = self.degree as usize;
    let pw = &self.ctrl;
    let up = &self.kv.knots;

    // Basic structural checks (similar spirit to U_curbre usage)
    if pw.len() < p + 1 || up.len() < pw.len() + p + 1 {
        // Degenerate curve -> treat as single piece
        return Ok(vec![self.clone()]);
    }

    // number of non-zero spans
    let nsp = self.knot_nonzero_span_count();
    if nsp == 0 {
        return Ok(vec![self.clone()]);
    }

    // Allocate output curves (each will be a Bezier segment represented as NurbsCurve)
    let mut pieces: Vec<NurbsCurve> = Vec::with_capacity(nsp);
    for _ in 0..nsp {
        let ctrl = vec![Point4D::homogeneous(0.0, 0.0, 0.0, 1.0); p + 1];
        let knots = KnotVector { knots: vec![0.0; 2 * p + 2] };
        let mut seg = NurbsCurve::new(self.degree, ctrl, knots)?;
        seg.dimension = self.dimension; // preserve 2D/3D tag
        pieces.push(seg);
    }

    // ---- Initialize first piece ctrl = Pw[0..p] ----
    for i in 0..=p {
        pieces[0].ctrl[i] = pw[i];
    }

    // ---- Loop through knot vector and extract each segment ----
    let m = up.len() - 1; // highest index, as in C
    let mut is_ = p;      // is = p
    let mut ie = p + 1;   // ie = p+1
    let mut iq = 0usize;  // iq starts at 0 for first segment

    // Temporary alpha arrays (size p like C alfs/omas)
    let mut alfs = vec![0.0f64; p];
    let mut omas = vec![0.0f64; p];

    while ie < m {
        // split mutable borrows safely: current piece and optional next piece
        let (left, right) = pieces.split_at_mut(iq + 1);
        let cur = &mut left[iq];


        let qw = &mut cur.ctrl;
        let uq = &mut cur.kv.knots;

        // ---- knot multiplicity at UP[ie] ----
        let i0 = ie;
        while ie < m && up[ie] == up[ie + 1] {
            ie += 1;
        }
        let mlt = ie - i0 + 1;
        let r = p.saturating_sub(mlt);

        // ---- Insert knot locally (alpha blending) ----
        if mlt < p {
            let num = up[ie] - up[is_];
            // for i=p; i>mlt; i-- : fill alpha arrays
            for ii in (mlt + 1..=p).rev() {
                let den = up[is_ + ii] - up[is_];
                let a = if den.abs() <= ON_TOL14 { 0.0 } else { num / den };
                let idx = ii - mlt - 1;
                alfs[idx] = a;
                omas[idx] = 1.0 - a;
            }

            // for i=1..r
            for ins_i in 1..=r {
                let s = mlt + ins_i;
                let save = r - ins_i;

                // for j=p down to s
                for j in (s..=p).rev() {
                    let a = alfs[j - s];
                    let b = omas[j - s];

                    // A_comcpt(a,Qw[j], b,Qw[j-1], &Qw[j])
                    // => Qw[j] = a*Qw[j] + b*Qw[j-1]  (homogeneous blend)
                    qw[j] = Point4D {
                        x: a * qw[j].x + b * qw[j - 1].x,
                        y: a * qw[j].y + b * qw[j - 1].y,
                        z: a * qw[j].z + b * qw[j - 1].z,
                        w: a * qw[j].w + b * qw[j - 1].w,
                    };
                }

                // if (ie < m) NQw[save] = Qw[p]
                if ie < m {
                    if let Some(next) = right.get_mut(0) {
                        next.ctrl[save] = qw[p];
                    }
                }
            }
        }

        // ---- Set Bezier piece knot vector: [UP[is_]..] and [UP[ie]..] clamped ----
        for i in 0..=p {
            uq[i] = up[is_];
            uq[i + p + 1] = up[ie];
        }

        // ---- Segment completed: prepare next piece initial ctrl ----
        if ie < m {
            if let Some(next) = right.get_mut(0) {
                // for i=r..p: NQw[i] = Pw[ie-p+i]
                for i in r..=p {
                    let src = ie - p + i;
                    next.ctrl[i] = pw[src];
                }
            }
        }

        // advance
        is_ = ie;
        ie += 1;
        iq += 1;
        // safety: iq should never exceed nsp-1
        if iq >= nsp {
            break;
        }
    }

    Ok(pieces)
}
```

## 📘 3. derivative_curve_non_rational()
- ✔ 기능
    - 비(非)유리 NURBS 곡선의 d차 도함수 곡선을 생성한다.
- ✔ 수식
- Piegl & Tiller의 control point differencing 공식:
```math
P_i^{(k)}=\frac{p-(k-1)}{U_{i+p+1}-U_{i+k}}\left( P_{i+1}^{(k-1)}-P_i^{(k-1)}\right)
``` 
- 여기서
    - $P_i^{(0)}=P_i$
    - k = 1..d
    - p = degree
- ✔ 알고리즘 검증
    - Rust 코드:
```math
let alf = pk / denom;
tmp[i] = alf*(tmp[i+1] - tmp[i]);
```

- 이는 위 수식을 정확히 구현한 것이다.
- ✔ knot vector 재구성
    - d차 미분 후 degree는:
```math
p_d=p-d
```
- knot vector는 내부 knot를 d개씩 제거:
```math
V=[U_0^{(p_d+1)},U_{p+1},...,U_n,U_m^{(p_d+1)}]
```
- Rust 코드도 동일하게 구현되어 있다.
- ✔ 용도
    - 곡선의 기하학적 특성 분석
    - 곡률, 접선, 법선 계산
    - 고차 미분 기반의 CAD 기능
    - 곡선 평활화, 곡선 fitting

### 소스 코드
```rust
/// Symbolic derivative curve of a **non-rational** NURBS curve.
/// This matches the classic control-point differencing formula (Piegl/Tiller):
/// repeated application yields the d-th derivative curve.
pub fn derivative_curve_non_rational(&self, d: usize) -> Result<NurbsCurve> {
    if self.is_rational() {
        return Err(NurbsError::InvalidArgument { msg: "derivative_curve_non rational requires a non-rational curve".into() });
    }
    let p = self.degree as usize;
    if d > p {
        return Err(NurbsError::InvalidArgument { msg: "derivative order exceeds degree".into() });
    }

    let n = self.ctrl.len() - 1; // highest ctrl index
    let mut tmp = self.ctrl.clone();
    let u = self.kv.knots.as_slice();

    // k from 1..=d
    for k in 1..=d {
        let pk = (p - (k - 1)) as Real;
        for i in 0..=(n - k) {
            let denom = u[i + p + 1] - u[i + k];
            if denom == 0.0 {
                return Err(NurbsError::InvalidArgument { msg: "zero knot interval in derivative".into() });
            }
            let alf = pk / denom;
            // tmp[i] = alf*(tmp[i+1]-tmp[i])
            let a = tmp[i + 1] * alf;
            let b = tmp[i] * (-alf);
            tmp[i] = a + b;
        }
    }

    let nd = n - d;
    let pd = (p - d) as Degree;
    let mut ctrl = tmp[..=nd].to_vec();
    // make sure output is non-rational (w = 1)
    for c in &mut ctrl {
        c.w = 1.0;
    }

    // knot vector: drop first d and last d internal knots (clamped form)
    // Following N_SYMCND: V = [U[0] repeated (p-d+1), U[p+1..n] , U[m] repeated (p-d+1)]
    let m = self.kv.knots.len() - 1;
    let mut v = Vec::with_capacity(nd + (pd as usize) + 2);
    for _ in 0..=(p - d) { v.push(u[0]); }
    for i in (p + 1)..=n { v.push(u[i]); }
    for _ in 0..=(p - d) { v.push(u[m]); }
    let knots = KnotVector::new(v)?;

    // domain stays the same
    NurbsCurve::new(pd, ctrl, knots)
}
```

## 📘 4. extract_num_den()
- ✔ 기능
- Rational NURBS 곡선을 다음 두 개로 분리한다:
- Numerator curve
```math
C_{num}(u)=\sum N_i(u)(w_iP_i)
```

- Denominator function
```math
w(u)=\sum N_i(u)w_i
```
- ✔ 수식
    - Rational NURBS 곡선:
```math
C(u)=\frac{\sum N_i(u)w_iP_i}{\sum N_i(u)w_i}
```
- 따라서:
    - Numerator control point = $w_iP_i$
    - Denominator control point = $w_i$
- ✔ 검증
- Rust 코드:
```rust
num_ctrl.push(Point4D::non_homogeneous(p.x, p.y, p.z, 1.0));
den.push(p.w);
```

- 즉:
    - numerator는 (xw, yw, z*w, 1)
    - denominator는 weight만 저장
- ✔ 용도
    - Rational curve를 polynomial form으로 변환
    - 곡선-곡선 교차
    - implicitization
    - rational curve subdivision
    - 수치적 안정성 향상
 
### 소스 코드
```rust
/// Extract numerator curve (non-rational) and denominator function for a rational curve.
/// Numerator control points store (x*w, y*w, z*w) with w=1; denominator stores weights.
pub fn extract_num_den(&self) -> Result<(NurbsCurve, crate::core::cfun::CFun)> {
    if !self.is_rational() {
        return Err(NurbsError::InvalidArgument { msg: "extract_num_den requires a rational curve".into() });
    }
    let mut num_ctrl = Vec::with_capacity(self.ctrl.len());
    let mut den = Vec::with_capacity(self.ctrl.len());
    for p in &self.ctrl {
        num_ctrl.push(Point4D::non_homogeneous(p.x, p.y, p.z, 1.0));
        den.push(p.w);
    }
    let num = NurbsCurve::new(self.degree, num_ctrl, self.kv.clone())?;
    let den = CFun::new(self.degree, self.kv.clone().clone(), den)?;
    Ok((num, den))
}
```

## 🎯 전체 요약 테이블 (Markdown)
| Function                        | Formula / Principle                                          | Purpose |
|---------------------------------|--------------------------------------------------------------|---------|
| knot_nonzero_span_count()       | Count spans where U[i] ≠ U[i+1]                             | Bezier segment count, knot analysis |
| to_bezier_pieces_no_refine()    | Local knot insertion, Bezier extraction (N_TOOCDC)          | Bezier decomposition, tessellation |
| derivative_curve_non_rational() | Pᵢ^(k) = α(Pᵢ⁺¹^(k-1) − Pᵢ^(k-1)), α = (p−k+1)/(U[i+p+1]-U[i+k]) | Derivative curve generation |
| extract_num_den()               | C(u) = Num(u) / Den(u), Num = Σ Nᵢ wᵢ Pᵢ, Den = Σ Nᵢ wᵢ      | Rational → polynomial decomposition |

---
