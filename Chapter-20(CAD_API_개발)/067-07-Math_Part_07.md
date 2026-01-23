## on_offset_point

- 이 함수는 정말 핵심만 정확하게 잡은 두 점(P1→P2) 방향으로 일정 거리만큼  
    P2를 밀어내는(offset) 유틸리티.

### 📘 on_offset_point(p1, p2, length)
- P1→P2 방향으로 정규화한 뒤, 그 방향으로 P2를 length만큼 이동한 점을 반환
```rust
pub fn on_offset_point(p1: Point3D, p2: Point3D, length: f64) -> Point3D {
    let v = (p2 - p1).unitize();
    Point3D::new(
        p2.x + length * v.x,
        p2.y + length * v.y,
        p2.z + length * v.z,
    )
}
```


### 1️⃣ 동작 원리
- 방향 벡터 계산
```rust
let v = (p2 - p1).unitize;
```
- P1 → P2 방향.
- 정규화
```rust
unitize();
```
- 길이를 1로 만든다.
- P2에서 length만큼 이동
```rust
p2 + length * v
```

즉,
```math
\mathrm{offset}=P_2+\mathrm{length}\cdot \frac{P_2-P_1}{\| P_2-P_1\| }
```

### 2️⃣ 사용 예시- 선분(P1→P2)의 방향으로 일정 거리 떨어진 점 계산
- 폴리라인 offset
- 곡선의 forward/backward stepping
- CAD에서 trimming/extension 시 endpoint 이동

```rust
pub fn on_offset_point(p1: Point3D, p2: Point3D, length: f64) -> Point3D {
    let v = p2 - p1;
    v.unitize();
    Point3D::new(
        p2.x + length * v.x,
        p2.y + length * v.y,
        p2.z + length * v.z,
    )
}
```

## on_sort_doubles

- 부동소수 정렬 시 발생할 수 있는 NaN/−0.0 문제까지 안전하게 처리하는 정석 구현.

### 📘 on_sort_doubles(arr, increasing)
- f64 배열을 total ordering 기반으로 정렬 (오름차순/내림차순)
```rust
pub fn on_sort_doubles(arr: &mut [f64], increasing: bool) {
    if arr.len() <= 1 {
        return;
    }
    if increasing {
        arr.sort_by(|a, b| a.total_cmp(b));
    } else {
        arr.sort_by(|a, b| b.total_cmp(a));
    }
}
```

### 1️⃣ 핵심 포인트
- ✔ total_cmp 사용
    - Rust의 f64::total_cmp는 IEEE 754 값을 **완전한 전순서(total order)** 로 비교한다.
    - NaN도 정렬 가능
    - -0.0과 +0.0도 구분
    - partial_cmp와 달리 None이 발생하지 않음
    - 정렬 시 panic 위험 없음
- 즉, 부동소수 정렬 시 가장 안전한 방식이다.

### 2️⃣ increasing 플래그
- true → 오름차순
- false → 내림차순 (비교 순서만 뒤집음)

### 3️⃣ 시간 복잡도
- Rust의 .sort_by는 Timsort 기반 O(n log n)  
    부동소수 비교 비용도 매우 작아서 전체적으로 빠르다.


```rust
pub fn on_sort_doubles(arr: &mut [f64], increasing: bool) {
    if arr.len() <= 1 {
        return;
    }
    if increasing {
        arr.sort_by(|a, b| a.total_cmp(b));
    } else {
        arr.sort_by(|a, b| b.total_cmp(a));
    }
}
```
## on_cull_doubles

- 이 함수는 정렬된 f64 배열에서 서로 너무 가까운 값들을 제거(cull)하는 안정적인 유틸리티.

