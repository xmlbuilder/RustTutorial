# trait 심화
 이 코드는 Rust의 트레이트 시스템, 특히 associated type, trait object, 그리고 다중 트레이트 바운드를 활용한 예제.  
 
## 🧩 1. trait Printable 정의
```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```


### ✅ 핵심 개념: `trait Printable`

| 요소         | 설명                                      |
|--------------|-------------------------------------------|
| `type Age`   | 연관 타입 (associated type)으로 `Age`의 타입을 정의 |
| `print()`    | 객체의 정보를 출력하는 메서드               |
| `get_age()`  | `Age` 타입의 값을 반환하는 메서드            |

→ Self::Age는 트레이트를 구현한 타입이 지정한 Age 타입을 의미합니다.

## 👤 2. Person 구조체와 Printable 구현
```rust
struct Person {
    name: String,
    age: u32,
}


impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}
```

- Person은 Printable을 구현하며 Age 타입을 u32로 지정
- print()는 이름과 나이를 출력
- get_age()는 age 필드를 반환

## 📚 3. Book 구조체와 Printable 구현
```rust
struct Book {
    title: String,
    author: String,
    published: u32,
}


impl Printable for Book {
    type Age = u32;
    fn print(&self) {
        println!("Title: {}\nAuthor: {}\nPublished: {}", self.title, self.author, self.published);
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}
```

- Book도 Printable을 구현하며 Age를 u32로 지정
- print()는 책 정보를 출력
- get_age()는 출판 연도를 반환

### 🧠 4. Trait Object와 dyn 키워드
```rust
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}
``` 

### ✅ 설명: `dyn Printable` 사용 맥락
| 요소             | 설명                                                       |
|------------------|------------------------------------------------------------|
| `dyn Printable`   | `Printable` 트레이트를 구현한 객체를 참조하는 trait object |
| `Age = u32`       | `Printable` 트레이트의 연관 타입 `Age`가 `u32`로 고정됨     |
| `item.print()`    | trait object를 통해 `print()` 메서드를 동적 디스패치로 호출 |


→ dyn은 동적 디스패치를 통해 다양한 타입을 하나의 인터페이스로 처리할 수 있게 해줍니다.

### 🔗 5. 다중 트레이트 바운드
```rust
trait Trait1 {}
trait Trait2 {}

fn some_function(param: &(dyn Trait1 + Trait2)) {}
```

### ✅ 설명: 다중 트레이트 객체와 참조
| 요소                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| `dyn Trait1 + Trait2` | 두 개 이상의 트레이트를 동시에 구현한 객체를 trait object로 참조함     |
| `&(...)`              | 트레이트 객체는 반드시 참조 형태로 사용해야 하며, 소유권을 가지지 않음 |

→ 이 방식은 다형성을 확장하여, 복합적인 기능을 가진 객체를 처리할 수 있게 합니다.

### 🎯 전체 요약: `Printable` 트레이트 활용

| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| `trait Printable` | 연관 타입 `Age`를 포함한 출력용 트레이트 정의                         |
| `impl Printable`  | `Person`, `Book` 구조체가 `Printable`을 구현하며 `Age = u32`로 지정    |
| `dyn Printable`   | 런타임에 `Printable`을 구현한 객체를 참조하는 trait object             |
| `Age = u32`       | trait object 사용 시 `Printable`의 연관 타입을 `u32`로 제한             |
| `dyn Trait1 + Trait2` | 두 개 이상의 트레이트를 동시에 구현한 객체를 trait object로 참조함     |

## 전체 코드
```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}

struct Person {
    name: String,
    age : u32
}

impl Person {
    fn new(name: &str, age: u32) ->Self {
        Person {
            name : name.to_string(),
            age : age,
        }
    }
}

impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}

struct Book {
    title: String,
    author: String,
    published: u32
}

impl Printable for Book {
    type Age = u32;
    fn print(&self) {
        println!("Title: {}\nAuthor: {}\nPublished: {}", self.title, self.author, self.published);
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}

fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}

fn main() {
    let person = Person::new("Alice", 22);
    let book = Book{
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228
    };

    print_info(&person); //Name: Alice, 22 years old
    print_info(&book);
    // Title: The Rust Programming Language
    // Author: Steve Klabnik and Carol Nichols
    // Published: 20230228
}


// Dyn 키워드가 중요하다.
// "dyn Printable"은 Printable 트레이드를 구현한 객체를 의미 합니다.
// 또한 Age타입을 u32로 정의한 객체라는 제약 조건을 줄 수 있습니다.
// 만약에 여러개의 트레이트륵 구현하는 타입을 지정하고 싶은면 다음과 같이 사용.

trait Trait1 {
}

trait Trait2 {
}

fn some_function(param: &(dyn Trait1 + Trait2)) {
}
```

---


