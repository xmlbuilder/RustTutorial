# Arc
이제 전체 Arc 구조체의 기능을 수식과 설명 중심으로 정리.

## 🧮 수식 및 기능 정리: Arc 구조체
## 📌 1. 원호의 파라미터 계산
###  project(p: Point) → f64
- 3D 점 p를 원호에 사영하여 대응되는 각도 t를 반환
- Circle::project를 통해 원 위의 각도 t를 얻고, Arc의 각도 구간에 맞게 보정
- 보정 로직:

$$
\mathrm{If\  }t\notin [t_0,t_1],\mathrm{\  then\  try\  }t+\pi \mathrm{\  or\  }t-\pi \mathrm{\  as\  candidates}
$$


### ✅ closest_param_to(p: Point) → f64
- 점 p에 가장 가까운 원호의 파라미터 t를 반환
- project(p)로 얻은 t가 도메인 밖이면 start_point 또는 end_point 중 가까운 쪽으로 스냅
- 도메인 안이면 중심-사영점 거리와 끝점 거리 비교하여 스냅 여부 결정

## 📏 2. 길이 ↔ 파라미터 변환
### ✅ get_param_from_length(length: f64) → (bool, f64)
- 길이 ℓ에 대응되는 각도 u 계산:

$$
u=t_0+\mathrm{dir}\cdot \frac{\ell }{r}
$$

- dir = ±1 (각도 증가 방향)
- r = 반지름

### ✅ get_length_from_param(u: f64) → (bool, f64)
- 각도 u에 대응되는 길이 계산:

$$
\ell =|(u-t_0)\cdot \mathrm{dir}|\cdot r
$$

## 🧪 3. 샘플링 및 기하 연산
### ✅ points_by_length(step: f64) → Vec<Point>
- 원호를 길이 기준으로 균등 분할하여 점 샘플링
- 분할 수:

$$
n=\left\lceil \frac{\mathrm{length}}{\mathrm{step}}\right\rceil
$$ 

### ✅ tight_bbox() → (Point, Point)
- 원호의 최소/최대 좌표를 계산하여 AABB 반환
- 전체 원이면 circle.get_tight_bbox() 사용
- 부분 원이면:
- 후보 각도: t0, t1, atan2(ay.axis, ax.axis) (x/y/z 각각)
- 각 후보 각도에 대해 point_at(t) 계산 후 min/max 갱신

## 🧵 4. NURBS 변환
### ✅ to_nurbs() → Curve
- 원호를 2차 NURBS 곡선으로 변환
- 최대 90° 단위로 분할하여 각 세그먼트를 3개의 제어점으로 표현
- 중간 제어점의 weight:

$$
w=\cos \left( \frac{\theta }{2}\right) - 제어점 수: 2 * segments + 1
$$

- Knot 수: 2 * segments + 4 (clamped)

## 🧩 기타 유틸리티

| 메서드                         | 설명                                      |
|-------------------------------|-------------------------------------------|
| wrap_to_2pi(t)                | 각도 t를 [0, 2π) 범위로 정규화             |
| of(center, radius, angle_len) | 중심, 반지름, 각도 길이로 간편 생성자      |
| is_linear(tol)                | 선형 여부 판단 (항상 false 반환)          |


## 📐 Arc 관련 수식 정리

### 1. 원호 길이
```
length = |Δθ| × r
        = |t1 - t0| × r
```

### 2. 보정 로직 (Arc::project)
```
if t ∉ [t0, t1] then candidates = { t + π, t - π }
```

### 3. 길이 → 파라미터 변환 (get_param_from_length)
```
u = t0 + dir × (length / r)
where dir = +1 if t1 ≥ t0, else -1
```

### 4. 파라미터 → 길이 변환 (get_length_from_param)
```
length = |(u - t0) × dir| × r
```

### 5. NURBS 변환 시 중간 제어점 가중치
```
w = cos(Δθ / 2)
(Δθ: 세그먼트 각도 길이)
```

### 6. 원호 샘플링 분할 수
```
n = ceil(length / step)
```

## 테스트 코드
테스트 코드를 기능별로 정리한 표와 함께 정리.

