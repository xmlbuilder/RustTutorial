# Lifetime
Rust의 라이프타임은 메모리 안전성을 보장하는 핵심 개념

## 🧠 Rust의 라이프타임(Lifetime) 핵심 요약
### 🔍 라이프타임이란?
- 참조가 유효한 기간을 명시하는 개념.
- Rust는 **소유권과 빌림(Borrowing)** 을 통해 메모리 안전성을 보장하며, 라이프타임은 빌림이 유효한지를 판단하는 기준.
- **Dangling reference(댕글링 참조)** 를 방지하기 위해 도입됨.
- 이미 해지된 객체에 대한 포인터에 접근 하는 경우를 방지 하기 위한 기법.
- 임대한 참조의 수명이 유효한지 검사하는 임대 검사
- 참조의 수명보다 원래값을 수명이 같거나 길어야 함.

## 📌 댕글링 참조
```rust 
let r; 
{ 
  let x = 5; 
  r = &x; 
} 
println!("{}", r);
```
이 코드는 x가 스코프를 벗어나면서 r이 댕글링 참조가 되어 컴파일 오류 발생.
→ [참조가 유효하지 않으면 컴파일되지 않음]


## 🧪 라이프타임 오류 예시
```rust
fn short_lifetime() {
    let x: &i32;
    {
        let y = 5;
        x = &y; // y는 스코프를 벗어나므로 x는 유효하지 않음
    }
    println!("x = {}", x); // 컴파일 오류
}


fn ok_lifetime() {
    let y = 5;
    let x = &y; // y가 스코프 내에 있으므로 x는 유효
    println!("x = {}", x);
}
```


## 🧬 제네릭 라이프타임 파라미터
```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

- 'a는 입력 참조자들의 공통 라이프타임을 나타냄.
- 반환값도 'a 라이프타임을 가져야 하므로, 두 입력 중 더 짧은 라이프타임을 따름.


### 📌 에러 예시
fn longest(s1: &str, s2: &str) -> &str → 컴파일 오류
→ 'a 라이프타임 명시로 해결됨
→ [ 컴파일러가 반환 참조의 출처를 알 수 없기 때문에 lifetime 명시 필요]


### 🧨 라이프타임 오류 사례
```rust
fn main() {
    let s1 = String::from("jhjeong");
    let res: &str;
    {
        let s2 = String::from("hyangseon");
        res = longest(s1.as_str(), s2.as_str()); // s2는 스코프를 벗어남
    }
    println!("ret = {}", res); // 컴파일 오류
}
```

#### 📌 s2 does not live long enough
→ s2 를 참조하고 있는데, 스코프를 벗어나므로 오류 발생
→ [이미지 설명: 빌림이 스코프를 벗어나면 참조가 유효하지 않음]

#### 이 코드가 오류 나지 않게 하는 방법
```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
           "y is no use here"
        }
    }
}
```

### 실패 하지 않는 경우
```rust
fn main() {
    let s1 =  String::from("jhjeong");
    let s2 = String::from("hyangseon");
    let s3 = longest(s1.as_str(), s2.as_str());
    println!("ret = {}", s3);
}
```

```
외부 스코프가 끝날 때까지 유효하고 s2 내부 스코프가 끝날 때까지 유효하며, s3 내부 스코프가 끝날 때까지 유효한 무언가를 참조합니다. 
빌림 검사기는 이 코드를 승인합니다; 
이는 컴파일되며 실행했을 때 hyangseon를 출력합니다:
```

```rust
fn main() {
    let s1 =  String::from("jhjeong");
    {
        let s2 = "hyangseon";
        let s3 = longest(s1.as_str(), s2);
        println!("ret = {}", s3);
    }
}
```


## 🧱 구조체에서의 라이프타임
struct ImportantPart<'a> {
    part: &'a str,
}

- 구조체가 참조자를 포함할 경우, 라이프타임 명시 필수.
- 구조체가 참조하는 데이터의 유효성을 보장
- 구조체의 필드가 참조하는 데이터가 구조체보다 먼저 해지되면 안 되기 때문.
```rust
impl<'a> ImportantPart<'a> {
    fn notice(&self, text: &str) -> &str {
        println!("notice {}", text);
        self.part
    }
}
```


### 🧠 수명 생략 규칙 (Lifetime Elision)
Rust는 일부 경우에 라이프타임을 자동 추론합니다.
- 입력 파라미터가 하나 → 그 라이프타임이 반환값에 적용됨
- 입력 파라미터가 여러 개 → 반환값에 적용할 수 없음
- 메서드에서 &self 사용 → self의 라이프타임이 반환값에 적용됨
```rust
fn first_word(s: &str) -> &str {
    s
}
```
→ 라이프타임 생략 규칙에 따라 'a를 명시하지 않아도 컴파일됨

## 🧊 정적 라이프타임 'static
```rust
fn static_lifetime() {
    let s: &'static str = "프로그램 실행 주 내내 유효한 수명";
}
```
- 'static은 프로그램 전체 생명주기 동안 유효한 참조
- 문자열 리터럴은 기본적으로 'static 라이프타임을 가짐


## 📦 실전 예제: Storage와 Statistics
```rust
struct Statistics<'a> {
    items: &'a HashMap<String, Item>,
}
```

- Statistics는 Storage의 데이터를 참조하므로, 'a 라이프타임 명시
- Storage가 먼저 해지되면 안 되므로, 컴파일러가 이를 체크
📌 설명: 참조된 객체가 먼저 해지되지 않도록 라이프타임 지정
→ [실전 예제: 창고 데이터 참조 구조체의 라이프타임 관리]

### 전체 코드
```rust
use std::collections::HashMap;

