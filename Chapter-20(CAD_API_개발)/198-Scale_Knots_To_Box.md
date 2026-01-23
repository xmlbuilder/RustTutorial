# scale_knots_to_box

- 이 코드는 NURBS Surface Function(SFun)의 U/V 방향 knot vector를  
    주어진 사각형(Box2D) 범위로 선형 스케일링(affine transform)하는 함수.

## 🎯 1. 함수의 목적
- scale_knots_to_box(rect, dir)는 다음을 수행한다:
- SFun의 U knot vector 또는 V knot vector를  
    주어진 구간 [a, b], [c, d]로 affine 변환한다.
- 즉, 기존 knot 범위 [u0, ur]를 새로운 범위 [a, b]로 매핑한다.

- 수식은 다음과 같다:
```math
u'=a+(u-u_0)\frac{b-a}{u_r-u_0}
```
- V 방향도 동일:
```math
v'=c+(v-v_0)\frac{d-c}{v_s-v_0}
```
- 그리고 clamped B-spline의 특성에 따라:
    - 시작 부분 p+1개는 모두 a
    - 끝 부분 p+1개는 모두 b
    - 내부 knot만 affine 변환

## 🎯 2. 코드 구조 설명
- ✔ U 방향 처리
```rust
let a = rect.x.t0;
let b = rect.x.t1;

let u0 = self.ku.knots[0];
let ur = self.ku.knots[r];
```

- u0 = U knot의 첫 값
- ur = U knot의 마지막 값
- 기존 knot 범위 [u0, ur]를 [a, b]로 변환

## ✔ affine factor 계산
```rust
let fac = (b - a) / denom;
```

- 여기서 denom = ur - u0.
- ✔ 시작 clamped 구간
```rust
for i in 0..=p {
    self.ku.knots[i] = a;
}
```
- B-spline degree가 p이면
    - 시작 knot는 p+1개가 동일해야 한다.
- ✔ 내부 knot 변환
```rust
for i in (p + 1)..=(r - p - 1) {
    self.ku.knots[i] = fac * (self.ku.knots[i] - u0) + a;
}
```


- 수식 그대로:
```math
u'_i=a+(u_i-u_0)\frac{b-a}{u_r-u_0}
```
- ✔ 끝 clamped 구간
```rust
for i in (r - p)..=r {
    self.ku.knots[i] = b;
}
```

- ✔ V 방향도 동일한 구조
    - 시작 q+1개 = c
    - 끝 q+1개 = d
    - 내부 knot만 affine 변환

## 🎯 3. 이 함수가 왜 중요한가
- NURBS surface는:
    - U/V knot vector가 surface의 parameter domain을 정의한다.
    - knot vector를 스케일링하면 surface의 parameterization이 바뀐다.
    - geometry는 바뀌지 않고 parameter domain만 재조정된다.
- 즉, 이 함수는:
    - surface를 다른 좌표계로 매핑하거나
    - parameter domain을 정규화하거나
    - 여러 surface를 동일한 domain으로 맞출 때
- 필수적인 기능이다.

## 🎯 4. 테스트 코드의 의미

- ✔ 테스트 1: scale_knots_to_box_scales_u_only_and_preserves_values
- 검증 내용:
    - V knot은 그대로 유지
    - U knot의 시작/끝 clamped 구간이 정확히 a,b로 설정됨
    - 내부 knot가 정확한 affine 변환을 따름
    - values 배열은 절대 변하지 않음
- 즉, U 방향만 정확히 스케일링되는지 검증.

- ✔ 테스트 2: scale_knots_to_box_scales_v_only
- 검증 내용:
    - U는 그대로
    - V만 스케일링
    - clamped 구간(q+1개)이 정확히 c,d로 설정
    - 내부 knot affine 변환
    - degree(q)에 따라 clamped 구간이 정확히 유지되는지

- ✔ 테스트 3: scale_knots_to_box_scales_uv_both
    - U와 V 둘 다 스케일링
    - clamped 구간 정확성
    - affine 변환 정확성

- ✔ 테스트 4: scale_knots_to_box_noop_when_ranges_already_match
- 이미 [u0, ur] == [a, b]이면 아무것도 하지 않아야 한다.
- 즉, 불필요한 연산을 하지 않는지 검증.

