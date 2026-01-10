## 📘 NURBS Curve Weight Shaping
- (mod_shape_weight_prepare, mod_shape_weight_interact) Technical Documentation  
    이 문서는 Piegl & Tiller The NURBS Book 의 N_SHACMW (Modify One Curve Weight)  
    알고리즘을 Rust 기반 NURBS 커널에서 재구현한 함수들의 수학적 의미, 알고리즘 구조,  검증 절차를 설명한다.
- 이 기능은 곡선의 특정 control point weight를 조정하여 곡선을 해당 점 방향으로 당기거나  
    밀어내는 고급 Shape Editing 기능이다.

## 1. 목적
- NURBS 곡선에서 weight $w_k$ 를 조정하면, 해당 control point $P_k$ 방향으로  
    곡선이 당겨지거나(push/pull) 밀려난다.
- 이 알고리즘은 다음을 만족한다:
    - 곡선의 degree, knot, control point 개수는 유지
    - parameterization 유지
    - 연속성(C¹, C²) 유지
    - weight 조정이 수학적으로 안정적
    - interactive UI에서 슬라이더로 weight를 조절할 때 부드럽게 반응

## 2. 수학적 배경
### 2.1 Rational Basis Function
- NURBS 곡선은 다음과 같이 정의된다:
```math
C(u)=\frac{\sum _{i=0}^nN_{i,p}(u)w_iP_i}{\sum _{i=0}^nN_{i,p}(u)w_i}
```
- 여기서 rational basis function은:
```math
R_i(u)=\frac{N_{i,p}(u)w_i}{\sum _{j=0}^nN_{j,p}(u)w_j}
```

### 2.2 Weight Shaping 공식
- Piegl & Tiller의 mod_shape_weight 공식은 다음과 같다:
#### 1) 거리 계산
- control point의 Euclidean 좌표:
```math
P_k=\frac{P_{w,k}}{w_k}
```
- 곡선 점:
```math
C(u)
```
- 거리:
```math
\mathrm{pkp}=\| C(u)-P_k\|
``` 

#### 2) 분모 계산
```math
\mathrm{den}=R_k(u)\cdot (\mathrm{pkp}-d)
```
- 여기서
    - d = 사용자가 원하는 push/pull 거리
    - $R_k(u)$ = rational basis 값

#### 3) 새로운 weight 계산
```math
w_k'=w_k\left( 1+\frac{d}{\mathrm{den}}\right)
``` 

#### 4) weight 범위 제한
```math
W_{\min }\leq w_k'\leq W_{\max }
```

#### 5) 새로운 homogeneous control point
```math
P_{w,k}'=(P_k\cdot w_k',\; w_k')
```

## 3. Rust 구현 구조
### 3.1 전체 흐름
- PREPARE 단계:
    - pk, wk 계산 (고정)
    - rku 계산
    - pkp 계산
    - 첫 weight 적용
    - 세션 객체 반환

- INTERACT 단계:
    - PREPARE에서 계산된 pk, wk, pkp, rku 재사용
    - d 값만 바꿔 weight 재계산

## 4. 코드 설명
- 아래는 문서화된 형태의 코드 설명이다.

### 4.1 Index 검증
```rust
fn ensure_ctrl_index(&self, k: usize) -> Result<()> {
    if k >= self.ctrl.len() {
        return Err(NurbsError::InvalidIndex { k, n: self.ctrl.len().saturating_sub(1) });
    }
    Ok(())
}
```
- 역할
    - control point index가 유효한지 확인
    - PREPARE/INTERACT 모두에서 안전성 확보

### 4.2 PREPARE 단계
```rust
pub fn mod_shape_weight_prepare(&mut self, k: usize, u: f64, d: f64)
    -> Result<ShapingModWeightSession>
```

- 수행 내용
    - 곡선 유효성 검사
    - index 검사
    - control point에서
    - Euclidean 좌표 P_k
    - weight w_k
- 추출
    - rational basis $R_k(u)$ 계산
    - 곡선 점 $C(u)$ 계산
    - 거리 $\mathrm{pkp}=\| C(u)-P_k\|$  계산
    - 첫 weight 업데이트 수행
    - 세션 객체 반환

### 세션에 저장되는 값
| Name | Description                     |
|------|---------------------------------|
| pk   | Euclidean control point P_k     |
| wk   | Original weight w_k             |
| pkp  | Distance between C(u) and P_k   |
| rku  | Rational basis value R_k(u)     |
| k    | Control point index             |
| u    | Parameter value                 |


- 이 값들은 INTERACT 단계에서 재사용된다.

