# SimpleArray

지금까지 Rust로 옮겨온 SimpleArray<T>를 정리해서 소개 자료.  
이 자료는 개념, 특징, 주요 기능, 사용 예시까지 포함된 간단한 문서 형식입니다.

## 📄 SimpleArray 소개 자료
### 1. 개요
SimpleArray<T>는 OpenNURBS의 ON_SimpleArray<T>를 Rust 스타일로 재구현한 경량 동적 배열 컨테이너입니다.  
Rust의 Vec<T>를 내부적으로 사용하면서, OpenNURBS에서 제공하던 다양한 배열 조작 기능을 안전하게 제공합니다.

## 2. 특징
- 경량성: 불필요한 오버헤드 없이 Vec<T> 기반으로 구현
- 안전성: Rust의 소유권/참조 규칙을 활용하여 메모리 안전 보장
- 호환성: OpenNURBS의 함수군(AppendNew, RemoveValue, Permute, Sort 등)을 Rust 스타일로 제공
- 유연성: 제네릭 타입 지원 (SimpleArray<T>), PartialEq, Ord, Clone, Default 제약을 통해 기능 확장

### 3. 주요 기능
| C++ 기능                     | Rust 메서드                          | 설명 |
|------------------------------|--------------------------------------|------|
| Count()                      | count()                              | 요소 개수 반환 |
| Append()                     | append(v)                            | 요소 추가 |
| AppendNew()                  | append_default()                     | 기본값 요소 추가 |
| Remove()                     | remove()                             | 마지막 요소 제거 |
| RemoveAt()                   | remove_at(i)                         | 특정 인덱스 요소 제거 |
| RemoveValue()                | remove_value(&T)                     | 특정 값 제거 |
| Empty()                      | empty()                              | 배열 비우기 |
| Reverse()                    | reverse()                            | 배열 뒤집기 |
| Swap(i,j)                    | swap(i,j)                            | 두 요소 교환 |
| Search()                     | search(&T), find_by(|x| ...)         | 선형 검색 |
| BinarySearch()               | binary_search_by()                   | 이진 검색 (정렬된 배열) |
| QuickSort()                  | sort_by()                            | 사용자 정의 정렬 |
| QuickSortAndRemoveDuplicates() | sort_and_remove_duplicates()        | 정렬 후 중복 제거 |
| Permute()                    | permute(&[usize])                    | 인덱스 배열로 재배열 |
| Zero()                       | zero()                               | 모든 요소 기본값으로 초기화 |
| MemSet()                     | memset(value)                        | 모든 요소 동일 값으로 설정 |
| SetRange()                   | set_range(from,count,value)          | 특정 범위 값 설정 |
| Sort(index[])                | sort_indices()                       | 정렬된 인덱스 배열 반환 |


### 4. 사용 예시
```rust
let mut arr = SimpleArray::new();

// Append
arr.append(10);
arr.append(20);

// Append default
let x = arr.append_default();
*x = 30;

// Search
assert_eq!(arr.search(&20), Some(1));

// Sort
arr.sort_by(|a, b| a.cmp(b));

// Binary search
let idx = arr.binary_search_by(|x| x.cmp(&30)).unwrap();
assert_eq!(idx, 2);

// Remove value
arr.remove_value(&10);

// Permute
arr.permute(&[1,0]); // => [30,20]

// Zero / Memset
arr.zero();          // => [0,0]
arr.memset(5);       // => [5,5]
```

### 5. 활용 분야
- CAD/Geometry 엔진: OpenNURBS와 호환되는 Rust 기반 NURBS 커브/서피스 구현
- 데이터 처리: 빠른 배열 조작, 검색, 정렬, 중복 제거
- 알고리즘 실험: Permute, Sort indices 등 인덱스 기반 연산을 활용한 최적화

---

## 기능 확장

## 🔁 1단계: 기본 기능 확장
### ✅ first(), last(), at()
```rust
impl<T> SimpleArray<T> {
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }
```
```rust
    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }
```
```rust
    pub fn at(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
```
```rust
    pub fn at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }
}
```
### ✅ remove(), remove_at(), empty()
```rust
impl<T> SimpleArray<T> {
    pub fn remove(&mut self) -> Option<T> {
        self.data.pop()
    }
```
```rust
    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }
```
```rust
    pub fn empty(&mut self) {
        self.data.clear();
    }
}
```

