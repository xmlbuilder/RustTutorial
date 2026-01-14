## ptr_eq
- Arc 전용 메서드가 절대 아님.
- Rust 표준 라이브러리에서 여러 스마트 포인터 타입이  **두 포인터가 같은 메모리 블록을 가리키는지** 비교할 수 있도록 제공하는 공통 기능.
- 아래에서 어떤 타입에 있는지, 각각 어떻게 동작이 다른지 깔끔하게 정리.
- C의 포인터 비교(p1 == p2)와 Rust의 ptr_eq는 **같은 메모리 주소를 가리키는지** 비교한다는 점에서 비슷하다.
- 하지만 Rust는 스마트 포인터(Rc/Arc/Weak) 의 내부 구조 때문에 단순 비교로는 안 되는 경우가 있어서 ptr_eq가 따로 존재한다.


## 📌 ptr_eq가 존재하는 주요 타입
- Rust 표준 라이브러리에서 ptr_eq를 제공하는 타입은 다음 네 가지:
  - Rc<T>
  - Arc<T>
  - Weak<T>
  - Weak<dyn Trait> / Weak<TraitObject> (trait object 버전 포함)
- 즉, 참조 카운팅 기반 스마트 포인터(Rc/Arc) 계열에서 제공하는 기능이라고 보면 됨.

## 🧩 왜 Rc/Arc 계열에만 ptr_eq가 있을까?
- 이유는 간단:
  - Rc와 Arc는 여러 개의 포인터가 같은 데이터를 공유할 수 있음
  - 따라서 **두 포인터가 같은 객체를 가리키는지** 를 확인하는 기능이 필요함
  - 반면 Box, &T, &mut T 같은 포인터는
  - 보통 단일 소유권 또는 단일 참조
  - aliasing이 제한적
  - 그래서 ptr_eq 같은 기능이 필요하지 않음

## 🔍 각 타입별 ptr_eq의 의미와 차이
### 1. Rc::ptr_eq
```rust
impl<T> Rc<T> {
    pub fn ptr_eq(this: &Self, other: &Self) -> bool
}
```
- 의미
  - 두 Rc<T>가 같은 Rc 할당 블록을 가리키면 true
  - 즉, 같은 Rc clone인지 확인하는 용도
- 특징
- 내부 값이 같아도, 다른 Rc로 감싸져 있으면 false
```rust
let a = Rc::new(5);
let b = Rc::new(5);

assert!(!Rc::ptr_eq(&a, &b)); // 값은 같지만 다른 Rc
```

### 2. Arc::ptr_eq
```rust
impl<T> Arc<T> {
    pub fn ptr_eq(this: &Self, other: &Self) -> bool
}
```

- 의미
  - Rc와 동일하지만 스레드 안전한 Arc 버전
- 특징
  - 멀티스레드 환경에서 Arc clone들이 같은 데이터를 공유하는지 확인할 때 사용

### 3. Weak::ptr_eq
```rust
impl<T> Weak<T> {
    pub fn ptr_eq(this: &Self, other: &Self) -> bool
}
```

- 의미
  - 두 Weak 포인터가 같은 Arc/Rc의 약한 참조인지 확인
- 특징
  - Weak는 업그레이드(upgrade())하기 전에는 실제 데이터에 접근할 수 없지만  
  ptr_eq는 업그레이드 없이도 비교 가능
```rust
let a = Arc::new(10);
let w1 = Arc::downgrade(&a);
let w2 = Arc::downgrade(&a);

assert!(Weak::ptr_eq(&w1, &w2));
```


### 4. Trait Object 버전 (Arc<dyn Trait>, Rc<dyn Trait>)
- Trait object는 내부적으로 fat pointer(데이터 포인터 + vtable 포인터)를 가지기 때문에  
  단순 비교로는 **같은 객체인지** 판단하기 어려움.
그래서 ptr_eq는 데이터 포인터만 비교해서
“같은 객체를 가리키는지”를 정확히 판단해줘.

## 📌 ptr_eq가 비교하는 것은 “값”이 아니라 “주소”
- 이게 핵심.
- 예를 들어:
```rust
let a = Arc::new(String::from("hello"));
let b = a.clone();
let c = Arc::new(String::from("hello"));

assert!(Arc::ptr_eq(&a, &b)); // 같은 Arc
assert!(!Arc::ptr_eq(&a, &c)); // 값은 같지만 다른 Arc
```
## 📦 정리 테이블
| 타입        | ptr_eq 의미                                  | 특징 |
|-------------|-----------------------------------------------|-------|
| Rc<T>       | 같은 Rc 할당 블록을 가리키는지 비교          | 단일 스레드 |
| Arc<T>      | 같은 Arc 할당 블록을 가리키는지 비교         | 멀티스레드 |
| Weak<T>     | 같은 Weak가 같은 Rc/Arc를 가리키는지 비교    | 업그레이드 없이 비교 가능 |
| Rc/Arc<dyn Trait> | fat pointer 중 데이터 포인터만 비교     | trait object에서도 정확한 비교 |


## 🎯 결론
- ptr_eq는 Arc 전용이 아니라 Rc/Arc/Weak 전부에 존재
- 목적은 두 스마트 포인터가 같은 객체를 가리키는지 확인하는 것
- 값 비교가 아니라 메모리 주소 비교
- trait object에서도 정확한 비교를 위해 제공됨

