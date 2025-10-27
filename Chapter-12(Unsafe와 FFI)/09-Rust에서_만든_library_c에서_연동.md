## 🧱 1. Rust에서 .so 라이브러리 만들기
### Cargo.toml
```
[lib]
name = "my_rust_lib"
crate-type = ["cdylib"]
```

- cdylib은 C와 연동 가능한 동적 라이브러리를 의미합니다
## 예시 코드 (lib.rs)
```rust
#[repr(C)]
pub struct Config {
    pub mode: u32,
    pub timeout_ms: u64,
}

#[no_mangle]
pub extern "C" fn create_config(mode: u32, timeout: u64) -> *mut Config {
    let cfg = Config { mode, timeout_ms: timeout };
    Box::into_raw(Box::new(cfg))
}

#[no_mangle]
pub extern "C" fn destroy_config(ptr: *mut Config) {
    if !ptr.is_null() {
        unsafe { Box::from_raw(ptr); }
    }
}
```

## 빌드
```
cargo build --release
```

- 결과:
```
target/release/libmy_rust_lib.so
```


## 🔗 2. C에서 .so 라이브러리 링크하기
### 헤더 파일 (my_rust_lib.h)
```cpp
#ifndef MY_RUST_LIB_H
#define MY_RUST_LIB_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Config Config;

Config* create_config(unsigned int mode, unsigned long timeout);
void destroy_config(Config* cfg);

#ifdef __cplusplus
}
#endif

#endif
```

### C 코드에서 사용
```cpp
#include "my_rust_lib.h"

int main() {
    Config* cfg = create_config(1, 5000);
    // 사용...
    destroy_config(cfg);
    return 0;
}
```

### 컴파일 & 링크
```
gcc main.c -L./target/release -lmy_rust_lib -o main
```

- -L은 .so 파일이 있는 디렉토리
- -lmy_rust_lib는 libmy_rust_lib.so를 의미
### 실행 시 .so 경로 설정
```
export LD_LIBRARY_PATH=./target/release
./main
```

- 또는 rpath를 설정하거나 .so를 /usr/lib에 설치할 수도 있음


## ✅ 요약

| 항목             | 설명 또는 예시                  |
|------------------|----------------------------------|
| `cdylib`         | `libmy_rust_lib.so` 생성용 crate-type |
| `extern "C"`     | C와 ABI 호환되는 함수 선언 방식     |
| `gcc -L -l`      | `.so` 파일을 링크할 때 사용         |
| `LD_LIBRARY_PATH`| `.so` 파일의 실행 시 검색 경로 설정  |

 ### 전체 흐름 요약
- Rust에서 cdylib 설정 → .so 생성
- extern "C"로 함수 노출 → C에서 호출 가능
- C 코드에서 헤더 작성 후 gcc -L -l로 링크
- 실행 시 LD_LIBRARY_PATH로 .so 경로 지정

---