| 테스트 함수                          | 검증 내용 요약                                                  |
|-------------------------------------|------------------------------------------------------------------|
| arc_param_length_roundtrip          | 길이 → 파라미터 → 길이 roundtrip 정확성                         |
| arc_param_length_reverse_direction  | 감소 방향 원호의 길이/파라미터 변환 정확성                      |
| arc_param_endpoint_tolerance_snap   | 끝점 근접 시 스냅 허용 오차 테스트                              |
| arc_to_nurbs                        | NURBS 변환 후 끝점 정확성 검증                                  |
| arc_split_at_midpoint               | 중간 각도에서 split 후 두 원호의 길이 합이 원래와 같은지 확인   |
| arc_trim_at_start                   | trim_at으로 앞쪽 절단 후 길이가 정확히 반영되는지 확인          |
| arc_sub_curve_full_circle           | 전체 원에서 sub_curve 호출 시 원본과 동일한지 확인             |            | NURBS 변환 후 끝점 정확성 검증                                  |



## 🧪 테스트 코드

### 1. arc_param_length_roundtrip
```rust
fn circle_param_length_round_trip() {
    let r = 2.5;
    let c = GCircle::from_center(Point3D::new(1.0, -2.0, 0.5), r).expect("circle");

    // 전체 길이
    let total = c.length(); // = 2πr
    assert!(close(total, 2.0 * ON_PI * r, 1e-12));

    // length=0 → t = domain.t0
    let (ok0, t0) = c.get_param_from_length(0.0);
    assert!(ok0);
    assert!(close(t0, c.domain().t0, 1e-12));

    // length = 전체 → t = domain.t1
    let (ok1, t1) = c.get_param_from_length(total);
    assert!(ok1);
    assert!(close(t1, c.domain().t1, 1e-12));

    // 중간 길이(π r) → t = t0 + π
    let mid_len = ON_PI * r;
    let (okm, tm) = c.get_param_from_length(mid_len);
    assert!(okm);
    assert!(close(tm, c.domain().t0 + ON_PI, 1e-12));

    // 역변환 체크
    let (okl0, l0) = c.get_length_from_param(t0);
    let (oklm, lm) = c.get_length_from_param(tm);
    let (okl1, l1) = c.get_length_from_param(t1);
    assert!(okl0 && oklm && okl1);
    assert!(close(l0, 0.0, 1e-12));
    assert!(close(lm, mid_len, 1e-12));
    assert!(close(l1, total, 1e-12));
}
```

### 2. circle_param_length_out_of_range
```rust
#[test]
fn circle_param_length_out_of_range() {
    let r = 3.0;
    let c = GCircle::from_center(Point3D::new(0.0, 0.0, 0.0), r).expect("circle");
    let total = c.length();

    // 음수 길이
    let (ok_neg, tneg) = c.get_param_from_length(-1.0);
    assert!(!ok_neg);
    assert!(close(tneg, c.domain().t0, 1e-12));

    // 전체를 초과
    let (ok_over, tover) = c.get_param_from_length(total + 1e-3);
    assert!(!ok_over);
    assert!(close(tover, c.domain().t0, 1e-12));

    // 도메인 밖 파라미터 → length 변환 실패
    let (oklen_neg, _) = c.get_length_from_param(c.domain().t0 - 1e-6);
    let (oklen_over, _) = c.get_length_from_param(c.domain().t1 + 1e-6);
    assert!(!oklen_neg && !oklen_over);
}
```

### 3. arc_param_length_roundtrip
```rust
#[test]
fn arc_param_length_roundtrip() {
    // 가정: GArc::new(plane, center, radius, t0, t1) 혹은 유사 생성자가 있음.
    let plane = Plane::xy();
    let center = Point3D::new(0.0, 0.0, 0.0);
    let r = 4.0;
    let t0 = 0.5;
    let t1 = 2.0;
    let arc = GArc::new(plane, center, r, t0, t1).expect("arc");

    // 전체 호 길이 = |t1 - t0|*r
    let total = arc.length();
    assert!(close(total, (t1 - t0).abs() * r, 1e-12));

    // length 0 → t0
    let (ok0, u0) = arc.get_param_from_length(0.0);
    assert!(ok0 && close(u0, t0, 1e-12));

    // length total → t1
    let (ok1, u1) = arc.get_param_from_length(total);
    assert!(ok1 && close(u1, t1, 1e-12));

    // 40% 길이 지점
    let l40 = 0.4 * total;
    let (ok40, u40) = arc.get_param_from_length(l40);
    assert!(ok40);
    let (okl40, back_l40) = arc.get_length_from_param(u40);
    assert!(okl40);
    assert!(close(back_l40, l40, 1e-12));
}
```