## 🎯 5. 전체 기능 요약
- 이 함수는:
    - NURBS surface function의 knot vector를 새로운 구간으로 선형 스케일링
    - clamped B-spline의 구조(p+1 multiplicity)를 유지
    - geometry는 바꾸지 않고 parameter domain만 조정
    - U/V 방향을 선택적으로 스케일링
    - in-place로 수행
    - values는 절대 변경하지 않음
- 그리고 테스트는:
    - clamped 구간 검증
    - affine 변환 검증
    - no-op 조건 검증
    - values 불변성 검증
- 즉, 수학적으로 완벽한 NURBS knot scaling 함수.

```rust
impl SFun {
    /// Scale surface-function knot vectors to a given rectangle.
    /// The operation is IN-PLACE (original knots are destroyed).
    pub fn scale_knots_to_box(&mut self, rect: Box2D, dir: KnotScaleDir) {
        let p = self.pu as usize;
        let q = self.pv as usize;

        let r = self.ku.knots.len() - 1;
        let s = self.kv.knots.len() - 1;

        // ---- U direction ----
        if matches!(dir, KnotScaleDir::U | KnotScaleDir::UV) {
            let a = rect.x.t0;
            let b = rect.x.t1;

            let u0 = self.ku.knots[0];
            let ur = self.ku.knots[r];

            if a != u0 || b != ur {
                let denom = ur - u0;
                debug_assert!(denom != 0.0, "degenerate U knot range");

                let fac = (b - a) / denom;

                // start clamped
                for i in 0..=p {
                    self.ku.knots[i] = a;
                }

                // internal knots
                for i in (p + 1)..=(r - p - 1) {
                    self.ku.knots[i] = fac * (self.ku.knots[i] - u0) + a;
                }

                // end clamped
                for i in (r - p)..=r {
                    self.ku.knots[i] = b;
                }
            }
        }

        // ---- V direction ----
        if matches!(dir, KnotScaleDir::V | KnotScaleDir::UV) {
            let c = rect.y.t0;
            let d = rect.y.t1;

            let v0 = self.kv.knots[0];
            let vs = self.kv.knots[s];

            if c != v0 || d != vs {
                let denom = vs - v0;
                debug_assert!(denom != 0.0, "degenerate V knot range");

                let fac = (d - c) / denom;

                // start clamped
                for j in 0..=q {
                    self.kv.knots[j] = c;
                }

                // internal knots
                for j in (q + 1)..=(s - q - 1) {
                    self.kv.knots[j] = fac * (self.kv.knots[j] - v0) + c;
                }

                // end clamped
                for j in (s - q)..=s {
                    self.kv.knots[j] = d;
                }
            }
        }
    }
}
```
---
# 테스트 코드
```rust
use nurbslib::core::box2d::Box2D;
use nurbslib::core::cfun::{Degree, Index};
use nurbslib::core::prelude::Interval;
use nurbslib::core::sfun::{ensure_sfun_shape, KnotScaleDir, SFun};


fn approx(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

/// 내부 knot들이 affine 변환으로 맞는지 확인:
/// new = a + (old - u0) * (b-a)/(u1-u0)
fn expected_affine(old: f64, old0: f64, old1: f64, a: f64, b: f64) -> f64 {
    let fac = (b - a) / (old1 - old0);
    fac * (old - old0) + a
}
```
```rust
#[test]
fn scale_knots_to_box_scales_u_only_and_preserves_values() {
    // degree (p,q) = (2,2), control net (n,m) = (4,3) => nu=5, nv=4
    let (n, m, p, q) = (4usize, 3usize, 2usize, 2usize);

    let r = n + p + 1; // highest knot index in U
    let s = m + q + 1; // highest knot index in V

    let mut sfn = SFun::new_empty();
    ensure_sfun_shape(&mut sfn, n as Index, m as Index,
        p as Degree, q as Degree, r as Index, s as Index);

    // values를 랜덤/패턴으로 채워서 "불변" 확인
    for (k, v) in sfn.values.iter_mut().enumerate() {
        *v = (k as f64) * 0.123 + 7.0;
    }
    let values_before = sfn.values.clone();

    // U knot: clamped [0,0,0, 0.2,0.7, 1,1,1] (len = r+1 = 8)
    sfn.ku.knots = vec![0.0, 0.0, 0.0, 0.2, 0.7, 1.0, 1.0, 1.0];
    // V knot은 건드리지 않도록 적당히
    sfn.kv.knots = vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0]; // (예: s+1=8에 맞춤)

    let u_before = sfn.ku.knots.clone();
    let v_before = sfn.kv.knots.clone();

    // 새 U 범위 [10, 20], V는 그대로 유지되길 기대
    let rect = Box2D {
        x: Interval::new(10.0, 20.0),
        y: Interval::new(-5.0, 5.0),
    };

    sfn.scale_knots_to_box(rect, KnotScaleDir::U);

    // 1) V knot unchanged
    assert_eq!(sfn.kv.knots, v_before);

    // 2) U knot clamped ends
    for i in 0..=p {
        assert!(approx(sfn.ku.knots[i], 10.0, 1e-12));
    }
    for i in (r - p)..=r {
        assert!(approx(sfn.ku.knots[i], 20.0, 1e-12));
    }

    // 3) 내부 knot affine 변환
    // internal indices: p+1 .. r-p-1  => 3..4
    let old0 = u_before[0];
    let old1 = u_before[r];
    let a = 10.0;
    let b = 20.0;

    for i in (p + 1)..=(r - p - 1) {
        let exp = expected_affine(u_before[i], old0, old1, a, b);
        assert!(
            approx(sfn.ku.knots[i], exp, 1e-12),
            "U internal knot mismatch at i={i}: got {}, exp {}",
            sfn.ku.knots[i],
            exp
        );
    }

    // 4) values unchanged
    assert_eq!(sfn.values, values_before);
}
```
```rust
#[test]
fn scale_knots_to_box_scales_v_only() {
    // (n,m)만 정하고 ensure가 p,q까지 세팅한다고 가정
    let (n, m) = (2usize, 2usize);

    // 너 프로젝트의 ensure_sfun_shape 시그니처에 맞춰서 호출
    let mut sfn = SFun::new_empty();

    // ⚠️ 여기서 p,q,r,s는 ensure가 요구하는대로 넣어야 함
    // 만약 ensure가 (n,m,p,q,r,s)를 요구한다면 일단 네가 의도한 p,q로 넣되,
    // 아래에서 반드시 sfn.pu/pv를 다시 읽어서 "실제 적용된" p,q를 사용한다.
    let p_in = 2usize;
    let q_in = 2usize;
    let r_in = n + p_in + 1;
    let s_in = m + q_in + 1;

    ensure_sfun_shape(
        &mut sfn,
        n, m,
        p_in as Degree, q_in as Degree,
        r_in, s_in
    );

    // ✅ 실제 p,q를 SFun에서 다시 읽는다 (이게 핵심)
    let p = sfn.pu as usize;
    let q = sfn.pv as usize;

    let r = sfn.ku.knots.len() - 1;
    let s = sfn.kv.knots.len() - 1;

    // U는 아무거나(변경 안될 예정)
    sfn.ku.knots = vec![0.0; r + 1];
    for i in 0..=p { sfn.ku.knots[i] = 0.0; }
    for i in (r - p)..=r { sfn.ku.knots[i] = 1.0; }

    // V: "q에 맞춘" clamped knot vector를 만든다.
    // 끝 multiplicity = q+1 이 반드시 만족해야 C 코드 전제가 성립.
    // 길이는 (s+1) 이어야 함.
    let mut vk = vec![0.0; s + 1];

    // start clamp: c0
    for j in 0..=q { vk[j] = 2.0; }
    // end clamp: c1
    for j in (s - q)..=s { vk[j] = 4.0; }

    // 내부 knot가 들어갈 자리가 있으면 하나 넣어보자.
    // internal index 구간: (q+1 ..= s-q-1)
    if q + 1 <= s.saturating_sub(q + 1) {
        let jmid = q + 1; // 가장 첫 내부 knot 위치
        if jmid <= s - q - 1 {
            vk[jmid] = 3.0; // 내부 knot
        }
    }

    sfn.kv.knots = vk;

    let u_before = sfn.ku.knots.clone();
    let v_before = sfn.kv.knots.clone();

    // V만 스케일
    let rect = Box2D {
        x: Interval::new(0.0, 1.0),
        y: Interval::new(10.0, 30.0),
    };

    sfn.scale_knots_to_box(rect, KnotScaleDir::V);

    // U unchanged
    assert_eq!(sfn.ku.knots, u_before);

    // V ends should be exactly [10..30] clamped with q+1 multiplicity
    for j in 0..=q {
        assert!((sfn.kv.knots[j] - 10.0).abs() <= 1e-12, "start clamp j={j}");
    }
    for j in (s - q)..=s {
        assert!(
            (sfn.kv.knots[j] - 30.0).abs() <= 1e-12,
            "end clamp j={j}, got {}",
            sfn.kv.knots[j]
        );
    }

    // 내부 knot는 affine 변환 확인 (가능한 경우만)
    let old0 = v_before[0];
    let old1 = v_before[s];
    let c = 10.0;
    let d = 30.0;

    if q + 1 <= s.saturating_sub(q + 1) {
        for j in (q + 1)..=(s - q - 1) {
            let denom = old1 - old0;
            if denom.abs() > 1e-30 {
                let fac = (d - c) / denom;
                let exp = fac * (v_before[j] - old0) + c;
                assert!(
                    (sfn.kv.knots[j] - exp).abs() <= 1e-12,
                    "internal knot mismatch j={j}: got {}, exp {}",
                    sfn.kv.knots[j],
                    exp
                );
            }
        }
    }
}
```
```rust
#[test]
fn scale_knots_to_box_scales_uv_both() {
    let (n, m, p, q) = (3usize, 1usize, 2usize, 1usize);
    let r = n + p + 1; // 3+2+1=6 => len 7
    let s = m + q + 1; // 1+1+1=3 => len 4

    let mut sfn = SFun::new_empty();
    ensure_sfun_shape(&mut sfn, n as Index, m as Index,
        p as Degree, q as Degree, r as Index, s as Index);

    sfn.ku.knots = vec![0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0];
    sfn.kv.knots = vec![5.0, 5.0, 6.0, 6.0];

    let rect = Box2D {
        x: Interval::new(-2.0, 2.0),
        y: Interval::new(100.0, 200.0),
    };

    sfn.scale_knots_to_box(rect, KnotScaleDir::UV);

    // U endpoints
    for i in 0..=p {
        assert!(approx(sfn.ku.knots[i], -2.0, 1e-12));
    }
    for i in (r - p)..=r {
        assert!(approx(sfn.ku.knots[i], 2.0, 1e-12));
    }

    // V endpoints (q=1)
    for j in 0..=q {
        assert!(approx(sfn.kv.knots[j], 100.0, 1e-12));
    }
    for j in (s - q)..=s {
        assert!(approx(sfn.kv.knots[j], 200.0, 1e-12));
    }
}
```
```rust
#[test]
fn scale_knots_to_box_noop_when_ranges_already_match() {
    let (n, m, p, q) = (2usize, 2usize, 2usize, 2usize);
    let r = n + p + 1;
    let s = m + q + 1;

    let mut sfn = SFun::new_empty();
    ensure_sfun_shape(&mut sfn, n as Index, m as Index,
        p as Degree, q as Degree, r as Index, s as Index);

    sfn.ku.knots = vec![10.0, 10.0, 10.0, 12.0, 20.0, 20.0, 20.0];
    sfn.kv.knots = vec![1.0, 1.0, 1.0, 2.0, 3.0, 3.0, 3.0];

    let u_before = sfn.ku.knots.clone();
    let v_before = sfn.kv.knots.clone();

    // 이미 U[0]=10, U[r]=20 / V[0]=1, V[s]=3 이므로 noop 기대
    let rect = Box2D {
        x: Interval::new(10.0, 20.0),
        y: Interval::new(1.0, 3.0),
    };

    sfn.scale_knots_to_box(rect, KnotScaleDir::UV);

    assert_eq!(sfn.ku.knots, u_before);
    assert_eq!(sfn.kv.knots, v_before);
}
```
# extract_coordinate_functions / scale_knots_to_box
## 🌋 1 결합이 **필수** 인 이유
- 이 둘은 단순히 편의 기능이 아니라, NURBS 기반 해석(analysis) 파이프라인의  
    근본 구조를 만든다.
