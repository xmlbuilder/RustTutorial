
# 🧠 함수 포인터 vs 클로저
Rust에서는 함수를 값처럼 다룰 수 있습니다. 즉, 함수 포인터를 통해 함수를 다른 함수에 인자로 넘길 수 있고, 클로저도 동일한 방식으로 전달 가능합니다.
## ✅ 함수 포인터
```rust
fn fun1(n: i32) -> bool {
    n % 3 == 0
}

fn fun2(n: i32) -> bool {
    n % 5 == 0
}
```

- fun1, fun2는 일반 함수이며, fn(i32) -> bool 타입의 함수 포인터로 사용됩니다.

## ✅ 함수 포인터를 인자로 받는 함수
```rust
fn fun3(fun1: fn(i32) -> bool, fun2: fn(i32) -> bool) {
    for i in 0..100 {
        if fun1(i) && fun2(i) {
            println!("case1");
        } else if fun1(i) {
            println!("case2");
        } else if fun2(i) {
            println!("case3");
        } else {
            println!("case4");
        }
    }
}
```


- fun3은 두 개의 함수 포인터를 받아서 조건에 따라 분기합니다.
- fn(i32) -> bool 타입으로 명시되어 있으므로 클로저는 직접 전달 불가합니다.

## ❌ 클로저를 직접 전달하면 오류 발생
```rust
let fn1 = |x| x % 3 == 0;
let fn2 = |x| x % 5 == 0;

fun3(fn1, fn2); // ❌ 컴파일 오류 발생
```


- 클로저는 fn 타입이 아니라 **impl Fn(i32) -> bool** 타입입니다.
- 따라서 fun3의 시그니처를 제네릭으로 바꿔야 클로저도 받을 수 있어요.

## ✅ 클로저와 함수 포인터 모두 받는 버전
```rust
fn fun3<F1, F2>(fun1: F1, fun2: F2)
where
    F1: Fn(i32) -> bool,
    F2: Fn(i32) -> bool,
{
    for i in 0..100 {
        if fun1(i) && fun2(i) {
            println!("case1");
        } else if fun1(i) {
            println!("case2");
        } else if fun2(i) {
            println!("case3");
        } else {
            println!("case4");
        }
    }
}
```

- 이렇게 하면 함수 포인터와 클로저 모두 전달 가능해집니다.
- **fun3(fun1, fun2)도 되고, fun3(|x| x % 3 == 0, |x| x % 5 == 0)** 도 됩니다.

## 📦 요약: 함수 포인터와 클로저 비교
| 항목             | 함수 포인터 (`fn`)                  | 클로저 (`|x| ...`)                          |
|------------------|--------------------------------------|---------------------------------------------|
| 타입             | `fn(i32) -> bool`                   | `impl Fn(i32) -> bool`                      |
| 상태 캡처        | 불가능                               | 가능 (외부 변수 사용 가능)                  |
| 유연성           | 고정된 함수만 가능                   | 동적으로 생성 가능, 환경 캡처 가능          |
| 제네릭 필요 여부 | 없음                                 | 있음 (`F: Fn(...)` 형태로 제네릭 필요)      |

---

# Fn, FnMut, FnOnce
Fn, FnMut, FnOnce의 차이를 이해하면 함수형 프로그래밍도 훨씬 자유롭게 다룰 수 있음.

## 🧠 Fn, FnMut, FnOnce 차이점
Rust에서는 클로저가 환경을 어떻게 캡처하는지에 따라 세 가지 트레이트로 분류됩니다:

| 트레이트   | 호출 방식     | 캡처 방식     | 특징 및 사용 예시                     |
|------------|----------------|----------------|----------------------------------------|
| `Fn`       | `&self`        | 불변 참조 `&`  | 환경을 읽기만 함. 여러 번 호출 가능     |
| `FnMut`    | `&mut self`    | 가변 참조 `&mut` | 환경을 수정 가능. 여러 번 호출 가능     |
| `FnOnce`   | `self`         | 값 이동 `move` | 환경을 소비함. 한 번만 호출 가능        |