### 4. arc_param_length_reverse_direction
```rust
#[test]
fn arc_param_length_reverse_direction() {
    let plane = Plane::xy();
    let center = Point3D::new(0.0, 0.0, 0.0);
    let r = 2.0;
    let t0_in = 1.8;
    let t1_in = 0.7; // 감소

    let arc = GArc::new(plane, center, r, t0_in, t1_in).expect("arc-rev");

    // 실제 도메인(생성자에서 정규화/역전될 수 있음)
    let dom = arc.domain();
    let total = arc.length(); // = |t1 - t0| * r

    // length=0 -> dom.t0
    let (ok0, u0) = arc.get_param_from_length(0.0);
    assert!(ok0, "should map length=0");
    assert!(close(u0, dom.t0, 1e-12), "u0 must equal domain.t0");

    // length=total -> dom.t1
    let (ok1, u1) = arc.get_param_from_length(total);
    assert!(ok1, "should map length=total");
    assert!(close(u1, dom.t1, 1e-12), "u1 must equal domain.t1");

    // 중간
    let half = 0.5 * total;
    let (okm, um) = arc.get_param_from_length(half);
    assert!(okm);
    let (oklm, lm) = arc.get_length_from_param(um);
    assert!(oklm && close(lm, half, 1e-12));
}
```

### 5. arc_param_endpoint_tolerance_snap
```rust
#[test]
fn arc_param_endpoint_tolerance_snap() {
    // 끝점 스냅 허용 오차 확인
    let plane = Plane::xy();
    let center = Point3D::new(0.0, 0.0, 0.0);
    let r = 5.0;
    let t0 = 0.2;
    let t1 = 1.4;
    let arc = GArc::new(plane, center, r, t0, t1).expect("arc");

    let total = arc.length();
    let eps = total * 1e-14; // 매우 작은 오차

    // 거의 0
    let (ok_a, ua) = arc.get_param_from_length(0.0 + eps);
    assert!(ok_a);
    // 아주 근접하면 t0로 스냅되진 않을 수도 있지만, 아래 역변환이 안정적이어야 함
    let (ok_la, la) = arc.get_length_from_param(ua);
    assert!(ok_la);
    assert!(close(la, eps, 1e-10));

    // 거의 total
    let (ok_b, ub) = arc.get_param_from_length(total - eps);
    assert!(ok_b);
    let (ok_lb, lb) = arc.get_length_from_param(ub);
    assert!(ok_lb);
    assert!(close(lb, total - eps, 1e-10));
}
```
### 6. arc_split_at_midpoint
```rust
#[test]
fn arc_split_at_midpoint() {
    let plane = Plane::xy();
    let center = Point::new(0.0, 0.0, 0.0);
    let r = 3.0;
    let t0 = 0.0;
    let t1 = std::f64::consts::PI;
    let arc = Arc::new(plane, center, r, t0, t1).expect("arc");

    let mid = (t0 + t1) * 0.5;
    let (arc1, arc2) = arc.split_at(mid).expect("split");

    assert!(close(arc1.length() + arc2.length(), arc.length(), 1e-12));
    assert!(on_are_point_close(&arc1.end_point(), &arc2.start_point(), ON_TOL12));
}
```

### 7. arc_trim_at_start
```rust
#[test]
fn arc_trim_at_start() {
    let plane = Plane::xy();
    let center = Point::new(0.0, 0.0, 0.0);
    let r = 2.0;
    let arc = Arc::new(plane, center, r, 0.0, std::f64::consts::PI).expect("arc");

    let mut arc_clone = arc;
    let ok = arc_clone.trim_at(std::f64::consts::FRAC_PI_2, false);
    assert!(ok);
    assert!(close(arc_clone.length(), r * std::f64::consts::FRAC_PI_2, 1e-12));
}
```

### 8. arc_sub_curve_full_circle
```rust
#[test]
fn arc_sub_curve_full_circle() {
    let plane = Plane::xy();
    let center = Point::new(0.0, 0.0, 0.0);
    let r = 1.0;
    let arc = Arc::new(plane, center, r, 0.0, std::f64::consts::TAU).expect("full circle");

    let sub = arc.sub_curve(0.0, std::f64::consts::TAU).expect("subcurve");
    assert!(close(sub.length(), arc.length(), 1e-12));
    assert!(on_are_point_close(&sub.start_point(), &arc.start_point(), ON_TOL12));
    assert!(on_are_point_close(&sub.end_point(), &arc.end_point(), ON_TOL12));
}
```



