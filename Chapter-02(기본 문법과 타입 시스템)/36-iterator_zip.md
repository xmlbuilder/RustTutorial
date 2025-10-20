# zip
Rust의 .zip()은 두 개의 반복자(iterator)를 **쌍(pair)** 으로 묶어서 동시에 순회할 수 있게 해주는 아주 유용한 메서드입니다. 

## 🔗 .zip()이란?
`.zip()` 은 두 개의 반복자를 묶어서 튜플 (a, b) 형태로 반환하는 반복자를 생성합니다.
```rust
let xs = vec![1, 2, 3];
let ys = vec!['a', 'b', 'c'];

for (x, y) in xs.iter().zip(ys.iter()) {
    println!("x = {}, y = {}", x, y);
}
```

### 출력:
```
x = 1, y = a
x = 2, y = b
x = 3, y = c
```


## 🧠 핵심 특징
| 특징 항목         | 설명                                      |
|------------------|-------------------------------------------|
| 반환 형태        | `(x, y)` 형태의 튜플 이터레이터            |
| 길이 기준        | 짧은 쪽 이터레이터가 끝나면 전체 종료      |
| 입력 이터레이터  | `.iter()`, `.into_iter()` 모두 사용 가능   |
| 조합 가능 메서드 | `.map()`, `.filter()`, `.enumerate()` 등과 체이닝 가능 |
| 내부 동작        | 두 이터레이터의 `next()`를 동시에 호출함   |



## 🔍 예제: 좌표 처리
```rust
let xs = vec![1.0, 2.0, 3.0];
let ys = vec![4.0, 5.0, 6.0];

let points: Vec<(f64, f64)> = xs.iter().zip(ys.iter())
    .map(|(x, y)| (*x, *y))
    .collect();
```

### 결과:
```
points == vec![(1.0, 4.0), (2.0, 5.0), (3.0, 6.0)]
```


### ⚠️ 주의할 점
- zip은 짧은 쪽에 맞춰서 반복을 멈춥니다.
```rust
let a = vec![1, 2];
let b = vec![10, 20, 30];
for (x, y) in a.iter().zip(b.iter()) {
    println!("{x}, {y}");
}

// 출력: 1, 10 / 2, 20 (30은 무시됨)
```
- .iter()를 쓰면 참조(&T)를 얻고, .into_iter()를 쓰면 값(T)을 얻습니다.

## ✨ 실전 팁
- .zip()은 enumerate()와 함께 쓰면 인덱스까지 같이 처리할 수 있음:
```rust
for (i, (x, y)) in xs.iter().zip(ys.iter()).enumerate() {
    println!("Index {i}: x = {x}, y = {y}");
}
```
---

# zip 과 Iterator 결합

 .zip()은 Rust의 Iterator 트레이트에서 제공하는 아주 강력한 메서드 중 하나. 
 이걸 다른 메서드들과 조합하거나 내부 구현까지 이해하면, Rust의 이터레이터 시스템을 훨씬 깊게 활용할 수 있음.

## 🔗 .zip() 기본 개념
```rust
fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter>
where
    Self: Sized,
    U: IntoIterator,
```

- 두 이터레이터를 병렬로 묶어서 (A, B) 형태의 튜플을 생성합니다.
- 짧은 쪽 이터레이터가 끝나면 전체가 종료됩니다.
```rust
let a = [1, 2, 3];
let b = ["a", "b"];

for (num, ch) in a.iter().zip(b.iter()) {
    println!("{num} - {ch}");
}
// 출력: 1 - a, 2 - b
```


## 🧪 다른 메서드들과 조합 예시

### 1️⃣ .zip().map()
```rust
let a = [1, 2, 3];
let b = [4, 5, 6];

let summed: Vec<_> = a.iter()
    .zip(b.iter())
    .map(|(x, y)| x + y)
    .collect();

println!("{:?}", summed); // [5, 7, 9]
```

### 2️⃣ .zip().filter()
```rust
let a = [1, 2, 3, 4];
let b = [10, 20, 30, 40];

let filtered: Vec<_> = a.iter()
    .zip(b.iter())
    .filter(|(x, y)| x % 2 == 0 && y > &20)
    .collect();

println!("{:?}", filtered); // [(2, 20), (4, 40)]
```


### 3️⃣ .zip().enumerate()
```rust
let a = ["x", "y", "z"];
let b = [100, 200, 300];

for (i, (ch, val)) in a.iter().zip(b.iter()).enumerate() {
    println!("{}: {} - {}", i, ch, val);
}
```


## 🔍 내부 구현: Zip 구조체
Rust의 표준 라이브러리에는 Zip이라는 구조체가 정의:
```rust
pub struct Zip<A, B> {
    a: A,
    b: B,
}
```

그리고 impl Iterator for Zip<A, B>가 구현되어 있습니다:
```rust
impl<A, B> Iterator for Zip<A, B>
where
    A: Iterator,
    B: Iterator,
{
    type Item = (A::Item, B::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}
```

### ✅ 핵심 포인트
- zip은 두 이터레이터의 next()를 동시에 호출해서 Some((a, b))를 반환
- 하나라도 None이면 종료
- fused 이터레이터가 아니면, 이후 next() 호출 시 동작이 보장되지 않음

