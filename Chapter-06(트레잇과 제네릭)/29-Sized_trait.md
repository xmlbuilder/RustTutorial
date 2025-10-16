# 📦 Sized 트레이트란?
- Sized는 타입의 크기가 컴파일 타임에 고정되어 있는지를 나타내는 내장 트레이트.
- 대부분의 타입은 Sized예요. 예: i32, String, Vec<T> 등
- 하지만 **동적 크기 타입(DST)** 은 Sized가 아님. 예: str, [T], dyn Trait
```rust
fn foo<T>(x: T) {} // T는 기본적으로 Sized
```

- 위 함수는 사실 다음과 같이 해석:
```rust
fn foo<T: Sized>(x: T) {}
```
## 🧠 Self: Sized의 의미
```rust
pub trait Split
where
    Self: Sized,
{
    // ...
}
```
- 이 선언은 Split 트레이트를 Sized 타입에만 구현할 수 있도록 제한.
- 즉, Split은 크기가 고정된 타입에만 적용 가능하고, dyn Split 같은 동적 트레이트 객체로는 사용할 수 없음.

## 🔍 왜 Self: Sized를 명시할까?
- 트레이트 메서드에서 Self를 값으로 받거나 반환할 때 Sized가 필요:
```rust
trait Split
where
    Self: Sized,
{
    fn split(self) -> (Self, Self); // self를 값으로 받음 → Sized 필요
}
```

- 만약 self를 참조로 받는다면 Sized 제약 없이도 가능:
```rust
trait Split {
    fn split(&self) -> (&Self, &Self); // 참조 → Sized 없어도 됨
}
```

## 🧩 동적 디스패치와의 관계
- Self: Sized가 없으면 dyn Split 같은 트레이트 객체로도 사용할 수 있음.
- Self: Sized가 있으면 트레이트 객체로 사용 불가 → Box<dyn Split> 불가능

## ✅ 정리
| 항목                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| Sized                 | 타입의 크기가 컴파일 타임에 고정되어 있음을 나타내는 내장 트레이트       |
| Self: Sized           | 해당 트레이트는 크기 고정된 타입에만 구현 가능                           |
| Self: Sized → dyn Trait 불가 | 트레이트 객체(dyn Trait)로 사용할 수 없음 → 동적 디스패치 제한             |
| self 값 사용 시 Sized 필요 | `self`를 값으로 받거나 반환할 경우 `Sized` 제약이 반드시 필요               |

---

# 🔍 차이점 설명
## ✅ 1. fn split(self) -> (Self, Self)
```rust
trait Split
where
    Self: Sized,
{
    fn split(self) -> (Self, Self);
}
```
- 여기서 self는 값으로 받기 때문에 Self의 크기를 알아야 함.
- 즉, 메모리에 얼마만큼 복사해야 하는지 알아야 하므로 Self: Sized 제약이 필요.
- 이 트레이트는 dyn Split 같은 트레이트 객체로는 사용할 수 없음.

## ✅ 2. fn split(&self) -> (&Self, &Self)
```rust
trait Split {
    fn split(&self) -> (&Self, &Self);
}
```
- 여기서는 self를 **참조(&self)** 로 받기 때문에 Self의 크기를 몰라도 됨.
- 참조는 포인터 크기만 있으면 되기 때문에 Sized 제약이 없어도 됨.
- 이 트레이트는 dyn Split으로도 사용할 수 있음 → 동적 디스패치 가능

## 📌 비교 요약
| 메서드 선언 방식         | Sized 제약 필요 여부 | 트레이트 객체(dyn Split) 사용 가능 여부 |
|--------------------------|----------------------|----------------------------------------|
| fn split(self)           | ✅ 필요함             | ❌ 불가능 (`Self: Sized` 필요)          |
| fn split(&self)          | ❌ 필요 없음          | ✅ 가능 (`Self` 크기 몰라도 참조 가능)  |


## 💡 비유로 이해하기
- self를 값으로 받는 건 "복사해서 넘겨줘" 라는 뜻 → 크기를 알아야 복사 가능
- &self는 "주소만 넘겨줘" 라는 뜻 → 크기를 몰라도 포인터만 있으면 됨

---

# 실무 적용