### ✅ reverse(), swap(i,j)
```
```rust
impl<T> SimpleArray<T> {
    pub fn reverse(&mut self) {
        self.data.reverse();
    }
```
```rust
    pub fn swap(&mut self, i: usize, j: usize) -> bool {
        if i < self.data.len() && j < self.data.len() {
            self.data.swap(i, j);
            true
        } else {
            false
        }
    }
}
```
## 🧠 다음 단계 예고
다음 단계에서는 아래 기능들을 Rust 스타일로 확장할 수 있어요:
| C++ 기능        | Rust 확장 방향                          |
|----------------|-----------------------------------------|
| Search()       | find_index(&T), find_by(\|x\| ...)         |
| BinarySearch() | binary_search_by()                      |
| QuickSort()    | sort_by()                               |
| AppendNew()    | append_default()                        |
| RemoveValue()  | remove_value(&T)                        |
| Permute()      | permute(&[usize])                       |

---

### 🔍 1. Search → find_index, find_by
```rust
impl<T: PartialEq> SimpleArray<T> {
    /// Returns the index of the first element equal to `value`
    pub fn find_index(&self, value: &T) -> Option<usize> {
        self.data.iter().position(|x| x == value)
    }
}
```
```rust
impl<T> SimpleArray<T> {
    /// Returns the index of the first element satisfying the predicate
    pub fn find_by<F>(&self, mut predicate: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool,
    {
        self.data.iter().position(|x| predicate(x))
    }
}
```
### 🔢 2. BinarySearch → binary_search_by
```rust
impl<T> SimpleArray<T> {
    /// Binary search on sorted array using comparator
    pub fn binary_search_by<F>(&self, compare: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> std::cmp::Ordering,
    {
        self.data.binary_search_by(compare)
    }
}
```
- ⚠️ 이 함수는 정렬된 배열에서만 정확히 작동합니다. sort_by()와 함께 사용하세요.


### 🔀 3. QuickSort → sort_by
```rust
impl<T> SimpleArray<T> {
    /// Sorts the array using the provided comparator
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.data.sort_by(compare);
    }
}
```

---

### 1. AppendNew → append_default()
```rust
impl<T: Default> SimpleArray<T> {
    /// Appends a default-initialized element and returns mutable reference
    pub fn append_default(&mut self) -> &mut T {
        self.data.push(T::default());
        self.data.last_mut().unwrap()
    }
}
```
- C++의 AppendNew()처럼 memset(0)된 새 요소를 추가하는 기능입니다. Rust에서는 T: Default 제약으로 안전하게 처리합니다.


### 🗑️ 2. RemoveValue → remove_value(&T)
```rust
impl<T: PartialEq> SimpleArray<T> {
    /// Removes all elements equal to `value`
    pub fn remove_value(&mut self, value: &T) -> usize {
        let before = self.data.len();
        self.data.retain(|x| x != value);
        before - self.data.len()
    }
}
```
- C++의 RemoveValue()처럼 == 비교로 일치하는 모든 요소를 제거합니다. 제거된 개수를 반환합니다.


### 🔀 3. Permute → permute(&[usize])
```rust
impl<T: Clone> SimpleArray<T> {
    /// Reorders elements using index permutation
    pub fn permute(&mut self, index: &[usize]) -> bool {
        if index.len() != self.data.len() {
            return false;
        }
        let mut new_data = Vec::with_capacity(self.data.len());
        for &i in index {
            if i >= self.data.len() {
                return false;
            }
            new_data.push(self.data[i].clone());
        }
        self.data = new_data;
        true
    }
}
```
- C++의 Permute()처럼 index[i]에 따라 순서를 재배열합니다. T: Clone 제약으로 안전하게 복사합니다.


### ✅ 사용 예시
```rust
let mut arr = SimpleArray::new();
arr.append(10);
arr.append(20);
arr.append(30);

// append_default
let x = arr.append_default();
*x = 40;

// remove_value
arr.remove_value(&20);

// permute
let ok = arr.permute(&[2, 0, 1]); // 재배열
```
---

## 이번 단계에서는 중복 제거 정렬, 메모리 관리 유틸리티를 추가합니다.

### 🧹 1. QuickSortAndRemoveDuplicates → sort_and_remove_duplicates()
```rust
impl<T: Ord> SimpleArray<T> {
    /// Sorts the array and removes duplicates
    pub fn sort_and_remove_duplicates(&mut self) {
        self.data.sort();
        self.data.dedup();
    }
}
```
- C++의 QuickSortAndRemoveDuplicates()와 동일하게 정렬 후 중복 제거를 수행합니다.

### 📦 2. Reserve → reserve()
```rust
impl<T> SimpleArray<T> {
    /// Ensures capacity for at least `n` elements
    pub fn reserve(&mut self, n: usize) {
        self.data.reserve(n);
    }
}
```
### 🗜️ 3. Shrink → shrink()
```rust
impl<T> SimpleArray<T> {
    /// Shrinks capacity to fit current length
    pub fn shrink(&mut self) {
        self.data.shrink_to_fit();
    }
}
```

### 🔄 4. Zero → zero()
```rust
impl<T: Default + Clone> SimpleArray<T> {
    /// Resets all elements to default value
    pub fn zero(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = T::default();
        }
    }
}
```
### 🧩 5. MemSet → memset(value)
```rust
impl<T: Clone> SimpleArray<T> {
    /// Sets all elements to given value
    pub fn memset(&mut self, value: T) {
        for i in 0..self.data.len() {
            self.data[i] = value.clone();
        }
    }
}
```
### 🎯 6. SetRange → set_range(from, count, value)
```rust
impl<T: Clone> SimpleArray<T> {
    /// Sets a range of elements to given value
    pub fn set_range(&mut self, from: usize, count: usize, value: T) -> bool {
        if from + count > self.data.len() {
            return false;
        }
        for i in from..from+count {
            self.data[i] = value.clone();
        }
        true
    }
}
```
### ✅ 사용 예시
```rust
let mut arr = SimpleArray::new();
arr.append(1);
arr.append(2);
arr.append(2);
arr.append(3);

