# struct update syntax
Rust에서 사용하는 `..avery` 문법은 **구조체 업데이트 문법(struct update syntax)** 이라고 불리며,  
기존 구조체의 값을 복사하면서 명시적으로 지정한 필드만 덮어쓰는 방식입니다.  
아래에 문서 스타일로 정리:

## 🧠 Rust 구조체 업데이트 문법: `..기존_구조체`
### 📌 개념 요약
- `..기존_구조체` 는 명시되지 않은 필드를 기존 구조체에서 복사하는 문법
- 새로 생성되는 구조체는 명시된 필드는 새 값으로, 나머지는 기존 구조체의 값으로 채워짐
- 단, 기존 구조체의 소유권이 이동되므로 이후에 사용할 수 없음 (Copy 타입 제외)

## ✅ 예제
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
- avery는 String을 포함하므로 소유권이 jackie로 이동됨 → 이후 avery는 사용 불가

## ⚠️ 주의사항
| 항목              | 설명                                                                 |
|-------------------|----------------------------------------------------------------------|
| 소유권 이동       | `..avery`는 `avery`의 필드를 복사하면서 소유권을 가져감 (Copy 타입 제외) |
| 필드 덮어쓰기     | `name: "재키"`처럼 명시된 필드는 기존 값보다 우선함                     |
| 전체 필드 초기화  | `..` 문법은 생략된 필드를 기존 구조체에서 채우지만, 전체 필드가 반드시 초기화되어야 함 |


## 💡 실무 팁
- `..` 문법은 기존 값을 기반으로 일부만 바꾸고 싶을 때 매우 유용
- Clone을 구현하면 소유권 문제 없이 복사 가능
- Copy 타입(u8, bool 등)은 소유권 이동 없이 사용 가능

---

# 소유권 이동 주의

jackie를 생성할 때 ..avery를 사용하면, avery의 모든 필드 중에서 `사용되지 않은 것들` 도 `소유권이 이동` 됩니다.  
Rust는 구조체 업데이트 문법에서 전체 구조체의 소유권을 가져가는 방식을 사용하기 때문.

## 🧠 왜 name도 소유권이 이동될까?
```rust
let jackie = Person {
    name: String::from("재키"),
    ..avery
};

```
- jackie.name은 새로 지정했기 때문에 `avery.name` 은 `사용되지 않음`
- 하지만 `..avery` 는 avery 전체를 대상으로 하기 때문에
→ avery.name의 소유권도 jackie로 이동됨
- 결과적으로 avery는 이후에 사용할 수 없음 (컴파일 에러 발생)

## 🔍 실제로 확인해보면
```rust
let jackie = Person {
    name: String::from("재키"),
    ..avery
};

println!("{}", avery.name); // ❌ 컴파일 에러: value moved
```

### 에러 메시지:
```
value borrowed here after move
```

## ✅ 해결 방법
- Person이 Clone을 구현하고 있다면, ..avery.clone()으로 복사 가능
- age처럼 Copy 타입은 소유권 이동 없이 복사됨 → 문제 없음
- String은 Clone이 필요하거나 소유권을 넘겨야 함

---

# Clone 사용

전제 조건은 Person 구조체가 Clone 트레이트를 구현하고 있어야 함.

## ✅ 전제 조건: Clone 구현
```rust
#[derive(Clone)]
struct Person {
    name: String,
    age: u8,
}
```

- `#[derive(Clone)]` 을 붙이면 Person 구조체가 .clone()을 사용할 수 있게 됩니다
- String과 u8은 각각 Clone을 구현하고 있으므로 문제 없음

## 🔍 코드 동작 설명
```rust
let jackie = Person {
    name: String::from("재키"),
    ..avery.clone()
};
```

- `avery.clone()` 은 Person 전체를 복사
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