where Self: Sized는 실무에서 트레이트 설계 시 타입의 크기 제약을 명시적으로 걸어주는 방식으로 자주 사용.  
아래에 실무에서 어떻게 활용되는지 구체적으로 설명.

## ✅ 실무에서 where Self: Sized를 쓰는 이유
### 1. 트레이트 객체로 사용하지 못하게 막기 위해
```rust
trait MyTrait
where
    Self: Sized,
{
    fn consume(self); // self를 값으로 받음
}
```

- 이 트레이트는 dyn MyTrait으로 사용할 수 없음.
- 즉, 동적 디스패치 불가능 → Box<dyn MyTrait> 불가
- 실무에서는 트레이트 객체로 쓰면 위험하거나 의미 없는 경우 이를 명시적으로 막기 위해 사용.

### 2. 값 기반 메서드를 정의할 때
```rust
trait Cloneable
where
    Self: Sized,
{
    fn clone_self(self) -> Self;
}
```

- self를 값으로 받거나 반환할 때는 크기를 알아야 복사 가능 → Sized 제약 필요
- 실무에서는 복사, 이동, 소유권 이전 같은 동작을 정의할 때 자주 등장.

### 3. 부분적으로만 Sized 제약을 걸고 싶을 때
```rust
trait MyTrait {
    fn borrow(&self); // Sized 없어도 됨

    fn take(self)
    where
        Self: Sized;
}
```

- 트레이트 전체에는 Sized 제약을 걸지 않고, 특정 메서드에만 제한적으로 적용
- 실무에서는 참조 기반 메서드는 트레이트 객체로 쓰고, 값 기반 메서드는 컴파일 타임 타입에만 쓰도록 분리할 때 유용

## 🧠 실무 예시: Iterator 트레이트
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn collect<B>(self) -> B
    where
        Self: Sized;
}
```
- next()는 참조 기반 → 트레이트 객체에서도 사용 가능
- collect()는 값 기반 → Self: Sized 제약 필요
- 실무에서 dyn Iterator는 next()는 호출 가능하지만 collect()는 호출 불가

## ✨ 실무 활용 요약
| 활용 요소             | 설명                                                                 |
|------------------------|----------------------------------------------------------------------|
| dyn Trait 제한         | `Self: Sized`가 있으면 트레이트 객체(dyn Trait)로 사용할 수 없음         |
| self 값 사용           | `self`를 값으로 받거나 반환할 경우 `Sized` 제약이 반드시 필요               |
| Sized 제약 분리        | 트레이트 전체가 아닌 특정 메서드에만 `Self: Sized`를 걸어 유연하게 설계 가능 |

---

# Iterator collect

아래는 Iterator 트레이트의 collect() 메서드가 Self: Sized 제약 때문에 트레이트 객체에서는 사용할 수 없는 예시를 보여주는 코드.

## ❌ 트레이트 객체에서 collect() 사용 불가 예시
```rust
fn main() {
    let data = vec![1, 2, 3];

    // Iterator 트레이트 객체로 변환
    let iter: &dyn Iterator<Item = i32> = &data.into_iter();

    // 컴파일 오류 발생: collect는 Self: Sized 제약이 있어 트레이트 객체에서 호출 불가
    let collected: Vec<i32> = iter.collect(); // ❌ 오류!
}
```

### 🔥 오류 메시지 예시
```
error[E0038]: the trait `Iterator` cannot be made into an object
  --> main.rs:7:30
   |
7  |     let collected: Vec<i32> = iter.collect();
   |                              ^^^^^^^^^^^^^^^ `collect` method cannot be called on `dyn Iterator`

```

## ✅ 해결 방법: 제네릭으로 받기
```rust
fn collect_values<I>(iter: I) -> Vec<I::Item>
where
    I: Iterator, // Sized가 기본으로 포함됨
{
    iter.collect()
}