// sort and remove duplicates
arr.sort_and_remove_duplicates(); // => [1,2,3]

// reserve capacity
arr.reserve(100);

// zero all elements
arr.zero(); // => [0,0,0]

// memset
arr.memset(5); // => [5,5,5]

// set_range
arr.set_range(1, 2, 9); // => [5,9,9]
```

---


### 🔍 1. Search (고급 검색)
```rust
impl<T: PartialEq> SimpleArray<T> {
    /// Linear search for value, returns index or None
    pub fn search(&self, value: &T) -> Option<usize> {
        self.data.iter().position(|x| x == value)
    }
}
```
```rust
impl<T> SimpleArray<T> {
    /// Linear search with custom comparator
    pub fn search_by<F>(&self, key: &T, mut compare: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.data.iter().position(|x| compare(key, x) == std::cmp::Ordering::Equal)
    }
}
```
- C++의 Search()와 동일하게 선형 탐색을 수행합니다. 작은 배열이나 드문 검색에 적합합니다.

### 📊 2. Sort (인덱스 반환)
```rust
impl<T> SimpleArray<T> {
    /// Returns sorted index array without modifying original data
    pub fn sort_indices<F>(&self, mut compare: F) -> Vec<usize>
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        let mut indices: Vec<usize> = (0..self.data.len()).collect();
        indices.sort_by(|&i, &j| compare(&self.data[i], &self.data[j]));
        indices
    }
}
```
- C++의 Sort(index[])와 동일하게, 원본 배열은 그대로 두고 정렬된 인덱스 배열을 반환합니다.

### ✅ 사용 예시
```rust
let mut arr = SimpleArray::new();
arr.append(30);
arr.append(10);
arr.append(20);

// Search
if let Some(idx) = arr.search(&10) {
    println!("Found 10 at index {}", idx);
}

// Permute
arr.permute(&[1, 2, 0]); // => [10,20,30]

// Sort indices
let indices = arr.sort_indices(|a, b| a.cmp(b));
println!("Sorted indices = {:?}", indices); // => [0,1,2] (이미 정렬됨)
```
---

## 🧪 전체 테스트 코드

```rust
#[cfg(test)]
mod tests {
    use super::SimpleArray;

