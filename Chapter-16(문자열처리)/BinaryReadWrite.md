# Binary Read / Write
Rust에서 바이너리 파일을 읽고 쓰는 방법은 매우 강력하면서도 안전하게 설계되어 있음.  
아래에 개념 설명부터 샘플 코드, 그리고 실전 예제까지 단계별로 정리.

## 항목 정리
| 작업 항목           | 설명 및 관련 함수들                                      |
|--------------------|----------------------------------------------------------|
| 파일 쓰기          | `std::fs::File` + `write_all()`                          |
| 파일 읽기          | `std::fs::File` + `read_exact()` / `read_to_end()`       |
| 숫자 → 바이트 변환 | `u8`, `i32`, `f64` 등 → `.to_le_bytes()` / `.to_be_bytes()` |
| 바이트 → 숫자 복원 | `.from_le_bytes()` / `.from_be_bytes()`                 |


## 🧪 샘플 1: 숫자 배열을 바이너리로 저장하고 다시 읽기
```rust
use std::fs::File;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {
    let numbers: [i32; 4] = [10, 20, 30, 40];

    // 바이너리로 저장
    let mut file = File::create("data.bin")?;
    for &num in &numbers {
        file.write_all(&num.to_le_bytes())?;
    }

    // 바이너리로 읽기
    let mut file = File::open("data.bin")?;
    let mut buffer = [0u8; 4];
    let mut result = Vec::new();

    for _ in 0..4 {
        file.read_exact(&mut buffer)?;
        let value = i32::from_le_bytes(buffer);
        result.push(value);
    }

    println!("읽은 값: {:?}", result);
    Ok(())
}
```


## 🧪 샘플 2: 구조체를 바이너리로 저장하고 읽기
```rust
use std::fs::File;
use std::io::{Write, Read};

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() -> std::io::Result<()> {
    let point = Point { x: 1.5, y: -2.75 };

    // 저장
    let mut file = File::create("point.bin")?;
    file.write_all(&point.x.to_le_bytes())?;
    file.write_all(&point.y.to_le_bytes())?;

    // 읽기
    let mut file = File::open("point.bin")?;
    let mut buf = [0u8; 4];

    file.read_exact(&mut buf)?;
    let x = f32::from_le_bytes(buf);

    file.read_exact(&mut buf)?;
    let y = f32::from_le_bytes(buf);

    let loaded = Point { x, y };
    println!("읽은 구조체: {:?}", loaded);
    Ok(())
}
```


## 🧪 실전 예제: 사용자 데이터 저장 및 로딩
```rust
use std::fs::File;
use std::io::{Write, Read};

#[derive(Debug)]
struct User {
    id: u32,
    score: f64,
}

fn save_user(user: &User, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&user.id.to_le_bytes())?;
    file.write_all(&user.score.to_le_bytes())?;
    Ok(())
}

fn load_user(path: &str) -> std::io::Result<User> {
    let mut file = File::open(path)?;
    let mut buf_u32 = [0u8; 4];
    let mut buf_f64 = [0u8; 8];

    file.read_exact(&mut buf_u32)?;
    file.read_exact(&mut buf_f64)?;

    Ok(User {
        id: u32::from_le_bytes(buf_u32),
        score: f64::from_le_bytes(buf_f64),
    })
}

fn main() -> std::io::Result<()> {
    let user = User { id: 42, score: 98.6 };
    save_user(&user, "user.bin")?;

    let loaded = load_user("user.bin")?;
    println!("불러온 사용자: {:?}", loaded);
    Ok(())
}
```


## ✅ 팁
- to_le_bytes() → 리틀 엔디안 저장 (Windows, x86 기본)
- to_be_bytes() → 빅 엔디안 저장 (네트워크 전송 시 사용)
- bincode, serde 라이브러리를 쓰면 더 복잡한 구조도 자동 직렬화 가능

---

# 구조체 저장 및 읽기
그럴 경우에는 데이터의 개수도 함께 저장해야 정확하게 읽을 수 있음.
아래에 구조체 배열을 저장하고 다시 읽는 전체 샘플 제공.

## 🧠 목표
- User 구조체 배열을 바이너리로 저장
- 저장 시: 먼저 개수(u32) → 각 구조체의 필드(u32, f64)
- 읽을 때: 개수 읽고 → 반복해서 구조체 읽기

