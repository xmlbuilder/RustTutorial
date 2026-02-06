# Basis Index Range
- 결론 먼저:
- find_span(u) → 반환하는 i는 **해당 u가 속한 Knot Interval U[i], U[i+1])의 왼쪽 인덱스** 이다.
- 그리고
  - 그 interval에서 영향을 받는 Basis Function의 개수는 p+1개이다.
- 즉:
  - 영향받는 basis index 범위 = i − p  …  i
  - 총 개수 = p+1

## 📌 1. find_span(u)의 정확한 정의 (Piegl & Tiller 표준)
- NURBS 책(Piegl & Tiller)의 find_span 알고리즘은 이렇게 정의됨:
- i = find_span(n, p, u, U)
- i는 u가 속한 knot interval
```math
U[i]\leq u<U[i+1]
```
- 의 왼쪽 인덱스.

- 즉, find_span이 찾는 건:
```math
U[i] <= u < U[i+1]
```

- 이 구간의 왼쪽 index i.

## 📌 2. 그럼 Basis Function은 몇 개가 영향을 주나?
NURBS의 p차 B-spline basis는 항상 p+1개가 non-zero.
영향받는 basis index는:
i - p,  i - p + 1,  ...,  i


총 p+1개.

📌 3. 예시로 보면 더 명확해짐
- 예를 들어:
  - degree p = 3 (cubic)
  - find_span(u) = i = 7
- 그러면 영향을 주는 basis index는:
```
7 - 3 = 4
5
6
7
```
- 즉, 4개 (p+1).

## 📌 4. 정리표
| 항목                     | 내용                                               |
|--------------------------|----------------------------------------------------|
| find_span(u) 반환값      | u가 속한 구간 [U[i], U[i+1]) 의 왼쪽 index i       |
| 영향받는 basis 개수      | p+1                                                |
| 영향받는 basis index 범위 | i - p  …  i                                         |
| 안전 조건                | p ≤ i ≤ n  (즉, i-p ≥ 0 이고 i ≤ n)                 |


## 📌 5. Rust 코드에서 보통 이렇게 씀
```rust
let span = find_span(u);
for j in 0..=p {
    let idx = span - p + j;
    // basis[idx] 사용
}
```


## 🌟 핵심 요약
- find_span이 주는 i는 “interval index”이고,  
  실제 basis function은 항상 p+1개가 영향을 준다.
- index 범위는 i−p … i.
---
## 소스 코드
```rust
pub fn on_find_span_left_right(kv: &KnotVector, p: Degree, u: Param, side: Side) -> Result<Index> {
    on_ensure_param_in_knot_domain(kv, u)?;
    let m = kv.len() - 1;
    let p = p as usize;

    // n = m - p - 1 (basis index max)
    if m < p + 1 {
        return Err(NurbsError::InvalidArgument {
            msg: "knot vector too short".into(),
        });
    }
    let _n = m - p - 1;

    // Clamp-to-end rules (C/Piegl 스타일)
    // LEFT: u==U[n+1] -> last span (n)
    // RIGHT: u==U[p] -> first span (p)
    match side {
        Side::Left => Ok(on_find_span_left_eval(kv, p as Degree, u)?),
        Side::Right => Ok(on_find_span_right_eval(kv, p as Degree, u)?),
    }
}
```
```rust
pub fn on_find_span_left_slice(knots: &[Real], p: usize, u: Real) -> Result<usize> {
    let m = knots.len() - 1;
    if m < p + 1 {
        return Err(NurbsError::InvalidInput {
            msg: "Invalid Knot".into(),
        });
    }
    let n = m - p - 1;

    if on_are_equal(u, knots[n + 1], ON_TOL12) {
        return Ok(n);
    }
    if on_are_equal(u, knots[p], ON_TOL12) {
        return Ok(p);
    }

    let mut low = p;
    let mut high = n + 1;
    let mut mid = (low + high) / 2;

    let mut guard = 0usize;
    while u < knots[mid] || u >= knots[mid + 1] {
        guard += 1;
        if guard > 128 {
            return Err(NurbsError::EmptyKnots);
        }

        if u < knots[mid] {
            high = mid;
        } else {
            low = mid;
        }

        if high == low + 1 {
            mid = low;
            break;
        }
        mid = (low + high) / 2;
    }
    Ok(mid)
}
```
```rust
pub fn on_find_span_left_eval(kv: &KnotVector, p: Degree, u: Param) -> Result<Index> {
    on_ensure_param_in_knot_domain(kv, u)?;
    Ok(on_find_span_left_slice(kv.as_slice(), p as usize, u)? as Index)
}
```
---
## NURBS Knot Vector 정리
- index든 갯수이든 하나로 통일해야 한다

| 항목                       | 내용                                                         |
|----------------------------|--------------------------------------------------------------|
| Control point 개수         | n + 1                                                        |
| Degree                     | p                                                            |
| Knot vector 길이           | n + p + 1                                                    |
| Knot index 범위            | 0 … (n + p)                                                  |
| find_span(u) 반환값        | u ∈ [U[i], U[i+1]) 의 왼쪽 index i                           |
| 영향받는 basis 개수        | p + 1                                                        |
| 영향받는 basis index 범위  | i - p … i                                                    |
| 안전 조건                  | p ≤ i ≤ n  (즉, i - p ≥ 0 이고 i ≤ n)                         |

---


