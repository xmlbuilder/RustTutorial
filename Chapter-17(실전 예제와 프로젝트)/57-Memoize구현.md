# Memoize  구현
Rust는 HashMap과 `#[derive(Hash, Eq, PartialEq)]` 같은 매크로 지원 덕분에 구조체나 문자열을 키로 쓰는 게 매우 쉽습니다.

## 🦀 Rust 메모이제이션 예시
```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;

// 범용 Memoize 구조체
struct Memoize<F, A, R>
where
    F: Fn(&A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    func: F,
    cache: Rc<RefCell<HashMap<A, R>>>,
}

impl<F, A, R> Memoize<F, A, R>
where
    F: Fn(&A) -> R,
    A: Eq + Hash + Clone,
    R: Clone,
{
    fn new(func: F) -> Self {
        Memoize {
            func,
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn call(&self, arg: A) -> R {
        let mut cache = self.cache.borrow_mut();
        if let Some(result) = cache.get(&arg) {
            println!("Cache hit!");
            return result.clone();
        }
        println!("Computing...");
        let result = (self.func)(&arg);
        cache.insert(arg.clone(), result.clone());
        result
    }
}
```
```rust
// 사용자 정의 구조체
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Person {
    name: String,
    age: u32,
}

// 예시 함수
fn greet(person: &Person) -> String {
    format!("Hello {} ({})", person.name, person.age)
}
```
```rust
fn main() {
    let memoized_greet = Memoize::new(greet);

    let alice = Person { name: "Alice".to_string(), age: 30 };
    let bob = Person { name: "Bob".to_string(), age: 25 };

    println!("{}", memoized_greet.call(alice.clone())); // Computing...
    println!("{}", memoized_greet.call(alice.clone())); // Cache hit!
    println!("{}", memoized_greet.call(bob.clone()));   // Computing...
    println!("{}", memoized_greet.call(bob.clone()));   // Cache hit!
}
```

## 📌 설명
- HashMap<A, R>: 입력 인자(A)를 키로, 결과(R)를 값으로 저장합니다.
- #[derive(Hash, Eq, PartialEq)]: 구조체 Person을 해시 키로 쓸 수 있도록 자동 구현.
- Rc<RefCell<...>>: 캐시를 여러 클로저나 구조체 인스턴스에서 공유하고 수정할 수 있게 합니다.
- Memoize::call:
  - 캐시에 있으면 바로 반환 (Cache hit!).
  - 없으면 계산 후 캐시에 저장 (Computing...).

## ✨ Rust의 장점
- 구조체 해시/비교 연산을 매크로로 자동 구현 → C++보다 훨씬 간결.
- 안전한 메모리 관리 → 멀티스레드 환경에서도 Arc<Mutex<...>>로 쉽게 확장 가능.
- 성능은 C++과 동등하지만 코드 가독성과 유지보수성이 훨씬 뛰어남.

---

