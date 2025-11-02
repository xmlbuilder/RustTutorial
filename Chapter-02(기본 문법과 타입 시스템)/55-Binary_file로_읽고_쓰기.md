# Binary

## 🖨️ 바이너리 출력 (쓰기)

Rust에서는 `std::fs::File` 과 `std::io::Write` 를 사용해 바이너리 데이터를 파일에 저장할 수 있습니다.

```rust
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("data.bin")?;
    let buffer: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    file.write_all(&buffer)?;
    Ok(())
}
```

## ✅ 설명
- `File::create()` 로 파일 생성
- `write_all()` 은 바이트 슬라이스를 그대로 저장
- `u8` 배열이나 `Vec<u8>` 모두 사용 가능

## 📥 바이너리 읽기

### 🔹 전체 파일 읽기
```rust
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("data.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    println!("Read {} bytes: {:?}", buffer.len(), buffer);
    Ok(())
}
```

### 🔹 버퍼로 부분 읽기 (대용량 파일 대응)
```rust
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> std::io::Result<()> {
    let file = File::open("data.bin")?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 { break; }
        println!("Read {} bytes", bytes_read);
        // buffer[0..bytes_read] 처리
    }
    Ok(())
}
```


## 🧠 고급 팁
| 기능               | 설명 또는 예시                                 |
|--------------------|------------------------------------------------|
| `write_all()`      | 바이트 슬라이스를 파일에 그대로 저장            |
| `read_to_end()`    | 전체 파일을 `Vec<u8>`로 읽어들임                |
| `BufReader`        | 대용량 파일을 효율적으로 읽기 위한 버퍼링       |
| `byteorder`        | `read_u32::<LittleEndian>()` 등 엔디안 변환 지원 |
| `binread`, `binrw` | 바이너리 구조체를 자동으로 파싱하는 고급 라이브러리 |


## 📦 바이너리 구조체 저장 예시
```rust
#[repr(C)]
#[derive(Debug)]
struct Header {
    magic: u32,
    version: u16,
    flags: u16,
}

use std::mem;

fn write_struct(file: &mut File, header: &Header) -> std::io::Result<()> {
    let bytes = unsafe {
        std::slice::from_raw_parts(
            (header as *const Header) as *const u8,
            mem::size_of::<Header>(),
        )
    };
    file.write_all(bytes)
}
```
이 방식은 unsafe를 사용하므로 구조체는 반드시 `#[repr(C)]` 로 메모리 정렬을 보장해야 합니다.

---

# float 값 읽고 쓰기
Rust에서 int, float 값을 순서대로 바이너리 파일에 쓰고, 다시 순서대로 읽는 샘플 코드.   
이 예제는 byteorder 크레이트를 사용해서 엔디안 제어까지 포함한 안전한 방식으로 구현되어 있음.

## 📦 1. Cargo.toml 설정
```
[dependencies]
byteorder = "1.4"
```


## 📝 2. 전체 코드: 쓰기 + 읽기
```rust
use std::fs::File;
use std::io::{Write, Read, BufReader};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

fn main() -> std::io::Result<()> {
    // 1️⃣ 바이너리 파일에 쓰기
    let mut file = File::create("data.bin")?;
    file.write_i32::<LittleEndian>(42)?;         // int
    file.write_f64::<LittleEndian>(3.14159)?;    // float
    file.write_u8(255)?;                         // byte
    file.write_f32::<LittleEndian>(2.718)?;      // float32

    // 2️⃣ 바이너리 파일에서 읽기
    let file = File::open("data.bin")?;
    let mut reader = BufReader::new(file);

    let int_val = reader.read_i32::<LittleEndian>()?;
    let float64_val = reader.read_f64::<LittleEndian>()?;
    let byte_val = reader.read_u8()?;
    let float32_val = reader.read_f32::<LittleEndian>()?;

    println!("Read values:");
    println!("int: {}", int_val);
    println!("float64: {}", float64_val);
    println!("byte: {}", byte_val);
    println!("float32: {}", float32_val);

    Ok(())
}
```

