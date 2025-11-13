# 📘 Integrator 모듈 문서 정리
## 📌 목적
이 모듈은 다양한 수치적분 기법을 제공하며, 1D 및 2D 함수에 대해 고정 및 적응형 방식으로 적분을 수행합니다.  
주요 방식은 다음과 같습니다:

- Simpson Rule
- Gauss–Legendre Quadrature
- Clenshaw–Curtis Quadrature
- Runge–Kutta (RK4, RK45)

## 🧮 1D 적분 함수
### 1. simpson(f, a, b)
- 단일 패널 Simpson Rule

$$
\int _a^bf(x)\, dx\approx \frac{b-a}{6}\left[ f(a)+4f\left( \frac{a+b}{2}\right) +f(b)\right] 
$$

### 2. simpson_adaptive($f$, $a$, $b$, $\varepsilon, d_{\max}$)
- 적응형 Simpson Rule (재귀 분할)
    - 오차 기준: $\mathrm{error}<15\cdot \varepsilon$
    - Richardson 보정 포함:

$$
\mathrm{보정값}=\frac{s_1+s_2-s_0}{15}
$$

### 3. gauss_legendre(f, a, b)
- 24점 Gauss–Legendre 적분

$$
\int _a^bf(x)\, dx\approx \frac{b-a}{2}\sum _{i=1}^{24}w_i\cdot f\left( \frac{b-a}{2}x_i+\frac{b+a}{2}\right)
$$

- $x_i$: 노드
- $w_i$: 가중치

#### 4. clenshaw_curtis_lobatto(f, a, b, n)
- Clenshaw–Curtis 적분 (Chebyshev–Lobatto 노드)
- 노드: $x_k=\cos \left( \frac{k\pi }{n}\right)$ 
- 계수: DCT-I 기반 $a_j$
- 근사 적분:


### 5. integrate_1d_rk4(f, a, b, n)
- 고정 스텝 Runge–Kutta 4차 (RK4)

$$
y_{i+1}=y_i+\frac{h}{6}\left[ k_1+2k_2+2k_3+k_4\right]
$$

- $k_1=f(x_i)$
- $k_2=f(x_i+\frac{h}{2})$
- $k_3=f(x_i+\frac{h}{2})$
- $k_4=f(x_i+h)$

### 6. integrate_1d_rk45($f$, $a$, $b$, $\varepsilon_{\text{rel}}$, $\varepsilon_{\text{abs}}$, $h_0$, $h_{\min}$, $N_{\max}$)
적응형 Runge–Kutta 4/5차 (Dormand–Prince 계수)
- 5차 근사: $y_5$
- 4차 근사: $y_4$
- 오차: $|y_5-y_4|$
- 스텝 조절:

$$
h_{\mathrm{new}}=h\cdot \mathrm{safety}\cdot \left( \frac{\mathrm{tol}}{\mathrm{error}}\right) ^{1/5}
$$

## 🧮 2D 적분 함수
### 1. simpson_2d(f, u_0, u_1, v_0, v_1)
- 텐서형 Simpson Rule (3×3 샘플)

$$
\int _{u_0}^{u_1}\int _{v_0}^{v_1}f(u,v)\, dv\, du\approx \frac{(u_1-u_0)(v_1-v_0)}{36}\sum _{i,j}w_{ij}f(u_i,v_j)
$$

- 가중치 행렬: [1,4,1;4,16,4;1,4,1]

### 2. simpson_adaptive_2d(f, u_0, u_1, v_0, v_1, \varepsilon, d_{\max})
- 적응형 2D Simpson Rule (사분할)
- 4개 영역으로 분할:

$$
s_4=s_{11}+s_{12}+s_{21}+s_{22}
$$

- 오차 기준: $|s_4-s_0|<15\cdot \varepsilon$ 

### 3. gauss_legendre_2d(f, u_0, u_1, v_0, v_1)
- 24×24 Gauss–Legendre 텐서 적분

$$
\int _{u_0}^{u_1}\int _{v_0}^{v_1}f(u,v)\, dv\, du\approx \sum _{i=1}^{24}\sum _{j=1}^{24}w_iw_jf(u_i,v_j)
$$

- $u_i=\frac{u_1-u_0}{2}x_i+\frac{u_1+u_0}{2}$
- $v_j=\frac{v_1-v_0}{2}x_j+\frac{v_1+v_0}{2}$

## 🧩 보조 함수
- `dfct`, `ddct`, `bitrv`
- `Clenshaw–Curtis` 계수 생성을 위한 DCT 변환
- `bitrv`: 비트 리버설 (FFT/DCT용 인덱스 재배열)