fn main() {
    let data = vec![1, 2, 3];
    let result = collect_values(data.into_iter()); // 정상 작동
    println!("{:?}", result);
}
```

## 📌 핵심 요약
| 상황                        | 사용 가능 여부 | 제약 조건         | 설명                                               |
|-----------------------------|----------------|-------------------|----------------------------------------------------|
| dyn Iterator에서 collect()  | ❌ 불가능       | Self: Sized 필요   | 트레이트 객체는 크기를 알 수 없으므로 호출 불가       |
| 일반 Iterator에서 collect() | ✅ 가능         | Sized 기본 포함됨 | 제네릭 타입은 기본적으로 Sized → 호출 가능          |
이처럼 Self: Sized 제약이 걸린 메서드는 트레이트 객체에서는 호출할 수 없고, 제네릭 타입으로 받아야만 사용할 수 있음.

## map 사용
map()이 중간에 들어오는 이유는 트레이트 객체에서 collect()를 직접 호출할 수 없기 때문.

### 🧩 문제 상황: dyn Iterator는 collect() 못 씀
- collect()는 Self: Sized 제약이 있기 때문에 트레이트 객체에서는 호출 불가
- 예: 아래 코드는 컴파일 오류
```rust
let iter: &dyn Iterator<Item = i32> = &vec![1, 2, 3].into_iter();
let result: Vec<i32> = iter.collect(); // ❌ 오류: Self: Sized
```


### ✅ 해결 전략: map()으로 처리하고, 이후에 collect()는 제네릭에서 호출
- map()은 &self 기반 메서드 → Sized 없어도 호출 가능
- 그래서 dyn Iterator에서도 map()은 호출 가능
- 이후 map() 결과는 새로운 제네릭 타입의 Iterator가 되므로 collect() 가능
```rust
fn main() {
    let data = vec![1, 2, 3];
    let iter: &dyn Iterator<Item = i32> = &data.into_iter();

    // map은 &self 기반이므로 dyn Iterator에서도 호출 가능
    let mapped = iter.map(|x| x * 2); // ✅ 가능

    // 하지만 mapped는 제네릭 타입이므로 collect() 가능
    let result: Vec<_> = mapped.collect(); // ✅ 가능
}
```


## 📌 핵심 요약
| 메서드     | 호출 방식 | Sized 제약 여부     | 설명                                               |
|------------|------------|----------------------|----------------------------------------------------|
| map()      | &self      | ❌ 불필요             | 참조 기반이므로 트레이트 객체에서도 호출 가능         |
| collect()  | self       | ✅ Self: Sized 필요   | 값 기반이므로 크기 고정된 타입에서만 호출 가능       |

## 🧩 왜 Box<dyn Iterator>를 쓰는가?
- dyn Iterator는 크기를 알 수 없는 동적 타입 → 힙에 저장해야 함
- Box<dyn Iterator<Item = T>>는 트레이트 객체를 힙에 저장하고 참조할 수 있게 해줌
- 이 방식은 런타임에 결정되는 반복자를 다룰 때 유용해요 (예: 조건에 따라 다른 반복자 선택)

## ❌ 직접 collect()는 불가능
```rust
fn main() {
    let iter: Box<dyn Iterator<Item = i32>> = Box::new(vec![1, 2, 3].into_iter());

    // 컴파일 오류: collect()는 Self: Sized 제약이 있어 호출 불가
    let result: Vec<i32> = iter.collect(); // ❌ 오류
}
```

## ✅ 우회 전략: 제네릭 함수에 넘겨서 collect()
```rust
fn collect_boxed<I>(iter: I) -> Vec<I::Item>
where
    I: Iterator, // Sized 포함됨
{
    iter.collect()
}

