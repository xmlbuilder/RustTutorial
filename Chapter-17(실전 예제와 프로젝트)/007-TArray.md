# TArray
TArray<T>는 Rust의 Vec<T>를 감싼 래퍼 타입으로,  
C++ 스타일의 배열 API를 Rust에서 사용할 수 있도록 설계된 구조.
기존 사용한 코드에 너무 많은 TArray가 사용 중이라 Wrapper개념으로 설계

## 소스
```rust
use std::fmt::{self, Display};
use std::ops::{Add, AddAssign, Index, IndexMut};
use num_traits::ToPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TArray<T> {
    data: Vec<T>,
}

impl<T> TArray<T> {
    // ---- 생성 ----
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// C++: TArray(int size) 와 동일 (요소는 T::default()로 채움)
    pub fn with_size(size: usize) -> Self
    where
        T: Default + Clone,
    {
        Self { data: vec![T::default(); size] }
    }

    /// C++: SetSize
    pub fn set_size(&mut self, size: usize)
    where
        T: Default + Clone,
    {
        self.data.resize(size, T::default());
    }

    /// C++: Resize (데이터 유지, 확장 시 default로 채움)
    pub fn resize(&mut self, size: usize)
    where
        T: Default + Clone,
    {
        self.set_size(size);
    }

    /// C++: RemoveAll
    pub fn remove_all(&mut self) {
        self.data.clear();
    }

    /// C++: IsEmpty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // ---- 크기/길이 ----
    pub fn len(&self) -> usize { self.data.len() }
    pub fn get_size(&self) -> usize { self.len() }
    pub fn get_count(&self) -> usize { self.len() }
    pub fn count(&self) -> usize { self.len() }
    pub fn length(&self) -> usize { self.len() }

    // ---- 접근 ----
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }
    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.data.first_mut()
    }

    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }


    pub fn last_mut(&mut self) -> &mut T {
        assert!(!self.data.is_empty(), "TArray::last_mut on empty array");
        let n = self.data.len();
        &mut self.data[n - 1]
    }

    // C++ Left/Right 별칭
    pub fn left(&self) -> Option<&T> { self.first() }
    pub fn right(&self) -> Option<&T> { self.last() }

    // 슬라이스/포인터 접근 (안전한 슬라이스 우선 권장)
    pub fn as_slice(&self) -> &[T] { &self.data }
    pub fn as_mut_slice(&mut self) -> &mut [T] { &mut self.data }
    pub fn as_ptr(&self) -> *const T { self.data.as_ptr() }
    pub fn as_mut_ptr(&mut self) -> *mut T { self.data.as_mut_ptr() }

    // ---- 수정 ----
    /// C++: Append(const Type&)
    pub fn append(&mut self, value: T) {
        self.data.push(value);
    }

    /// C++: Fill(value)
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.data.iter_mut().for_each(|x| *x = value.clone());
    }

    pub fn zero(&mut self)
    where
        T: Default,
    {
        self.data.iter_mut().for_each(|x| *x = T::default());
    }

    /// C++: SetData(arData, size)
    pub fn set_data_from_slice(&mut self, src: &[T])
    where
        T: Clone,
    {
        self.data.clear();
        self.data.extend_from_slice(src);
    }

    /// C++: RemoveAt(index)
    pub fn remove_at(&mut self, index: usize) -> Result<T, String> {
        if index < self.data.len() {
            Ok(self.data.remove(index))
        } else {
            Err(format!("Index {} out of bounds", index))
        }
    }

    pub fn insert_at(&mut self, index: usize, value: T) -> Result<(), String> {
        if index <= self.data.len() {
            self.data.insert(index, value);
            Ok(())
        } else {
            Err(format!("Index {} out of bounds", index))
        }
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn rotate_left(&mut self) {
        if self.data.len() > 1 {
            self.data.rotate_left(1);
        }
    }

    /// C++: RotateRight()
    pub fn rotate_right(&mut self) {
        if self.data.len() > 1 {
            self.data.rotate_right(1);
        }
    }

    pub fn rotate_to_index(&mut self, index: usize) {
        if !self.data.is_empty() {
            let len = self.data.len();
            self.data.rotate_left(index % len);
        }
    }

    pub fn initialize<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        self.data.iter_mut().for_each(|x| f(x));
    }

    // ---- 합계/누적 ----
    pub fn sum_f64(&self) -> f64
    where
        T: ToPrimitive,
    {
        self.data
            .iter()
            .filter_map(|x| x.to_f64())
            .sum()
    }

    /// C++: SumRange<R>(start,count) — R=f64 고정
    pub fn sum_range_f64(&self, start: isize, count: isize) -> f64
    where
        T: Copy + Into<f64>,
    {
        if self.data.is_empty() || count <= 0 { return 0.0; }
        let n = self.data.len() as isize;
        let s = start.max(0).min(n);
        let e = (s + count).max(s).min(n);
        self.data[s as usize..e as usize].iter().copied().map(Into::into).sum()
    }

    pub fn kahan_sum_f64(&self) -> f64
    where
        T: ToPrimitive,
    {
        let mut sum = 0.0f64;
        let mut c = 0.0f64;

        for x in &self.data {
            if let Some(v) = x.to_f64() {
                let y = v - c;
                let t = sum + y;
                c = (t - sum) - y;
                sum = t;
            }
        }

        sum
    }

    /// C++: SumBy<R>(map) — R: AddAssign + Default
    pub fn sum_by<R, F>(&self, mut map: F) -> R
    where
        R: Default + Add<Output = R>,
        F: Fn(&T) -> R,
    {
        self.data.iter().map(map).fold(R::default(), |acc, x| acc + x)
    }

    // ---- 이터레이터 ----
    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }
}

// ---- 인덱싱 ----
impl<T> Index<usize> for TArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for TArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// ---- 생성 보조 ----
impl<T> From<Vec<T>> for TArray<T> {
    fn from(v: Vec<T>) -> Self { Self { data: v } }
}
impl<T> From<&[T]> for TArray<T>
where
    T: Clone,
{
    fn from(slice: &[T]) -> Self { Self { data: slice.to_vec() } }
}
impl<T> FromIterator<T> for TArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self { data: iter.into_iter().collect() }
    }
}

// ---- 표시 ----
impl<T> Display for TArray<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, x) in self.data.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{x}")?;
        }
        Ok(())
    }
}

// ---- 연산자: a += &b  (C++ operator+ in-place와 유사) ----
impl<T: Clone> AddAssign<&TArray<T>> for TArray<T> {
    fn add_assign(&mut self, rhs: &TArray<T>) {
        self.data.extend_from_slice(&rhs.data);
    }
}

// (선택) a + b -> 새 배열 (Clone 필요)
impl<T: Clone> Add<&TArray<T>> for TArray<T> {
    type Output = TArray<T>;
    fn add(self, rhs: &TArray<T>) -> Self::Output {
        let mut out = self;
        out += rhs;
        out
    }
}

// ---- 편의 별칭(C++ API와 이름 맞춤) ----
impl<T> TArray<T> {
    pub fn get_data(&self) -> *const T { self.as_ptr() } // C++: GetData()
}


```

