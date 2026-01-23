## on_insert_value_into_sorted_array_option

- 이 함수는 **정렬된 f64 배열에 값을 삽입하되, 범위 밖이면 None** 을 반환하는  
    안전한 삽입 함수

### 📘 on_insert_value_into_sorted_array_option
- 정렬된 Vec<f64>에 value를 삽입하되, 범위 밖이면 None 반환
```rust
pub fn on_insert_value_into_sorted_array_option(mut v: Vec<f64>, value: f64) 
    -> Option<Vec<f64>> {
    if v.is_empty() {
        return None;
    }
    if value < v[0] || value > *v.last().unwrap() {
        return None;
    }
    v.push(value); // 확장
    let mut i = v.len() - 1;
    while i > 0 && v[i - 1] > value {
        v[i] = v[i - 1];
        i -= 1;
    }
    v[i] = value;
    Some(v)
}
```

### 1️⃣ 동작 방식 요약
- 빈 벡터면 삽입 불가 → None
- value가 정렬 범위 밖이면 → None
- 즉, v[0] ≤ value ≤ v[last]일 때만 삽입
- push 후 삽입 위치까지 뒤에서 앞으로 shift
- value를 정확한 위치에 배치
- Some(v) 반환
- 이건 사실상 binary search 없이 insertion sort의 한 스텝을 구현한 형태.

### 2️⃣ 시간 복잡도
- 최악: O(n) (shift 때문에)
- 하지만 단일 삽입에서는 충분히 빠르고, Vec 기반에서는 흔히 쓰는 패턴

### 3️⃣ 장점
- 정렬 유지 보장
- 범위 체크로 불필요한 삽입 방지
- Vec 재할당 없이 push → shift → write
- Option 반환으로 실패/성공 명확

4️⃣ 개선 포인트(선택)
- ✔ binary search로 삽입 위치를 찾을 수 있음
- 지금도 충분히 빠르지만, 더 깔끔하게 만들려면:
```rust
let pos = v.binary_search_by(|x| x.total_cmp(&value)).unwrap_or_else(|p| p);
```
```rust
pub fn insert_sorted_option(mut v: Vec<f64>, value: f64) -> Option<Vec<f64>> {
    if v.is_empty() {
        return None;
    }
    if value < v[0] || value > v[v.len() - 1] {
        return None;
    }

    match v.binary_search_by(|x| x.partial_cmp(&value).unwrap()) {
        Ok(_) => Some(v), // 이미 존재하면 그대로 반환
        Err(idx) => {
            v.insert(idx, value);
            Some(v)
        }
    }
}
```
- 이렇게 하면 shift만 하면 되고, while 루프가 사라진다.

### 📌 최종 요약
- 이 함수는:
    - 정렬된 f64 배열에 값을 삽입
    - 범위 밖이면 None
    - 내부적으로 insertion-sort-style shift
    - 안정적이고 명확한 구현


```rust
pub fn on_insert_value_into_sorted_array_option(mut v: Vec<f64>, value: f64) -> Option<Vec<f64>> {
    if v.is_empty() {
        return None;
    }
    if value < v[0] || value > *v.last().unwrap() {
        return None;
    }
    v.push(value); // 확장
    let mut i = v.len() - 1;
    while i > 0 && v[i - 1] > value {
        v[i] = v[i - 1];
        i -= 1;
    }
    v[i] = value;
    Some(v)
}
```

## on_closest_points_of_3d_lines

- 이 함수는 **두 3D 직선의 최근접점(closest points)** 을 구하는 가장 정석적이고  
    안정적인 구현.

### 📘 on_closest_points_of_3d_lines
- 두 직선 L1(s) = p1 + s·d1, L2(t) = p2 + t·d2 의 최근접점 ps, qt 계산
```rust
pub fn on_closest_points_of_3d_lines(
    p1: Vector3D,
    d1: Vector3D,
    p2: Vector3D,
    d2: Vector3D,
) -> Result<(f64, f64, Vector3D, Vector3D), &'static str> {
    let a = d1.dot(&d1);
    let b = d1.dot(&d2);
    let c = d2.dot(&d2);
    let w0 = p1 - p2;
    let d = d1.dot(&w0);
    let e = d2.dot(&w0);

    let denom = a * c - b * b;
    if denom.abs() < 1e-30 {
        return Err("parallel or nearly parallel");
    }
    let s = (b * e - c * d) / denom;
    let t = (a * e - b * d) / denom;

    let ps = p1 + d1.scale(s);
    let qt = p2 + d2.scale(t);
    Ok((s, t, ps, qt))
}
```


### 1️⃣ 수학적 의미
- 두 직선:
    - L1(s) = p1 + s·d1
    - L2(t) = p2 + t·d2
- 최근접점 조건은 다음 선형 시스템을 푸는 것과 같다:
```math
\begin{aligned}(d_1\cdot d_1)s-(d_1\cdot d_2)t&=d_1\cdot (p_1-p_2)\\ -(d_1\cdot d_2)s+(d_2\cdot d_2)t&=d_2\cdot (p_1-p_2)\end{aligned}
```
- 이를 행렬로 쓰면:
```math
\left[ \begin{matrix}a&-b\\ -b&c\end{matrix}\right] \left[ \begin{matrix}s\\ t\end{matrix}\right] =\left[ \begin{matrix}d\\ e\end{matrix}\right]
``` 
- 해는:
```math
s=\frac{be-cd}{ac-b^2},\quad t=\frac{ae-bd}{ac-b^2}
```
- 코드가 정확히 이 공식을 구현하고 있다.

