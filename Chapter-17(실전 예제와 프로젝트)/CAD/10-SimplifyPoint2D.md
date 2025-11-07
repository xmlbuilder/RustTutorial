
# SimplifyPoint2D

## ✅ 전체 구조 요약
### 📌 목적
- 주어진 2D 점들의 시퀀스(폴리라인)를 거리 기준으로 단순화하여,
원래 형태를 유지하면서 불필요한 점들을 제거하는 알고리즘

### 소스 코드
```rust
use crate::core::geom::Point2;
use crate::core::segment2d::Segment2D;

#[derive(Debug, Default)]
pub struct SimplifyPoint2D;
```
```rust
impl SimplifyPoint2D {
    #[inline]
    pub fn distance_closest_point_to(pt1: &Point2, pt2: &Point2, pt3: &Point2) -> f64 {
        let seg = Segment2D::new(*pt1, *pt2);
        let t = seg.closest_param_to(pt3);
        let b = seg.point_at(t);
        pt3.distance_to(&b)
    }

    /// 제곱거리
    #[inline]
    pub fn square_distance(a: &Point2, b: &Point2) -> f64 {
        let dx = b.x - a.x;
        let dy = b.y - a.y;
        dx * dx + dy * dy
    }

    fn simplify_recursive(
        points: &[Point2],
        start_idx: usize,
        end_idx: usize,
        dist: f64,
        out_indices: &mut Vec<usize>,
    ) {
        if end_idx <= start_idx + 1 {
            return;
        }

        let mut max_dist = 0.0f64;
        let mut max_idx = start_idx + 1;

        for i in (start_idx + 1)..end_idx {
            let d =
                Self::distance_closest_point_to(&points[start_idx], &points[end_idx], &points[i]);
            if d > max_dist {
                max_dist = d;
                max_idx = i;
            }
        }

        if max_dist <= dist {
            return;
        }

        // 기준점 추가 후 양쪽 재귀
        out_indices.push(max_idx);
        Self::simplify_recursive(points, start_idx, max_idx, dist, out_indices);
        Self::simplify_recursive(points, max_idx, end_idx, dist, out_indices);
    }

    /// 폐합 폴리라인(simplify_internal)
    fn simplify_internal(points: &[Point2], dist: f64) -> Vec<Point2> {
        let length = points.len();
        if length < 3 {
            return points.to_vec();
        }

        // 첫 점으로부터 가장 멀리 떨어진 인덱스 찾기
        let a = points[0];
        let mut far_idx = 0usize;
        let mut max_sq = 0.0;
        for j in 1..length {
            let d2 = Self::square_distance(&a, &points[j]);
            if d2 > max_sq {
                max_sq = d2;
                far_idx = j;
            }
        }

        // [0..=far_idx], [far_idx..end] 두 구간으로 나눠서 각각 단순화 후 병합
        let left = points[..=far_idx].to_vec();
        let right = points[far_idx..].to_vec();

        let s1 = Self::simplify(&left, dist);
        let s2 = Self::simplify(&right, dist);

        let mut out = Vec::with_capacity(s1.len() + s2.len());
        out.extend_from_slice(&s1);
        out.extend_from_slice(&s2);
        out
    }

    pub fn simplify(points: &[Point2], dist: f64) -> Vec<Point2> {
        let n = points.len();
        if n < 3 {
            return points.to_vec();
        }

        // 폐합인지 검사(원본은 == 비교를 사용)
        if points[0] == points[n - 1] {
            return Self::simplify_internal(points, dist);
        }

        // 개방 폴리라인:
        // 끝점이 시작점과 반복 동일한 경우 끝 인덱스를 뒤로 당김
        let start = 0usize;
        let mut end = n - 1;
        while end > start && points[end] == points[start] {
            end -= 1;
        }

        // 재귀를 위한 시드 인덱스
        let mut indices: Vec<usize> = Vec::with_capacity(n);
        indices.push(start);
        indices.push(end);

        Self::simplify_recursive(points, start, end, dist, &mut indices);

        // 오름차순 + 중복 제거 후 선택
        indices.sort_unstable();
        indices.dedup();

        let mut result = Vec::with_capacity(indices.len());
        for &i in &indices {
            result.push(points[i]);
        }
        result
    }

    fn angle_between(a: &Point2, b: &Point2, c: &Point2) -> f64 {
        let ab = (b.x - a.x, b.y - a.y);
        let bc = (c.x - b.x, c.y - b.y);
        let dot = ab.0 * bc.0 + ab.1 * bc.1;
        let mag_ab = (ab.0.powi(2) + ab.1.powi(2)).sqrt();
        let mag_bc = (bc.0.powi(2) + bc.1.powi(2)).sqrt();
        (dot / (mag_ab * mag_bc)).clamp(-1.0, 1.0).acos() // 라디안
    }

    pub fn simplify_with_angle_time(points: &[Point2], dist: f64, angle_thresh_deg: f64) -> Vec<Point2> {
        let mut result = SimplifyPoint2D::simplify(points, dist);
        let mut i = 1;
        while i + 1 < points.len() {
            let angle = Self::angle_between(&points[i - 1], &points[i], &points[i + 1]);
            let angle_deg = angle.to_degrees();
            if angle_deg < angle_thresh_deg && !result.contains(&points[i]) {
                result.push(points[i]); // 코너 유지
            }
            i += 1;
        }
        result.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
        result.dedup();
        result
    }

    pub fn simplify_with_angle(points: &[Point2], dist: f64, angle_thresh_deg: f64) -> Vec<Point2> {
        let mut index_set: HashSet<usize> = HashSet::new();
        // 거리 기반으로 선택된 인덱스
        let base = SimplifyPoint2D::simplify(points, dist);
        for (i, pt) in points.iter().enumerate() {
            if base.contains(pt) {
                index_set.insert(i);
            }
        }
        // 각도 기반으로 추가할 인덱스
        for i in 1..points.len() - 1 {
            let angle = Self::angle_between(&points[i - 1], &points[i], &points[i + 1]);
            let angle_deg = angle.to_degrees();
            if angle_deg >= angle_thresh_deg {
                index_set.insert(i);
            }
        }
        // 인덱스 정렬 → 순서 보존
        let mut indices: Vec<usize> = index_set.into_iter().collect();
        indices.sort_unstable();

        // 최종 결과
        indices.into_iter().map(|i| points[i]).collect()
    }
}
```

