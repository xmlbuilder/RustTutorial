# Geometry Derivatives

## 1) `on_ev_normal_partials` — 법선의 편미분 $\(N_s, N_t\)$

곡면의 1·2차 편미분 $\(ds, dt, dss, dst, dtt\)$ 가 주어졌을 때 단위 법선  

$$
N(u,v)=\frac{V}{\|V\|},\quad V = ds \times dt
$$

의 $\(u,v\)$ -방향 미분을 계산합니다.

$$
N'=\frac{V'}{\|V\|}-\frac{V\cdot V'}{\|V\|^3}\,V,\qquad
V_s = dss\times dt + ds\times dst,\quad
V_t = dst\times dt + ds\times dtt.
$$

야코비안 $\(EG-F^2\)$ 가 수치적으로 퇴화하면(거의 0) **None**을 반환합니다.

**Plain text**
```
V = ds × dt,  N = V / |V|
Ns = (1/|V|)*Vs - ((V·Vs)/|V|^3)*V,  Vs = dss×dt + ds×dst
Nt = (1/|V|)*Vt - ((V·Vt)/|V|^3)*V,  Vt = dst×dt + ds×dtt
degenerate if EG - F^2 ≈ 0
```

### 수식
```rust
pub fn on_ev_normal_partials(
    ds: Vector3D, dt: Vector3D,
    dss: Vector3D, dst: Vector3D, dtt: Vector3D,
) -> Option<(Vector3D, Vector3D)> {
    // 기존: on_ev_jacobian(...) -> 과도한 false 가능
    // 변경: 직접 교차노름 기반 신뢰성 판정
    if !jacobian_reliable(ds, dt) {
        return None;
    }

    let v = ds.cross(&dt);
    let len = v.length();
    if len < 1e-15 {
        return None;
    }
    let len3 = len * len * len;

    // Vs = dss×dt + ds×dst, Vt = dst×dt + ds×dtt
    let vs = dss.cross(&dt) + ds.cross(&dst);
    let vt = dst.cross(&dt) + ds.cross(&dtt);

    // N' = Vs/|V| - ((V·Vs)/|V|^3) V  (V=ds×dt)
    let ns = (1.0/len) * vs - (v.dot(&vs)/len3) * v;
    let nt = (1.0/len) * vt - (v.dot(&vt)/len3) * v;

    Some((ns, nt))
}

```

### 사용법
```rust
#[test]
    fn normal_partials_fd_check() {
        fn derivs(u: f64, v: f64) -> (Vector3D, Vector3D, Vector3D, Vector3D, Vector3D, Vector3D) {
            // S(u,v) = (u, v, u^2+v^2)
            let s = Vector3D::new(u, v, u*u + v*v);
            let su  = Vector3D::new(1.0, 0.0, 2.0*u);
            let sv  = Vector3D::new(0.0, 1.0, 2.0*v);
            let suu = Vector3D::new(0.0, 0.0, 2.0);
            let suv = Vector3D::new(0.0, 0.0, 0.0);
            let svv = Vector3D::new(0.0, 0.0, 2.0);
            (s, su, sv, suu, suv, svv)
        }
        fn normal(u: f64, v: f64) -> Vector3D {
            let (_s, su, sv, _suu, _suv, _svv) = derivs(u,v);
            unit(su.cross(&sv))
        }
        let (u0, v0) = (1.0, 2.0);
        let (_s, su, sv, suu, suv, svv) = derivs(u0, v0);

        // 이론: ns, nt
        let (ns, nt) = on_ev_normal_partials(su, sv, suu, suv, svv)
            .unwrap_or_else(|| panic!("normal partials failed"));

        // 유한차 근사
        let h = 1e-6;
        let n0 = normal(u0, v0);
        let nu = normal(u0 + h, v0);
        let nv = normal(u0, v0 + h);
        let ns_fd = (nu - n0) / h;
        let nt_fd = (nv - n0) / h;

        assert!(approx_vec(&ns, &ns_fd, 5e-4), "ns {:?} vs fd {:?}", ns, ns_fd); // 비교적 관대한 오차
        assert!(approx_vec(&nt, &nt_fd, 5e-4), "nt {:?} vs fd {:?}", nt, nt_fd);
    }
```

---

## 2) `pullback_3d_vector` — 3D 벡터의 (u,v) 성분 분해

3D 벡터 $\(\mathbf{w}\)$ 를 $\(\mathbf{w}\approx \alpha\,ds+\beta\,dt\)$ 로 분해합니다.  
오프셋 거리 $\(d\neq 0\)$ 이면 법선 편미분으로 기저를 보정합니다.

$$
\mathbf{w}\approx
\begin{cases}
\alpha\,ds+\beta\,dt, & d=0\\[2pt]
\alpha\,(ds+d\,N_s)+\beta\,(dt+d\,N_t), & d\neq 0
\end{cases}
$$

$\((\alpha,\beta)\)$ 는 2×2 선형계로 풉니다.

**Plain text**
```
w ≈ α ds + β dt                (d = 0)
w ≈ α (ds + d Ns) + β (dt + d Nt)   (d ≠ 0)
solve 2×2 for (α, β)
```

### 소스
```rust
pub fn pullback_3d_vector(
    vector: Vector3D,
    distance: f64,     // Du×Dv 기준으로 위(+) / 아래(-) 부호
    ds: Vector3D, dt: Vector3D,
    dss: Vector3D, dst: Vector3D, dtt: Vector3D,
) -> Option<(f64, f64)> {
    if distance != 0.0 {
        // ns, nt 계산 후 보정 기저로 분해
        let (ns, nt) = on_ev_normal_partials(ds, dt, dss, dst, dtt)?;
        let bs = ds + ns * distance;
        let bt = dt + nt * distance;
        on_decompose_vector(vector, bs, bt)
    } else {
        // 그냥 ds, dt로 분해
        on_decompose_vector(vector, ds, dt)
    }
}

```

### 사용법
```rust

```
#[test]
    fn pullback_basic_and_offset() {
        // paraboloid at (u,v) = (0.5, -0.3)
        let u = 0.5;
        let v = -0.3;
        let ds  = Vector3D::new(1.0, 0.0, 2.0*u);
        let dt  = Vector3D::new(0.0, 1.0, 2.0*v);
        let dss = Vector3D::new(0.0, 0.0, 2.0);
        let dst = Vector3D::new(0.0, 0.0, 0.0);
        let dtt = Vector3D::new(0.0, 0.0, 2.0);

        // 임의 (α, β)
        let a = 1.2;
        let b = -0.7;
        let w = ds*a + dt*b;

        // d=0
        let (aa, bb) = pullback_3d_vector(w, 0.0, ds, dt, dss, dst, dtt)
            .or_else(|| tests_support::on_decompose_vector(w, ds, dt))
            .unwrap();
        assert!(approx(aa, a, 1e-12) && approx(bb, b, 1e-12));

        // d ≠ 0 (오프셋 보정) — ns, nt로 보정한 기저에서 동일 α,β를 기대
        let d = 0.1;
        let (ns, nt) = on_ev_normal_partials(ds, dt, dss, dst, dtt).unwrap();
        let w2 = (ds + ns*d)*a + (dt + nt*d)*b;
        let (pa, pb) = pullback_3d_vector(w2, d, ds, dt, dss, dst, dtt).unwrap();
        assert!(approx(pa, a, 1e-8) && approx(pb, b, 1e-8));
    }
---

## 3) `get_parameter_tolerance` — 파라미터 공차

도메인 $\([t_0,t_1]\)$ 안의 $\(t\)$ 주위에서 안전한 공차 $\([t^- , t^+]\)$ 를 추정합니다.

$$
\Delta t = 8\sqrt{\varepsilon}(t_1-t_0) + \varepsilon(|t_0|+|t_1|),\qquad
\Delta t \le \tfrac12(t_1-t_0),\quad
[t^- , t^+] = [t-\Delta t,\, t+\Delta t].
$$

**Plain text**
```
dt = 8*sqrt(eps)*(t1 - t0) + eps*(|t0| + |t1|)
dt = min(dt, 0.5*(t1 - t0))
[t- , t+] = [t - dt, t + dt]
```

### 소스
```rust
pub fn get_parameter_tolerance(t0: f64, t1: f64, mut t: f64) -> Option<(f64, f64)> {
    if !(t0 < t1) { return None; }
    if t < t0 { t = t0; } else if t > t1 { t = t1; }

    let eps = f64::EPSILON;
    let sqrt_eps = eps.sqrt();
    let mut dt = (t1 - t0)*8.0*sqrt_eps + (t0.abs() + t1.abs())*eps;
    if dt >= (t1 - t0) { dt = 0.5*(t1 - t0); }

    Some((t - dt, t + dt))
}

```

### 사용법
```rust

```
#[test]
    fn pullback_basic_and_offset() {
        // paraboloid at (u,v) = (0.5, -0.3)
        let u = 0.5;
        let v = -0.3;
        let ds  = Vector3D::new(1.0, 0.0, 2.0*u);
        let dt  = Vector3D::new(0.0, 1.0, 2.0*v);
        let dss = Vector3D::new(0.0, 0.0, 2.0);
        let dst = Vector3D::new(0.0, 0.0, 0.0);
        let dtt = Vector3D::new(0.0, 0.0, 2.0);

        // 임의 (α, β)
        let a = 1.2;
        let b = -0.7;
        let w = ds*a + dt*b;

        // d=0
        let (aa, bb) = pullback_3d_vector(w, 0.0, ds, dt, dss, dst, dtt)
            .or_else(|| tests_support::on_decompose_vector(w, ds, dt))
            .unwrap();
        assert!(approx(aa, a, 1e-12) && approx(bb, b, 1e-12));

        // d ≠ 0 (오프셋 보정) — ns, nt로 보정한 기저에서 동일 α,β를 기대
        let d = 0.1;
        let (ns, nt) = on_ev_normal_partials(ds, dt, dss, dst, dtt).unwrap();
        let w2 = (ds + ns*d)*a + (dt + nt*d)*b;
        let (pa, pb) = pullback_3d_vector(w2, d, ds, dt, dss, dst, dtt).unwrap();
        assert!(approx(pa, a, 1e-8) && approx(pb, b, 1e-8));
    }
---

## 4) `on_ev_normal` — 단위 법선 $\(N\)$

기본은 $\(N \propto Du\times Dv\)$. 야코비안이 퇴화하면 2차 편미분으로 극한 방향을 근사합니다.

$$
A=Du\times Duv + Duu\times Dv,\qquad
B=Du\times Dvv + Duv\times Dv,
$$


$$
\text{dir} \sim aA+bB,\ \ (a,b)\in\{\pm1\}\ \text{(사분면에 따라)}.
$$

수치적으로는

$$
aA+bB = Du\times(a\,Duv+b\,Dvv)\;-\;Dv\times(a\,Duu+b\,Duv)
$$

가 안정적입니다.

**Plain text**
```
If EG - F^2 not degenerate: N = unit(Du × Dv)
Else use: dir ≈ Du×(a Duv + b Dvv) − Dv×(a Duu + b Duv), then normalize.
```

### 수식
```rust
pub fn on_ev_normal(
    limit_dir: i32,
    du: Vector3D, dv: Vector3D,
    duu: Vector3D, duv: Vector3D, dvv: Vector3D,
) -> Option<Vector3D> {
    // 정상 케이스: 바로 Du×Dv 사용
    if jacobian_reliable(du, dv) {
        let mut n = du.cross(&dv);
        if n.normalize() { return Some(n); }
        return None;
    }

    // 퇴화 케이스: 2차 편미분으로 극한 방향
    if !(is_valid(&duu) && is_valid(&duv) && is_valid(&dvv)) {
        return None;
    }
    let (a, b) = match limit_dir {
        2 => (-1.0,  1.0),
        3 => (-1.0, -1.0),
        4 => ( 1.0, -1.0),
        _ => ( 1.0,  1.0),
    };

    // aA+bB = Du×(aDuv+bDvv) − Dv×(aDuu+bDuv)
    let v1 = du.cross(&(duv * a + dvv * b));
    let v2 = dv.cross(&(duu * a + duv * b));
    let mut n = v1 - v2;
    if n.normalize() { Some(n) } else { None }
}

```

### 사용법
```rust
#[test]
    fn ev_normal_regular_and_degenerate() {
        // 정규 케이스: N = unit(Du × Dv)
        let du = Vector3D::new(1.0, 0.0, 2.0);
        let dv = Vector3D::new(0.0, 1.0, -1.0);
        let n_expected = unit(du.cross(&dv));
        let n = on_ev_normal(1, du, dv, Vector3D::zero(), Vector3D::zero(), Vector3D::zero()).unwrap();
        assert!(approx_vec(&n, &n_expected, 1e-12));

        // 퇴화 케이스: Du × Dv = 0, 2차 편미분으로 근사 방향 반환
        let du = Vector3D::new(0.0, 0.0, 0.0);
        let dv = Vector3D::new(1.0, 0.0, 0.0);
        let duu = Vector3D::new(0.0, 1.0, 0.0);
        let duv = Vector3D::new(0.0, 0.0, 1.0);
        let dvv = Vector3D::new(0.0, 0.0, 0.0);
        let n = on_ev_normal(2, du, dv, duu, duv, dvv)
            .expect("should produce a finite normal direction");
        assert!(approx(n.length(), 1.0, 1e-12));
    }
```

---

## 5) `on_ev_tangent` — 단위 접선 \(T\)

일반적으로

$$
T=\frac{D1}{\|D1\|}.
$$

만약 $\(D1=0,\,D2\ne 0\)$ 이면 로피탈에 의해

$$
T=\pm\frac{D2}{\|D2\|}.
$$

**Plain text**
```
If |D1|>0: T = D1 / |D1|
Else if |D2|>0: T = ± D2 / |D2|  (sign from limit)
Else: undefined
```

### 소스
```rust
pub fn on_ev_tangent(d1: Vector3D, d2: Vector3D) -> Option<Vector3D> {
    let mut l1 = d1.length();
    if l1 == 0.0 {
        l1 = d2.length();
        if l1 > 0.0 {
            let mut t = d2 / l1;
            // 부호는 s→0에서 D1·D2의 부호에 따르지만, 여기서는 방향만 반환
            return Some(t);
        } else {
            return None;
        }
    } else {
        let mut t = d1 / l1;
        return Some(t);
    }
}
```

### 사용법
```rust
   #[test]
    fn ev_tangent_basic_and_lhopital() {
        // 일반: T = D1/|D1|
        let d1 = Vector3D::new(2.0, -2.0, 0.0);
        let d2 = Vector3D::new(0.0, 0.0, 1.0);
        let t = on_ev_tangent(d1, d2).unwrap();
        assert!(approx_vec(&t, &unit(Vector3D::new(1.0, -1.0, 0.0)), 1e-12));

        // D1=0, D2≠0 → 로피탈
        let d1 = Vector3D::new(0.0, 0.0, 0.0);
        let d2 = Vector3D::new(0.0, -3.0, 4.0);
        let t = on_ev_tangent(d1, d2).unwrap();
        assert!(approx_vec(&t, &unit(d2), 1e-12));
    }

```

---

## 6) `on_ev_curvature` — 접선 $\(T\)$ 와 곡률 벡터 $\(K\)$

$$
T=\frac{D1}{\|D1\|},\qquad
K=\frac{D2-(D2\cdot T)T}{\|D1\|^2},\qquad
k=\|K\|.
$$

**Plain text**
```
T = D1 / |D1|
K = (D2 − (D2·T) T) / |D1|^2
k = |K|
```

### 소스
```rust
pub fn on_ev_curvature(d1: Vector3D, d2: Vector3D) -> Option<(Vector3D, Vector3D)> {
    let mut l1 = d1.length();
    if l1 == 0.0 {
        // D1=0이면 로피탈 → T≈±D2̂, K는 0(정의 곤란)
        l1 = d2.length();
        if l1 > 0.0 {
            let t = d2 / l1;
            let k = Vector3D::zero();
            return Some((t, k));
        }
        return None;
    }
    let t = d1 / l1;
    let d2_dot_t = d2.dot(&t);
    let k = (d2 - t * d2_dot_t) / (l1 * l1);
    Some((t, k))
}
```

### 사용법
```rust
  #[test]
    fn ev_curvature_circle() {
        // 원: r(t) = (cos t, sin t, 0)
        // D1 = (-sin t, cos t, 0), D2 = (-cos t, -sin t, 0)
        // t=0 → D1=(0,1,0), D2=(-1,0,0)
        let d1 = Vector3D::new(0.0, 1.0, 0.0);
        let d2 = Vector3D::new(-1.0, 0.0, 0.0);
        let (t,k) = on_ev_curvature(d1, d2).unwrap();
        assert!(approx_vec(&t, &Vector3D::new(0.0,1.0,0.0), 1e-12));
        assert!(approx_vec(&k, &Vector3D::new(-1.0,0.0,0.0), 1e-12)); // 반지름 1 → 곡률 1
    }
```

---

## 7) `on_ev_curvature_1der` — 곡률의 1차 미분 $\(k'\)$ 과 토션 $\(\tau\)$

$$
T=\frac{D1}{\|D1\|},\quad
K=\frac{D2-(D2\cdot T)T}{\|D1\|^2},\quad
q=D1\times D2,\quad k=\frac{\|q\|}{\|D1\|^3}.
$$

$$
k'=\frac{(q\cdot q')\|D1\|^2 - 3\|q\|^2(D1\cdot D2)}
{\|q\|\,\|D1\|^{5}},\qquad q' = D1\times D3.
$$

