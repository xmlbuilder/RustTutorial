# Iterator

Rust에서 Iterator 트레이트는 매우 핵심적인 개념입니다.  
데이터를 반복적으로 처리할 수 있게 해주며, 안전성과 성능을 모두 만족시키는 방식으로 설계.  
여기에 수명(Lifetime) 개념도 깊게 연결되어 있어서, Rust의 메모리 안전성을 유지하는 데 중요한 역할을 합니다.


## 🧭 왜 Iterator 트레이트를 사용할까?
### 1. 반복 처리의 표준화
- Iterator 트레이트를 구현하면 for 루프, .map(), .filter() 등 다양한 고수준 API를 사용할 수 있어요.
- 반복자 기반의 처리 방식은 lazy evaluation을 지원해 성능도 좋습니다.
### 2. 메모리 안전성과 제어력
- Rust는 명시적인 제어를 통해 반복 중에 소유권, 수명, 불변/가변 접근을 안전하게 관리합니다.
- C++의 반복자보다 훨씬 안전하고 예측 가능해요.
### 3. 추상화와 재사용성
- Iterator를 구현하면 다양한 알고리즘에 재사용할 수 있어요.
- 예: collect(), fold(), find(), take_while() 등

## 🔧 어떻게 사용하는가?
###  기본 구조

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

- next()는 반복자의 핵심 메서드로, 다음 값을 Some(value)로 반환하거나 끝났으면 None을 반환합니다.

### 예제: 직접 구현
```rust
struct Counter {
    count: usize,
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut c = Counter { count: 0 };
    for val in c {
        println!("{}", val); // 1, 2, 3, 4, 5
    }
}
```


## 🧬 수명(Lifetime)과의 관계
### 1. 참조 기반 반복자
```rust
fn print_all(v: &Vec<String>) {
    for s in v.iter() {
        println!("{}", s);
    }
}
```

- v.iter()는 &String을 반복하므로, s의 수명은 v의 수명에 종속됩니다.
- Rust는 이 관계를 수명 파라미터로 추론하거나 명시하게 합니다.
### 2. Iterator 트레이트와 수명
```rust
impl<'a> Iterator for MyIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // 반환되는 참조의 수명은 'a
    }
}
```


- Item이 참조 타입일 경우, 반드시 수명 'a를 명시해야 합니다.
- 이는 반환값이 원본 데이터보다 오래 살아남지 않도록 보장하는 역할을 해요.

## 🧪 고급 활용: map, filter, collect
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let squared: Vec<_> = v.iter().map(|x| x * x).collect();
    println!("{:?}", squared); // [1, 4, 9, 16, 25]
}
```

- iter()는 &i32를 반복
- map()은 클로저를 적용
- collect()는 결과를 새로운 벡터로 수집

| 개념        | 설명 또는 반환값             |
|-------------|------------------------------|
| `Iterator`  | 반복자 트레이트, `next()` 메서드 필요 |
| `next()`    | `Option<Item>` 반환 (`Some` 또는 `None`) |
| `'a`        | 참조 기반 반복자에서 수명 지정 |
| 주요 메서드 | `map`, `filter`, `collect`, `fold`, `take`, `skip` 등 |


---


## 🔍 핵심 차이 요약
| 트레이트       | 핵심 메서드       | 설명                                |
|----------------|-------------------|-------------------------------------|
| `Iterator`     | `next()`          | 반복자 자체를 정의, 값을 하나씩 반환 |
| `IntoIterator` | `into_iter()`     | 반복 가능한 객체를 반복자로 변환     |


## 🧠 비유로 이해하기
- IntoIterator: "나는 반복될 수 있어요!" → 반복 가능한 컨테이너 (예: Vec, HashMap)
- Iterator: "나는 반복 중이에요!" → 실제로 next()를 호출할 수 있는 반복자

## 🧪 예제 비교
```rust
fn main() {
    let v = vec![1, 2, 3];

    // IntoIterator: Vec 자체가 반복 가능한 객체
    let iter1 = v.into_iter(); // v는 move됨
    for i in iter1 {
        println!("IntoIterator: {}", i);
    }

    let v2 = vec![4, 5, 6];

    // Iterator: 참조 기반 반복자
    let iter2 = v2.iter(); // v2는 그대로 유지됨
    for i in iter2 {
        println!("Iterator: {}", i);
    }
}
```

