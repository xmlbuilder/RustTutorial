

# Default::default

-  **Default::default()는 결국 T::default()를 호출하는 문법적 설탕(syntactic sugar)**.
- 그래서 타입이 Interval이면:
```rust
Interval::default()
```
- 과
```rust
Default::default()
```

- 은 완전히 동일한 동작을 함.
- 단, 전제 조건 : 컴파일러가 T가 무엇인지 알고 있어야 한다.


## 🔍 왜 둘이 같은가?
- Rust의 Default 트레이트:
```rust
pub trait Default {
    fn default() -> Self;
}
```

- 그리고 Default::default()는 실제로는:
```rust
<T as Default>::default()
```

- 이렇게 타입을 추론해서 호출.
- 즉, 아래 둘은 완전히 동일한 호출:
```rust
let a: Interval = Default::default();
let b = Interval::default();
```


## 🔍 언제 Default::default()를 쓰는 게 좋을까?
- ✔ 제네릭 코드에서 타입이 명확하지 않을 때
- 예를 들어:
```rust
fn foo<T: Default>() -> T {
    Default::default()
}
```

- 여기서는 T가 뭔지 모르니까 Interval::default()라고 쓸 수 없음.
- 그래서 Default::default()가 필요.

## 🔍 언제 Interval::default()가 더 좋을까?
- ✔ 타입이 명확할 때
- ✔ IDE에서 타입 추론이 잘 안 될 때
- ✔ 코드 가독성을 높이고 싶을 때
- 예:
```rust
let interval = Interval::default();
```        


## 🔍 예시로 비교
### 1) 타입이 명확한 경우
```rust
let iv = Interval::default();   // OK
let iv: Interval = Default::default(); // OK
```
- 둘 다 동일.

### 2) 제네릭 함수 안에서는?
```rust
fn make_default<T: Default>() -> T {
    Default::default()   // OK
    // T::default()      // OK
}
```

- 둘 다 가능하지만,  
    Default::default()가 더 일반적으로 쓰임.

### 3) 타입 추론이 안 되는 경우
```rust
let x = Default::default(); // ❌ 타입을 모르면 컴파일 에러
```

- 이럴 때는:
```rust
let x: Interval = Default::default(); // OK
```

- 또는
```rust
let x = Interval::default(); // OK
```


## 🌟 결론
- Default::default()는 Interval::default()와 완전히 동일한 동작을 한다.
- 단, 컴파일러가 타입을 알고 있어야 한다.

---


## 🟦 Option<T>의 Default
- ✔ 결론
- Option<T>의 default()는 **항상 None**이다.
- 즉:
```rust
let a: Option<i32> = Default::default();
let b = Option::<i32>::default();
```
- 둘 다:
```
None
```
- 이 된다.
- ✔ 왜 항상 None인가?
- Rust의 Option<T>는 이렇게 Default가 구현되어 있음:
```rust
impl<T> Default for Option<T> {
    fn default() -> Self {
        None
    }
}
```

- 즉, Some(T::default())가 아니라 무조건 None.
- ✔ 이유
- Option은 **값이 없을 수 있음** 을 표현하는 타입이기 때문에
    기본값은 **없음(None)** 이 가장 자연스럽기 때문.

## 🟦 Vec<T>의 Default
- ✔ 결론
- Vec<T>의 default()는 빈 벡터이다.
- 즉:
```rust
let v: Vec<i32> = Default::default();
let v2 = Vec::<i32>::default();
```

- 둘 다:
```
[]
```

- 이 된다.
- ✔ 왜 빈 벡터인가?
- Rust의 Vec<T>는 이렇게 구현되어 있어:
```rust
impl<T> Default for Vec<T> {
    fn default() -> Self {
        Vec::new()
    }
}
```

- 즉, Vec::default()는 Vec::new()와 완전히 동일.

## 🟦 Option<T> vs Vec<T> Default 비교
| 타입        | Default 값         | 의미                         |
|-------------|---------------------|------------------------------|
| Option<T>   | None                | 값이 없음                    |
| Vec<T>      | 빈 벡터 []          | 요소가 없음                  |


- 둘 다 “비어 있음”을 의미하지만, Option은 값 자체가 없음
- Vec은 값은 있지만 요소가 없음


## 🟦 Default::default() vs T::default()
- 둘은 완전히 동일한 동작이지만:
    - 타입이 명확하면 → T::default()
    - 제네릭 코드에서는 → Default::default()

---


