# Sized

`Sized` 는 Rust에서 `모든 타입이 기본적으로 갖는 특성` 으로,  
컴파일 타임에 크기가 결정되는 타입을 의미합니다.  
이를 정확히 이해하지 못하면 제네릭, 트레잇 객체, 슬라이스 등에서 오류가 발생하기 쉽습니다.

## 📐 Sized란 무엇인가?
- Sized는 Rust의 marker trait입니다.
- 의미: 해당 타입의 크기가 `컴파일 타임에 고정` 되어 있다.
- 정의:
```rust
pub trait Sized {}
```
- 대부분의 타입은 Sized입니다. 
- 예: 
```rust
i32, String, Vec<T>, struct Foo { ... }

struct MyStr {
    data: str, // ❌ unsized → MyStr 자체도 unsized
}


struct Foo {
    x: i32,
    y: [i32],     // ❌ Foo 자체가 unsized 타입이 됨
}

```

## ❓ 왜 중요한가?
- `스택에 값을 직접 저장` 하려면 `크기를 알아야 하기 때문` 입니다.
- 힙에서는 Sized 필요 없음
- 예를 들어, 다음은 오류가 납니다:
```rust
fn takes_unsized(x: [u8]) {} // error: the size of `[u8]` is not known at compile time
```

## 🧩 ?Sized로 제약 제거하기
- **T가 Sized여도 가능** 이라고 의미
- 즉, sized도 가능, unsized도 가능
- 모든 제네릭 타입은 기본적으로 Sized 제약이 있습니다:
```rust
fn foo<T>(x: T) {} // T: Sized 가 암묵적으로 붙음
```
- `?Sized` 를 사용하면 이 제약을 제거할 수 있습니다:
```rust
fn bar<T: ?Sized>(x: &T) {} // T는 크기를 몰라도 됨 (단, 참조로만 사용 가능)
```

### 📦 동적 크기 타입 (DST: Dynamically Sized Types)

| 타입       | 크기 정보 | 설명                                                   |
|------------|------------|--------------------------------------------------------|
| `[T]`      | ❌         | 슬라이스 타입. 크기를 알 수 없어 참조로만 사용 가능     |
| `str`      | ❌         | UTF-8 문자열 슬라이스. `&str` 또는 `Box<str>`로 사용     |
| `dyn Trait`| ❌         | 트레잇 객체. 런타임에 크기 결정되며 `&dyn Trait`로 사용  |

## 🔍 참고 사항
- DST는 스택에 직접 저장할 수 없고, 반드시 참조나 포인터(&, Box, Rc, Arc)로 다뤄야 합니다.
- &dyn Trait은 fat pointer (두 단어: data ptr + vtable ptr)
- Box<dyn Trait>도 fat pointer
- 제네릭에서 DST를 허용하려면 T: ?Sized 제약을 명시해야 합니다.
- Sized가 아닌 타입은 대부분 런타임에 크기가 결정되거나 유연성을 위해 설계된 타입입니다.
- 이들은 직접 사용할 수 없고, 반드시 참조나 포인터로 사용해야 합니다:
- 
```rust
let s: &str = "hello";       // OK
let arr: &[u8] = &[1, 2, 3]; // OK
let t: &dyn Display = &42;   // OK
```


## 🧠 트레잇과 Sized
- 트레잇 정의 시 Sized를 명시하면 트레잇 객체로 사용할 수 없습니다:
```rust
trait Foo: Sized {} // ❌ dyn Foo 불가능
trait Bar {}        // ✅ dyn Bar 가능
```
- 트레잇 객체는 런타임에 크기가 결정되므로 Sized 제약이 있으면 dyn Trait으로 사용할 수 없습니다.

### 🛠 실전에서 자주 마주치는 오류

| 오류 메시지                                               | 원인                                                   |
|------------------------------------------------------------|--------------------------------------------------------|
| the size for values of type `T` cannot be known at compile time | `T`가 `Sized`가 아닌데 값으로 사용하려 함               |
| the trait cannot be made into an object                    | 트레잇에 `Sized`가 있어서 `dyn Trait`로 만들 수 없음    |


## ✅ 요약
- Sized는 타입의 크기가 컴파일 타임에 결정됨을 나타냅니다.
- 모든 제네릭은 기본적으로 Sized 제약이 있으며, ?Sized로 제거할 수 있습니다.
- 슬라이스, str, dyn Trait은 Sized가 아니므로 참조로만 사용해야 합니다.
- 트레잇 객체를 만들려면 트레잇 정의에 Sized를 붙이지 않아야 합니다.

---

## 🧪 오류 1: T가 Sized가 아닌데 값으로 사용하려 함
### ❌ 오류 코드
```rust
fn take_unsized<T>(val: T) {
    // ...
}
```

### 🧨 오류 메시지
```
error[E0277]: the size for values of type `T` cannot be known at compilation time
```


### ✅ 해결 방법
```rust
fn take_unsized<T: ?Sized>(val: &T) {
    // 참조로 받으면 크기 몰라도 OK
}
```


## 🧪 오류 2: 트레잇에 Sized가 있어서 dyn Trait 불가능
### ❌ 오류 코드

```rust
trait MyTrait: Sized {
    fn do_something(&self);
}

fn use_trait_object(obj: &dyn MyTrait) {
    obj.do_something();
}
```

### 🧨 오류 메시지
```
error: the trait `MyTrait` cannot be made into an object
```

## ✅ 해결 방법
```rust
trait MyTrait {
    fn do_something(&self);
}

fn use_trait_object(obj: &dyn MyTrait) {
    obj.do_something();
}
```


## 🧠 보너스: DST 타입을 참조 없이 사용하면 생기는 오류
### ❌ 오류 코드
```rust
fn take_str(s: str) {
    // ...
}
```

### 🧨 오류 메시지
```
error[E0277]: the size for values of type `str` cannot be known at compilation time
```

### ✅ 해결 방법
```rust
fn take_str(s: &str) {
    // OK
}
```
---




