# Print

## 🖥️ 1. 화면 출력 (콘솔)
Rust는 `println!`, `print!`, `eprintln!`, `format!` 등의 매크로를 통해 다양한 형식으로 출력할 수 있습니다.
### 🔹 기본 출력
```rust
println!("Hello, {}", name);
```

### 🔹 위치/명명된 인자
```rust
println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
println!("{subject} {verb} {object}", subject="Fox", verb="jumps over", object="dog");
```

### 🔹 정렬 및 패딩 (`:>`, `:<`, `:^`, `:0>`)
```rust
println!("{:>5}", 42);     // 오른쪽 정렬: "   42"
println!("{:<5}", 42);     // 왼쪽 정렬: "42   "
println!("{:^5}", 42);     // 가운데 정렬: " 42  "
println!("{:0>5}", 42);    // 0으로 채움: "00042"
```

### 🔹 진법 출력 (`{:b}`, `{:o}`, `{:x}`, `{:X}`)
```rust
println!("{:b}", 42); // 101010
println!("{:o}", 42); // 52
println!("{:x}", 42); // 2a
println!("{:X}", 42); // 2A
```


## 📄 2. 파일 출력
Rust의 std::fs::File과 std::io::Write를 사용하면 파일에 출력할 수 있습니다.
```rust
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.txt")?;
    writeln!(file, "Hello, Rust!")?;
    Ok(())
}
```

- write! / writeln!은 println!과 동일한 포맷을 사용
- BufWriter를 사용하면 성능 향상 가능

## 📊 3. CSV 출력
```rust
Rust에는  크레이트가 있어 CSV 출력이 매우 간단합니다.
use csv::Writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path("data.csv")?;
    wtr.write_record(&["name", "age", "city"])?;
    wtr.write_record(&["Alice", "30", "Seoul"])?;
    wtr.flush()?;
    Ok(())
}
```


- 자동으로 쉼표 구분
- UTF-8 인코딩
- serde와 함께 사용하면 구조체 직렬화도 가능


## 📐 4. 고정 폭 출력 (Fixed Column)
```rust
println!("{:<10} {:>5}", "Name", "Age");
println!("{:<10} {:>5}", "Alice", 30);
println!("{:<10} {:>5}", "Bob", 42);
```

### 출력 결과:
```
Name       Age
Alice       30
Bob         42
```

- :<10 → 왼쪽 정렬, 너비 10
- :>5 → 오른쪽 정렬, 너비 5

## 🔢 5. 소수점 제한 (Float Precision)
```rust
let pi = 3.1415926535;
println!("{:.2}", pi); // 3.14
println!("{:.5}", pi); // 3.14159
```

- `{:.N}` → 소수점 N자리까지 출력
- format!으로 문자열로 저장도 가능

## 🔬 6. 지수 표현 (Scientific Notation)
```rust
let value = 12345.6789;
println!("{:e}", value); // 1.234568e4
println!("{:.2e}", value); // 1.23e4
```

- `{:e}` → 지수 표현
- `{:.Ne}` → 소수점 N자리 지수 표현


## 🧠 출력 방식 요약

| 출력 유형           | 예시 코드                                | 설명                            |
|--------------------|-------------------------------------------|---------------------------------|
| 소수점 제한 출력    | `println!("{:.2}", pi)`                   | 소수점 2자리까지 출력            |
| 파일 출력           | `writeln!(file, "text")`                  | 파일에 텍스트 저장               |
| CSV 출력            | `csv::Writer::write_record(...)`          | 쉼표 구분 데이터 저장            |
| 고정 폭 텍스트 출력 | `println!("{:<10} {:>5}", ...)`           | 열 정렬된 텍스트 출력            |
| 지수 표현 출력      | `println!("{:e}", value)`                 | 과학적 표기법으로 float 출력     |

---

