# mod
## 🧭 Rust의 mod는 무엇인가요?
- mod는 **모듈(module)** 을 정의하는 키워드입니다.
- 모듈은 코드를 논리적으로 그룹화하고, **이름 공간(namespace)** 을 제공합니다.
- pub을 붙이면 해당 모듈이 **외부에서 접근 가능(public)** 하도록 공개됩니다.

## 🔍 mod vs. namespace 비교
| 항목             | Rust `mod`           | C++/Java/C# `namespace`/`package` |
|------------------|----------------------|-----------------------------------|
| 접근 제어        | `pub`                | `public`, `private`               |
| 선언 키워드      | `mod`                | `namespace`, `package`            |
| 이름 공간 제공   | ✅ 가능               | ✅ 가능                            |
| 외부 접근 방식   | `crate::mod_name`    | `namespace.name` 또는 `import`    |
| 파일 분리 가능   | ✅ 가능 (`mod.rs`)    | ✅ 가능 (`.h`, `.java`, `.cs`)     |
| 구조적 계층화    | 모듈 트리 지원       | 패키지/네임스페이스 계층 지원     |


## ✨ 예시

```rust
// main.rs
pub mod tri_tri {
    pub fn hello() {
        println!("Hello from tri_tri!");
    }
}

fn main() {
    tri_tri::hello(); // tri_tri 모듈의 함수 호출
}
```

이처럼 tri_tri는 이름 공간을 제공하는 모듈로서, 내부의 함수나 구조체 등을 외부에서 사용할 수 있게 해줍니다.

---




