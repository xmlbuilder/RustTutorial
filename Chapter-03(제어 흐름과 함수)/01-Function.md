# Function
Rust에서 함수 선언은 매우 직관적이면서도 강력한 기능을 제공. 
- fn 키워드를 사용하여 함수를 정의합니다.
- 매개변수는 이름: 타입 형식으로 작성합니다.
- 반환 타입은 -> 타입으로 명시합니다.
- 함수 본문은 {} 중괄호로 감싸며, return 키워드로 값을 반환할 수 있습니다.


## 🧠 Rust 함수 선언 기본 구조

Rust에서 함수를 선언할 때는 fn 키워드를 사용하며, 다음과 같은 형식을 따릅니다:
```rust
fn 함수이름(매개변수: 타입, ...) -> 반환타입 {
    // 함수 본문
}
```

### 예시:
```rust
fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}
```

- `fn`: 함수 선언 키워드
- `add`: 함수 이름
- `(num1: i32, num2: i32)`: 매개변수와 타입
- `-> i32`: 반환 타입
- `return num1 + num2;`: 반환 값


## 📦 함수 위치에 대한 Rust의 특징
```rust
fn main() {
    a_function();
}

fn a_function() {
    println!("I am a function");
}
```

- Rust는 함수의 위치를 신경 쓰지 않습니다.
- 호출하는 함수보다 뒤에 정의되어 있어도 컴파일러가 자동으로 인식합니다.
- 이는 C/C++에서의 `forward declaration` 과는 다른 Rust의 유연한 구조입니다.

## 🛠️ 함수 선언 요약 표 (Markdown)
| 항목         | 설명                                               |
|--------------|----------------------------------------------------|
| 키워드       | `fn`으로 함수 선언                                 |
| 매개변수     | `이름: 타입` 형식으로 작성                         |
| 반환 타입    | `-> 타입`으로 명시                                 |
| 본문         | `{}` 중괄호로 감싸고 `return`으로 값 반환 가능     |
| 위치 유연성  | 함수의 위치는 호출 순서와 무관, 어디든 정의 가능  |

---

# C/C++에서의 forward declaration(전방 선언)

컴파일러에게 어떤 식별자(주로 함수나 클래스)가 존재한다는 사실을 미리 알려주는 방식입니다.  
이 개념은 특히 컴파일 순서, 헤더 파일 분리, 순환 참조 해결 등에 매우 중요하게 작용합니다.

## 🧠 Forward Declaration이란?
- 정의 없이 선언만 먼저 하는 것입니다.
- 컴파일러가 해당 식별자를 사용할 수 있도록 미리 알려주는 역할을 합니다.
- 실제 정의는 나중에 등장해도 괜찮습니다.

### 예시:
```cpp
// 함수의 forward declaration
void sayHello();  // 선언

int main() {
    sayHello();   // 사용 가능
}

void sayHello() { // 정의
    std::cout << "Hello!" << std::endl;
}
```

## 🧱 왜 필요한가?
### 1. 컴파일러는 위에서 아래로 읽는다
- C/C++ 컴파일러는 소스 코드를 위에서 아래로 순차적으로 읽기 때문에, 사용 전에 선언이 필요합니다.
  
### 2. 헤더 파일 분리와 모듈화
- 큰 프로젝트에서는 여러 파일로 나누어 관리합니다.
- forward declaration을 통해 불필요한 include를 줄이고, 컴파일 속도를 개선할 수 있습니다.
  
### 3. 순환 참조 해결
클래스 간 서로를 참조할 때 순환 참조 문제가 발생할 수 있습니다. 

이때 forward declaration이 유용합니다.
```cpp
// A.h
class B;  // forward declaration

class A {
    B* b;  // B의 포인터 사용 가능
};
```


## 🧩 클래스의 forward declaration
클래스는 `포인터` 나 `참조` 로 사용할 경우 `forward declaration` 만으로 충분합니다.  
하지만 멤버 변수로 `직접 포함` 하거나 `메서드를 호출` 하려면 정의가 필요합니다.
```cpp
class B;  // forward declaration

class A {
    B* b;  // OK: 포인터는 크기를 알 필요 없음
    // B b2; // ❌ Error: B의 크기를 알아야 하므로 정의 필요
};
```


## ⚠️ 주의할 점
- forward declaration은 정의가 반드시 존재해야 합니다. 선언만 있고 정의가 없으면 link error 발생.
- **구조체(struct)** 도 forward declaration 가능하지만, 마찬가지로 사용 방식에 따라 정의가 필요합니다.
- 템플릿 클래스나 함수는 forward declaration이 까다롭고 일반적으로 정의가 함께 있어야 합니다.

## 🧪 실전 팁
- 헤더 파일에는 가능한 한 `forward declaration` 을 사용하고, 필요한 경우에만 `#include` 를 사용하세요.
- 순환 참조가 발생할 수 있는 경우, 포인터나 참조로 설계하고 forward declaration을 활용.
- forward declaration은 컴파일 시간 최적화에 큰 도움이 됩니다.

---



