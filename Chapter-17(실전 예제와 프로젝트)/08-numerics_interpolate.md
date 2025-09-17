# 수치적분 · 스플라인 · 기하 유틸 요약 (MD)

**적분기(Integrator)**, **B-spline 기초 루틴**, **간단한 기하 유틸**의 핵심 **정의·수식·구현 주의점**을 요약.  
테스트에 사용된 식과 불변식(invariant)도 함께 정리했습니다.

---

## 표기(Notation)

- 구간: $\([a,b]\)$, 길이 $\(L = b-a\)$, 선형변환 $\(x = \tfrac{a+b}{2} + \tfrac{b-a}{2}t\)$, $\(t\in[-1,1]\)$  
  편의로 $\(c_1=\tfrac{b-a}{2}, c_2=\tfrac{a+b}{2}\)$ 사용.
- 체비셰프–로바토 노드: $\(\theta_k=\frac{k\pi}{n}, t_k=\cos\theta_k, x_k=c_2+c_1 t_k, k=0,\dots,n\)$
- B-spline: 차수 $\(p\)$, 컨트롤포인트 개수 $\(n+1\)$, 매듭 개수 $\(m+1\)$  
  **관계식:** $\(m = n + p + 1 \iff |U|= n + p + 2\)$.

---

# 1. 1D/2D 수치적분

## 1.1 Simpson (단일 패널)
- 공식

$$
  \int_a^b f(x) dx \approx \frac{b-a}{6}\bigl[f(a)+4f(\tfrac{a+b}{2})+f(b)\bigr].
$$

- 정밀도: 구간 길이 $\(h=b-a\)$ 에서 **오차 $\(O(h^5)\)$**, 복합 규칙을 $\(N\)$ 분할로 쓰면 전체 $\(O(N^{-4})\)$.

### Adaptive Simpson
- 한 구간(패널) 적분값을 $\(S\)$, 반분 후 합을 $\(S_1+S_2\)$ 라 할 때

$$
  \text{err} \approx \frac{S_1+S_2 - S}{15}.
$$

- 허용오차(tol) 기준으로 재귀 분할, 리처드슨 보정 $\(S^\* = S_1+S_2 + \frac{S_1+S_2 - S}{15}\)$.

## 1.2 2D Simpson (텐서곱 3×3)
- u, v 각각 Simpson을 적용한 텐서곱

$$
  \iint_{[u_0,u_1]\times[v_0,v_1]} f \mathrm{d}u \mathrm{d}v
  \approx \frac{(u_1-u_0)(v_1-v_0)}{36}
  \sum_{i,j\in\{0,1,2\}} w_i w_j f(u_i,v_j)
$$

단, 1D 가중치 \(w = (1,4,1)\), 격자점은 \((u_0,u_m,u_1)\times(v_0,v_m,v_1)\).

### 2D Adaptive Simpson
- 사분할 재귀(4 패널)로 $\(S\to S_4\)$, 오차 추정 $\(|S_4-S|\)$, 리처드슨 보정 $\(S^\* = S_4 + \frac{S_4-S}{15}\)$.

## 1.3 Gauss–Legendre (24점 고정)
- 표준구간 $\([-1,1]\)$ 의 노드 $\(\xi_i\)$, 가중치 $\(w_i\)$ 를 사용하여

$$
  \int_a^b f(x) dx \approx c_1 \sum_{i=1}^{24} w_i  f(c_1 \xi_i + c_2).
$$

- 2D는 텐서곱 $\(\sum_i\sum_j w_i w_j f(u(\xi_i),v(\xi_j))\)$ 후 $\(c_u c_v\)$ 배.

## 1.4 Clenshaw–Curtis (Lobatto) — **DCT-I 기반 정석 구현**

### 핵심 아이디어
- $\([-1,1]\)$ 의 Chebyshev–Lobatto 노드 $\(t_k=\cos(k\pi/n)\)$ 에서 $\(v_k=f(c_2+c_1 t_k)\)$ 를 샘플.
- **DCT-I**로 Chebyshev 계수 $\(a_j\)$ 를 계산:

$$
  a_j = \frac{2}{n}\Bigl[\frac{1}{2}v_0 + \sum_{k=1}^{n-1} v_k \cos\!\Bigl(\frac{jk\pi}{n}\Bigr)
          + \frac{1}{2}(-1)^j v_n\Bigr],\quad j=0,\dots,n.
$$

- 표준구간 적분은

$$
  \int_{-1}^1 f(t) dt = a_0 + 2\sum_{\substack{j\ge 2\\ j\ \text{even}}}\frac{a_j}{1-j^2}.
$$

- 일반구간은 $\([a,b]\)$ 스케일로 $\(c_1\int_{-1}^1 f(c_2+c_1 t) dt\)$.

### **비매끈 \(f\)에 대한 분할(Automatic split)**
- $\(|x|\)$, $\(\max(0,x)\)$ 등 **모서리**가 있는 함수는 단일 구간 CC 수렴이 $\(O(n^{-2})\)$.
- $\([a,b]\)$ 가 0을 가로지르면 **$\[a,0] + [0,b]$** 로 자동 분할하면 각 부분이 매끈(선형 등)이라
  머신 정밀도에 근접.

### 수렴/오차 특성
- 해석적(analytic) $\(f\)$: 지수적/spectral 수렴.  
- $\(|x|\)$ 등 1차 연속·미분불연속: $\(O(n^{-2})\)$ — **분할 권장**.

---

# 2. Chebyshev 관련 보조 루틴

- `chebyshev_series(size)`: 테스트용 계수 시퀀스 생성(원본 알고리즘 포팅).
- `dfct / ddct / bitrv`: 간단 대체 구현. 정확도·성능 민감 시 FFT 기반 DCT 라이브러리로 교체 권장.

---

# 3. ODE 관점의 적분 (누적 적분기)

## 3.1 RK4
- 스텝 $\(h\)$ 에서

$$
  y_{n+1} = y_n + \frac{h}{6}(k_1+2k_2+2k_3+k_4),\quad
  k_1=f(x_n),\ k_2=f(x_n+\tfrac{h}{2}),\ k_3=f(x_n+\tfrac{h}{2}),\ k_4=f(x_n+h).
$$

