# 📘 on_non_rational_point_curve
- **하나의 점 P만을 나타내는 비유리(non‑rational) B‑Spline 곡선 생성기**

## 1. 이 함수가 만드는 곡선의 의미
- 이 함수는 다음 조건을 만족하는 NURBS 곡선을 만든다:
  - 모든 u에서 C(u) = P
  - 즉, 곡선 전체가 하나의 점 P로 “수축된” 형태
  - control points 전부가 동일한 점 P
  - weight = 1 → non‑rational
  - knot vector는 open‑clamped + 내부 균등 분포
- 이런 곡선은 CAD 시스템에서 다음 용도로 자주 쓰인다:
  - 곡선 생성 초기화
  - 디폴트 곡선
  - sweep/loft에서 degenerate boundary
  - patch의 singular corner
  - 테스트용 curve

## 2. on_non_rational_point_curve 입력

| 이름 | 의미 |
|------|--------------------------------------------------------------|
| `pt` | 곡선이 항상 지나야 하는 점 P (Point3D)                      |
| `n`  | control point의 마지막 인덱스 → control points 개수 = n+1    |
| `p`  | 곡선 차수 (degree)                                           |



## 3. Control Points 구성
- 모든 control point는 동일한 점 P를 사용한다.
```math
P_i=(x,y,z,1),\quad i=0..n
```
- 즉,
```math
C(u)=\sum _{i=0}^nN_{i,p}(u)P
```
- 하지만 basis 함수의 합은 항상 1이므로:
```math
C(u)=P
```

## 4. Knot Vector 구성
### 4.1 Knot Vector 길이
- B‑Spline 곡선에서:
  - control point 개수 = n+1
  - degree = p
- knot vector 길이:
```math
m=n+p+1
```
```math
\mathrm{len}=m+1=n+p+2
```
- 코드:
```rust
let m = n + p_usize + 1;
let mut knots = vec![0.0; m + 1];
```

## 5. Knot Vector 수식
- 이 함수는 open‑clamped + 내부 균등 분포 knot vector를 만든다.
- ✔ 1) 시작 클램프
```math
U_0=U_1=\cdots =U_p=0
```
- 코드:
```math
for i in 0..=p {
    knots[i] = 0.0;
}
```


- ✔ 2) 내부 knot 균등 분포
- 내부 knot 개수:
```math
(n-p)
```
- 균등 간격:
```math
u_{\mathrm{inc}}=\frac{1}{n-p+1}
```
- 내부 knot:
```math
U_{i+p}=i\cdot u_{\mathrm{inc}},\quad i=1..(n-p)
```

- 코드:
```rust
let denom = (n - p + 1) as Real;
let u_inc = 1.0 / denom;

for i in 1..=(n - p) {
    knots[i + p] = (i as Real) * u_inc;
}
```

- ✔ 3) 끝 클램프
```math
U_{n+1}=U_{n+2}=\cdots =U_{n+p+1}=1
```
- 코드:
```rust
for i in (n + 1)..=m {
    knots[i] = 1.0;
}
```



## 6. 왜 이렇게 하면 C(u) = P가 되는가?
- 모든 control point가 동일한 점 P이므로:
```math
C(u)=\sum _{i=0}^nN_{i,p}(u)P=P\sum _{i=0}^nN_{i,p}(u)
```
- B‑Spline basis 함수의 partition of unity 성질:
```math
\sum _{i=0}^nN_{i,p}(u)=1
```
- 따라서:
```math
C(u)=P
```
- 즉, 어떤 u에서도 곡선은 항상 P를 반환한다.

## 7. 이 함수의 용도 요약

- 하나의 점 P만을 나타내는 NURBS 곡선 생성
- sweep/loft/skin에서 degenerate boundary curve
- patch의 singular corner 표현
- 초기화용 dummy curve
- 테스트용 curve

```rust
/// 의미:
/// - n: control point array의 the highest index => control points 개수는 (n+1)
/// - p: degree
/// - 결과 curve는 모든 u에서 C(u)=P (즉, control points 전부 같은 점)
/// - knot vector는 [0..0, 내부 균등, 1..1] open clamped

pub fn on_non_rational_point_curve(pt: Point3D, n: usize, p: Degree) 
  -> Result<NurbsCurve> {
    let p_usize = p as usize;

    if p_usize > n {
        return Err(NurbsError::InvalidArgument {
            msg: format!("on_non_rational_point_curve: degree p={} > n={}", p_usize, n),
        });
    }

    if p_usize > MAX_DEGREE as usize {
        return Err(NurbsError::InvalidDegreeCurve { got: p });
    }

    // control points: Pw[i] = P (non-rational => w=1)
    let mut ctrl: Vec<Point4D> = Vec::with_capacity(n + 1);
    for _ in 0..=n {
        ctrl.push(Point4D::homogeneous(pt.x, pt.y, pt.z, 1.0));
    }

    // knot vector size:
    // m = n+p+1 (highest index) => len = m+1 = n+p+2
    let m = n + p_usize + 1;
    let mut knots: Vec<Real> = vec![0.0; m + 1];

    // u_inc = 1/(n-p+1)
    // (n==p면 분모=1, 내부 knot 없음)
    let denom = (n - p_usize + 1) as Real;
    let u_inc = 1.0 / denom;

    // U[0..=p] = 0
    for i in 0..=p_usize {
        knots[i] = 0.0;
    }

    // U[i+p] = i*u_inc for i=1..=n-p
    // (range empty 가능)
    if n > p_usize {
        for i in 1..=(n - p_usize) {
            knots[i + p_usize] = (i as Real) * u_inc;
        }
    }

    // U[n+1..=m] = 1
    for i in (n + 1)..=m {
        knots[i] = 1.0;
    }

    let kv = KnotVector { knots };

    // domain은 이 코드베이스에서 0..1로 두는 패턴이 많음
    NurbsCurve::new(p, ctrl, kv)
}
```
---