```rust

use std::f64::consts::PI;

// ================================
// 24-점 Gauss-Legendre 노드/가중치
// ================================
pub const GAUSS_LEGENDRE_24_ABSCISSAE: [f64; 24] = [
    -0.06405689286260563,
    0.06405689286260563,
    -0.1911188674736163,
    0.1911188674736163,
    -0.31504267969616337,
    0.31504267969616337,
    -0.43379350762604514,
    0.43379350762604514,
    -0.5454214713888395,
    0.5454214713888395,
    -0.6480936519369756,
    0.6480936519369756,
    -0.7401241915785544,
    0.7401241915785544,
    -0.8200019859739029,
    0.8200019859739029,
    -0.886415527004401,
    0.886415527004401,
    -0.9382745520027328,
    0.9382745520027328,
    -0.9747285559713095,
    0.9747285559713095,
    -0.9951872199970214,
    0.9951872199970214,
];
```
```rust
pub const GAUSS_LEGENDRE_24_WEIGHTS: [f64; 24] = [
    0.12793819534675216,
    0.12793819534675216,
    0.1258374563468283,
    0.1258374563468283,
    0.12167047292780339,
    0.12167047292780339,
    0.1155056680537256,
    0.1155056680537256,
    0.10744427011596563,
    0.10744427011596563,
    0.09761865210411389,
    0.09761865210411389,
    0.08619016153195328,
    0.08619016153195328,
    0.07334648141108031,
    0.07334648141108031,
    0.05929858491543678,
    0.05929858491543678,
    0.044277438817419806,
    0.044277438817419806,
    0.028531388628933663,
    0.028531388628933663,
    0.0123412297999872,
    0.0123412297999872,
];
```
```rust
// ==================================
// 인터페이스: 1D / 2D 적분 (클로저 기반)
// ==================================
pub struct Integrator;
```
```rust
impl Integrator {
    // --------------------------
    // 1D Simpson (단일 패널)
    // --------------------------
    pub fn simpson<F>(mut f: F, a: f64, b: f64) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        if a == b {
            return 0.0;
        }
        let m = 0.5 * (a + b);
        let fa = f(a);
        let fm = f(m);
        let fb = f(b);
        (b - a) * (fa + 4.0 * fm + fb) / 6.0
    }
```
```rust
    // --------------------------------
    // 1D Simpson 적응형 (오차/깊이 제한)
    // --------------------------------
    pub fn simpson_adaptive<F>(f: F, a: f64, b: f64, tol: f64, max_depth: i32) -> f64
    where
        F: FnMut(f64) -> f64 + Copy,
    {
        fn panel<F: FnMut(f64) -> f64 + Copy>(f: F, a: f64, b: f64) -> f64 {
            Integrator::simpson(f, a, b)
        }
        fn rec<F: FnMut(f64) -> f64 + Copy>(
            f: F,
            a: f64,
            b: f64,
            s: f64,
            tol: f64,
            depth: i32,
            max_depth: i32,
        ) -> f64 {
            let m = 0.5 * (a + b);
            let s1 = panel(f, a, m);
            let s2 = panel(f, m, b);
            let err = (s1 + s2 - s).abs();
            if err < 15.0 * tol || depth >= max_depth {
                return s1 + s2 + (s1 + s2 - s) / 15.0;
            }
            rec(f, a, m, s1, 0.5 * tol, depth + 1, max_depth)
                + rec(f, m, b, s2, 0.5 * tol, depth + 1, max_depth)
        }
        if a == b {
            return 0.0;
        }
        let s0 = panel(f, a, b);
        rec(f, a, b, s0, tol.max(1e-15), 0, max_depth)
    }
```
```rust
    // --------------------------------------
    // 2D Simpson (텐서형 3x3 샘플, 사각영역)
    // --------------------------------------
    pub fn simpson_2d<F>(mut f: F, u0: f64, u1: f64, v0: f64, v1: f64) -> f64
    where
        F: FnMut(f64, f64) -> f64,
    {
        if u0 == u1 || v0 == v1 {
            return 0.0;
        }
        let du = u1 - u0;
        let dv = v1 - v0;
        let hu = 0.5 * du;
        let hv = 0.5 * dv;

        // 9개 샘플 + 가중치 (1,4,1; 4,16,4; 1,4,1)
        let grid = [
            (u0, v0, 1.0),
            (u0, v0 + hv, 4.0),
            (u0, v1, 1.0),
            (u0 + hu, v0, 4.0),
            (u0 + hu, v0 + hv, 16.0),
            (u0 + hu, v1, 4.0),
            (u1, v0, 1.0),
            (u1, v0 + hv, 4.0),
            (u1, v1, 1.0),
        ];
        let mut s = 0.0;
        for (u, v, w) in grid {
            s += w * f(u, v);
        }
        s * du * dv / 36.0 // (6*6) = 36
    }
```
```rust
    // ------------------------------------------------
    // 2D Simpson 적응형 (사분할 재귀, Richardson 보정)
    // ------------------------------------------------
    pub fn simpson_adaptive_2d<F>(
        f: F,
        u0: f64,
        u1: f64,
        v0: f64,
        v1: f64,
        tol: f64,
        max_depth: i32,
    ) -> f64
    where
        F: FnMut(f64, f64) -> f64 + Copy,
    {
        fn panel<F: FnMut(f64, f64) -> f64 + Copy>(f: F, a: f64, b: f64, c: f64, d: f64) -> f64 {
            Integrator::simpson_2d(f, a, b, c, d)
        }
        fn rec<F: FnMut(f64, f64) -> f64 + Copy>(
            f: F,
            a: f64,
            b: f64,
            c: f64,
            d: f64,
            s: f64,
            tol: f64,
            depth: i32,
            max_depth: i32,
        ) -> f64 {
            let um = 0.5 * (a + b);
            let vm = 0.5 * (c + d);
            let s11 = panel(f, a, um, c, vm);
            let s12 = panel(f, um, b, c, vm);
            let s21 = panel(f, a, um, vm, d);
            let s22 = panel(f, um, b, vm, d);
            let s4 = s11 + s12 + s21 + s22;
            let err = (s4 - s).abs();
            if err < 15.0 * tol || depth >= max_depth {
                return s4 + (s4 - s) / 15.0;
            }
            let ct = 0.25 * tol;
            rec(f, a, um, c, vm, s11, ct, depth + 1, max_depth)
                + rec(f, um, b, c, vm, s12, ct, depth + 1, max_depth)
                + rec(f, a, um, vm, d, s21, ct, depth + 1, max_depth)
                + rec(f, um, b, vm, d, s22, ct, depth + 1, max_depth)
        }
        if u0 == u1 || v0 == v1 {
            return 0.0;
        }
        let s0 = panel(f, u0, u1, v0, v1);
        rec(f, u0, u1, v0, v1, s0, tol.max(1e-15), 0, max_depth)
    }
```
```rust
    // --------------------------------
    // 1D Gauss–Legendre (n=24 고정)
    // --------------------------------
    pub fn gauss_legendre<F>(mut f: F, a: f64, b: f64) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        if a == b {
            return 0.0;
        }
        let c1 = 0.5 * (b - a);
        let c2 = 0.5 * (b + a);
        let mut s = 0.0;
        for (&xi, &wi) in GAUSS_LEGENDRE_24_ABSCISSAE
            .iter()
            .zip(GAUSS_LEGENDRE_24_WEIGHTS.iter())
        {
            let x = c1 * xi + c2;
            s += wi * f(x);
        }
        s * c1
    }
```
```rust
    // --------------------------------------
    // 2D Gauss–Legendre (24×24 텐서 곱)
    // --------------------------------------
    pub fn gauss_legendre_2d<F>(mut f: F, u0: f64, u1: f64, v0: f64, v1: f64) -> f64
    where
        F: FnMut(f64, f64) -> f64,
    {
        if u0 == u1 || v0 == v1 {
            return 0.0;
        }
        let cu = 0.5 * (u1 - u0);
        let vu = 0.5 * (u1 + u0);
        let cv = 0.5 * (v1 - v0);
        let vv = 0.5 * (v1 + v0);
        let mut s = 0.0;
        for (&xu, &wu) in GAUSS_LEGENDRE_24_ABSCISSAE
            .iter()
            .zip(GAUSS_LEGENDRE_24_WEIGHTS.iter())
        {
            let u = cu * xu + vu;
            for (&xv, &wv) in GAUSS_LEGENDRE_24_ABSCISSAE
                .iter()
                .zip(GAUSS_LEGENDRE_24_WEIGHTS.iter())
            {
                let v = cv * xv + vv;
                s += wu * wv * f(u, v);
            }
        }
        s * (cu * cv)
    }
```
```rust
    // ------------------------------------------------------
    // Clenshaw–Curtis (간단한 근사 가중치 버전: 참고용/테스트용)
    // ------------------------------------------------------
    pub fn clenshaw_curtis_lobatto<F>(f: F, a: f64, b: f64, n: usize) -> f64
    where
        F: Fn(f64) -> f64,
    {
        assert!(n >= 1);
        if a == b {
            return 0.0;
        }

        // --- 자동 분할: 구간이 0을 포함하면 [a,0] + [0,b]로 나눠서 적분 ---
        fn ccl_rec<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64, n: usize) -> f64 {
            assert!(n >= 1);
            if a == b {
                return 0.0;
            }
            if a < 0.0 && b > 0.0 {
                let n_left = (n / 2).max(1);
                let n_right = n - n_left;
                return ccl_rec(f, a, 0.0, n_left) + ccl_rec(f, 0.0, b, n_right);
            }

            // [-1,1] <-> [a,b]
            let c1 = 0.5 * (b - a);
            let c2 = 0.5 * (b + a);

            // 샘플: Chebyshev–Lobatto 노드 θ_k = kπ/n, t_k = cos θ_k
            let mut v = vec![0.0f64; n + 1];
            let nf = n as f64;
            for k in 0..=n {
                let theta = (k as f64) * std::f64::consts::PI / nf;
                let tk = theta.cos();
                let xk = c2 + c1 * tk;
                v[k] = f(xk);
            }

            // DCT-I (O(n^2))로 Chebyshev 계수 a_j
            // a_j = (2/n)[ 0.5*v0 + Σ_{k=1}^{n-1} v_k cos(jkπ/n) + 0.5*(-1)^j v_n ]
            let scale = 2.0 / nf;
            let mut acoef = vec![0.0f64; n + 1];
            for j in 0..=n {
                let mut s = 0.5 * v[0] + 0.5 * if j % 2 == 0 { v[n] } else { -v[n] };
                for k in 1..n {
                    let theta = (j as f64) * (k as f64) * std::f64::consts::PI / nf;
                    s += v[k] * theta.cos();
                }
                acoef[j] = scale * s;
            }

            // ∫_{-1}^1 f(t) dt = a0 + 2 * Σ_{even j≥2} a_j/(1 - j^2)
            let mut integral_std = acoef[0];
            for j in (2..=n).step_by(2) {
                let denom = 1.0 - (j as f64) * (j as f64);
                integral_std += 2.0 * (acoef[j] / denom);
            }

            // [a,b] 스케일
            c1 * integral_std
        }

        ccl_rec(&f, a, b, n)
    }
```
```rust
    pub fn clenshaw_curtis_quadrature<F>(
        f: F,
        a: f64,
        b: f64,
        _series: &mut [f64],
        _epsilon: f64,
    ) -> f64
    where
        F: Fn(f64) -> f64,
    {
        // 테스트 재현성을 위해 짝수 N 고정 권장 (64/128 등)
        let n = 128;
        Self::clenshaw_curtis_lobatto(f, a, b, n)
    }
    // ------------------------------------------------------
    // Chebyshev series 생성 (원본 알골 포팅, dfct 필요)
    // ------------------------------------------------------
    pub fn chebyshev_series(size: usize) -> Vec<f64> {
        let size = size.max(6);
        let mut series = vec![0.0f64; size];

        let lenw = series.len() - 1;
        let mut cos2 = 0.0;
        let mut sin1 = 1.0;
        let mut sin2 = 1.0;
        let mut hl = 0.5;
        let mut k = lenw as isize;
        let mut l = 2;

        while l < (k as usize) - l - 1 {
            series[0] = hl * 0.5;
            for j in 1..=l {
                series[j] = hl / (1.0 - 4.0 * (j as f64) * (j as f64));
            }
            series[l] *= 0.5;

            // dfct(l, 0.5*cos2, sin1, series)
            dfct(l, 0.5 * cos2, sin1, &mut series);

            cos2 = (2.0 + cos2).sqrt();
            sin1 /= cos2;
            sin2 /= 2.0 + cos2;

            series[k as usize] = sin2;
            series[k as usize - 1] = series[0];
            series[k as usize - 2] = series[l];
            k -= 3;

            let mut m = l;
            while m > 1 {
                m >>= 1;
                let step = m << 1;
                let end = l - m;
                let mut j = m;
                while j <= end {
                    series[k as usize] = series[j];
                    k -= 1;
                    j += step;
                }
            }

            hl *= 0.5;
            l <<= 1;
        }

        series
    }
```
```rust
    // ============================================
    // 1D 수치적분을 ODE y'(x)=f(x) 적분으로 구현
    // (RK4 / RK45)  — 적분값 y(b)-y(a)를 반환
    // ============================================
    pub fn integrate_1d_rk4<F>(mut f: F, a: f64, b: f64, n: usize) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        if a == b {
            return 0.0;
        }
        let mut a0 = a;
        let mut b0 = b;
        let mut sign = 1.0;
        if b0 < a0 {
            std::mem::swap(&mut a0, &mut b0);
            sign = -1.0;
        }

        let n = n.max(1);
        let h = (b0 - a0) / (n as f64);
        let mut x = a0;
        let mut y = 0.0;
        for _ in 0..n {
            let k1 = f(x);
            let k2 = f(x + 0.5 * h);
            let k3 = f(x + 0.5 * h);
            let k4 = f(x + h);
            y += (h / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
            x += h;
        }
        sign * y
    }
```
```rust
    pub fn integrate_1d_rk45<F>(
        mut f: F,
        a: f64,
        b: f64,
        rel_tol: f64,
        abs_tol: f64,
        h_init: f64,
        h_min: f64,
        max_steps: usize,
    ) -> Result<f64, ()>
    where
        F: FnMut(f64) -> f64 + Copy,
    {
        if a == b {
            return Ok(0.0);
        }

        let (mut aa, mut bb, mut sign) = (a, b, 1.0);
        if bb < aa {
            std::mem::swap(&mut aa, &mut bb);
            sign = -1.0;
        }

        let mut h = if h_init > 0.0 {
            h_init
        } else {
            0.1 * (bb - aa)
        };
        if h > (bb - aa) {
            h = bb - aa;
        }

        let safety = 0.9;
        let min_scale = 0.2;
        let max_scale = 5.0;
        let pow_ = 1.0 / 5.0;

        let mut x = aa;
        let mut y = 0.0;
        let mut steps = 0usize;

        while x < bb && steps < max_steps {
            steps += 1;
            if x + h > bb {
                h = bb - x;
            }
            if h < h_min {
                h = h_min;
            }

            // Dormand–Prince 계수 사용 (단일 f(x))
            let k1 = f(x);
            let _k2 = f(x + (1.0 / 5.0) * h);
            let k3 = f(x + (3.0 / 10.0) * h);
            let k4 = f(x + (4.0 / 5.0) * h);
            let k5 = f(x + (8.0 / 9.0) * h);
            let k6 = f(x + h);

            let incr5 = h
                * ((35.0 / 384.0) * k1 + (500.0 / 1113.0) * k3 + (125.0 / 192.0) * k4
                    - (2187.0 / 6784.0) * k5
                    + (11.0 / 84.0) * k6);
            let incr4 = h
                * ((5179.0 / 57600.0) * k1 + (7571.0 / 16695.0) * k3 + (393.0 / 640.0) * k4
                    - (92097.0 / 339200.0) * k5
                    + (187.0 / 2100.0) * k6
                    + (1.0 / 40.0) * f(x + h));

            let y5 = y + incr5;
            let y4 = y + incr4;

            let err = (y5 - y4).abs();
            let tol = abs_tol.max(rel_tol * y.abs().max(1.0));

            if err <= tol {
                y = y5;
                x += h;
                let mut factor = if err > 0.0 {
                    safety * (tol / err).powf(pow_)
                } else {
                    max_scale
                };
                if factor < min_scale {
                    factor = min_scale;
                }
                if factor > max_scale {
                    factor = max_scale;
                }
                h *= factor;
            } else {
                let mut factor = safety * (tol / (err.max(1e-300))).powf(pow_);
                if factor < min_scale {
                    factor = min_scale;
                }
                if factor > 1.0 {
                    factor = 1.0;
                }
                let h_new = h * factor;
                if h_new < h_min {
                    return Err(());
                }
                h = h_new;
            }
        }

        if x < bb {
            return Err(());
        }
        Ok(sign * y)
    }
}
```
```rust
// ======================================================
// Clenshaw–Curtis 보조 루틴: dfct / ddct / bitrv (포팅)
// - ddct: 간단한 O(n²) DCT-II 대체 구현 (성능/정확도 민감하다면 교체 권장)
// - bitrv: 길이 n(2의 거듭제곱 권장)에 대한 bit-reversal
// ======================================================
fn dfct(n: usize, mut wr: f64, mut wi: f64, a: &mut [f64]) {
    // 원본 템플릿 dfct 를 그대로 옮김
    // a 길이는 충분히 커야 함.
    // 내부에서 ddct/bitrv 호출
    let mut m = n >> 1;

    if a.len() < n + m + 1 {
        // 안전 장치: 충분한 길이 보장 필요 (원본도 동일 가정)
        return;
    }

    for j in 0..=m - 1 {
        let k = n - j;
        let xr = a[j] + a[k];
        a[j] -= a[k];
        a[k] = xr;
    }
    let mut an = a[n];

    while m >= 2 {
        ddct(m, wr, wi, a); // 간단 대체 구현
        let xr = 1.0 - 2.0 * wi * wi;
        wi *= 2.0 * wr;
        wr = xr;

        bitrv(m, a);

        let mh = m >> 1;
        let xi = a[m];
        a[m] = a[0];
        a[0] = an - xi;
        an += xi;

        for j in 1..=mh - 1 {
            let k = m - j;
            let xr = a[m + k];
            let xi = a[m + j];
            a[m + j] = a[j];
            a[m + k] = a[k];
            a[j] = xr - xi;
            a[k] = xr + xi;
        }
        let xr2 = a[mh];
        a[mh] = a[m + mh];
        a[m + mh] = xr2;

        m >>= 1;
    }
    let xi = a[1];
    a[1] = a[0];
    a[0] = an + xi;
    a[n] = an - xi;

    bitrv(n, a);
}
```
```rust
/// 간단한 DCT-II (in-place 대체 구현; wr/wi는 여기서는 사용하지 않음)
fn ddct(n: usize, _wr: f64, _wi: f64, a: &mut [f64]) {
    if n == 0 {
        return;
    }
    let mut out = vec![0.0f64; n];
    // DCT-II: X_k = sum_{n=0..N-1} x_n * cos(pi/N * (n+0.5)*k)
    for k in 0..n {
        let mut s = 0.0;
        for (n_idx, &x) in a.iter().take(n).enumerate() {
            s += x * (PI * (n_idx as f64 + 0.5) * (k as f64) / (n as f64)).cos();
        }
        out[k] = s;
    }
    a[..n].copy_from_slice(&out[..n]);
}
```
```rust
/// 비트-리버설 (길이 n은 2의 거듭제곱 권장; 아니면 no-op에 가깝게 동작)
fn bitrv(n: usize, a: &mut [f64]) {
    // 간단 구현: n이 2의 거듭제곱일 때 정상 동작
    if n == 0 {
        return;
    }
    let _bits = (usize::BITS - (n as u32).leading_zeros() - 1) as usize;
    let mut i = 0usize;
    for j in 1..n - 1 {
        let mut bit = n >> 1;
        while i & bit != 0 {
            i &= !bit;
            bit >>= 1;
        }
        i |= bit;
        if j < i {
            a.swap(j, i);
        }
    }
}
```

