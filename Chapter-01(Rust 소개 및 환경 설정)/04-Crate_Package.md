# 📦 Crate vs Package 개념 요약
| 개념        | Rust 설명                                                                 | C++ 대응 개념                          |
|-------------|---------------------------------------------------------------------------|----------------------------------------|
| Crate       | 컴파일러가 한 번에 처리하는 **컴파일 단위**. 하나의 실행 파일 또는 라이브러리 생성 | `.cpp` 파일 단위 또는 라이브러리(.lib/.dll/.so) |
| Binary Crate| `main.rs`가 포함된 실행 가능한 크레이트. 실행 파일 생성                    | `main.cpp` → 실행 파일 생성             |
| Library Crate| `lib.rs`가 포함된 라이브러리 크레이트. 다른 Crate에서 참조 가능            | `.lib`, `.dll`, `.so` 형태의 라이브러리 |
| Package     | 여러 Crate를 묶은 **Cargo 프로젝트 단위**. `Cargo.toml`로 구성 정의         | CMake 프로젝트 또는 디렉토리 기반 구성 |
| Cargo.toml  | Crate 및 의존성, 빌드 설정을 정의하는 메타데이터 파일                      | `CMakeLists.txt`, `Makefile`, `.vcxproj` |
| Dependencies| `[dependencies]` 섹션에 외부 Crate 추가 가능                               | `find_package()` 또는 `vcpkg`, `Conan` |



## 🧠 Rust의 구조적 특징
- Crate는 Rust 컴파일러가 처리하는 최소 단위입니다.
- 하나의 Crate는 하나의 결과물(실행 파일 또는 라이브러리)을 만듭니다.
- Crate는 여러 모듈(mod)로 구성될 수 있습니다.
- Package는 Cargo가 관리하는 프로젝트 단위입니다.
- 하나의 Package는 최대 하나의 라이브러리 Crate와 여러 개의 바이너리 Crate를 포함할 수 있습니다.
- Cargo.toml 파일로 전체 Package의 빌드 방식과 의존성을 정의합니다.
예: src/lib.rs + src/bin/binary1.rs, src/bin/binary2.rs → 하나의 Package에 여러 Crate 포함


## 🧩 C++에서의 대응 방식
Rust의 구조는 C보다 더 명확하고 통합적입니다. C에서는 다음과 같이 구성됩니다:
- Crate ≈ 컴파일 단위
- .cpp 파일 하나 또는 여러 파일을 컴파일하여 하나의 실행 파일 또는 라이브러리 생성
- Package ≈ 프로젝트 디렉토리
- CMake 프로젝트 또는 Visual Studio 솔루션이 여러 라이브러리와 실행 파일을 포함할 수 있음
- Cargo.toml ≈ CMakeLists.txt / Makefile
- 빌드 설정, 의존성, 타겟 정의를 포함

## 🔍 예시 비교
Rust
```
[package]
name = "my_package"
version = "0.1.0"

[[bin]]
name = "binary1"
path = "src/bin/binary1.rs"

[[bin]]
name = "binary2"
path = "src/bin/binary2.rs"
```

## C++
```
add_executable(binary1 src/bin/binary1.cpp)
add_executable(binary2 src/bin/binary2.cpp)
add_library(my_lib src/lib.cpp)
```


## ✅ 요약
- Rust는 Crate = 컴파일 단위, Package = Crate 모음이라는 구조로 명확하게 분리되어 있습니다.
- C++은 Crate에 해당하는 개념이 파일 또는 라이브러리 단위로 암묵적으로 존재하며, Package는 프로젝트 디렉토리나 CMake 설정으로 구성됩니다.
- Rust의 Cargo는 의존성 관리, 빌드, 테스트, 배포까지 통합적으로 처리하는 반면, C++은 여러 도구를 조합해야 비슷한 기능을 구현할 수 있습니다.
