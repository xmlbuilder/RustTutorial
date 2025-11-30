# Subdivision

## 1D Subdivision Algorithms (subdivide & subdivide_adaptive)

### Overview

This document describes two 1‑dimensional subdivision algorithms used to  
generate mesh points along a line segment or polyline:

- 1.  **subdivide** -- metric‑based linear subdivision
- 2.  **subdivide_adaptive** -- target‑aware exponential subdivision with  
    smooth start/end grading

Both methods generate: 
- New point positions along the parametric axis `pos` 
- Corresponding metric values at each new point `metrics` 
- Resulting index list representing subdivided nodes `indices`

### 1. subdivide - Linear Metric-Based Subdivision

#### Inputs

-   `pos[i]` --- coordinate of ith node
-   `metrics[i]` --- desired element size at ith node
-   `nodes[k]` --- index list defining a polyline
-   `force_up` --- round‑up subdivision count
-   `force_even` --- force even total element count
-   `min_n` --- minimum edges per segment
-   `max_n` --- maximum edges per segment
-   `indices` --- output reordered node indices

#### Purpose
Divide each polyline segment proportionally to its metric value, 
- ensuring: 
    - No segment has fewer than `min_n` edges 
    - No segment has more than `max_n` edges 
    - Global evenness when requested

#### Ideal Edge Count Formula

- For each segment:

```
    length = abs(x1 - x0)
    m_avg  = (m0 + m1) * 0.5

    ideal_edges = length / m_avg

    if ideal_edges < 1 => ideal_edges = 1
    if force_up       => edges = ceil(ideal_edges)
    else               => edges = round(ideal_edges)

    edges = clamp(edges, min_n, max_n)
```

#### New Points

Interior new points computed via linear interpolation:
```
    t = k / edges
    x_new = (1 - t)*x0 + t*x1
    m_new = (1 - t)*m0 + t*m1
```

---

## 2. subdivide_adaptive - Smooth Target-Based Adaptive Subdivision

### New Concept

Instead of linear interpolation of metric values, we construct a **smooth bell‑shaped metric curve**:

-   Start size → increase smoothly
-   Reach (or attempt to reach) a target size
-   Maintain plateau if possible
-   Decrease smoothly to end size
-   Never exceed target
-   Reduce total node count as much as possible

### Inputs
- start_size  = metrics[nodes[0]]
- end_size    = metrics[nodes.back()]
- target_size = maximum desired metric
- length      = pos[end] - pos[start]
- edges_min   = minimal acceptable subdivisions
- edges_max   = maximal acceptable subdivisions

### Parameterization

Normalize coordinate:

```
u = (x - x_start) / length; 0 <= u <= 1
```
### Shape Functions

We define three regions:

- 1.  **Growth region** (`u < u_peak`)
- 2.  **Plateau region** (`u_peak <= u <= 1 - u_peak`)
- 3.  **Decay region**

- A smooth exponential-like growth is used:
```
g(u) = start_size + (target_size - start_size) *
       (1 - exp(-k * u)) / (1 - exp(-k))
```

- Similarly, decay is symmetric:
```
d(u) = end_size + (target_size - end_size) *
       (1 - exp(-k * (1 - u))) / (1 - exp(-k))
```

- Final metric profile:
```
if u < u_peak      => m(u) = g(u)
if u_peak < u < 1-u_peak => m(u) = target_size
else               => m(u) = d(u)
```

- `u_peak` is determined by checking if the start  
    → target and target  
    → end growth curves meet without crossing.  

### Ideal Edge Count

- Total edge count is determined using average metric:
```
avg_m = integral(m(u) du) / 1.0

ideal_edges = length / avg_m
ideal_edges = clamp(round(ideal_edges), edges_min, edges_max)
```

### Node Placement
- Nodes are placed so that element size approximates `m(u)`:
```
x_next = x_prev + m(u_prev)
repeat until x reaches end
```

---

## Example Metric Profiles

### Linear (subdivide)

- Smooth linear interpolation of metric values.

### Adaptive (subdivide_adaptive)

- Smooth exponential bell curve reaching target region only when possible.

---

## Summary Table

| Feature               | subdivide       | subdivide_adaptive |
|----------------------|----------------|--------------------|
| Metric interpolation | Linear         |  Exponential + Plateau |
| Target size limit    |  No            |   Yes                  |
| Mesh count reduction |  Minimal       |   Strong               |
| Smoothness           |  Moderate      |   Very high            |
| Good for             |  Simple grading |   Boundary layers, curved features |

