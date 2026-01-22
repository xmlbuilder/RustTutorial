# 방향 프레임 
- **곡선 위에서 B-벡터(일종의 방향 프레임)를 따라가는 알고리즘**.

## 1. 이 함수가 하는 일 한 줄 요약
- on_comp_curve_b_vector는:
    - 주어진 NURBS 곡선 cur 위에서
    - 파라미터 샘플 t[0..k]에 대해
    - 시작 방향 벡터 B₀를 기준으로
    - 각 점에서의 **B-벡터(곡선에 수직인 방향 벡터)**들을 연속적으로 만들어주는 함수.
- 즉, 곡선 위를 따라가는 **부드러운 방향 필드** 를 만드는 함수라고 보면 된다.

## 2. 입력과 출력 의미
- cur: &NurbsCurve
- 곡선 C(u)
- b0: Vector3D
    - 시작점 t[0]에서의 초기 B 벡터 후보 $B_0$
- t: &[Real]
    - 파라미터 샘플 배열 $\{ t_0,t_1,\dots ,t_k\}$ , 길이 k+1
- b_out: &mut [Vector3D]
    - 결과 
    - B 벡터 배열 $\{ B_0,B_1,\dots ,B_k\}$, 길이 k+1
- 제약:
    - t.len() == b_out.len()
    - t는 비어 있으면 안 됨
    - B₀ 및 중간 B-벡터는 정규화 가능해야 함 (길이 0 근처면 에러)
    - 탄젠트도 정규화 가능해야 함

## 3. 수식과 단계별 동작
### 3-1. 초기화
```rust
let k = t.len() - 1;
b_out[0] = on_normalize_or_err(b0, "B0")?;
```

- $B_0=\frac{b_0}{\| b_0\| }$
- 시작 B-벡터를 단위 벡터로 만든다.

### 3-2. 각 샘플에서의 단위 탄젠트 계산
```rust
let mut tan = vec![Vector3D::zero(); k + 1];

for i in 1..=k {
    let ti = t[i];
    let t_i = on_eval_unit_tangent(cur, ti).expect("Invalid unit tangent");
    tan[i] = t_i;
    ...
}
```

- 곡선 C(u)의 도함수 C'(u)를 이용해  
    각 t_i에서의 단위 탄젠트 T_i를 구한다:
```math
T_i=\frac{C'(t_i)}{\| C'(t_i)\| }
```

### 3-3. forward pass: B-벡터의 재귀적 구성
- 핵심 수식:
```rust
let dot = b_out[i - 1].dot(&tan[i]);
let bi = b_out[i - 1] - tan[i] * dot;
b_out[i] = on_normalize_or_err(bi, "B[i]")?;
```


- 수식으로 쓰면:
    - 이전 B-벡터: $B_{i-1}$
    - 현재 탄젠트: $T_i$
    - 내적: $d_i=B_{i-1}\cdot T_i$
```math
\tilde {B}_i=B_{i-1}-d_iT_i
```
```math
B_i=\frac{\tilde {B}_i}{\| \tilde {B}_i\| }
```
- 이게 의미하는 건:
    - $B_{i-1}$ 를 현재 탄젠트 $T_i$ 에 대해 직교 성분만 남기고 투영하는 것
    - 즉, $B_i$ 는 항상 $T_i$ 에 수직한 단위 벡터가 된다.
- 이 과정을 따라가면:
    - $B_0$ 에서 시작해서
    - 곡선을 따라가며
        - 각 점에서 **탄젠트에 수직인 방향** 을 부드럽게 이어가는 B-벡터 필드를 만든다.

## 4. 폐곡선(closed curve) 보정 단계
```rust
if on_is_curve_closed(cur)? {
    let dis = (b_out[0] - b_out[k]).length();
    if dis > ON_ZERO_TOL {
        b_out[k] = b_out[0];

        for i in (2..=k).rev() {
            let dot = b_out[i].dot(&tan[i - 1]);
            let a = on_normalize_or_err(b_out[i] - tan[i - 1] * dot, "A(backward)")?;

            let mid = b_out[i - 1] * 0.5 + a * 0.5;
            b_out[i - 1] = on_normalize_or_err(mid, "B[i-1](backward)")?;
        }
    }
}
```

### 4-1. 문제 인식
- 폐곡선에서:
    - $t_0$ 와 $t_k$ 는 같은 점이지만
    - forward pass만 하면 $B_0$ 와 $B_k$ 가 조금씩 달라질 수 있다.
- 그래서:
```math
\| B_0-B_k\| >\varepsilon
```
- 이면 “연속성이 깨진” 상태다.

### 4-2. 첫 보정: 끝점 맞추기
```rust
b_out[k] = b_out[0];
```

- 일단 B_k=B_0로 강제로 맞춘다.

## 4-3. backward smoothing pass
- 역방향으로 부드럽게 보정하는 단계:
    - $i=k,k-1,\dots ,2$ 에 대해 반복
