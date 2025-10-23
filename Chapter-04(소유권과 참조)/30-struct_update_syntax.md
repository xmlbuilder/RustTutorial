# struct update syntax

Rust에서 사용하는 ..avery 문법은 구조체 업데이트 문법(struct update syntax) 이라고 불리며,  
기존 구조체의 값을 복사하면서 명시적으로 지정한 필드만 덮어쓰는 방식입니다.

## 🧠 Rust 구조체 업데이트 문법: ..기존_구조체
### 📌 개념 요약
- `..기존_구조체` 는 명시되지 않은 필드를 기존 구조체에서 복사하는 문법
- 새로 생성되는 구조체는 명시된 필드는 새 값으로, 나머지는 기존 구조체의 값으로 채워짐
- 사용된 필드만 기존 구조체에서 가져오며, 해당 필드의 소유권만 이동됨 (Copy 타입 제외)

### ✅ 예제
```rust
struct Person {
    name: String,
    age: u8,
}

let avery = Person {
    name: String::from("에이버리"),
    age: 39,
};

let jackie = Person {
    name: String::from("재키"),
    ..avery
};
```
- jackie.name → "재키" (새로 지정됨)
- jackie.age → 39 (avery에서 복사됨)
- age는 Copy 타입이므로 소유권 이동 없음
- name은 사용되지 않았으므로 소유권 이동 없음 → 이후에도 avery.name 사용 가능

## ⚠️ 주의사항

| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| `..avery`        | 명시되지 않은 필드를 기존 구조체에서 가져옴. 해당 필드의 소유권이 이동됨 (Copy 타입 제외) |
| `name: "재키"`   | 명시적으로 지정된 필드는 기존 구조체에서 가져오지 않음 → 소유권 이동 없음         |
| `..` 문법        | 전체 필드를 초기화해야 하며, 생략된 필드는 기존 구조체에서 채움                     |


💡 실무 팁
- `..` 문법은 기존 값을 기반으로 일부만 바꾸고 싶을 때 매우 유용
- `Clone` 을 구현하면 소유권 문제 없이 복사 가능
- `Copy 타입(u8, bool 등)` 은 소유권 이동 없이 사용 가능

## 소유권 이동 주의
- jackie를 생성할 때 ..avery를 사용하면, 사용된 필드만 avery에서 가져오며 해당 필드의 소유권이 이동됩니다.  
- Rust는 구조체 업데이트 문법에서 전체 구조체가 아닌, `실제로 사용된 필드` 만 이동시키는 방식을 사용합니다.  

## 🧠 왜 name은 소유권이 유지될까?
```rust
let jackie = Person {
    name: String::from("재키"),
    ..avery
};
```
- jackie.name은 새로 지정했기 때문에 `avery.name` 은 사용되지 않음
- ..avery 는 age 필드만 사용 → `avery.age` 의 소유권만 이동됨
- 결과적으로 avery.name은 이후에도 사용 가능
  
## 🔍 실제로 확인해보면
```rust
let jackie = Person {
    name: String::from("재키"),
    ..avery
};

println!("{}", avery.name); // ✅ 정상 동작
```
- 정상 컴파일됨

## ✅ 주의할 점
- 만약 name도 ..avery를 통해 가져오면 → 소유권 이동 발생
- 이후 avery.name을 사용하면 컴파일 에러 발생

## Clone 사용
전제 조건은 Person 구조체가 Clone 트레이트를 구현하고 있어야 함.
### ✅ 전제 조건: Clone 구현
```rust
#[derive(Clone)]
struct Person {
    name: String,
    age: u8,
}
```

- `#[derive(Clone)]` 을 붙이면 Person 구조체가 .clone()을 사용할 수 있게 됩니다
- String과 u8은 각각 Clone을 구현하고 있으므로 문제 없음
  
### 🔍 코드 동작 설명
```rust
let jackie = Person {
    name: String::from("재키"),
    ..avery.clone()
};
```
- avery.clone() 은 Person 전체를 복사
- name 필드는 "재키"로 덮어쓰기
- age는 avery의 값을 그대로 복사
- 소유권 이동 없이 안전하게 복제되므로 avery는 이후에도 사용 가능

## 💡 장점
| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| 안전한 복사       | `avery`의 소유권을 이동하지 않고 그대로 유지할 수 있음                  |
| 재사용 가능       | `avery`를 이후에도 자유롭게 사용할 수 있음                             |
| 선택적 덮어쓰기   | 필요한 필드만 새 값으로 지정하고 나머지는 기존 값으로 채움               |
| 간결한 문법       | `..` 문법으로 구조체 생성이 간결하고 가독성이 높아짐                     |

---