### 결과:
```
IntoIterator: 1
IntoIterator: 2
IntoIterator: 3
Iterator: 4
Iterator: 5
Iterator: 6
```


## 📦 구현 관점에서의 차이
### IntoIterator 트레이트
```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

- self를 소비해서 반복자를 생성
- for 루프에서 가장 먼저 호출되는 트레이트
### Iterator 트레이트
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

- 반복자의 핵심: next() 호출로 값을 하나씩 반환

## 🔁 for 루프에서의 동작 흐름
```rust
for x in collection {
    // 내부적으로는 이렇게 동작함:
    let mut iter = IntoIterator::into_iter(collection);
    while let Some(x) = iter.next() {
        // 반복 처리
    }
}
```

- 즉, for 루프는 IntoIterator를 먼저 호출해서 Iterator를 얻고, 그걸로 반복을 수행합니다.

## 🧬 수명과의 관계
- IntoIterator는 self를 소비하므로 소유권이 중요합니다.
- Iterator는 &self 또는 &mut self를 기반으로 할 수 있어 **수명 'a**가 자주 등장합니다.
- 예: fn iter(&'a self) -> impl Iterator<Item = &'a T>

## ✅ 요약 표 (Markdown)
| 트레이트       | 역할                          | 사용 예시             |
|----------------|-------------------------------|------------------------|
| `IntoIterator` | 반복 가능한 객체를 정의       | `vec.into_iter()`     |
| `Iterator`     | 반복자 자체를 정의 (`next()`) | `vec.iter()`          |

---

## 🧪 예제: 참조 기반 반복자와 수명 'a
```rust
struct StrIter<'a> {
    data: &'a [&'a str], // 슬라이스 내부의 참조도 `'a` 수명
    index: usize,
}

impl<'a> Iterator for StrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let result = self.data[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let words = vec!["hello", "world", "rust"];
    let iter = StrIter {
        data: &words,
        index: 0,
    };

    for word in iter {
        println!("{}", word);
    }
}
```


## 🔍 설명
- StrIter<'a>는 'a라는 수명을 가진 참조 슬라이스를 저장합니다.
- Iterator 구현 시 type Item = &'a str로 명시하여, 반환되는 값의 수명이 원본 데이터(words)와 같음을 보장합니다.
- 이 구조 덕분에 반환된 참조가 원본보다 오래 살아남지 않도록 컴파일 타임에 체크됩니다.

## ✅ 수명 'a가 필요한 이유
- data: &'a [&'a str]에서 두 개의 'a는 다음을 의미합니다:
- 첫 번째 'a: 슬라이스 자체의 수명
- 두 번째 'a: 슬라이스 안의 각 &str의 수명
- next()에서 반환하는 &'a str은 data의 요소를 그대로 반환하므로, 반환값의 수명은 원본 참조와 같아야 안전합니다.



## 🔍 왜 둘 다 'a가 필요한가?
```rust
data: &'a [&'a str]
```

이 표현은 다음 두 가지를 의미합니다:
- &'a [...]: 슬라이스 자체가 'a 동안 유효한 참조
- &'a str: 슬라이스 안의 각 요소도 'a 동안 유효한 참조
즉, 슬라이스도 'a 수명을 가져야 하고, 그 안에 있는 각 &str도 'a 수명을 가져야 전체적으로 안전한 참조가 됩니다.

## 🧪 예제로 확인
```rust
fn main() {
    let words = vec!["hello", "world"];
    let slice: &[&str] = &words;

    let iter = StrIter { data: slice, index: 0 };
}
```

이때 StrIter의 정의는 다음과 같아야 합니다:
```rust
struct StrIter<'a> {
    data: &'a [&'a str],
    index: usize,
}
```

- 첫 번째 'a: data라는 슬라이스 참조 자체의 수명
- 두 번째 'a: 슬라이스 내부의 각 &str의 수명

## ✅ 만약 생략하면?
```rust
struct StrIter<'a> {
    data: &'a [&str], // 내부 요소의 수명 생략
}
```

이렇게 하면 Rust는 내부 &str의 수명을 'static으로 추론할 수도 있고,    
컴파일러가 수명 충돌을 감지하지 못할 수도 있음.    
특히 반환값에 참조를 넘길 때 문제가 생깁니다.  

| 표현            | 수명 해석                          |
|-----------------|------------------------------------|
| `&'a [&'a str]` | 슬라이스와 내부 요소 모두 `'a` 수명 |
| `&'a [&str]`    | 슬라이스는 `'a`, 내부 요소는 추론됨 |
| `&[&'a str]`    | 슬라이스는 `'static`일 수도 있음    |