## 🧠 바이너리 출력 메서드 요약

| 타입   | 쓰기 메서드                          | 설명                         |
|--------|--------------------------------------|------------------------------|
| `i32`  | `write_i32::<LittleEndian>()`        | 4바이트 정수, 리틀 엔디안     |
| `f64`  | `write_f64::<LittleEndian>()`        | 8바이트 부동소수점, 리틀 엔디안 |
| `u8`   | `write_u8()`                         | 1바이트 정수                  |
| `f32`  | `write_f32::<LittleEndian>()`        | 4바이트 부동소수점, 리틀 엔디안 |

- LittleEndian은 x86 시스템에서 일반적인 바이트 순서입니다.
- BufReader는 읽기 성능을 높여줍니다.
- byteorder 크레이트는 엔디안 변환을 자동으로 처리해줍니다.

## 🧠 확장 아이디어
- `#[repr(C)]` 구조체를 만들어서 여러 필드를 한 번에 저장
- CAD 시스템에서 좌표, 행렬, 커브 데이터를 바이너리로 저장할 때 활용 가능

---

# 배열 읽고 쓰기
Rust에서 배열을 바이너리로 저장하고 읽는 방법도 아주 강력하게 지원됩니다.  
아래는 i32, f64 등의 배열을 순서대로 파일에 쓰고, 다시 정확히 읽어오는 샘플 코드와 함께 핵심 개념을 정리.

## 📝 예제: i32 배열을 바이너리로 저장하고 읽기
```rust
use std::fs::File;
use std::io::{Write, Read, BufReader};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

fn main() -> std::io::Result<()> {
    let data: [i32; 4] = [10, 20, 30, 40];

    // 1️⃣ 쓰기
    let mut file = File::create("array.bin")?;
    for &value in &data {
        file.write_i32::<LittleEndian>(value)?;
    }

    // 2️⃣ 읽기
    let file = File::open("array.bin")?;
    let mut reader = BufReader::new(file);
    let mut result = [0i32; 4];
    for i in 0..4 {
        result[i] = reader.read_i32::<LittleEndian>()?;
    }

    println!("Read array: {:?}", result);
    Ok(())
}
```

## 🔍 다른 타입 배열도 가능
| 타입   | 쓰기 메서드                          | 읽기 메서드                          |
|--------|--------------------------------------|--------------------------------------|
| `i32`  | `write_i32::<LittleEndian>()`        | `read_i32::<LittleEndian>()`         |
| `f64`  | `write_f64::<LittleEndian>()`        | `read_f64::<LittleEndian>()`         |
| `u8`   | `write_all(&[u8])`                   | `read_exact(&mut [u8])`              |
| `f32`  | `write_f32::<LittleEndian>()`        | `read_f32::<LittleEndian>()`         |


## 🧠 고급 팁
- 배열을 한 번에 저장하려면 unsafe로 메모리 슬라이스를 만들 수도 있지만, 안전하게 하려면 반복문이 좋습니다.
- byteorder 크레이트는 엔디안 문제를 자동으로 해결해줍니다.
- read_exact()은 바이트 배열을 정확히 채워야 할 때 유용합니다.

## 📦 구조체 + 배열 예시
```rust
#[repr(C)]
struct Matrix4x4 {
    data: [[f64; 4]; 4],
}
```

이런 구조체도 unsafe를 통해 메모리 슬라이스로 변환해 저장할 수 있음.

---

# 이중 배열 

아래는 Rust에서 이중 배열 ([[f64; 4]; 4])을 바이너리 파일에 저장하고 다시 읽는 샘플 코드입니다. 
Matrix4x4 같은 구조를 다룰 때 매우 유용하며, byteorder 크레이트를 활용해 엔디안 제어까지 포함.

## 🧩 1. Cargo.toml 설정
```
[dependencies]
byteorder = "1.4"
```