---

## Output

- Both functions return:
```
ret >= 0  => number of generated edges
ret < 0   => error code
```
- And output updated:
```
pos[]      ; x-coordinates
metrics[]  ; metric values at new nodes
indices[]  ; ordering of subdivided nodes
```
---

## 소스 코드
```rust
pub type DoubleVec = Vec<f64>;
pub type UIntVec = Vec<usize>;

/// Subdivides a 1-D line (or polyline).
///
/// * `pos`      : 좌표 벡터 (1D 파라미터, 예: U). 새 점이 push 됩니다.
/// * `metrics`  : 각 노드에서의 "목표 edge 길이" (size). 새 점의 metric도 push 됩니다.
/// * `nodes`    : 이 1D 라인(또는 polyline)을 정의하는 노드 인덱스 열.
/// * `force_up` : 이상적인 edge 개수를 올림(ceil)할지 여부 (false면 round).
/// * `force_even`: 생성된 전체 edge 수를 짝수로 맞추려고 시도.
/// * `min_n`    : 전체 라인에서 최소 edge 개수.
/// * `max_n`    : 전체 라인에서 최대 edge 개수.
/// * `indices`  : subdivide 후, 라인 위의 노드 인덱스들을 좌표 증가 순으로 반환.
///
/// Return:
/// * >= 0 : 생성된 edge 개수
/// * <  0 : 에러. -k => k번째 인자가 잘못됨(대략적인 규칙)
///
/// 주의:
/// - 기존 pos / metrics 순서는 유지, 새 점은 push_back.
/// - nodes 는 pos/metrics 의 인덱스로 가정.
pub fn subdivide(
    pos: &mut DoubleVec,
    metrics: &mut DoubleVec,
    nodes: &UIntVec,
    force_up: bool,
    force_even: bool,
    min_n: u32,
    max_n: u32,
    indices: &mut UIntVec,
) -> i32 {
    // --- 인자 검증 ---
    // 1: pos, 2: metrics, 3: nodes, 4: force_up, 5: force_even, 6: min_n, 7: max_n, 8: indices

    // (1) metrics > 0 검사
    if metrics.iter().any(|&m| !(m > 0.0)) {
        return -2; // 2번째 인자(metrics) 오류
    }

    // (2) min_n, max_n 검사
    if min_n == 0 {
        return -6; // 6번째 인자(min_n) 오류
    }
    if min_n > max_n {
        return -7; // 7번째 인자(max_n) 오류
    }

    // (3) nodes 검사
    if nodes.len() < 2 {
        return -3; // 3번째 인자(nodes) 오류
    }

    let n_pos = pos.len();
    let n_met = metrics.len();
    if nodes.iter().any(|&idx| idx >= n_pos || idx >= n_met) {
        return -3;
    }

    indices.clear();
    if pos.is_empty() {
        return 0;
    }

    // polyline 상 segment 개수
    let seg_count = if nodes.len() >= 2 { nodes.len() - 1 } else { 0 };
    if seg_count == 0 {
        return 0;
    }

    // 각 segment 별 edge 개수 결정
    let mut seg_edges: Vec<u32> = vec![1; seg_count];
    let mut total_edges: u32 = 0;

    for si in 0..seg_count {
        let i0 = nodes[si];
        let i1 = nodes[si + 1];

        let x0 = pos[i0];
        let x1 = pos[i1];
        let l = (x1 - x0).abs();

        let m0 = metrics[i0];
        let m1 = metrics[i1];

        // 평균 metric
        let mut m_avg = 0.5 * (m0 + m1);
        if !(m_avg > 0.0) {
            if m0 > 0.0 {
                m_avg = m0;
            } else if m1 > 0.0 {
                m_avg = m1;
            } else {
                m_avg = 1.0;
            }
        }

        // 이상적인 edge 개수
        let mut n_real = l / m_avg;
        if n_real < 1.0 {
            n_real = 1.0;
        }

        let n_round = if force_up {
            n_real.ceil()
        } else {
            n_real.round()
        };

        let mut n = if n_round < 1.0 { 1 } else { n_round as u32 };

        // min/max 보정
        if n < min_n {
            n = min_n;
        }
        if n > max_n {
            n = max_n;
        }

        seg_edges[si] = n;
        total_edges += n;
    }

    // force_even: 전체 edge 수를 짝수로 맞추려고 시도
    if force_even && (total_edges % 2) != 0 {
        if let Some(last) = seg_edges.last_mut() {
            let n_last = *last;
            // +1 가능한 경우
            if n_last + 1 <= max_n {
                *last = n_last + 1;
                total_edges += 1;
            } else if n_last > min_n {
                // -1 가능한 경우
                *last = n_last - 1;
                total_edges -= 1;
            }
            // 그래도 홀수면 그냥 놔둠.
        }
    }

    // 실제 점 생성 + indices 채우기
    // polyline 순서: nodes[0], (seg0 내부 점들), nodes[1], (seg1 내부 점들), ..., nodes.last()
    indices.reserve(nodes.len() + total_edges as usize);
    indices.push(nodes[0]);

    for si in 0..seg_count {
        let i0 = nodes[si];
        let i1 = nodes[si + 1];

        let x0 = pos[i0];
        let x1 = pos[i1];

        let m0 = metrics[i0];
        let m1 = metrics[i1];

        let n = seg_edges[si];

        if n > 1 {
            // n개의 edge → interior node 는 n-1개
            for k in 1..n {
                let t = (k as f64) / (n as f64); // 0..1
                let x = x0 + t * (x1 - x0);
                let m = (1.0 - t) * m0 + t * m1;

                pos.push(x);
                metrics.push(m);

                let new_index = pos.len() - 1;
                indices.push(new_index);
            }
        }

        // segment 끝 노드도 넣어 줌
        indices.push(i1);
    }

    // post:
    // indices.first() == Some(&nodes[0])
    // indices.last()  == Some(&nodes.last().unwrap())

    total_edges as i32
}
```
```rust
/// 0 <= t <= 1 에서 0 → 1 → 0 형태의 부드러운 bump.
/// 가운데(t=0.5)에서 최대 1, 양 끝에서는 0.
/// exponent 를 2로 줘서 초반 기울기를 완만하게 만든다.
fn smooth_bump01(t: f64) -> f64 {
    if t <= 0.0 || t >= 1.0 {
        return 0.0;
    }
    // 기본 parabola: 4 t (1 - t) ∈ [0,1]
    let b = 4.0 * t * (1.0 - t);
    // 더 완만하게 만들기 위해 제곱
    let bump = b * b; // exponent = 2
    if bump > 1.0 { 1.0 } else { bump }
}
```
```rust
/// adaptive 버전:
/// - edge 개수 결정은 subdivide 와 동일하게 m_avg 기반.
/// - target_size 는 metric 상한(도달하면 좋은 값).
/// - 내부 노드에서: 끝점 선형보간 + 중앙에서만 smooth bump 로 target 쪽으로 살짝 끌어올림.
///   → 길이 충분하면 가운데서 target 근처까지, 짧으면 그보다 아래에서 봉우리 찍고 내려감.
/// adaptive 버전:
/// - target_size: "도달하면 좋은" 최대 edge 길이 (상한), 어디서도 이 값을 넘지 않음.
/// - start/end 는 보통 target_size 보다 작다고 가정 (fine → coarse → fine).
/// - edge 개수는 m_avg 와 target_size 를 기하 보간해서 결정 (메쉬 폭발 방지).
/// - 내부 노드 metric 은 끝점 선형보간 + 중앙에서 smooth bump 로 target 쪽으로 살짝 끌어올린 bell 모양.
pub fn subdivide_adaptive(
    pos: &mut DoubleVec,
    metrics: &mut DoubleVec,
    nodes: &UIntVec,
    target_size: f64,
    force_up: bool,
    force_even: bool,
    min_n: u32,
    max_n: u32,
    indices: &mut UIntVec,
) -> i32 {
    // --- 인자 검증 ---
    if !(target_size > 0.0) {
        return -9; // 4: target_size 오류 (새 인자 기준이면 다를 수 있음)
    }
    if metrics.iter().any(|&m| !(m > 0.0)) {
        return -2; // metrics 오류
    }
    if min_n == 0 {
        return -6;
    }
    if min_n > max_n {
        return -7;
    }
    if nodes.len() < 2 {
        return -3;
    }

    let n_pos = pos.len();
    let n_met = metrics.len();
    if nodes.iter().any(|&idx| idx >= n_pos || idx >= n_met) {
        return -3;
    }

    indices.clear();
    if pos.is_empty() {
        return 0;
    }

    let seg_count = nodes.len() - 1;
    if seg_count == 0 {
        return 0;
    }

    // coarsening 정도 (0 = 원래 subdivide, 1 = 완전히 target만 사용)
    let alpha_coarsen = 0.5_f64; // 필요하면 0.3~0.7 사이로 튜닝 가능

    // --- 세그먼트별 edge 개수 계산 ---
    let mut seg_edges: Vec<u32> = vec![1; seg_count];
    let mut total_edges: u32 = 0;

    for si in 0..seg_count {
        let i0 = nodes[si];
        let i1 = nodes[si + 1];

        let x0 = pos[i0];
        let x1 = pos[i1];
        let l = (x1 - x0).abs();

        let m0 = metrics[i0];
        let m1 = metrics[i1];

        // 끝점 평균 크기 (fine 기준)
        let mut m_avg = 0.5 * (m0 + m1);
        if !(m_avg > 0.0) {
            if m0 > 0.0 {
                m_avg = m0;
            } else if m1 > 0.0 {
                m_avg = m1;
            } else {
                m_avg = 1.0;
            }
        }

        // target_size 와 결합해서 "유효 size" 계산
        // m_eff = m_avg^(1-alpha) * target^alpha
        let m_eff = if alpha_coarsen <= 0.0 {
            m_avg
        } else if alpha_coarsen >= 1.0 {
            target_size
        } else {
            let a = alpha_coarsen;
            m_avg.powf(1.0 - a) * target_size.powf(a)
        };

        let mut n_real = l / m_eff;
        if n_real < 1.0 {
            n_real = 1.0;
        }

        let n_round = if force_up {
            n_real.ceil()
        } else {
            n_real.round()
        };

        let mut n = if n_round < 1.0 { 1 } else { n_round as u32 };

        if n < min_n {
            n = min_n;
        }
        if n > max_n {
            n = max_n;
        }

        seg_edges[si] = n;
        total_edges += n;
    }

    // --- force_even: 전체 edge 수 짝수로 맞추기 ---
    if force_even && (total_edges % 2) != 0 {
        if let Some(last) = seg_edges.last_mut() {
            let n_last = *last;
            if n_last + 1 <= max_n {
                *last = n_last + 1;
                total_edges += 1;
            } else if n_last > min_n {
                *last = n_last - 1;
                total_edges -= 1;
            }
        }
    }

    // --- 실제 점 생성 ---
    indices.reserve(nodes.len() + total_edges as usize);
    indices.push(nodes[0]);

    for si in 0..seg_count {
        let i0 = nodes[si];
        let i1 = nodes[si + 1];

        let x0 = pos[i0];
        let x1 = pos[i1];

        let m0 = metrics[i0];
        let m1 = metrics[i1];

        let n = seg_edges[si];

        if n > 1 {
            for k in 1..n {
                let t = (k as f64) / (n as f64); // 0..1

                let x = x0 + t * (x1 - x0);

                // 1) 끝점 기준 선형보간
                let m_lin = (1.0 - t) * m0 + t * m1;

                // 2) 가운데에서만 target 쪽으로 살짝 끌어올리는 bump (0→1→0)
                let bump = smooth_bump01(t);

                // 3) target 쪽으로 이동 (절대 target 넘지 않게)
                let mut m = m_lin + (target_size - m_lin) * bump;
                if m > target_size {
                    m = target_size;
                }

                pos.push(x);
                metrics.push(m);

                let new_index = pos.len() - 1;
                indices.push(new_index);
            }
        }

        indices.push(i1);
    }

    total_edges as i32
}
```
---
 ## 샘플 소스
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::mesh_tools::mesh_tools_1d::{DoubleVec, UIntVec, subdivide, subdivide_adaptive};

    // 편하게 호출하기 위한 헬퍼
    fn run_subdivide(
        mut pos: DoubleVec,
        mut metrics: DoubleVec,
        nodes: UIntVec,
        force_up: bool,
        force_even: bool,
        min_n: u32,
        max_n: u32,
    ) -> (i32, DoubleVec, DoubleVec, UIntVec) {
        let mut indices: UIntVec = Vec::new();
        let ret = subdivide(
            &mut pos,
            &mut metrics,
            &nodes,
            force_up,
            force_even,
            min_n,
            max_n,
            &mut indices,
        );
        (ret, pos, metrics, indices)
    }
