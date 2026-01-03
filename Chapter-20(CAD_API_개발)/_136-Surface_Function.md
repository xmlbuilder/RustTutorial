# 📘 NURBS Surface Knot Derivative 문서
- Analytic vs Numeric (Finite Difference)

## 1. 문제 정의
- NURBS 표면 함수:
```math
F(u,v)=\sum _{i=0}^n\sum _{j=0}^mf_{ij}\, N_{i,p}(u)\, N_{j,q}(v)
```
- 여기서:
    - $N_{i,p}(u)$: U 방향 B-spline basis
    - $N_{j,q}(v)$: V 방향 B-spline basis
    - $f_{ij}$: scalar surface function coefficient
- 우리가 구하고 싶은 값:
```math
\frac{\partial F(u,v)}{\partial t_k}
```
- 여기서 t_k는 U 또는 V knot vector의 특정 knot 값이다.

## 2. Analytic Knot Derivative (N_sfndrk)
### 2.1 개념
- Piegl & Tiller는 knot derivative를 local support 영역만 고려하여 정의한다.
- 즉, knot t_k는 basis function 중 일부에만 영향을 주므로:
    - UDIR → $i\in [k-p-1,k]$
    - VDIR → $j\in [k-q-1,k]$
- 이 범위만 미분에 포함된다.

### 2.2 수식
- UDIR (U knot에 대한 미분)
```math
\frac{\partial F}{\partial t_k}=\sum _{i=k-p-1}^k\sum _{j=spn-q}^{spn}f_{ij}\left( \frac{\partial N_{i,p}(u)}{\partial t_k}\right) N_{j,q}(v)
```
- VDIR (V knot에 대한 미분)
```math
\frac{\partial F}{\partial t_k}=\sum _{i=spn-p}^{spn}\sum _{j=k-q-1}^kf_{ij}N_{i,p}(u)\left( \frac{\partial N_{j,q}(v)}{\partial t_k}\right)
``` 
- 여기서:
    - $\frac{\partial N}{\partial t_k}$ 는 on_compute_surface_basis_derivative_wrt_knot가 계산
    - spn은 v 방향 span index

### 2.3 특징
- 정확한 analytic 미분
- local support만 포함
- basis derivative wrt knot을 직접 계산
- Piegl 알고리즘과 100% 일치

## 3. Numeric Knot Derivative (Finite Difference)
### 3.1 개념
- knot t_k를 아주 조금 변화시키고:
```math
\frac{\partial F}{\partial t_k}\approx \frac{F(t_k+h)-F(t_k-h)}{2h}
```
- 이 방식은 전체 surface function이 변하므로 analytic과 다를 수 있다.

### 3.2 전체 surface function을 사용한 numeric
```math
F(u,v)=\sum _{i,j}f_{ij}N_{i,p}(u)N_{j,q}(v)
```
- 이걸 perturb하면:
    - 전체 basis가 변함
    - 전체 surface가 변함
    - analytic과 값이 다르게 나오는 것이 정상

### 3.3 analytic과 비교하려면 “local-only numeric”을 사용해야 함
- Piegl analytic과 동일한 영역만 포함:
```math
F_{\mathrm{local}}(u,v)=\sum _{i=k-p-1}^k\sum _{j=spn-q}^{spn}f_{ij}N_{i,p}(u)N_{j,q}(v)
```
- numeric 미분:
```math
\frac{\partial F_{\mathrm{local}}}{\partial t_k}\approx \frac{F_{\mathrm{local}}(t_k+h)-F_{\mathrm{local}}(t_k-h)}{2h}
```
- 이렇게 해야 analytic과 finite difference가 거의 동일해진다.


## 4. Analytic vs Numeric 비교