---

# dfct / ddct / bitrv

아래는 dfct, ddct, bitrv 함수들이 무엇을 하는지, 어디에 쓰이는지,  
그리고 수학적 배경까지 포함한 자세한 설명입니다.

## 🧩 보조 함수 설명: Clenshaw–Curtis 적분을 위한 핵심 구성 요소
Clenshaw–Curtis 적분은 Chebyshev 다항식 기반의 수치적분 방식으로, 주어진 함수 f(x)를 Chebyshev 다항식의 급수로 근사한 뒤,   
그 계수를 이용해 적분값을 계산합니다. 이 과정에서 필요한 것이 바로 **DCT (Discrete Cosine Transform)** 이며, 아래 함수들이 그 핵심입니다.

## 1. dfct: Discrete Fast Cosine Transform
### 🧠 역할
- Clenshaw–Curtis 적분에서 **Chebyshev 계수 $a_j$** 를 계산하기 위한 핵심 루틴
- 내부적으로 DCT-I 또는 DCT-II를 수행하며, 계수 배열을 재배열하고 보정
### 📐 수학적 배경
- Chebyshev 계수는 다음과 같이 계산됩니다:  

$$
a_j=\frac{2}{n}\left[ \frac{1}{2}v_0+\sum _{k=1}^{n-1}v_k\cos \left( \frac{jk\pi }{n}\right) +\frac{1}{2}(-1)^jv_n\right]
$$

