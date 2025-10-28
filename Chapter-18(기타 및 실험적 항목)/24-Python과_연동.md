# Rust와 Python 연동

## 🧠 핵심 개념: Rust ↔ Python 연동 방식
Rust는 C ABI를 따르는 방식으로 라이브러리를 만들 수 있기 때문에,  
Python에서는 ctypes, cffi, 또는 PyO3 같은 도구를 통해 Rust 라이브러리를 불러올 수 있음.

### ✅ 방법 1: ctypes로 .so 또는 .dll 호출

#### Rust 라이브러리 만들기
- Cargo.toml
```
[lib]
crate-type = ["cdylib"]
```

#### 🧩 선택적 / 상황별 Crate

| Crate 이름       | 용도 설명 |
|------------------|-----------|
| `pyo3-asyncio`   | Rust의 `async` 함수를 Python의 `asyncio`와 연결할 때 사용. 비동기 연동이 필요할 때 유용 |
| `pyo3-numpy`     | Python의 `numpy.ndarray`를 Rust에서 안전하게 다룰 수 있게 해줌. 과학 계산, 이미지 처리 등에서 활용 |
| `serde`          | JSON, TOML 등 직렬화/역직렬화에 사용. Python ↔ Rust 간 데이터 변환 시 매우 유용 |
| `anyhow`         | 다양한 에러 타입을 하나로 감싸서 간단하게 에러 처리할 수 있게 해줌. 실전 코드에서 에러 핸들링을 깔끔하게 |
| `thiserror`      | 사용자 정의 에러 타입을 선언적으로 만들 수 있게 해줌. `anyhow`과 함께 쓰면 에러 처리 구조가 매우 명확해짐 |


- lib.rs
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
```
cargo build --release
```
- 결과:
```
 target/release/libyourlib.so (Linux)
```

#### Python에서 사용
```python
from ctypes import cdll

lib = cdll.LoadLibrary("./target/release/libyourlib.so")
result = lib.add(3, 5)
print("결과:", result)
```


### ✅ 방법 2: PyO3 + maturin으로 직접 Python 모듈 생성
이건 더 고급 방식으로, Rust 코드를 Python 모듈처럼 pip로 설치 가능하게 만들 수 있음.
#### Rust 코드
```rust
use pyo3::prelude::*;

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pymodule]
fn mylib(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
```

#### 설치 방법
- Rust 프로젝트 내부
```
cargo add pyo3 --features extension-module
```

- maturin은 Python 쪽에 설치
```
pip install maturin
maturin develop
```


#### Python에서 사용
```python
import mylib

print(mylib.add(3, 5))
```


## 💬 결론
Rust로 만든 라이브러리를 Python에서 사용하는 건 성능 + 안정성 + 생산성을 동시에 잡을 수 있는 전략.  
간단한 함수부터 복잡한 구조체, 설정 로직까지 넘길 수 있고, 드라이버 설정이나 데이터 처리 같은 영역에서 특히 강력하게 작동합니다.  

---

# 🚀 PyO3가 뛰어난 이유

## ✅ Pythonic한 API 제공
- Rust에서 직접 `#[pyfunction]`, `#[pymodule]` 로 Python 함수/모듈을 정의 가능
- Python에서 import mylib처럼 자연스럽게 사용 가능

## ✅ 빌드/배포 자동화 (maturin)
- maturin을 사용하면 pip install .로 설치 가능
- Python 패키지처럼 배포 가능 (.whl 생성)

## ✅ 타입 안전성과 성능
- Rust의 타입 시스템과 성능을 그대로 활용
- Python에서 느린 로직을 Rust로 대체 가능

## ✅ 구조체, 클래스, 예외 처리까지 지원
- `#[pyclass]`, `#[pymethods]` 로 Python 클래스처럼 노출 가능
- Rust에서 Python 예외를 던질 수도 있음

## 🧪 간단 예제: PyO3로 Python 모듈 만들기
```rust
use pyo3::prelude::*;

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pymodule]
fn mylib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
```