| 항목                     | Analytic (N_sfndrk)                          | Numeric (전체 surface)                       | Numeric (local-only)                         |
|--------------------------|-----------------------------------------------|-----------------------------------------------|-----------------------------------------------|
| 정의                     | Piegl 공식 기반 정확한 knot 미분             | 전체 surface function을 수치 미분             | Piegl과 동일한 local support만 수치 미분      |
| 포함 영역                | local support 영역만 포함                     | 전체 basis function 포함                      | local support 영역만 포함                     |
| 정확도                   | 매우 정확                                     | analytic과 다를 수 있음                       | analytic과 거의 동일                          |
| 계산 방식                | basis derivative wrt knot 직접 계산           | F(t_k+h) - F(t_k-h) / (2h)                    | F_local(t_k+h) - F_local(t_k-h) / (2h)        |
| 사용 목적                | knot removal, knot optimization, Jacobian     | 전체 surface 변화량 관찰                      | analytic 검증용                                |


## 5. Rust 함수 요약
### 5.1 Analytic
- on_surface_function_derivative_wrt_knot(...)
- local support만 포함

### 5.2 Numeric (전체 surface)
- on_eval_scalar_surface(...)
- 전체 surface function 평가
- analytic과 다를 수 있음

### 5.3 Numeric (local-only)
- on_eval_scalar_surface_local(...)
- analytic과 비교할 때 반드시 사용
- local support만 포함

## 6. 왜 analytic ≠ numeric 이었는가?
- local-only numeric을 사용하면 analytic과 거의 동일해진다.

## 7. 결론
- analytic(N_son_compute_surface_basis_derivative_wrt_knot)은 Piegl 공식에 따른 정확한 knot derivative
- numeric(finite diff)은 전체 surface를 미분하므로 analytic과 다를 수 있음
- analytic과 비교하려면 반드시 local-only numeric을 사용해야 함
- 이 두 방식의 차이를 이해하면 NURBS knot optimization, knot removal, fairing 등 고급 기능을 정확하게 구현할 수 있다



## 📘 on_rational_basis_function 수학적 의미
- on_rational_basis_function 는 Rational NURBS basis function $R_{i,j}(u,v)$ 을 평가하는 루틴.
- 즉, 가중치(w)와 B-spline basis(N)로 구성된 rational basis를 계산하는 함수.
- 주어진 surface function sfn은 분모(denominator) 를 나타낸다:
```math
\mathrm{den}(u,v)=\sum _{r,s}w_{r,s}N_{r,p}(u)N_{s,q}(v)
```
- 그리고 rational basis function은:
```math
R_{i,j}(u,v)=\frac{w_{i,j}N_{i,p}(u)N_{j,q}(v)}{\mathrm{den}(u,v)}
```
- 즉:
    - 분자(numerator) = weight × basis_u × basis_v
    - 분모(denominator) = surface function 평가값


## 📘 Surface Function Evaluation at a Single Point


### 1. 목적
- SFun eval은 다음 표면 함수:
```math
F(u,v)=\sum _{i=0}^n\sum _{j=0}^mf_{ij}N_{i,p}(u)N_{j,q}(v)
```
- 을 단일 파라미터 (u,v) 에서 평가한다.
    - $fuv[i][j]$ : surface function coefficient
    - $N_{i,p}(u)$ : U 방향 B-spline basis
    - $N_{j,q}(v)$ : V 방향 B-spline basis

### 2. 수학적 과정
- U 방향 span 찾기
```math
\mathrm{usp}=\mathrm{find\_ span}(u)
```
- V 방향 span 찾기
```math
\mathrm{vsp}=\mathrm{find\_ span}(v)
```
- U 방향 basis 계산
```math
NU[i]=N_{usp-p+i,p}(u)
```
- V 방향 basis 계산
```math
NV[j]=N_{vsp-q+j,q}(v)
```
- 중간 합 tu[i] 계산
```math
tu[i]=\sum _{j=0}^qNV[j]\cdot f_{usp-p+i,\; vsp-q+j}
```
- 최종 surface function 값
```math
F(u,v)=\sum _{i=0}^pNU[i]\cdot tu[i]
```