```
```rust
    /// [기본 케이스]
    /// 0..1 구간, metric=0.25 이면 이상 edge 개수는 4개.
    #[test]
    fn subdivide_single_segment_uniform_metric() {
        let pos = vec![0.0, 1.0];
        let metrics = vec![0.25, 0.25];
        let nodes = vec![0_usize, 1_usize];

        let (ret, pos2, metrics2, indices) =
            run_subdivide(pos, metrics, nodes, false, false, 1, 100);

        // edge 개수 4개 기대
        assert_eq!(ret, 4);

        // 새 포인트는 뒤에 추가됨 → 전체 길이 5 (= 기존 2 + 내부 3)
        assert_eq!(pos2.len(), 5);
        assert_eq!(metrics2.len(), 5);

        // indices 는 polyline 순서대로: 0 -> (내부) -> 1
        // 실제 좌표로 확인해 봄
        let xs: Vec<f64> = indices.iter().map(|&i| pos2[i]).collect();
        let expected = vec![0.0, 0.25, 0.5, 0.75, 1.0];

        assert_eq!(xs.len(), expected.len());
        for (x, e) in xs.iter().zip(expected.iter()) {
            assert!((x - e).abs() < 1e-12, "x = {x}, expected = {e}");
        }

        // metric 도 선형 보간이 제대로 되었는지 대략 확인
        let ms: Vec<f64> = indices.iter().map(|&i| metrics2[i]).collect();
        for m in ms {
            assert!((m - 0.25).abs() < 1e-12);
        }
    }