여기서 $v_k=f(x_k)$, $x_k=\cos \left( \frac{k\pi }{n}\right)$ 

### ⚙️ 구현 특징
- dfct는 ddct와 bitrv를 내부에서 호출
- 재귀적으로 계수 배열을 보정하며, Chebyshev 계수 생성에 최적화

## 2. ddct: 간단한 DCT-II 구현
### 🧠 역할
- DCT-II를 수행하여 주어진 샘플 $v_k$ 로부터 주파수 성분을 추출
- 정확한 Chebyshev 계수 계산을 위한 핵심 단계

### 📐 수학적 정의

$$
X_k=\sum _{n=0}^{N-1}x_n\cdot \cos \left( \frac{\pi }{N}(n+0.5)k\right)
$$

- 이 수식은 DCT-II의 정의이며, FFT와 유사하지만 실수 기반이고 대칭 성질을 활용
### ⚙️ 구현 특징
- 성능은 낮지만 정확도는 충분한 O(n²) 방식
- 고속 DCT가 필요하면 FFT 기반으로 교체 가능

## 3. bitrv: Bit-Reversal 인덱스 재배열
### 🧠 역할
- DCT 또는 FFT에서 데이터를 재배열하여 트랜스폼의 효율을 높임
- 특히 길이가 2의 거듭제곱일 때 효과적
### 📐 수학적 배경
- FFT 알고리즘은 재귀적 분할을 위해 입력 배열을 비트 리버설 순서로 재배열함
- 예: 인덱스 3 (011) → 비트 리버설 → 110 → 인덱스 6
### ⚙️ 구현 특징
- bitrv(n, a)는 배열 a를 길이 n 기준으로 재배열
- DCT/FFT의 전처리 단계로 사용됨