struct Item{
    name: String,
    price: f32,
    quantity:u32,
}

struct Storage {
    items: HashMap<String, Item>,
}

impl Storage {
    fn new() -> Self{

        Storage {
            items: HashMap::new(),
        }
    }

    fn storage_store(&mut self, item:Item){
        self.items.insert(item.name.clone(), item);
    }

    fn get_storage(&self) -> usize {
        self.items.len()
    }
}

struct Statistics<'a> {
    items: &'a HashMap<String, Item>,
}

impl<'a> Statistics<'a> {

    fn new(items: &'a HashMap<String, Item>) -> Self{
        Statistics {items}
    }

    fn get_average(&self) -> f32{
        let total = self.items.values().fold(0.0, |acc, i| acc + i.price);
        let count = self.items.values().fold(0, |acc, i| acc + i.quantity);
        total / (count as f32)
    }
}

fn main(){

    let mut my_storage = Storage::new();

    let apple = Item {
        name: "apple".to_string(),
        price : 1.0,
        quantity: 10,
    };

    my_storage.storage_store(apple);

    let banana = Item{
        name: "banana".to_string(),
        price: 2.0,
        quantity: 20
    };

    my_storage.storage_store(banana);

    let stats = Statistics::new(&my_storage.items);

    println!("{}", stats.get_average());
    println!("count = {}", my_storage.get_storage());
}
```


### 🧭 라이프타임을 이해하는 핵심 포인트

| 개념             | 설명                                                                 | 예시 또는 특징                                                                 |
|------------------|----------------------------------------------------------------------|--------------------------------------------------------------------------------|
| 참조의 유효성    | 참조는 원래 값이 유효한 동안만 사용할 수 있음                        | 댕글링 참조 방지 (`let r; { let x = 5; r = &x; }`)                             |
| 라이프타임 명시  | 컴파일러가 추론할 수 없을 때 명시적으로 라이프타임을 지정해야 함      | `fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str`                          |
| 구조체 내 참조   | 구조체가 참조자를 포함할 경우 라이프타임을 명시해야 함                | `struct ImportantPart<'a> { part: &'a str }`                                   |
| 생략 규칙        | Rust가 자동으로 라이프타임을 추론하는 규칙                            | 단일 파라미터, &self 사용 시 자동 추론 가능                                   |
| 라이프타임 오류  | 참조 대상이 스코프를 벗어나면 컴파일 오류 발생                        | `string2 does not live long enough`                                           |
| `'static`        | 프로그램 전체 생명주기 동안 유효한 참조                               | 문자열 리터럴: `let s: &'static str = "hello";`                               |