### 🧪 Rust 실전 샘플: 구조체 배열을 바이너리로 저장/읽기
```rust
use std::fs::File;
use std::io::{Write, Read};

#[derive(Debug)]
struct User {
    id: u32,
    score: f64,
}

fn save_users(users: &[User], path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // 1. 저장할 개수 먼저 기록
    let count = users.len() as u32;
    file.write_all(&count.to_le_bytes())?;

    // 2. 각 구조체의 필드 기록
    for user in users {
        file.write_all(&user.id.to_le_bytes())?;
        file.write_all(&user.score.to_le_bytes())?;
    }

    Ok(())
}

fn load_users(path: &str) -> std::io::Result<Vec<User>> {
    let mut file = File::open(path)?;

    // 1. 개수 읽기
    let mut buf_u32 = [0u8; 4];
    file.read_exact(&mut buf_u32)?;
    let count = u32::from_le_bytes(buf_u32);

    // 2. 구조체 반복해서 읽기
    let mut users = Vec::new();
    for _ in 0..count {
        file.read_exact(&mut buf_u32)?;
        let id = u32::from_le_bytes(buf_u32);

        let mut buf_f64 = [0u8; 8];
        file.read_exact(&mut buf_f64)?;
        let score = f64::from_le_bytes(buf_f64);

        users.push(User { id, score });
    }

    Ok(users)
}

fn main() -> std::io::Result<()> {
    let users = vec![
        User { id: 1, score: 95.5 },
        User { id: 2, score: 88.0 },
        User { id: 3, score: 76.25 },
    ];

    save_users(&users, "users.bin")?;
    let loaded = load_users("users.bin")?;

    println!("불러온 사용자 목록:");
    for user in loaded {
        println!("{:?}", user);
    }

    Ok(())
}
```

### ✅ 결과 예시
```
불러온 사용자 목록:
User { id: 1, score: 95.5 }
User { id: 2, score: 88.0 }
User { id: 3, score: 76.25 }
```


## 💡 확장 아이디어
- 구조체에 name: String 같은 필드 추가 시 → 길이 + 바이트로 저장해야 함
- Vec<T>를 저장할 때는 항상 개수 먼저 기록하는 게 안전
- 이 방식은 직렬화 없이도 완전한 제어가 가능해서 실무에서 유용함

--- 

# String 쓰기

String을 바이너리로 저장할 때는 반드시 길이(갯수)를 먼저 써야 합니다.
Rust의 String은 내부적으로 UTF-8 바이트 배열이기 때문에,
읽을 때 정확히 몇 바이트를 읽어야 할지 알 수 없으면 데이터가 깨지거나 잘못 해석될 수 있음.

## 🧠 왜 길이를 먼저 써야 할까?
|  |  | 
|  | String | 
|  | char | 
|  | read_exact() | 



## 🧪 실전 샘플: 구조체에 String 포함해서 저장/읽기
```rust
use std::fs::File;
use std::io::{Write, Read};

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    score: f64,
}

fn save_users(users: &[User], path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    // 개수 저장
    let count = users.len() as u32;
    file.write_all(&count.to_le_bytes())?;

    for user in users {
        // id
        file.write_all(&user.id.to_le_bytes())?;

        // name 길이 + 내용
        let name_bytes = user.name.as_bytes();
        let name_len = name_bytes.len() as u32;
        file.write_all(&name_len.to_le_bytes())?;
        file.write_all(name_bytes)?;

        // score
        file.write_all(&user.score.to_le_bytes())?;
    }

    Ok(())
}

fn load_users(path: &str) -> std::io::Result<Vec<User>> {
    let mut file = File::open(path)?;
    let mut buf_u32 = [0u8; 4];
    let mut buf_f64 = [0u8; 8];

    // 개수 읽기
    file.read_exact(&mut buf_u32)?;
    let count = u32::from_le_bytes(buf_u32);

    let mut users = Vec::new();
    for _ in 0..count {
        // id
        file.read_exact(&mut buf_u32)?;
        let id = u32::from_le_bytes(buf_u32);

        // name 길이 + 내용
        file.read_exact(&mut buf_u32)?;
        let name_len = u32::from_le_bytes(buf_u32) as usize;
        let mut name_buf = vec![0u8; name_len];
        file.read_exact(&mut name_buf)?;
        let name = String::from_utf8(name_buf).unwrap_or_default();

        // score
        file.read_exact(&mut buf_f64)?;
        let score = f64::from_le_bytes(buf_f64);

        users.push(User { id, name, score });
    }

    Ok(users)
}

fn main() -> std::io::Result<()> {
    let users = vec![
        User { id: 1, name: "JungHwan".to_string(), score: 95.5 },
        User { id: 2, name: "Rustacean".to_string(), score: 88.0 },
    ];

    save_users(&users, "users_with_name.bin")?;
    let loaded = load_users("users_with_name.bin")?;

    println!("불러온 사용자 목록:");
    for user in loaded {
        println!("{:?}", user);
    }

    Ok(())
}
```
### ✅ 결과 예시
```
불러온 사용자 목록:
User { id: 1, name: "JungHwan", score: 95.5 }
User { id: 2, name: "Rustacean", score: 88.0 }
```

