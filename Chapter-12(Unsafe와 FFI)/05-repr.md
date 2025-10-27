# 핵심 개념: #[repr(u32)]와 C 연동
Rust의 enum은 기본적으로 내부적으로 자동으로 값이 할당되며,  
그 값은 컴파일러가 최적화에 따라 다르게 처리할 수 있습니다.  
하지만 C와 연동하려면 정확한 메모리 표현과 값이 필요하므로,  
`#[repr(u32)]` 를 사용해 C 스타일로 정수형 표현을 강제합니다. 
 
```rust
#[repr(u32)]
enum Bar {
    A,         // 0
    B = 10000, // 명시적 지정
    C,         // 10001 (자동 증가)
}
```

- `#[repr(u32)]` → enum을 u32 타입으로 표현하겠다는 뜻
- B = 10000 → 명시적으로 값 지정
- C → 이전 값(B)이 10000이므로 자동으로 10001이 됨

## 🔧 출력 결과
```rust
fn main() {
    println!("A: {}", Bar::A as u32); // 0
    println!("B: {}", Bar::B as u32); // 10000
    println!("C: {}", Bar::C as u32); // 10001
}
```

- as u32 → enum 값을 명시적으로 정수로 변환
- 출력:
```
A: 0
B: 10000
C: 10001
```


## 🛠️ C 연동에서의 실전 활용
### C 코드 예시:
```cpp
typedef enum {
    A = 0,
    B = 10000,
    C = 10001
} Bar;
```

## Rust와 연동 시:
- Rust에서 #[repr(u32)]를 지정하면
→ C에서 Bar enum과 메모리 표현이 동일해짐
- FFI (Foreign Function Interface)로 C 함수와 안전하게 연결 가능

```cpp
extern "C" {
    fn process_bar(value: Bar);
}
```

이때 Bar는 C에서 정의된 enum과 값과 타입이 정확히 일치해야 하므로
`#[repr(u32)]` 는 필수입니다.


## 🧩 C++ 쪽 대응 코드
```cpp
#include <cstdint>
#include <iostream>

// Rust와 동일한 메모리 표현을 갖는 enum
enum Bar : uint32_t {
    A = 0,
    B = 10000,
    C = 10001
};

// Rust에서 extern "C"로 선언된 함수를 C++에서 구현
extern "C" void process_bar(Bar value) {
    switch (value) {
        case A:
            std::cout << "Received: A (0)" << std::endl;
            break;
        case B:
            std::cout << "Received: B (10000)" << std::endl;
            break;
        case C:
            std::cout << "Received: C (10001)" << std::endl;
            break;
        default:
            std::cout << "Unknown value: " << static_cast<uint32_t>(value) << std::endl;
            break;
    }
}
```

## ✅ 핵심 포인트: Rust ↔ C++ enum 연동
| 항목                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| enum Bar : uint32_t   | C++에서 Rust의 #[repr(u32)] enum과 메모리 표현을 맞추기 위해 사용         |
| #[repr(u32)]          | Rust에서 enum을 u32로 표현 → C++의 uint32_t enum과 호환됨               |
| extern "C"            | C++에서 Rust 함수와 ABI를 맞추기 위한 선언 → 이름 맹글링 방지             |
| switch                | C++에서 enum 값을 처리하는 방식 → 각 variant에 대한 분기 처리             |
| static_cast<uint32_t> | enum 값을 정수로 출력하거나 비교할 때 사용 → 디버깅 및 로깅에 유용         |

이제 Rust에서 process_bar(Bar::B);를 호출하면 C++ 쪽에서 Received: B (10000)이 출력됩니다.


## ✅ 보강 포인트
| 항목                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| 메모리 표현 강제       | #[repr(u32)]로 enum을 C 스타일 u32로 표현 → FFI에서 메모리 호환성 확보     |
| 값 지정 가능           | 각 variant에 명시적으로 값을 지정 가능 → C enum과 정확히 일치시킬 수 있음 |
| 자동 증가 규칙         | 명시된 값 다음 variant는 자동으로 +1 증가됨                           |
| 형 변환 (as u32)       | enum variant를 정수로 변환할 때 사용 → 로깅, 디버깅, FFI에 유용             |
| 타입 안정성 주의       | as u32는 안전하지만, 숫자 기반 로직은 타입 안정성을 해칠 수 있어 제한적으로 사용 |


## 💡 결론
#[repr(u32)]는 Rust에서 C와 안전하게 연동하기 위한 메모리 표현 강제 도구입니다.  
enum 값이 정확히 일치해야 하는 상황에서는 명시적 지정과 자동 증가 규칙을 잘 이해하고 있어야  
실전에서 버그 없이 연동할 수 있음.

---

# Rust를 통한 메모리 할당 및 해제

## ✅ Rust에서도 같은 방식 가능
Rust에서는 #[repr(C)]를 사용해 구조체 레이아웃을 명시하고, extern "C"로 C 스타일 API를 제공합니다.
```rust
#[repr(C)]
pub struct DriverConfig {
    pub mode: u32,
    pub timeout_ms: u64,
}

#[no_mangle]
pub extern "C" fn create_config(mode: u32, timeout: u64) -> *mut DriverConfig {
    let config = DriverConfig { mode, timeout_ms: timeout };
    Box::into_raw(Box::new(config))
}

#[no_mangle]
pub extern "C" fn destroy_config(ptr: *mut DriverConfig) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(ptr); } // drop
    }
}
```

###  C나 C++ 에서 사용
```cpp
// Rust에서 노출된 헤더
typedef struct DriverConfig DriverConfig;

DriverConfig* create_config(unsigned int mode, unsigned long timeout);
void destroy_config(DriverConfig* ptr);



// 사용 예
DriverConfig* cfg = create_config(1, 5000);
// cfg->mode, cfg->timeout_ms 사용 가능 (구조체 정의가 있다면)
destroy_config(cfg);

```

## 💬 결론
구조체를 노출하되 내부는 숨기고, 메모리 관리는 반드시 라이브러리 내부 함수로만 처리하는 방식은
Windows에서 디버그/릴리스 호환성과 ABI 안정성을 확보하는 가장 실용적인 전략입니다.