$$
\tau=\frac{(D1\times D2)\cdot D3}{\|D1\times D2\|^2}.
$$

**Plain text**
```
q = D1 × D2,  k = |q| / |D1|^3
q' = D1 × D3
k' = ((q·q') |D1|^2 − 3 |q|^2 (D1·D2)) / ( |q| * |D1|^5 )
τ  = ( (D1×D2) · D3 ) / |D1×D2|^2
```

### 소스
```rust
pub fn on_ev_curvature_1der(
    d1: Vector3D, d2: Vector3D, d3: Vector3D
) -> Option<Curvature1Der> {
    let dsdt = d1.length();
    if dsdt <= 0.0 { return None; }

    let t = d1 / dsdt;
    let q  = d1.cross(&d2);
    let q_len2 = q.length_squared();
    let dsdt2 = dsdt * dsdt;
    let k_vec = (d2 - t * d2.dot(&t)) / dsdt2;

    // k' (곡률의 1차 미분)
    let kprime = {
        if q_len2 > 0.0 {
            let qprime = d1.cross(&d3);
            let num = (q.dot(&qprime)) * d1.length_squared() - 3.0 * q_len2 * d1.dot(&d2);
            let den = q_len2.sqrt() * d1.length().powf(5.0);
            Some(num / den)
        } else {
            // q=0 이면 k=0 상태 → 근사식
            let qprime = d1.cross(&d3);
            Some(qprime.length() / d1.length().powf(3.0))
        }
    };

    // 토션 τ
    let torsion = if q_len2 > 0.0 {
        Some(q.dot(&d3) / q_len2)
    } else {
        None
    };

    Some(Curvature1Der { t, k_vec, kprime, torsion })
}

```

