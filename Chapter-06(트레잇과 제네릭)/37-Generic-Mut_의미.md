# Generic mut
- Rust에서 함수 파라미터를 &mut T로 받으면, 값 자체를 소유하는 게 아니라 **mutable reference**를 소유하게 됩니다.
- 그래서 함수 안에서 value는 **참조** 이고, 실제 값을 바꾸려면 반드시 `역참조(*value)` 를 해야 합니다.

## 단계적으로 설명
- 함수 시그니처
```rust
pub fn on_limit_range<T>(low: T, value: &mut T, high: T)
```
- low, high는 값 자체를 받습니다 (소유).
- value는 &mut T, 즉 가변 참조를 받습니다.
  - 원본 변수를 직접 수정할 수 있게 해줍니다.
  
- 역참조 필요성
  - value 자체는 참조 타입(&mut T)이므로, 바로 대입할 수 없습니다.
  - `*value` 로 역참조해야 실제 값(T)에 접근하거나 수정할 수 있습니다.
  - 예시:
```rust
  *value = low;   // 참조가 가리키는 실제 값을 변경
```
- mutable reference의 의미
- 호출하는 쪽에서:
```rust
let mut x = 5.0;
on_limit_range(0.0, &mut x, 10.0);
```
- 여기서 `&mut x` 를 넘겼으므로, 함수 안에서 `*value` 를 바꾸면 x가 직접 수정됩니다.
- 왜 그냥 value = low가 안 되나?
  - value는 &mut T 타입이지 T 타입이 아닙니다.
  - value = low라고 쓰면  **참조 자체를 다른 참조로 바꾸려는 시** 가 되는데, Rust에서는 함수 파라미터로 받은 참조를 재바인딩할 수 없습니다.
  - 따라서 반드시 `*value = low` 처럼 역참조해서 원본 값을 수정해야 합니다.

## 간단 예시
```rust
fn main() {
    let mut x = 5.0;
    on_limit_range(0.0, &mut x, 10.0);
    println!("{}", x); // 함수 안에서 수정된 값이 반영됨
}
```

- 👉 결론:
  - `value` 는 `mutable reference` 라서 직접 대입할 수 없고, `*value로 역참조` 해야 원본 값을 바꿀 수 있습니다.

---

## Generic trait bound

- 지금 말씀하신 건 제네릭(generic)의 특정화 또는 제약(bound) 개념. 
- Rust에서 제네릭을 쓸 때는 **아무 타입이나 받아라** 라고 할 수도 있지만,  
  대부분은 그 타입이 어떤 연산을 지원해야만 함수가 제대로 동작합니다. 
- 그래서 **트레이트 바운드(trait bound)** 를 걸어 특정 타입만 허용.

### 🔑 제네릭의 특정(제약) 원리
- 제네릭 기본형

```rust
pub fn foo<T>(x: T) {
    // 여기서는 T가 어떤 타입인지 전혀 모름
}
```
- 아무 타입이나 들어올 수 있지만, 내부에서 할 수 있는 게 거의 없음.
- 트레이트 바운드로 특정

```rust
pub fn foo<T: Copy>(x: T) {
    // 이제 T는 Copy 가능한 타입만 들어올 수 있음
}
```

- Copy 트레이트를 구현한 타입만 허용. 따라서 String은 안 되고 i32, f32는 가능.

- 여러 제약을 동시에
```rust
pub fn bar<T>(x: T, y: T) -> T
where
    T: Copy + PartialOrd + Sub<Output = T>,
{
    if x < y { x } else { y }
}
```
- T는 반드시 Copy, PartialOrd, Sub를 구현해야 함.
- 즉, 비교와 뺄셈이 가능한 타입만 들어올 수 있음.
- 특정 트레이트로 제한
```rust
use num_traits::Float;

pub fn baz<T: Float>(x: T) -> T {
    x.sqrt() + T::epsilon()
}
```
- 이제 T는 f32나 f64 같은 부동소수점 타입만 가능. 정수형은 불가능.

## 📌 요약
- 제네릭(generic): 타입을 일반화해서 함수/구조체를 여러 타입에 대해 재사용 가능하게 함.
- 특정(bound): 제네릭 타입이 반드시 특정 트레이트를 구현해야 한다는 제약을 거는 것.
- 실무 활용: Copy, Clone, PartialOrd, Debug, Float 같은 트레이트를 걸어 필요한 연산만 가능한 타입을 받도록 제한.

- 👉 즉, generic의 특정은 **제네릭을 아무 타입이나 받지 말고, 내가 원하는 성질을 가진 타입만 받도록 제한하는 것** 임.

---