### 소스 코드
```rust

#[derive(Debug, Clone)]
pub struct SFun {
    pub pu: Degree,
    pub pv: Degree,
    pub nu: Index,         // # of control points in U (count)
    pub nv: Index,         // # of control points in V (count)
    pub values: Vec<Real>, // row-major: idx = i + nu * j
    pub ku: KnotVector,    // U knots
    pub kv: KnotVector,    // V knots
}
impl SFun {
    pub fn deg(&self) -> (Degree, Degree) {
        (self.pu, self.pv)
    }
    pub fn indices(&self) -> (Index, Index, Index, Index) {
        let n = self.nu - 1;
        let m = self.nv - 1;
        let r = self.ku.len() as Index - 1;
        let s = self.kv.len() as Index - 1;
        (n, m, r, s)
    }

    #[inline]
    pub fn values(&self) -> &[Real] {
        &self.values
    }

    /// (i,j) 셀 접근 (row-major: i + nu * j)
    #[inline]
    pub fn idx(&self, i: usize, j: usize) -> usize {
        i + (self.nu as usize) * j
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize) -> Real {
        self.values[self.idx(i, j)]
    }

    #[inline]
    pub fn set(&mut self, i: usize, j: usize, val: Real) {
        let k = self.idx(i, j);
        self.values[k] = val;
    }

    /// 외부 버퍼 그대로 채택 (values: row-major, 길이는 nu*nv 이어야 함)
    pub fn set_storage(
        &mut self,
        values: Vec<Real>,
        nu: Index,
        nv: Index,
        u_vec: Vec<Real>,
        v_vec: Vec<Real>,
        pu: Degree,
        pv: Degree,
    ) {
        debug_assert_eq!(values.len(), (nu as usize) * (nv as usize));
        self.values = values;
        self.nu = nu;
        self.nv = nv;
        self.ku.knots = u_vec;
        self.kv.knots = v_vec;
        self.pu = pu;
        self.pv = pv;
    }

    /// 2D 그리드에서 채택 (grid[i][j]) → row-major로 변환
    pub fn set_storage_from_grid(
        &mut self,
        grid: Vec<Vec<Real>>,
        u_vec: Vec<Real>,
        v_vec: Vec<Real>,
        pu: Degree,
        pv: Degree,
    ) {
        let nu = grid.len() as Index;
        let nv = if nu > 0 { grid[0].len() as Index } else { 0 };
        self.values.clear();
        self.values.reserve((nu as usize) * (nv as usize));
        for j in 0..(nv as usize) {
            for i in 0..(nu as usize) {
                self.values.push(grid[i][j]);
            }
        }
        self.nu = nu;
        self.nv = nv;
        self.ku.knots = u_vec;
        self.kv.knots = v_vec;
        self.pu = pu;
        self.pv = pv;
    }

    /// 깊은 복사 (dst 크기 자동 보정)
    pub fn copy_into(&self, dst: &mut SFun) {
        let (n, m, r, s) = self.indices();
        let (p, q) = self.deg();
        ensure_sfun_shape(dst, n, m, p, q, r, s);
        dst.values.clone_from(&self.values);
        dst.ku.knots.clone_from(&self.ku.knots);
        dst.kv.knots.clone_from(&self.kv.knots);
    }

    /// 압축(연속 메모리 재확보) — row-major라 실제로는 clone으로 충분
    pub fn compact(&mut self) {
        self.values = self.values.clone();
        self.ku.knots = self.ku.knots.clone();
        self.kv.knots = self.kv.knots.clone();
    }

    /// Knot들을 직사각형 [a,b]x[c,d]에 재매핑
    pub fn rescale_knots(&mut self, rect: Rectangle, dir: SurfaceDir) {
        let (p, q) = self.deg();
        let (_, _, r, s) = self.indices();
        let (ul, ur, vb, vt) = (rect.ul, rect.ur, rect.vb, rect.vt);
        let u_vec = &mut self.ku.knots;
        let v_vec = &mut self.kv.knots;

        match dir {
            SurfaceDir::UDir => {
                if !u_vec.is_empty() && ul != u_vec[0] || ur != u_vec[r as usize] {
                    let u0 = u_vec[0];
                    let u1 = u_vec[r as usize];
                    let len = u1 - u0;
                    let fac = if len != 0.0 { (ur - ul) / len } else { 0.0 };

                    // 좌측 클램프 p+1개
                    for i in 0..=p as usize {
                        u_vec[i] = ul;
                    }

                    // 내부만 선형 스케일
                    let first_in = (p as usize).saturating_add(1);
                    let last_in = (r as usize).saturating_sub(p as usize + 1);
                    if first_in <= last_in && fac != 0.0 {
                        for i in first_in..=last_in {
                            u_vec[i] = fac * (u_vec[i] - u0) + ul;
                        }
                    }

                    // 우측 클램프 p+1개
                    for i in (r as usize - p as usize)..=r as usize {
                        u_vec[i] = ur;
                    }
                }
            }
            SurfaceDir::VDir => {
                if !v_vec.is_empty() && vb != v_vec[0] || vt != v_vec[s as usize] {
                    let v0 = v_vec[0];
                    let v1 = v_vec[s as usize];
                    let len = v1 - v0;
                    let fac = if len != 0.0 { (vt - vb) / len } else { 0.0 };

                    // 하단 클램프 q+1개
                    for j in 0..=q as usize {
                        v_vec[j] = vb;
                    }

                    // 내부만 선형 스케일
                    let first_in = (q as usize).saturating_add(1);
                    let last_in = (s as usize).saturating_sub(q as usize + 1);
                    if first_in <= last_in && fac != 0.0 {
                        for j in first_in..=last_in {
                            v_vec[j] = fac * (v_vec[j] - v0) + vb;
                        }
                    }

                    // 상단 클램프 q+1개
                    for j in (s as usize - q as usize)..=s as usize {
                        v_vec[j] = vt;
                    }
                }
            }
        }
    }

    /// C의 UKILSFN: 내부 버퍼 해제 + 메타 리셋
    pub fn clear(&mut self) {
        self.values.clear(); // f(u,v) 그리드(1D row-major) 비움
        self.ku.knots.clear(); // U knot 비움
        self.kv.knots.clear(); // V knot 비움
        self.pu = 0;
        self.pv = 0;
        self.nu = 0;
        self.nv = 0;
    }

    pub fn resize_grid(&mut self, new_nu: usize, new_nv: usize) {
        on_ral_f2d_row_major(
            &mut self.values,
            self.nu as usize,
            self.nv as usize,
            new_nu,
            new_nv,
            0.0,
        );
        self.nu = new_nu as Index;
        self.nv = new_nv as Index;
    }

    pub fn eval_scalar(&self, u: Real, v: Real) -> Real {
        let pu = self.pu as usize;
        let pv = self.pv as usize;

        let nu_pts = self.nu as usize;
        let nv_pts = self.nv as usize;
        let nu = nu_pts.saturating_sub(1);
        let nv = nv_pts.saturating_sub(1);

        let su = on_find_span_index(nu, pu as u16, u, &self.ku.knots);
        let sv = on_find_span_index(nv, pv as u16, v, &self.kv.knots);

        let mut n_u = vec![0.0; pu + 1];
        let mut n_v = vec![0.0; pv + 1];
        on_basis_funs(su, u, pu as u16, &self.ku.knots, &mut n_u);
        on_basis_funs(sv, v, pv as u16, &self.kv.knots, &mut n_v);

        let iu0 = (su - pu) as usize;
        let jv0 = (sv - pv) as usize;

        let mut val = 0.0;
        for l in 0..n_v.len() {
            for k in 0..n_u.len() {
                let i = iu0 + k;
                let j = jv0 + l;
                val += n_u[k] * n_v[l] * self.values[i + nu_pts * j];
            }
        }
        val
    }

    /// Piegl N_sfnevn()과 동일한 평가 함수
    /// - u, v: 파라미터
    /// - side_u, side_v: LEFT/RIGHT (현재는 span 계산에 영향 없음 → on_find_span_index 사용)
    #[inline]
    pub fn eval(&self, u: Real, v: Real) -> Real {
        self.eval_scalar(u, v)
    }

    /// side 버전 (호환용)
    #[inline]
    pub fn eval_side(&self, u: Real, v: Real, _su: Side, _sv: Side) -> Real {
        // 현재 SFun은 side에 따른 span-left/right 구분을 하지 않음.
        // Piegl의 N_sfnevn도 기본적으로 LEFT 사용.
        self.eval_scalar(u, v)
    }
}

#[inline]
pub fn ensure_sfun_shape(
    out: &mut SFun,
    n: Index,
    m: Index, // 마지막 인덱스 (→ 개수는 +1)
    p: Degree,
    q: Degree,
    r: Index,
    s: Index, // knot 마지막 인덱스 (→ 길이는 +1)
) {
    // 1) value 버퍼 크기 보장 (row-major: nu * nv)
    let nu = n + 1;
    let nv = m + 1;
    let need = (nu as usize) * (nv as usize);

    if out.nu != nu || out.nv != nv || out.values.len() != need {
        out.nu = nu;
        out.nv = nv;
        out.values.resize(need, 0.0);
    }

    // 2) 차수 갱신
    out.pu = p;
    out.pv = q;

    // 3) knot 길이 보장
    let rr = (r as usize) + 1;
    let ss = (s as usize) + 1;

    if out.ku.len() != rr {
        out.ku.resize_len(rr, 0.0);
    }
    if out.kv.len() != ss {
        out.kv.resize_len(ss, 0.0);
    }
}

#[inline]
fn idx_nu(nu: usize, i: usize, j: usize) -> usize {
    i + nu * j
}

impl fmt::Display for SFun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prec = 6;
        let max_knots = 10;
        let max_u = 6;
        let max_v = 6;

        let (p, q) = (self.pu, self.pv);
        let (nu, nv) = (self.nu as usize, self.nv as usize);
        writeln!(f, "SFun {{")?;
        writeln!(f, "  degree: (pu={}, pv={})", p, q)?;
        writeln!(f, "  size  : (nu={}, nv={})", nu, nv)?;

        write!(f, "  ku    : ")?;
        fmt_slice(f, &self.ku.knots, max_knots, prec)?;
        writeln!(f)?;

        write!(f, "  kv    : ")?;
        fmt_slice(f, &self.kv.knots, max_knots, prec)?;
        writeln!(f)?;

        writeln!(
            f,
            "  values: row-major (i + nu*j), showing up to {}x{} entries",
            max_u, max_v
        )?;
        let mu = nu.min(max_u);
        let mv = nv.min(max_v);
        for j in 0..mv {
            write!(f, "    v[{j}] ")?;
            for i in 0..mu {
                let k = idx_nu(nu, i, j);
                write!(f, "{:.*} ", prec, self.values[k])?;
            }
            if mu < nu {
                write!(f, "...")?;
            }
            writeln!(f)?;
        }
        if mv < nv {
            writeln!(f, "    ...")?;
        }

        write!(f, "}}")
    }
}

pub fn dump_sfun_limited(fv: &SFun, max_u: usize, max_v: usize, knot_max: usize, prec: usize) {
    println!("SFun:");
    println!("  degree: (pu={}, pv={})", fv.pu, fv.pv);
    println!("  size  : (nu={}, nv={})", fv.nu, fv.nv);

    // knots
    {
        let mut buf = String::new();
        let _ = fmt_slice(&mut buf, &fv.ku.knots, knot_max, prec);
        println!("  ku    : {}", buf);
    }
    {
        let mut buf = String::new();
        let _ = fmt_slice(&mut buf, &fv.kv.knots, knot_max, prec);
        println!("  kv    : {}", buf);
    }

    // values
    let nu = fv.nu as usize;
    let nv = fv.nv as usize;
    let mu = nu.min(max_u);
    let mv = nv.min(max_v);
    println!(
        "  values: row-major (i + nu*j), showing up to {}x{}",
        mu, mv
    );
    for j in 0..mv {
        print!("    v[{j}] ");
        for i in 0..mu {
            let k = idx_nu(nu, i, j);
            print!("{:.*} ", prec, fv.values[k]);
        }
        if mu < nu {
            print!("...");
        }
        println!();
    }
    if mv < nv {
        println!("    ...");
    }
}

#[inline]
pub fn dump_sfun(fv: &SFun) {
    dump_sfun_limited(fv, 6, 6, 10, 6);
}

/// SFun(values/ku/kv)를 (n,m,r,s, p,q) 관례로 리사이즈
pub fn sfun_ensure_shape(
    f: &mut SFun,
    n: Index,
    m: Index,
    p: Degree,
    q: Degree,
    r: Index,
    s: Index,
    fill: Real,
) {
    let new_nu = (n as usize) + 1;
    let new_nv = (m as usize) + 1;

    on_ral_f2d_row_major(
        &mut f.values,
        f.nu as usize,
        f.nv as usize,
        new_nu,
        new_nv,
        fill,
    );
    f.nu = new_nu as Index;
    f.nv = new_nv as Index;

    f.pu = p;
    f.pv = q;

    let ru = (r as usize) + 1;
    let rv = (s as usize) + 1;
    f.ku.knots.resize(ru, 0.0);
    f.kv.knots.resize(rv, 0.0);
}


pub fn on_eval_sfun_scalar(sfn: &SFun, u: Real, v: Real) -> Real {
    let p = sfn.pu;
    let q = sfn.pv;

    let nu = sfn.nu as usize; // = n+1
    let nv = sfn.nv as usize; // = m+1

    let ku = sfn.ku.knots.as_slice();
    let kv = sfn.kv.knots.as_slice();

    // spans
    let span_u = on_find_span_left(&sfn.ku, p, u as Param).expect("invalid knots");
    let span_v = on_find_span_left(&sfn.kv, q, v as Param).expect("invalid knots");

    let nu_basis = on_basis_func_ret_vec(ku, span_u, u, p as usize); // len p+1
    let nv_basis = on_basis_func_ret_vec(kv, span_v, v, q as usize); // len q+1

    // local control index ranges
    let i0 = span_u - p as Index; // 0..nu-1
    let j0 = span_v - q as Index; // 0..nv-1

    // tensor product sum
    let mut s = 0.0;
    for (a, &Nu) in nu_basis.iter().enumerate() {
        let ii = i0 + a;
        for (b, &Nv) in nv_basis.iter().enumerate() {
            let jj = j0 + b;
            // row-major: i + nu*j
            let idx = ii + nu * jj;
            let fuv = sfn.values[idx];
            s += Nu * Nv * fuv;
        }
    }
    s
}
```