---

# 복잡한 구조체에 적용


이번엔 Rust에서 직렬화 없이 직접 바이너리로 저장/복원하는 복잡한 구조체를 다뤄봄.

## 다룰 요소는 다음과 같습니다:
- ✅ Vec<String>: 문자열 리스트 → 각 문자열 길이 + 내용
- ✅ Option<String>: 존재 여부 + 길이 + 내용
- ✅ enum: 태그값 + 해당 variant의 데이터

## 🧠 구조체 정의
```rust
#[derive(Debug)]
enum Role {
    Admin,
    Guest,
    Custom(String),
}

#[derive(Debug)]
struct User {
    id: u32,
    name: Option<String>,
    tags: Vec<String>,
    role: Role,
}
```

## 📦 저장 로직
```rust
use std::fs::File;
use std::io::{Write, Read};
fn write_string(file: &mut File, s: &str) -> std::io::Result<()> {
    let bytes = s.as_bytes();
    let len = bytes.len() as u32;
    file.write_all(&len.to_le_bytes())?;
    file.write_all(bytes)?;
    Ok(())
}

fn write_option_string(file: &mut File, opt: &Option<String>) -> std::io::Result<()> {
    match opt {
        Some(s) => {
            file.write_all(&[1])?;
            write_string(file, s)?;
        }
        None => {
            file.write_all(&[0])?;
        }
    }
    Ok(())
}

fn write_vec_string(file: &mut File, vec: &Vec<String>) -> std::io::Result<()> {
    let count = vec.len() as u32;
    file.write_all(&count.to_le_bytes())?;
    for s in vec {
        write_string(file, s)?;
    }
    Ok(())
}

fn write_role(file: &mut File, role: &Role) -> std::io::Result<()> {
    match role {
        Role::Admin => file.write_all(&[0])?,
        Role::Guest => file.write_all(&[1])?,
        Role::Custom(s) => {
            file.write_all(&[2])?;
            write_string(file, s)?;
        }
    }
    Ok(())
}

fn save_user(user: &User, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&user.id.to_le_bytes())?;
    write_option_string(&mut file, &user.name)?;
    write_vec_string(&mut file, &user.tags)?;
    write_role(&mut file, &user.role)?;
    Ok(())
}
```


## 📦 읽기 로직
```rust
fn read_u32(file: &mut File) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_string(file: &mut File) -> std::io::Result<String> {
    let len = read_u32(file)? as usize;
    let mut buf = vec![0u8; len];
    file.read_exact(&mut buf)?;
    Ok(String::from_utf8(buf).unwrap_or_default())
}

fn read_option_string(file: &mut File) -> std::io::Result<Option<String>> {
    let mut flag = [0u8; 1];
    file.read_exact(&mut flag)?;
    if flag[0] == 1 {
        Ok(Some(read_string(file)?))
    } else {
        Ok(None)
    }
}

fn read_vec_string(file: &mut File) -> std::io::Result<Vec<String>> {
    let count = read_u32(file)? as usize;
    let mut vec = Vec::new();
    for _ in 0..count {
        vec.push(read_string(file)?);
    }
    Ok(vec)
}

fn read_role(file: &mut File) -> std::io::Result<Role> {
    let mut tag = [0u8; 1];
    file.read_exact(&mut tag)?;
    match tag[0] {
        0 => Ok(Role::Admin),
        1 => Ok(Role::Guest),
        2 => Ok(Role::Custom(read_string(file)?)),
        _ => Ok(Role::Guest), // fallback
    }
}

fn load_user(path: &str) -> std::io::Result<User> {
    let mut file = File::open(path)?;
    let id = read_u32(&mut file)?;
    let name = read_option_string(&mut file)?;
    let tags = read_vec_string(&mut file)?;
    let role = read_role(&mut file)?;
    Ok(User { id, name, tags, role })
}
```