- 각 단계에서:
- 현재 B_i를 이전 탄젠트 $T_{i-1}$ 에 대해 직교화:
```math
\tilde {A}_i=B_i-(B_i\cdot T_{i-1})T_{i-1}
```
```math
A_i=\frac{\tilde {A}_i}{\| \tilde {A}_i\| }
```
- 이전 B-벡터 $B_{i-1}$ 와 $A_i$ 를 평균 내고 정규화:
```math
\tilde {B}_{i-1}=\frac{1}{2}B_{i-1}+\frac{1}{2}A_i
```
```math
B_{i-1}=\frac{\tilde {B}_{i-1}}{\| \tilde {B}_{i-1}\| }
```
- 이 과정을 통해:
    - $B_k=B_0$ 를 유지하면서
    - $B_{k-1},B_{k-2},\dots ,B_1$ 까지
    - 연속적으로 부드럽게 보정된다.
- 결과적으로:
    - 폐곡선 위에서
    - B_0와 B_k가 일치하고
    - 전체 B-벡터 필드가 자연스럽게 이어지는 형태가 된다.

## 5. 실패 조건 정리
- 이 함수는 다음 상황에서 실패(Err)를 반환할 수 있다:
    - t가 비어 있음
    - b_out.len() != t.len()
    - B₀의 길이가 0에 가까워 정규화 불가
    - forward pass 중 어떤 B_i가 0에 가까워 정규화 불가
    - backward pass 중 보정된 벡터가 0에 가까워 정규화 불가
    - 탄젠트 T_i가 0에 가까워 단위 벡터를 만들 수 없는 경우
- 즉, 곡선이 퇴화되어 있거나, B₀가 이상하거나, 특정 구간에서  
    곡선이 거의 점처럼 뭉개진 경우에 실패할 수 있다.

## 6. 이 함수의 쓰임새
- 이 함수는 보통 이런 데 쓰인다:
    - 곡선 위를 따라가는 프레임(field of frames) 구성
    - 스윕(sweep) 서피스 생성 시, 단면 방향 벡터 생성
    - 애니메이션에서 카메라/오브젝트의 “업 벡터”를 곡선을 따라 부드럽게 이동시키는 용도
    - 곡선 기반 리본/튜브/레일 생성 시 방향 안정화
- 즉, 단순한 탄젠트만으로는 부족하고 곡선을 따라가는 **안정된 방향** 이  
    필요할 때 쓰는 함수.

---

