# Ownership & Borrowing

Rust의 Ownership과 Borrowing은 처음엔 좀 장황하게 느껴질 수 있지만,  
핵심만 잘 잡으면 아주 직관적이고 강력한 개념.

## 🧠 왜 Ownership이 필요한가?
Rust는 **GC(Garbage Collector)** 가 없음. 대신 컴파일 타임에 메모리 안전성을 보장하는 방식으로 설계.
- Java, Python, Go: 런타임에 GC가 메모리 정리
- C, C++: 개발자가 직접 malloc, free 또는 new, delete로 관리
- Rust: 컴파일러가 자동으로 메모리 해제를 결정함 → 안전 + 빠름

## 🧩 Ownership이란?
Rust에서는 **모든 값은 소유자(owner)** 를 갖습니다. 이 소유자는 변수입니다.

### 📜 소유권 규칙
- 모든 값은 하나의 owner를 가진다
- 한 번에 하나의 owner만 존재할 수 있다
- owner가 스코프를 벗어나면 값은 자동으로 drop된다
```rust
fn main() {
    let s = String::from("hello"); // s가 "hello"의 owner
    let t = s; // ownership이 s → t로 이동
    // println!("{}", s); // ❌ 에러! s는 더 이상 owner가 아님
}
```

➡️ s는 더 이상 사용할 수 없어요. Rust는 이중 해제나 dangling pointer를 방지하기 위해 이렇게 강제합니다.

### 🔄 Borrowing이란?
값을 직접 넘기지 않고 **참조(&)**를 통해 빌려 쓰는 방식입니다.
### ✅ 불변 참조
```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("hello");
    print_length(&s); // s를 빌려줌
    println!("{}", s); // ✅ 여전히 사용 가능
}
```
➡️ &s는 s를 빌려주는 것이고, 소유권은 유지됩니다.

### 🔧 가변 참조
```rust
fn add_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("hello");
    add_exclamation(&mut s);
    println!("{}", s); // hello!
}
```

➡️ &mut s는 s를 가변으로 빌려줌. 단, 한 번에 하나의 가변 참조만 허용됩니다.


## 🚫 왜 이런 규칙이 필요한가?
- **데이터 경쟁(race condition)** 을 방지
- 이중 해제(double free) 방지
- 런타임 비용 없이 안전성 확보
Rust는 이런 규칙을 컴파일 타임에 검사해서, 런타임 오류 없이 안전한 코드를 보장합니다.

## 📋 Ownership & Borrowing 요약표
| 개념         | 설명                                                                 |
|--------------|----------------------------------------------------------------------|
| Ownership    | 값은 하나의 owner를 가지며, owner가 스코프를 벗어나면 값은 drop됨     |
| Move         | 소유권을 다른 변수로 이동시키는 것 (`let b = a`)                     |
| Borrowing    | 참조를 통해 값을 빌려 사용 (`&T`, `&mut T`)                          |
| 불변 참조     | 여러 개 가능 (`&T`)                                                  |
| 가변 참조     | 한 번에 하나만 가능 (`&mut T`)                                       |
| Drop         | 소유자가 스코프를 벗어나면 자동으로 메모리 해제                      |
| 런타임 비용   | 없음. 모든 검사는 컴파일 타임에 이루어짐                             |

Rust의 Ownership은 처음엔 까다롭지만, 익숙해지면 버그 없는 시스템 프로그래밍을 가능하게 해주는 강력한 무기.

---

# 시각적 이해와 샘플
Rust의 Ownership과 Borrowing 개념을 시각적으로 이해하고, 실전 예제로 감 잡는 것이 핵심임.  
아래에 다이어그램, 코드 예제와 함께 엮어서 Rust의 메모리 관리 철학을 이해.

🧭 Ownership & Borrowing 다이어그램
Rust의 메모리 흐름을 시각적으로 표현한 다이어그램은:
![Ownership_Borrowing](/image/Ownership_Borrowing.png)
- 값이 스택에 생성되고
- 소유권이 이동(move)되며
- 참조(borrow)가 발생하고
- 스코프를 벗어나면 drop되는 흐름
이런 시각적 표현은 Rust의 **"누가 메모리를 책임지고 있는가?"** 를 직관적으로 이해하는 데 큰 도움이 됩니다.

## 🧪 실전 예제로 배우는 Ownership & Borrowing
### 1. Ownership 이동 (Move)
```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1; // s1의 소유권이 s2로 이동
    // println!("{}", s1); // ❌ 에러! s1은 더 이상 유효하지 않음
}
```
➡️ s1은 더 이상 사용할 수 없어요. Rust는 이중 해제 방지를 위해 소유권을 강하게 관리합니다.

📺 Rust: Ownership and Borrowing에서는 이런 소유권 이동이 왜 중요한지,  
그리고 GC 없는 Rust가 어떻게 안전성을 확보하는지 설명.

### 2. Borrowing (참조)
```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("Rust");
    print_length(&s); // s를 빌려줌
    println!("{}", s); // ✅ 여전히 사용 가능
}
```
➡️ &s는 소유권을 넘기지 않고 읽기 전용으로 빌려주는 방식입니다.

### 3. Mutable Borrowing (가변 참조)
```rust
fn add_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("Rust"); 
    add_exclamation(&mut s);
    println!("{}", s); // Rust!
}
```

➡️ &mut s는 한 번에 하나만 허용되는 가변 참조입니다. 이 규칙 덕분에 **데이터 경쟁(race condition)** 을 방지할 수 있음.

