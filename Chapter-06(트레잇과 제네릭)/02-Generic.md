# 제네릭
제네릭의 개념과 활용법을 체계적으로 정리.

## 🧠 제네릭(Generic)이란?
**제네릭(Generic)** 은 함수, 구조체, 트레이트 등을 특정 타입에 국한하지 않고 타입 파라미터를 통해 다양한 타입에 대해 동작하도록 만드는 기능입니다.
### ✅ 제네릭의 장점
- 재사용성: 하나의 코드로 여러 타입을 처리할 수 있어 중복을 줄임
- 유연성: 다양한 타입을 받아들이면서도 타입 안정성을 유지
- 효율성: 컴파일 시 타입이 결정되므로 런타임 오버헤드 없음

## 🔤 타입 파라미터 사용법
이미지에서 설명된 것처럼, 제네릭 타입은 <T>와 같이 표현합니다.
```rust
fn foo<T>(arg: T) { ... }
```

- T는 i32처럼 고정된 타입이 아니라 임의의 타입을 의미합니다.
- 여러 타입을 동시에 사용하려면 <T, U>처럼 여러 파라미터를 지정할 수 있습니다.

## 📦 구조체에서의 제네릭
```rust
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}
```

- Point<T>는 x와 y가 같은 타입
- Point2<T, U>는 서로 다른 타입도 허용
### ❌ 타입 불일치 오류 예시
```rust
let int_float = Point { x: 5, y: 10.0 };
// error[E0308]: mismatched types
```

→ Point<T>는 x와 y가 같은 타입이어야 하므로 오류 발생

## 🔀 제네릭 메서드와 타입 혼합
```rust
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 { x: self.x, y: other.y }
    }
}
```
### 사용법
```rust
let int_float = Point2 { x: 5, y: 10.0 };

let integer2 = Point2 { x: 5, y: 10 };
let float2 = Point2 { x: 0.1, y: 4.0 };

let mixed = integer2.mixup(float2);
println!("mixed.x = {}, mixed.y = {}", mixed.x, mixed.y);
//mixed.x = 5, mixed.y = 4
```

- mixup은 두 개의 Point2를 받아서 x는 첫 번째, y는 두 번째에서 가져옴
- 타입 파라미터를 자유롭게 조합할 수 있음

## 🔍 제네릭 함수로 최소값 찾기
### 타입별로 반복되는 코드
```rust
fn smallest_i32(list: &[i32]) -> &i32 { ... }
fn smallest_char(list: &[char]) -> &char { ... }
```
#### 사용법
```rust
fn main() {
    let item = [1, 2, 3, 4, 5, 6, -10];
    let small = smallest_i32(&item);
    println!("smallest = {small}");
    let item_char = ['a', 'b', 'c', 'd', 'e', 'f'];
    let small_char = smallest_char(&item_char);
    println!("small char = {}", small_char);
}

fn main() {
    let numbers = vec![3, 4, 1, 6, 8, 10];
    let result = smallest_i32(&numbers);
    println!("가장 작은 수는 {}", result);
    let chars = vec!['홍', '길', '동'];
    let result = smallest_char(&chars);
    println!("가장 작은 글자는 {}", result);
    
}
```

### 제네릭으로 통합
```rust
fn smallest<T: PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];
    for item in list {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}
```

#### 사용법
```rust
fn main() {
    let numbers = vec![3, 4, 1, 6, 8, 10];
    let result = smallest(&numbers);
    println!("가장 작은 수는 {}", result);
    let chars = vec!['홍', '길', '동'];
    let result = smallest(&chars);
    println!("가장 작은 글자는 {}", result);
    let result = smallest(&["jhjeong", "hyangseon", "hyunji", "hyunah"]);
    println!("가장 작은 글자는 {}", result); //가장 작은 글자는 hyangseon

}
```

- PartialOrd 트레이트를 통해 비교 가능하도록 제한
- 다양한 타입(i32, char, &str)에 대해 동일한 로직 사용 가능

## 🧩 트레이트 바운드(Trait Bound)
제네릭 타입에 트레이트 제약을 걸어 특정 기능을 보장할 수 있습니다.
### 기본 문법
```rust
fn some_function<T: Display>(t: &T) {
    println!("{}", t);
}
```

### 복합 바운드
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
```

### where 절로 가독성 향상
```rust
fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{} {:?}", t, u);
}
```


### 🖨️ 트레이트 바운드를 활용한 출력 함수
```rust
fn print_number<T: Debug>(number: T) {
    println!("Here is your number : {:?}", number);
}
```

또는
```rust
fn print_number1(number: &impl Display) {
    println!("Here is your number : {:?}", number);
}
```

→ impl Trait 문법은 간결하게 표현할 수 있는 방법으로 이미지에서도 언급됨

## 🐶 구조체와 제네릭 함수 활용
```rust
#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your name: {:?}", item);
}
```

### 사용법
```rust
fn main(){

    let charile = Animal {
        name: "Charlie".to_string(),
        age : 1,
    };

    print_item(charile);
    let number = 5;
    print_item(number);
    
    
}
```

→ Animal 구조체와 i32 등 다양한 타입을 받아 출력 가능

## ⚖️ 비교와 출력을 동시에
```rust
fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num1: U, num2: U) {
    println!("{} Is {} greater than {}? {}", statement, num1, num2, num1 > num2);
}
```

### 사용법
```rust
use std::fmt::Display;
use std::cmp::PartialOrd;

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num1: U, num2: U){
    println!("{} Is {} greater than {}? {}", statement, num1, num2, num1 > num2);
}

fn main(){
    compare_and_display("Listen up!", 9, 8);
    //Listen up! Is 9 greater than 8? true
}

```
→ Display로 출력하고, PartialOrd로 비교하는 복합 기능 구현

## 🧩 마무리 요약

| 개념           | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| 제네릭         | 다양한 타입을 처리할 수 있는 추상화 기능                             |
| 타입 파라미터  | `<T>`, `<T, U>` 형태로 표현하며 임의의 타입을 의미                    |
| 트레이트 바운드| `T: Display + Clone`처럼 타입에 기능 제약을 걸어 안전성과 유연성 확보 |
| where 절       | 복잡한 바운드를 `where`로 분리해 가독성 향상                         |
| 재사용성       | 중복 코드를 줄이고 유지보수 용이                                     |
| 유연성         | 다양한 타입을 받아들이면서도 타입 안정성 유지                        |

---