### 📘 on_cull_doubles(arr, tol)
- 정렬 후, 인접한 값들 중 Δ ≤ tol 인 항목을 제거하여 unique-like 결과 생성
```rust
pub fn on_cull_doubles(arr: &mut Vec<f64>, mut tol: f64) -> usize {
    if arr.len() <= 1 {
        return arr.len();
    }
    arr.sort_by(|a, b| a.total_cmp(b));
    if tol < f64::EPSILON.sqrt() {
        tol = f64::EPSILON.sqrt();
    }
    // 뒤에서 앞으로 스캔하며 근접값 제거
    let mut i = arr.len() - 1;
    let mut d = arr[i];
    while i > 0 {
        let j = i - 1;
        if (d - arr[j]).abs() <= tol {
            arr.remove(j);
        } else {
            d = arr[j];
        }
        i = j;
    }
    arr.shrink_to_fit();
    arr.len()
}
```


### 1️⃣ 동작 방식 요약
- 정렬
```rust
arr.sort_by(|a, b| a.total_cmp(b));
```
- total_cmp 사용 → NaN, -0.0 문제 없이 완전한 전순서 정렬.
- tol 최소값 보정
```rust
if tol < f64::EPSILON.sqrt() {
    tol = f64::EPSILON.sqrt();
}
```
- 너무 작은 tol은 floating-point noise 때문에 의미가 없으므로 하한을 둠.
- 뒤에서 앞으로 스캔하며 근접값 제거
- 기준값 d를 유지
- 바로 앞 값과 비교
- d − arr[j] ≤ tol 이면 제거
- 아니면 기준값 갱신
- 메모리 정리
```rust
arr.shrink_to_fit();
```
### 2️⃣ 왜 “뒤에서 앞으로” 스캔할까?
- Vec::remove(j)는 j 이후의 모든 요소를 왼쪽으로 shift한다.
- 앞에서 뒤로 스캔하면 index가 꼬일 위험이 있음
- 뒤에서 앞으로 스캔하면 안전하게 제거 가능
이건 Rust에서도 가장 흔히 쓰는 패턴이다.
### 3️⃣ 시간 복잡도- 정렬: O(n log n)
- 스캔: O(n)
- remove는 O(n)이지만, 제거 횟수가 많지 않으면 전체적으로 여전히 빠름  
    (필요하면 retain 기반 최적화 버전도 만들 수 있음)
### 4️⃣ 개선 포인트(선택)성능을 더 높이고 싶다면 
- remove 대신 새 벡터에 push하는 방식으로 O(n) 유지 가능:
- 정렬 후
    - 앞에서 뒤로 스캔
    - “유지할 값만” 새 Vec에 push
    - 마지막에 swap
- 하지만 지금 구현도 충분히 빠르고 명확하다.
### 📌 최종 요약이 함수는:
- f64 배열을 정렬
- tol 이내로 가까운 값들을 제거
- 안정적인 total ordering 사용
- 뒤에서 앞으로 스캔하여 안전하게 중복 제거
- tol이 너무 작을 때 자동 보정

```rust
pub fn on_cull_doubles(arr: &mut Vec<f64>, mut tol: f64) -> usize {
    if arr.len() <= 1 {
        return arr.len();
    }
    arr.sort_by(|a, b| a.total_cmp(b));
    if tol < f64::EPSILON.sqrt() {
        tol = f64::EPSILON.sqrt();
    }
    // 뒤에서 앞으로 스캔하며 근접값 제거
    let mut i = arr.len() - 1;
    let mut d = arr[i];
    while i > 0 {
        let j = i - 1;
        if (d - arr[j]).abs() <= tol {
            arr.remove(j);
        } else {
            d = arr[j];
        }
        i = j;
    }
    arr.shrink_to_fit();
    arr.len()
}
```

## on_cull_doubles_keep_canonical

- 이 버전은 앞서 만든 on_cull_doubles의 “canonical representative 유지” 버전으로,  
    중복 제거 시 항상 첫 번째(가장 작은) 값을 대표값으로 남기는 안정적인 방식.

