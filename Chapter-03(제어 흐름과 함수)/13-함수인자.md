# 함수 인자

## 🧠 기본 방식: 구조체에 함수 인자 넘기기
### ✅ 1. 함수 포인터 (fn 타입)
```rust
struct Processor {
    callback: fn(i32) -> i32,
}

impl Processor {
    fn run(&self, value: i32) -> i32 {
        (self.callback)(value)
    }
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let p = Processor { callback: double };
    println!("{}", p.run(5)); // 출력: 10
}
```

fn(i32) -> i32는 함수 포인터 타입이에요.
클로저는 안 되고, 정적 함수만 가능합니다.


### ✅ 2. 클로저 지원: Box<dyn Fn(T) -> R>
```rust
struct Processor {
    callback: Box<dyn Fn(i32) -> i32>,
}

impl Processor {
    fn run(&self, value: i32) -> i32 {
        (self.callback)(value)
    }
}

fn main() {
    let closure = |x| x * 3;
    let p = Processor {
        callback: Box::new(closure),
    };
    println!("{}", p.run(4)); // 출력: 12
}
```

이 방식은 클로저, 함수 모두 지원하며,
Box<dyn Fn>은 트레이트 객체로 힙에 저장됩니다.


### ✅ 3. 제네릭 클로저 타입 (impl Fn or F: Fn)
```rust
struct Processor<F>
where
    F: Fn(i32) -> i32,
{
    callback: F,
}

impl<F> Processor<F>
where
    F: Fn(i32) -> i32,
{
    fn run(&self, value: i32) -> i32 {
        (self.callback)(value)
    }
}

fn main() {
    let p = Processor { callback: |x| x + 1 };
    println!("{}", p.run(10)); // 출력: 11
}
```

이 방식은 제네릭으로 타입을 고정하므로
성능이 좋고 인라인 최적화가 잘 됩니다.


## ✨ 요약 비교
| 방식                | 클로저 지원 | 성능       | 유연성         | 사용 시기                          |
|-----------------------------|--------------|------------|----------------|------------------------------------|
| `fn(T) -> R`                | ❌ 정적 함수만 | ✅ 매우 빠름 | ❌ 제한적       | 단순 함수 포인터만 필요할 때       |
| `Box<dyn Fn(T) -> R>`       | ✅ 가능       | ⚠️ 힙 할당   | ✅ 매우 유연함  | 런타임에 다양한 클로저를 다룰 때   |
| `F: Fn(T) -> R` (제네릭)    | ✅ 가능       | ✅ 인라인 최적화 | ⚠️ 타입 고정됨 | 성능이 중요하고 클로저 타입이 고정일 때 |


---