### 🔸 extract_coordinate_functions = Geometry → Function Space 변환
- NURBS surface는 원래 이렇게 생겼다:
```math
S(u,v)=\frac{\sum _{i,j}N_i^p(u)M_j^q(v)w_{ij}P_{ij}}{\sum _{i,j}N_i^p(u)M_j^q(v)w_{ij}}
```
- 이건 기하학적 객체다.
- 하지만 Newton solver는 기하학을 직접 다룰 수 없다.
- 그래서 U_SURCOR는 surface를 이렇게 분해한다:
- wx(u,v) = Xw(u,v)
- wy(u,v) = Yw(u,v)
- wz(u,v) = Zw(u,v)
- w(u,v) = w(u,v)
- 즉, surface를 4개의 스칼라 함수로 바꾼다.
- 이 순간 surface는:
    - CAD 형상 ❌
    - 수학적 함수 집합(analysis object) ✅
- 이게 Newton solver가 다룰 수 있는 형태다.

### 🔸 scale_knots_to_box = Function Space의 좌표계를 정규화
- extract_coordinate_functions로 얻은 함수들은 여전히 이런 domain을 가질 수 있다:
    - u ∈ [12.3, 98.7]
    - v ∈ [-0.002, 1045]
- 이 domain은 Newton solver에게 재앙이다.
- 그래서 scale_knots_to_box는 domain을 강제로:
```math
u,v\in [0,1]
```
- 로 바꾼다.
- 중요한 점:
    - control value는 그대로
    - geometry는 그대로
    - basis function의 정의역만 affine 변환
