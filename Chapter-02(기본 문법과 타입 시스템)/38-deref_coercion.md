
# deref coercion

## 소스
```rust
trait Action {
    fn run(&self);
}

struct A;
struct B;

impl Action for A {
    fn run(&self) {
        println!("A runs fast");
    }
}

impl Action for B {
    fn run(&self) {
        println!("B runs slow");
    }
}

fn main() {
    let a: Box<dyn Action> = Box::new(A);
    let b: Box<dyn Action> = Box::new(B);

    a.run(); // prints "A runs fast"
    b.run(); // prints "B runs slow"
}
```

## 🔍 왜 * 없이도 .run()이 되나?
```rust
let a: Box<dyn Action> = Box::new(A);
a.run(); // ✅ 바로 호출됨!
```

Rust에서는 Box<T>가 Deref 트레잇을 구현하고 있어서,  
`a.run()` 은 내부적으로 `(*a).run()` 으로 자동 변환.  
즉, Rust가 자동으로 deref를 해주는 거예요. 이걸 `deref coercion` 이라고 부릅니다.  

## 🧠 deref coercion이란?
| 개념             | 설명 또는 수식 표현                                 |
|------------------|----------------------------------------------------|
| `Deref` 트레잇    | `Box<T>`는 `Deref`를 구현 → `*box`로 내부 값 접근 가능 |
| 메서드 호출       | `.run()`은 내부적으로 `(*box).run()`으로 변환됨       |
| trait 객체 호출   | `Box<dyn Trait>.method()` → `(*Box).method()` → vtable lookup |


## ✅ 예시 비교
```rust
// 직접 deref
(*a).run(); // OK

// 자동 deref (더 일반적이고 깔끔함)
a.run();    // OK — 컴파일러가 알아서 처리
```


## ✨ 요약 테이블
| 표현 방식     | 설명                                      |
|---------------|-------------------------------------------|
| `a.run()`     | Rust가 자동 deref → 내부 객체의 메서드 호출 |
| `(*a).run()`  | 명시적 deref → 같은 결과지만 덜 깔끔함     |
| deref coercion| Rust의 자동 deref 기능으로 생략 가능        |


## 🔍 Rust에서의 메서드 참조 흐름
```rust
trait Action {
    fn run(&self);
}

struct A;
impl Action for A {
    fn run(&self) {
        println!("A runs");
    }
}

let a: Box<dyn Action> = Box::new(A);
a.run(); // ✅ 여기서 "자기 함수"가 호출됨
```

- `a.run()` 은 사실상 `(*a).run()` 으로 자동 변환됨 → deref coercion
- `Box<dyn Action>` 는 내부적으로 vtable을 통해 A의 run()을 참조해서 실행
- 이건 **자기 타입에 맞는 함수 참조(reference)** 를 통해 실행되는 거예요

## 🧠 수식처럼 표현하면
```
a.run() ≈ (*a).run() ≈ vtable[run](&*a)
```

- vtable[run]은 A가 구현한 run() 함수의 포인터
- `&*a` 는 Box가 가리키는 실제 객체의 참조
- 결국 자기 타입에 맞는 함수가 실행됨 → “자기 함수 reference” 임!

## ✅ 요약 테이블
| 표현               | 의미                                               |
|--------------------|----------------------------------------------------|
| `a.run()`          | 자동 deref + vtable lookup                         |
| `(*a).run()`       | 명시적 deref → 같은 결과                           |
| `vtable[run](&*a)` | 실제 타입의 메서드 참조를 통해 실행됨              |


## y.abs() / *y.abs()
`y.abs()` 가 됩니다.
`*y.abs()` 는 안 됩니다.

### 🔍 왜 y.abs()가 되는가?
```rust
let x = 32.0;
let y = &x;
y.abs(); // ✅ OK!
```
- f64 타입은 Copy이기 때문에 &f64도 Deref를 통해 자동으로 f64처럼 동작합니다.
- Rust는 deref coercion을 통해 &x.abs()를 내부적으로 (*x).abs()로 바꿈.
- 그래서 y.abs()는 (*y).abs()와 같고, 결국 f64::abs()가 호출됩니다.

### ❌ 왜 *y.abs()는 안 되는가?
```rust
let x = 32.0;
let y = &x;
*y.abs(); // ❌ 컴파일 에러!
```
- `y.abs()`는 f64 값을 반환합니다.
- `*y.abs()`는 f64 값을 다시 deref하려는 시도인데, f64는 포인터가 아니므로 deref할 수 없음.
- 즉, `*` 는 참조를 해제할 때만 쓰는 거고, abs()는 이미 값이니까 deref할 필요가 없음.

✅ 요약 테이블
| 표현        | 결과         | 설명                                      |
|-------------|--------------|-------------------------------------------|
| `y.abs()`   | ✅ OK         | `&f64` → `f64`로 자동 deref 후 메서드 호출 |
| `(*y).abs()`| ✅ OK         | 명시적 deref 후 메서드 호출               |
| `*y.abs()`  | ❌ 에러       | `f64`는 포인터가 아니므로 deref 불가       |

Rust의 deref coercion 덕분에 코드가 훨씬 깔끔해지고 직관적.

## `*` 붙여야 하는 경우