## 소스 코드
```rust
/// - cur: trajectory curve
/// - b0: start B vector at t[0]
/// - t: parameter samples, length = k+1
/// - b_out: output B vectors, length = k+1
///
/// 실패 조건:
/// - B0 또는 중간 B가 정규화 불가(길이 0 근접)
/// - tangent 정규화 불가(길이 0 근접)
pub fn on_comp_curve_b_vector(cur: &NurbsCurve, b0: Vector3D, t: &[Real], b_out: &mut [Vector3D]) -> Result<()> {
    if t.is_empty() {
        return Err(NurbsError::InvalidInput { msg: "t is empty".into() });
    }
    if b_out.len() != t.len() {
        return Err(NurbsError::InvalidInput { msg: "b_out.len() must equal t.len()".into() });
    }
    let k = t.len() - 1;

    // ---- init: B[0] = normalize(B0)
    b_out[0] = on_normalize_or_err(b0, "B0")?;

    // ---- tangents T[i] (unit) for i=1..k
    // C 코드에선 T 배열을 k size로 만들고 T[i]만 쓰지만,
    // 여기서는 편하게 (k+1)로 둠. T[0]은 안 씀.
    let mut tan = vec![Vector3D::zero(); k + 1];

    // ---- forward pass: build B[i]
    for i in 1..=k {
        let ti = t[i];

        // "unit tangent"만 필요
        let t_i = on_eval_unit_tangent(cur, ti).expect("Invalid unit tangent");
        tan[i] = t_i;

        // dot = B[i-1]·T[i]
        let dot = b_out[i - 1].dot(&tan[i]);

        // B[i] = normalize( B[i-1] - dot * T[i] )
        let bi = b_out[i - 1] - tan[i] * dot;
        b_out[i] = on_normalize_or_err(bi, "B[i]")?;
    }

    // ---- closed curve correction (C: if U_iscurc(cur))
    if on_is_curve_closed(cur)? {
        let dis = (b_out[0] - b_out[k]).length();
        if dis > ON_ZERO_TOL {
            // set B[k] = B[0]
            b_out[k] = b_out[0];

            // backward smoothing pass
            // for i=k; i>=2; i--
            for i in (2..=k).rev() {
                // A = normalize( B[i] - dot(B[i], T[i-1]) * T[i-1] )
                let dot = b_out[i].dot(&tan[i - 1]);
                let a = on_normalize_or_err(b_out[i] - tan[i - 1] * dot, "A(backward)")?;

                // B[i-1] = normalize( 0.5*B[i-1] + 0.5*A )
                let mid = b_out[i - 1] * 0.5 + a * 0.5;
                b_out[i - 1] = on_normalize_or_err(mid, "B[i-1](backward)")?;
            }
        }
    }

    Ok(())
}
```
---
### 테스트 코드
```rust
use nurbslib::core::geom::{Point3D, Vector3D};
use nurbslib::core::nurbs_curve::NurbsCurve;
use nurbslib::core::circle::Circle;

use nurbslib::core::nurbs_curve::on_comp_curve_b_vector;
use nurbslib::core::plane::Plane;

fn assert_unit(v: &Vector3D, eps: f64) {
    let m = v.length();
    assert!((m - 1.0).abs() <= eps, "not unit: |v|={m}");
}

fn assert_orthogonal(a: &Vector3D, b: &Vector3D, eps: f64) {
    let d = a.dot(b);
    assert!(d.abs() <= eps, "not orthogonal: dot={d}");
}
```
```rust
#[test]
fn b_vector_line_keeps_b_constant_and_perp_to_tangent() {
    // 직선: (0,0,0) -> (10,0,0)
    let p0 = Point3D::new(0.0, 0.0, 0.0);
    let p1 = Point3D::new(10.0, 0.0, 0.0);
    let cur = NurbsCurve::from_line(p0, p1);

    // 파라미터 샘플 (domain이 [0,1] 이라고 가정하지 말고 curve domain을 쓰는게 안전)
    let a = cur.domain.t0;
    let b = cur.domain.t1;

    let k: usize = 10;
    let mut t = Vec::with_capacity(k + 1);
    for i in 0..=k {
        let s = i as f64 / (k as f64);
        t.push(a + (b - a) * s);
    }

    // B0: 접선(+x)과 직교인 방향(예: +y)
    let b0 = Vector3D::new(0.0, 1.0, 0.0);

    let mut bvecs = vec![Vector3D::zero(); k + 1];
    on_comp_curve_b_vector(&cur, b0, &t, &mut bvecs).expect("on_comp_curve_b_vector failed");

    // 기대:
    // - 직선이므로 접선은 항상 +x
    // - B는 항상 (0,1,0) 근처
    let mut tan = Vector3D::zero();
    for i in 0..=k {
        // tangent
        // 너의 API에 맞게 eval_tangent / tangent_at 등으로 교체
        tan = cur.eval_tangent(t[i]).expect("Invalid tangent");

        assert_unit(&bvecs[i], 1e-12);
        assert_orthogonal(&bvecs[i], &tan, 1e-10);

        // B가 거의 일정
        let diff = (bvecs[i] - bvecs[0]).length();
        assert!(diff <= 1e-10, "B drift too large at i={i}: diff={diff}");
    }
}
```
```rust
#[test]
fn b_vector_circle_closed_curve_end_matches_start_after_correction() {
    let pln = Plane::from_origin_normal(Point3D::new(0.0, 0.0, 0.0),
        Vector3D::new(0.0, 0.0, 1.0)).expect("Invalid Plane");
    let c = Circle::from_plane_radius(pln, 5.0);
    let cur = NurbsCurve::from_circle(&c);

    assert!(cur.is_closed(), "circle curve should be closed");

    // from_circle의 domain이 [0, 2π] 라는 전제(너 코드에서 그렇게 보였음)
    let a = cur.domain.t0;
    let b = cur.domain.t1;

    // 0..2π 포함 샘플
    let k: usize = 40;
    let mut t = Vec::with_capacity(k + 1);
    for i in 0..=k {
        let s = i as f64 / (k as f64);
        t.push(a + (b - a) * s);
    }

    // 시작점 접선에 직교인 초기 B0를 하나 고른다.
    // 원이 XY 평면이라면:
    // - t=a 에서 접선이 +y 또는 -y 쪽일 수 있으니, B0는 +x 같은 방향이면 보통 직교.
    // 하지만 확실하게 하려면 실제 tangent로부터 직교 벡터를 만들자.
    let t0 = cur.eval_tangent(t[0]).unwrap_or(Vector3D::new(1.0, 0.0, 0.0)).unitize();
    // B0 = normalize( z × T )  (T가 z와 평행이면 다른 축 사용)
    let z = Vector3D::new(0.0, 0.0, 1.0);
    let mut b0 = z.cross(&t0);
    if b0.length() < 1e-14 {
        b0 = Vector3D::new(1.0, 0.0, 0.0).cross(&t0);
    }
    b0 = b0.unitize();

    let mut bvecs = vec![Vector3D::zero(); k + 1];
    on_comp_curve_b_vector(&cur, b0, &t, &mut bvecs).expect("on_comp_curve_b_vector failed");

    // 기본 성질 체크: 항상 unit, tangent와 직교
    for i in 0..=k {
        let ti = cur.eval_tangent(t[i]).expect("Invalid tangent");
        assert_unit(&bvecs[i], 1e-10);
        assert_orthogonal(&bvecs[i], &ti, 1e-8);
    }

    // 핵심: closed correction이 있다면 B[k] ~ B[0] 이어야 한다.
    let end_gap = (bvecs[k] - bvecs[0]).length();
    assert!(
        end_gap <= 1e-6,
        "closed correction failed: |B_end - B_start| = {end_gap}"
    );
}
```
---