---

# 실전 예제

- 이번에는 ptr_eq가 실제로 왜 필요한지, 그리고 어떤 상황에서 값 비교(==)와 주소 비교(ptr_eq)가 다르게 동작하는지를 실전 예제.
- 단순한 예제가 아니라 **ptr_eq가 없으면 곤란한 상황** 을 중심으로 설명.

## 🧪 실전 예제 1: 캐싱된 객체를 재사용해야 하는 경우
- 어떤 시스템에서 문자열을 intern(중복 제거)해서 같은 문자열이면 기존 객체를 재사용하고 싶다고 가정.
```rust
use std::rc::Rc;
use std::collections::HashSet;

fn main() {
    let mut pool: HashSet<Rc<String>> = HashSet::new();

    // 새로운 문자열이 들어왔다고 가정
    let s1 = Rc::new("hello".to_string());

    // pool에 같은 문자열이 있는지 확인
    if let Some(existing) = pool.get(&s1) {
        // 값은 같지만, 진짜로 같은 객체인지 확인
        println!("pool에 이미 있음");
        // 여기서 ptr_eq가 필요함
        assert!(Rc::ptr_eq(existing, existing));
        assert!(!Rc::ptr_eq(existing, &s1)); // s1은 pool에 있는 객체가 아님
    } else {
        println!("pool에 없으므로 삽입");
        pool.insert(s1.clone());
    }

    // 다시 같은 문자열이 들어왔다고 가정
    let s2 = Rc::new("hello".to_string());
    if let Some(existing) = pool.get(&s2) {
        println!("기존 객체 재사용");
        // ptr_eq로 진짜 같은 객체인지 확인
        assert!(Rc::ptr_eq(existing, pool.get(&s2).unwrap()));
        assert!(!Rc::ptr_eq(existing, &s2)); // s2는 새로 만든 Rc
    }
}
```

- 이 예제의 핵심
  - HashSet은 값(Eq, Hash) 기준으로 비교하므로  
    "hello"라는 문자열이 같으면 동등한 값으로 취급함.
  - 하지만 우리는 동일한 객체(메모리 블록) 를 재사용하고 싶음.
  - 이때 ptr_eq가 없으면 **값은 같지만 다른 Rc** 를 구분할 방법이 없음.

## 🧪 실전 예제 2: 그래프 구조에서 노드 동일성 비교
- 그래프를 Rc로 구성했다고 해보자.
```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
}

fn main() {
    let a = Rc::new(Node { value: 10 });
    let b = a.clone();
    let c = Rc::new(Node { value: 10 });

    // 값은 같지만
    assert_eq!(a.value, c.value);

    // 같은 노드인지 확인하려면 ptr_eq 필요
    assert!(Rc::ptr_eq(&a, &b)); // 같은 노드
    assert!(!Rc::ptr_eq(&a, &c)); // 값은 같지만 다른 노드
}
```
- 왜 필요할까?
- 그래프 알고리즘에서 **같은 노드인지** 판단할 때 값이 같다고 같은 노드가 아님.
- 동일한 객체인지 확인해야 함.

### 🧪 실전 예제 3: Weak 포인터 비교 (업그레이드 없이)
- Weak는 upgrade() 하기 전에는 값에 접근할 수 없음.
- 그런데 두 Weak가 같은 Arc를 가리키는지 알고 싶을 때가 있음.
```rust
use std::sync::{Arc, Weak};

fn main() {
    let a = Arc::new(100);

    let w1 = Arc::downgrade(&a);
    let w2 = Arc::downgrade(&a);
    let w3 = Weak::new(); // 아무것도 가리키지 않음

    assert!(Weak::ptr_eq(&w1, &w2)); // 같은 Arc의 weak
    assert!(!Weak::ptr_eq(&w1, &w3)); // 다른 weak
}
```

- 왜 필요할까?
  - Weak는 upgrade()가 실패할 수 있음
  - upgrade()를 매번 호출하면 비용이 들고, race condition도 생길 수 있음
  - ptr_eq는 upgrade 없이 비교 가능

### 🧪 실전 예제 4: Trait Object 비교
- Trait object는 fat pointer라서 `==` 로 비교하면 값 비교가 되지 않음.
```rust
use std::rc::Rc;

trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {}
}

fn main() {
    let a: Rc<dyn Animal> = Rc::new(Dog);
    let b = a.clone();
    let c: Rc<dyn Animal> = Rc::new(Dog);

    assert!(Rc::ptr_eq(&a, &b)); // 같은 객체
    assert!(!Rc::ptr_eq(&a, &c)); // 다른 객체
}
```

- 왜 필요할까?
  - Trait object는 내부적으로
  - 데이터 포인터
  - vtable 포인터
  - 두 개로 구성되기 때문에 단순 비교로는 **같은 객체인지** 판단이 어려움.
  - ptr_eq는 데이터 포인터만 비교해서 정확하게 판단해줌.

## 🧩 최종 요약
- ptr_eq는 "값이 같은가"가 아니라 **같은 객체(같은 메모리 블록)를 가리키는가** 를 확인하는 기능이다.

- 필요한 상황:
  - 캐싱된 객체 재사용
  - 그래프/트리 구조에서 노드 동일성 비교
  - Weak 포인터 비교 (upgrade 없이)
  - Trait object에서 동일 객체 비교

---



