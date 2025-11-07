
## ✅ 수식 및 알고리즘 검토
### 1. distance_closest_point_to
- 선분에 대해 pt3에서 가장 가까운 점을 구하고, 그 거리 반환
- 내부적으로 Segment2D::closest_param_to와 point_at(t) 사용
- 수식적으로 정확함:

$$
d=\| \mathbf{p_{\mathnormal{3}}}-\mathbf{b}\| ,\quad \mathbf{b}=\mathrm{projection\  of\  }\mathbf{p_{\mathnormal{3}}}\mathrm{\  onto\  segment\  }\mathbf{p_{\mathnormal{1}}p_{\mathnormal{2}}}
$$

### 2. square_distance
- 단순한 유클리드 거리 제곱 계산:

$$
d^2=(x_2-x_1)^2+(y_2-y_1)^2
$$

- 정확함

### 3. simplify_recursive
- RDP 알고리즘의 핵심:
- 선분 시작–끝 기준으로 가장 먼 점 찾기
- 그 점이 거리 기준 이상이면 재귀 분할
- 정확한 알고리즘 흐름

### 4. simplify_internal
- 폐합된 폴리라인을 처리하기 위해:
- 시작점에서 가장 먼 점을 기준으로 양쪽 분할
- 각각 단순화 후 병합
- RDP 알고리즘의 폐합 확장 버전으로 적절함

### 5. simplify
- 폐합 여부 판단: points[0] == points[n - 1]
- 개방형일 경우 끝점 반복 제거
- 재귀 시드 설정 후 단순화 수행
- 전체 흐름이 안정적이고 정확함

## ✅ 개선 포인트
- simplify_internal에서 가장 먼 점 기준 분할은 RDP의 폐합 확장으로 괜찮지만,  
    곡선이 매우 비대칭일 경우 성능이 떨어질 수 있음 → 선택적 개선 가능

## ✅ 결론
- 수식적으로 문제 없음
- 알고리즘 흐름 정확함
- 폐합/개방 폴리라인 모두 안정적으로 처리
- RDP 알고리즘을 잘 반영한 구조

---
## 소스 코드
```rust
use crate::core::geom::Point2;
use crate::core::segment2d::Segment2D;

#[derive(Debug, Default)]
pub struct SimplifyPoint2;
```
```rust
impl SimplifyPoint2 {
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
}
```

### 테스트 코드