### 📘 on_cull_doubles_keep_canonical(arr, tol)
- 정렬 후, tol 이내로 가까운 값들을 제거하되 **첫 번째 값** 을 canonical 로 유지
```rust
pub fn on_cull_doubles_keep_canonical(arr: &mut Vec<f64>, mut tol: f64) 
-> usize {
    if arr.len() <= 1 {
        return arr.len();
    }

    if tol < f64::EPSILON.sqrt() {
        tol = f64::EPSILON.sqrt();
    }

    arr.sort_by(|a, b| a.total_cmp(b));

    let mut out = Vec::with_capacity(arr.len());
    let mut last = arr[0];
    out.push(last);

    for &x in arr.iter().skip(1) {
        if (x - last).abs() > tol {
            out.push(x);
            last = x;
        }
    }

    *arr = out;
    arr.shrink_to_fit();
    arr.len()
}
```

### 1️⃣ 동작 방식 요약
- tol 보정
- 너무 작은 tol은 floating-point noise 때문에 의미가 없으므로  
    sqrt(EPSILON)을 하한으로 둔다.
- 정렬
```rust
arr.sort_by(|a, b| a.total_cmp(b));
```
- total_cmp 사용 → NaN, -0.0 문제 없이 완전한 전순서.
- canonical representative 유지
- 정렬된 상태에서 첫 값 last = arr[0]을 대표값으로 선택
- 이후 값 x가 |x - last| > tol일 때만 새 그룹 시작
- 즉, 각 근접 그룹의 첫 번째 값만 남김
- 결과 벡터로 교체

### 2️⃣ 왜 “canonical”인가?
- 예를 들어:
```rust
arr = [1.0000, 1.0000000001, 1.0000000002, 2.0]
tol = 1e-8
```

- 일반 cull:
```
[1.0000000002, 2.0]   // 뒤에서 앞으로 스캔하면 마지막 값이 대표가 됨
```

- canonical cull:
```
[1.0000, 2.0]         // 항상 첫 번째 값이 대표
```

- 즉, 정렬된 상태에서 가장 작은 값이 그룹 대표로 유지된다.
- 이건 CAD/Geometry에서 매우 중요:
    - 파라미터 값 정리
    - knot vector 정리
    - 중복 t-value 제거
    - floating-point cluster를 안정적으로 대표값 하나로 축약

### 3️⃣ 시간 복잡도
- 정렬: O(n log n)
- 스캔: O(n)
- 메모리: O(n) (out 벡터)
- remove()를 쓰지 않기 때문에 O(n²) worst-case가 절대 발생하지 않는다는 점이 장점.


```rust
pub fn on_cull_doubles_keep_canonical(arr: &mut Vec<f64>, mut tol: f64) -> usize {
    if arr.len() <= 1 {
        return arr.len();
    }

    if tol < f64::EPSILON.sqrt() {
        tol = f64::EPSILON.sqrt();
    }

    arr.sort_by(|a, b| a.total_cmp(b));

    let mut out = Vec::with_capacity(arr.len());
    let mut last = arr[0];
    out.push(last);

    for &x in arr.iter().skip(1) {
        if (x - last).abs() > tol {
            out.push(x);
            last = x;
        }
    }

    *arr = out;
    arr.shrink_to_fit();
    arr.len()
}
```

## on_factorial_u128

- Rust에서 팩토리얼을 구현할 때 가장 중요한 건 오버플로를 절대 허용하지 않는 것

### 📘 on_factorial_u128(n)
- u128 범위에서 팩토리얼을 계산하며, 오버플로 시 None 반환
```rust
pub fn on_factorial_u128(n: usize) -> Option<u128> {
    let mut acc: u128 = 1;
    for i in 2..=n {
        acc = acc.checked_mul(i as u128)?;
    }
    Some(acc)
}
```


### 1️⃣ 동작 방식
- acc를 1로 시작
- 2부터 n까지 곱해 나감
- 매 곱셈마다 checked_mul 사용
- 오버플로 발생 시 None 반환
- 안전하고 예측 가능한 동작

### 2️⃣ u128에서 팩토리얼이 가능한 최대 n
- 참고로 u128에서 overflow 없이 계산 가능한 최대 팩토리얼은:
    - 34! → fits
    - 35! → overflow