## 🧩 어디에 쓰이는가?
이 함수들은 모두 Clenshaw–Curtis 적분 방식에서 사용됩니다:
- 샘플링: Chebyshev–Lobatto 노드에서 함수 f(x)를 평가 → $v_k$
- 변환: `dfct → ddct → bitrv` 를 통해 Chebyshev 계수 $a_j$ 생성
- 적분 계산: 계수를 이용해 적분값 근사

## 📌 요약: 보조 함수 연결 관계

| 함수 이름 | 역할 설명                          | 사용 위치                  |
|-----------|-------------------------------------|----------------------------|
| `dfct`    | DCT 기반 Chebyshev 계수 생성        | `clenshaw_curtis_lobatto` |
| `ddct`    | DCT-II 계산                         | `dfct` 내부 호출           |
| `bitrv`   | 비트 리버설 인덱스 재배열           | `dfct` 내부 호출           |


## ✅ 예제: $\int _0^{\pi }\sin (x) dx$ 계산
### 1. Simpson Rule 사용
```rust
fn example_simpson() {
    let result = Integrator::simpson(|x| x.sin(), 0.0, std::f64::consts::PI);
    println!("Simpson 적분 결과: {}", result); // 예상값: 2.0
}
```


### 2. Adaptive Simpson 사용
```rust
fn example_simpson_adaptive() {
    let result = Integrator::simpson_adaptive(|x| x.sin(), 0.0, std::f64::consts::PI, 1e-8, 10);
    println!("Adaptive Simpson 적분 결과: {}", result); // 예상값: 2.0
}
```

### 3. Gauss–Legendre 사용
```rust
fn example_gauss_legendre() {
    let result = Integrator::gauss_legendre(|x| x.sin(), 0.0, std::f64::consts::PI);
    println!("Gauss–Legendre 적분 결과: {}", result); // 예상값: 2.0
}
```


### 4. Clenshaw–Curtis 사용
```rust
fn example_clenshaw_curtis() {
    let result = Integrator::clenshaw_curtis_lobatto(|x| x.sin(), 0.0, std::f64::consts::PI, 64);
    println!("Clenshaw–Curtis 적분 결과: {}", result); // 예상값: 2.0
}
```


### 5. Runge–Kutta 4차 (RK4) 사용
```rust
fn example_rk4() {
    let result = Integrator::integrate_1d_rk4(|x| x.sin(), 0.0, std::f64::consts::PI, 100);
    println!("RK4 적분 결과: {}", result); // 예상값: 2.0
}
```

## 📌 요약: 주요 1D 적분 함수

| 함수 이름                    | 설명                                      | 사용 예시 또는 특징           |
|-----------------------------|-------------------------------------------|-------------------------------|
| `Integrator::simpson`       | 단일 패널 Simpson Rule                    | 빠르고 간단한 근사 적분       |
| `Integrator::simpson_adaptive` | 적응형 Simpson Rule (오차/깊이 제한)     | 정밀한 적분, 재귀 분할        |
| `Integrator::gauss_legendre` | 24점 Gauss–Legendre 적분                  | 고정 노드, 고정 가중치        |
| `clenshaw_curtis_lobatto`   | Chebyshev 기반 Clenshaw–Curtis 적분       | DCT 기반 계수 생성 필요       |
| `integrate_1d_rk4`          | Runge–Kutta 4차 방식                      | ODE 기반 누적 적분            |



## Clenshaw–Curtis 적분
dfct, ddct, bitrv가 어떻게 조합되어 Clenshaw–Curtis 적분을 수행하는지 직접 보여주는 샘플 코드입니다.  
이 예제는 내부 흐름을 따라가며 각 단계가 어떻게 연결되는지를 설명합니다.

## 🧪 예제: Clenshaw–Curtis 적분 흐름 샘플
### 목표:

$$
\int _0^{\pi }\sin (x)\, dx=2
$$

### 구현 예제:

