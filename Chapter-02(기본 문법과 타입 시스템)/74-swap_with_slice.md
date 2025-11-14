# swap_with_slice
swap_with_slice는 Rust에서 두 개의 동일한 길이의 슬라이스의 내용을 서로 교환할 때 사용하는 매우 유용한 메서드입니다.  
아래에 자세히 설명.

## 🔁 swap_with_slice란?
```rust
fn swap_with_slice(&mut self, other: &mut [T])
```
- 정의: self와 other 슬라이스의 내용을 서로 교환합니다.
- 조건: 두 슬라이스는 길이가 같아야 하며, 겹치지 않아야 합니다.
- 제약: 슬라이스가 겹치면 런타임에 panic이 발생합니다.

## ✅ 사용 예시
```rust
let mut a = [1, 2, 3];
let mut b = [4, 5, 6];

a.swap_with_slice(&mut b);

assert_eq!(a, [4, 5, 6]);
assert_eq!(b, [1, 2, 3]);
```
- a와 b의 내용을 서로 교환합니다.
- 내부적으로는 `mem::swap` 을 반복해서 수행합니다.

### ⚠️ 주의사항
- 슬라이스 길이 불일치
```rust
let mut a = [1, 2, 3];
let mut b = [4, 5];
a.swap_with_slice(&mut b); // ❌ panic 발생
```
- 슬라이스가 겹치는 경우
```rust
let mut data = [1, 2, 3, 4];
let (a, b) = data.split_at_mut(2);
a.swap_with_slice(b); // ✅ 안전
```
- 하지만 아래처럼 겹치면 안 됩니다:
```rust
let mut data = [1, 2, 3, 4];
let a = &mut data[1..3];
let b = &mut data[2..4];
a.swap_with_slice(b); // ❌ panic: overlapping slices
```
- 소유권 문제가 먼저 발생

### 🧠 내부 동작swap_with_slice는 다음과 같은 루프를 내부적으로 수행합니다:
```rust
for (x, y) in self.iter_mut().zip(other.iter_mut()) {
    mem::swap(x, y);
}
```
- 즉, 각 요소를 하나씩 교환합니다.

### 📌 실전 활용 예: 이미지 수직 플립
```rust
let row_bytes = width * channels;
let (top, bottom) = pixels.split_at_mut(i2);
let a = &mut top[i1..i1 + row_bytes];
let b = &mut bottom[..row_bytes];
a.swap_with_slice(b);
```
- 이미지의 두 줄을 서로 교환할 때 매우 유용합니다.
- 단, 반드시 split_at_mut으로 겹치지 않게 분리해야 안전합니다.

---

swap_with_slice를 안전하게 사용할 수 있도록 슬라이스 길이와 겹침 여부를 검사하는 헬퍼 함수를 만들어 사용.  
이 함수는 런타임에 panic을 방지하고, 조건을 만족할 때만 안전하게 교환을 수행합니다.

## 🛡️ 안전한 swap_with_slice 헬퍼 함수
```rust
use std::ptr;

/// 두 슬라이스의 길이와 메모리 겹침 여부를 검사한 후 안전하게 swap_with_slice를 수행합니다.
pub fn safe_swap_with_slice<T>(a: &mut [T], b: &mut [T]) {
    // 길이 확인
    assert_eq!(a.len(), b.len(), "슬라이스 길이가 다릅니다.");

    // 메모리 겹침 확인
    let a_start = a.as_ptr() as usize;
    let a_end = unsafe { a.as_ptr().add(a.len()) as usize };
    let b_start = b.as_ptr() as usize;
    let b_end = unsafe { b.as_ptr().add(b.len()) as usize };

    let overlap = a_start < b_end && b_start < a_end;
    assert!(!overlap, "슬라이스가 메모리에서 겹칩니다.");

    // 안전하게 교환
    a.swap_with_slice(b);
}
```

### ✅ 사용 예시
```rust
fn main() {
    let mut a = [1, 2, 3];
    let mut b = [4, 5, 6];

    safe_swap_with_slice(&mut a, &mut b);

    assert_eq!(a, [4, 5, 6]);
    assert_eq!(b, [1, 2, 3]);
}

```
###  💡 팁: 매크로로 감싸고 싶다면?
```rust
macro_rules! safe_swap {
    ($a:expr, $b:expr) => {
        safe_swap_with_slice($a, $b)
    };
}
```
---

## 테스트 코드

```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::maths::on_safe_swap_with_slice;

    #[test]
    fn safe_swap_with_slice_test() {
        let mut a = [1, 2, 3];
        let mut b = [4, 5, 6];
        on_safe_swap_with_slice(&mut a, &mut b);
        assert_eq!(a, [4, 5, 6]);
        assert_eq!(b, [1, 2, 3]);
    }
```
```rust
    #[test]
    fn safe_swap_with_slice_test_one_array() {
        let mut data = [1, 2, 3, 4];
        let (a, b) = data.split_at_mut(2);
        on_safe_swap_with_slice(a, b);
        assert_eq!(data, [3, 4, 1, 2]);
    }
}
```

---
