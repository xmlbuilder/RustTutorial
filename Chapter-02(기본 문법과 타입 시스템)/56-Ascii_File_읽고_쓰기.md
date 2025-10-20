# ASCII 읽고 쓰기

ust에서는 std::fs::File과 std::io::{Read, Write} 트레이트를 사용해 ASCII 텍스트 파일을 쉽게 읽고 쓸 수 있습니다.  
아래에 기본 샘플과 실전 예제를 함께 소개.

## 🧾 기본 개념 요약
- 읽기: `File::open() + read_to_string()`
- 쓰기: `File::create() + write_all()`
- 모두 Result 타입을 반환하며 ? 연산자로 에러 전파 가능

## 🧪 샘플 예제: ASCII 파일 읽고 쓰기
```rust
use std::fs::File;
use std::io::{self, Read, Write};

fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_string_to_file(filename: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() {
    let input_filename = "input.txt";
    let output_filename = "output.txt";

    match read_file_to_string(input_filename) {
        Ok(contents) => {
            println!("읽은 내용:\n{}", contents);
            match write_string_to_file(output_filename, &contents) {
                Ok(_) => println!("output.txt에 성공적으로 저장됨"),
                Err(e) => println!("쓰기 오류: {}", e),
            }
        }
        Err(e) => println!("읽기 오류: {}", e),
    }
}
```

## 🧑‍💻 실전 예제: ASCII 로그 파일 필터링 후 저장
```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let input = File::open("server.log")?;
    let reader = BufReader::new(input);
    let mut output = File::create("errors.log")?;

    for line in reader.lines() {
        let line = line?;
        if line.contains("ERROR") {
            writeln!(output, "{}", line)?;
        }
    }

    println!("ERROR 로그만 errors.log에 저장됨");
    Ok(())
}
```

- BufReader를 사용해 줄 단위로 읽기
- "ERROR"가 포함된 줄만 필터링해서 새 파일에 저장

## 📌 팁
- 파일이 없을 경우 `File::open()` 은 에러를 반환하므로 match 또는 `?` 로 처리해야 함
- 쓰기 시 기존 파일은 덮어쓰기 (`File::create()` 는 truncate 동작 포함)
- 유니코드가 아닌 ASCII만 처리할 경우 String 기반으로 충분


---
# 이어 쓰기

## ✅ 이어쓰기 방법: OpenOptions
```rust
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)         // 이어쓰기 모드
        .create(true)         // 파일이 없으면 새로 생성
        .open("log.txt")?;    // 대상 파일

    writeln!(file, "새 로그 줄입니다")?;
    Ok(())
}
```


## 🔧 주요 옵션 설명: OpenOptions

| 옵션              | 설명                                                  |
|-------------------|-------------------------------------------------------|
| `.append(true)`   | 기존 파일 끝에 내용을 이어서 씀 (덮어쓰기 아님)        |
| `.create(true)`   | 파일이 없으면 새로 생성함                             |
| `.write(true)`    | 파일에 쓰기 권한을 부여함                             |
| `.truncate(true)` | 파일을 열 때 기존 내용을 모두 지움 (`File::create()`와 동일) |



## 📁 예시: 로그 파일에 계속 추가하기
```rust
fn log_message(msg: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("app.log")?;

    writeln!(file, "[INFO] {}", msg)?;
    Ok(())
}
```
- 이렇게 하면 app.log에 로그 메시지가 계속 누적됩니다.

---