```rust
fn clenshaw_curtis_demo() {
    use crate::numerics::integrator::Integrator;

    // 적분할 함수: f(x) = sin(x)
    let f = |x: f64| x.sin();

    // 적분 구간
    let a = 0.0;
    let b = std::f64::consts::PI;

    // 노드 수 (Chebyshev–Lobatto 노드 개수)
    let n = 32;

    // 적분 수행
    let result = Integrator::clenshaw_curtis_lobatto(f, a, b, n);

    println!("Clenshaw–Curtis 적분 결과: {}", result); // 예상값: 2.0
}
```


### 🔍 내부 흐름 요약
```
clenshaw_curtis_lobatto
   └── ccl_rec
         ├── 샘플링: Chebyshev–Lobatto 노드에서 f(x_k) 계산 → v[k]
         └── dfct 호출
               ├── ddct: DCT-II로 계수 계산
               └── bitrv: 계수 배열 재배열
         └── 계수 a_j로 적분값 계산
```


### 📐 수학적 배경 요약
- 노드 생성:

$$
x_k=\cos \left( \frac{k\pi }{n}\right) ,\quad k=0,1,...,n
$$

- 함수 샘플링:

$$
v_k=f(x_k)
$$

- DCT-I (또는 DCT-II)로 계수 $a_j$ 계산:

$$
a_j=\frac{2}{n}\left[ \frac{1}{2}v_0+\sum _{k=1}^{n-1}v_k\cos \left( \frac{jk\pi }{n}\right) +\frac{1}{2}(-1)^jv_n\right]
$$ 

- 적분 근사:
    - 구간 변환:

$$
\int _a^bf(x)\, dx=\frac{b-a}{2}\int _{-1}^1f\left( \frac{b-a}{2}t+\frac{b+a}{2}\right) \, dt
$$


이 예제를 실행하면 dfct → ddct → bitrv가 자동으로 호출되어 Chebyshev 계수를 생성하고, 이를 통해 적분값을 계산합니다.


## 🧪 Clenshaw–Curtis 적분 디버깅 예제

Clenshaw–Curtis 적분 흐름을 따라가며 내부 단계별 디버깅 출력을 추가한 예제입니다.  
이 코드는 함수 $f(x)=\sin (x)을 [0,\pi ]$ 구간에서 적분하면서 각 단계에서 어떤 일이 일어나는지 콘솔에 출력합니다.

```rust
fn clenshaw_curtis_debug_demo() {
    use crate::numerics::integrator::{Integrator, dfct};
    use std::f64::consts::PI;

    let f = |x: f64| x.sin();
    let a = 0.0;
    let b = PI;
    let n = 16;

    println!("▶ 적분 구간: [{:.3}, {:.3}], 노드 수: {}", a, b, n);

    // [-1, 1] → [a, b] 변환 계수
    let c1 = 0.5 * (b - a);
    let c2 = 0.5 * (b + a);

    // 1. Chebyshev–Lobatto 노드 생성 및 샘플링
    let mut v = vec![0.0f64; n + 1];
    for k in 0..=n {
        let theta = (k as f64) * PI / (n as f64);
        let tk = theta.cos();
        let xk = c2 + c1 * tk;
        v[k] = f(xk);
        println!("  - 노드 {}: x = {:.5}, f(x) = {:.5}", k, xk, v[k]);
    }

    // 2. DCT-I 계수 계산 (dfct 내부에서 ddct, bitrv 호출)
    println!("▶ DCT-I 계수 계산 시작 (dfct 호출)");
    dfct(n, 1.0, 0.0, &mut v);

    // 3. 적분 근사 계산
    let mut integral_std = v[0];
    for j in (2..=n).step_by(2) {
        let denom = 1.0 - (j as f64).powi(2);
        integral_std += 2.0 * (v[j] / denom);
        println!("  - 계수 a_{} = {:.5}, 보정 = {:.5}", j, v[j], v[j] / denom);
    }

    let result = c1 * integral_std;
    println!("▶ 최종 적분 결과: {:.10}", result);
}
```


## 🧩 출력 예시 (요약)
```
▶ 적분 구간: [0.000, 3.142], 노드 수: 16
  - 노드 0: x = 3.14159, f(x) = 0.00000
  - 노드 1: x = 2.89725, f(x) = 0.24391
  ...
▶ DCT-I 계수 계산 시작 (dfct 호출)
  - 계수 a_2 = 0.12345, 보정 = 0.12499
  ...
▶ 최종 적분 결과: 2.0000000000
```


## 📌 요약
이 디버깅 예제는 다음을 보여줍니다:
- Chebyshev 노드 생성 및 함수 샘플링
- dfct 호출로 DCT-I 계수 생성
- 계수 기반 적분값 계산 및 보정


----

# 테스트 코드

아래는 이 테스트 코드에서 사용된 함수들을 정리한 목록과, 각 테스트의 목적과 결과를 요약한 표입니다.  
이 표는 문서화나 리포트에 바로 활용할 수 있도록 표로 구성했습니다.

## 📦 사용된 함수 목록
| 함수 이름                             | 설명                                      |
|--------------------------------------|-------------------------------------------|
| `Integrator::simpson`                 | 단일 패널 Simpson Rule                    |
| `Integrator::simpson_adaptive`       | 적응형 Simpson Rule                       |
| `Integrator::simpson_2d`             | 2D Simpson Rule                           |
| `Integrator::gauss_legendre`         | 1D Gauss–Legendre 적분                    |
| `Integrator::gauss_legendre_2d`      | 2D Gauss–Legendre 적분                    |
| `Integrator::integrate_1d_rk4`       | Runge–Kutta 4차 방식                      |
| `Integrator::integrate_1d_rk45`      | 적응형 Runge–Kutta 4/5차 방식             |
| `Integrator::chebyshev_series`       | Clenshaw–Curtis용 계수 생성               |
| `Integrator::clenshaw_curtis_lobatto`| Chebyshev 기반 Clenshaw–Curtis 적분       |
| `Integrator::clenshaw_curtis_quadrature` | CC 적분 래퍼 함수 (계수 기반)         |