---


## 📘Surface Function Grid Evaluation
- on_surface_function_eval_grid는 다음 표면 함수:
```math
F(u,v)=\sum _{i=0}^n\sum _{j=0}^mf_{ij}\, N_{i,p}(u)\, N_{j,q}(v)
```
- 을 여러 개의 u-grid, v-grid에서 동시에 평가하는 루틴이다.
- 즉:
    - 단일 점 평가: SFun::eval
    - 그리드 전체 평가: on_surface_function_eval_grid

### 🎯 1. 입력
- u[0..n]: 증가하는 u-파라미터 샘플
- v[0..m]: 증가하는 v-파라미터 샘플
- $f_{ij}$: surface function coefficient
- p,q: B-spline degree
- U,V: knot vectors

### 🎯 2. 출력
```math
F[k][l]=F(u_k,v_l)
```
- 즉,
    - u-grid × v-grid 전체에 대해 surface function 값을 계산한 2D 배열.

### 🎯 3. 수학적 의미
- on_surface_function_eval_grid는 다음을 계산한다:
```math
F(k,l)=F(u_k,v_l)
```
```math
F(u_k,v_l)=\sum _{i=0}^n\sum _{j=0}^mf_{ij}\, N_{i,p}(u_k)\, N_{j,q}(v_l)
```
- 즉:
    - u 방향 basis $N_{i,p}(u_k)$
    - v 방향 basis $N_{j,q}(v_l)$
    - $fuv[i][j]$