- 즉, 형상은 그대로 두고 parameter 좌표계만 바꾼다.

### 🌋 2. Newton solver가 왜 이걸 반드시 요구하는가
- Newton은 다음을 푼다:
```math
F(x)=0
```
- surface-surface intersection이면:
```math
F(u,v,s,t)=S_1(u,v)-S_2(s,t)
```
- Newton이 안정적으로 동작하려면 다음 조건이 필요하다.

### 🔸 조건 1: 정의역이 작고 정규화돼 있어야 한다
- Newton step:
```math
x_{k+1}=x_k-J^{-1}F(x_k)
```
- 여기서 step 크기, damping, tolerance는 정규화된 domain을 가정한다.
- 만약 domain이:
    - [12.3, 98.7]
    - [-0.002, 1045]
- 이면:
    - step 크기 해석 불가
    - damping이 의미 없음
    - tolerance가 domain 크기와 mismatch
    - Jacobian이 scale mismatch로 ill-conditioned
- 즉, Newton이 폭주하거나 수렴 실패한다.

### 🔸 조건 2: 함수 스케일이 균등해야 한다
- Jacobian의 각 항은 다음과 같다:
```math
\frac{\partial S}{\partial u},\quad \frac{\partial S}{\partial v}
```
- 만약 u-range가 1000배 크면:
    - ∂S/∂u 값이 1000배 작아짐
    - Jacobian의 column scale이 불균형
    - condition number 폭증
    - Newton step이 엉뚱한 방향으로 튐