### 테스트 코드
```rust
#[cfg(test)]
mod tests_point_curve {

    use nurbslib::core::geom::Point3D;
    use nurbslib::core::nurbs_curve::on_non_rational_point_curve;
    use nurbslib::core::types::{Real, Degree};

    fn is_nondecreasing(a: &[Real]) -> bool {
        a.windows(2).all(|w| w[0] <= w[1])
    }

    fn is_open_clamped_01(knots: &[Real], p: usize) -> bool {
        if knots.len() < 2 * (p + 1) {
            return false;
        }
        // 앞 p+1개 0, 뒤 p+1개 1
        for i in 0..=p {
            if (knots[i] - 0.0).abs() > 1e-14 {
                return false;
            }
            if (knots[knots.len() - 1 - i] - 1.0).abs() > 1e-14 {
                return false;
            }
        }
        true
    }
```
```rust
    #[test]
    fn point_curve_basic_p3_n7_knots_ctrl_eval() {
        let p: Degree = 3;
        let n = 7usize; // highest index => ctrl count = 8
        let P = Point3D::new(1.25, -2.0, 3.5);

        let c = on_non_rational_point_curve(P, n, p).expect("on_non_rational_point_curve failed");

        // 1) degree / counts
        assert_eq!(c.degree as usize, p as usize);
        assert_eq!(c.ctrl.len(), n + 1);

        // 2) all control points equal to P (w=1)
        for (i, cp) in c.ctrl.iter().enumerate() {
            assert!(
                (cp.x - P.x).abs() < 1e-14 &&
                    (cp.y - P.y).abs() < 1e-14 &&
                    (cp.z - P.z).abs() < 1e-14,
                "ctrl[{}] xyz mismatch: ({},{},{}) vs ({},{},{})",
                i, cp.x, cp.y, cp.z, P.x, P.y, P.z
            );
            assert!((cp.w - 1.0).abs() < 1e-14, "ctrl[{}] w != 1", i);
        }

        // 3) knot vector size & structure
        // C: m = n+p+1 => len = m+1 = n+p+2
        let need_len = n + (p as usize) + 2;
        assert_eq!(c.kv.knots.len(), need_len);

        assert!(is_nondecreasing(&c.kv.knots));
        assert!(is_open_clamped_01(&c.kv.knots, p as usize));

        // 내부 균등 분할: uinc = 1/(n-p+1)
        // for i=1..=n-p: U[i+p] = i*uinc
        let p_usize = p as usize;
        let uinc = 1.0 / ((n - p_usize + 1) as Real);
        for i in 1..=(n - p_usize) {
            let idx = i + p_usize;
            let expect = (i as Real) * uinc;
            let got = c.kv.knots[idx];
            assert!(
                (got - expect).abs() < 1e-14,
                "internal knot mismatch at U[{}]: got {} expect {}",
                idx, got, expect
            );
        }

        // 4) evaluation: any u gives same P (domain 0..1)
        for &t in &[0.0, 0.1, 0.5, 0.9, 1.0] {
            let q = c.eval_point(t);
            assert!(
                (q.x - P.x).abs() < 1e-12 &&
                    (q.y - P.y).abs() < 1e-12 &&
                    (q.z - P.z).abs() < 1e-12,
                "eval mismatch at t={}: ({},{},{}) vs ({},{},{})",
                t, q.x, q.y, q.z, P.x, P.y, P.z
            );
        }
    }
```
```rust
    #[test]
    fn point_curve_bezier_case_n_eq_p_has_no_internal_knots() {
        let p: Degree = 3;
        let n = 3usize; // n==p => 내부 knot 없음
        let P = Point3D::new(-7.0, 0.25, 10.0);

        let c = on_non_rational_point_curve(P, n, p).expect("on_non_rational_point_curve failed");

        // knot size: n+p+2 = 3+3+2=8
        assert_eq!(c.kv.knots.len(), n + (p as usize) + 2);
        assert!(is_nondecreasing(&c.kv.knots));
        assert!(is_open_clamped_01(&c.kv.knots, p as usize));

        // 내부 knot 구간 (i=1..=n-p) 이 비어 있어야 함 (n-p=0)
        // 즉, 0..0..0..0, 1..1..1..1 패턴
        assert_eq!(&c.kv.knots[0..4], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&c.kv.knots[4..8], &[1.0, 1.0, 1.0, 1.0]);

        // eval constant
        for &t in &[0.0, 0.33, 0.66, 1.0] {
            let q = c.eval_point(t);
            assert!((q.x - P.x).abs() < 1e-12);
            assert!((q.y - P.y).abs() < 1e-12);
            assert!((q.z - P.z).abs() < 1e-12);
        }
    }
```
```rust
    #[test]
    fn point_curve_rejects_degree_greater_than_n() {
        let P = Point3D::new(0.0, 0.0, 0.0);
        let n = 2usize;
        let p: Degree = 3; // p>n => error

        let rc = on_non_rational_point_curve(P, n, p);
        assert!(rc.is_err());
    }
}
```
---
