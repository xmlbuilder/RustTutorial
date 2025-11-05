# ArrayHandler
ArrayHandler<T> 구조체에 대한 설명 자료입니다.   
이 자료는 문서화, 코드 리뷰, 팀 공유 또는 라이브러리 주석용으로 활용하실 수 있도록 구성했습니다.  

## 📦 ArrayHandler<T> 설명 자료
## 개요
ArrayHandler<T>는 Rust의 Vec<T>를 래핑(wrapping)하여 버퍼 기반의 동적 배열 관리 기능을 제공하는 유틸리티 구조체입니다.  
일반적인 Vec<T>보다 더 유연한 버퍼 예약, 자동 확장, 삽입/삭제 최적화, 파라미터 관리 기능을 제공합니다.  

## 📐 구조 정의
```rust
pub struct ArrayHandler<T: Default + Clone> {
    data: Vec<T>,         // 내부 데이터 저장소
    buffer_size: usize,   // 확장 시 추가로 확보할 버퍼 크기
    param: i64,           // 사용자 정의 파라미터 (옵션)
}
```

## 🛠 주요 기능

### 🔹 생성자

| 메서드             | 설명                                      |
|--------------------|-------------------------------------------|
| `new()`            | 기본 생성자 (버퍼 크기 10,000)             |
| `with_buffer(size)`| 사용자 정의 버퍼 크기로 생성 (최소 4 이상) |

### 🔹 데이터 접근

| 메서드                  | 설명                                      |
|--------------------------|-------------------------------------------|
| `as_slice()`             | 읽기 전용 슬라이스 반환                   |
| `as_mut_slice()`         | 가변 슬라이스 반환                        |
| `as_split_at_mut(index)` | 슬라이스를 두 부분으로 분할               |
| `get(idx)`               | 인덱스 위치의 요소 반환                   |
| `get_mut(idx)`           | 인덱스 위치의 요소를 가변 참조로 반환     |

### 🔹 크기 및 버퍼 관리

| 메서드             | 설명                                      |
|--------------------|-------------------------------------------|
| `get_size()`       | 현재 요소 개수 반환                        |
| `get_alloc_size()` | 할당된 용량(capacity) 반환                 |
| `get_buffer_size()`| 버퍼 크기 반환                             |
| `set_size(size)`   | 크기 조정 (부족 시 `T::default()`로 채움) |
| `reserve(size)`    | 용량 확보 (여유 공간 추가)                |
| `reserve_exact(size)` | 정확한 용량 확보                         |
| `re_array_size()`  | 현재 크기에 맞춰 capacity 줄이기          |
| `clear()`          | 전체 초기화                               |
| `reset_array_size()` | 전체 초기화 (동일 기능)                 |

### 🔹 삽입 및 추가

| 메서드                    | 설명                                      |
|---------------------------|-------------------------------------------|
| `push_back(val)`          | 요소 추가                                 |
| `push_back_ref(&val)`     | 참조 기반 요소 추가                       |
| `push_back_slice(slice)`  | 슬라이스 추가                             |
| `insert(index, val)`      | 특정 위치에 단일 요소 삽입                |
| `insert_ref(index, &val)` | 참조 기반 삽입                            |
| `insert_slice(index, slice)` | 슬라이스 삽입                          |
| `append(&other)`          | 다른 ArrayHandler 이어붙이기              |
| `plus_assign(&other)`     | += 연산처럼 이어붙이기                    |

### 🔹 삭제 및 필터링

| 메서드                    | 설명                                      |
|---------------------------|-------------------------------------------|
| `remove_at(index)`        | 단일 요소 제거                            |
| `remove_range(index, size)` | 범위 제거 + shrink 최적화              |
| `remove_by_flags(flags, flag)` | 플래그 기반 필터링 삭제             |

### 🔹 설정 및 복사

| 메서드              | 설명                                      |
|---------------------|-------------------------------------------|
| `set_value(idx, val)` | 인덱스에 값 설정 (자동 확장 포함)        |
| `set_array(slice)`   | 전체 교체                                 |
| `assign_from(&other)`| 전체 복사                                 |
| `copy_from(&other)`  | 전체 복사 (동일 기능)                     |
| `set_param(p)`       | 사용자 정의 파라미터 설정                 |
| `get_param()`        | 파라미터 조회                             |