- 이 세 가지를 모두 곱해서 합산한 값

### 🎯 4. Piegl 방식의 계산 구조
- Piegl은 계산 효율을 위해 다음과 같이 분리한다.
    - Step 1 — u 방향 basis 미리 계산
    ```math
    NU[k][i]=N_{i,p}(u_k)
    ```
    - Step 2 — v 방향 basis 미리 계산
    ```math
    NV[l][j]=N_{j,q}(v_l)
    ```
    - Step 3 — 표면 함수 계산
        - 각 grid (k,l)에 대해:
    ```math
    F[k][l]=\sum _{i=0}^pNU[k][i]\cdot \left( \sum _{j=0}^qNV[l][j]\cdot f_{(usp[k]-p+i),(vsp[l]-q+j)}\right)
    ```
    - 여기서:
        - usp[k] = u_k의 span index
        - vsp[l] = v_l의 span index
        - 즉, local support만 사용한다.

### 🎯 5. on_surface_function_eval_grid 의 핵심 아이디어
- ✔ 1. B-spline은 local support
    - 전체 fuv를 다 쓰지 않고,
    - p+1 × q+1 개만 사용한다.
- ✔ 2. basis를 미리 계산해두면
    - grid 전체를 매우 빠르게 계산할 수 있다.
- ✔ 3. surface function은 tensor-product 구조
    - u 방향과 v 방향을 분리해서 계산 가능
    - CPU 캐시 효율이 매우 좋다.