### 사용법
```rust
    #[test]
    fn ev_curvature_1der_helix() {
        // 3D 헬릭스: r(t) = (cos t, sin t, b t)
        // D1 = (-sin t, cos t, b), D2 = (-cos t, -sin t, 0), D3=(sin t,-cos t,0)
        // k = 1/(1+b^2), τ = b/(1+b^2),  k' = 0
        let b = 0.5_f64;
        let t0 = 0.3_f64; // 임의
        let d1 = Vector3D::new(-t0.sin(), t0.cos(), b);
        let d2 = Vector3D::new(-t0.cos(), -t0.sin(), 0.0);
        let d3 = Vector3D::new( t0.sin(), -t0.cos(), 0.0);
        let out = on_ev_curvature_1der(d1, d2, d3).unwrap();
        let k_expected = 1.0/(1.0 + b*b);
        assert!(approx(out.k_vec.length(), k_expected, 1e-8));
        assert!(approx(out.kprime.unwrap(), 0.0, 1e-8));
        assert!(approx(out.torsion.unwrap(), b/(1.0+b*b), 1e-8));
    }

```

---

## 8) `on_ev_sectional_curvature` — 평면 교선의 곡률 벡터 $\(K\)$

표면의 부분미분 $\(Su, Sv, Suu, Suv, Svv\)$ 와 평면 법선 $\(n\)$ 이 주어질 때,  
교선 곡선의 곡률 벡터를 계산합니다.