## 📝 2. 전체 코드: 쓰기 + 읽기
```rust
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

type Matrix4x4 = [[f64; 4]; 4];

fn main() -> std::io::Result<()> {
    let matrix: Matrix4x4 = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    // 1️⃣ 쓰기
    let file = File::create("matrix.bin")?;
    let mut writer = BufWriter::new(file);
    for row in &matrix {
        for &val in row {
            writer.write_f64::<LittleEndian>(val)?;
        }
    }

    // 2️⃣ 읽기
    let file = File::open("matrix.bin")?;
    let mut reader = BufReader::new(file);
    let mut loaded: Matrix4x4 = [[0.0; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            loaded[row][col] = reader.read_f64::<LittleEndian>()?;
        }
    }
    println!("Loaded matrix:");
    for row in &loaded {
        println!("{:?}", row);
    }

    Ok(())
}
```

## 🔍 설명
| 항목                     | 설명 또는 예시                                 |
|--------------------------|-----------------------------------------------|
| `BufWriter` + `write_f64::<LittleEndian>()` | 이중 배열의 각 `f64` 값을 리틀 엔디안으로 파일에 저장 |
| `BufReader` + `read_f64::<LittleEndian>()`  | 저장된 `f64` 값을 순서대로 읽어와 이중 배열에 복원     |
| `Matrix4x4 = [[f64; 4]; 4]`                 | 4x4 이중 배열 타입 정의, 행렬 표현에 적합               |


## 🧠 확장 아이디어
- Matrix3x3, Transform, Point3D 등 다양한 구조체에 적용 가능
- #[repr(C)] 구조체로 메모리 정렬 보장 후 unsafe로 슬라이스 변환 가능

---
# Byteorder 없이 순수 Rust 이용하기

## ✅ 안전한 방식: 반복문으로 직접 처리
```rust
use std::fs::File;
use std::io::{Write, Read, BufReader, BufWriter};

fn main() -> std::io::Result<()> {
    let array: [i32; 4] = [10, 20, 30, 40];

    // 쓰기
    let file = File::create("array.bin")?;
    let mut writer = BufWriter::new(file);
    for &val in &array {
        writer.write_all(&val.to_le_bytes())?; // 리틀 엔디안으로 직접 변환
    }

    // 읽기
    let file = File::open("array.bin")?;
    let mut reader = BufReader::new(file);
    let mut result = [0i32; 4];
    for i in 0..4 {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        result[i] = i32::from_le_bytes(buf); // 바이트 배열을 i32로 복원
    }

    println!("Read: {:?}", result);
    Ok(())
}
```

## 🔍 핵심 포인트

| 메서드              | 설명                                      |
|---------------------|-------------------------------------------|
| `to_le_bytes()`     | 숫자를 리틀 엔디안 바이트 배열로 변환       |
| `from_le_bytes()`   | 바이트 배열을 숫자로 복원                   |
| `write_all()`       | 바이트 슬라이스를 파일에 그대로 저장         |
| `read_exact()`      | 지정된 크기만큼 정확히 읽어오기             |



## ⚠️ 고급 방식: unsafe로 메모리 슬라이스 처리
```rust
use std::fs::File;
use std::io::{Write, Read};

fn main() -> std::io::Result<()> {
    let array: [f64; 4] = [1.1, 2.2, 3.3, 4.4];

    // 쓰기
    let mut file = File::create("array_f64.bin")?;
    let bytes = unsafe {
        std::slice::from_raw_parts(
            array.as_ptr() as *const u8,
            std::mem::size_of_val(&array),
        )
    };
    file.write_all(bytes)?;

    // 읽기
    let mut file = File::open("array_f64.bin")?;
    let mut buffer = [0u8; 32]; // 4 * 8 bytes
    file.read_exact(&mut buffer)?;
    let result = unsafe {
        std::ptr::read(buffer.as_ptr() as *const [f64; 4])
    };

    println!("Read: {:?}", result);
    Ok(())
}
```

⚠️ 이 방식은 메모리 정렬과 플랫폼 엔디안에 의존하므로, 반드시 #[repr(C)] 구조체나 배열을 사용할 때만 안전하게 동작합니다.


