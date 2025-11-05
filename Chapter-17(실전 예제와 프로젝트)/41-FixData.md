# 🧩 FixData<T> 구조 설계 문서
## 📦 개요
FixData<T>는 다중 컴포넌트 기반의 2차원 고정형 데이터 구조입니다.  
각 컴포넌트는 동일한 길이의 Vec<T>로 구성되며, 전체적으로 Vec<Vec<T>> 형태로 데이터를 보관합니다.  
이 구조는 성능 중심의 배열 처리, 컴포넌트 단위 접근, 인덱스 기반 조회 및 수정을 목적으로 설계되었습니다.

## 🧠 설계 의도
- ✅ 고정 길이의 컴포넌트 배열을 관리
- ✅ 컴포넌트 수와 배열 크기 초기화 및 재설정
- ✅ 인덱스 기반 접근 및 수정
- ✅ 컴포넌트 단위 슬라이스 제공
- ✅ 데이터 초기화 및 채우기 기능
- ✅ Index/IndexMut 연산자 오버로딩으로 직관적 접근

## 🧱 데이터 구조
```rust
pub struct FixData<T> {
    comps: Vec<Vec<T>>, // 컴포넌트별 데이터
    size: usize,         // 각 컴포넌트의 길이
}
```

- comps: 컴포넌트별 데이터 벡터
- size: 각 컴포넌트의 고정 길이

## 🔧 주요 메서드

| 메서드 이름              | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| new()                    | 기본 초기화된 빈 구조체를 생성합니다.                                |
| with(n, size)            | n개의 컴포넌트와 고정된 길이를 가진 데이터 구조를 생성합니다.         |
| init(n, size)            | 기존 구조체를 재초기화하여 컴포넌트 수와 길이를 설정합니다.           |
| clear()                  | 모든 컴포넌트와 데이터를 제거합니다.                                 |
| comp_count()             | 현재 컴포넌트의 개수를 반환합니다.                                   |
| len()                    | 각 컴포넌트의 길이(데이터 수)를 반환합니다.                          |
| is_empty()               | 데이터가 비어 있는지 확인합니다.                                     |
| get(comp, idx)           | 지정된 컴포넌트와 인덱스의 값을 반환합니다. (범위 검사 포함)          |
| set(comp, idx, val)      | 지정된 위치에 값을 설정합니다. (범위 검사 포함)                       |
| try_get(comp, idx)       | 안전하게 값을 조회하며 Option으로 반환합니다.                         |
| try_get_mut(comp, idx)   | 안전하게 가변 참조를 반환합니다.                                     |
| comp_slice(comp)         | 지정된 컴포넌트의 전체 슬라이스를 반환합니다.                         |
| comp_mut_slice(comp)     | 지정된 컴포넌트의 가변 슬라이스를 반환합니다.                         |
| resize_component(n, keep)| 컴포넌트 수를 재조정하며 기존 데이터를 유지할지 여부를 선택합니다.     |
| fill(val)                | 모든 값을 지정된 값으로 채웁니다.                                    |



## 🔁 Index 연산자 오버로딩
```rust
pub struct CompIndex(pub usize, pub usize);
impl<T> Index<CompIndex> for FixData<T> {
    fn index(&self, idx: CompIndex) -> &T
}
```
```rust
impl<T> IndexMut<CompIndex> for FixData<T> {
    fn index_mut(&mut self, idx: CompIndex) -> &mut T
}
```
- data[CompIndex(1, 3)] 형태로 접근 가능
- 컴포넌트와 인덱스를 튜플처럼 지정

## 🧪 타입 별 별칭
```rust
pub type FixDataF32 = FixData<f32>;
pub type FixDataF64 = FixData<f64>;
pub type FixDataI32 = FixData<i32>;
pub type FixDataI64 = FixData<i64>;
pub type FixDataArrayF64 = FixData<TArray<f64>>;
```

- 다양한 타입에 대해 명시적 별칭 제공
- TArray<f64> 같은 복합 타입도 지원

## 🧪 사용 예시
```rust
let mut data = FixData::<i32>::with(3, 5);
data.set(0, 2, 42);
println!("Value: {}", data.get(0, 2));

data[CompIndex(1, 4)] = 99;
println!("Slice: {:?}", data.comp_slice(1));
```


## 📌 확장 가능성
- FixData<T>는 ArrayPoolContainer와 연동하여 컴포넌트 기반 데이터 처리에 활용 가능
- TArray<T>와 함께 사용하면 다차원 배열 처리도 가능
- fill(), resize_component() 등을 통해 시뮬레이션, 그래픽, 수치해석 등 다양한 분야에 적용 가능


