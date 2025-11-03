# std::fs::File / std::io::Write

## 에러 소스
```rust
let mut f = File::create(filename)?; 
let is_rat = self.ctrl.iter().any(|c| c.w.is_finite())
writeln!(f, "pu {}", self.pu)?;
```

## 에러 코드
```
error[E0599]: cannot write into File
```
- writeln!(f, ...)이 작동하지 않는 이유는 File 타입에 대해 write_fmt 메서드가 직접 호출 불가능한 상태라는 뜻입니다.

## 🔍 원인 분석
- Rust에서 `writeln!` 매크로는 내부적으로 `write_fmt()` 를 호출합니다.
- 이 메서드는 std::io::Write 트레이트에 정의되어 있고, File 타입은 이 트레이트를 구현하고 있습니다.
- 그런데 이 에러가 발생하는 이유는 `use std::io::Write;` 를 하지 않았기 때문입니다.
- Rust는 트레이트 메서드를 사용하려면 해당 트레이트를 스코프에 명시적으로 가져와야 합니다.
- File은 write_fmt()를 구현하고 있지만, Write 트레이트가 현재 스코프에 없으면 컴파일러는 해당 메서드를 찾지 못합니다.

## ✅ 해결 방법
```rust
use std::fs::File;
use std::io::{self, Write}; // 이 줄이 필요합니다!

fn save(filename: &str, pu: f64) -> io::Result<()> {
    let mut f = File::create(filename)?;
    writeln!(f, "pu {}", pu)?;
    Ok(())
}
```

## 🧠 요약
-` writeln!` 은 Write 트레이트의 `write_fmt()`를 사용함
- `File` 은 `Write` 를 구현하지만, 트레이트를 스코프에 가져오지 않으면 메서드 호출 불가
- 해결책: `use std::io::Write;` 를 추가하면 정상 작동

---

## 🧠 왜 IDE가 못 잡을까?
Rust에서는 어떤 타입이 특정 메서드를 사용할 수 있으려면, 그 메서드를 정의한 트레이트가 스코프에 있어야 합니다. 예를 들어:
- writeln!(f, ...) → 내부적으로 f.write_fmt(...) 호출
- write_fmt()는 std::io::Write 트레이트에 정의됨
- File은 Write를 구현하지만…
- `use std::io::Write;` 를 하지 않으면 `IDE도 컴파일러` 도 `해당 메서드를 인식하지 못함`
- 즉, 트레이트 메서드는 트레이트를 스코프에 명시적으로 가져와야만 IDE가 자동 완성이나 타입 추론을 제대로 해줄 수 있음.

## ✅ 팁: Rust에서 자주 쓰는 트레이트는 직접 use 해주자
```rust
use std::io::Write; // writeln!(), write!(), write_fmt() 등 사용 가능
```
이걸 해주면:
- IDE가 writeln!() 사용 가능하다고 인식
- 컴파일러도 write_fmt()를 찾을 수 있음
- 오류 E0599 해결

## 🛠️ 참고로…
이런 트레이트 기반 메서드 호출은 Rust의 확장성과 안전성을 위한 설계입니다.  
처음엔 불편하게 느껴질 수 있지만, 익숙해지면 명확한 의존성 관리와 정확한 타입 추론이 가능.  

---