- scale_knots_to_box는 이 문제를 완전히 제거한다.

### 🔸 조건 3: 도함수 계산이 단순해야 한다
- NURBS 도함수는:
```math
\frac{\partial S}{\partial u}=\frac{\sum N_i'(u)M_j(v)w_{ij}P_{ij}}{W}-S(u,v)\frac{W_u}{W}
```
- 여기서 basis derivative는 knot spacing에 민감하다.
- knot spacing이:
    - 0.0001
    - 1000
- 이런 식으로 섞여 있으면:
    - basis derivative가 극단적으로 커지거나 작아짐
    - Newton이 완전히 불안정해짐
- scale_knots_to_box는 knot spacing을 균등한 단위 공간으로 매핑한다.

### 🌋 3. extract_coordinate_functions + scale_knots_to_box 
- Newton이 살 수 있는 세계
- 이 둘을 결합하면:
- extract_coordinate_functions
    - surface를 함수(wx,wy,wz,w)로 변환
- scale_knots_to_box
    - domain을 [0,1]×[0,1]로 정규화
- Newton solver
    - 안정적으로 해를 찾음
- 이 구조는 Parasolid, ACIS, OpenCascade 등  
    모든 상용 커널이 사용하는 방식이다.

### 🌋 4. 실제 surface-surface intersection에서 어떻게 쓰이는가
- Newton이 풀고 싶은 식:
```math
\begin{aligned}F_1&=x_1(u,v)-x_2(s,t)\\ F_2&=y_1(u,v)-y_2(s,t)\\ F_3&=z_1(u,v)-z_2(s,t)\end{aligned}
```
- 여기서:
    - x1 = wx1 / w1
    - y1 = wy1 / w1
    - z1 = wz1 / w1
- 이 모든 함수는 extract_coordinate_functions 로 얻는다.
- 그리고 모든 함수는 scale_knots_to_box로 domain이 [0,1]이 된다.
- 그 후 Newton은:
    - 안정적인 Jacobian
    - 균등한 parameter scale
    - predictable step size
- 를 기반으로 빠르게 수렴한다.

