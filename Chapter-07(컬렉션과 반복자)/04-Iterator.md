# iterator
Rust의 **이터레이터(iterator)**는 반복 작업을 안전하고 효율적으로 처리하는 핵심 기능.

## 🔁 이터레이터란?
- "이터레이터(iterator)는 반복 가능한 시퀀스를 입력으로 받아 각 원소에 특정 작업을 수행할 수 있도록 하는 기능"
- Rust에서 이터레이터는 for 루프, .map(), .filter(), .collect() 등 다양한 방식으로 활용되며, 메모리 안전성과 성능을 동시에 확보할 수 있도록 설계.

## 🧪 기본 예제: .iter() 사용
```rust
fn main() {
    let names = vec!["james", "cameron", "indo"];

    for name in names.iter() {
        println!("Hello, {}!", name);
    }

    println!("{:?}", names);
}



fn main() {
    let users = [
            String::from("My"),
            String::from("Bloody"),
            String::from("Valentine")
    ];

    for c in users.iter() {
        println!("{}", c);
    }
    println!("{:?}", users);
}

```

### 🔍 설명
- .iter()는 불변 참조를 반환하는 이터레이터.
- names의 각 요소를 참조하면서 반복 → 소유권 이동 없음.
- 반복 후에도 names를 그대로 사용할 수 있음.
출력 결과:
```
Hello, james!
Hello, cameron!
Hello, indo!
["james", "cameron", "indo"]


My
Bloody
Valentine
["My", "Bloody", "Valentine"]

```


### 🧨 주의: .into_iter()와 소유권
- "for name in names"에서 .into_iter()가 암묵적으로 호출되어 소유권이 이동됨 → 이후 names를 참조하면 컴파일 오류 발생"

```rust
fn main() {
    let names = vec!["james", "cameron", "indo"];

    for name in names {
        println!("{}", name);
    }

    println!("{:?}", names); // ❌ 오류 발생: names의 소유권이 이동됨
}


fn main() {

    let users = [
            String::from("My"),
            String::from("Bloody"),
            String::from("Valentine")
    ];

    for c in users.into_iter() {
        println!("{}", c);
    }

    //borrow of moved value: users values borrowed here after move.
    //println!("{:?}", users); //❌ Compile error
}

```

## 🔍 핵심 차이
| 메서드         | 반환 타입   | 소유권 이동 | 수정 가능 여부 | 반복 후 원본 사용 가능 |
|----------------|-------------|--------------|----------------|------------------------|
| `.iter()`      | `&T`        | ❌ 없음       | ❌ 불가능        | ✅ 가능                |
| `.iter_mut()`  | `&mut T`    | ❌ 없음       | ✅ 가능          | ✅ 가능                |
| `.into_iter()` | `T`         | ✅ 있음       | ✅ 가능 (값에 따라) | ❌ 불가능              |


## 🧠 이터레이터의 종류와 특징
| 메서드         | 설명                                      | 사용 예시                              |
|----------------|-------------------------------------------|----------------------------------------|
| `iter()`       | 불변 참조로 반복                          | `for x in vec.iter()`                  |
| `iter_mut()`   | 가변 참조로 반복                          | `for x in vec.iter_mut()`              |
| `into_iter()`  | 값 자체를 반복 (소유권 이동)              | `for x in vec.into_iter()`             |
| `.map()`       | 각 요소에 함수 적용                       | `vec.iter().map(|x| x + 1)`            |
| `.filter()`    | 조건에 맞는 요소만 추출                   | `vec.iter().filter(|x| ...)`           |
| `.collect()`   | 결과를 새로운 컬렉션으로 수집             | `vec.iter().map(...).collect()`        |


## 🧬 이터레이터와 소유권
Rust의 이터레이터는 소유권 시스템과 밀접하게 연결되어 있어요. 반복 방식에 따라 값이 복사되거나 참조되며, 이로 인해 컴파일 오류가 발생할 수도 있고, 반대로 불필요한 복사를 줄일 수도 있어요.
- .into_iter()는 벡터 원소의 값과 소유권을 루프 안으로 가져옴 → 이후 원본 벡터는 사용할 수 없음


## 🧭 언제 어떤 이터레이터를 써야 할까?
| 상황                             | 추천 메서드     | 이유                                           |
|----------------------------------|------------------|------------------------------------------------|
| 원본을 그대로 유지하고 싶을 때   | `.iter()`        | 불변 참조로 반복하므로 소유권이 이동되지 않음     |
| 원소를 수정하고 싶을 때          | `.iter_mut()`    | 가변 참조를 통해 각 요소를 직접 수정 가능         |
| 원본을 더 이상 사용하지 않을 때  | `.into_iter()`    | 소유권을 넘겨 반복하며, 이후 원본은 사용 불가       |



## ✅ 정리
### Rust 이터레이터 핵심 요약

- `.iter()` → 참조 기반 반복 (소유권 유지)
- `.into_iter()` → 값 기반 반복 (소유권 이동)
- `.iter_mut()` → 가변 참조 반복 (수정 가능)
- 고차 함수 `.map()`, `.filter()`, `.collect()` 등과 함께 사용 가능
- 반복 후 원본을 사용할 수 있는지 여부는 소유권에 따라 결정됨


이터레이터는 Rust의 철학인 안전성, 명확성, 성능을 그대로 반영한 기능.

----