$$
M = Su\times Sv,\qquad
D1 = (Su\times Sv)\times n.
$$

$\(D1 = a\,Su + b\,Sv\)$ 를 풀어 $\(a,b\)$ 를 구하고,

$$
M_1 = (a\,Suu+b\,Suv)\times Sv \;+\; Su\times(a\,Suv+b\,Svv),
\quad
D2 = M_1 \times n.
$$

$$
K=\frac{D2 - \dfrac{D2\cdot D1}{D1\cdot D1}\,D1}{D1\cdot D1}.
$$

**Plain text**
```
M  = Su × Sv
D1 = M × n
Solve D1 = a Su + b Sv
M1 = (a Suu + b Suv) × Sv  +  Su × (a Suv + b Svv)
D2 = M1 × n
K  = (D2 − ((D2·D1)/(D1·D1)) D1) / (D1·D1)
```

### 소스
```rust
pub fn on_ev_sectional_curvature(
    s10: Vector3D, s01: Vector3D,
    s20: Vector3D, s11: Vector3D, s02: Vector3D,
    plane_normal: Vector3D,
) -> Option<Vector3D> {
    // M = Su×Sv
    let m = s10.cross(&s01);

    // D1 = (Su×Sv)×planeNormal
    let d1 = m.cross(&plane_normal);

    // D1 = a*Su + b*Sv 풀기 (3×2)
    let (_rank, a, b, _err, _pr) = solve_3x2(s10, s01, d1.x, d1.y, d1.z);
    if _rank < 2 {
        return None;
    }

    // M1 = (a*Suu + b*Suv)×Sv + Su×(a*Suv + b*Svv)
    let left  = (s20 * a + s11 * b).cross(&s01);
    let right = s10.cross(&(s11 * a + s02 * b));
    let m1 = left + right;

    // D2 = M1 × planeNormal
    let d2 = m1.cross(&plane_normal);

    // K = (D2 − proj_{D1}(D2)) / |D1|^2
    let d1_len2 = d1.length_squared();
    if d1_len2 <= f64::MIN_POSITIVE {
        return None;
    }
    let bcoef = - (d2.dot(&d1)) / d1_len2;
    let k = (d2 + d1 * bcoef) / d1_len2;
    Some(k)
}

```

