# Rust 초기화
Rust에서는 객체 초기화 시 필드가 정의된 순서대로 초기화되며,  
상속이 없기 때문에 초기화 순서가 단순하고 명확합니다.  
함수 인자나 구조체 필드의 평가 순서도 왼쪽에서 오른쪽으로 진행됩니다.

## 🧭 Rust의 초기화 순서 요약
Rust는 객체 지향 언어가 아니기 때문에 Java처럼 상속 기반의 복잡한 초기화 순서는 없습니다.  
대신 **구조체(struct)** 와 필드 초기화, 함수 인자 평가, Drop 처리가 핵심입니다.

### ✅ 1. 구조체 필드 초기화 순서
- 구조체의 필드는 정의된 순서대로 초기화됩니다.
- 예를 들어:
```rust
struct MyStruct {
    a: String,
    b: String,
}

fn main() {
    let s = MyStruct {
        a: String::from("Hello"),
        b: String::from("World"),
    };
}
```

- 여기서 a가 먼저 초기화되고, 그 다음 b가 초기화됩니다.

### ✅ 2. 함수 인자 평가 순서
- 함수 인자의 평가 순서는 왼쪽에서 오른쪽입니다.
- 예:
```rust
fn f(a: u64, b: u64, c: u64) {}

fn g() -> u64 { println!("g"); 0 }
fn h() -> u64 { println!("h"); 1 }
fn i() -> u64 { println!("i"); 2 }

fn main() {
    f(g(), h(), i()); // 출력 순서: g → h → i
}
```       
- Rust는 명시적으로 왼쪽부터 평가하도록 정의되어 있어 예측 가능합니다.

### ✅ 3. Drop 순서 (소멸자 호출)
- 구조체가 스코프를 벗어나면서 drop될 때는 필드가 정의된 역순으로 drop됩니다.
- 예:
```rust
struct MyStruct {
    a: String,
    b: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct");
    }
}

fn main() {
    let s = MyStruct {
        a: String::from("A"),
        b: String::from("B"),
    };
}
```

- 내부적으로 b → a 순서로 drop됩니다.

### ✅ 4. 구조체 업데이트 문법 (..)
- 기존 인스턴스를 기반으로 일부 필드만 변경할 때도 정의된 순서대로 초기화됩니다.
- 예:
```rust
struct Config {
    width: u32,
    height: u32,
}

let base = Config { width: 800, height: 600 };
let updated = Config { width: 1024, ..base };
```

- width가 먼저 초기화되고, height는 base에서 복사됩니다.

## 📌 Rust 초기화 순서 요약표

| 항목                     | 설명 |
|--------------------------|------|
| 구조체 필드 초기화       | 필드 정의 순서대로 초기화됨 (`a`, `b`, `c` 순서 등) |
| 함수 인자 평가           | 왼쪽에서 오른쪽 순서로 평가됨 (`f(g(), h(), i())` → g → h → i) |
| Drop 처리 순서           | 구조체 필드는 정의된 **역순** 으로 drop됨 (`c` → `b` → `a`) |
| 구조체 업데이트 문법 `..` | 명시된 필드 먼저 초기화 → `..`로 복사된 필드가 나중에 채워짐 |

Rust는 초기화 순서가 명확하고 예측 가능하도록 설계되어 있어,  
안전성과 디버깅 측면에서 매우 유리합니다.

---