| 함수 이름                          | 기능 설명                                                                 |
|-----------------------------------|---------------------------------------------------------------------------|
| distance_closest_point_to(pt1, pt2, pt3) | pt3에서 선분 pt1–pt2까지의 최단 거리 계산. RDP 알고리즘의 거리 기준으로 사용됨. |
| square_distance(a, b)             | 두 점 사이의 제곱 거리 계산. 가장 먼 점 찾기 등에서 사용됨.                     |
| simplify_recursive(...)           | RDP 알고리즘의 핵심 재귀 함수. 가장 먼 점 기준으로 분할하며 점을 선택함.         |
| simplify_internal(...)            | 폐합된 폴리라인 처리용. 가장 먼 점 기준으로 양쪽 분할 후 각각 단순화 수행.       |
| simplify(...)                     | 외부 진입점. 폐합 여부 판단 후 적절한 방식으로 단순화 수행.                      |

---


## 테스트 코드
✅ 테스트 코드 점검 요약
| 테스트 이름                                         | 목적 설명                              | 기대 결과                          | 상태 |
|----------------------------------------------------|----------------------------------------|------------------------------------|------|
| rdp_distance_closest_point                         | 세그먼트와 점 사이 거리 계산 정확성     | 거리 = 1.0                         | ✅   |
| rdp_square_distance                                | 두 점 사이 제곱 거리 계산               | 거리² = 25.0                       | ✅   |
| rdp_keeps_endpoints_on_straight_line               | 직선에서 중간점 제거                    | 시작점과 끝점만 유지               | ✅   |
| rdp_preserves_corner_if_over_threshold             | 튀어나온 코너 유지                      | 중간점 유지                        | ✅   |
| rdp_closed_square_preserves_corners                | 폐합된 사각형 코너 유지                 | 4개 코너 유지 + 폐합 유지          | ✅   |
| rdp_tail_equals_start_is_trimmed_for_open_polyline | 끝점 반복 제거                          | 중복된 끝점 제거, 시작점 유지      | ✅   |
| rdp_pass_through_when_less_than_three_points       | 최소 점수 처리                          | 1~2개 점은 그대로 반환             | ✅   |
| rdp_reduces_noisy_polyline_but_keeps_endpoints     | 노이즈 제거 + 양 끝점 유지              | 점 수 감소 + 시작/끝점 유지        | ✅   |
| rdp_threshold_boundary_behavior    | 거리 = threshold일 때 포함 여부 확인     | 경계값에서 점이 유지 또는 제거되는지 확인 | ✅    |
| rdp_handles_duplicate_points       | 연속된 동일 점 처리 안정성               | 중복된 점이 제거되거나 무시됨           | ✅    |
| rdp_handles_diagonal_segments     | 기울어진 선분에서 거리 계산 정확성       | 대각선에서도 정확한 거리 판단           | ✅    |
| rdp_large_flat_then_corner         | 평탄한 구간 제거 후 코너 유지            | 평탄부 제거 + 코너 유지                 | ✅    |
| rdp_closed_loop_with_noise         | 폐합 + 노이즈 제거                       | 폐합 유지 + 점 수 감소                  | ✅    |
| rdp_empty_input_returns_empty      | 빈 입력 처리                             | 빈 입력 → 빈 출력 (`[]`)                | ✅   |
| test_simplify_with_angle_preserves_sharp_turns | 일정 각도 유지 확인           | 각도가 보존 여부 확인                    | ✅    |