### 🎯 6. Rust 포팅 버전의 수학적 의미
- Rust 함수:
```math
F[k][l] = Σ_i Σ_j fuv[a][b] * NU[k][i] * NV[l][j]
```

- 여기서:
    - a = usp[k] - p + i
    - b = vsp[l] - q + j
- 즉,
    - 정확히 Piegl의 local tensor-product 구조를 그대로 구현한 것이다.

## 🎉 최종 요약
- on_surface_function_eval_grid 는 다음을 수행하는 루틴이다:
```math
F[k][l]=\sum _{i=0}^p\sum _{j=0}^qf_{(usp[k]-p+i),(vsp[l]-q+j)}\cdot N_{i,p}(u_k)\cdot N_{j,q}(v_l)
```
- 즉:
    - 표면 함수 F(u,v)를 grid 전체에서 평가
    - basis를 미리 계산하여 효율적으로 수행
    - local support만 사용하여 빠르게 계산
    - tensor-product 구조를 그대로 반영한 공식

---

## 소스 코드
```rust
pub fn on_surface_function_eval_grid(
    fuv: &[Vec<f64>],
    knu: &KnotVector,
    knv: &KnotVector,
    p: usize,
    q: usize,
    u_vals: &[f64],
    v_vals: &[f64],
) -> crate::core::prelude::Result<Vec<Vec<f64>>> {
    let nu = fuv.len();
    if nu == 0 {
        return Err(NurbsError::DimensionMismatch { msg: "empty fuv" });
    }
    let mv = fuv[0].len();

    let n = u_vals.len().saturating_sub(1);
    let m = v_vals.len().saturating_sub(1);

    if n == usize::MAX || m == usize::MAX {
        return Err(NurbsError::DimensionMismatch { msg: "invalid grid size" });
    }

    // 결과 F[k][l]
    let mut F = vec![vec![0.0; m + 1]; n + 1];

    let u_knots = knu.as_slice();
    let v_knots = knv.as_slice();

    // Precompute basis for all u and v
    let mut NU = Vec::with_capacity(n + 1);
    let mut usp = Vec::with_capacity(n + 1);

    for &u in u_vals {
        let span_u = u_knots.find_span(nu - 1, p, u);
        let Nu = u_knots.basis_funs(span_u, u, p);
        NU.push(Nu);
        usp.push(span_u);
    }

    let mut NV = Vec::with_capacity(m + 1);
    let mut vsp = Vec::with_capacity(m + 1);

    for &v in v_vals {
        let span_v = v_knots.find_span(mv - 1, q, v);
        let Nv = v_knots.basis_funs(span_v, v, q);
        NV.push(Nv);
        vsp.push(span_v);
    }

    // Compute grid
    for k in 0..=n {
        for l in 0..=m {
            let span_u = usp[k];
            let span_v = vsp[l];

            let Nu = &NU[k];
            let Nv = &NV[l];

            let mut sum = 0.0;

            for i in 0..=p {
                let a = span_u - p + i;
                if a >= nu { continue; }

                let mut T = 0.0;
                for j in 0..=q {
                    let b = span_v - q + j;
                    if b >= mv { continue; }

                    T += Nv[j] * fuv[a][b];
                }

                sum += Nu[i] * T;
            }

            F[k][l] = sum;
        }
    }

    Ok(F)
}
```
---



