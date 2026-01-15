# on_reparam_curve_knots_to_unit_span
- Reparameterize a clamped NURBS curve so its knot domain becomes [0,1]
- 이 함수는 클램프(clamped) NURBS 곡선의 knot vector를 [0,1] 구간으로  
    선형 변환(reparameterization) 하는 기능을 수행한다.
- 곡선의 형상(geometry)은 완전히 유지되며, 파라미터 구간만 변경된다.

## ✨ 목적
- NURBS 곡선은 일반적으로 다음과 같은 임의의 파라미터 구간을 가진다:
```math
u\in [u_{\min },u_{\max }]
```
- 하지만 많은 알고리즘(샘플링, subdivision, offset, fitting 등)은  
    정규화된 파라미터 구간 [0,1] 을 요구한다.
- 이 함수는 다음 변환을 수행한다:
```math
t=\frac{u-u_{\min }}{u_{\max }-u_{\min }}
```
- 이를 통해:
    - 곡선의 형상은 그대로 유지
    - knot vector는 [0,1] 범위로 변환
    - domain도 [0,1]로 갱신

## 📐 수식 (Mathematical Formulation)
### 1. 원래 knot vector
- 클램프된 NURBS 곡선의 knot vector는 다음과 같다:
```math
U=\{ u_0,u_1,\dots ,u_m\}
``` 
- 여기서:
    - 앞쪽 p+1개는 $u_{\min }$
    - 뒤쪽 p+1개는 $u_{\max }$
- 즉:
```math
u_0=u_1=\dots =u_p=u_{\min }
```
```math
u_{m-p}=\dots =u_m=u_{\max }
```

### 2. 선형 변환 (Affine Mapping)
- 내부 knot에 대해 다음 변환을 적용한다:
```math
u'_i=\frac{u_i-u_{\min }}{u_{\max }-u_{\min }}
```
- 경계 knot는 다음과 같이 고정한다:
```math
u'_0=\dots =u'_p=0
```
```math
u'_{m-p}=\dots =u'_m=1
```
### 3. 곡선 형상 보존 (Shape Preservation)
- NURBS 곡선은 다음과 같이 정의된다:
```math
C(u)=\frac{\sum _iN_{i,p}(u)P_i}{\sum _iN_{i,p}(u)}
```
- 파라미터를 t로 치환하면:
```math
u=u_{\min }+t(u_{\max }-u_{\min })
```
- 즉:
```math
C(u)=C(u(t))=C'(t)
```
- 따라서 형상은 완전히 동일하다.

## 🧠 알고리즘 설명 (Step-by-step)
- 차수 p, 제어점 개수 n 확인
- 유효한 NURBS인지 검사
- knot vector 길이 검사
- 클램프된 구간의 domain 추출
```rust
let umin = knots[p];
let umax = knots[n + 1];
```
- 선형 변환 계수 계산
```rust
let den = umax - umin;
```
- 앞쪽 p+1개의 knot = 0
- 뒤쪽 p+1개의 knot = 1
- 중간 knot만 선형 변환
```rust
knots[i] = (knots[i] - umin) / den;
- curve.domain = [0,1] 로 갱신
```

---

## 소스 코드
```rust
pub fn on_reparam_curve_knots_to_unit_span(cur: &mut NurbsCurve) {
    let p = cur.degree as usize;
    let n = cur.ctrl.len().saturating_sub(1);
    if n < p { return; }

    let knots = &mut cur.kv.knots;
    let m = knots.len().saturating_sub(1);
    if m < n + p + 1 { return; }

    // domain endpoints (clamped 기준)
    let umin = knots[p];
    let umax = knots[n + 1];
    let den = umax - umin;
    if den.abs() < 1e-300 { return; }

    // C처럼: 앞 p+1은 0, 뒤 p+1은 1, 가운데만 선형변환
    for i in 0..=p { knots[i] = 0.0; }
    for i in (p + 1)..=(m - p - 1) {
        knots[i] = (knots[i] - umin) / den;
    }
    for i in (m - p)..=m { knots[i] = 1.0; }
    cur.domain = Interval { t0: 0.0, t1: 1.0 };
}
```

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::prelude::*;
    use nurbslib::core::nurbs_curve::on_reparam_curve_knots_to_unit_span;
```
```rust
    fn assert_near(a: f64, b: f64, eps: f64, msg: &str) {
        let d = (a - b).abs();
        assert!(d <= eps, "{}: |{}-{}|={}", msg, a, b, d);
    }

    fn assert_pt3_near(a: Point3D, b: Point3D, eps: f64) {
        assert_near(a.x, b.x, eps, "x");
        assert_near(a.y, b.y, eps, "y");
        assert_near(a.z, b.z, eps, "z");
    }

    fn is_nondecreasing(v: &[Real]) -> bool {
        v.windows(2).all(|w| w[0] <= w[1])
    }