fn main() {
    let iter: Box<dyn Iterator<Item = i32>> = Box::new(vec![1, 2, 3].into_iter());

    // Box를 풀어서 제네릭 함수에 넘김
    let result = collect_boxed(iter); // ✅ 가능
    println!("{:?}", result);
}
```

- Box<dyn Iterator>는 Iterator를 구현하므로 collect_boxed()에 넘길 수 있음
- 제네릭 함수는 Sized 제약을 만족하므로 collect() 호출 가능

## ✨ 실무 활용 요약
| 상황 또는 전략               | 설명                                                                 |
|------------------------------|----------------------------------------------------------------------|
| 트레이트 객체 사용 (`dyn`)     | `Self: Sized` 제약이 있는 메서드 호출 불가 → 예: `collect()` 불가능       |
| Self: Sized 제약              | 값 기반 메서드에서 필요 → 트레이트 객체에서는 만족하지 않음               |
| Sized 제약 있는 collect() 호출 | 제네릭 함수 또는 구체 타입에서만 가능 → `Box<dyn Iterator>`는 직접 호출 불가 |
| 우회 전략                     | `Box<dyn Iterator>`를 제네릭 함수에 넘겨서 `collect()` 호출 가능           |


---

## ❌ 왜 dyn Trait에서는 Self: Sized가 안 되는가?
### 1. dyn Trait는 크기를 알 수 없음
```rust
let x: dyn MyTrait; // ❌ 불가능
let x: Box<dyn MyTrait>; // ✅ 가능 (힙에 저장하고 포인터로 참조)
```
- dyn Trait는 트레이트 객체로서 런타임에 어떤 타입인지 결정됨.
- 컴파일 타임에 크기를 알 수 없기 때문에 Sized를 만족하지 않음.

---

# Generic Sized  제약

Rust에서 제네릭 타입은 기본적으로 Sized 제약이 걸려 있음.  
이건 Rust의 타입 시스템이 안전한 메모리 관리를 보장하기 위한 기본 동작.  

## 🧩 제네릭 타입은 기본적으로 Sized
```rust
fn foo<T>(x: T) {}
```

- 위 함수는 사실 다음과 같이 해석:
```rust
fn foo<T: Sized>(x: T) {}
```
- 즉, Rust는 모든 제네릭 타입 T에 대해 기본적으로 Sized 제약을 걸림.
- 이유는 x: T처럼 값으로 받는 경우, 크기를 알아야 메모리에 복사하거나 이동할 수 있기 때문.

## ✅ Iterator 트레이트도 마찬가지
```rust
fn collect<I>(iter: I) -> Vec<I::Item>
where
    I: Iterator,
{
    iter.collect()
}
```
- 여기서 I는 제네릭 타입 → 기본적으로 Sized
- Iterator 트레이트의 collect()는 Self: Sized 제약이 있기 때문에, 제네릭 타입으로 받으면 Sized가 자동으로 포함돼서 호출 가능

## ❓ 그럼 Sized를 없애려면?
- ?Sized를 명시적으로 써야 Sized 제약을 제거할 수 있음:
```rust
fn foo<T: ?Sized>(x: &T) {}
```
- 이건 T가 Sized일 수도 있고 아닐 수도 있다는 뜻
- 단, 이 경우는 참조로만 받을 수 있어요 → x: &T

## 📌 요약
| 항목         | 설명                                                                 |
|--------------|----------------------------------------------------------------------|
| T: Sized     | 제네릭 타입은 기본적으로 `Sized` 제약이 걸림 → 값으로 받기 위해 필요     |
| Sized        | 타입의 크기가 컴파일 타임에 고정되어 있어야 함                          |
| Iterator     | `collect()`는 `Self: Sized` 제약이 있어 트레이트 객체에서는 호출 불가     |
| ?Sized       | `Sized` 제약을 제거하고 싶을 때 사용 → 참조로만 받을 수 있음              |

## ❌ 크기가 정해지지 않은 타입은 직접 사용할 수 없음
예: str, [T], dyn Trait 등은 Sized가 아님.
```rust
fn print_str(s: str) {} // ❌ 컴파일 오류: str은 크기를 알 수 없음
```

- 대신 참조로 받으면 가능:
```rust
fn print_str(s: &str) {} // ✅ 가능
```


## ✅ ?Sized로 제약을 풀 수는 있음
```rust
fn describe<T: ?Sized>(value: &T) {
    // T는 Sized일 수도 있고 아닐 수도 있음
}
```

- ?Sized를 쓰면 크기가 고정되지 않은 타입도 허용할 수 있어요
- 단, 이 경우는 반드시 참조로만 사용해야 해요 → value: &T

## 📌 요약
| 항목         | 설명                                                                 |
|--------------|----------------------------------------------------------------------|
| T: Sized     | 제네릭 타입은 기본적으로 `Sized` 제약이 걸림 → 값으로 받기 위해 필요     |
| Sized        | 타입의 크기가 컴파일 타임에 고정되어 있어야 함                          |
| str, dyn     | 크기를 알 수 없는 DST(Dynamically Sized Type) → 기본적으로 `Sized` 아님 |
| ?Sized       | `Sized` 제약을 제거할 때 사용 → 참조로만 사용 가능 (`&T`)                |

--- 

# String은 sized?

fn print_str(s: String) {}가 컴파일 오류 없이 잘 작동한다는 건, String 타입이 Sized 트레이트를 구현하고 있다는 뜻.

## ✅ 왜 String은 Sized인가?
- String은 Rust의 표준 라이브러리에 정의된 **구조체(struct)** 예요.
- 내부적으로 Vec<u8>를 감싸고 있고, 힙에 저장된 문자열 데이터를 소유.
- 구조체는 컴파일 타임에 크기가 고정되므로 Sized.

```rust
pub struct String {
    vec: Vec<u8>,
    // 기타 필드들...
}
```

- 따라서 String은 Sized를 자동으로 만족하고, 값으로 받을 수 있어요.

## ❌ 반면 str은 Sized가 아님
```rust
fn print_str(s: str) {} // ❌ 오류: str은 크기를 알 수 없음
fn print_str(s: &str) {} // ✅ 참조로 받으면 가능
```
- str은 **크기를 알 수 없는 동적 타입(DST)** 이기 때문에 직접 받을 수 없고,
- 항상 &str, Box<str>, Rc<str>처럼 포인터를 통해 참조해야 함.

## 📌 요약
| 타입    | Sized 여부 | 설명                                           |
|---------|-------------|------------------------------------------------|
| String  | ✅ Sized     | 구조체이므로 크기가 컴파일 타임에 고정됨 → 값으로 사용 가능 |
| str     | ❌ !Sized    | 동적 크기 타입(DST)이므로 참조로만 사용 가능 (`&str`, `Box<str>` 등) |

---

# 🧩 핵심 개념: Vec<T>는 구조체(struct)다
```rust
pub struct Vec<T> {
    ptr: *mut T,     // 힙에 있는 데이터의 포인터
    len: usize,      // 길이
    capacity: usize, // 할당된 용량
}
```

- Vec<T> 자체는 구조체.
- 이 구조체는 고정된 크기의 필드들로 구성돼 있음 → 포인터, 정수 등
- 따라서 Vec<T>는 컴파일 타임에 크기가 고정됨 → Sized 만족

## ✅ 힙에 있는 데이터는 크기를 몰라도 됨
- Vec<u8>는 실제 데이터(u8 배열)를 힙에 저장하고,
- 구조체 내부에는 그 데이터를 가리키는 포인터만 저장됨.
- 즉, Vec<u8> 자체는 스택에 저장되는 고정 크기 구조체이고,
- 힙에 있는 데이터는 크기를 몰라도 상관 없음.

## ❓ 그럼 [u8]은 왜 Sized가 아니야?
```rust
fn foo(data: [u8]) {} // ❌ 오류: 크기를 알 수 없음
fn foo(data: &[u8]) {} // ✅ 참조로 받으면 가능
```

- [u8]은 **크기를 알 수 없는 슬라이스 타입(DST)** 이기 때문에 Sized가 아님.
- 반면 Vec<u8>는 슬라이스를 감싸는 구조체라서 Sized.

## 📌 요약
| 타입       | Sized 여부 | 사용 방식                              |
|------------|-------------|----------------------------------------|
| Vec<u8>    | ✅ Sized     | 값으로 직접 사용 가능                  |
| [u8]       | ❌ !Sized    | 참조로만 사용 가능 → &[u8], Box<[u8]> |
Rust는 이런 구조 덕분에 동적 크기 데이터를 안전하게 다루면서도, 스택에는 고정 크기 타입만 저장해서 성능과 안정성을 동시에 확보.


## 🧩 핵심 구분: "데이터의 크기" vs "타입의 크기"
| 구분            | 예시                        | 설명                                                                 |
|-----------------|-----------------------------|----------------------------------------------------------------------|
| 데이터의 크기   | `Vec<T>`의 `len()`           | 실제 저장된 요소의 개수 → 런타임에 결정됨                             |
| 타입의 크기     | `Vec<T>` 자체                | 구조체의 크기 → 포인터 + 길이 + 용량 → 컴파일 타임에 고정됨 → `Sized` |


## ✅ 왜 Vec<T>는 Sized인가?
```rust
pub struct Vec<T> {
    ptr: *mut T,     // 힙에 있는 데이터의 주소
    len: usize,      // 현재 길이
    capacity: usize, // 할당된 용량
}
```
- 이 구조체는 항상 3개의 필드로 구성됨.
- 각 필드는 크기가 고정된 타입이므로 전체 구조체의 크기도 고정됨 → Sized 만족
- 즉, Vec<T>는 힙에 있는 데이터의 크기와 무관하게, 자기 자신은 스택에 고정된 크기로 저장될 수 있음.

## ❓ 그럼 [T]는 왜 Sized가 아니야?
- [T]는 슬라이스 타입으로, 크기를 알 수 없음.
- 예: [u8], str, dyn Trait → 모두 DST(Dynamically Sized Type)
- 이들은 포인터로만 접근 가능 → &[T], Box<[T]>, Rc<str>, Arc<dyn Trait> 등

## 📌 요약
| 항목        | Sized 여부 | 설명                                                                 |
|-------------|-------------|----------------------------------------------------------------------|
| Vec<T>      | ✅ Sized     | 구조체 자체는 고정된 크기 필드로 구성됨 → 스택에 저장 가능               |
| [T]         | ❌ !Sized    | 슬라이스 타입 → 크기 불명 → 참조로만 사용 가능 (`&[T]`, `Box<[T]>`)     |
| vec.len()   | 런타임 결정 | 실제 저장된 요소의 개수 → 실행 중에 결정됨 → 타입 크기와는 별개           |

--- 

# Array는 sized?

[T]는 슬라이스(slice) 타입이고, **크기를 알 수 없기 때문에 !Sized** 임.  
반면 array 타입인 [T; N]은 **크기가 고정되어 있어서 Sized** 입니다.

## 🧩 핵심 차이: 슬라이스 vs 배열
| 타입      | 예시         | Sized 여부 | 사용 방식                        |
|-----------|--------------|------------|----------------------------------|
| [T]       | [u8], str    | ❌ !Sized   | 참조로만 사용 → &[T], Box<[T]>   |
| [T; N]    | [u8; 4]      | ✅ Sized    | 값으로 직접 사용 가능            |

## ✅ 예시 코드

```rust
fn take_array(arr: [u8; 4]) {} // ✅ 배열은 Sized → 값으로 받기 가능
fn take_slice(slice: &[u8]) {} // ✅ 슬라이스는 참조로 받으면 가능
fn take_unsized(slice: [u8]) {} // ❌ 오류: [u8]은 Sized가 아님
```

## 💡 핵심 포인트
- [T; N]은 크기가 N으로 고정된 배열 → 컴파일 타임에 크기 결정됨 → Sized
- [T]는 크기를 알 수 없는 슬라이스 → 런타임에 참조로만 사용 가능 → !Sized

---

# ✅ 함수 인자와 Sized 제약
## 1. 값으로 받는 경우 (T) → T: Sized 필요
```rust
fn take_value<T>(x: T) {} // 사실은 fn take_value<T: Sized>(x: T) {}
```
- T의 크기를 알아야 스택에 메모리를 할당할 수 있으므로 Sized가 필요.
- str, [T], dyn Trait 같은 DST는 직접 받을 수 없어요.

## 2. 참조로 받는 경우 (&T) → T: ?Sized 가능
```rust
fn take_ref<T: ?Sized>(x: &T) {}
```

- 참조는 포인터이기 때문에, 실제 데이터의 크기를 몰라도 포인터 크기만 알수 있음.
- 그래서 ?Sized를 붙이면 Sized가 아닌 타입도 받을 수 있음.

## 📌 요약 표
| 인자 형태 | 타입 제약     | Sized 여부 | 설명                                               |
|-----------|----------------|------------|----------------------------------------------------|
| x: T      | T: Sized        | ✅ Sized    | 값으로 받기 위해 타입 크기가 컴파일 타임에 고정되어야 함 |
| x: &T     | T: ?Sized       | ❌ 가능     | 참조는 포인터이므로 크기 불명 타입도 허용됨 (`str`, `[T]`, `dyn Trait`) |

## 💡 예시
```rust
fn print_owned(s: String) {}         // ✅ String은 Sized → 값으로 받기 가능
fn print_slice(s: [u8]) {}           // ❌ [u8]은 !Sized → 값으로 받을 수 없음
fn print_slice_ref(s: &[u8]) {}      // ✅ 참조로 받으면 가능
fn print_generic<T>(x: T) {}         // ✅ T: Sized가 기본으로 붙음
fn print_generic_ref<T: ?Sized>(x: &T) {} // ✅ Sized가 아니어도 참조로 받으면 가능
```

---





