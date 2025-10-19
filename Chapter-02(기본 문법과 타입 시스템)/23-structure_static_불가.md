# struct 안에 static?

## 🧠 왜 struct 안에 static 필드를 못 넣는가?
- Rust는 구조체 필드를 인스턴스마다 고유한 값으로 간주
- static은 프로그램 전체에서 공유되는 전역 값
- 따라서 구조체 안에 static 필드를 넣는 건 개념적으로 충돌

## ✅ 대안: impl 블록에 `associated constant` 또는 `static` 선언

### 1. Associated constant (const)
```rust
struct MyStruct {
    x: i32,
}

impl MyStruct {
    pub const MY_CONST: f64 = 3.1415;
}
```
#### 사용 예: `MyStruct::MY_CONST`

### 2. Module-level static
```rust
static MY_STATIC: &str = "Hello, world!";
```
구조체와 같은 모듈에 선언해서 전역처럼 사용할 수 있음


### 3. static을 구조체와 연관시키는 방법
```rust
struct MyStruct;

impl MyStruct {
    pub fn get_static_value() -> &'static str {
        "Shared value"
    }
}
```
이렇게 하면 `MyStruct::get_static_value()` 로 접근 가능  
내부적으로 `static` 값을 반환하지만 구조체와 연관

## ✨ 요약
| 구분         | 설명                                                                 | 사용 예시                                      |
|--------------|----------------------------------------------------------------------|------------------------------------------------|
| `const`      | 컴파일 타임에 값이 결정되는 상수. 복사 가능한 값만 가능.             | `pub const PI: f64 = 3.1415;`                  |
| `impl const` | 구조체나 enum의 연관 상수. `impl` 블록 안에서 선언 가능.             | `impl Point { pub const ORIGIN: Point = ... }` |
| `static`     | 프로그램 전체에서 공유되는 전역 변수. 런타임 초기화 가능.             | `pub static CONFIG: Config = Config::new();`   |


Rust는 전역 상태를 최소화하고, 명확한 `ownership` 을 강조하는 언어라서  
static을 구조체 내부에 넣는 대신 모듈 또는 impl 블록에서 관리하는 방식이 일반적.

----