### 2️⃣ 평행성 검사
```rust
let denom = a * c - b * b;
if denom.abs() < 1e-30 {
    return Err("parallel or nearly parallel");
}
```

- denom = ac − b²
- denom ≈ 0 → 두 직선이 평행 또는 거의 평행
- 매우 작은 threshold(1e-30)를 사용해 robust하게 감지
- 이건 CAD/Geometry에서 자주 쓰는 안정적인 패턴이다.

### 3️⃣ 최근접점 계산
```rust
let ps = p1 + d1.scale(s);
let qt = p2 + d2.scale(t);
```

- ps = L1(s)
- qt = L2(t)
- skew lines(엇갈린 직선)도 ps ≠ qt 이지만, 두 점이 서로 가장 가까운 점이다.

### 4️⃣ 반환값
- Ok((s, t, ps, qt))
    - s, t: 각 직선에서의 파라미터
    - ps: L1의 최근접점
    - qt: L2의 최근접점
    - Err("parallel or nearly parallel")
    - 두 직선이 평행 또는 거의 평행

### 📌 최종 요약
- 이 함수는:
    - 두 3D 직선의 최근접점을 robust하게 계산
    - 평행성 검사 포함
    - textbook-level 정확한 공식 사용
    - CAD/Geometry 엔진에서 그대로 쓰는 정석 알고리즘

```rust
pub fn on_closest_points_of_3d_lines(
    p1: Vector3D,
    d1: Vector3D,
    p2: Vector3D,
    d2: Vector3D,
) -> Result<(f64, f64, Vector3D, Vector3D), &'static str> {
    let a = d1.dot(&d1);
    let b = d1.dot(&d2);
    let c = d2.dot(&d2);
    let w0 = p1 - p2;
    let d = d1.dot(&w0);
    let e = d2.dot(&w0);

    let denom = a * c - b * b;
    if denom.abs() < 1e-30 {
        return Err("parallel or nearly parallel");
    }
    let s = (b * e - c * d) / denom;
    let t = (a * e - b * d) / denom;

    let ps = p1 + d1.scale(s);
    let qt = p2 + d2.scale(t);
    Ok((s, t, ps, qt))
}
```

## on_calculate_arc_segments

- 이 함수는 호(arc)를 chord length 기준으로 몇 개의 세그먼트로 나눌지  
    계산하는 매우 깔끔한 구현.
- 이미 수학적으로 정확하고, edge case 처리도 잘 되어 있어서 
    실전 CAD/Geometry 코드로 손색이 없다.

### 📘 on_calculate_arc_segments(radius, arc_length, chord_length)
- 주어진 반지름·호길이·허용 chord 길이로 필요한 세그먼트 개수와 각 세그먼트의 호길이를 계산

### 1️⃣ 핵심 공식
- ✔ 최대 세그먼트 각도
    - chord length = c, radius = R 일 때:
```math
\theta _{\max }=2\cdot \arccos \left( \frac{R-c}{R}\right)
``` 
- 이 각도보다 큰 세그먼트는 chord 길이가 c를 초과하게 된다.
- 코드:
```rust
let mut ratio = (radius - chord_length) / radius;
ratio = ratio.clamp(-1.0, 1.0);
let max_seg_angle = 2.0 * ratio.acos();
```


### 2️⃣ 세그먼트 개수 계산
- 전체 호길이 L, 세그먼트 각도 θ_max일 때:
```math
n=\left\lceil \frac{|L|}{\theta _{\max }}\right\rceil
``` 
- 단, 최소 2개는 유지.
- 코드:
```rust
let n = if max_seg_angle > 0.0 {
    ((arc_length.abs() / max_seg_angle).ceil() as i64).max(2) as usize
} else {
    2
};
```


### 3️⃣ 세그먼트별 호길이
```math
\mathrm{segment\_ arc}=\frac{L}{n}
```
- 코드:
```rust
( n, arc_length / (n as f64) )
```

### 4️⃣ edge case 처리
- ✔ radius ≤ 0
- 반지름이 0 또는 음수면 의미가 없으므로 기본값 반환:
```rust
return (2, arc_length / 2.0);
```

- ✔ ratio 범위 제한
    - acos 입력은 반드시 [-1, 1]이어야 하므로 clamp 처리.
- ✔ max_seg_angle → 0
    - 세그먼트 각도가 0에 가까우면 세그먼트 수가 무한히 커지므로 최소 2로 고정.
- ✔ arc_length = 0
    - 결과적으로 n = 2, segment length = 0.

### 📌 최종 요약
- 이 함수는:
    - chord length 기준으로 arc를 몇 개로 나눌지 계산
    - 수학적으로 정확한 공식 사용
    - radius, chord, arc length의 edge case를 robust하게 처리
    - 세그먼트 개수와 각 세그먼트의 호길이를 반환

```rust
pub fn on_calculate_arc_segments(radius: f64, arc_length: f64, chord_length: f64) -> (usize, f64) {
    // Max segment angle = 2 * acos((R - c) / R)
    // Safeguards: zero, negative, or invalid values
    if radius <= 0.0 {
        return (2, arc_length / 2.0);
    }
    let mut ratio = (radius - chord_length) / radius;
    if ratio < -1.0 {
        ratio = -1.0;
    }
    if ratio > 1.0 {
        ratio = 1.0;
    }

    let max_seg_angle = 2.0 * ratio.acos();

    // As max_seg_angle approaches 0, a large number of segments is required.
    // If arc_length == 0, return 2.
    let n = if max_seg_angle > 0.0 {
        ((arc_length.abs() / max_seg_angle).ceil() as i64).max(2) as usize
    } else {
        2
    };
    (n, arc_length / (n as f64))
}
```
---