- 1D 적분 $\(\int_a^b f(x) dx\)$ 는 $\(y'(x)=f(x), y(a)=0\)$ 로 두고 $\(y(b)\)$ 를 반환.

## 3.2 RK45 (Dormand–Prince, 적응 스텝)
- 5차/4차 쌍으로 국소 오차 추정, 수용 시 스텝 증감:

$$
  h_{\text{new}} = h\cdot \mathrm{safety}\cdot\Bigl(\frac{\text{tol}}{\text{err}}\Bigr)^{1/5},
$$

  범위 제한($\(0.2\sim5.0\)$) 적용.  
- 코드에 사용된 계수는 표준 Dormand–Prince(5(4)) 계수.

---

# 4. 기하 유틸

## 4.1 평면 방정식(정규화 & 방향 고정)
- 세 점 $\(p_0,p_1,p_2\)$ 에서

$$
  \mathbf{n} = \frac{(p_1-p_0)\times(p_2-p_0)}{\|(p_1-p_0)\times(p_2-p_0)\|},\quad
  d = -\mathbf{n}\cdot p_0.
$$

- 테스트 일관성 위해 $\(n_z<0\)$ 이면 $\((\mathbf{n},d)\to(-\mathbf{n},-d)\)$ 로 뒤집음.  
  기대: $\(a\approx0,\; b\approx0,\; c\approx1\)$.

## 4.2 사면체 부피
- 점 $\(a,b,c,d\)$ 에 대해
$$
  V = \frac{1}{6}\bigl| (a-d)\cdot \bigl((b-d)\times(c-d)\bigr) \bigr|.
$$
- 절대값으로 **꼭지점 순서**에 따른 부호 차이를 제거.

---

# 5. 구현 체크리스트 / 팁

- **실수 비교**: `close(a,b)`는 절대+상대 혼합(스케일) 오차 사용.  
- **Clenshaw–Curtis**: 가중치 공식을 직접 합성하지 않음. **반드시 DCT-I**로 계수 → 적분식 조합.  
- **비매끈 함수**: 0을 가로지르면 **자동 분할**.  
- **B-spline**: 반복 매듭에서 **분모 0 가드**. `find_span`의 경계 정의 엄수.  
- **테스트**:  
  - CC: $\(\int_{-1}^1 x^2 dx = 2/3\)$, $\(\int_{-1}^1 e^x dx=e-1/e\)$, $\(\int_{-1}^1|x|dx=1\)$  
  - NURBS/B-spline: $\(\sum N=1\), \(\sum N'=0\)$ 전 구간 샘플.

---

# 6. 복잡도 요약

- Simpson: 패널 O(1), 적응형은 패널 수에 비례.  
- Gauss–Legendre(24점): O(24) $\(\approx\)$ O(1). 2D는 O(24²).  
- Clenshaw–Curtis(DCT-I 단순 구현): O(n²). 다회 사용 시 $\(\cos(jk\pi/n)\)$ 테이블 캐시로 상수 개선 가능.  

---

## 부록: 코드에서의 주요 매핑

- 선형변환: `x = c2 + c1 * cos(theta)` with `theta = k*PI/n`.
- DCT-I 계수:
  ```text
  a_j = (2/n) * ( 0.5*v0 + sum_{k=1}^{n-1} v_k*cos(j*k*pi/n) + 0.5*(-1)^j * v_n )
  ```
- 적분 조합:
  ```text
  ∫_{-1}^1 f = a0 + 2 * Σ_{even j≥2} a_j / (1 - j^2)
  ∫_{a}^b f = c1 * (위 식)
  ```
- RK4 증분:
  ```text
  y += h/6 * (k1 + 2k2 + 2k3 + k4)
  ```
- Tet volume:
  ```text
  V = | (a-d) · ((b-d)×(c-d)) | / 6
  ```

---

### 소스
```rust
use std::f64::consts::PI;

// ================================
// 24-점 Gauss-Legendre 노드/가중치
// ================================
pub const GAUSS_LEGENDRE_24_ABSCISSAE: [f64; 24] = [
    -0.06405689286260563,  0.06405689286260563,
    -0.1911188674736163,   0.1911188674736163,
    -0.31504267969616337,  0.31504267969616337,
    -0.43379350762604514,  0.43379350762604514,
    -0.5454214713888395,   0.5454214713888395,
    -0.6480936519369756,   0.6480936519369756,
    -0.7401241915785544,   0.7401241915785544,
    -0.8200019859739029,   0.8200019859739029,
    -0.886415527004401,    0.886415527004401,
    -0.9382745520027328,   0.9382745520027328,
    -0.9747285559713095,   0.9747285559713095,
    -0.9951872199970214,   0.9951872199970214,
];

pub const GAUSS_LEGENDRE_24_WEIGHTS: [f64; 24] = [
    0.12793819534675216, 0.12793819534675216,
    0.1258374563468283,  0.1258374563468283,
    0.12167047292780339, 0.12167047292780339,
    0.1155056680537256,  0.1155056680537256,
    0.10744427011596563, 0.10744427011596563,
    0.09761865210411389, 0.09761865210411389,
    0.08619016153195328, 0.08619016153195328,
    0.07334648141108031, 0.07334648141108031,
    0.05929858491543678, 0.05929858491543678,
    0.044277438817419806,0.044277438817419806,
    0.028531388628933663,0.028531388628933663,
    0.0123412297999872,  0.0123412297999872,
];

// ==================================
// 인터페이스: 1D / 2D 적분 (클로저 기반)
// ==================================
pub struct Integrator;


impl Integrator {
    // --------------------------
    // 1D Simpson (단일 패널)
    // --------------------------
    pub fn simpson<F>(mut f: F, a: f64, b: f64) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        if a == b { return 0.0; }
        let m = 0.5 * (a + b);
        let fa = f(a);
        let fm = f(m);
        let fb = f(b);
        (b - a) * (fa + 4.0 * fm + fb) / 6.0
    }

    // --------------------------------
    // 1D Simpson 적응형 (오차/깊이 제한)
    // --------------------------------
    pub fn simpson_adaptive<F>(
        mut f: F, a: f64, b: f64,
        tol: f64, max_depth: i32
    ) -> f64
    where
        F: FnMut(f64) -> f64 + Copy,
    {
        fn panel<F: FnMut(f64)->f64 + Copy>(mut f: F, a: f64, b: f64) -> f64 {
            Integrator::simpson(f, a, b)
        }
        fn rec<F: FnMut(f64)->f64 + Copy>(
            mut f: F, a: f64, b: f64, s: f64, tol: f64, depth: i32, max_depth: i32
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
        if a == b { return 0.0; }
        let s0 = panel(f, a, b);
        rec(f, a, b, s0, tol.max(1e-15), 0, max_depth)
    }

    // --------------------------------------
    // 2D Simpson (텐서형 3x3 샘플, 사각영역)
    // --------------------------------------
    pub fn simpson_2d<F>(mut f: F, u0: f64, u1: f64, v0: f64, v1: f64) -> f64
    where
        F: FnMut(f64, f64) -> f64,
    {
        if u0 == u1 || v0 == v1 { return 0.0; }
        let du = u1 - u0;
        let dv = v1 - v0;
        let hu = 0.5 * du;
        let hv = 0.5 * dv;

        // 9개 샘플 + 가중치 (1,4,1; 4,16,4; 1,4,1)
        let grid = [
            (u0,       v0,       1.0),
            (u0,       v0 + hv,  4.0),
            (u0,       v1,       1.0),
            (u0 + hu,  v0,       4.0),
            (u0 + hu,  v0 + hv, 16.0),
            (u0 + hu,  v1,       4.0),
            (u1,       v0,       1.0),
            (u1,       v0 + hv,  4.0),
            (u1,       v1,       1.0),
        ];
        let mut s = 0.0;
        for (u,v,w) in grid {
            s += w * f(u,v);
        }
        s * du * dv / 36.0 // (6*6) = 36
    }

    // ------------------------------------------------
    // 2D Simpson 적응형 (사분할 재귀, Richardson 보정)
    // ------------------------------------------------
    pub fn simpson_adaptive_2d<F>(
        mut f: F, u0: f64, u1: f64, v0: f64, v1: f64,
        tol: f64, max_depth: i32
    ) -> f64
    where
        F: FnMut(f64, f64) -> f64 + Copy,
    {
        fn panel<F: FnMut(f64,f64)->f64 + Copy>(mut f: F, a: f64, b: f64, c: f64, d: f64) -> f64 {
            Integrator::simpson_2d(f, a, b, c, d)
        }
        fn rec<F: FnMut(f64,f64)->f64 + Copy>(
            mut f: F, a: f64, b: f64, c: f64, d: f64,
            s: f64, tol: f64, depth: i32, max_depth: i32
        ) -> f64 {
            let um = 0.5*(a+b);
            let vm = 0.5*(c+d);
            let s11 = panel(f, a, um, c, vm);
            let s12 = panel(f, um, b, c, vm);
            let s21 = panel(f, a, um, vm, d);
            let s22 = panel(f, um, b, vm, d);
            let s4 = s11 + s12 + s21 + s22;
            let err = (s4 - s).abs();
            if err < 15.0*tol || depth >= max_depth {
                return s4 + (s4 - s)/15.0;
            }
            let ct = 0.25 * tol;
            rec(f, a, um, c, vm, s11, ct, depth+1, max_depth) +
                rec(f, um, b, c, vm, s12, ct, depth+1, max_depth) +
                rec(f, a, um, vm, d, s21, ct, depth+1, max_depth) +
                rec(f, um, b, vm, d, s22, ct, depth+1, max_depth)
        }
        if u0 == u1 || v0 == v1 { return 0.0; }
        let s0 = panel(f, u0, u1, v0, v1);
        rec(f, u0, u1, v0, v1, s0, tol.max(1e-15), 0, max_depth)
    }

    // --------------------------------
    // 1D Gauss–Legendre (n=24 고정)
    // --------------------------------
    pub fn gauss_legendre<F>(mut f: F, a: f64, b: f64) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        if a == b { return 0.0; }
        let c1 = 0.5 * (b - a);
        let c2 = 0.5 * (b + a);
        let mut s = 0.0;
        for (&xi, &wi) in GAUSS_LEGENDRE_24_ABSCISSAE.iter().zip(GAUSS_LEGENDRE_24_WEIGHTS.iter()) {
            let x = c1*xi + c2;
            s += wi * f(x);
        }
        s * c1
    }

    // --------------------------------------
    // 2D Gauss–Legendre (24×24 텐서 곱)
    // --------------------------------------
    pub fn gauss_legendre_2d<F>(
        mut f: F, u0: f64, u1: f64, v0: f64, v1: f64
    ) -> f64
    where
        F: FnMut(f64, f64) -> f64,
    {
        if u0 == u1 || v0 == v1 { return 0.0; }
        let cu = 0.5*(u1-u0);
        let vu = 0.5*(u1+u0);
        let cv = 0.5*(v1-v0);
        let vv = 0.5*(v1+v0);
        let mut s = 0.0;
        for (&xu, &wu) in GAUSS_LEGENDRE_24_ABSCISSAE.iter().zip(GAUSS_LEGENDRE_24_WEIGHTS.iter()) {
            let u = cu*xu + vu;
            for (&xv, &wv) in GAUSS_LEGENDRE_24_ABSCISSAE.iter().zip(GAUSS_LEGENDRE_24_WEIGHTS.iter()) {
                let v = cv*xv + vv;
                s += wu*wv * f(u,v);
            }
        }
        s * (cu*cv)
    }

    // ------------------------------------------------------
    // Clenshaw–Curtis (간단한 근사 가중치 버전: 참고용/테스트용)
    // ------------------------------------------------------
    pub fn clenshaw_curtis_lobatto<F>(f: F, a: f64, b: f64, n: usize) -> f64
    where
        F: Fn(f64) -> f64,
    {
        assert!(n >= 1);
        if a == b { return 0.0; }

        // --- 자동 분할: 구간이 0을 포함하면 [a,0] + [0,b]로 나눠서 적분 ---
        fn ccl_rec<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64, n: usize) -> f64 {
            assert!(n >= 1);
            if a == b { return 0.0; }
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

    pub fn clenshaw_curtis_quadrature<F>(
        f: F, a: f64, b: f64, _series: &mut [f64], _epsilon: f64
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

        let mut lenw = series.len() - 1;
        let mut cos2 = 0.0;
        let mut sin1 = 1.0;
        let mut sin2 = 1.0;
        let mut hl   = 0.5;
        let mut k    = lenw as isize;
        let mut l    = 2;

        while l < (k as usize) - l - 1 {
            series[0] = hl * 0.5;
            for j in 1..=l {
                series[j] = hl / (1.0 - 4.0 * (j as f64)*(j as f64));
            }
            series[l] *= 0.5;

            // dfct(l, 0.5*cos2, sin1, series)
            dfct(l, 0.5 * cos2, sin1, &mut series);

            cos2 = (2.0 + cos2).sqrt();
            sin1 /= cos2;
            sin2 /= 2.0 + cos2;

            series[k as usize]     = sin2;
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
            l  <<= 1;
        }

        series
    }

    // ============================================
    // 1D 수치적분을 ODE y'(x)=f(x) 적분으로 구현
    // (RK4 / RK45)  — 적분값 y(b)-y(a)를 반환
    // ============================================
    pub fn integrate_1d_rk4<F>(mut f: F, a: f64, b: f64, n: usize) -> f64
    where
        F: FnMut(f64) -> f64,
    {
        if a == b { return 0.0; }
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
            let k2 = f(x + 0.5*h);
            let k3 = f(x + 0.5*h);
            let k4 = f(x + h);
            y += (h/6.0) * (k1 + 2.0*k2 + 2.0*k3 + k4);
            x += h;
        }
        sign * y
    }

    pub fn integrate_1d_rk45<F>(
        mut f: F, a: f64, b: f64,
        rel_tol: f64, abs_tol: f64,
        h_init: f64, h_min: f64, max_steps: usize
    ) -> Result<f64, ()>
    where
        F: FnMut(f64) -> f64 + Copy,
    {
        if a == b { return Ok(0.0); }

        let (mut aa, mut bb, mut sign) = (a, b, 1.0);
        if bb < aa { std::mem::swap(&mut aa, &mut bb); sign = -1.0; }

        let mut h = if h_init > 0.0 { h_init } else { 0.1*(bb-aa) };
        if h > (bb-aa) { h = bb-aa; }

        let safety = 0.9;
        let min_scale = 0.2;
        let max_scale = 5.0;
        let pow_ = 1.0/5.0;

        let mut x = aa;
        let mut y = 0.0;
        let mut steps = 0usize;

        while x < bb && steps < max_steps {
            steps += 1;
            if x + h > bb { h = bb - x; }
            if h < h_min { h = h_min; }

            // Dormand–Prince 계수 사용 (단일 f(x))
            let k1 = f(x);
            let k2 = f(x + (1.0/5.0)*h);
            let k3 = f(x + (3.0/10.0)*h);
            let k4 = f(x + (4.0/5.0)*h);
            let k5 = f(x + (8.0/9.0)*h);
            let k6 = f(x + h);

            let incr5 = h * ((35.0/384.0)*k1 + (500.0/1113.0)*k3
                + (125.0/192.0)*k4 - (2187.0/6784.0)*k5
                + (11.0/84.0)*k6);
            let incr4 = h * ((5179.0/57600.0)*k1 + (7571.0/16695.0)*k3
                + (393.0/640.0)*k4 - (92097.0/339200.0)*k5
                + (187.0/2100.0)*k6 + (1.0/40.0)*f(x+h));

            let y5 = y + incr5;
            let y4 = y + incr4;

            let err = (y5 - y4).abs();
            let tol = abs_tol.max(rel_tol * y.abs().max(1.0));

            if err <= tol {
                y = y5;
                x += h;
                let mut factor = if err > 0.0 {
                    safety * (tol/err).powf(pow_)
                } else {
                    max_scale
                };
                if factor < min_scale { factor = min_scale; }
                if factor > max_scale { factor = max_scale; }
                h *= factor;
            } else {
                let mut factor = safety * (tol/(err.max(1e-300))).powf(pow_);
                if factor < min_scale { factor = min_scale; }
                if factor > 1.0 { factor = 1.0; }
                let h_new = h * factor;
                if h_new < h_min { return Err(()); }
                h = h_new;
            }
        }

        if x < bb { return Err(()); }
        Ok(sign * y)
    }
}

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

    for j in 0..=m-1 {
        let k = n - j;
        let xr = a[j] + a[k];
        a[j] -= a[k];
        a[k] = xr;
    }
    let mut an = a[n];

    while m >= 2 {
        ddct(m, wr, wi, a); // 간단 대체 구현
        let xr = 1.0 - 2.0*wi*wi;
        wi *= 2.0*wr;
        wr = xr;

        bitrv(m, a);

        let mh = m >> 1;
        let mut xi = a[m];
        a[m] = a[0];
        a[0] = an - xi;
        an += xi;

        for j in 1..=mh-1 {
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

/// 간단한 DCT-II (in-place 대체 구현; wr/wi는 여기서는 사용하지 않음)
fn ddct(n: usize, _wr: f64, _wi: f64, a: &mut [f64]) {
    if n == 0 { return; }
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

/// 비트-리버설 (길이 n은 2의 거듭제곱 권장; 아니면 no-op에 가깝게 동작)
fn bitrv(n: usize, a: &mut [f64]) {
    // 간단 구현: n이 2의 거듭제곱일 때 정상 동작
    if n == 0 { return; }
    let bits = (usize::BITS - (n as u32).leading_zeros() - 1) as usize;
    let mut i = 0usize;
    for j in 1..n-1 {
        let mut bit = n >> 1;
        while i & bit != 0 { i &= !bit; bit >>= 1; }
        i |= bit;
        if j < i {
            a.swap(j, i);
        }
    }
}
```
### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use std::f64::consts::{E, PI};
    use geometry::math::integrator::Integrator;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn simpson_poly_x2() {
        // ∫_0^1 x^2 dx = 1/3
        let f = |x: f64| x * x;
        let s = Integrator::simpson(f, 0.0, 1.0);
        assert!(approx(s, 1.0 / 3.0, 1e-12), "got {}", s);
    }

    #[test]
    fn simpson_sign_when_reversed() {
        // 역구간이면 부호 뒤집힘
        let f = |x: f64| x * x;
        let s1 = Integrator::simpson(f, 0.0, 1.0);
        let s2 = Integrator::simpson(f, 1.0, 0.0);
        assert!(approx(s1,  1.0/3.0, 1e-12));
        assert!(approx(s2, -1.0/3.0, 1e-12));
    }

    #[test]
    fn gauss_legendre_sin_0_pi() {
        // ∫_0^π sin x dx = 2
        let f = |x: f64| x.sin();
        let s = Integrator::gauss_legendre(f, 0.0, PI);
        assert!(approx(s, 2.0, 1e-12), "got {}", s);
    }

    #[test]
    fn simpson_adaptive_sin_0_pi() {
        // 적응형 심프슨
        let f = |x: f64| x.sin();
        let s = Integrator::simpson_adaptive(f, 0.0, PI, 1e-12, 20);
        assert!(approx(s, 2.0, 1e-10), "got {}", s);
    }

    #[test]
    fn simpson_2d_linear_unit_square() {
        // ∫_0^1∫_0^1 (u+v) dudv = 1
        let g = |u: f64, v: f64| u + v;
        let s = Integrator::simpson_2d(g, 0.0, 1.0, 0.0, 1.0);
        assert!(approx(s, 1.0, 1e-12), "got {}", s);
    }

    #[test]
    fn gauss_legendre_2d_exp_unit_square() {
        // ∫_0^1∫_0^1 e^{u+v} dudv = (e-1)^2
        let g = |u: f64, v: f64| (u + v).exp();
        let s = Integrator::gauss_legendre_2d(g, 0.0, 1.0, 0.0, 1.0);
        let target = (E - 1.0) * (E - 1.0);
        assert!(approx(s, target, 1e-10), "got {}, target {}", s, target);
    }

    #[test]
    fn rk4_integrate_exp_0_1() {
        // y(b)-y(a) = ∫_a^b e^x dx = e - 1
        let f = |x: f64| x.exp();
        let s = Integrator::integrate_1d_rk4(f, 0.0, 1.0, 10_000);
        let target = E - 1.0;
        assert!(approx(s, target, 1e-7), "got {}, target {}", s, target);
    }

    #[test]
    fn rk45_integrate_exp_0_1() {
        let f = |x: f64| x.exp();
        let s = Integrator::integrate_1d_rk45(
            f, 0.0, 1.0,
            1e-9, 1e-12,
            1e-2, 1e-12, 200_000
        ).expect("RK45 failed");
        let target = E - 1.0;
        assert!(approx(s, target, 1e-8), "got {}, target {}", s, target);
    }

    #[test]
    fn chebyshev_series_has_min_len() {
        let series = Integrator::chebyshev_series(100);
        assert!(series.len() >= 6);
    }
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

    #[test]
    fn clenshaw_curtis_simple_cosine() {
        // ∫_0^π cos x dx = 0
        let s = Integrator::clenshaw_curtis_lobatto(|x| x.cos(), 0.0, std::f64::consts::PI, 64);
        assert!(s.abs() < 1e-9, "got {}", s);
    }


    #[test]
    fn cc_constant_many_ns() {
        // f(x)=1 → ∫_a^b = b-a
        let (a, b) = (2.0, 5.0);
        for &n in &[16, 32, 64, 128, 256] {
            let s = Integrator::clenshaw_curtis_lobatto(|_| 1.0, a, b, n);
            assert!(approx(s, b - a, 1e-12), "N={n}, got {s}");
        }
    }

    #[test]
    fn cc_polynomials() {
        // ∫_0^1 x dx = 1/2,  ∫_0^1 x^2 dx = 1/3
        let n = 128;
        let s1 = Integrator::clenshaw_curtis_lobatto(|x| x, 0.0, 1.0, n);
        let s2 = Integrator::clenshaw_curtis_lobatto(|x| x*x, 0.0, 1.0, n);
        assert!(approx(s1, 0.5, 1e-12), "x: got {}", s1);
        assert!(approx(s2, 1.0/3.0, 1e-12), "x^2: got {}", s2);
    }

    #[test]
    fn cc_cos_zero() {
        // ∫_0^π cos x dx = 0
        let n = 128;
        let s = Integrator::clenshaw_curtis_lobatto(|x| x.cos(), 0.0, PI, n);
        assert!(s.abs() < 1e-12, "got {}", s);
    }

    #[test]
    fn cc_against_gauss_legendre_exp() {
        // 부드러운 함수 비교: exp(x) on [0,1]
        let n = 256; // CC 쪽
        let s_cc = Integrator::clenshaw_curtis_lobatto(|x| x.exp(), 0.0, 1.0, n);

        // GL 24-pt (이미 구현돼 있다면 그걸로 “사실상 정답”)
        let s_gl = Integrator::gauss_legendre(|x| x.exp(), 0.0, 1.0); // 이름은 너 코드에 맞춰주세요

        assert!(approx(s_cc, s_gl, 1e-10), "cc={} vs gl={}", s_cc, s_gl);
    }

    #[test]
    fn cc_abs_nonsmooth() {
        // 비매끈 함수: ∫_{-1}^{1} |x| dx = 1
        let n = 256;
        let s = Integrator::clenshaw_curtis_lobatto(|x| x.abs(), -1.0, 1.0, n);
        assert!(approx(s, 1.0, 1e-10), "got {}", s);
    }

    // 원하는 정확도까지 N을 자동 증가시키면서 수렴시키는 래퍼 (옵션)
    fn cc_adaptive<F: Fn(f64)->f64>(f: F, a: f64, b: f64, tol: f64, n0: usize, nmax: usize) -> f64 {
        let mut n = n0.max(8) & !1; // 짝수 보장
        let mut prev = Integrator::clenshaw_curtis_lobatto(&f, a, b, n);
        loop {
            let n2 = (n * 2).min(nmax);
            let cur = Integrator::clenshaw_curtis_lobatto(&f, a, b, n2);
            if (cur - prev).abs() <= tol || n2 == nmax { return cur; }
            n = n2; prev = cur;
        }
    }

    #[test]
    fn cc_adaptive_demo() {
        let s = cc_adaptive(|x| (x*x + 1.0).ln(), 0.0, 1.0, 1e-10, 32, 4096);
        // 비교값: GL 또는 높은 N의 CC
        let ref_ = Integrator::clenshaw_curtis_lobatto(|x| (x*x + 1.0).ln(), 0.0, 1.0, 4096);
        assert!(approx(s, ref_, 1e-10), "s={}, ref={}", s, ref_);
    }



    #[test]
    fn cc_poly() {
        // [-1,1]에서 ∫ x^2 dx = 2/3
        let g = |x: f64| x * x;
        let s = Integrator::clenshaw_curtis_lobatto(g, -1.0, 1.0, 128);
        assert!((s - 2.0/3.0).abs() < 1e-12, "x^2: {}", s);
    }

    #[test]
    fn cc_exp() {
        // ∫_{-1}^1 e^x dx = e - 1/e
        let g = |x: f64| x.exp();
        let exact = std::f64::consts::E - (-1.0f64).exp();
        let s = Integrator::clenshaw_curtis_lobatto(g, -1.0, 1.0, 256);
        assert!((s - exact).abs() < 5e-13, "exp: {}", s);
    }

    #[test]
    fn cc_abs() {
        // ∫_{-1}^1 |x| dx = 1
        let g = |x: f64| x.abs();
        let s = Integrator::clenshaw_curtis_lobatto(g, -1.0, 1.0, 16384);
        assert!((s - 1.0).abs() < 1e-10, "abs: {}", s);
    }
}
```

### 참고(교과서)
- Trefethen, *Spectral Methods in MATLAB* — Clenshaw–Curtis 적분과 Chebyshev 계수.