### 4. Stack vs Heap 메모리 구조
Rust는 값이 Stack에 있는지 Heap에 있는지에 따라 동작이 달라집니다.

| 구분       | Stack 메모리                             | Heap 메모리                              |
|------------|-------------------------------------------|-------------------------------------------|
| 저장 위치  | 함수 호출 시 자동으로 할당되는 메모리 영역 | 동적으로 할당되는 메모리 영역             |
| 저장 대상  | 지역 변수, 고정 크기 데이터                | 가변 크기 데이터, 구조체, 컬렉션 등       |
| 할당 속도  | 빠름                                       | 상대적으로 느림                           |
| 해제 방식  | 스코프 종료 시 자동 해제                   | 명시적 해제 또는 소유권에 따라 drop 호출 |
| 크기 제한  | 제한적 (스택 오버플로우 가능)              | 크고 유연함                                |
| 접근 방식  | LIFO 구조 (후입선출)                       | 포인터를 통해 접근                        |
| Rust에서의 사용 | `i32`, `bool`, `char` 등 Copy 타입         | `String`, `Vec`, `Box` 등 힙 할당 타입     |



#### 🦀 Rust의 Ownership 관점에서 비교
| 구분               | Stack (Copy 타입)                  | Heap (String, Vec 등)                  |
|--------------------|------------------------------------|----------------------------------------|
| 저장 위치          | Stack                              | Heap                                   |
| 소유권 이동        | `Copy`로 값 복사됨 (`i32`, `bool`) | `Move`로 소유권 이전됨 (`String`, `Vec`) |
| 참조 방식          | `&T` (불변 참조)                   | `&T`, `&mut T` (불변/가변 참조)         |
| 해제 방식          | 스코프 종료 시 자동 해제           | `drop()` 호출로 자동 해제              |
| 복사 가능 여부     | `Copy` 트레잇 구현됨               | `Clone` 호출 필요                      |
| 런타임 비용        | 없음                               | 있음 (힙 할당/해제 비용)               |
| Rust에서의 예시    | `let x: i32 = 5;`                  | `let s = String::from("hi");`          |



#### 🔍 예제 코드로 이해하기
##### ✅ Stack: 값 복사 (Copy)
```rust
fn main() {
    let x = 42;      // Stack에 저장
    let y = x;       // x는 Copy되므로 y도 사용 가능
    println!("{}, {}", x, y); // 둘 다 사용 가능
}
```

➡️ i32는 Copy 트레잇을 구현하므로 소유권 이동이 아닌 복사가 일어납니다.

##### 🧠 Heap: 소유권 이동 (Move)
```rust
fn main() {
    let s1 = String::from("hello"); // 힙에 저장
    let s2 = s1;                    // 소유권 이동
    // println!("{}", s1);          // ❌ 에러! s1은 더 이상 유효하지 않음
}
```

➡️ String은 힙에 데이터를 저장하므로, 소유권이 s2로 이동하면 s1은 무효화됩니다.

##### 🔄 Borrowing: 참조로 안전하게 접근
```rust
fn print_len(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("Rust");
    print_len(&s); // s를 빌려줌
    println!("{}", s); // ✅ 여전히 사용 가능
}
```

➡️ &s는 소유권을 넘기지 않고 읽기 전용 참조를 제공합니다.

#### 📌 핵심 요약
| 개념             | Stack                                | Heap                                 |
|------------------|--------------------------------------|--------------------------------------|
| 저장 위치        | 고정 크기 값                         | 동적 크기 값                         |
| 소유권 이동      | 복사(Copy)                           | 이동(Move)                           |
| 참조 방식        | &T (복사 또는 참조)                  | &T / &mut T (참조)                   |
| 해제 시점        | 스코프 종료 시 자동                  | drop() 호출로 자동 해제              |
| 런타임 비용       | 없음       



### 5. Drop과 Clone
```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1.clone(); // 깊은 복사
    println!("{}", s1); // ✅ 여전히 사용 가능
}
```

➡️ clone()을 사용하면 소유권을 유지하면서 복사할 수 있음. 단, 비용이 발생하므로 신중하게 사용해야 합니다.


### 6. Rust의 철학
Rust는 런타임 비용 없이 메모리 안전성을 확보합니다. 이게 바로 Rust의 가장 독특한 기능.

## 🧾 마무리 요약
| 개념             | 설명                                                                 | 예시 코드 또는 키워드       |
|------------------|----------------------------------------------------------------------|-----------------------------|
| Ownership        | 값은 하나의 owner를 가지며, owner가 스코프를 벗어나면 drop됨         | `let s = String::from("hi")` |
| Move             | 소유권을 다른 변수로 이동시키는 것                                   | `let s2 = s1`               |
| Borrowing        | 참조를 통해 값을 빌려 사용 (`&T`, `&mut T`)                          | `&s`, `&mut s`              |
| 불변 참조 (`&T`) | 여러 개 가능. 읽기만 가능. 소유권은 유지됨                           | `fn read(s: &String)`       |
| 가변 참조 (`&mut T`) | 한 번에 하나만 가능. 수정 가능. 소유권은 유지됨                     | `fn change(s: &mut String)` |
| Drop             | owner가 스코프를 벗어나면 자동으로 메모리 해제                        | `{ let s = String::from("bye"); }` |
| Clone            | 깊은 복사. 소유권 유지하며 값 복제                                    | `let s2 = s1.clone()`       |

---