## 🧠 요약
| 방식              | 특징                                | 엔디안 제어     | 안전성       | 사용 예시                         |
|-------------------|-------------------------------------|------------------|--------------|----------------------------------|
| `to_le_bytes()`   | 숫자를 바이트 배열로 변환            | 직접 제어 가능   | ✅ 안전       | 반복문으로 배열 저장/복원         |
| `unsafe`          | 메모리 슬라이스로 직접 변환          | 플랫폼 의존      | ⚠️ 위험 가능 | 구조체나 이중 배열을 빠르게 처리  |


--- 
# Block 단위로 건너 띄면서 읽기
Fortran의 바이너리 파일은 종종 **레코드 기반 포맷(record-based format)** 을 사용해서,  
각 데이터 블록 앞뒤에 레코드 길이를 나타내는 바이트가 삽입됩니다.  
이 때문에 Rust에서 읽을 때는 정확한 바이트 크기 단위로 읽고 건너뛰는 로직이 필요합니다.  

## 🧠 핵심 개념: Fortran 바이너리 구조
Fortran의 unformatted 파일은 다음과 같은 구조를 가질 수 있습니다:
[4-byte 레코드 길이] [데이터 블록] [4-byte 레코드 길이]


예를 들어, REAL*8 3개를 저장했다면:
```
[24] [f64, f64, f64] [24]
```


## ✅ Rust에서 특정 바이트 크기로 읽는 방법
```rust
use std::fs::File;
use std::io::{Read, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("fortran_data.bin")?;
    let mut reader = BufReader::new(file);

    // 1️⃣ 레코드 길이 읽기 (앞쪽)
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let record_len = u32::from_le_bytes(len_buf); // 또는 from_be_bytes

    println!("Record length: {}", record_len);

    // 2️⃣ 데이터 블록 읽기
    let mut data_buf = vec![0u8; record_len as usize];
    reader.read_exact(&mut data_buf)?;

    // 예: f64 3개 읽기
    for i in 0..3 {
        let start = i * 8;
        let end = start + 8;
        let val = f64::from_le_bytes(data_buf[start..end].try_into().unwrap());
        println!("Value {}: {}", i, val);
    }

    // 3️⃣ 레코드 길이 읽기 (뒤쪽)
    reader.read_exact(&mut len_buf)?;
    let trailing_len = u32::from_le_bytes(len_buf);
    assert_eq!(record_len, trailing_len);

    Ok(())
}
```

## 🔍 설명
| 기능 또는 메서드               | 설명                                               |
|-------------------------------|----------------------------------------------------|
| `read_exact(&mut [u8])`       | 지정된 바이트 수만큼 정확히 읽어오기               |
| `from_le_bytes()` / `from_be_bytes()` | 바이트 배열을 숫자로 변환 (엔디안에 따라 선택)     |
| `Vec<u8>`                     | 가변 길이의 바이트 버퍼로 레코드 데이터 저장 가능   |
| `try_into().unwrap()`         | `[u8]` 슬라이스를 `[u8; N]` 고정 배열로 변환         |



## 🧠 팁
- Fortran은 보통 빅 엔디안을 사용하므로 from_be_bytes()가 필요할 수 있어요.
- 레코드 길이 앞뒤가 일치하는지 반드시 확인해야 데이터 손상 방지됩니다.
- 여러 레코드가 연속된 경우 반복해서 읽는 루프를 구성하면 됩니다.

---

# Block에서 데이터 쪼개서 쓰기

## 🧩 기본 전략
- 전체 블록을 Vec<u8>로 읽는다
- 슬라이스를 원하는 크기로 나눈다
- 각 슬라이스를 from_le_bytes() 또는 from_be_bytes()로 해석한다
- 타입별로 분리해서 저장하거나 처리한다

## ✅ 예제: Vec<u8>에서 i32, f64, u8 분리
```rust
use std::fs::File;
use std::io::{Read, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("record_block.bin")?;
    let mut reader = BufReader::new(file);

    // 1️⃣ 전체 블록 읽기
    let mut buffer = vec![0u8; 64]; // 예: 64바이트 블록
    reader.read_exact(&mut buffer)?;

    // 2️⃣ 분리해서 해석
    let int_val = i32::from_le_bytes(buffer[0..4].try_into().unwrap());
    let float_val = f64::from_le_bytes(buffer[4..12].try_into().unwrap());
    let byte_val = buffer[12]; // u8은 그대로 사용

    println!("int: {}", int_val);
    println!("float: {}", float_val);
    println!("byte: {}", byte_val);

    Ok(())
}
```