## 🌟 특징

|  항목 | 설명 |
|--------|------|
| 내부 구조 | `Vec<T>` 기반 |
| C++ 스타일 메서드 | `SetSize`, `RemoveAll`, `InsertAt`, `RotateLeft` 등 |
| 유틸리티 기능 | `sum_f64`, `kahan_sum_f64`, `initialize`, `fill`, `zero` |
| 연산자 오버로드 | `Add`, `AddAssign` → `a + b`, `a += b` 가능 |
| 출력 기능 | `Display` 구현 → `"1, 2, 3"` 형태로 출력 |
| 인덱싱 | `Index`, `IndexMut` → `array[i]` 접근 가능 |
| 생성 방식 | `Vec<T>`, `&[T]`, `Iterator`로부터 생성 가능 |



## ✅ 장점
### 1. C++ 스타일 API 친화적
- SetSize, RemoveAt, InsertAt, RotateToIndex 등 C++ 개발자에게 익숙한 메서드 이름과 동작을 제공.
- get_size, get_count, length 등 다양한 별칭으로 유연한 코드 작성 가능.
### 2. 다양한 유틸리티 기능
- fill, zero, initialize 등으로 배열 초기화 및 값 설정이 간편.
- sum_f64, kahan_sum_f64, sum_by 등 수치 연산 기능 내장.
### 3. 안전성과 표현력
- assert!를 통해 빈 배열 접근 시 런타임 오류 방지.
- Display 구현으로 디버깅 및 출력이 용이.
### 4. 연산자 오버로드
- +, += 연산으로 배열 병합이 직관적이고 간결함.

