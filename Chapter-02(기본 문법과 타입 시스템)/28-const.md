# const

## 🦀 Rust에서 const 선언
```rust
const PI: f64 = 3.141592;
```

## ✅ 특징
- 컴파일 타임 상수: 반드시 컴파일 시점에 값이 결정되어야 함
- 타입 명시 필수: const는 타입을 생략할 수 없음
- 전역/지역 모두 가능: 함수 안에서도 선언 가능
- 불변이며 재할당 불가: let과 달리 절대 변경 불가

## 🔒 Rust의 static과의 차이
```rust
static GREETING: &str = "Hello";
```
| 키워드   | 설명                                       |
|----------|--------------------------------------------|
| `const`  | 컴파일 타임 상수. 복사되어 사용되며 타입 명시 필수 |
| `static` | 런타임에 메모리에 저장되는 전역 상수. 참조됨       |

static은 전역 메모리에 저장되며, mut와 함께 쓰면 가변 전역 변수도 가능하지만 unsafe가 필요.


## 🧪 C#의 const vs readonly 비교
| 키워드     | 특징 및 설명                                      |
|------------|--------------------------------------------------|
| `const`    | 컴파일 타임 상수. 자동으로 `static` 처리됨. 선언과 동시에 초기화 필요 |
| `static`   | 클래스 수준의 정적 멤버. 모든 인스턴스가 공유함               |
| `readonly` | 런타임 상수. 생성자에서 초기화 가능. 참조 타입도 지원됨        |

### C# 예시
```csharp
public const double Pi = 3.14;           // 컴파일 타임 상수
public readonly int Id;                 // 생성자에서 초기화 가능

public MyClass(int id) {
    this.Id = id;                       // 여기서 값 설정 가능
}
```


### 🔍 Rust에는 readonly 같은 개념이 있을까?
Rust에는 readonly 키워드는 없지만, **불변 변수(let)** 와 **불변 참조(&T)** 가 그 역할을 합니다.
```rust
let x = 5;          // 불변 변수
let y = &x;         // 불변 참조
```

- Rust의 기본은 불변성이기 때문에 readonly가 따로 필요 없음
- 가변성을 원할 경우 let mut x = ...처럼 명시적으로 선언해야 합니다

## ✨ 요약 비교
| 언어   | 키워드     | 초기화 시점 | 변경 가능 여부 | 참조 타입 지원 | 메모리 위치     |
|--------|------------|-------------|----------------|----------------|-----------------|
| Rust   | `const`    | 컴파일 타임 | ❌ 불가능       | ❌ 제한적       | 복사됨           |
| Rust   | `static`   | 컴파일 타임 | ✅ (unsafe 필요) | ✅ 가능         | 전역 메모리      |
| C#     | `const`    | 컴파일 타임 | ❌ 불가능       | ❌ (string만 가능) | 메타데이터에 기록 |
| C#     | `readonly` | 런타임      | ✅ 생성자에서 가능 | ✅ 가능         | 힙 메모리         |

Rust는 기본적으로 불변을 우선시하는 철학을 갖고 있어서, C#처럼 readonly를 따로 만들 필요가 없음.

---

# Static 변수

Rust에서 static 변수는 전역 상수처럼 동작하지만, mut로 선언하면 전역 가변 변수가 됩니다.  
다만 Rust는 안전성을 최우선으로 하기 때문에 static mut을 사용할 때는 반드시 unsafe 블록이 필요.

## 🧪 기본 예제: static mut로 문자열 변경하기
```rust
static mut GREETING: &str = "Hello";

fn main() {
    unsafe {
        GREETING = "안녕하세요";
        println!("{}", GREETING); // 출력: 안녕하세요
    }
}
```

### ✅ 핵심 포인트
- static mut은 전역 가변 변수를 선언할 수 있게 해줍니다
- 하지만 **모든 접근(읽기/쓰기)** 에 unsafe가 필요
- 이유는 데이터 레이스 방지 때문 (멀티스레드 환경에서 위험)

## ⚠️ 권장 방식: `Mutex` 또는` RwLock` 사용
Rust에서는 static mut보다 아래를 사용하는 것이 훨씬 안전하고 일반적.
### ✅ 예제: Mutex<String>로 안전한 전역 문자열
```rust
use std::sync::Mutex;

static GREETING: Mutex<String> = Mutex::new(String::from("Hello"));

fn main() {
    {
        let mut greeting = GREETING.lock().unwrap();
        *greeting = String::from("안녕하세요");
    }

    let greeting = GREETING.lock().unwrap();
    println!("{}", *greeting); // 출력: 안녕하세요
}
```

이 방식은 스레드 안전하며 unsafe 없이도 전역 상태를 안전하게 다룰 수 있음.


### 🧠 고급: lazy_static 또는 once_cell과 함께 쓰기
Rust에서는 전역 가변 상태를 초기화할 때 lazy_static!이나 once_cell::sync::Lazy를 자주 사용합니다.
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;
static GREETING: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::from("Hello")));
```

이 방식은 초기화 시점 제어 + 스레드 안전성을 동시에 만족.

## ✨ 요약 비교
| 방식             | 안전성 | 스레드 안전 | 초기화 시점 | 사용 권장 |
|------------------|--------|--------------|--------------|------------|
| `static mut`     | ❌ unsafe 필요 | ❌ 위험함     | 컴파일 타임 | ❌ 피해야 함 |
| `Mutex<String>`  | ✅ 안전 | ✅ 안전함     | 런타임       | ✅ 추천     |
| `Lazy + Mutex`   | ✅ 안전 | ✅ 안전함     | 지연 초기화   | ✅ 고급 추천 |

---