----


## 🔍 왜 소유권이 이동되는가?
```rust
let v = vec![1, 2, 3];
for x in v.into_iter() {
    println!("{}", x);
}
// 여기서 v는 더 이상 사용할 수 없음
```

- into_iter()는 Vec<T>의 소유권을 소비합니다.
- 반복자 내부에서 T 값을 직접 꺼내기 때문에, 원래 벡터는 move되어 버림.
- 그래서 이후에 v를 다시 쓰면 컴파일 에러가 납니다.

## ✅ 해결 방법: 참조 기반 반복
### 1. iter() 사용 (불변 참조)
```rust
let v = vec![1, 2, 3];
for x in v.iter() {
    println!("{}", x); // x는 &i32
}
println!("{:?}", v); // OK!
```

- iter()는 &T를 반환하므로 소유권을 이동하지 않음
- 원본 벡터를 반복 후에도 계속 사용할 수 있어요
### 2. iter_mut() 사용 (가변 참조)
```rust
let mut v = vec![1, 2, 3];
for x in v.iter_mut() {
    *x *= 2;
}
println!("{:?}", v); // [2, 4, 6]
```

- iter_mut()는 &mut T를 반환
- 반복하면서 값을 수정할 수 있음

## 🧬 IntoIterator의 다양한 구현
Rust는 IntoIterator를 소유권 버전과 참조 버전으로 각각 구현:
| 타입            | 호출 방식       | 반환 타입 | 소유권 이동 여부 | 반복 후 사용 가능 |
|-----------------|-----------------|------------|------------------|-------------------|
| `Vec<T>`        | `into_iter()`   | `T`        | ✅ 이동됨         | ❌ 불가능          |
| `&Vec<T>`       | `.into_iter()`  | `&T`       | ❌ 이동 안 됨     | ✅ 가능            |
| `&mut Vec<T>`   | `.into_iter()`  | `&mut T`   | ❌ 이동 안 됨     | ✅ 가능            |


## 🔁 for 루프에서 자동 선택
```rust
let v = vec![1, 2, 3];
for x in &v {
    println!("{}", x); // 내부적으로 &v.into_iter() 호출됨
}
```

- for 루프는 타입에 따라 자동으로 적절한 IntoIterator 구현을 선택합니다.
- &v → &Vec<T> → IntoIterator → &T 반환

📌 요약
| 반복 방식     | 소유권 이동 | 반복 후 사용 가능 | 반환 타입 |
|---------------|-------------|-------------------|------------|
| `into_iter()` | ✅ 이동됨   | ❌ 불가능         | `T`        |
| `iter()`      | ❌ 참조     | ✅ 가능           | `&T`       |
| `iter_mut()`  | ❌ 참조     | ✅ 가능           | `&mut T`   |


Iterator는 반복자 자체를 정의하는 단일 트레이트인 반면, IntoIterator는 다양한 타입에 대해 반복자를 생성할 수 있도록 여러 구현을 제공

## 🔍 핵심 개념 비교
| 트레이트       | 핵심 메서드 | 구현 예시                        |
|----------------|-------------|----------------------------------|
| `Iterator`     | `next()`    | `VecIter`, `Range`, `StrIter`   |
| `IntoIterator` | `into_iter()`| `Vec<T>`, `&Vec<T>`, `&mut Vec<T>` |


