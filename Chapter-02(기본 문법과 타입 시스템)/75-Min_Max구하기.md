# Min / Max 구하기
Rust의 Vec<T>에서 최솟값, 최댓값, 그리고 그 인덱스를 구하는 방법은 표준 라이브러리의  
iterator 메서드를 활용하면 아주 간단하게 처리할 수 있음.

## 🔍 값만 구하기: min, max
```rust
let v = vec![10, 20, 5, 8, 30];

let min = v.iter().min(); // Some(&5)
let max = v.iter().max(); // Some(&30)
```

- iter()는 참조를 반환하므로 결과는 Option<&T> 타입
- 값(`Option<T>`) 만 원하면 `.copied()` 또는 `.cloned()` 를 붙이면 됨
  
```rust
let min_val = v.iter().copied().min(); // 5
```

## 📍 인덱스까지 구하기: position + min_by_key
```rust
let v = vec![10, 20, 5, 8, 30];

if let Some((idx, val)) = v.iter().enumerate().min_by_key(|&(_, v)| v) {
    println!("min: {} at index {}", val, idx);
}
```

- `enumerate()` 로 인덱스를 붙이고
- `min_by_key` 로 값 기준으로 최소를 찾음
- 같은 방식으로 max_by_key도 가능:
```rust
if let Some((idx, val)) = v.iter().enumerate().max_by_key(|&(_, v)| v) {
    println!("max: {} at index {}", val, idx);
}
```

## ✨ 정리
| 목적             | 코드 예시                                                  |
|------------------|-------------------------------------------------------------|
| 최솟값           | `v.iter().copied().min()`                                     |
| 최댓값           | `v.iter().copied().max()`                                     |
| 최솟값 인덱스    | `v.iter().enumerate().min_by_key(|&(_, v)| v)`               |
| 최댓값 인덱스    | `v.iter().enumerate().max_by_key(|&(_, v)| v)`               |

---

# 부동 소수점

 Vec<f32> 또는 Vec<f64>처럼 부동소수점(floating point) 값일 때도 min_by나 min_by_key를 사용해서 최솟값이나 인덱스를 구할 수 있음.  
 다만, Ord 트레이트가 구현되어 있지 않기 때문에 직접 비교해야 해요.

## ✅ 최솟값 / 최댓값 구하기 (f32, f64)
```rust
let v = vec![3.2, 1.5, 4.8, 2.0];
let min = v.iter().cloned().reduce(f32::min); // Some(1.5)
let max = v.iter().cloned().reduce(f32::max); // Some(4.8)
```
- reduce는 Iterator에서 누적 계산을 할 때 사용
- cloned() 또는 copied()로 값 복사

## 📍 인덱스까지 구하기 (enumerate + partial_cmp)
```rust
let v = vec![3.2, 1.5, 4.8, 2.0];

if let Some((idx, val)) = v
    .iter()
    .enumerate()
    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
{
    println!("min: {} at index {}", val, idx);
}
```
- partial_cmp는 f32/f64 비교에 사용
- unwrap()은 NaN이 없다는 전제 하에 사용 (있으면 None 반환됨)
- 같은 방식으로 max_by도 가능:
```rust
if let Some((idx, val)) = v
    .iter()
    .enumerate()
    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
{
    println!("max: {} at index {}", val, idx);
}
```

## ⚠️ NaN 주의
- f32::partial_cmp는 NaN이 포함되면 None을 반환하므로 unwrap()이 panic 날 수 있음
- 안전하게 하려면 filter(|v| !v.is_nan())로 NaN 제거 후 처리

---

## 🔍 iter()는 참조를 반환함
```rust
let v = vec![10, 20, 5];
let min = v.iter().min(); // 타입: Option<&i32>
```
- iter()는 &i32 같은 참조를 순회
- 그래서 min() 결과도 `Option<&i32>` 가 됨

## ✅ 값만 원할 때 .copied() 또는 .cloned()
```rust
let min_val = v.iter().copied().min(); // Option<i32>
```
- .copied()는 Copy 가능한 타입(i32, f32 등)을 값으로 복사
- .cloned()는 Clone 가능한 타입(String, Vec 등)을 값으로 복제
- 즉, `Option<&T>` → `Option<T>` 로 바꾸는 거예요

📌 예시 비교
| 코드                        | 반환 타입       | 설명                         |
|-----------------------------|------------------|------------------------------|
| `v.iter().min()`              | `Option<&i32>`     | 참조값 반환                  |
| `v.iter().copied().min()`     | `Option<i32>`      | 값 복사해서 반환 (`Copy`)   |
| `v.iter().cloned().min()`     | `Option<i32>`      | 값 복제해서 반환 (`Clone`)  |

---