Rust에서 `*` 를 붙여야 하는 경우는 주로 **참조를 해제(dereference)** 해서 값 자체에 접근하거나 수정할 때.
자동 deref가 안 되는 상황도 있고, 명시적으로 deref해야만 하는 경우도 있음.

## 🧪 1. 값 수정할 때는 반드시 * 필요
```rust
fn main() {
    let mut x = 10;
    let y = &mut x;

    *y += 5; // ✅ OK: y는 &mut i32 → *y로 값을 수정
    println!("{}", x); // 15
}
```

- &mut x는 i32에 대한 가변 참조
- 값을 바꾸려면 *y로 deref해서 직접 수정해야 함

## 🧮 2. 값 자체를 복사하거나 비교할 때
``` rust
fn main() {
    let x = 42;
    let y = &x;

    let z = *y + 1; // ✅ OK: y는 &i32 → *y로 값을 꺼냄
    println!("{}", z); // 43
}
```

- `*y` 는 i32 값을 꺼내는 deref
- 산술 연산 등에서는 값이 필요하므로 *가 필요함

## 🧠 3. 구조체 필드에 접근할 때도 * 필요할 수 있음
```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let r = &p;

    println!("{}", (*r).x); // ✅ OK
    println!("{}", r.x); // ✅ OK
}
```
- 구조체 참조에서 필드 접근은 자동 deref가 안 되는 경우가 있어요
- (*r).x처럼 명시적으로 deref해야 안전하게 접근 가능

## ✅ 요약 테이블
| 상황                     | `*` 필요 여부 | 설명                                       |
|--------------------------|----------------|--------------------------------------------|
| 값 수정 (`&mut T`)       | ✅ 필요         | 참조를 해제하고 값을 직접 수정해야 함       |
| 값 복사/비교             | ✅ 필요         | 참조를 해제해서 값 자체를 꺼내야 함         |
| 구조체 필드 접근          | ✅ 필요         | 자동 deref가 안 되는 경우 명시적 deref 필요 |
| 메서드 호출 (`y.abs()`)  | ❌ 불필요       | deref coercion으로 자동 처리됨              |


## 메소드 호출 시

구조체의 메서드 호출 시 *이 필요한지 여부에 대해 정리.  
Rust에서는 구조체에 대해 메서드를 호출할 때도 자동 deref가 작동하지만, 상황에 따라 *이 필요한 경우가 있음.

## 🧱 기본 구조체 메서드 호출
```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}
```

## ✅ 일반적인 호출
```rust
let p = Point { x: 1, y: 2 };
let r = &p;

r.sum();       // ✅ OK — 자동 deref로 &Point → Point
(*r).sum();    // ✅ OK — 명시적 deref
```

- r.sum()은 (*r).sum()으로 자동 변환됨 → deref coercion
- self가 &self이므로 &Point도 문제 없이 호출 가능

## 🔧 self가 값 타입일 때는 * 필요
```rust
impl Point {
    fn consume(self) -> i32 {
        self.x * self.y
    }
}
```

### ❌ 참조로는 호출 불가
```rust
let r = &Point { x: 3, y: 4 };

// r.consume();      // ❌ 에러: self가 값인데 r은 참조
(*r).consume();      // ✅ OK — 명시적 deref로 값 전달
````

- consume(self)는 Point 값을 직접 요구함
- &Point는 값이 아니므로 *r로 deref해서 전달해야 함

## ✅ 요약 테이블
| 메서드 타입      | 참조로 호출 가능 | `*` 필요 여부       | 설명                                      |
|------------------|------------------|----------------------|-------------------------------------------|
| `fn foo(&self)`  | ✅ 가능           | ❌ 자동 deref 됨     | 참조로도 호출 가능                        |
| `fn foo(self)`   | ❌ 불가           | ✅ 명시적 deref 필요 | 값 타입 요구 → 참조는 deref해서 전달해야 함 |
| `fn foo(&mut self)` | ✅ 가능        | ❌ 자동 deref 됨     | 가변 참조로도 자동 deref 가능             |

이제 구조체 메서드에서도 언제 `*` 이 필요한지 완벽하게 이해 가능.

---

# 연산자 우선 순위
연산자 우선 순위(operator precedence) 때문입니다.  
Rust에서는 `.` 연산자(메서드 호출)가 `*` 보다 우선순위가 높기 때문에, 아래 코드:
```rust
*y.abs()
```

은 실제로 이렇게 해석됩니다:
```rust
*(y.abs())
```

즉, y.abs()가 먼저 실행되고 그 결과(타입: f64)에 대해 `*` 를 시도.  
그런데 f64는 포인터가 아니기 때문에 `*` 로 역참조할 수 없어서 컴파일 에러가 발생합니다.

## ✅ 올바른 방식: 괄호로 우선순위 지정
```rust
(*y).abs() // ✅ OK
```

- `*y`가 먼저 실행되어 f64 값이 나오고
- 그 값에 대해 .abs() 메서드를 호출

## 🔍 Rust의 연산자 우선순위 요약 (일부)

| 연산자 | 우선순위 | 예시           | 설명                                 |
|--------|-----------|----------------|--------------------------------------|
| `.`    | 높음      | `x.abs()`      | 메서드 호출 또는 필드 접근           |
| `*`    | 중간      | `*ptr`         | 역참조 (포인터 해제)                 |
| `()`   | 제어용     | `(*x).abs()`   | 괄호로 연산자 우선순위 명시          |

---