## 🧠 왜 IntoIterator는 여러 구현을 제공할까?
Rust는 하나의 타입에 대해 여러 방식의 반복을 허용합니다.  
예를 들어 Vec<T>는 다음 세 가지 방식으로 반복할 수 있음:
```rust
let v = vec![1, 2, 3];

// 1. 소유권 이동
for x in v.into_iter() {
    println!("T: {}", x); // x는 T
}

// 2. 불변 참조
for x in (&v).into_iter() {
    println!("&T: {}", x); // x는 &T
}

// 3. 가변 참조
for x in (&mut v).into_iter() {
    println!("&mut T: {}", x); // x는 &mut T
}
```

- 이처럼 IntoIterator는 타입에 따라 다르게 구현되어 있고,
- for 루프는 자동으로 적절한 구현을 선택합니다.

## 🔧 반면 Iterator는 단일 구현
```rust
struct Counter {
    count: usize,
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

- Iterator는 반복자 자체를 정의하는 트레이트이기 때문에 하나의 타입에 하나의 구현만 존재합니다.
- next()를 직접 구현해야 하고, 반복의 내부 동작을 책임짐.

## ✅ 요약
| 트레이트       | 특징                                |
|----------------|-------------------------------------|
| `Iterator`     | 반복자 자체를 정의, 단일 구현       |
| `IntoIterator` | 반복 가능한 객체, 다양한 구현 가능 |

----


## 🧪 예제: 사용자 정의 타입에 IntoIterator 구현
```rust
struct Words {
    items: Vec<String>,
}

// 반복자 정의
struct WordsIter {
    index: usize,
    items: Vec<String>,
}

// Iterator 구현
impl Iterator for WordsIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let result = self.items[self.index].clone();
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

// IntoIterator 구현
impl IntoIterator for Words {
    type Item = String;
    type IntoIter = WordsIter;

    fn into_iter(self) -> Self::IntoIter {
        WordsIter {
            index: 0,
            items: self.items,
        }
    }
}

fn main() {
    let words = Words {
        items: vec!["hello".into(), "rust".into(), "world".into()],
    };

    for word in words {
        println!("{}", word);
    }
}
```

---


## 🔍 핵심 포인트
- Words는 반복 가능한 객체 (IntoIterator 구현)
- WordsIter는 실제 반복자 (Iterator 구현)
- into_iter()는 Words를 소비하고 WordsIter를 반환
- for word in words는 내부적으로 words.into_iter()를 호출

### ✅ 수명 관련 참고
이 예제는 String을 반환하므로 수명 'a가 필요하지 않지만, 만약 &str을 반환하려면 수명 파라미터를 명시해야 합니다:
```rust
impl<'a> IntoIterator for &'a Words {
    type Item = &'a str;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter().map(|s| s.as_str()).collect::<Vec<_>>().iter()
    }
}
```


## 🔗 구조 이해: IntoIterator → Iterator
```rust
// IntoIterator는 반복자를 생성하는 트레이트
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

- into_iter()는 반복자를 반환할 뿐, 직접 next()를 호출하거나 반복하지 않음.
- 반환된 IntoIter 타입은 반드시 Iterator를 구현해야 반복이 가능합니다.

## 🧪 예제: IntoIterator만으로는 반복 불가
```rust
struct MyCollection {
    data: Vec<i32>,
}

impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

fn main() {
    let coll = MyCollection { data: vec![1, 2, 3] };

    // 이건 가능: for 루프가 내부적으로 into_iter() 호출 후 Iterator 사용
    for x in coll {
        println!("{}", x);
    }

    // 이건 불가능: IntoIterator 자체는 next()가 없음
    // let mut iter = coll; // ❌ error: no method `next` found
}
```


## ✅ 핵심 요약
| 트레이트       | 역할                          | 직접 반복 가능? |
|----------------|-------------------------------|------------------|
| `IntoIterator` | 반복자를 생성하는 입구 역할   | ❌ 불가능         |
| `Iterator`     | 실제 반복 동작을 수행 (`next`) | ✅ 가능           |


## 🔍 비유로 이해
- IntoIterator: "나는 반복자를 만들어줄게!"
- Iterator: "내가 직접 반복할게!"
즉, IntoIterator는 반복을 시작할 수 있게 해주는 도어맨, Iterator는 반복을 실제로 수행하는 배달원 역할.