## 🧠 고급 활용: .zip() + .fold()
```rust
let a = [1, 2, 3];
let b = [4, 5, 6];

let dot_product = a.iter()
    .zip(b.iter())
    .fold(0, |acc, (x, y)| acc + x * y);

println!("Dot product: {}", dot_product); // 1*4 + 2*5 + 3*6 = 32
```


## 🧬 이터레이터 체이닝의 철학
Rust의 이터레이터는 lazy evaluation을 기반으로 하며, .zip()은 그 중에서도 병렬 처리를 위한 핵심 도구입니다.  
다른 메서드들과 조합하면 데이터 흐름을 함수형 스타일로 표현할 수 있어요.

## 🔍 .into_iter()의 소유권 이전 여부
| 표현        | 반복되는 타입 | 소유권 이전 여부 | 설명                                |
|-------------|----------------|------------------|-------------------------------------|
| `Vec<T>`    | `T`            | ✅ 발생함         | 각 요소의 소유권을 가져옴           |
| `&Vec<T>`   | `&T`           | ❌ 발생하지 않음  | 참조만 반복, 원본은 유지됨          |
| `[T; N]`    | `T`            | ✅ 발생함         | 배열의 각 요소를 move함             |
| `&[T]`      | `&T`           | ❌ 발생하지 않음  | 슬라이스 참조만 반복                |



## ✅ 예시 1: 소유권 이전 발생
```rust
let v = vec![String::from("a"), String::from("b")];

for s in v.into_iter() {
    println!("{}", s); // s는 String
}
```
// v는 여기서 move됨 → 이후 사용 불가


- v.into_iter()는 Vec<String>의 각 요소 String을 소유권을 넘겨서 반복합니다.
- 이후 v는 더 이상 사용할 수 없습니다.

## ❌ 예시 2: 참조만 반복 (소유권 유지)
```rust
let v = vec![String::from("a"), String::from("b")];

for s in (&v).into_iter() {
    println!("{}", s); // s는 &String
}
```
// v는 여전히 사용 가능


- &v.into_iter()는 &Vec<String>에 대해 호출되므로 참조만 반복합니다.
- v의 소유권은 유지됩니다.

## 🧠 핵심 요약
| 표현              | 반복되는 타입 | 소유권 이전 여부 |
|-------------------|---------------|------------------|
| `v.into_iter()`   | `T`           | ✅ 소유권 이전됨 |
| `v.iter()`        | `&T`          | ❌ 참조만 반환됨 |
| `(&v).into_iter()`| `&T`          | ❌ 참조만 반환됨 |



## ✨ 실전 팁
- into_iter()는 소유권을 넘기고 싶을 때 사용합니다. 예: Vec<String>을 String으로 소비할 때
- iter()는 읽기 전용 참조로 순회할 때 사용합니다.
- iter_mut()는 가변 참조로 순회할 때 사용합니다.


---
# let filtered: `Vec<_>` 의미

let filtered: `Vec<_>` 는 Rust에서 **타입 추론(type inference)** 을 활용한 변수 선언 방식입니다. 

## 🧠 `Vec<_>` 란?
- `Vec<_>` 에서 _는 타입을 자동으로 추론해달라는 뜻입니다.
- Rust 컴파일러가 오른쪽 값을 보고 Vec<T>의 T가 무엇인지 자동으로 결정합니다.

## 🔍 예시로 보면
```rust
let filtered: Vec<_> = vec![1, 2, 3, 4, 5]
    .into_iter()
    .filter(|x| x % 2 == 0)
    .collect();
```


- filter는 `Iterator<Item = i32>` 를 반환하고,
- `collect()` 는 `Vec<i32>` 를 생성합니다.
- 따라서 `Vec<_>` 는 자동으로 `Vec<i32>` 로 추론됩니다.

## ✅ 왜 쓰는가?
| 항목           | 설명                                         |
|----------------|----------------------------------------------|
| 코드 간결성    | 타입을 명시하지 않아도 되므로 짧고 깔끔함     |
| 추론 가능할 때 | 컴파일러가 오른쪽 값을 보고 자동으로 결정함   |
| 유연성         | 복잡한 제네릭 타입을 직접 쓰지 않아도 됨     |
| `_`의 의미     | "타입을 알아서 추론해줘"라는 컴파일러 지시어 |


## ⚠️ 주의할 점
- 타입이 불명확하거나 ambiguous할 경우 컴파일 에러가 날 수 있어요.
- 예: collect()가 Vec<_>인지 HashSet<_>인지 모호할 경우
```rust
let result = some_iter.collect(); // 타입 명시 없으면 에러 가능
```

→ 이런 경우엔 Vec<_> 또는 HashSet<_>처럼 타입을 일부라도 명시해줘야 함.

## ✨ 실전 팁
- Vec<_>는 map, filter, zip, enumerate, flat_map 등과 함께 자주 쓰입니다.
- let x: Vec<_>는 let x = vec![...]보다 더 유연한 추론을 허용합니다.

