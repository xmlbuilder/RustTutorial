## **커널 수학 레이어** 로 정리.
- 각 함수의 수학적 의미
- 정확한 수식

### 1. 공통 전제 (곡선/곡면 미분기하)
- 3D 곡선 $\mathbf{r}(t)$ 에서:
- 1차 도함수: $\mathbf{D1}=\mathbf{r}'(t)$
- 2차 도함수: $\mathbf{D2}=\mathbf{r}''(t)$
- 3차 도함수: $\mathbf{D3}=\mathbf{r}'''(t)$
- 단위 접선:
```math
\mathbf{T}=\frac{\mathbf{D1}}{\| \mathbf{D1}\| }
```
- 곡률 벡터:
```math
\mathbf{K}=\frac{\mathbf{D2}-(\mathbf{D2}\cdot \mathbf{T})\mathbf{T}}{\mathbf{D1}\cdot \mathbf{D1}}
```
- 스칼라 곡률 $k=\| \mathbf{K}\|$ 

- 법선 벡터 
```math
\mathbf{N}=\mathbf{K}/k ( k>0 일 때)
```
- 표면 $\mathbf{S}(u,v)$ 에서:
    - $\mathbf{S_{\mathnormal{u}}}=S_{10}, \mathbf{S_{\mathnormal{v}}}=S_{01}$
- $\mathbf{S_{\mathnormal{uu}}}=S_{20}, \mathbf{S_{\mathnormal{uv}}}=S_{11}, \mathbf{S_{\mathnormal{vv}}}=S_{02}$

### 2. on_ev_curvature_mut (D1, D2 → T, K)
- 함수 의미
```rust
pub fn on_ev_curvature_mut(
    d1: Vector3D,   // 1차 도함수 r'(t)
    d2: Vector3D,   // 2차 도함수 r''(t)
    t_out: &mut Vector3D, // 단위 접선 T
    k_out: &mut Vector3D, // 곡률 벡터 K
) -> bool
```

- 입력: 곡선의 1·2차 도함수
- 출력:
    - t_out = 단위 접선 $\mathbf{T}$
    - k_out = 곡률 벡터 $\mathbf{K}$
- 수학:
```math
\mathbf{T}=\frac{\mathbf{D1}}{\| \mathbf{D1}\| }
```
- 특이 케이스:
- $\| \mathbf{D1}\| =0$ 이면 D2로부터 단위 방향만 결정하고, 곡률은 0으로 둔다.

### 소스 코드
```rust
/// Evaluate unit tangent T and curvature vector K from first and second derivatives.
///
/// 수식:
///   T = D1 / |D1|
///   K = ( D2 - (D2·T) * T ) / (D1·D1)
///
/// 반환값:
///   true  = 정상 계산
///   false = 퇴화(D1, D2 모두 0 근처)
pub fn on_ev_curvature_mut(
    d1: Vector3D,   // first derivative
    d2: Vector3D,   // second derivative
    t_out: &mut Vector3D, // unit tangent
    k_out: &mut Vector3D, // curvature vector
) -> bool {
    let mut d1_len = d1.length();

    if d1_len == 0.0 {
        // L'Hospital: D1 -> 0 이고 D2 != 0 이면
        // unit tangent ≈ ± unit(D2)
        d1_len = d2.length();
        if d1_len > 0.0 {
            *t_out = d2 / d1_len;
        } else {
            *t_out = Vector3D::ZERO_VECTOR;
        }
        *k_out = Vector3D::ZERO_VECTOR;
        false
    } else {
        // 정상적인 경우
        *t_out = d1 / d1_len;
        let neg_d2_dot_t = -d2.dot(t_out);
        let inv_d1_sq = 1.0 / (d1_len * d1_len);
        *k_out = (d2 + t_out.scale(neg_d2_dot_t)) * inv_d1_sq;
        true
    }
}
```

### 테스트 코드 예시
```rust
#[test]
fn test_ev_curvature_mut_circle() {
    // r(t) = (cos t, sin t, 0), k = 1
    use crate::core::geom::Vector3D;
    use crate::core::curv::{ev_curvature_mut};

    let t = 0.3;

    let d1 = Vector3D::new(-t.sin(), t.cos(), 0.0);      // r'(t)
    let d2 = Vector3D::new(-t.cos(), -t.sin(), 0.0);     // r''(t)

    let mut T = Vector3D::ZERO_VECTOR;
    let mut K = Vector3D::ZERO_VECTOR;

    let ok = ev_curvature_mut(d1, d2, &mut T, &mut K);
    assert!(ok);

    // T는 단위 벡터
    assert!((T.length() - 1.0).abs() < 1e-12);

    // 곡률 벡터 크기는 1
    let k = K.length();
    assert!((k - 1.0).abs() < 1e-6);
}
```

## 3. on_ev_curvature_mut_1der (k', torsion까지)
- 함수 의미
```rust
pub fn on_ev_curvature_mut_1der(
    d1: Vector3D,       // r'(t)
    d2: Vector3D,       // r''(t)
    d3: Vector3D,       // r'''(t)
    t_out: &mut Vector3D,
    k_out: &mut Vector3D,
    kprime_out: Option<&mut Real>,   // 곡률 k'(t)
    torsion_out: Option<&mut Real>,  // 비틀림 τ(t)
) -> bool
```

- 수학:
- 곡률:
```math
k=\frac{\| \mathbf{D1}\times \mathbf{D2}\| }{\| \mathbf{D1}\| ^3}
```
- 단위 접선:
```math
\mathbf{T}=\frac{\mathbf{D1}}{\| \mathbf{D1}\| }
```
- 곡률 벡터:
```math
\mathbf{K}=\frac{\mathbf{D2}-(\mathbf{D2}\cdot \mathbf{T})\mathbf{T}}{\mathbf{D1}\cdot \mathbf{D1}}
```
- 곡률의 도함수 k':
```math
\mathbf{q}=\mathbf{D1}\times \mathbf{D2},\quad \mathbf{q'}=\mathbf{D1}\times \mathbf{D3}
```
- 비틀림 $\tau$ :
```math
\tau =\frac{(\mathbf{D1}\times \mathbf{D2})\cdot \mathbf{D3}}{\| \mathbf{D1}\times \mathbf{D2}\| ^2}
```

### 소스 코드
```rust
/// EvCurvature + 곡률의 1차 도함수 k'(t), 비틀림(torsion)까지 계산.
///
/// D1 : 1차 도함수
/// D2 : 2차 도함수
/// D3 : 3차 도함수
///
/// k    = |D1 x D2| / |D1|^3
/// k'   = d/dt k(t)
/// tau  = ((D1 x D2) · D3) / |D1 x D2|^2
pub fn on_ev_curvature_1der_mut(
    d1: Vector3D,
    d2: Vector3D,
    d3: Vector3D,
    t_out: &mut Vector3D,
    k_out: &mut Vector3D,
    kprime_out: Option<&mut Real>,
    torsion_out: Option<&mut Real>,
) -> bool {
    let dsdt = d1.length();
    if dsdt <= 0.0 {
        *t_out = Vector3D::ZERO_VECTOR;
        *k_out = Vector3D::ZERO_VECTOR;
        if let Some(kp) = kprime_out { *kp = 0.0; }
        if let Some(tau) = torsion_out { *tau = 0.0; }
        return false;
    }

    *t_out = d1 * (1.0 / dsdt);

    // q = D1 x D2
    let q = d1.cross(&d2);
    let q_len2 = q.length_squared();
    let dsdt2 = dsdt * dsdt;

    // K = (D2 - (D2·T)T) / |D1|^2
    *k_out = (d2 - *t_out * d2.dot(t_out)) * (1.0 / dsdt2);

    let mut rc = false;

    // k'
    if let Some(kp) = kprime_out {
        let qprime = d1.cross(&d3);
        if q_len2 > 0.0 {
            let num = (q * qprime) * d1.length_squared() - 3.0 * q_len2 * (d1 * d2);
            let den = q_len2.sqrt() * dsdt.powf(5.0);
            *kp = num / den;
        } else {
            *kp = qprime.length() / dsdt.powf(3.0);
        }
        rc = true;
    }

    // torsion
    if let Some(tau) = torsion_out {
        if q_len2 > 0.0 {
            *tau = (q * d3) / q_len2;
            rc = true;
        } else {
            rc = false;
        }
    }

    rc
}
```

### 간단 테스트 예시
- 비틀림/곡률이 일정한 곡선(원, 원통 나사선 등)을 이용하면 좋다.
- 예시는 개념 정도로:
```rust
#[test]
fn test_ev_curvature_mut_1der_circle() {
    use crate::core::geom::Vector3D;
    use crate::core::curv::ev_curvature_mut_1der;

    let t = 0.4;
    // r(t) = (cos t, sin t, 0)
    let d1 = Vector3D::new(-t.sin(), t.cos(), 0.0);
    let d2 = Vector3D::new(-t.cos(), -t.sin(), 0.0);
    let d3 = Vector3D::new(t.sin(), -t.cos(), 0.0);

    let mut T = Vector3D::ZERO_VECTOR;
    let mut K = Vector3D::ZERO_VECTOR;
    let mut kprime = 0.0;
    let mut torsion = 0.0;

    let ok = ev_curvature_mut_1der(
        d1, d2, d3,
        &mut T,
        &mut K,
        Some(&mut kprime),
        Some(&mut torsion),
    );
    assert!(ok);

    // 원은 곡률 1, k' = 0, torsion = 0
    assert!((K.length() - 1.0).abs() < 1e-6);
    assert!(kprime.abs() < 1e-5);
    assert!(torsion.abs() < 1e-5);
}
```

## 4. on_normal_curvature (주어진 방향에 대한 정규 곡률)
- 함수 의미
```rust
pub fn on_normal_curvature(
    s10: Vector3D,   // S_u
    s01: Vector3D,   // S_v
    s20: Vector3D,   // S_uu
    s11: Vector3D,   // S_uv
    s02: Vector3D,   // S_vv
    unit_normal: Vector3D,   // 표면 법선 N
    unit_tangent: Vector3D,  // 표면 위 방향 T
) -> Vector3D   // normal curvature vector (k_n N)
```

- 개념:
- 표면 위에서 주어진 방향 $\mathbf{T}$ 에 대한 정규 곡률 k_n 을 구하는 함수.
- 수학적 절차:
```math
- \mathbf{T}=a\mathbf{S_{\mathnormal{u}}}+b\mathbf{S_{\mathnormal{v}}} 
```
- 를 풀기 (solve 3x2)
- 방향 곡선 정의:
```math
\mathbf{C}(t)=\mathbf{S}(u_0+at,\  v_0+bt)- \mathbf{C}''(0) 
```
- 계산:
```math
\mathbf{D2}=a^2\mathbf{S_{\mathnormal{uu}}}+2ab\mathbf{S_{\mathnormal{uv}}}+b^2\mathbf{S_{\mathnormal{vv}}}
```
- 이 곡선의 곡률 벡터 $\mathbf{K}$ 를 on_ev_curvature로 계산
- 법선 방향 성분:
```math
k_n=\mathbf{K}\cdot \mathbf{N},\quad \mathrm{NormalCurvature}=k_n\mathbf{N}
```
### 소스 코드
```rust
/// 법선곡률 벡터
///
/// 주어진 단위 접선 T 방향의 **곡면 상 곡선**의 2차 도함수를 이용해
/// 곡률벡터 K 를 평가하고, N 방향 성분만 취합니다.
/// 일반적으로
/// ```text
/// γ(t) = S(u0 + a t, v0 + b t)
/// γ'(0) = a S_u + b S_v  ≡  T
/// γ''(0) = a^2 S_{uu} + 2ab S_{uv} + b^2 S_{vv}
/// 곡률벡터 K = proj_{N} ( κ_n N ) = (K·N) N
/// ```
pub fn on_normal_curvature(
    s10: Vector3D,
    s01: Vector3D,
    s20: Vector3D,
    s11: Vector3D,
    s02: Vector3D,
    unit_normal: Vector3D,
    unit_tangent: Vector3D,
) -> Vector3D {
    // T = a S_u + b S_v 를 푸는 대신, a,b 를 최소제곱으로 계산
    // 여기서는 간단히 Gram 시스템을 푼다.
    // [⟨Su,Su⟩ ⟨Su,Sv⟩][a] = [⟨Su,T⟩]
    // [⟨Sv,Su⟩ ⟨Sv,Sv⟩][b]   [⟨Sv,T⟩]
    let e = s10.dot(&s10);
    let f = s10.dot(&s01);
    let g = s01.dot(&s01);
    let det = e * g - f * f;
    if det == 0.0 {
        return Vector3D::zero();
    }
    let rhs_a = s10.dot(&unit_tangent);
    let rhs_b = s01.dot(&unit_tangent);
    let a = (g * rhs_a - f * rhs_b) / det;
    let b = (-f * rhs_a + e * rhs_b) / det;

    let d2 = s20 * (a * a) + s11 * (2.0 * a * b) + s02 * (b * b);

    // 곡률 벡터 K: K = γ''(0) 의 법선 성분
    let _t_vec = unit_tangent; // 필요 시 반환 확인용
    // N·K
    let kn = d2.dot(&unit_normal);
    unit_normal * kn
}
```

### 테스트 아이디어
- 가장 간단한 테스트:
    - 표면: 평면 z=0 → 모든 방향에서 정규 곡률 0
    - 표면: 구 $x^2+y^2+z^2=R^2$ → 모든 방향에서 $k_n=1/R$
- 예: 구 표면에서 테스트 (개략):
```rust
#[test]
fn test_normal_curvature_on_sphere() {

    let r = 2.0;

    // u = 0, v = 0 에서의 analytic derivatives
    let s10 = Vector3D::new(0.0, r, 0.0);     // S_u
    let s01 = Vector3D::new(0.0, 0.0, r);     // S_v

    let s20 = Vector3D::new(-r, 0.0, 0.0);    // S_uu
    let s11 = Vector3D::new(0.0, 0.0, 0.0);   // S_uv
    let s02 = Vector3D::new(-r, 0.0, 0.0);    // S_vv

    // 법선은 반지름 방향
    let n = Vector3D::new(1.0, 0.0, 0.0);     // unit normal
    // 접선 방향은 S_u 방향으로 선택
    let t = Vector3D::new(0.0, 1.0, 0.0);     // tangent direction

    let kn_vec = on_normal_curvature(s10, s01, s20, s11, s02, n, t);
    let kn = kn_vec.dot(&n);

    println!("kn = {}, expected = {}", kn, 1.0 / r);

    // 부호는 표면 방향/파라미터화에 따라 ±가 될 수 있으니 절대값 비교
    assert!((kn.abs() - 1.0 / r).abs() < 1e-6);
}
```

## 5. on_ev_sectional_curvature_mut (surface–plane 교선 곡률)
- 함수 의미
```rust
pub fn ev_sectional_curvature(
    s10: Vector3D,     // S_u
    s01: Vector3D,     // S_v
    s20: Vector3D,     // S_uu
    s11: Vector3D,     // S_uv
    s02: Vector3D,     // S_vv
    plane_normal: Vector3D,
    k_out: &mut Vector3D,  // 교선의 곡률 벡터
) -> bool
```
- 수학적 의미
    - 표면 $\mathbf{S}(u,v)$ 와 평면(법선 $\mathbf{n}$) 의 교선 $\mathbf{C}(t)$ 에서
    - 곡률 벡터 $\mathbf{K}$ 를 구하는 함수.
- 아이디어:
- 표면 법선:
```math
\mathbf{M}=\mathbf{S_{\mathnormal{u}}}\times \mathbf{S_{\mathnormal{v}}}
```
- 교선의 1차 도함수 (접선):
```math
\mathbf{D1}=\mathbf{M}\times \mathbf{n}
```
- 이 $\mathbf{D1}$ 이 parametric curve의 도함수도 되어야 하므로:
```math
\mathbf{D1}=a\mathbf{S_{\mathnormal{u}}}+b\mathbf{S_{\mathnormal{v}}}
```
- 를 solve_3x2로 풀어 a,b 계산.
- $\mathbf{M}$ 의 도함수(교선 방향으로):
```math
\mathbf{M_{\mathnormal{1}}}=(a\mathbf{S_{\mathnormal{uu}}}+b\mathbf{S_{\mathnormal{uv}}})\times \mathbf{S_{\mathnormal{v}}}+\mathbf{S_{\mathnormal{u}}}\times (a\mathbf{S_{\mathnormal{uv}}}+b\mathbf{S_{\mathnormal{vv}}})
```
- 교선의 2차 도함수:
```math
\mathbf{D2}=\mathbf{M_{\mathnormal{1}}}\times \mathbf{n}- 곡률 벡터:
```
- 간단 테스트 아이디어
    - 표면: 평면 + 평면 → 교선은 직선 → 곡률 = 0
    - 표면: sphere, 평면: great circle → 교선은 원 → 곡률 = 1/R


### 소스 코드
```rust
/// 표면과 평면의 교선(curve)의 곡률 벡터 K를 계산.
///
/// 입력:
///   S10, S01 : Su, Sv (1차 편미분)
///   S20, S11, S02 : Suu, Suv, Svv (2차 편미분)
///   plane_normal : 절단 평면 법선
///
/// 출력:
///   K : 교선에서의 곡률 벡터
///
/// 반환:
///   true  = 정상 계산
///   false = 퇴화 (Su, Sv 선형 종속 등)
pub fn on_ev_sectional_curvature_mut(
    s10: Vector3D,
    s01: Vector3D,
    s20: Vector3D,
    s11: Vector3D,
    s02: Vector3D,
    plane_normal: Vector3D,
    k_out: &mut Vector3D,
) -> bool {
    let mut m = Vector3D::ZERO_VECTOR;
    let mut d1 = Vector3D::ZERO_VECTOR;
    let mut d2 = Vector3D::ZERO_VECTOR;

    let mut a: Real = 0.0;
    let mut b: Real = 0.0;
    let mut e: Real = 0.0;
    let mut pr: Real = 0.0;

    // M = Su x Sv (표면 법선)
    m.x = s10.y * s01.z - s01.y * s10.z;
    m.y = s10.z * s01.x - s01.z * s10.x;
    m.z = s10.x * s01.y - s01.x * s10.y;

    // D1 = (Su x Sv) x plane_normal
    d1.x = m.y * plane_normal.z - plane_normal.y * m.z;
    d1.y = m.z * plane_normal.x - plane_normal.z * m.x;
    d1.z = m.x * plane_normal.y - plane_normal.x * m.y;

    // D1 = a * Su + b * Sv 풀기
    let rank = on_solve_3x2_su_sv(
        &s10,
        &s01,
        d1.x,
        d1.y,
        d1.z,
        &mut a,
        &mut b,
        &mut e,
        &mut pr,
    );
    if rank < 2 {
        *k_out = Vector3D::ZERO_VECTOR;
        return false;
    }

    // M1 = (a*Suu + b*Suv) x Sv  +  Su x (a*Suv + b*Svv)
    d2.x = a * s20.x + b * s11.x;
    d2.y = a * s20.y + b * s11.y;
    d2.z = a * s20.z + b * s11.z;
    m.x = d2.y * s01.z - s01.y * d2.z;
    m.y = d2.z * s01.x - s01.z * d2.x;
    m.z = d2.x * s01.y - s01.x * d2.y;

    d2.x = a * s11.x + b * s02.x;
    d2.y = a * s11.y + b * s02.y;
    d2.z = a * s11.z + b * s02.z;
    m.x += s10.y * d2.z - d2.y * s10.z;
    m.y += s10.z * d2.x - d2.z * s10.x;
    m.z += s10.x * d2.y - d2.x * s10.y;

    // D2 = M1 x plane_normal = 교선의 2차 도함수
    d2.x = m.y * plane_normal.z - plane_normal.y * m.z;
    d2.y = m.z * plane_normal.x - plane_normal.z * m.x;
    d2.z = m.x * plane_normal.y - plane_normal.x * m.y;

    // 곡률 벡터 K = (D2 - (D2·D1)/(D1·D1) * D1) / (D1·D1)
    let mut denom = d1.x * d1.x + d1.y * d1.y + d1.z * d1.z;
    if !(denom > f64::MIN_POSITIVE) {
        *k_out = Vector3D::ZERO_VECTOR;
        return false;
    }

    let inv = 1.0 / denom;
    let proj = -(d2.x * d1.x + d2.y * d1.y + d2.z * d1.z) * inv;

    k_out.x = inv * (d2.x + proj * d1.x);
    k_out.y = inv * (d2.y + proj * d1.y);
    k_out.z = inv * (d2.z + proj * d1.z);

    true
}
```

### 테스트 코드
```rust
#[test]
fn test_ev_sectional_curvature_plane_plane() {
    // 표면: z=0
    let s10 = Vector3D::new(1.0, 0.0, 0.0); // S_u
    let s01 = Vector3D::new(0.0, 1.0, 0.0); // S_v
    let s20 = Vector3D::ZERO_VECTOR;
    let s11 = Vector3D::ZERO_VECTOR;
    let s02 = Vector3D::ZERO_VECTOR;

    // 절단 평면: x=0 → normal = (1,0,0)
    let plane_normal = Vector3D::new(1.0, 0.0, 0.0);

    let mut K = Vector3D::ZERO_VECTOR;
    let ok = on_ev_sectional_curvature_mut(s10, s01, s20, s11, s02, plane_normal, &mut K);

    assert!(ok);
    assert!(K.length() < 1e-12); // 직선이므로 곡률 0
}
```