```
```rust
    /// [멀티 세그먼트]
    /// 0-1-2, metric=0.5 → 각 세그먼트당 edge 2개씩, 전체 4개.
    #[test]
    fn subdivide_two_segments_uniform_metric() {
        let pos = vec![0.0, 1.0, 2.0];
        let metrics = vec![0.5, 0.5, 0.5];
        let nodes = vec![0_usize, 1_usize, 2_usize];

        let (ret, pos2, _metrics2, indices) =
            run_subdivide(pos, metrics, nodes, false, false, 1, 100);

        // edge 개수 4개 기대 (세그먼트당 2개씩)
        assert_eq!(ret, 4);

        // 새 포인트는 각 세그먼트마다 1개씩 추가 → 기존 3 + 2 = 5
        assert_eq!(pos2.len(), 5);

        // indices 로 좌표 확인
        let xs: Vec<f64> = indices.iter().map(|&i| pos2[i]).collect();
        let expected = vec![0.0, 0.5, 1.0, 1.5, 2.0];

        assert_eq!(xs.len(), expected.len());
        for (x, e) in xs.iter().zip(expected.iter()) {
            assert!((x - e).abs() < 1e-12, "x = {x}, expected = {e}");
        }
    }
```
```rust
    /// [force_even 테스트]
    /// 한 세그먼트에서 이상 edge 개수가 3개가 되도록 metric 설정.
    /// force_even=true 이면 마지막 세그먼트 edge 수를 조정해서 전체를 짝수로 맞추려고 시도.
    #[test]
    fn subdivide_force_even_adjust_last_segment() {
        // 길이 L = 1, m_avg = 0.4 → n_real = 2.5, round → 3
        let pos = vec![0.0, 1.0];
        let metrics = vec![0.4, 0.4];
        let nodes = vec![0_usize, 1_usize];

        let (ret, _pos2, _metrics2, _indices) =
            run_subdivide(pos, metrics, nodes, false, true, 1, 10);

        // 원래라면 n=3이지만 force_even 때문에 4로 조정되어야 함
        assert_eq!(ret, 4);
    }