    #[test]
    fn test_basic_ops() {
        let mut arr = SimpleArray::new();
        arr.append(10);
        arr.append(20);
        arr.append(30);

        assert_eq!(arr.count(), 3);
        assert_eq!(arr[0], 10);
        assert_eq!(arr.first(), Some(&10));
        assert_eq!(arr.last(), Some(&30));
        assert_eq!(arr.at(1), Some(&20));
        assert_eq!(arr.at_mut(2), Some(&mut 30));
    }
```
```rust
    #[test]
    fn test_remove_ops() {
        let mut arr = SimpleArray::new();
        arr.append(1);
        arr.append(2);
        arr.append(3);

        assert_eq!(arr.remove(), Some(3));
        assert_eq!(arr.remove_at(0), Some(1));
        arr.empty();
        assert_eq!(arr.count(), 0);
    }
```
```rust
    #[test]
    fn test_reverse_swap() {
        let mut arr = SimpleArray::new();
        arr.append(1);
        arr.append(2);
        arr.append(3);

        arr.reverse();
        assert_eq!(arr.data, vec![3, 2, 1]);

        arr.swap(0, 2);
        assert_eq!(arr.data, vec![1, 2, 3]);
    }
```
```rust
    #[test]
    fn test_search() {
        let mut arr = SimpleArray::new();
        arr.append(5);
        arr.append(10);
        arr.append(15);

        assert_eq!(arr.search(&10), Some(1));
        assert_eq!(arr.find_index(&15), Some(2));
        assert_eq!(arr.find_by(|x| *x == 5), Some(0));
    }
```
```rust
    #[test]
    fn test_sort_binary_search() {
        let mut arr = SimpleArray::new();
        arr.append(30);
        arr.append(10);
        arr.append(20);

        arr.sort_by(|a, b| a.cmp(b));
        assert_eq!(arr.data, vec![10, 20, 30]);

        let idx = arr.binary_search_by(|x| x.cmp(&20)).unwrap();
        assert_eq!(idx, 1);
    }
```
```rust
    #[test]
    fn test_append_default_remove_value() {
        let mut arr = SimpleArray::new();
        arr.append(1);
        arr.append(2);
        arr.append(2);
        arr.append(3);

        let x = arr.append_default();
        *x = 4;
        assert!(arr.data.contains(&4));

        let removed = arr.remove_value(&2);
        assert_eq!(removed, 2);
        assert_eq!(arr.data, vec![1, 3, 4]);
    }
```
```rust
    #[test]
    fn test_permute() {
        let mut arr = SimpleArray::new();
        arr.append(10);
        arr.append(20);
        arr.append(30);

        let ok = arr.permute(&[2, 0, 1]);
        assert!(ok);
        assert_eq!(arr.data, vec![30, 10, 20]);
    }
```
```rust
    #[test]
    fn test_sort_indices() {
        let mut arr = SimpleArray::new();
        arr.append(30);
        arr.append(10);
        arr.append(20);

        let indices = arr.sort_indices(|a, b| a.cmp(b));
        assert_eq!(indices, vec![1, 2, 0]); // 10,20,30 순서
    }
```
```rust
    #[test]
    fn test_zero_memset_set_range() {
        let mut arr = SimpleArray::new();
        arr.append(1);
        arr.append(2);
        arr.append(3);

        arr.zero();
        assert_eq!(arr.data, vec![0, 0, 0]);

        arr.memset(5);
        assert_eq!(arr.data, vec![5, 5, 5]);

        let ok = arr.set_range(1, 2, 9);
        assert!(ok);
        assert_eq!(arr.data, vec![5, 9, 9]);
    }
```
```rust
    #[test]
    fn test_sort_and_remove_duplicates() {
        let mut arr = SimpleArray::new();
        arr.append(3);
        arr.append(1);
        arr.append(2);
        arr.append(2);

        arr.sort_and_remove_duplicates();
        assert_eq!(arr.data, vec![1, 2, 3]);
    }
}
```
---

## 소스
```rust
use std::ops::{Index, IndexMut};
use crate::core::matrix3::Matrix3;

#[derive(Clone, Default)]

