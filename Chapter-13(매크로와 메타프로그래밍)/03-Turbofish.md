# Turbofish
**터보피시(Turbofish)**는 Rust의 타입 시스템에서 아주 중요한 역할을 하는 문법.  
특히 collect()나 parse()처럼 제네릭 타입을 반환하는 메서드에서 컴파일러가 타입을 추론하지 못할 때, 우리가 직접 타입을 지정해주는 방식입니다.

## 🐟 터보피시란?
형태: identifier::<Type>
예시: collect::<Vec<i32>>(), parse::<i32>()
- ::<> 이 괄호 모양이 마치 물고기 같다고 해서 “Turbofish”라는 별명이 붙었어요.
- Rust는 타입 추론이 강력하지만, 제네릭 반환 타입이 모호할 때는 우리가 직접 타입을 지정해야 합니다.
- 장점: 컴파일러가 타입을 추론하지 못할 때 명확한 타입 지정으로 오류 방지


## 🧠 타입 추론 vs 명시적 타입 지정
- Rust는 대부분의 경우 타입을 자동으로 추론하지만,
- HashMap::new()처럼 빈 컬렉션을 생성할 때는 타입을 알 수 없기 때문에 명시적 타입 지정이 필요
```rust
let mut students: HashMap<&str, i32> = HashMap::new();
```
- 또는 터보피시를 사용해서 타입을 지정할 수도 있음:
```rust
let mut students: HashMap = HashMap::<&str, i32>::new();
```


## 🔍 코드에서 터보피시가 필요한 이유
```rust
["1", "2", "three"]
    .iter()
    .filter_map(|x| x.parse().ok())
    .collect::<Vec<i32>>();
```

- x.parse()는 Result<T, E>를 반환하는데, T가 무엇인지 컴파일러가 추론할 수 없을 때 문제가 생깁니다.
- collect()는 FromIterator를 구현한 타입으로 변환하는데, 이때도 어떤 타입으로 수집할지 명확하지 않으면 컴파일 에러가 발생합니다.

## ✅ 해결 방법: 터보피시로 타입 명시
```rust
let nums = ["1", "2", "three"]
    .iter()
    .filter_map(|x| x.parse::<i32>().ok()) // parse::<i32>()로 명시
    .collect::<Vec<i32>>();                // collect::<Vec<i32>>()로 명시

```
- parse::<i32>()는 문자열을 i32로 파싱하겠다는 뜻
- collect::<Vec<i32>>()는 결과를 Vec<i32>로 수집하겠다는 뜻


## 🔁 제네릭 함수와 터보피시 활용 예시
- double<T>(Vec<T>) -> impl Iterator<Item = T> 같은 제네릭 함수에서
- 반환된 반복자에 대해 collect::<Vec<i32>>() 또는 collect::<Vec<String>>()처럼 터보피시로 타입 지정 가능
- 이는 Rust의 타입 안정성과 유연한 제네릭 처리를 동시에 보여주는 예시



🧠 터보피시가 필요한 대표적인 상황
| 메서드             | 설명                                                         |
|--------------------|--------------------------------------------------------------|
| `parse()`          | 어떤 타입으로 파싱할지 명시해야 함 (`parse::<i32>()`)         |
| `collect()`        | 어떤 컬렉션으로 수집할지 명시해야 함 (`collect::<Vec<T>>()`)  |
| `into()`           | 어떤 타입으로 변환할지 명시해야 함 (`into::<T>()`)            |
| `unwrap_or_else()` | 반환 타입이 모호할 때 클로저의 반환 타입을 명시해야 함        |


## 🧪 실전 예시
```rust
fn main() {
    let nums: Vec<i32> = ["1", "2", "three"]
        .iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    println!("{:?}", nums); // 출력: [1, 2]
}
```

- "three"는 파싱 실패 → None → filter_map에서 제거됨
- nums.contains(&1)은 true 반환


## ✅ 터보피시 요약

| 문법               | 설명                                      |
|--------------------|-------------------------------------------|
| `parse::<T>()`     | 문자열을 T 타입으로 파싱                  |
| `collect::<T>()`   | 반복자 결과를 T 타입으로 수집             |
| `into::<T>()`      | 값을 T 타입으로 변환                      |
| 사용 시점          | 컴파일러가 타입을 추론하지 못할 때 필요   |
---