💡 모든 클로저는 최소 FnOnce를 구현합니다. FnMut은 FnOnce를 포함하고, Fn은 FnMut과 FnOnce를 모두 포함합니다.


## 🔍 예제: 각각의 클로저 구현 방식
### ✅ FnOnce 예제
```rust
fn main() {
    let s = String::from("hello");
    let closure = move || {
        drop(s); // s를 move해서 삭제
    };
    closure(); // ✅ 한 번 호출 가능
    // closure(); // ❌ 두 번째 호출은 오류
}
```


### ✅ FnMut 예제
```rust
fn main() {
    let mut s = String::from("hello");
    let mut closure = || {
        s.push_str(" world");
    };
    closure();
    closure();
    println!("{}", s); // "hello world world"
}
```

## ✅ Fn 예제
```rust
fn main() {
    let s = String::from("hello");
    let closure = || {
        println!("{}", s); // 읽기만 함
    };
    closure();
    closure(); // 여러 번 호출 가능
}
```


## 🧪 클로저를 반환하는 고차 함수
Rust에서는 함수를 반환하는 함수, 즉 고차 함수도 만들 수 있어요.  
클로저를 반환할 때는 impl Fn 또는 Boxed Trait Object를 사용합니다.
### ✅ impl Fn 방식
```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(10)); // 출력: 15
}
```


- x는 클로저 내부로 move되어 캡처됨.
- 반환된 클로저는 Fn을 구현하므로 여러 번 호출 가능.

### ✅ Boxed Trait Object 방식
```rust
fn make_multiplier(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x * y)
}

fn main() {
    let mul3 = make_multiplier(3);
    println!("{}", mul3(4)); // 출력: 12
}
```


- Box<dyn Fn>을 사용하면 동적 디스패치가 가능해져 유연성이 높아짐.
- 특히 반환 타입이 복잡하거나 조건에 따라 달라질 때 유용해요.

## 📦 요약: 클로저 반환 고차 함수
| 반환 방식           | 설명                                               | 예시 사용 |
|--------------------|----------------------------------------------------|-----------|
| `impl Fn(...)`     | 컴파일 시 타입 결정, 성능 우수                     | 일반적인 경우 |
| `Box<dyn Fn(...)>` | 런타임에 타입 결정, 유연성 높음                    | 조건부 반환 등 |



----

## 🧠 drop 함수란?
```rust
pub fn drop<T>(_x: T)
```

- drop은 값을 받아서 즉시 소멸시키는 함수입니다.
- 소유권을 가져오고, 해당 값은 스코프가 끝나기 전에 강제로 제거됩니다.
- 이때 Rust는 해당 타입에 구현된 Drop 트레이트의 drop() 메서드를 자동으로 호출합니다.

## ✅ 예제: drop 사용
```rust
fn main() {
    let s = String::from("hello");
    println!("Before drop");
    drop(s); // s는 여기서 소멸됨
    println!("After drop"); // s는 더 이상 사용할 수 없음
}
```

- drop(s)을 호출하면 s는 즉시 메모리에서 해제됩니다.
- 이후 s를 사용하면 컴파일 오류가 발생합니다.

## 🔍 왜 직접 drop()을 호출할까?
| 상황                             | 설명                                                                 | 예시 코드 또는 용도                          |
|----------------------------------|----------------------------------------------------------------------|---------------------------------------------|
| 리소스를 조기에 해제하고 싶을 때 | 스코프가 끝나기 전에 메모리를 해제하여 리소스를 절약하거나 충돌 방지 | `drop(file);` → 파일 핸들 조기 해제          |
| 이후 코드에서 해당 값을 참조하면 안 될 때 | 실수로 참조하지 않도록 명시적으로 제거                              | `drop(sensitive_data);` → 보안상 안전하게 제거 |
| `Drop` 트레이트의 동작을 테스트할 때 | 소멸 시점에 로그를 남기거나 동작을 확인하고 싶을 때                 | `drop(my_struct);` → 로그 출력 확인          |
| 스마트 포인터 내부 참조 수 제어 시 | `Rc`, `Arc` 등의 참조 카운트를 수동으로 줄이고 싶을 때              | `drop(rc_value);` → 참조 수 감소              |