```
```rust
    /// [에러 케이스: metric <= 0]
    #[test]
    fn subdivide_error_metrics_non_positive() {
        let pos = vec![0.0, 1.0];
        let metrics = vec![0.25, 0.0]; // 잘못된 metric
        let nodes = vec![0_usize, 1_usize];

        let (ret, _pos2, _metrics2, _indices) =
            run_subdivide(pos, metrics, nodes, false, false, 1, 10);

        assert_eq!(ret, -2); // 2번째 인자 오류
    }
```
```rust
    /// [에러 케이스: min_n == 0]
    #[test]
    fn subdivide_error_min_n_zero() {
        let pos = vec![0.0, 1.0];
        let metrics = vec![0.25, 0.25];
        let nodes = vec![0_usize, 1_usize];

        let (ret, pos2, _metrics2, indices) =
            run_subdivide(pos, metrics, nodes, false, false, 0, 10);


        println!("{:?}", pos2);

        assert_eq!(ret, -6);
    }
```
```rust
    /// [에러 케이스: min_n > max_n]
    #[test]
    fn subdivide_error_min_n_gt_max_n() {
        let pos = vec![0.0, 1.0];
        let metrics = vec![0.25, 0.25];
        let nodes = vec![0_usize, 1_usize];

        let (ret, _pos2, _metrics2, _indices) =
            run_subdivide(pos, metrics, nodes, false, false, 5, 3);

        assert_eq!(ret, -7);
    }