### 🌋 5. 커널 설계 관점에서의 핵심
- 이 구조는 단순히 **좋은 방법** 이 아니라 커널 설계의 정석이다.
- 왜냐하면:
    - Geometry는 절대 건드리지 않는다
    - Function space에서만 해석을 수행한다
    - Newton은 function space에서만 의미가 있다
    - domain 정규화는 필수 안정화 과정이다

## 🌋 6. 한 줄 요약
- extract_coordinate_functions는 surface를 해석 가능한 함수로 바꾸고,  
    scale_knots_to_box는 Newton이 안정적으로 작동할 수 있는 domain을 만든다.
- 이 둘 없이는 Newton solver는 구조적으로 불안정하다.

--

## 🔥 1) “정규화된 u,v”를 찾았을 때 Geometry가 변하느냐?
- 절대 변하지 않는다.
- 왜냐하면:
- extract_coordinate_functions는 geometry를 분해할 뿐 geometry를 바꾸지 않는다
- scale_knots_to_box는 SFun의 knot만 바꾼다
- 원본 NURBS surface의 knot는 절대 건드리지 않는다
- 즉, Newton으로 찾은 u,v는: 
    정규화된 함수 공간(Function Space)에서의 u,v 이고,  
    Geometry Space의 u,v와는 다르다
- 하지만 geometry는 그대로이기 때문에 pos(u,v)는 변하지 않는다.

## 🔥 2) “만약 원본 surface의 knot도 바꿨다면?”
- 만약 원본 NURBS surface의 knot vector를 직접 바꿨다면,
    그건 단순한 parameter scaling이 아니라:  
    - surface parameterization 변경
    - basis function shape 변경
    - control point 영향 범위 변경
    - 결국 geometry 자체가 변형됨
- 즉:
    - 원본 surface의 knot를 바꾸면 geometry가 바뀐다.

- 그래서 CAD 커널들은 절대 원본 knot를 건드리지 않는다.

## 🔥 3) 왜 SFun만 scale하고 원본 surface는 scale하지 않는가
- 이게 바로 커널 설계의 핵심.
- Geometry Layer
    - CAD에서 정의된 원본 surface
    - 절대 변하면 안 됨
    - 제조/설계/데이터 교환의 기준
- Function Layer
    - Newton solver, intersection, trimming 등
    - 해석용으로만 쓰는 “복제된 함수 공간”
    - 여기서는 domain을 마음대로 바꿔도 됨
    - geometry는 그대로
- 즉:
- 해석을 위해 domain을 바꾸는 건 Function Layer에서만 한다.
- Geometry Layer는 절대 건드리지 않는다.
- Parasolid, ACIS, OpenCascade 모두 이렇게 한다.

## 🔥 4) 정규화된 u,v → 원본 u,v로 어떻게 돌아가나?
- scale_knots_to_box는 affine 변환이기 때문에 역변환도 affine이다.
- 예를 들어 U 방향:
- 원본 knot range: [u_0,u_r]
- 정규화 range: [0,1]
- 정규화된 u'를 찾았다면:
```math
u=u_0+u'(u_r-u_0)
```
- 즉:
    - Newton solver는 정규화된 u', v'에서 일함
    - 최종 pos 계산은 원본 surface의 u,v로 변환해서 계산
- 그래서 geometry는 절대 변하지 않는다.

## 🔥 5) 왜 이렇게 복잡하게 두 개의 u,v를 쓰는가?
- 이유는 단 하나:
    - Newton solver는 정규화된 domain에서만 안정적으로 동작한다.

- 하지만:
    - CAD geometry는 원본 domain을 유지해야 한다.

- 이게 바로 Geometry–Analysis 분리 구조다.

## 🔥 6) 결론
- ✔ Newton으로 찾은 u,v는 “정규화된 함수 공간의 u,v”
- ✔ Geometry는 절대 변하지 않기 때문에 pos는 변하지 않는다
- ✔ 원본 surface의 knot를 바꾸지 않는 한 geometry는 안전하다
- ✔ 원본 knot를 바꾸면 geometry가 변형되므로 절대 하면 안 된다
- ✔ scale_knots_to_box는 오직 SFun(해석용 복제)에만 적용해야 한다
---

## 1️⃣ 왜 SFun의 u,v와 NurbsSurface의 u,v가 달라지는가
- 이유는 간단:
    - ✔ SFun은 정규화된 domain을 사용