## 🧩 목표: data[comp][idx] 형태로 접근
이를 위해 FixData<T>에 대해 Index<usize>를 구현하면 됩니다:
```rust
impl<T> Index<usize> for FixData<T> {
    type Output = [T];

    fn index(&self, comp: usize) -> &Self::Output {
        self.comp_slice(comp)
    }
}
```
```rust
impl<T> IndexMut<usize> for FixData<T> {
    fn index_mut(&mut self, comp: usize) -> &mut Self::Output {
        self.comp_mut_slice(comp)
    }
}
```


## ✅ 결과
이제 다음과 같이 사용할 수 있어요:
```rust
let mut data = FixData::<i32>::with(3, 5);

// 이중 배열처럼 접근
data[0][2] = 42;
println!("Value: {}", data[0][2]);

// 슬라이스 전체 접근
let slice = &data[1];
println!("Slice: {:?}", slice);
```

## 📌 요약

| 구현 항목           | 설명 또는 사용 예시            |
|---------------------|-------------------------------|
| Index<usize>        | FixData[comp] → &[T]          |
| IndexMut<usize>     | FixData[comp] → &mut [T]      |
| 이중 인덱싱 지원     | FixData[comp][idx] 형태로 접근 가능 |

---

## 소스 코드 

```rust
use crate::core::tarray::TArray;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct FixData<T> {
    comps: Vec<Vec<T>>,
    size: usize,
}
```
```rust
impl<T> Default for FixData<T> {
    fn default() -> Self {
        Self {
            comps: Vec::new(),
            size: 0,
        }
    }
}
```
```rust

impl<T> FixData<T> {
    /// 빈 컨테이너 (comp=0, size=0)
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(n_comp: usize, size: usize) -> Self
    where
        T: Default + Clone,
    {
        let comps = vec![vec![T::default(); size]; n_comp];
        Self { comps, size }
    }

    pub fn init(&mut self, n_comp: usize, size: usize)
    where
        T: Default + Clone,
    {
        self.comps = vec![vec![T::default(); size]; n_comp];
        self.size = size;
    }

    pub fn clear(&mut self) {
        self.comps.clear();
        self.size = 0;
    }

    pub fn comp_count(&self) -> usize {
        self.comps.len()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    /// (C++: isEmpty)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get(&self, comp: usize, idx: usize) -> &T {
        assert!(
            comp < self.comp_count(),
            "comp {} out of range {}",
            comp,
            self.comp_count()
        );
        assert!(idx < self.size, "index {} out of range {}", idx, self.size);
        &self.comps[comp][idx]
    }

    pub fn set(&mut self, comp: usize, idx: usize, val: T) {
        assert!(
            comp < self.comp_count(),
            "comp {} out of range {}",
            comp,
            self.comp_count()
        );
        assert!(idx < self.size, "index {} out of range {}", idx, self.size);
        self.comps[comp][idx] = val;
    }

    pub fn try_get(&self, comp: usize, idx: usize) -> Option<&T> {
        self.comps.get(comp).and_then(|v| v.get(idx))
    }
    pub fn try_get_mut(&mut self, comp: usize, idx: usize) -> Option<&mut T> {
        self.comps.get_mut(comp).and_then(|v| v.get_mut(idx))
    }

    pub fn comp_slice(&self, comp: usize) -> &[T] {
        assert!(
            comp < self.comp_count(),
            "comp {} out of range {}",
            comp,
            self.comp_count()
        );
        &self.comps[comp]
    }
    pub fn comp_mut_slice(&mut self, comp: usize) -> &mut [T] {
        assert!(
            comp < self.comp_count(),
            "comp {} out of range {}",
            comp,
            self.comp_count()
        );
        &mut self.comps[comp]
    }

    pub fn resize_component(&mut self, n_comp: usize, keep_data: bool)
    where
        T: Default + Clone,
    {
        if n_comp == self.comp_count() {
            return;
        }

        if self.size == 0 {
            self.comps.resize_with(n_comp, Vec::new);
            return;
        }

        if keep_data {
            self.comps
                .resize_with(n_comp, || vec![T::default(); self.size]);
        } else {
            self.comps = vec![vec![T::default(); self.size]; n_comp];
        }
    }

    pub fn fill(&mut self, val: T)
    where
        T: Clone,
    {
        for c in &mut self.comps {
            for x in c.iter_mut() {
                *x = val.clone();
            }
        }
    }
}
```

