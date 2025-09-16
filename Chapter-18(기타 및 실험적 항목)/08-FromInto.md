# Iterator & Lifetime
Rust의 반복자 시스템은 단순한 루프를 넘어서 메모리 안전성과 추상화의 정수를 보여주는 구조. 
특히 Lifetime과 Iterator를 직접 구현하는 예제는 Rust의 핵심 철학를 정리.

## 🧠 핵심 개념 요약
| 개념        | 핵심 요소 또는 설명                                 |
|-------------|------------------------------------------------------|
| `Iterator`  | `next()` 메서드를 통해 순차적으로 값을 반환           |
| `Lifetime`  | 참조의 유효 범위를 명시하여 메모리 안전성 확보        |
| 반복자 어댑터 | `map`, `filter`, `zip`, `collect`, `sum`, `next()` 등 |
| 소비 어댑터 | `map`, `filter`는 게으름 / `collect`, `sum`은 소비함 |



### 📚 1. 사용자 정의 반복자 + Lifetime
```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Book {
    fn new(title: String, author: String, published: u32) -> Self {
        Book { title, author, published }
    }
}

#[derive(Debug)]
struct BookSelf<'a> {
    books: &'a [Book],
}

impl<'a> Iterator for BookSelf<'a> {
    type Item = &'a Book;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first, remainder)) = self.books.split_first() {
            self.books = remainder;
            Some(first)
        } else {
            None
        }
    }
}
```

- BookSelf<'a>는 &'a [Book]을 참조하며, next() 호출 시 하나씩 반환
- split_first()를 통해 첫 요소와 나머지를 분리
- Lifetime 'a는 반환되는 &Book이 원본 배열의 생명주기를 따름

#### 🧪 사용 예시
```rust
fn main() -> std::io::Result<()> {
    let book_array = [
        Book::new("1".into(), "1_1".into(), 1),
        Book::new("1".into(), "1_1".into(), 1),
        Book::new("1".into(), "1_1".into(), 1),
    ];

    let mut my_book_iter = BookSelf { books: &book_array };
    println!("{:?}", my_book_iter.next());
    println!("{:?}", my_book_iter.next());
    println!("{:?}", my_book_iter.next());
    println!("{:?}", my_book_iter.next()); // None

    let my_book_iter = BookSelf { books: &book_array };
    for b in my_book_iter {
        println!("{:?}", b);
    }
    Ok(())
}
```

#### 출력 결과
```
// Some(Book { title: "1", author: "1_1", published: 1 })
// Some(Book { title: "1", author: "1_1", published: 1 })
// Some(Book { title: "1", author: "1_1", published: 1 })
// None


// Book { title: "1", author: "1_1", published: 1 }
// Book { title: "1", author: "1_1", published: 1 }
// Book { title: "1", author: "1_1", published: 1 }
```

### 🔁 반복자 어댑터: map, filter, collect, sum
#### map + collect
```rust
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
```

- map은 게으르게 작동
- collect()가 호출되어야 실제로 실행됨

#### filter + 환경 캡처
```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}
```

- 클로저가 외부 변수 shoe_size를 캡처
- filter는 조건에 맞는 요소만 반환

### 🔢 사용자 정의 반복자: Counter
```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

#### 테스트
```rust
#[test]
fn counter_test() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```


### 🧮 고급 조합: zip, map, filter, sum
```rust
fn counter_sum_test() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(sum, 18);
}
```
#### 계산과정
```
1*2 : 2 %3 == 2 X
2*3 --> 6  O 
3*4 --> 12 O
4*5  X
6+12 -> 18
```


- zip은 두 반복자를 병렬로 묶음
- skip(1)은 첫 요소를 건너뜀
- map으로 곱셈, filter로 조건 필터링
- sum으로 최종 누적합 계산

📦 반복자 소비 요약

| 메서드   | 소비 여부 | 특징 또는 설명                          |
|----------|-----------|-----------------------------------------|
| `map`    | ❌         | 게으른 평가. `collect` 등으로 소비 필요 |
| `filter` | ❌         | 게으른 평가. 조건에 따라 요소 선택     |
| `collect`| ✅         | 반복자를 소비하고 결과를 수집           |
| `sum`    | ✅         | 반복자를 소비하며 누적합 계산           |
| `for`    | ✅         | 내부적으로 `next()` 호출하여 소비       |



## 🔁 반복자 확장 기능 정리
### 1️⃣ Peekable – 다음 값을 미리 볼 수 있는 반복자
```rust
let xs = [1, 2, 3];
let mut iter = xs.iter().peekable();