## 💡 설계 의도 및 장점
- 버퍼 기반 확장: push, insert 시마다 재할당을 줄이기 위해 buffer_size만큼 여유 공간 확보
- 자동 확장: set_value() 등에서 인덱스 초과 시 자동으로 resize 및 reserve
- 유연한 삽입/삭제: 슬라이스 단위 삽입, 플래그 기반 삭제 등 고급 기능 제공
- 메모리 최적화: 필요 시 shrink_to()로 capacity 줄이기

## 🧪 사용 예시
```rust
let mut arr = ArrayHandler::<i32>::with_buffer(1024);
arr.set_value(5, 42); // 자동 확장 및 값 설정
arr.push_back(100);
arr.insert(1, 77);
arr.remove_by_flags(&[0, 1, 0, 1, 0, 0], 1); // 플래그 기반 삭제
```

## 📌 주의사항
- T는 반드시 Default + Clone을 구현해야 합니다
- remove_by_flags()는 flags.len() >= data.len()을 요구합니다
- insert_slice()는 내부적으로 resize 및 memmove를 수행하므로 비용이 있습니다

---

## 소스 코드
```rust
#[derive(Clone, Debug)]
pub struct ArrayHandler<T: Default + Clone> {
    data: Vec<T>,
    buffer_size: usize,
    param: i64,
}

impl<T: Default + Clone> ArrayHandler<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            buffer_size: 10_000,
            param: 0,
        }
    }

    pub fn with_buffer(buffer_size: usize) -> Self {
        Self {
            data: Vec::new(),
            buffer_size: buffer_size.max(4),
            param: 0,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn as_split_at_mut(&mut self, index : usize) -> (&mut [T], &mut [T]) {
        self.data.split_at_mut(index)
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_alloc_size(&self) -> usize {
        self.data.capacity()
    }

    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub fn get_param(&self) -> i64 {
        self.param
    }

    pub fn set_buffer_size(&mut self, size: usize) {
        self.buffer_size = size.max(4);
    }

    pub fn set_param(&mut self, p: i64) {
        self.param = p;
    }

    pub fn set_size(&mut self, size: usize) {
        self.data.resize(size, T::default());
    }

    pub fn set_value(&mut self, idx: usize, element: T) {
        if idx >= self.data.len() {
            let new_size = idx + 1;
            if new_size > self.data.capacity() {
                self.data
                    .reserve(new_size + self.buffer_size - self.data.capacity());
            }
            self.data.resize(new_size, T::default());
        }
        self.data[idx] = element;
    }

    pub fn set_array(&mut self, src: &[T]) {
        self.data.clear();
        self.data.reserve(src.len());
        self.data.extend_from_slice(src);
    }

    pub fn set_array_size(&mut self, size: usize) {
        self.set_size(size);
    }

    pub fn reset_array_size(&mut self) {
        self.data.clear();
    }

    pub fn get(&self, idx: usize) -> &T {
        &self.data[idx]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut T {
        &mut self.data[idx]
    }

    pub fn reserve_exact(&mut self, size: usize) {
        if self.data.capacity() < size {
            self.data.reserve_exact(size - self.data.capacity());
        }
    }

    pub fn reserve(&mut self, size: usize) {
        if self.data.capacity() < size {
            self.data.reserve(size - self.data.capacity());
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn copy_from(&mut self, other: &Self) {
        self.data.clone_from(&other.data);
        self.buffer_size = other.buffer_size;
        self.param = other.param;
    }

    pub fn push_back_ref(&mut self, element: &T) -> usize {
        if self.data.len() + 1 > self.data.capacity() {
            self.data.reserve(self.buffer_size);
        }
        self.data.push(element.clone());
        self.data.len() - 1
    }

    pub fn push_back(&mut self, element: T) -> usize {
        self.push_back_ref(&element)
    }

    pub fn push_back_slice(&mut self, slice: &[T]) -> usize {
        let need = self.data.len() + slice.len();
        if need > self.data.capacity() {
            let add_size = if slice.len() > self.buffer_size {
                slice.len()
            } else {
                self.buffer_size
            };
            self.data.reserve(add_size);
        }
        self.data.extend_from_slice(slice);
        self.data.len().saturating_sub(1)
    }

    pub fn append(&mut self, other: &Self) {
        self.push_back_slice(&other.data);
    }

    pub fn assign_from(&mut self, other: &Self) {
        self.copy_from(other);
    }

    pub fn plus_assign(&mut self, other: &Self) {
        self.append(other);
    }

    pub fn insert_ref(&mut self, index: usize, element: &T) {
        if self.data.len() + 1 > self.data.capacity() {
            self.data.reserve(self.buffer_size);
        }
        self.data.insert(index, element.clone());
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.insert_slice(index, std::slice::from_ref(&element));
    }

    pub fn insert_slice(&mut self, index: usize, slice: &[T]) {
        assert!(index <= self.data.len());
        let k = slice.len();
        if k == 0 {
            return;
        }

        let old_len = self.data.len();
        let need = old_len + k;

        if need > self.data.capacity() {
            let add_size = if k > self.buffer_size {
                k
            } else {
                self.buffer_size
            };
            let reserve_more = (need - self.data.capacity()).max(add_size);
            self.data.reserve(reserve_more);
        }

        self.data.resize(need, T::default());

        for i in (index..old_len).rev() {
            self.data[i + k] = self.data[i].clone();
        }

        for (j, v) in slice.iter().cloned().enumerate() {
            self.data[index + j] = v;
        }
    }

    pub fn remove_at(&mut self, index: usize) {
        assert!(index < self.data.len());
        self.data.remove(index);
    }

    pub fn remove_range(&mut self, index: usize, size: usize) {
        assert!(index + size <= self.data.len());
        self.data.drain(index..index + size);
        // 필요 시 capacity를 다이어트
        let target_cap = self.data.len() + self.buffer_size;
        if self.data.capacity() > target_cap + self.buffer_size {
            self.data.shrink_to(target_cap);
        }
    }

    pub fn remove_by_flags(&mut self, flags: &[u8], remove_flag: u8) {
        assert!(flags.len() >= self.data.len());
        let mut write = 0usize;
        for i in 0..self.data.len() {
            if flags[i] != remove_flag {
                if write != i {
                    self.data[write] = self.data[i].clone();
                }
                write += 1;
            }
        }
        self.data.truncate(write);
        let target_cap = self.data.len() + self.buffer_size;
        if self.data.capacity() > target_cap + self.buffer_size {
            self.data.shrink_to(target_cap);
        }
    }

    pub fn re_array_size(&mut self) {
        let target_cap = self.data.len() + 1;
        if self.data.capacity() > target_cap {
            self.data.shrink_to(target_cap);
        }
    }
}
```
### 테스트 코드
```rust
fn array_handler_basic() {
    let mut ah: ArrayHandler<i32> = ArrayHandler::with_buffer(16);
    assert_eq!(ah.get_size(), 0);
    assert!(ah.is_empty());

    ah.set_size(3); // [0,0,0]
    assert_eq!(ah.get_size(), 3);
    assert_eq!(ah.as_slice(), &[0, 0, 0]);

    ah.set_value(5, 42); // [0,0,0,0,0,42]
    assert_eq!(ah.get_size(), 6);
    assert_eq!(ah.get(5), &42);

    ah.push_back(7);
    assert_eq!(ah.as_slice(), &[0, 0, 0, 0, 0, 42, 7]);

    ah.insert(1, 9);
    assert_eq!(ah.as_slice(), &[0, 9, 0, 0, 0, 0, 42, 7]);

    ah.insert_slice(3, &[1, 2, 3]);
    assert_eq!(ah.as_slice(), &[0, 9, 0, 1, 2, 3, 0, 0, 0, 42, 7]); // ← 이렇게

    ah.remove_at(2);
    assert_eq!(ah.as_slice(), &[0, 9, 1, 2, 3, 0, 0, 0, 42, 7]);

    ah.remove_range(3, 2); // remove 2 items at idx 3
    assert_eq!(ah.as_slice(), &[0, 9, 1, 0, 0, 0, 42, 7]);

    let flags = vec![0u8, 1, 0, 1, 1, 1, 0, 1];
    ah.remove_by_flags(&flags, 1);
    assert_eq!(ah.as_slice(), &[0, 1, 42]);

    ah.re_array_size(); // shrink
    assert!(ah.get_alloc_size() >= ah.get_size());
}

#[test]
fn array_handler_copy_append() {
    let mut a: ArrayHandler<i32> = ArrayHandler::new();
    a.set_array(&[1, 2, 3, 4]);

    let mut b = ArrayHandler::with_buffer(8);
    b.copy_from(&a);
    assert_eq!(b.as_slice(), &[1, 2, 3, 4]);

    let mut c: ArrayHandler<i32> = ArrayHandler::new();
    c.set_array(&[5, 6]);
    b.append(&c);
    assert_eq!(b.as_slice(), &[1, 2, 3, 4, 5, 6]);
}
```
---