## 🔍 고급: 반복적으로 여러 값 추출
```rust
let mut offset = 0;
let mut ints = Vec::new();
let mut floats = Vec::new();

while offset + 12 <= buffer.len() {
    let i = i32::from_le_bytes(buffer[offset..offset+4].try_into().unwrap());
    let f = f64::from_le_bytes(buffer[offset+4..offset+12].try_into().unwrap());
    ints.push(i);
    floats.push(f);
    offset += 12;
}
```

- 이 방식은 i32 + f64가 반복되는 구조에 적합
- offset을 수동으로 조절해 원하는 구조로 파싱 가능

## 🧠 팁
| 기능 또는 키워드         | 설명 또는 예시                                 |
|--------------------------|-----------------------------------------------|
| `read_exact()`           | 지정된 바이트 수만큼 정확히 읽어오기           |
| `Vec<u8>`                | 가변 크기 블록을 저장하는 바이트 버퍼          |
| `try_into().unwrap()`    | `[u8]` 슬라이스를 `[u8; N]` 고정 배열로 변환    |
| `from_le_bytes()`        | 바이트 배열을 숫자로 해석 (리틀 엔디안 기준)    |
| `offset`                 | 블록 내 위치를 추적하며 반복적으로 파싱        |


## 📦 확장 아이디어
- #[repr(C)] 구조체를 정의하고 unsafe로 슬라이스를 구조체로 변환
- binrw 또는 nom 라이브러리를 사용해 복잡한 바이너리 포맷 파싱
- 레코드 길이 앞뒤에 있는 Fortran-style 헤더를 먼저 읽고, 그 길이만큼 반복 처리

---
# Big Endian 처리
Unix 시스템(특히 오래된 SPARC, PowerPC, 일부 IBM 메인프레임 등)은 **빅 엔디안(Big Endian)** 을 사용하는 경우가 많습니다.  
그래서 그런 환경에서 생성된 바이너리 파일을 Rust에서 읽을 때는 **리틀 엔디안 처리(from_le_bytes)** 를 하면 안 되고,  
빅 엔디안 처리(from_be_bytes)를 해야 정확하게 해석됩니다.

## 🧠 핵심 요약
| 상황 또는 플랫폼         | 해석 메서드              |
|--------------------------|--------------------------|
| 리틀 엔디안 (x86/x86_64) | `from_le_bytes()`        |
| 빅 엔디안 (Unix/서버 등) | `from_be_bytes()`        |

## ✅ 예제: 빅 엔디안 바이너리 읽기
```rust
use std::fs::File;
use std::io::{Read, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("unix_data.bin")?;
    let mut reader = BufReader::new(file);

    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let value = u32::from_be_bytes(buf); // 빅 엔디안으로 해석

    println!("Read value: {}", value);
    Ok(())
}
```
## 🔍 실무 팁
- 파일 포맷이 명확하지 않으면 처음 몇 바이트를 from_le_bytes와 from_be_bytes로 각각 해석해보고 값이 말이 되는지 비교해보는 것도 방법입니다.
- 네트워크 프로토콜도 대부분 빅 엔디안을 사용하므로, from_be_bytes()가 기본입니다.

---

# Byte Swap

## 🧩 전체 코드: 엔디안 변환 함수들
```rust
fn swap_endian_i32(val: i32) -> i32 {
    i32::from_be_bytes(val.to_le_bytes())
}

fn swap_endian_f32(val: f32) -> f32 {
    f32::from_be_bytes(val.to_le_bytes())
}

fn swap_endian_f64(val: f64) -> f64 {
    f64::from_be_bytes(val.to_le_bytes())
}
```


## ✅ 사용 예시