```
```rust
    /// [에러 케이스: nodes 길이 < 2]
    #[test]
    fn subdivide_error_nodes_too_short() {
        let pos = vec![0.0];
        let metrics = vec![0.25];
        let nodes = vec![0_usize]; // 한 개뿐

        let (ret, _pos2, _metrics2, _indices) =
            run_subdivide(pos, metrics, nodes, false, false, 1, 10);

        assert_eq!(ret, -3);
    }
```
```rust
    /// [에러 케이스: nodes 인덱스 범위 초과]
    #[test]
    fn subdivide_error_nodes_out_of_range() {
        let pos = vec![0.0, 1.0];
        let metrics = vec![0.25, 0.25];
        let nodes = vec![0_usize, 2_usize]; // 2는 out-of-range

        let (ret, _pos2, _metrics2, _indices) =
            run_subdivide(pos, metrics, nodes, false, false, 1, 10);

        assert_eq!(ret, -3);
    }
```
```rust
    #[test]
    fn subdivide_edge_10_metric_1_to_0_5() {
        // 초기 데이터
        let pos = vec![0.0, 10.0];
        let metrics = vec![1.0, 0.5];
        let nodes = vec![0_usize, 1_usize];

        let mut pos2 = pos.clone();
        let mut met2 = metrics.clone();
        let mut indices: UIntVec = Vec::new();

        let ret = subdivide(
            &mut pos2,
            &mut met2,
            &nodes,
            false,  // force_up = false (round)
            false,  // force_even = false
            1,      // min_n
            100,    // max_n
            &mut indices,
        );

        // 1) edge 개수 확인
        assert_eq!(ret, 13);

        // 2) 새로 생성된 전체 포인트 수 = 기존 2 + 내부 12 = 14
        assert_eq!(pos2.len(), 14);
        assert_eq!(met2.len(), 14);

        println!("pos parameter {:?}", pos2);

        // 3) indices 는 0 -> 내부 12점 -> 1 순서로 14개 인덱스를 가져야 함
        assert_eq!(indices.len(), 14);
        assert_eq!(indices.first(), Some(&0));
        assert_eq!(indices.last(), Some(&1));

        // 4) 좌표/metric 값이 이론값과 맞는지 확인
        //    (indices 순서대로 좌표/metric 체크)
        let expected_x: Vec<f64> = (0..=13)
            .map(|k| 10.0 * (k as f64) / 13.0)
            .collect();
        let expected_m: Vec<f64> = (0..=13)
            .map(|k| {
                let t = (k as f64) / 13.0;
                1.0 - 0.5 * t
            })
            .collect();

        let eps = 1e-12;
        for (p_idx, (&idx, k)) in indices.iter().zip(0..=13).enumerate() {
            let x = pos2[idx];
            let m = met2[idx];
            assert!(
                (x - expected_x[k]).abs() < eps,
                "x mismatch at p_idx={p_idx}, idx={idx}, got={x}, expected={}",
                expected_x[k]
            );
            assert!(
                (m - expected_m[k]).abs() < eps,
                "m mismatch at p_idx={p_idx}, idx={idx}, got={m}, expected={}",
                expected_m[k]
            );
        }
    }
```
```rust
    #[test]
    fn test_subdivide_adaptive_edge_10() {
        let mut pos = vec![0.0, 10.0];
        let mut metrics = vec![0.1, 0.5];
        let nodes = vec![0_usize, 1_usize];
        let mut indices = Vec::new();

        let ret = subdivide_adaptive(
            &mut pos,
            &mut metrics,
            &nodes,
            1.0,   // target_size
            false, // force_up
            false, // force_even
            1,
            100,
            &mut indices,
        );

        println!("ret edges = {ret}");
        for (pi, &idx) in indices.iter().enumerate() {
            println!(
                "p[{pi}] : x = {:8.4}, m = {:8.4}",
                pos[idx], metrics[idx]
            );
        }
    }
}
```

## Subdivide Adaptive 결과
!(SubDivide Adaptive)(/image/subdivde_adaptive.png)


---