## Python에서 사용
```python
import mylib
print(mylib.add(3, 5))  # 출력: 8
```

## PyO3 기반 프로젝트 템플릿, `#[pyclass]` 로 Python 객체 노출하는 방법


## 💬 결론
- Rust에서 Python 모듈을 만들려면 pyo3는 필수,
- 빌드와 배포를 위해 maturin을 Python 쪽에 설치하면 됩니다.
- 필요에 따라 serde, pyo3-asyncio, pyo3-numpy 같은 확장 crate도 함께 고려.

---

# cdylib

crate-type = ["cdylib"]을 선언하면 Rust 내부에서 일반적인 방식으로 사용하는 데 제약이 생길 수 있음,  
하지만 완전히 못 쓰는 건 아닙니다. 아래에 자세히 설명.

## 🧠 crate-type = ["cdylib"]란?
- cdylib은 C-compatible dynamic library를 생성하는 타입입니다.
- 목적은 C, Python, Node.js 등 외부 언어에서 Rust 함수를 호출할 수 있게 만드는 것이에요.
- 이 타입은 Rust의 내부 메타데이터, 트레잇, 모듈 시스템 등을 제거해서 C ABI에 맞게 빌드합니다.

## ❌ Rust 내부에서 직접 use하거나 extern crate로 쓰기 어려움
- cdylib은 Rust의 crate로서 재사용하기 위한 메타정보를 포함하지 않기 때문에,  
    다른 Rust 프로젝트에서 `use mylib::some_function` 처럼 직접 가져다 쓰는 건 불가능합니다.
- cargo는 cdylib을 Rust 라이브러리로 인식하지 않고, 외부 바이너리처럼 취급합니다.

## ✅ 해결 방법: 다중 crate-type 선언
Rust에서 내부에서도 쓰고 외부에서도 노출하고 싶다면, crate-type을 이렇게 설정하세요:
```
[lib]
crate-type = ["rlib", "cdylib"]
```

## 🧩 crate-type 요약

| crate-type | 용도 설명 |
|------------|-----------|
| `rlib`     | Rust 내부에서 `use`, `cargo` 의존성으로 사용 가능. 다른 Rust crate에서 재사용할 때 필요 |
| `cdylib`   | 외부 언어(C, Python 등)에서 동적 라이브러리로 사용. C ABI에 맞춰 빌드됨 |

이렇게 하면:
- Rust 프로젝트에서는 use mylib::some_function으로 사용 가능
- Python이나 C에서는 .so 또는 .dll로 로드해서 사용 가능

## 🧱 생성되는 라이브러리 종류
| 타입      | 파일 이름 예시                    | 용도 설명       |
|-----------|-----------------------------------|------------------|
| `rlib`    | `libmylib.rlib`                   | Rust 내부에서 `use`, `cargo` 의존성으로 사용 가능 |
| `cdylib`  | `libmylib.so` / `.dll` / `.dylib` | 외부 언어(C, Python 등)에서 동적 라이브러리로 사용 |

- 이 둘은 동시에 생성되며, 각각의 목적에 맞게 사용됩니다.

## ✅ 실전에서 어떻게 활용하나?
- Rust 내부 모듈이나 다른 crate에서 재사용 → rlib
- Python, C, Node.js 등에서 FFI로 호출 → cdylib
### 예를 들어:
- 드라이버 설정 로직은 Rust 내부에서 use해서 테스트하고,
- Python에서 ctypes나 PyO3로 .so를 불러와서 실제 설정에 적용

## 💬 결론
crate-type = ["cdylib"]만 선언하면 Rust 내부에서는 재사용이 어렵고,  
외부 언어용으로만 빌드되는 라이브러리가 됩니다.
Rust에서도 쓰고 싶다면 "rlib"을 함께 선언하는 게 가장 안전한 방법.

crate-type = ["rlib", "cdylib"]을 쓰면  
Rust 내부용과 외부 연동용 라이브러리를 동시에 생성할 수 있음.  
하나의 코드베이스로 두 환경을 모두 커버할 수 있는 전략적 선택입니다.

---

---