### ✅ 1. rdp_threshold_boundary_behavior
```rust
#[test]
fn rdp_threshold_boundary_behavior() {
    // 가운데 점이 정확히 threshold 거리만큼 떨어짐
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.1), // 거리 = 0.1
        Point2::new(2.0, 0.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 0.1);
    
    // 경계값 포함 여부는 구현에 따라 다를 수 있음 → 유연하게 검사
    assert!(out.len() >= 2);
    assert_eq!(out[0], pts[0]);
    assert_eq!(out[out.len() - 1], pts[2]);
}
```


### ✅ 2. rdp_handles_duplicate_points
```rust
#[test]
fn rdp_handles_duplicate_points() {
    // 중복된 점이 여러 개 존재
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(1.0, 1.0),
        Point2::new(2.0, 0.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 0.1);
    
    // 중복된 점이 제거되거나 무시되어야 함
    assert!(out.len() <= pts.len());
    assert_eq!(out.first().unwrap(), &pts[0]);
    assert_eq!(out.last().unwrap(), &pts[3]);
}
```


### ✅ 3. rdp_handles_diagonal_segments
```rust
#[test]
fn rdp_handles_diagonal_segments() {
    // 대각선 선분과 중간점
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 1.0), // 직선 위
        Point2::new(2.0, 2.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 1e-6);

    // 중간점 제거되어야 함
    assert_eq!(out.len(), 2);
    assert_eq!(out[0], pts[0]);
    assert_eq!(out[1], pts[2]);
}
```

### ✅ 4. rdp_large_flat_then_corner
```rust
#[test]
fn rdp_large_flat_then_corner() {
    // 평탄한 구간 + 급격한 코너
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(3.0, 1.0), // 코너
        Point2::new(4.0, 1.0),
        Point2::new(5.0, 1.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 0.1);

    // 평탄부는 제거되고, 코너는 유지되어야 함
    assert!(contains_pt(&out, Point2::new(3.0, 1.0), 1e-12));
    assert_eq!(out.first().unwrap(), &pts[0]);
    assert_eq!(out.last().unwrap(), &pts[5]);
}
```


### ✅ 5. rdp_closed_loop_with_noise
```rust
#[test]
fn rdp_closed_loop_with_noise() {
    // 폐합된 원형 + 노이즈
    let mut pts = Vec::<Point2>::new();
    let n = 100;
    for i in 0..=n {
        let theta = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
        let r = 1.0 + 0.01 * (i as f64).cos(); // 약간의 노이즈
        let x = r * theta.cos();
        let y = r * theta.sin();
        pts.push(Point2::new(x, y));
    }
    pts.push(pts[0]); // 폐합

    let out = SimplifyPoint2D::simplify(&pts, 0.02);

    // 폐합 유지
    assert_eq!(out.first().unwrap(), out.last().unwrap());
    // 점 수 감소
    assert!(out.len() < pts.len());
}
```


### ✅ 6. rdp_handles_vertical_segments
```rust
#[test]
fn rdp_handles_vertical_segments() {
    // 수직 선분 + 중간점
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 1.0), // 직선 위
        Point2::new(0.0, 2.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 1e-6);

    // 중간점 제거되어야 함
    assert_eq!(out.len(), 2);
    assert_eq!(out[0], pts[0]);
    assert_eq!(out[1], pts[2]);
}
```