- scale_knots_to_box로 인해
```math
u',v'\in [0,1]
```
- ✔ NurbsSurface는 원본 domain을 유지
- 예를 들어:
- NurbsSurface:
```math
u\in [12.3,98.7],\quad v\in [-0.002,1045]
```
- SFun:
```math
u'\in [0,1],\quad v'\in [0,1]
```
- 즉, 두 domain은 다른 공간.
- 그래서 Newton solver가 찾은 u', v'는  
    그대로 NurbsSurface에 넣으면 엉뚱한 위치가 나온다.

## 2️⃣ 그럼 어떻게 해야 하나? → 반드시 역변환 필요
- scale_knots_to_box는 다음 변환을 적용한다:
```math
u'=\frac{u-u_0}{u_r-u_0}
```
- 여기서:
    - $u_0$ = 원본 knot 첫 값
    - $u_r$ = 원본 knot 마지막 값
- 그러면 역변환은:
```math
u=u_0+u'(u_r-u_0)
```
- v도 동일:
```math
v=v_0+v'(v_s-v_0)
```
- 즉:
    - 🎯 정규화된 u',v' → 원본 u,v로 affine 역변환하면 된다.

## 3️⃣ 왜 geometry는 변하지 않는가
- extract_coordinate_functions 는 geometry를 분해만 함
- scale_knots_to_box는 SFun의 knot만 바꿈
- NurbsSurface의 knot는 절대 건드리지 않음
- control point도 안 바뀜
- geometry는 그대로
- 즉:
    - ✔ SFun은 해석용 복제본
    - ✔ NurbsSurface는 CAD geometry 원본
- Newton solver는 SFun에서 u',v'를 찾고  
    최종 pos는 NurbsSurface에서 u,v로 계산한다.

## 4️⃣ 전체 파이프라인을 그림으로 보면
```
NurbsSurface (원본 geometry)
    |
    |  U_SURCOR
    v
SFun(wx,wy,wz,w)  ← geometry의 함수 표현
    |
    |  scale_knots_to_box
    v
정규화된 SFun (u',v' ∈ [0,1])
    |
    |  Newton solver
    v
정규화된 해 (u',v')
    |
    |  역변환 (affine)
    v
원본 surface의 (u,v)
    |
    |  evaluate
    v
pos(x,y,z)
```

## 5️⃣ 왜 이렇게 복잡하게 두 개의 domain을 쓰는가?
- 이유는 단 하나:
    - Newton solver는 정규화된 domain에서만 안정적으로 동작한다.
- 하지만:
    - CAD geometry는 원본 domain을 유지해야 한다.
- 그래서 domain이 두 개 존재:

| Layer            | Domain                         | Purpose                          |
|------------------|--------------------------------|----------------------------------|
| Geometry Layer   | Original knot domain           | Keep CAD geometry exact          |
|                  | (e.g., [u0, ur] × [v0, vs])    | Never modified                   |
| Function Layer   | Normalized domain              | Stable Newton iteration          |
| (SFun)           | [0,1] × [0,1]                  | Good Jacobian conditioning       |


## 6️⃣ 결론
-  SFun에서 찾은 u',v'는 NurbsSurface의 u,v로 직접 쓸 수 없다.  
    반드시 **affine 역변환** 을 거쳐야 한다.
- 그리고:
- 역변환은 매우 단순한 affine mapping이므로 안전하고 빠르다.

---