pub struct SimpleArray<T> {
    pub data: Vec<T>,
}
```
```rust
impl<T> SimpleArray<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn count(&self) -> usize {
        self.data.len()
    }
    pub fn append(&mut self, v: T) {
        self.data.push(v);
    }
    pub fn set_capacity(&mut self, n: usize) {
        self.data.reserve(n);
    }
    pub fn shrink(&mut self) {
        self.data.shrink_to_fit();
    }
}
```
```rust
impl SimpleArray<f64> {
    pub fn get(&self, idx: usize) -> f64 {
        self.data[idx]
    }
}
```
```rust
impl<T> Index<usize> for SimpleArray<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
```
```rust
impl<T> IndexMut<usize> for SimpleArray<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}
```
```rust
impl<T> SimpleArray<T> {
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }
}
```
```rust
impl<T> SimpleArray<T> {
    pub fn remove(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn empty(&mut self) {
        self.data.clear();
    }
}
```
```rust
impl<T> SimpleArray<T> {
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn swap(&mut self, i: usize, j: usize) -> bool {
        if i < self.data.len() && j < self.data.len() {
            self.data.swap(i, j);
            true
        } else {
            false
        }
    }
}
```
```rust
impl<T: PartialEq> SimpleArray<T> {
    /// Returns the index of the first element equal to `value`
    pub fn find_index(&self, value: &T) -> Option<usize> {
        self.data.iter().position(|x| x == value)
    }
}
```
```rust
impl<T> SimpleArray<T> {
    /// Returns the index of the first element satisfying the predicate
    pub fn find_by<F>(&self, mut predicate: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool,
    {
        self.data.iter().position(|x| predicate(x))
    }
}
```
```rust
impl<T> SimpleArray<T> {
    /// Binary search on sorted array using comparator
    pub fn binary_search_by<F>(&self, compare: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> std::cmp::Ordering,
    {
        self.data.binary_search_by(compare)
    }
}
```
```rust
impl<T> SimpleArray<T> {
    /// Sorts the array using the provided comparator
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.data.sort_by(compare);
    }
}
```
```rust
impl<T: Default> SimpleArray<T> {
    /// Appends a default-initialized element and returns mutable reference
    pub fn append_default(&mut self) -> &mut T {
        self.data.push(T::default());
        self.data.last_mut().unwrap()
    }
}
```
```rust
impl<T: PartialEq> SimpleArray<T> {
    /// Removes all elements equal to `value`
    pub fn remove_value(&mut self, value: &T) -> usize {
        let before = self.data.len();
        self.data.retain(|x| x != value);
        before - self.data.len()
    }
}
```
```rust
impl<T: Ord> SimpleArray<T> {
    /// Sorts the array and removes duplicates
    pub fn sort_and_remove_duplicates(&mut self) {
        self.data.sort();
        self.data.dedup();
    }
}
```
```rust
impl<T> SimpleArray<T> {
    /// Ensures capacity for at least `n` elements
    pub fn reserve(&mut self, n: usize) {
        self.data.reserve(n);
    }
}
```
```rust
impl<T: Default + Clone> SimpleArray<T> {
    /// Resets all elements to default value
    pub fn zero(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = T::default();
        }
    }
}
```
```rust
impl<T: Clone> SimpleArray<T> {
    /// Sets all elements to given value
    pub fn memset(&mut self, value: T) {
        for i in 0..self.data.len() {
            self.data[i] = value.clone();
        }
    }
}
```
```rust
impl<T: Clone> SimpleArray<T> {
    /// Sets a range of elements to given value
    pub fn set_range(&mut self, from: usize, count: usize, value: T) -> bool {
        if from + count > self.data.len() {
            return false;
        }
        for i in from..from+count {
            self.data[i] = value.clone();
        }
        true
    }
}
```
```rust
impl<T: PartialEq> SimpleArray<T> {
    /// Linear search for value, returns index or None
    pub fn search(&self, value: &T) -> Option<usize> {
        self.data.iter().position(|x| x == value)
    }
}
```
```rust
impl<T> SimpleArray<T> {
    /// Linear search with custom comparator
    pub fn search_by<F>(&self, key: &T, mut compare: F) -> Option<usize>
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        self.data.iter().position(|x| compare(key, x) == std::cmp::Ordering::Equal)
    }
}
```
```rust
impl<T: Clone> SimpleArray<T> {
    /// Applies permutation index array to reorder elements
    pub fn permute(&mut self, index: &[usize]) -> bool {
        if index.len() != self.data.len() {
            return false;
        }
        let mut new_data = Vec::with_capacity(self.data.len());
        for &i in index {
            if i >= self.data.len() {
                return false;
            }
            new_data.push(self.data[i].clone());
        }
        self.data = new_data;
        true
    }
}
```
```rust
impl<T> SimpleArray<T> {
    /// Returns sorted index array without modifying original data
    pub fn sort_indices<F>(&self, mut compare: F) -> Vec<usize>
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        let mut indices: Vec<usize> = (0..self.data.len()).collect();
        indices.sort_by(|&i, &j| compare(&self.data[i], &self.data[j]));
        indices
    }
}
```
---