```rust
pub struct CompIndex(pub usize, pub usize);
impl<T> Index<CompIndex> for FixData<T> {
    type Output = T;
    fn index(&self, idx: CompIndex) -> &Self::Output {
        self.get(idx.0, idx.1)
    }
}

impl<T> IndexMut<CompIndex> for FixData<T> {
    fn index_mut(&mut self, idx: CompIndex) -> &mut Self::Output {
        assert!(idx.0 < self.comp_count() && idx.1 < self.size);
        &mut self.comps[idx.0][idx.1]
    }
}
```
```rust
pub type FixDataF32 = FixData<f32>;
pub type FixDataF64 = FixData<f64>;
pub type FixDataI32 = FixData<i32>;
pub type FixDataI64 = FixData<i64>;
pub type FixDataArrayF64 = FixData<TArray<f64>>;

```

## 🧱 구조 개요: FixData<T>
`FixData<T>`는 컴포넌트(component) 단위로 데이터를 나누고, 각 컴포넌트는 고정 길이의 벡터를 갖습니다.  
예를 들어, FixData::<i32>::with(3, 4)는 Vec<Vec<i32>> 형태로 3개의 컴포넌트에 각각 4개의 값을 갖습니다.  
```rust
FixData<T> {
    comps: Vec<Vec<T>>, // 컴포넌트별 데이터
    size: usize,         // 각 컴포넌트의 길이
}
```

## 📋 주요 함수 요약표

| 함수 이름                  | 설명 또는 제약 조건                  |
|----------------------------|--------------------------------------|
| new()                      | size = 0                             |
| with(n_comp, size)         | T: Default + Clone                   |
| init(n_comp, size)         | 기존 객체를 재초기화                |
| clear()                    | 모든 데이터 제거                    |
| comp_count()               | 컴포넌트 수 반환                    |
| len()                      | 각 컴포넌트의 길이 반환             |
| is_empty()                 | 길이가 0인지 확인                   |
| get(comp, idx)             | 안전한 접근 (panic 발생 가능)       |
| set(comp, idx, val)        | 안전한 설정 (panic 발생 가능)       |
| try_get(comp, idx)         | 안전하지 않은 접근 (Option 반환)    |
| try_get_mut(comp, idx)     | 가변 참조 접근 (Option 반환)        |
| comp_slice(comp)           | 컴포넌트의 슬라이스 반환            |
| comp_mut_slice(comp)       | 컴포넌트의 가변 슬라이스 반환       |
| resize_component(n, keep_data) | 컴포넌트 수 변경 및 데이터 유지 여부 |
| fill(val)                  | 모든 값을 동일한 값으로 채움 (T: Clone) |



## 🧪 동작 예시
### 1. 생성 및 값 설정
```rust
let mut fd = FixData::<i32>::with(2, 3);
fd.set(0, 0, 10);
fd.set(1, 2, 99);
assert_eq!(fd.get(1, 2), &99);
```

### 2. 슬라이스 접근
```rust
let slice = fd.comp_slice(1);
println!("{:?}", slice); // [0, 0, 99]
```

### 3. 컴포넌트 리사이즈
```rust
fd.resize_component(3, true); // 기존 데이터 유지
fd.resize_component(1, false); // 전체 초기화
```

## 🧪 추가 샘플 코드: 평균 계산
```rust
fn average_per_component(fd: &FixData<f64>) -> Vec<f64> {
    let mut result = Vec::new();
    for c in 0..fd.comp_count() {
        let slice = fd.comp_slice(c);
        let sum: f64 = slice.iter().copied().sum();
        result.push(sum / slice.len() as f64);
    }
    result
}

#[test]
fn test_average() {
    let mut fd = FixData::<f64>::with(2, 3);
    fd.set(0, 0, 1.0);
    fd.set(0, 1, 2.0);
    fd.set(0, 2, 3.0);
    fd.set(1, 0, 4.0);
    fd.set(1, 1, 5.0);
    fd.set(1, 2, 6.0);

    let avg = average_per_component(&fd);
    assert_eq!(avg, vec![2.0, 5.0]);
}
```


## 🧠 활용 예시
- 수치 해석: 각 컴포넌트가 변수별 시계열 데이터일 때 유용
- 멀티 채널 처리: 이미지, 센서, 시뮬레이션 등에서 채널별 데이터 관리

---