### 4.3 INTERACT 단계
```rust
pub fn mod_shape_weight_interact(&mut self, sess: &ShapingModWeightSession, d: f64)
````

- 수행 내용
    - PREPARE에서 계산된 값들을 그대로 사용
    - d 값만 바꿔 weight 재계산
    - UI 슬라이더와 같은 인터랙티브 조작에 적합

### 4.4 핵심 weight 업데이트 함수
```rust
fn mod_shape_weight_apply_with_cached(...)
```

- 수행 내용
- 분모 계산
```math
\mathrm{den}=R_k(u)(\mathrm{pkp}-d)
```
- 분모 검증
    - 0 또는 비정상 값이면 오류
    - 새로운 weight 계산
```math
w_k'=w_k\left( 1+\frac{d}{\mathrm{den}}\right) 
```
- weight 범위 검증
- 새로운 homogeneous control point 생성
- 곡선에 적용

## 5. 검증 포인트
- ✔ PREPARE에서 pk, wk는 고정
    - INTERACT에서 다시 계산하면 안 됨
        - 원본 알고리즘과 동일하게 구현됨
- ✔ rational basis 계산 정확
    - span 찾기
    - basis 계산
    - 분모 계산
    - $R_k(u)=\frac{N_{k,p}(u)w_k}{\sum N_{j,p}(u)w_j}$
- ✔ weight 업데이트 공식 정확
    - Piegl & Tiller 공식 그대로
- ✔ weight 범위 제한
    - 수치 폭주 방지
- ✔ 2D curve 처리
    - z=0 강제

## 6. 예제
```rust
let mut sess = curve.mod_shape_weight_prepare(k, u, d0)?;
curve.mod_shape_weight_interact(&sess, d1)?;
curve.mod_shape_weight_interact(&sess, d2)?;
```


## 7. 결론
- 이 구현은:
    - Piegl & Tiller의 N_SHACMW 알고리즘을 정확히 재현
    - PREPARE/INTERACT 구조를 그대로 유지
    - 수학적으로 안정적
    - UI/툴에서 실시간 weight shaping에 적합
    - NURBS 구조를 보존하면서 곡선을 부드럽게 변형
- 즉, CAD 커널 수준의 고급 Shape Editing 기능을 완벽하게 구현한 것이다.


---

## 소스 코드
```rust
impl NurbsCurve {
    #[inline]
    #[allow(unused)]
    fn ensure_ctrl_index(&self, k: usize) -> Result<()> {
        if k >= self.ctrl.len() {
            return Err(NurbsError::InvalidIndex { k, n: self.ctrl.len().saturating_sub(1) });
        }
        Ok(())
    }
```
```rust
    pub fn mod_shape_weight_prepare(&mut self, k: usize, u: f64, d: f64) -> Result<ShapingModWeightSession> {

        if !self.is_valid() {
            return Err(NurbsError::InvalidCurve);
        }

        // ✅ 반드시 ctrl[k] 접근 전에 체크
        let n = self.ctrl.len();
        if k >= n {
            return Err(NurbsError::InvalidIndex { k, n: n.saturating_sub(1) });
        }

        // ctrl[k]에서 pk와 wk 추출 (단, pk는 이후 고정)
        let cp = self.ctrl[k];
        let pk = cp.from_w();
        let wk = cp.w;

        let rku = self.eval_basis_funs(k, u, Side::Left);
        let p = self.eval_point(u);
        let pkp = Point3D::distance_squared_point(&pk, &p).sqrt();

        // 첫 적용
        self.mod_shape_weight_apply_with_cached(k, pk, wk, pkp, rku, d)?;

        Ok(ShapingModWeightSession { k, u, pk, wk, pkp, rku })
    }
```
```rust
    pub fn mod_shape_weight_interact(&mut self, sess: &ShapingModWeightSession, d: f64) -> Result<()> {
        self.mod_shape_weight_apply_with_cached(sess.k, sess.pk, sess.wk, sess.pkp, sess.rku, d)
    }
```
```rust
    fn mod_shape_weight_apply_with_cached(
        &mut self,
        k: usize,
        pk: Point3D,
        wk: f64,
        pkp: f64,
        rku: f64,
        d: f64,
    ) -> Result<()> {
        let den = rku * (pkp - d);
        if !den.is_finite() || den.abs() < 1.0e-15 {
            return Err(NurbsError::DivisionByZero);
        }

        let wh = wk * (1.0 + d / den);
        if !(CURVE_W_MIN..=CURVE_W_MAX).contains(&wh) || !wh.is_finite() {
            return Err(NurbsError::WeightOutOfRange { w: wh, w_min: CURVE_W_MIN, w_max: CURVE_W_MAX });
        }

        let mut new_cp = Point4D { x: pk.x * wh, y: pk.y * wh, z: pk.z * wh, w: wh };
        if self.dimension != 3 { new_cp.z = 0.0; }

        self.ctrl[k] = new_cp;
        Ok(())
    }
}
```

--- 
## 테스트 코드
```rust
#[cfg(test)]
mod mod_shape_weight_session_tests {
    use nurbslib::core::prelude::{NurbsCurve, Point3D};
    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }
```
```rust
    #[test]
    fn mod_shape_weight_prepare_and_interact_line_expected_weights() {
        // line: (0,0,0) -> (1,0,0), degree=1, weights=1 가정
        let mut c = NurbsCurve::from_line(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        assert!(c.is_valid());

        let k = 0usize;
        let u = 0.5;

        // PREPARE at d=0.1
        let sess = c.mod_shape_weight_prepare(k, u, 0.1).expect("prepare should succeed");

        // line이면:
        // Pk=(0,0,0), C(0.5)=(0.5,0,0) => pkp=0.5
        // rku = 0.5 (left basis)
        assert!(approx(sess.wk,  1.0, 1e-12));
        assert!(approx(sess.pkp, 0.5, 1e-12));
        assert!(approx(sess.rku, 0.5, 1e-12));
        assert!(approx(sess.pk.x, 0.0, 1e-12));
        assert!(approx(sess.pk.y, 0.0, 1e-12));
        assert!(approx(sess.pk.z, 0.0, 1e-12));

        // PREPARE가 첫 적용까지 했으므로 weight가 바뀌었어야 함:
        // den=0.5*(0.5-0.1)=0.2, wh=1*(1+0.1/0.2)=1.5
        assert!(approx(c.ctrl[k].w, 1.5, 1e-12));

        // INTERACT at d=0.2
        c.mod_shape_weight_interact(&sess, 0.2).expect("interact should succeed");

        // den=0.5*(0.5-0.2)=0.15, wh=1*(1+0.2/0.15)=2.333333333333333...
        assert!(approx(c.ctrl[k].w, 2.333333333333333, 1e-12));
    }
```
```rust
    #[test]
    fn mod_shape_weight_interact_is_invariant_to_ctrl_mutation_after_prepare() {
        // 이 테스트가 네가 지적한 "기준점이 움직이면 안 된다"를 정확히 잡아냄.
        let mut c1 = NurbsCurve::from_line(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let mut c2 = c1.clone();

        let k = 0usize;
        let u = 0.5;

        // 동일한 prepare
        let sess1 = c1.mod_shape_weight_prepare(k, u, 0.1).unwrap();
        let sess2 = c2.mod_shape_weight_prepare(k, u, 0.1).unwrap();

        // c2는 prepare 이후 control point를 일부러 망가뜨림 (homogeneous xyz 조작)
        // "INTERACT에서 ctrl[k]로부터 pk를 다시 뽑는 구현"이면 결과가 달라질 수 있다.
        c2.ctrl[k].x += 123.456;
        c2.ctrl[k].y -= 78.9;
        c2.ctrl[k].z += 0.111;

        // 동일한 d로 INTERACT 수행
        c1.mod_shape_weight_interact(&sess1, 0.2).unwrap();
        c2.mod_shape_weight_interact(&sess2, 0.2).unwrap();

        // ✅ 세션 캐시 방식이면 ctrl를 망가뜨려도 결과 weight는 같아야 한다.
        assert!(approx(c1.ctrl[k].w, c2.ctrl[k].w, 1e-12));

        // (원하면 homogeneous xyz까지도 동일해지길 기대할 수 있음)
        // 단, 2D z 강제 등 정책이 있다면 xyz 비교는 보수적으로.
    }
```
```rust
    #[test]
    fn mod_shape_weight_errors_on_denominator_zero() {
        let mut c = NurbsCurve::from_line(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let k = 0usize;
        let u = 0.5;

        // prepare: pkp를 얻기 위함
        let sess = c.mod_shape_weight_prepare(k, u, 0.1).unwrap();

        // d == pkp  -> (pkp - d) == 0 -> den == 0
        let err = c.mod_shape_weight_interact(&sess, sess.pkp);
        assert!(err.is_err());
    }
```
```rust
    #[test]
    fn mod_shape_weight_errors_on_weight_out_of_range() {
        let mut c = NurbsCurve::from_line(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let k = 0usize;
        let u = 0.5;

        let sess = c.mod_shape_weight_prepare(k, u, 0.1).unwrap();

        // pkp에 아주 가깝게 d를 두면 (pkp-d)가 매우 작아져 wh가 폭증 -> WMAX 초과 유도 가능
        let d = sess.pkp - 1e-12;
        let err = c.mod_shape_weight_interact(&sess, d);
        assert!(err.is_err());
    }
```
```rust
    #[test]
    fn mod_shape_weight_prepare_errors_on_bad_index() {
        let mut c = NurbsCurve::from_line(Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0));
        let bad_k = 999usize;

        let err = c.mod_shape_weight_prepare(bad_k, 0.5, 0.1);
        assert!(err.is_err());
    }
}
```
---