assert_eq!(iter.peek(), Some(&&1)); // double reference: & to &i32
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.peek(), Some(&&2));
```


- peek()은 반복자를 진행시키지 않고 다음 값을 참조
- peek_mut()으로 가변 참조도 가능
- 주의: peek()는 참조를 반환하므로 &&T 형태가 나올 수 있음

### 2️⃣ FusedIterator – 반복이 끝나면 계속 None만 반환
```rust
use std::iter::FusedIterator;

struct MyIter {
    count: u8,
}

impl Iterator for MyIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 3 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

impl FusedIterator for MyIter {} // 반복 종료 후에도 None만 반환
```

- fused()는 반복자가 종료된 후에도 다시 None을 반환하는지 보장
- 일부 반복자 어댑터(zip, chain)에서 최적화에 사용됨

### 3️⃣ rev() – 역순 반복
```rust
let v = vec![1, 2, 3];
for x in v.iter().rev() {
    println!("{}", x); // 3, 2, 1
}
```

- DoubleEndedIterator를 구현한 반복자만 사용 가능
- Vec, slice, Range 등 대부분 지원

### 4️⃣ enumerate() – 인덱스와 값 함께 반환
```rust
let v = vec!["a", "b", "c"];
for (i, val) in v.iter().enumerate() {
    println!("{}: {}", i, val);
}
```

- (usize, T) 형태로 반환
- 디버깅, UI 렌더링, 로그 출력 등에 유용

### 🧬 복잡한 Lifetime이 얽힌 구조체 설계
Rust에서 lifetime이 복잡해지는 경우는 다음과 같은 상황에서 자주 발생합니다:
### ✅ 구조체가 참조를 포함할 때
```rust
struct RefHolder<'a> {
    data: &'a str,
}
```

- 'a는 data가 유효한 범위를 나타냄
- 이 구조체는 'a 동안만 유효함

### ✅ 여러 참조가 얽힐 때
```rust
struct MultiRef<'a, 'b> {
    first: &'a str,
    second: &'b str,
}
```

- 'a와 'b는 서로 다른 lifetime을 가질 수 있음
- 함수 인자나 반환값에서 lifetime 관계를 명시해야 할 수도 있음

### ✅ 반복자와 lifetime이 얽힐 때
```rust
struct BookIter<'a> {
    books: &'a [String],
}

impl<'a> Iterator for BookIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first, rest)) = self.books.split_first() {
            self.books = rest;
            Some(first)
        } else {
            None
        }
    }
}
```

- Item = &'a T로 명시해야 반환값이 원본 참조와 연결됨
- split_first()는 slice를 나누면서 lifetime을 유지함

### ⚠️ lifetime이 꼬일 때 흔한 오류
- borrowed value does not live long enough
- cannot infer appropriate lifetime
- temporary value dropped while borrowed
이런 오류는 대부분 참조가 구조체보다 먼저 사라지거나, 반환값이 구조체보다 오래 살아야 할 때 발생. 해결하려면:
- 구조체에 lifetime 명시
- 반환값에 lifetime 연결
- 참조 대신 Arc, Rc, Box 등으로 소유권 이전 고려



## 🧙‍♂️ PhantomData<T> – 타입 정보만 보존, 실제 데이터 없음
Rust는 타입 시스템이 강력해서, 컴파일러가 타입만 보고도 많은 걸 추론합니다. 
그런데 어떤 경우엔 타입만 있고 값은 없는 구조가 필요해요. 그럴 때 쓰는 게 PhantomData<T>입니다.

### ✅ 예시: 소유권이나 lifetime을 명시하고 싶을 때
```rust
use std::marker::PhantomData;

