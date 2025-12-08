# Rc::as_ptr
```rust
let ptr = Rc::as_ptr(&comp) as *const (); 
```

- 이 코드의 목적을 단계적으로 풀어보면 다음과 같습니다:

## 1. Rc::as_ptr(&comp)
- Rc<T>는 참조 카운팅 스마트 포인터.
- Rc::as_ptr(&comp)는 내부적으로 Rc<T>가 관리하는 `실제 데이터(T)의 포인터` 를 가져옵니다.
- 반환 타입은 `*const T` 입니다. 즉, Rc<T>가 가리키는 값에 대한 불변 포인터.

## 2. as *const ()
- 여기서 *const ()로 캐스팅하는 이유는 타입 정보를 지워서 "raw pointer identity"만 남기기 위해서입니다.
- ()는 Rust에서 "unit type"이고, 아무 의미 없는 타입.
- 따라서 *const ()는 "어떤 타입이든 상관없이 그냥 포인터 주소만 필요하다"는 의도를 표현합니다.

## 3. 왜 이렇게 하냐?
- 비교/해시용: Rc::ptr_eq 같은 함수도 내부적으로 이런 식으로 포인터 주소를 비교합니다.
  - 즉, 두 Rc<T>가 같은 데이터를 가리키는지 확인할 때 타입은 중요하지 않고, 주소만 필요합니다.
- 타입 지우기(type erasure): 어떤 상황에서는 Rc<T>의 구체적인 T 타입을 몰라도 **같은 객체인지** 만 판별하고 싶을 수 있습니다.
- 안전한 식별자: *const ()는 **이 포인터는 단순히 식별용이지, 역참조하지 않는다** 는 의도를 명확히 합니다.

## 4. 예시 상황
```rust
use std::rc::Rc;
use std::collections::HashSet;

let a = Rc::new(5);
let b = Rc::clone(&a);

let mut set = HashSet::new();
set.insert(Rc::as_ptr(&a) as *const ());
set.insert(Rc::as_ptr(&b) as *const ());

// 같은 데이터를 가리키므로 set에는 하나만 들어감
println!("{}", set.len()); // 1
```

## ✅ 정리:
`Rc::as_ptr(&comp) as *const ()` 는 Rc가 가리키는 실제 데이터의 주소만 추출해서 타입을 지운 포인터로 만드는 코드입니다.  
주로 동일 객체 여부 판별이나 해시 키로 사용할 때 쓰이며, 역참조해서 데이터를 읽는 용도가 아니라 **identity(정체성)** 만 필요할 때 사용합니다.

---

## 실전 예제

- 이미 등록되어 있느지 확인용으로 사용
  
```rust
  pub fn attach_component(&mut self, comp: Rc<RefCell<dyn ArrayComp>>) {
      let ptr = Rc::as_ptr(&comp) as *const ();
      let exists = self
          .components
          .iter()
          .any(|rc| Rc::as_ptr(rc) as *const () == ptr);
      if !exists {
          self.components.push(comp);
      }
  }
```

---