## 소스 코드
```rust
/// - wx,wy,wz: 항상 채움
/// - w: surface가 rational일 때만 채움 (비-rational이면 untouched)
///
/// 반환값: surface가 rational이면 true
pub fn extract_coordinate_functions(
    &self,
    wx: &mut SFun,
    wy: &mut SFun,
    wz: &mut SFun,
    mut w: Option<&mut SFun>,
) -> bool {
    // ---- local notation (C: U_surbre + U_surknp) ----
    let (n, m, r, s) = self.indices();      // last indices
    let (p, q) = self.deg();                // degrees
    let rat = self.is_rational();

    // ---- ensure memory (C: U_sfnchk + U_sfnfuv) ----
    ensure_sfun_shape(wx, n, m, p, q, r, s);
    ensure_sfun_shape(wy, n, m, p, q, r, s);
    ensure_sfun_shape(wz, n, m, p, q, r, s);

    if rat {
        if let Some(ref mut ww) = w {
            ensure_sfun_shape(ww, n, m, p, q, r, s);
        } else {
            // rational인데 w 저장용 버퍼가 안 들어오면,
            // C에서는 "w가 필요하면 호출자가 준비" 개념이므로 여기선 그냥 무시.
            // (원하면 Result로 바꿔서 에러 처리 가능)
        }
    }

    // ---- fill control values (C: A_extcpc) ----
    // fx[i][j] = Pw[i][j].x, fy = .y, fz = .z, fw = .w
    // 여기서 Pw는 homogeneous (Xw,Yw,Zw,w)
    let nu = (n + 1) as usize;
    let nv = (m + 1) as usize;

    for i in 0..nu {
        for j in 0..nv {
            let cp = self.ctrl_at(i, j);

            wx.set(i, j, cp.x);
            wy.set(i, j, cp.y);
            wz.set(i, j, cp.z);

            if rat {
                if let Some(ref mut ww) = w {
                    ww.set(i, j, cp.w);
                }
            }
        }
    }

    // ---- copy knots (C: UX/UY/UZ/(UW) and VX/VY/VZ/(VW)) ----
    // SFun의 knot vector는 KnotVector 내부 knots를 그대로 갱신
    // ensure_sfun_shape()가 길이를 맞춰놨기 때문에 인덱스 대입 OK.
    for i in 0..=(r as usize) {
        let ui = self.ku.knots[i];
        wx.ku.knots[i] = ui;
        wy.ku.knots[i] = ui;
        wz.ku.knots[i] = ui;
        if rat {
            if let Some(ref mut ww) = w {
                ww.ku.knots[i] = ui;
            }
        }
    }

    for j in 0..=(s as usize) {
        let vj = self.kv.knots[j];
        wx.kv.knots[j] = vj;
        wy.kv.knots[j] = vj;
        wz.kv.knots[j] = vj;
        if rat {
            if let Some(ref mut ww) = w {
                ww.kv.knots[j] = vj;
            }
        }
    }
    rat
}
```
```rust
/// 편의 함수: w까지 반드시 받고 싶을 때 (rational 아니면 w는 clear해둘 수도 있음)
pub fn extract_coordinate_functions_with_w(
    &self,
    wx: &mut SFun,
    wy: &mut SFun,
    wz: &mut SFun,
    w: &mut SFun,
) -> bool {
    let rat = self.extract_coordinate_functions(wx, wy, wz, Some(w));
    if !rat {
        // 비-rational이면 C처럼 "w를 반환 안 한다"가 원칙이지만
        // Rust에서는 호출자가 실수로 쓰는 걸 막으려면 clear가 안전.
        w.clear();
    }
    rat
}
```
```rust
#[inline]
pub fn on_ensure_sfun_shape(
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
```
```rust
/// Scale surface-function knot vectors to a given rectangle.
/// The operation is IN-PLACE (original knots are destroyed).
pub fn scale_knots_to_box(&mut self, rect: Box2D, dir: KnotScaleDir) {
    let p = self.pu as usize;
    let q = self.pv as usize;

    let r = self.ku.knots.len() - 1;
    let s = self.kv.knots.len() - 1;

    // ---- U direction ----
    if matches!(dir, KnotScaleDir::U | KnotScaleDir::UV) {
        let a = rect.x.t0;
        let b = rect.x.t1;

        let u0 = self.ku.knots[0];
        let ur = self.ku.knots[r];

        if a != u0 || b != ur {
            let denom = ur - u0;
            debug_assert!(denom != 0.0, "degenerate U knot range");

            let fac = (b - a) / denom;

            // start clamped
            for i in 0..=p {
                self.ku.knots[i] = a;
            }

            // internal knots
            for i in (p + 1)..=(r - p - 1) {
                self.ku.knots[i] = fac * (self.ku.knots[i] - u0) + a;
            }

            // end clamped
            for i in (r - p)..=r {
                self.ku.knots[i] = b;
            }
        }
    }

    // ---- V direction ----
    if matches!(dir, KnotScaleDir::V | KnotScaleDir::UV) {
        let c = rect.y.t0;
        let d = rect.y.t1;

        let v0 = self.kv.knots[0];
        let vs = self.kv.knots[s];

        if c != v0 || d != vs {
            let denom = vs - v0;
            debug_assert!(denom != 0.0, "degenerate V knot range");

            let fac = (d - c) / denom;

            // start clamped
            for j in 0..=q {
                self.kv.knots[j] = c;
            }

            // internal knots
            for j in (q + 1)..=(s - q - 1) {
                self.kv.knots[j] = fac * (self.kv.knots[j] - v0) + c;
            }

            // end clamped
            for j in (s - q)..=s {
                self.kv.knots[j] = d;
            }
        }
    }
}
```
---
