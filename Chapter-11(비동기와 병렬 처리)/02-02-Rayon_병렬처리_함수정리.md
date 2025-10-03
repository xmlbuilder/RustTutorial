# Rayon

병렬 처리임에도 불구하고 문법이 거의 동일하다는 건 바로 Rayon의 강점 💡
Rust의 iterator 철학과 Rayon의 디자인이 얼마나 잘 맞물려 있는지를 보여주는 사례.

### ✅ Rayon의 핵심 철학
| 특징               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| 문법 호환성        | `iter()` → `par_iter()`, `chunks_mut()` → `par_chunks_mut()` 등 거의 동일 |
| 자동 스레드 분배   | 내부 thread pool을 통해 작업을 자동 분산. 개발자가 직접 스레드 관리할 필요 없음 |
| 데이터 안전성 유지 | Rust의 borrow checker와 함께 안전한 병렬 처리 보장                     |
| 성능 최적화        | chunk 크기, 작업 분할, 캐시 친화적 접근 등을 자동으로 조정               |

## 🔁 비교 예시
### 일반 처리
```rust
for x in data.iter_mut() {
    *x += 1;
}
```

### 병렬 처리
```rust
use rayon::prelude::*;
data.par_iter_mut().for_each(|x| {
    *x += 1;
});
```

거의 동일한 구조지만, 병렬로 실행되며 성능 향상이 가능.


## 🧠 Matrix 연산에 적용하면?
- par_chunks_mut() → 행 단위 병렬 처리
- par_iter_mut() → 전체 요소 병렬 처리
- par_windows() → 슬라이딩 병렬 처리
- IndexedParallelIterator → 인덱스 기반 병렬 접근

## 💬 결론
Rayon은 Rust의 iterator 문법을 그대로 유지하면서  
병렬성을 “붙이기만 하면 되는” 수준으로 추상화해줍니다.  
그래서 병렬이 아닌 코드와 거의 동일한 문법으로 병렬 처리가 가능.

---

# Rayon 핵심 기능 정리

아래는 Rayon에서 자주 쓰이는 핵심 기능들을 문서로 정리한 버전입니다.
문법책이나 실전 코드 설계에 바로 넣기 좋게 구성.

## 📘 Rayon 핵심 기능 정리

### ✅ Rayon 병렬 반복자 핵심 기능
| 기능              | 반환 타입         | 예시 코드                                  |
|-------------------|-------------------|--------------------------------------------|
| `par_iter()`      | `ParallelIterator`| `vec.par_iter().for_each(|x| ...)`         |
| `par_iter_mut()`  | `ParallelIterator`| `vec.par_iter_mut().for_each(|x| *x += 1)` |
| `into_par_iter()` | `ParallelIterator`| `vec.into_par_iter().for_each(...)`        |


### ✅ 병렬 분할 처리
| 기능                | 분할 단위 | 예시 코드                                  |
|---------------------|------------|---------------------------------------------|
| `par_chunks(n)`     | `n`개씩    | `vec.par_chunks(2).for_each(...)`           |
| `par_chunks_mut(n)` | `n`개씩    | `vec.par_chunks_mut(2).for_each(...)`       |
| `par_windows(n)`    | 슬라이딩   | `vec.par_windows(2).for_each(...)`          |


### ✅ 병렬 map/reduce/filter
| 기능      | 반환 타입         | 예시 코드                                         |
|-----------|-------------------|--------------------------------------------------|
| `map()`   | `ParallelIterator`| `vec.par_iter().map(|x| x * 2)`                  |
| `filter()`| `ParallelIterator`| `vec.par_iter().filter(|x| x % 2 == 0)`          |
| `reduce()`| 값                | `vec.par_iter().reduce(|| 0, |a, b| a + b)`      |


### ✅ 인덱스 기반 병렬 처리
| 기능                    | 활용 방식        | 예시 코드                                         |
|-------------------------|------------------|--------------------------------------------------|
| `IndexedParallelIterator` | `enumerate`, `zip` | `vec.par_iter().enumerate().for_each(...)`       |


### ✅ 병렬 정렬
| 기능            | 비교 방식       | 예시 코드                                      |
|-----------------|------------------|------------------------------------------------|
| `par_sort()`     | 기본 정렬        | `vec.par_sort()`                               |
| `par_sort_by()`  | 커스텀 비교 함수 | `vec.par_sort_by(|a, b| a.cmp(b))`             |

### ✅ 병렬 collect
| 기능        | 반환 타입         | 예시 코드                                                           |
|-------------|-------------------|----------------------------------------------------------------------|
| `collect()` | 컬렉션 (예: Vec)  | `let doubled: Vec<_> = vec.par_iter().map(|x| x * 2).collect();`     |



## 💬 참고 사항
- Rayon은 Send + Sync 타입만 병렬 처리 가능
- 내부적으로 thread pool을 사용하며, 자동으로 작업을 분할
- par_iter()는 slice, Vec, HashMap, BTreeMap 등 다양한 컬렉션에서 사용 가능