## 🧪 테스트 코드
```rust
fn main() -> std::io::Result<()> {
    let user = User {
        id: 101,
        name: Some("JungHwan".to_string()),
        tags: vec!["rustacean".to_string(), "developer".to_string()],
        role: Role::Custom("PowerUser".to_string()),
    };

    save_user(&user, "complex_user.bin")?;
    let loaded = load_user("complex_user.bin")?;

    println!("불러온 사용자: {:?}", loaded);
    Ok(())
}
```


### ✅ 결과 예시
```
불러온 사용자: User {
    id: 101,
    name: Some("JungHwan"),
    tags: ["rustacean", "developer"],
    role: Custom("PowerUser")
}
```

---

# Data Block Read / Write

이번엔 데이터를 블록 단위로 저장하고, 각 블록의 헤더만 읽은 뒤 크기를 기반으로 다음 블록으로 건너뛰는 방식을 Rust로 구현.

## 🧠 시나리오 개요
- 여러 개의 구조체를 각각 블록 단위로 저장
- 각 블록은 다음과 같은 구조:
[블록 크기 (u32)] + [블록 데이터 (구조체)]
- 파일 전체는 여러 블록이 연속적으로 저장됨
- 읽을 때는:
- 블록 크기만 읽고
- 해당 크기만큼 seek으로 건너뛰기

## 📦 구조체 정의
```rust
#[derive(Debug)]
struct LogEntry {
    timestamp: u64,
    level: u8,
    message: String,
}
```


## 🧪 저장 로직: 블록 단위로 저장
```rust
use std::fs::File;
use std::io::{Write, Seek, SeekFrom};

fn write_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len() as u32;
    buf.extend(&len.to_le_bytes());
    buf.extend(bytes);
}

fn serialize_entry(entry: &LogEntry) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend(&entry.timestamp.to_le_bytes());
    buf.push(entry.level);
    write_string(&mut buf, &entry.message);
    buf
}

fn save_blocks(entries: &[LogEntry], path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for entry in entries {
        let data = serialize_entry(entry);
        let block_size = data.len() as u32;
        file.write_all(&block_size.to_le_bytes())?;
        file.write_all(&data)?;
    }
    Ok(())
}
```


## 🧪 읽기 로직: 블록 헤더만 읽고 건너뛰기
```rust
use std::io::{Read};

fn read_u32(file: &mut File) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_block_headers(path: &str) -> std::io::Result<Vec<u32>> {
    let mut file = File::open(path)?;
    let mut headers = Vec::new();

    loop {
        let pos = file.stream_position()?; // 현재 위치
        match read_u32(&mut file) {
            Ok(size) => {
                headers.push(pos); // 블록 시작 위치 저장
                file.seek(SeekFrom::Current(size as i64))?; // 블록 건너뛰기
            }
            Err(_) => break, // EOF
        }
    }

    Ok(headers)
}
```


## 🧪 테스트 코드
```rust
fn main() -> std::io::Result<()> {
    let entries = vec![
        LogEntry { timestamp: 1697100000, level: 1, message: "System started".to_string() },
        LogEntry { timestamp: 1697100050, level: 2, message: "Warning: high memory".to_string() },
        LogEntry { timestamp: 1697100100, level: 3, message: "Error: disk full".to_string() },
    ];

    save_blocks(&entries, "log_blocks.bin")?;
    let headers = read_block_headers("log_blocks.bin")?;

    println!("블록 시작 위치 목록:");
    for (i, pos) in headers.iter().enumerate() {
        println!("Block {} starts at byte offset {}", i, pos);
    }

    Ok(())
}
```


### ✅ 결과 예시
```
블록 시작 위치 목록:
Block 0 starts at byte offset 0
Block 1 starts at byte offset 41
Block 2 starts at byte offset 86

```

## 💡 확장 아이디어
- 블록에 type, version, checksum 같은 메타데이터 추가 가능
- 특정 블록만 읽고 싶을 때 seek으로 바로 이동 가능
- 블록 크기를 기준으로 병렬 처리, 압축, 인덱싱도 가능


----

# Block Index file

이번엔 블록 인덱스 파일을 별도로 생성해서, 특정 블록을 빠르게 탐색할 수 있는 구조를 구현.
이 구조는 대용량 로그, 데이터베이스, 스트리밍 파일 등에서 매우 유용하게 쓰입니다.

## 🧠 구조 개요
### 🔹 데이터 파일 (data.bin)
- 여러 개의 블록이 연속 저장됨
- 각 블록 구조:
```
[블록 크기 (u32)] + [블록 데이터 (가변)]
```