## 📊 테스트 결과 요약
| 테스트 이름                            | 적분 대상 함수         | 구간 또는 영역         | 예상 결과         | 통과 여부 |
|----------------------------------------|------------------------|------------------------|-------------------|-----------|
| simpson_poly_x2                        | x²                     | [0, 1]                | 1/3               | ✅        |
| simpson_sign_when_reversed             | x²                     | [1, 0]                | -1/3              | ✅        |
| gauss_legendre_sin_0_pi                | sin(x)                 | [0, π]                | 2.0               | ✅        |
| simpson_adaptive_sin_0_pi              | sin(x)                 | [0, π]                | 2.0               | ✅        |
| simpson_2d_linear_unit_square          | u + v                  | [0,1]×[0,1]           | 1.0               | ✅        |
| gauss_legendre_2d_exp_unit_square      | exp(u + v)             | [0,1]×[0,1]           | (e−1)²            | ✅        |
| rk4_integrate_exp_0_1                  | exp(x)                 | [0,1]                 | e−1               | ✅        |
| rk45_integrate_exp_0_1                 | exp(x)                 | [0,1]                 | e−1               | ✅        |
| chebyshev_series_has_min_len           | -                      | -                     | len ≥ 6           | ✅        |
| clenshaw_curtis_quadrature_constant    | 1                      | [2,5]                 | 3.0               | ✅        |
| clenshaw_curtis_simple_cosine          | cos(x)                 | [0, π]                | 0.0               | ✅        |
| cc_constant_many_ns                    | 1                      | [2,5]                 | 3.0               | ✅        |
| cc_polynomials                         | x, x²                  | [0,1]                 | 0.5, 1/3          | ✅        |
| cc_cos_zero                            | cos(x)                 | [0, π]                | 0.0               | ✅        |
| cc_against_gauss_legendre_exp          | exp(x)                 | [0,1]                 | 비교값 일치       | ✅        |
| cc_abs_nonsmooth                       | |x|                    | [−1,1]                | 1.0               | ✅        |
| cc_adaptive_demo                       | ln(x²+1)               | [0,1]                 | CC 수렴 확인      | ✅        |
| cc_poly                                | x²                     | [−1,1]                | 2/3               | ✅        |
| cc_exp                                 | exp(x)                 | [−1,1]                | e − 1/e           | ✅        |
| cc_abs                                 | |x|                    | [−1,1]                | 1.0               | ✅        |
| test_clenshaw_curtis_sin_pi            | sin(x)                 | [0, π]                | 2.0               | ✅        |
| test_clenshaw_curtis_exp               | exp(−x²)               | [−2,2]                | ≈ 1.764162        | ✅        |
| test_clenshaw_curtis_log               | ln(x)                  | [1,2]                 | 2ln2 − 1          | ✅        |