--- 

# 샘플 예제
아래는 Rayon의 주요 기능들 각각에 대해 독립적인 main() 함수 샘플을 만들어 정리한 문서입니다.
각 예제는 바로 실행 가능하며, 문법책이나 실전 프로젝트로 제작.

## ✅ 1. par_iter()
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    data.par_iter().for_each(|x| {
        println!("Value: {}", x);
    });
}
```


## ✅ 2. par_iter_mut()
```rust
use rayon::prelude::*;
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    data.par_iter_mut().for_each(|x| {
        *x *= 2;
    });
    println!("Doubled: {:?}", data);
}
```


## ✅ 3. into_par_iter()
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![10, 20, 30];
    data.into_par_iter().for_each(|x| {
        println!("Owned: {}", x);
    });
}
```


## ✅ 4. par_chunks(n)
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];
    data.par_chunks(2).for_each(|chunk| {
        println!("Chunk: {:?}", chunk);
    });
}
```


## ✅ 5. par_chunks_mut(n)
```rust
use rayon::prelude::*;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 6];
    data.par_chunks_mut(2).for_each(|chunk| {
        chunk[0] += 100;
    });
    println!("Modified: {:?}", data);
}
```


## ✅ 6. par_windows(n)
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    data.par_windows(2).for_each(|window| {
        println!("Window: {:?}", window);
    });
}
```


## ✅ 7. map()
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3];
    let doubled: Vec<_> = data.par_iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
}
```


## ✅ 8. filter()
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let even: Vec<_> = data.par_iter().filter(|x| *x % 2 == 0).collect();
    println!("Even: {:?}", even);
}
```


## ✅ 9. reduce()
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let sum = data.par_iter().cloned().reduce(|| 0, |a, b| a + b);
    println!("Sum: {}", sum);
}
```


## ✅ 10. enumerate() (IndexedParallelIterator)
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![10, 20, 30];
    data.par_iter().enumerate().for_each(|(i, x)| {
        println!("Index {} → Value {}", i, x);
    });
}
```


## ✅ 11. zip() (IndexedParallelIterator)
```rust
use rayon::prelude::*;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    a.par_iter().zip(b.par_iter()).for_each(|(x, y)| {
        println!("Pair: {} + {} = {}", x, y, x + y);
    });
}
```


## ✅ 12. par_sort()
```rust
use rayon::prelude::*;

fn main() {
    let mut data = vec![5, 3, 1, 4, 2];
    data.par_sort();
    println!("Sorted: {:?}", data);
}

```

## ✅ 13. par_sort_by()
```rust
use rayon::prelude::*;

fn main() {
    let mut data = vec!["apple", "banana", "pear"];
    data.par_sort_by(|a, b| b.cmp(a)); // reverse sort
    println!("Sorted: {:?}", data);
}

```

## ✅ 14. collect()
```rust
use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3];
    let squared: Vec<_> = data.par_iter().map(|x| x * x).collect();
    println!("Squared: {:?}", squared);
}
```

---

# 병렬화
Rayon은 Rust의 iterator 시스템을 그대로 확장해서 병렬 처리에 적용하기 때문에,  
par_iter()만 바꾸면 기존 Vec 코드와 거의 똑같은 문법으로 병렬화를 할 수 있습니다.

## 🔁 비교 예시: 일반 vs 병렬
### 일반 반복자
```rust
let data = vec![1, 2, 3];
for x in data.iter() {
    println!("{}", x);
}
```

### 병렬 반복자 (Rayon)
```rust
use rayon::prelude::*;

let data = vec![1, 2, 3];
data.par_iter().for_each(|x| {
    println!("{}", x);
});
```

- iter() → par_iter()
- for → for_each()
- 나머지는 거의 동일

### ✅ 왜 이렇게 설계됐을까?
| 설계 철학           | 설명                                                                 |
|---------------------|----------------------------------------------------------------------|
| 문법 일관성         | 기존 `iter()` 기반 코드와 거의 동일한 문법으로 병렬화 가능             |
| 학습 곡선 최소화     | 기존 iterator 패턴을 그대로 확장하므로 초보자도 쉽게 병렬 처리 적용 가능 |
| 안전한 병렬 처리     | Rust의 타입 시스템과 borrow checker를 활용해 데이터 경쟁 없이 병렬화     |
| 자동 스레드 분배     | 내부 thread pool이 작업을 자동 분산하여 성능 최적화와 개발 편의성 제공   |
| 고성능 추상화       | 낮은 수준의 스레드 관리 없이도 높은 병렬 성능을 얻을 수 있음             |


## 💬 결론
Rayon은 Rust의 iterator를 병렬로 확장한 것이기 때문에  
Vec을 다루는 기존 코드에 par_만 붙이면 병렬화가 되는 구조.