```rust
fn main() {
    let i: i32 = 0x12345678;
    let f32_val: f32 = 123.456;
    let f64_val: f64 = 9876.54321;

    let i_swapped = swap_endian_i32(i);
    let f32_swapped = swap_endian_f32(f32_val);
    let f64_swapped = swap_endian_f64(f64_val);

    println!("i32 original: {:#x}, swapped: {:#x}", i, i_swapped);
    println!("f32 original: {}, swapped: {}", f32_val, f32_swapped);
    println!("f64 original: {}, swapped: {}", f64_val, f64_swapped);
}
```

## 🔍 설명

| 기능 또는 메서드        | 설명                                               |
|-------------------------|----------------------------------------------------|
| `to_le_bytes()`         | 값을 리틀 엔디안 바이트 배열로 변환                 |
| `from_be_bytes()`       | 빅 엔디안 바이트 배열을 값으로 해석                 |
| `swap_endian_*()`       | 값의 바이트 순서를 반대로 뒤집어 엔디안 변환 수행   |

이 방식은 플랫폼 독립적이며, byteorder 크레이트 없이도 동작합니다.


## 예제: Vec<u8>에서 바이트 추출 후 타입 변환
```rust
fn parse_i32_le(bytes: &[u8], offset: usize) -> i32 {
    let slice = &bytes[offset..offset + 4];
    i32::from_le_bytes(slice.try_into().unwrap())
}

fn parse_i32_be(bytes: &[u8], offset: usize) -> i32 {
    let slice = &bytes[offset..offset + 4];
    i32::from_be_bytes(slice.try_into().unwrap())
}

fn parse_f32_le(bytes: &[u8], offset: usize) -> f32 {
    let slice = &bytes[offset..offset + 4];
    f32::from_le_bytes(slice.try_into().unwrap())
}

fn parse_f64_be(bytes: &[u8], offset: usize) -> f64 {
    let slice = &bytes[offset..offset + 8];
    f64::from_be_bytes(slice.try_into().unwrap())
}
```

## ✅ 사용 예시
```rust
fn main() {
    let raw: Vec<u8> = vec![
        0x78, 0x56, 0x34, 0x12,             // i32 (LE): 0x12345678
        0x42, 0xf6, 0xe9, 0x79,             // f32 (LE): ~123.456
        0x40, 0xc8, 0x1c, 0xd6, 0xe6, 0x31, 0xf8, 0xa1, // f64 (BE): ~9876.54321
    ];

    let int_val = parse_i32_le(&raw, 0);
    let float_val = parse_f32_le(&raw, 4);
    let double_val = parse_f64_be(&raw, 8);

    println!("i32 (LE): {}", int_val);
    println!("f32 (LE): {}", float_val);
    println!("f64 (BE): {}", double_val);
}
```

## 🔍 핵심 포인트
| 표현 또는 메서드                  | 설명 또는 예시                                 |
|----------------------------------|-----------------------------------------------|
| `&bytes[offset..offset+N]`       | 원하는 위치에서 N바이트 슬라이스 추출           |
| `try_into().unwrap()`            | `[u8]` → `[u8; N]` 고정 배열로 변환             |
| `from_le_bytes()` / `from_be_bytes()` | 바이트 배열을 숫자 타입으로 해석 (엔디안에 따라 선택) |
| `offset += N`                    | 다음 데이터 블록으로 이동 (반복 파싱 시 사용)   |


## 🧠 확장 아이디어
- Vec<u8>에서 반복적으로 구조체 배열을 추출하려면 offset을 루프에서 증가시키면 됩니다.
- Result를 반환하도록 바꾸면 에러 핸들링도 가능 (try_into()?).
- #[repr(C)] 구조체를 정의하고 unsafe로 슬라이스를 구조체로 변환하는 방식도 가능하지만, 지금처럼 안전한 방식이 실무에 더 적합합니다.

이제 Unix에서 넘어온 바이너리든, 센서 로그든, 구조체든 Vec<u8>만 있으면 Rust에서 자유자재로 해석할 수 있습니다.

---