### 🔹 인덱스 파일 (index.bin)
- 각 블록의 시작 위치(offset)와 메타데이터 저장
- 구조 예시:
```
[블록 수 (u32)]
[offset_0 (u64)] [timestamp_0 (u64)]
[offset_1 (u64)] [timestamp_1 (u64)]
...

```

### 📦 구조체 정의
```rust
#[derive(Debug)]
struct Block {
    timestamp: u64,
    message: String,
}
```


## 🛠 저장: 데이터 + 인덱스 파일 생성
```rust
use std::fs::File;
use std::io::{Write, Seek, SeekFrom};

fn write_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len() as u32;
    buf.extend(&len.to_le_bytes());
    buf.extend(bytes);
}

fn save_blocks_with_index(blocks: &[Block], data_path: &str, index_path: &str) -> std::io::Result<()> {
    let mut data_file = File::create(data_path)?;
    let mut index_file = File::create(index_path)?;

    // 인덱스: 블록 수 먼저 기록
    let count = blocks.len() as u32;
    index_file.write_all(&count.to_le_bytes())?;

    for block in blocks {
        let offset = data_file.stream_position()?; // 현재 위치 저장

        // 블록 직렬화
        let mut buf = Vec::new();
        buf.extend(&block.timestamp.to_le_bytes());
        write_string(&mut buf, &block.message);

        // 블록 크기 + 데이터 저장
        let size = buf.len() as u32;
        data_file.write_all(&size.to_le_bytes())?;
        data_file.write_all(&buf)?;

        // 인덱스에 offset + timestamp 저장
        index_file.write_all(&offset.to_le_bytes())?;
        index_file.write_all(&block.timestamp.to_le_bytes())?;
    }

    Ok(())
}


### 🔍 탐색: 인덱스 파일만 읽어서 특정 블록 위치 찾기

```rust
use std::io::{Read};

fn read_u32(file: &mut File) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64(file: &mut File) -> std::io::Result<u64> {
    let mut buf = [0u8; 8];
    file.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

fn load_index(index_path: &str) -> std::io::Result<Vec<(u64, u64)>> {
    let mut file = File::open(index_path)?;
    let count = read_u32(&mut file)?;
    let mut index = Vec::new();

    for _ in 0..count {
        let offset = read_u64(&mut file)?;
        let timestamp = read_u64(&mut file)?;
        index.push((offset, timestamp));
    }

    Ok(index)
}
```

### 🔍 특정 블록만 빠르게 로딩
```rust
fn read_block_at(data_path: &str, offset: u64) -> std::io::Result<Block> {
    let mut file = File::open(data_path)?;
    file.seek(SeekFrom::Start(offset))?;

    let size = read_u32(&mut file)?;
    let mut buf = vec![0u8; size as usize];
    file.read_exact(&mut buf)?;

    let mut cursor = &buf[..];
    let mut ts_buf = [0u8; 8];
    cursor.read_exact(&mut ts_buf)?;
    let timestamp = u64::from_le_bytes(ts_buf);

    let mut len_buf = [0u8; 4];
    cursor.read_exact(&mut len_buf)?;
    let msg_len = u32::from_le_bytes(len_buf) as usize;

    let mut msg_buf = vec![0u8; msg_len];
    cursor.read_exact(&mut msg_buf)?;
    let message = String::from_utf8(msg_buf).unwrap_or_default();

    Ok(Block { timestamp, message })
}
```


### 🧪 테스트 코드
```rust
fn main() -> std::io::Result<()> {
    let blocks = vec![
        Block { timestamp: 1001, message: "Start".into() },
        Block { timestamp: 1002, message: "Processing".into() },
        Block { timestamp: 1003, message: "Done".into() },
    ];

    save_blocks_with_index(&blocks, "data.bin", "index.bin")?;

    let index = load_index("index.bin")?;
    println!("인덱스 목록:");
    for (i, (offset, ts)) in index.iter().enumerate() {
        println!("Block {}: offset={}, timestamp={}", i, offset, ts);
    }

    // 예: 두 번째 블록만 로딩
    let (offset, _) = index[1];
    let block = read_block_at("data.bin", offset)?;
    println!("선택된 블록: {:?}", block);

    Ok(())
}
```


### ✅ 출력 예시
```
인덱스 목록:
Block 0: offset=0, timestamp=1001
Block 1: offset=21, timestamp=1002
Block 2: offset=47, timestamp=1003
선택된 블록: Block { timestamp: 1002, message: "Processing" }
```
---