struct MyType<T> {
    _marker: PhantomData<T>,
}
```

- T를 실제로 쓰지 않지만, 타입 시스템에선 MyType<T>로 인식됨
- PhantomData<&'a T>를 쓰면 lifetime 'a를 구조체에 명시할 수 있음
### 🔍 실전 용도
- unsafe 코드에서 drop 순서 제어
- lifetime을 명시적으로 구조체에 연결
- 제네릭 타입을 컴파일러에게 알려주기

### 🧬 GAT – Generic Associated Types
GAT는 트레이트 내부에서 제네릭 타입을 정의할 수 있게 해주는 기능이에요.
기존에는 트레이트의 type Item 같은 associated type이 고정 타입만 가능했지만, GAT는 제네릭 타입도 가능하게 해줘요.
#### ✅ 예시: GAT를 활용한 반복자 트레이트
```rust
trait StreamingIterator {
    type Item<'a>;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```

- Item<'a>는 lifetime 'a를 가진 타입
- 반복자에서 참조를 반환할 때 유용
#### 🔍 실전 용도
- async 트레이트 구현
- lifetime이 얽힌 구조에서 유연한 반환 타입
- 고급 API 설계에서 타입 추론을 더 정밀하게

### 🧠 Higher-Rank Trait Bounds (HRTB)
HRTB는 모든 lifetime에 대해 동작 가능한 트레이트 바운드를 의미해요.
즉, for<'a>는 “어떤 lifetime이든 만족해야 한다”는 뜻이에요.
#### ✅ 예시: 클로저가 모든 참조에 대해 동작할 때
```rust
fn call_with_str<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = "hello";
    println!("{}", f(s));
}
```

- for<'a>는 클로저 f가 어떤 'a에도 동작해야 함을 의미
- 일반적인 lifetime 바운드보다 훨씬 유연함
#### 🔍 실전 용도
- 클로저를 인자로 받을 때
- 트레이트 객체에서 lifetime 추론
- 고차 함수와 콜백 설계
자세한 설명은 Rustonomicon의 HRTB 문서에서 확인할 수 있어요.

### ⚡ async와 lifetime이 얽힌 구조
Rust의 async는 state machine으로 변환되기 때문에, 내부에 있는 참조는 lifetime이 **정적(static)**이어야 안전해요.  
그래서 async fn에서 참조를 반환하거나, 참조를 가진 구조체를 await할 때 lifetime 충돌이 자주 발생해요.
#### ✅ 예시: async에서 참조 반환이 어려운 이유
```rust
async fn get_ref<'a>(s: &'a str) -> &'a str {
    s // ❌ 컴파일 에러: async fn은 참조 반환에 제약이 있음
}
```

#### 🔍 해결 전략
- 참조 대신 Arc, Rc, Box로 소유권 이전
- async-trait 크레이트 사용 (단, 성능 trade-off 있음)
- GAT와 HRTB를 조합해 lifetime을 유연하게 설계

### 📦 고급 Rust 개념 요약

| 개념            | 핵심 역할                                      | 실전 용도                          |
|-----------------|------------------------------------------------|------------------------------------|
| `PhantomData<T>`| 타입/라이프타임 정보 보존 (값 없음)            | unsafe, drop 순서, 타입 추론       |
| GAT             | 트레이트 내부에서 제네릭 타입 정의              | async 트레이트, 반복자 설계        |
| HRTB            | 모든 lifetime에 대해 동작 가능한 트레이트 바운드| 클로저, 고차 함수, 트레이트 객체   |
| async + lifetime| 참조와 비동기 간의 충돌 해결                    | 소유권 이전, `async-trait` 활용    |

---