## ⚠️ 단점
### 1. 불필요한 래핑
- 대부분의 기능이 Vec<T>로도 충분히 구현 가능하며, TArray<T>는 중복된 추상화일 수 있음.
- Rust의 idiomatic 스타일과는 다소 거리가 있음 (예: first() 대신 get(0) 사용 권장).
### 2. 제네릭 제약
- 많은 메서드가 T: Clone, T: Default, T: Into<f64> 등의 제약을 요구하여 사용 범위가 제한될 수 있음.


## 🧠 요약
TArray<T>는 Rust에서 C++ 스타일 배열을 흉내 내고자 할 때 유용한 래퍼입니다. 
C++에서 Rust로 이식하는 프로젝트에서는 직관적인 API를 제공하지만,  
Rust의 철학(안전성, 표현력, 최소 추상화)과는 다소 거리가 있습니다.



## 🛠️ 리팩토링 전략 요약
| 항목       | 개선 방향 또는 대체 방식             |
|------------------------|--------------------------------------|
| `assert!` 사용         | `Option<&T>` 또는 `Result` 반환으로 안전성 향상 |
| 제네릭 제약            | `Clone` → `Copy` 또는 참조 기반 처리로 효율 개선 |
| 중복된 길이 메서드     | `get_size`, `get_count` → `len()` 하나로 통합 |
| 표준 트레잇 활용       | `IntoIterator`, `Extend`, `From` 적극 활용 |
| 인덱스 기반 수정 메서드 | `remove_at`, `insert_at` → `Result` 반환으로 오류 처리 |


## ✨ 리팩토링 예시
### 1. first, last → Option<&T> 반환
```rust
pub fn first(&self) -> Option<&T> {
    self.data.first()
}
```

```rust
let (x_min, x_max) = (ds.points.first().fx, ds.points.last().fx);
```

```rust
let (x_min, x_max) = match (ds.points.first(), ds.points.last()) {
    (Some(first), Some(last)) => (first.fx, last.fx),
    _ => return Err("points 배열이 비어 있습니다.".into()),
};
```

```rust
pub fn first_mut(&mut self) -> Option<&mut T> {
    self.data.first_mut()
}

pub fn last(&self) -> Option<&T> {
    self.data.last()
}

pub fn last_mut(&mut self) -> Option<&mut T> {
    self.data.last_mut()
}
```

### ✅ assert! 제거로 panic 없이 안전하게 사용 가능


## 2. remove_at, insert_at → Result 반환

```rust
pub fn remove_at(&mut self, index: usize) {
    assert!(index < self.data.len(), "index out of bounds");
    self.data.remove(index);
}
```

```rust
pub fn remove_at(&mut self, index: usize) -> Result<T, String> {
    if index < self.data.len() {
        Ok(self.data.remove(index))
    } else {
        Err(format!("Index {} out of bounds", index))
    }
}
```

```rust
pub fn insert_at(&mut self, index: usize, value: T) {
    assert!(index <= self.data.len(), "index out of bounds");
    self.data.insert(index, value);
}
```

```rust
pub fn insert_at(&mut self, index: usize, value: T) -> Result<(), String> {
    if index <= self.data.len() {
        self.data.insert(index, value);
        Ok(())
    } else {
        Err(format!("Index {} out of bounds", index))
    }
}
```

### ✅ 에러를 명시적으로 처리할 수 있어 안정성 향상




### 3. fill, zero → fill_with로 개선 가능
```rust
pub fn fill(&mut self, value: T)
where
    T: Clone,
{
    for x in &mut self.data {
        *x = value.clone();
    }
}
```



```rust
pub fn fill(&mut self, value: T)
where
    T: Clone,
{
    self.data.iter_mut().for_each(|x| *x = value.clone());
}
```


```rust
pub fn zero(&mut self)
where
    T: Default + Clone,
{
    self.fill(T::default());
}
```


```rust
pub fn zero(&mut self)
where
    T: Default,
{
    self.data.iter_mut().for_each(|x| *x = T::default());
}
```


### ✅ Clone 제약은 유지하되, 더 idiomatic한 반복 방식 사용

```rust
pub fn rotate_left(&mut self) {
    if self.data.len() > 1 {
        self.data.rotate_left(1);
    }
}
```

## 4. rotate_to_index → rotate_left 활용