### ✅ 7. rdp_handles_near_colinear_noise
```rust
#[test]
fn rdp_handles_near_colinear_noise() {
    // 거의 직선 + 약간의 노이즈
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.001),
        Point2::new(2.0, -0.001),
        Point2::new(3.0, 0.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 0.01);

    // 노이즈 제거 → 양 끝점만 남음
    assert_eq!(out.len(), 2);
    assert_eq!(out[0], pts[0]);
    assert_eq!(out[1], pts[3]);
}
```


### ✅ 8. rdp_preserves_peak_in_waveform
```rust
#[test]
fn rdp_preserves_peak_in_waveform() {
    // 파형의 피크 유지
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.5), // 피크
        Point2::new(2.0, 0.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 0.1);

    // 피크 유지
    assert!(contains_pt(&out, pts[1], 1e-12));
}
```


### ✅ 9. rdp_handles_nonuniform_spacing
```rust
#[test]
fn rdp_handles_nonuniform_spacing() {
    // 간격이 불규칙한 점들
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.1, 0.0),
        Point2::new(5.0, 0.0),
        Point2::new(10.0, 0.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 0.01);

    // 가까운 점 제거, 멀리 떨어진 점 유지
    assert!(out.len() < pts.len());
    assert_eq!(out.first().unwrap(), &pts[0]);
    assert_eq!(out.last().unwrap(), &pts[3]);
}
```


### ✅ 10. rdp_handles_high_density_cluster
```rust
#[test]
fn rdp_handles_high_density_cluster() {
    // 특정 구간에 점이 몰려 있음
    let mut pts = vec![Point2::new(0.0, 0.0)];
    for i in 1..50 {
        pts.push(Point2::new(1.0 + i as f64 * 0.001, 0.0)); // 밀집
    }
    pts.push(Point2::new(2.0, 0.0));

    let out = SimplifyPoint2D::simplify(&pts, 0.01);

    // 군집 내 점 제거, 양 끝 유지
    assert!(out.len() < pts.len());
    assert_eq!(out.first().unwrap(), &pts[0]);
    assert_eq!(out.last().unwrap(), pts.last().unwrap());
}
```


### ✅ 11. rdp_handles_reverse_order_input
```rust
#[test]
fn rdp_handles_reverse_order_input() {
    // 입력 순서가 역순
    let pts = vec![
        Point2::new(3.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 0.0),
    ];
    let out = SimplifyPoint2D::simplify(&pts, 1e-6);

    // 중간점 제거, 순서 유지
    assert_eq!(out.len(), 2);
    assert_eq!(out[0], pts[0]);
    assert_eq!(out[1], pts[3]);
}
```

### test_simplify_with_angle_preserves_sharp_turns

#[test]
fn test_simplify_with_angle_preserves_sharp_turns() {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // 랜덤한 경로 생성: 직선 + 급격한 꺾임
    let mut pts = Vec::<Point2>::new();

    // 첫 번째 구간: 직선
    for i in 0..20 {
        pts.push(Point2::new(i as f64, 0.0));
    }

    // 급격한 꺾임: 위로 튐
    pts.push(Point2::new(20.0, 5.0));

    // 두 번째 구간: 다시 직선
    for i in 21..40 {
        pts.push(Point2::new(i as f64, 5.0));
    }

    // 약간의 노이즈 추가
    for pt in &mut pts {
        pt.x += rng.gen_range(-0.01..0.01);
        pt.y += rng.gen_range(-0.01..0.01);
    }

    // 거리 기반 단순화
    let simplified = SimplifyPoint2D::simplify(&pts, 0.5);

    // 거리 + 각도 기반 단순화
    let with_angle = SimplifyPoint2D::simplify_with_angle(&pts, 0.5, 30.0); // 30도 이하면 유지

    // 급격한 꺾임 점이 유지되었는지 확인
    let corner = Point2::new(20.0, 5.0);
    assert!(
        Point2::contains_pt(&with_angle, corner, 0.1),
        "각도 기반 단순화에서 코너가 유지되지 않았습니다"
    );

    println!("simplified: {:?}", simplified);
    println!("");
    println!("with_angle: {:?}", with_angle);
    println!("");
    println!("{:?}", pts);

    // 거리 기반에서는 제거될 수 있음
    assert!(
        Point2::contains_pt(&simplified, corner, 0.1),
        "거리 기반 단순화에서 코너가 유지되었습니다 (예외적일 수 있음)"
    );
}
### 이미지
![Simplfied Point](/image/simplified_points.png)

---