### 사용법
```rust
    #[test]
    fn ev_sectional_curvature_paraboloid_plane_y0() {
        // S(u,v) = (u, v, u^2+v^2), plane: y=0 (normal n=(0,1,0))
        // 교선은 v=0 부근 곡선 C(u) = (u, 0, u^2)
        // u0=1에서 C'=(1,0,2), C''=(0,0,2)
        // k = |C'×C''|/|C'|^3 = 2 / ( (1+4)^(3/2) ) = 2/(5√5)
        let u0 = 1.0;
        let v0 = 0.0;

        let su  = Vector3D::new(1.0, 0.0, 2.0*u0);
        let sv  = Vector3D::new(0.0, 1.0, 2.0*v0);
        let suu = Vector3D::new(0.0, 0.0, 2.0);
        let suv = Vector3D::new(0.0, 0.0, 0.0);
        let svv = Vector3D::new(0.0, 0.0, 2.0);
        let n   = Vector3D::new(0.0, 1.0, 0.0);

        let k_vec = on_ev_sectional_curvature(su, sv, suu, suv, svv, n).unwrap();

        // 곡선 파라미터화로 직접 K 계산
        let c1 = Vector3D::new(1.0, 0.0, 2.0*u0); // (1,0,2)
        let c2 = Vector3D::new(0.0, 0.0, 2.0);
        let t  = unit(c1);
        let k_curve = (c2 - t * c2.dot(&t)) / c1.length_squared(); // 공식을 동일하게
        // 방향 비교(정규화) + 크기 비교
        assert!(approx_vec(&unit(k_vec), &unit(k_curve), 1e-8));
        assert!(approx(k_vec.length(), k_curve.length(), 1e-8));
    }
}
```

---