```rust
pub fn rotate_to_index(&mut self, index: usize) {
    let n = self.data.len();
    if n <= 1 { return; }
    let k = index % n;
    if k > 0 {
        self.data.rotate_left(k);
    }
}
```

```rust
pub fn rotate_to_index(&mut self, index: usize) {
    if !self.data.is_empty() {
        self.data.rotate_left(index % self.data.len());
    }
}
```

#### ✅ 조건문 간결화, rotate_left 활용


## 🧠 추가 제안 (선택적)
- sum_f64, kahan_sum_f64은 num_traits::ToPrimitive 사용 시 더 범용적으로 개선 가능
- initialize는 iter_mut().for_each()로 표현 가능
- ToPrimitive는 Rust 표준 라이브러리에 포함된 것이 아니라 **외부 크레이트인 **에서 제공되는 트레잇입니다.


### 📦 어떻게 사용하나요?
Cargo.toml에 다음을 추가하면 됩니다:
```
[dependencies]
num-traits = "0.2"
``

그리고 코드에서는 이렇게 임포트합니다:
```rust
use num_traits::ToPrimitive;
```
이제 i32, u64, f32, f64 등 다양한 숫자 타입에 대해 .to_f64() 같은 메서드를 사용할 수 있어요.


### 1. sum_f64 → ToPrimitive 기반으로 개선
```rust
pub fn sum_f64(&self) -> f64
where
    T: Copy + Into<f64>,
{
    self.data.iter().copied().map(Into::into).sum()
}
```

```rust
use num_traits::ToPrimitive;

pub fn sum_f64(&self) -> f64
where
    T: ToPrimitive,
{
    self.data
        .iter()
        .filter_map(|x| x.to_f64())
        .sum()
}
```

🔍 filter_map을 사용해 None 값은 무시하고 안전하게 합산합니다.


### ✅ 2. kahan_sum_f64 → ToPrimitive 기반으로 개선

```rust
pub fn kahan_sum_f64(&self) -> f64
where
    T: Copy + Into<f64>,
{
    let mut sum = 0.0f64;
    let mut c = 0.0f64;
    for &v in &self.data {
        let y = Into::<f64>::into(v) - c;
        let t = sum + y;
        c = (t - sum) - y;
        sum = t;
    }
    sum
}
```

```rust
pub fn kahan_sum_f64(&self) -> f64
where
    T: ToPrimitive,
{
    let mut sum = 0.0f64;
    let mut c = 0.0f64;

    for x in &self.data {
        if let Some(v) = x.to_f64() {
            let y = v - c;
            let t = sum + y;
            c = (t - sum) - y;
            sum = t;
        }
    }

    sum
}
```

### ✅ to_f64()는 실패할 수 있으므로 if let으로 안전하게 처리


### ✅ 3. initialize → iter_mut().for_each()로 개선

```rust
pub fn initialize<F: FnMut(&mut T)>(&mut self, mut f: F) {
    for x in &mut self.data {
        f(x);
    }
}
```

```rust
pub fn initialize<F>(&mut self, mut f: F)
where
    F: FnMut(&mut T),
{
    self.data.iter_mut().for_each(|x| f(x));
}
```

### ✅ 더 간결하고 idiomatic한 표현으로 반복 처리



#### ✅ 기존 구현
```rust
pub fn sum_by<R, F>(&self, mut map: F) -> R
where
    R: Default + std::ops::AddAssign,
    F: FnMut(&T) -> R,
{
    let mut acc = R::default();
    for x in &self.data {
        acc += map(x);
    }
    acc
}
```


#### ✨ 개선된 버전
```rust
pub fn sum_by<R, F>(&self, map: F) -> R
where
    R: Default + Add<Output = R>,
    F: Fn(&T) -> R,
{
    self.data.iter().map(map).fold(R::default(), |acc, x| acc + x)
}
```

## 🔍 개선 포인트
- AddAssign 대신 Add를 사용해 더 일반적인 연산자 지원 (+ 기반)
- fold를 사용해 더 함수형이고 간결한 표현
- FnMut → Fn으로 변경해 불필요한 mut 제거 (필요 시 다시 FnMut로 바꿀 수 있음)

## 💡 사용 예시
```rust
let arr = TArray::from(vec![1, 2, 3, 4]);
let total = arr.sum_by(|x| x * 2); // 결과: 20
```

이렇게 하면 sum_by는 다양한 타입에 대해 더 유연하게 작동하며, Rust의 iterator 패턴을 자연스럽게 따릅니다.  

