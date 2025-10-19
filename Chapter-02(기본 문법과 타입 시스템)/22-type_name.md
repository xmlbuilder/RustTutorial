# type_name
Rust의 `std::any::type_name::<T>()` 는 컴파일 타임에 타입 정보를 문자열로 반환하는 함수로,  
디버깅이나 로깅, 제네릭 타입 추적에 매우 유용한 도구입니다.

## 🧠 `std::any::type_name::<T>()` 란?
- 정의: fn type_name<T>() -> &'static str
- 역할: 제네릭 타입 T의 정적 타입 이름을 문자열로 반환
- 사용 목적: 디버깅, 로깅, 타입 추적, 제네릭 함수 내부에서 타입 확인
```rust
use std::any::type_name;
fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
```

### 🔍 `type_name::<T>()` 특징 요약

| 항목                  | 설명 또는 예시 출력값                        |
|-----------------------|----------------------------------------------|
| 반환 타입             | `&'static str`                               |
| 기본 타입             | `"i32"`, `"f64"`, `"bool"` 등                |
| 제네릭 타입           | `"std::vec::Vec<i32>"`                       |
| 함수 이름             | `"print_type_of::<i32>"`                     |
| 클로저 표현           | `"main::{{closure}}"`                        |
| 함수 포인터           | `"playground::main"`                         |
| 문자열 리터럴         | `"&str"`                                     |
| 라이프타임 정보        | `'a`, `'static` 등은 **출력에 포함되지 않음** |

## 🧪 예제 분석
```rust
fn main() {
    let s = "Hello";                  // &str
    let i = 42;                       // i32
    print_type_of(&s);               // prints "&str"
    print_type_of(&i);               // prints "i32"
    print_type_of(&main);            // prints "playground::main"
    print_type_of(&print_type_of::<i32>); // prints "playground::print_type_of<i32>"
    print_type_of(&{ || "Hi!" });    // prints "playground::main::{{closure}}"
    print_type_of(&32.90);           // prints "f64"
    print_type_of(&vec![1, 2, 4]);   // prints "std::vec::Vec<i32>"
}
```

### 🔎 해석
- `&s`: &str → 문자열 slice
- `&i`: i32 → 정수 타입
- `&main`: 함수 포인터 → 네임스페이스 포함
- `&print_type_of::<i32>`: 제네릭 함수 → 타입 인자까지 명시
- `&{ || "Hi!" }`: 클로저 → 익명 함수로 표현됨
- `&vec![1, 2, 4]: Vec<i32>` → 제네릭 컨테이너 타입

### ⚠️ 주의사항
- `type_name::<T>()` 은 `디버깅용` 이지, `타입 비교` 나 `로직 분기` 에는 사용하지 마세요
- 타입 이름은 컴파일러 버전이나 최적화 수준에 따라 달라질 수 있음
- 라이프타임, 트레잇 바운드, const generics 등은 출력에 포함되지 않음

### 🎯 실전 활용 예시
```rust
fn log_type<T>(value: &T) {
    println!("Type: {}", std::any::type_name::<T>());
}
```
- 제네릭 함수에서 타입 추적
- 디버깅 시 구조체나 enum의 실제 타입 확인
- 매크로 내부에서 타입 기반 로깅

---


## ✅ 실전에서는 어떻게 해야 할까?
Rust에서는 실전 로직에서 타입을 구분하거나 다르게 처리하고 싶을 때 다음과 같은 방식을 사용합니다:
### 1. `Trait` 기반 분기
```kotlin
trait Animal {
    fn sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn sound(&self) {
        println!("멍멍");
    }
}

impl Animal for Cat {
    fn sound(&self) {
        println!("야옹");
    }
}

fn make_sound<T: Animal>(animal: T) {
    animal.sound();
}
```

- 타입 이름 대신 trait 구현 여부로 기능을 분리
- type_name 없이도 타입별 동작을 안전하게 처리 가능

### 2. `enum` 으로 타입 구분
```kotlin
enum Pet {
    Dog,
    Cat,
}

fn make_sound(pet: Pet) {
    match pet {
        Pet::Dog => println!("멍멍"),
        Pet::Cat => println!("야옹"),
    }
}
```
- 여러 타입을 하나의 enum으로 묶고 match로 분기
- 실전에서 가장 흔히 쓰이는 방식

### 3. `where` 특정 타입에만 동작하도록 구현
```rust
fn do_something<T>(value: T)
where
    T: SpecificTrait,
{
    // T가 SpecificTrait을 구현한 경우에만 호출 가능
}
```

- 타입 이름이 아니라 `trait 제약으로 타입을 제한`

## 🔍 type_name::<T>()은 언제 쓰나?
- 디버깅 로그
- 테스트 중 타입 확인
- panic 메시지에 타입 정보 포함
- 개발 중 "이게 무슨 타입이지?" 확인용

---

