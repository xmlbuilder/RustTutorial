#구조체 Destructuring(구조체 분해) 개념
이 기능은 구조체의 필드를 개별 변수로 해체해서 추출하는 방식으로, 코드의 가독성과 유연성을 높여줍니다.

## 🧱 구조체 Destructuring이란?
Destructuring은 구조체의 필드를 하나하나 꺼내서 개별 변수로 바인딩하는 문법입니다.
이는 패턴 매칭의 일종이며, 구조체의 내부 데이터를 쉽게 접근하거나 재구성할 때 유용합니다.

## 📌 기본 문법
```rust
let StructName { field1, field2, ... } = instance;
```

또는 필드 이름을 새 변수로 바꾸고 싶을 때:
```rust
let StructName { field1: new_var1, field2: new_var2 } = instance;
```


## 🧪 예제 ①: Person 구조체 분해
```rust
#[derive(Debug)]
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

fn main() {
    let papa_doc = Person {
        name: "Person Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    let Person {
        name: a,
        real_name: b,
        height: c,
        happiness: d,
    } = papa_doc;

    println!(
        "They call him {} but his real name is {}. his is {} cm tall and is he happy? {}",
        a, b, c, d
    );
}
```


## 🔍 설명
| 코드 라인                                      | 설명                                                                 |
|------------------------------------------------|----------------------------------------------------------------------|
| `let papa_doc = Person { ... }`               | `Person` 구조체 인스턴스를 생성하고 `papa_doc` 변수에 저장합니다.     |
| `let Person { name: a, ... } = papa_doc;`     | 구조체를 분해하여 각 필드를 `a`, `b`, `c`, `d`라는 새 변수에 바인딩합니다. |
| `a`, `b`, `c`, `d`                             | 각각 `name`, `real_name`, `height`, `happiness` 값을 갖습니다.       |
| `println!(...)`                               | 분해된 변수들을 사용하여 정보를 출력합니다.                          |


#### 📌 주의: papa_doc의 필드들은 모두 String, u8, bool 타입으로 소유권이 이동되므로 이후 papa_doc을 사용할 수 없습니다.
→ println!("{:?}", papa_doc);는 컴파일 에러 발생!

## 🧪 예제 ②: City 구조체 참조 기반 분해
```rust
#[derive(Debug)]
struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
        City {
            name,
            name_before,
            population,
            date_founded,
        }
    }
}

fn process_city_values(city: &City) {
    let City {
        name,
        name_before,
        population,
        date_founded,
    } = city;

    let two_names = vec![name, name_before];
    println!("The city's two names are {:?}", two_names);
}

fn main() {
    let tallinn = City::new("Tallinn".to_string(), "Reval".to_string(), 426_538, 1219);
    process_city_values(&tallinn);
}
```


## 🔍 설명
| 코드 라인                            | 설명                                                                 |
|-------------------------------------|----------------------------------------------------------------------|
| `let tallinn = City::new(...)`      | `City` 구조체 인스턴스를 생성하고 `tallinn` 변수에 저장합니다.         |
| `process_city_values(&tallinn)`     | 구조체 참조를 함수에 전달하여 내부 데이터를 처리합니다.               |
| `let City { name, ... } = city;`    | 구조체를 참조 기반으로 분해하여 각 필드를 변수로 바인딩합니다.         |
| `name`, `name_before`               | 각각 `String` 타입의 참조 (`&String`)로 바인딩됩니다.                  |
| `vec![name, name_before]`           | 두 개의 이름을 벡터로 묶어 출력합니다.                                |


#### 📌 이 경우는 참조를 분해했기 때문에 소유권을 가져가지 않으며, tallinn은 이후에도 계속 사용 가능!

## 🧠 정리
| 문법 또는 개념                        | 설명                                                                 |
|-------------------------------------|----------------------------------------------------------------------|
| `let Struct { a, b } = instance;`   | 구조체를 직접 분해하며 **소유권을 가져옵니다**. 이후 원본 인스턴스는 사용할 수 없습니다. |
| `let Struct { a, b } = &instance;`  | 구조체를 **참조 기반으로 분해**하여 소유권을 유지합니다. 원본 인스턴스를 계속 사용할 수 있습니다. |
| `field: new_name`                   | 구조체 필드를 **새 변수명으로 바인딩**할 수 있습니다. 예: `name: a`는 `name` 필드를 `a` 변수에 저장. |
| 소유권 주의                         | `String`, `Vec`, `Box` 등은 소유권을 가져오므로 직접 분해 시 원본 구조체는 더 이상 유효하지 않습니다. |

---