```
```rust
    #[test]
    fn reparam_knots_clamp_and_range_ok() {
        let p: usize = 3;
        let ctrl = vec![
            Point4D{x:0.0,y:0.0,z:0.0,w:1.0},
            Point4D{x:1.0,y:2.0,z:0.0,w:1.0},
            Point4D{x:2.0,y:0.0,z:0.0,w:1.0},
            Point4D{x:3.0,y:2.0,z:0.0,w:1.0},
            Point4D{x:4.0,y:0.0,z:0.0,w:1.0},
            Point4D{x:5.0,y:1.0,z:0.0,w:1.0},
        ];
        let mut cur = NurbsCurve::from_ctrl_clamped_uniform(p as _, ctrl);

        on_reparam_curve_knots_to_unit_span(&mut cur);

        let k = &cur.kv.knots;
        assert!(is_nondecreasing(k), "knots must be nondecreasing");
        assert_near(cur.domain.t0, 0.0, 0.0, "domain.t0");
        assert_near(cur.domain.t1, 1.0, 0.0, "domain.t1");

        // clamp check (끝이 정확히 0/1인지)
        let n = cur.ctrl.len() - 1;
        let m = n + p + 1;
        for i in 0..=p { assert_near(k[i], 0.0, 0.0, "front clamp"); }
        for i in (m - p)..=m { assert_near(k[i], 1.0, 0.0, "back clamp"); }

        // interior range
        for i in (p+1)..=(m-p-1) {
            assert!(k[i] >= 0.0 - 1e-14 && k[i] <= 1.0 + 1e-14, "interior out of [0,1]");
        }
    }
```
```rust
    #[test]
    fn reparam_preserves_shape_with_param_mapping() {
        let p: usize = 3;
        let ctrl = vec![
            Point4D{x:0.0,y:0.0,z:0.0,w:1.0},
            Point4D{x:1.0,y:2.0,z:0.5,w:1.0},
            Point4D{x:2.0,y:0.2,z:1.0,w:1.0},
            Point4D{x:3.0,y:2.2,z:-0.5,w:1.0},
            Point4D{x:4.0,y:0.0,z:0.0,w:1.0},
            Point4D{x:5.0,y:1.0,z:0.7,w:1.0},
            Point4D{x:6.0,y:0.5,z:1.1,w:1.0},
        ];
        let mut cur0 = NurbsCurve::from_ctrl_clamped_uniform(p as _, ctrl);
        let mut cur1 = cur0.clone();

        // original domain in knot sense
        let n = cur0.ctrl.len() - 1;
        let umin = cur0.kv.knots[p];
        let umax = cur0.kv.knots[n + 1];
        let den = umax - umin;

        on_reparam_curve_knots_to_unit_span(&mut cur1);

        // compare: cur0(u) == cur1(t) where t=(u-umin)/den
        let eps = 1e-9;
        for j in 0..=50 {
            let t = j as Real / 50.0;
            let u = umin + t * den;

            let p0 = cur0.point_at(u);      // old param
            let p1 = cur1.point_at(t);      // new param in [0,1]
            assert_pt3_near(p0, p1, eps);
        }
    }
}
```

## 🧪 테스트 설명
- ✔ Test 1: reparam_knots_clamp_and_range_ok
- 검증 내용:
    - knot vector가 여전히 비감소(non-decreasing)
    - domain이 정확히 [0,1]
    - 앞쪽 p+1 knot = 0
    - 뒤쪽 p+1 knot = 1
    - 내부 knot가 모두 [0,1] 범위 안에 있음
- 즉, 정상적인 reparameterization 여부를 확인한다.

- ✔ Test 2: reparam_preserves_shape_with_param_mapping
- 검증 내용:
    - 원래 곡선 cur0
    - reparam 후 곡선 cur1
    - 두 곡선의 형상이 동일한지 비교
- 비교 방식:
```math
u=u_{\min }+t(u_{\max }-u_{\min })
```
- 즉:
```math
C_0(u)\approx C_1(t)
```
- 테스트는 51개 샘플에서 다음을 확인한다:
```rust
assert_pt3_near(cur0.point_at(u), cur1.point_at(t), eps);
```

- 이 테스트는 형상 보존(shape preservation) 을 보장한다.


## 📦 요약

| 항목 | 설명 |
|------|------|
| 목적 | NURBS 곡선의 파라미터 구간을 정규화하여 `[0,1]`로 변환 |
| 형상 보존 | 선형 파라미터 매핑으로 인해 곡선의 기하학적 형상은 완전히 동일 |
| 변경되는 것 | Knot vector, domain (→ `[0,1]`) |
| 변경되지 않는 것 | 제어점, 차수, 곡선 형상 |
| 적용 대상 | Clamped NURBS curve |
| 내부 knot 처리 | `(u - u_min) / (u_max - u_min)` 선형 변환 |
| 경계 knot 처리 | 앞쪽 `p+1`개 = 0, 뒤쪽 `p+1`개 = 1 |


---