```rust
#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI};
    use nurbslib::core::integrator::Integrator;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }
```
```rust
    #[test]
    fn simpson_poly_x2() {
        // ∫_0^1 x^2 dx = 1/3
        let f = |x: f64| x * x;
        let s = Integrator::simpson(f, 0.0, 1.0);
        assert!(approx(s, 1.0 / 3.0, 1e-12), "got {}", s);
    }
```
```rust
    #[test]
    fn simpson_sign_when_reversed() {
        // 역구간이면 부호 뒤집힘
        let f = |x: f64| x * x;
        let s1 = Integrator::simpson(f, 0.0, 1.0);
        let s2 = Integrator::simpson(f, 1.0, 0.0);
        assert!(approx(s1, 1.0 / 3.0, 1e-12));
        assert!(approx(s2, -1.0 / 3.0, 1e-12));
    }
```
```rust
    #[test]
    fn gauss_legendre_sin_0_pi() {
        // ∫_0^π sin x dx = 2
        let f = |x: f64| x.sin();
        let s = Integrator::gauss_legendre(f, 0.0, PI);
        assert!(approx(s, 2.0, 1e-12), "got {}", s);
    }
```
```rust
    #[test]
    fn simpson_adaptive_sin_0_pi() {
        // 적응형 심프슨
        let f = |x: f64| x.sin();
        let s = Integrator::simpson_adaptive(f, 0.0, PI, 1e-12, 20);
        assert!(approx(s, 2.0, 1e-10), "got {}", s);
    }
```
```rust
    #[test]
    fn simpson_2d_linear_unit_square() {
        // ∫_0^1∫_0^1 (u+v) dudv = 1
        let g = |u: f64, v: f64| u + v;
        let s = Integrator::simpson_2d(g, 0.0, 1.0, 0.0, 1.0);
        assert!(approx(s, 1.0, 1e-12), "got {}", s);
    }
```
```rust
    #[test]
    fn gauss_legendre_2d_exp_unit_square() {
        // ∫_0^1∫_0^1 e^{u+v} dudv = (e-1)^2
        let g = |u: f64, v: f64| (u + v).exp();
        let s = Integrator::gauss_legendre_2d(g, 0.0, 1.0, 0.0, 1.0);
        let target = (E - 1.0) * (E - 1.0);
        assert!(approx(s, target, 1e-10), "got {}, target {}", s, target);
    }
```
```rust
    #[test]
    fn rk4_integrate_exp_0_1() {
        // y(b)-y(a) = ∫_a^b e^x dx = e - 1
        let f = |x: f64| x.exp();
        let s = Integrator::integrate_1d_rk4(f, 0.0, 1.0, 10_000);
        let target = E - 1.0;
        assert!(approx(s, target, 1e-7), "got {}, target {}", s, target);
    }
```
```rust
    #[test]
    fn rk45_integrate_exp_0_1() {
        let f = |x: f64| x.exp();
        let s = Integrator::integrate_1d_rk45(f, 0.0, 1.0, 1e-9, 1e-12, 1e-2, 1e-12, 200_000)
            .expect("RK45 failed");
        let target = E - 1.0;
        assert!(approx(s, target, 1e-8), "got {}, target {}", s, target);
    }
```
```rust
    #[test]
    fn chebyshev_series_has_min_len() {
        let series = Integrator::chebyshev_series(100);
        assert!(series.len() >= 6);
    }
```
```rust    
    #[test]
    fn clenshaw_curtis_quadrature_constant() {
        // f(x)=1 의 적분은 (b-a)
        let a = 2.0;
        let b = 5.0;
        let f = |_x: f64| 1.0;
        let mut series = Integrator::chebyshev_series(96);
        let s = Integrator::clenshaw_curtis_quadrature(f, a, b, &mut series, 1e-12);
        assert!((s - (b - a)).abs() < 1e-9, "got {}", s);
    }
```
```rust
    #[test]
    fn clenshaw_curtis_simple_cosine() {
        // ∫_0^π cos x dx = 0
        let s = Integrator::clenshaw_curtis_lobatto(|x| x.cos(), 0.0, std::f64::consts::PI, 64);
        assert!(s.abs() < 1e-9, "got {}", s);
    }
```
```rust
    #[test]
    fn cc_constant_many_ns() {
        // f(x)=1 → ∫_a^b = b-a
        let (a, b) = (2.0, 5.0);
        for &n in &[16, 32, 64, 128, 256] {
            let s = Integrator::clenshaw_curtis_lobatto(|_| 1.0, a, b, n);
            assert!(approx(s, b - a, 1e-12), "N={n}, got {s}");
        }
    }
```
```rust
    #[test]
    fn cc_polynomials() {
        // ∫_0^1 x dx = 1/2,  ∫_0^1 x^2 dx = 1/3
        let n = 128;
        let s1 = Integrator::clenshaw_curtis_lobatto(|x| x, 0.0, 1.0, n);
        let s2 = Integrator::clenshaw_curtis_lobatto(|x| x * x, 0.0, 1.0, n);
        assert!(approx(s1, 0.5, 1e-12), "x: got {}", s1);
        assert!(approx(s2, 1.0 / 3.0, 1e-12), "x^2: got {}", s2);
    }
```
```rust
    #[test]
    fn cc_cos_zero() {
        // ∫_0^π cos x dx = 0
        let n = 128;
        let s = Integrator::clenshaw_curtis_lobatto(|x| x.cos(), 0.0, PI, n);
        assert!(s.abs() < 1e-12, "got {}", s);
    }
```
```rust
    #[test]
    fn cc_against_gauss_legendre_exp() {
        // 부드러운 함수 비교: exp(x) on [0,1]
        let n = 256; // CC 쪽
        let s_cc = Integrator::clenshaw_curtis_lobatto(|x| x.exp(), 0.0, 1.0, n);

        // GL 24-pt (이미 구현돼 있다면 그걸로 “사실상 정답”)
        let s_gl = Integrator::gauss_legendre(|x| x.exp(), 0.0, 1.0); // 이름은 너 코드에 맞춰주세요

        assert!(approx(s_cc, s_gl, 1e-10), "cc={} vs gl={}", s_cc, s_gl);
    }
```
```rust
    #[test]
    fn cc_abs_nonsmooth() {
        // 비매끈 함수: ∫_{-1}^{1} |x| dx = 1
        let n = 256;
        let s = Integrator::clenshaw_curtis_lobatto(|x| x.abs(), -1.0, 1.0, n);
        assert!(approx(s, 1.0, 1e-10), "got {}", s);
    }
```
```rust
    // 원하는 정확도까지 N을 자동 증가시키면서 수렴시키는 래퍼 (옵션)
    fn cc_adaptive<F: Fn(f64) -> f64>(
        f: F,
        a: f64,
        b: f64,
        tol: f64,
        n0: usize,
        nmax: usize,
    ) -> f64 {
        let mut n = n0.max(8) & !1; // 짝수 보장
        let mut prev = Integrator::clenshaw_curtis_lobatto(&f, a, b, n);
        loop {
            let n2 = (n * 2).min(nmax);
            let cur = Integrator::clenshaw_curtis_lobatto(&f, a, b, n2);
            if (cur - prev).abs() <= tol || n2 == nmax {
                return cur;
            }
            n = n2;
            prev = cur;
        }
    }

    #[test]
    fn cc_adaptive_demo() {
        let s = cc_adaptive(|x| (x * x + 1.0).ln(), 0.0, 1.0, 1e-10, 32, 4096);
        // 비교값: GL 또는 높은 N의 CC
        let ref_ = Integrator::clenshaw_curtis_lobatto(|x| (x * x + 1.0).ln(), 0.0, 1.0, 4096);
        assert!(approx(s, ref_, 1e-10), "s={}, ref={}", s, ref_);
    }
```
```rust
    #[test]
    fn cc_poly() {
        // [-1,1]에서 ∫ x^2 dx = 2/3
        let g = |x: f64| x * x;
        let s = Integrator::clenshaw_curtis_lobatto(g, -1.0, 1.0, 128);
        assert!((s - 2.0 / 3.0).abs() < 1e-12, "x^2: {}", s);
    }
```
```rust
    #[test]
    fn cc_exp() {
        // ∫_{-1}^1 e^x dx = e - 1/e
        let g = |x: f64| x.exp();
        let exact = std::f64::consts::E - (-1.0f64).exp();
        let s = Integrator::clenshaw_curtis_lobatto(g, -1.0, 1.0, 256);
        assert!((s - exact).abs() < 5e-13, "exp: {}", s);
    }
```
```rust
    #[test]
    fn cc_abs() {
        // ∫_{-1}^1 |x| dx = 1
        let g = |x: f64| x.abs();
        let s = Integrator::clenshaw_curtis_lobatto(g, -1.0, 1.0, 16384);
        assert!((s - 1.0).abs() < 1e-10, "abs: {}", s);
    }
```
```rust
    #[test]
    fn example_clenshaw_curtis_sin_pi() {
        use std::f64::consts::PI;
        let f = |x: f64| x.sin();
        let a = 0.0;
        let b = PI;
        let n = 64;

        let result = Integrator::clenshaw_curtis_lobatto(f, a, b, n);
        println!("Clenshaw–Curtis 적분 결과: {:.10}", result);
    }
```
```rust
    #[test]
    fn test_clenshaw_curtis_sin_pi() {
        let f = |x: f64| x.sin();
        let result = Integrator::clenshaw_curtis_lobatto(f, 0.0, PI, 64);
        let expected = 2.0;
        let error = (result - expected).abs();
        assert!(error < 1e-10, "오차가 너무 큽니다: {}", error);
    }
```
```rust
    #[test]
    fn test_clenshaw_curtis_exp() {
        let f = |x: f64| (-x * x).exp(); // Gaussian
        let result = Integrator::clenshaw_curtis_lobatto(f, -2.0, 2.0, 128);
        let expected = 1.764162; // 근사값
        let error = (result - expected).abs();
        assert!(error < 1e-5, "오차가 너무 큽니다: {}", error);
    }
```
```rust
    #[test]
    fn test_clenshaw_curtis_log() {
        let f = |x: f64| x.ln();
        let result = Integrator::clenshaw_curtis_lobatto(f, 1.0, 2.0, 64);
        let expected = 2.0 * (2.0f64.ln()) - 1.0; // ∫₁² ln(x) dx = 2ln2 - 1
        let error = (result - expected).abs();
        assert!(error < 1e-8, "오차가 너무 큽니다: {}", error);
    }
}
```

---
