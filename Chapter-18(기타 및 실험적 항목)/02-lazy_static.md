# lazy_static
Rust에서 런타임 초기화가 필요한 전역(static) 변수를 선언할 때 아주 유용한 크레이트입니다.  
Rust는 기본적으로 컴파일 타임에 모든 static 변수의 값을 결정해야 하기 때문에, 복잡한 초기화가 필요한 경우에는 lazy_static이 해결책이 됩니다.

## 🧠 lazy_static이란?
### 📌 개념
- Rust의 static 변수는 컴파일 타임에 초기화되어야 함.
- 하지만 어떤 경우에는 함수를 호출하거나 계산을 통해 초기화해야 할 수도 있음.
- 이럴 때 lazy_static을 사용하면 런타임에 한 번만 초기화되는 static 변수를 만들 수 있음.
- 내부적으로는 스레드 안전한 lazy 초기화를 보장함 (std::sync::Once 사용).


### 🔧 사용 예시 (이미지 기반 코드 활용)
이미지에 나온 코드를 기반으로 설명해볼게요:
```rust
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref STATIC: i32 = define_static();
}

fn define_static() -> i32 {
    3
}

fn main() {
    println!("{}", *STATIC);
}
```

### 💡 설명
- define_static() 함수는 i32 값을 반환하는 일반 함수.
- STATIC은 lazy_static! 매크로를 통해 선언된 런타임 초기화 static 변수.
- main()에서 STATIC을 사용할 때, 최초 접근 시점에 초기화됨.
- *STATIC으로 역참조하여 값을 출력.

### 🧭 `lazy_static` 언제 사용하면 좋을까?

| 상황                             | 설명                                                                 | 예시 또는 사용 대상                          |
|----------------------------------|----------------------------------------------------------------------|---------------------------------------------|
| 복잡한 초기화가 필요한 전역 변수 | 함수 호출이나 계산이 필요한 static 변수                             | 설정 파일 파싱, 환경 변수 로딩              |
| 전역 컬렉션                      | 프로그램 전체에서 공유되는 데이터 구조                               | `HashMap`, `Vec`                             |
| 외부 라이브러리 초기화           | 런타임에 초기화가 필요한 외부 크레이트 사용                          | `regex`, `chrono`, `reqwest`                |
| 테스트용 상수 또는 공통 설정     | 여러 테스트나 모듈에서 공유되는 값                                   | 테스트용 전역 문자열, 공통 설정 구조체      |
| 스레드 안전한 전역 리소스       | 여러 스레드에서 안전하게 접근 가능한 리소스                         | DB 커넥션 풀, 캐시 객체                      |



### 🧊 예시: 전역 HashMap
```rust
use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("host", "localhost");
        m.insert("port", "8080");
        m
    };
}

fn main() {
    println!("Host: {}", CONFIG.get("host").unwrap());
}
```

- CONFIG는 프로그램 전체에서 접근 가능한 전역 설정 맵.
- 초기화는 블록 내에서 수행되며, 첫 사용 시점에 실행됨.

### 🚀 `lazy_static` 요약

| 항목               | 설명                                                                 | 관련 기술 또는 특징     |
|--------------------|----------------------------------------------------------------------|--------------------------|
| 초기화 시점        | 런타임에 최초 접근 시 한 번만 초기화됨                              | `lazy_static!` 매크로    |
| 사용 목적          | 복잡한 계산이나 함수 호출이 필요한 static 변수 선언                  | 설정, 캐시, 전역 리소스  |
| 스레드 안전성      | 내부적으로 `std::sync::Once`를 사용하여 안전하게 초기화              | ✅ 안전한 병렬 접근 가능 |
| 타입 유연성        | 기본 타입뿐 아니라 구조체, 컬렉션 등 다양한 타입 지원                | `HashMap`, `Vec`, 구조체 |
| 대안 크레이트      | 최신 Rust에서는 `once_cell`도 유사한 기능을 제공                     | `OnceCell`, `Lazy`       |


---