- 즉:
```rust
on_factorial_u128(34) → Some(295232799039604140847618609643520000000)
on_factorial_u128(35) → None
```


```rust
#[allow(unused)]
pub fn on_factorial_u128(n: usize) -> Option<u128> {
    let mut acc: u128 = 1;
    for i in 2..=n {
        acc = acc.checked_mul(i as u128)?;
    }
    Some(acc)
}
```
## on_binomial_via_factorial_f64 

- 이 함수는 팩토리얼 기반 조합수(binomial coefficient) 계산의 가장 단순한 형태지만,  
    수치적으로는 몇 가지 중요한 특징과 한계가 있음

### 📘 on_binomial_via_factorial_f64(n, k)
- u128 팩토리얼을 이용해 조합수 C(n,k)를 f64로 계산
```rust
pub fn on_binomial_via_factorial_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let nf = on_factorial_u128(n).unwrap_or(0) as f64;
    let kf = on_factorial_u128(k).unwrap_or(0) as f64;
    let nk = on_factorial_u128(n - k).unwrap_or(0) as f64;
    nf / (kf * nk)
}
```

### 1️⃣ 동작 방식
- 조합수:
```math
{n \choose k}=\frac{n!}{k!(n-k)!}
```
- 이걸 그대로 구현한 함수다.
    - on_factorial_u128로 정확한 정수 팩토리얼을 구함
    - 오버플로 시 None → unwrap_or(0) → 결과 0.0
    - 마지막에 f64로 변환하여 나눗셈

### 2️⃣ 장점
- 작은 n에서는 매우 정확
    - u128 팩토리얼을 사용하므로  
        n ≤ 34 범위에서는 정확한 정수 조합수를 얻을 수 있음
- 구현이 단순하고 명확

### 3️⃣ 한계 (중요)
- ✔ u128 factorial의 한계
    - u128에서 factorial이 가능한 최대 n은 34.
    - 35! → overflow → None → 0.0
    - 따라서 n ≥ 35인 경우 대부분 **잘못된 결과(0.0)** 가 나온다.
- ✔ f64 변환 시 정밀도 손실
- f64는 53비트 정밀도이므로:
    - 20! 정도까지는 정확
    - 30! 이상은 정밀도 손실
    - 조합수 C(34,17) 같은 값은 f64로는 정확히 표현 불가
- ✔ unwrap_or(0)의 의미
- 팩토리얼이 overflow하면 0을 반환하므로:
    - C(n,k)가 실제로 0이 아닌데도 0.0이 나올 수 있음
    - 즉, overflow를 **조용히** 삼켜버리는 형태

### 4️⃣ 개선 포인트 (선택)
- 만약 더 robust하게 만들고 싶다면:
#### ① overflow를 명확히 구분
```rust
let nf = on_factorial_u128(n)?;
```
- 처럼 Option<f64>로 propagate.
#### ② f64 overflow 방지
- 팩토리얼 대신 곱셈 기반 조합 계산을 사용:
```math
{n \choose k}=\prod _{i=1}^k\frac{n+1-i}{i}
```
- 이 방식은 n이 매우 커도 안정적.
#### ③ log 기반 계산
- 큰 n에서도 안정적인 Stirling/log 방식.

#### 📌 최종 요약
- 이 함수는:
    - n ≤ 34 범위에서는 정확
    - 그 이상에서는 u128 overflow 때문에 0.0이 나올 수 있음
- f64 변환으로 인해 큰 조합수는 정밀도 손실 발생


```rust
#[allow(unused)]
pub fn on_binomial_via_factorial_f64(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let nf = on_factorial_u128(n).unwrap_or(0) as f64;
    let kf = on_factorial_u128(k).unwrap_or(0) as f64;
    let nk = on_factorial_u128(n - k).unwrap_or(0) as f64;
    nf / (kf * nk)
}
```


```rust
#[allow(unused)]
pub fn on_factorial(n: usize) -> i64 {
    if n <= 1 {
        1
    } else {
        (n as i64) * on_factorial(n - 1)
    }
}
```
---
