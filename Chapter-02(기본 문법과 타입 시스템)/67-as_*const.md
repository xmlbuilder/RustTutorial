# 🧩 1. as *const T란?
- as *const T는 어떤 값의 **불변 참조(&T)** 를 raw pointer인 `*const T` 로 변환합니다.
- 반대로 가변 참조(&mut T)는 `*mut T` 로 변환할 수 있습니다.
```rust
let x = 42;
let ptr: *const i32 = &x as *const i32;
```
- 여기서 ptr은 x를 가리키는 불변(raw) 포인터입니다.
- 이 포인터는 unsafe 블록 안에서만 역참조할 수 있습니다.

## 🔐 2. 왜 raw pointer를 사용할까?
Rust는 기본적으로 **안전성(safety)** 을 보장하기 위해 참조에 대해 엄격한 규칙을 적용합니다.  
하지만 다음과 같은 상황에서는 raw pointer가 필요합니다:
- FFI (Foreign Function Interface): C 라이브러리와 상호작용할 때
- Unsafe 최적화: 성능을 위해 안전성 검사를 우회할 때
- 동시성 제어: 여러 스레드에서 메모리에 접근할 때

## 🔧 3. 예제: raw pointer 사용
```rust
fn main() {
    let x = 100;
    let ptr = &x as *const i32;

    unsafe {
        println!("ptr points to: {}", *ptr);
    }
}
```

- `&x as *const i32`: 안전한 참조를 raw pointer로 변환
- `unsafe { *ptr }`: raw pointer를 역참조하려면 반드시 unsafe 블록이 필요

## 🔁 4. *const vs *mut

| 포인터 타입 | 설명                     | 변환 예시              |
|-------------|--------------------------|------------------------|
| *const T    | 불변(raw) 포인터         | &x as *const T         |
| *mut T      | 가변(raw) 포인터         | &mut x as *mut T       |

- raw pointer는 null일 수 있고, dangling일 수도 있으며, 데이터 경합을 일으킬 수도 있기 때문에 사용 시 주의가 필요합니다.

## 🧠 5. 실전 활용: FFI 예시
```rust
extern "C" {
    fn strlen(s: *const u8) -> usize;
}

fn main() {
    let s = "hello";
    let len = unsafe {
        strlen(s.as_ptr())
    };
    println!("Length: {}", len);
}
```

- `s.as_ptr()` 는 `*const u8` 을 반환 → C 함수에 넘기기 적합
- strlen은 C의 문자열 길이 함수

## ⚠️ 6. 주의사항
- raw pointer는 소유권, 수명, 빌림 검사를 우회하므로 안전하지 않음
- 반드시 unsafe 블록 안에서만 사용해야 하며, 역참조 시 유효성 보장 책임은 개발자에게 있음
- 일반적인 Rust 코드에서는 &T, &mut T를 사용하는 것이 훨씬 안전하고 권장됨

## ✅ 정리

| 표현             | 설명                                      |
|------------------|-------------------------------------------|
| `as *const T`      | 안전한 참조를 불변 raw 포인터로 변환       |
| `unsafe *ptr`      | raw 포인터를 역참조할 때 필요한 unsafe 블록 |
| `*const T` / `*mut T`| 불변 / 가변 raw 포인터 타입                |

## 🧠 핵심 요점
- `as *const T`: &T를 raw 포인터로 변환하는 문법
- `unsafe *ptr`: raw 포인터는 안전하지 않기 때문에 역참조 시 unsafe 필요
- `*const T`: 읽기 전용 포인터
- `*mut T`: 쓰기 가능한 포인터

--


## 🧩 1. 구조체 포인터: as *const Struct
### ✅ 예제
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```
```rust
fn main() {
    let p = Point { x: 10, y: 20 };
    let ptr: *const Point = &p as *const Point;

    unsafe {
        println!("x = {}, y = {}", (*ptr).x, (*ptr).y);
    }
}
```
- `&p as *const Point`: 구조체 Point에 대한 불변 raw 포인터 생성
- unsafe 블록에서 `*ptr` 로 `역참조` 하여 필드 접근

## 🧩 2. 배열 포인터: as *const [T; N]
### ✅ 예제
```rust
fn main() {
    let arr = [1, 2, 3, 4];
    let ptr: *const [i32; 4] = &arr as *const [i32; 4];

    unsafe {
        println!("first = {}", (*ptr)[0]);
    }
}
```
- 배열 전체를 포인터로 변환
- (*ptr)[i]로 요소 접근 가능

## 🧩 3. 슬라이스 포인터: as *const T
### ✅ 예제
```rust
fn main() {
    let arr = [10, 20, 30];
    let slice = &arr[..]; // &[i32]
    let ptr: *const i32 = slice.as_ptr(); // 슬라이스의 첫 요소 포인터

    unsafe {
        println!("first = {}", *ptr);
    }
}
```
- slice.as_ptr()는 슬라이스의 첫 요소를 가리키는 *const T 반환
- 슬라이스는 [T] 타입이므로 포인터는 *const T로 표현됨

### ⚠️ 주의사항

| 항목           | 설명                                      |
|----------------|-------------------------------------------|
| unsafe         | raw pointer 역참조는 반드시 unsafe 필요     |
| 유효성 책임     | 포인터가 유효한 메모리를 가리키는지 개발자가 직접 보장해야 함 |
| null 가능성     | raw pointer는 null일 수 있음               |
| dangling 위험   | 이미 해제된 메모리를 참조할 수 있음        |
| 수명 추적 없음 | Rust의 borrow checker가 관여하지 않음       |

### 🧠 핵심 요점
- raw pointer는 강력하지만 Rust의 안전성 보장 밖에 있음
- unsafe는 단순히 "위험을 감수하겠다"는 선언이지, 자동으로 안전해지는 건 아님
- 포인터가 null이거나 dangling일 경우, undefined behavior가 발생할 수 있


## ✅ 정리

| 대상       | 변환 표현식               | 포인터 타입       |
|------------|---------------------------|-------------------|
| 구조체     | &s as *const Struct       | *const Struct     |
| 배열       | &arr as *const [T; N]     | *const [T; N]     |
| 슬라이스   | slice.as_ptr()            | *const T          |

## 🧠 핵심 요점
- &s as *const Struct: 구조체를 불변 raw 포인터로 변환
- &arr as *const [T; N]: 배열 전체를 raw 포인터로 변환
- slice.as_ptr(): 슬라이스의 첫 요소를 가리키는 raw 포인터 반환

---

## 🔧 기본 방법: 바이트 → 숫자 타입 변환
Rust는 숫자 타입마다 바이트 배열을 변환하는 메서드를 제공합니다:

| 타입 | 변환 방식                                               | 바이트 수 |
|------|----------------------------------------------------------|-----------|
| i32  | i32::from_ne_bytes([u8; 4])                              | 4         |
| f32  | f32::from_bits(u32::from_ne_bytes([u8; 4]))              | 4         |
| f64  | f64::from_bits(u64::from_ne_bytes([u8; 8]))              | 8         |


## ✅ 예제: 바이트 슬라이스에서 숫자 추출
```rust
fn main() {
    let bytes: &[u8] = &[0x00, 0x00, 0x80, 0x3f,  // f32: 1.0
                         0x00, 0x00, 0x00, 0x40,  // f64: 2.0
                         0x78, 0x56, 0x34, 0x12]; // i32: 0x12345678

    let f = f32::from_bits(u32::from_le_bytes(bytes[0..4].try_into().unwrap()));
    let d = f64::from_bits(u64::from_le_bytes(bytes[4..12].try_into().unwrap()));
    let i = i32::from_le_bytes(bytes[12..16].try_into().unwrap());

    println!("f32: {}", f);   // 1.0
    println!("f64: {}", d);   // 2.0
    println!("i32: {}", i);   // 305419896
}
```

- try_into()는 [u8] → [u8; N]로 변환
- from_le_bytes는 리틀 엔디안 기준 (파일 포맷에 따라 be 또는 ne로 변경 가능)

## 🧠 고급: 안전 vs 성능
- 안전한 방식: try_into() + from_le_bytes → 안정적이고 명확
- 고성능 (unsafe): std::ptr::read_unaligned 또는 transmute → 빠르지만 위험
```rust
unsafe {
    let ptr = bytes.as_ptr().add(4) as *const u64;
    let val = f64::from_bits(ptr.read_unaligned());
    println!("f64: {}", val);
}
```

- 반드시 unsafe 블록에서 사용
- 메모리 정렬(alignment) 문제에 주의

## 📦 추천 라이브러리
- : 다양한 엔디안 지원, 안전한 API
- : 고성능 바이트 변환기 (unsafe 기반)

## ✅ 정리

| 변환 대상 | 변환 방법                                                        |
|-----------|------------------------------------------------------------------|
| i32       | i32::from_le_bytes(bytes[..4].try_into().unwrap())              |
| f32       | f32::from_bits(u32::from_le_bytes(bytes[..4].try_into().unwrap())) |
| f64       | f64::from_bits(u64::from_le_bytes(bytes[..8].try_into().unwrap())) |

| 기타 도구         | 설명                                      |
|------------------|-------------------------------------------|
| bytes[start..end]| 바이트 슬라이스에서 원하는 범위 추출       |
| try_into()       | [u8] → [u8; N]로 변환 (정확한 길이 필요)     |
| unsafe read_unaligned | 정렬되지 않은 메모리에서 직접 읽기 (고성능, 위험) |

---

## 🎧 WAV 파일 기본 구조

| 필드 이름       | 크기 (바이트) | 설명                                         |
|------------------|----------------|----------------------------------------------|
| ChunkID          | 4              | "RIFF" 문자열                                 |
| ChunkSize        | 4              | 전체 파일 크기 - 8                            |
| Format           | 4              | "WAVE" 문자열                                 |
| Subchunk1ID      | 4              | "fmt " 문자열                                 |
| Subchunk1Size    | 4              | 포맷 데이터 크기 (PCM은 16)                   |
| AudioFormat      | 2              | 오디오 포맷 (1 = PCM)                         |
| NumChannels      | 2              | 채널 수 (1 = Mono, 2 = Stereo)               |
| SampleRate       | 4              | 샘플링 레이트 (예: 44100Hz)                   |
| ByteRate         | 4              | SampleRate × NumChannels × BitsPerSample/8   |
| BlockAlign       | 2              | NumChannels × BitsPerSample/8                |
| BitsPerSample    | 2              | 샘플당 비트 수 (예: 16)                       |
| Subchunk2ID      | 4              | "data" 문자열                                 |
| Subchunk2Size    | 4              | 오디오 데이터 크기 (바이트 단위)             |
| Data             | N              | 실제 오디오 샘플 데이터                       |

## 🧠 참고 사항
- 전체 헤더 크기는 일반적으로 44바이트
- 모든 필드는 리틀 엔디안으로 저장됨
- Data 필드 이후부터가 실제 오디오 샘플이며, BitsPerSample에 따라 i16, u8, f32 등으로 해석됨


## 🧩 Rust 구조체 정의
```rust
#[repr(C)]
#[derive(Debug)]
struct WavHeader {
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
    subchunk1_id: [u8; 4],
    subchunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    subchunk2_id: [u8; 4],
    subchunk2_size: u32,
}
```
- `#[repr(C)]` 는 C 스타일 메모리 정렬을 보장
- u32, u16은 from_le_bytes()로 변환해야 함

## 📥 파싱 예제
```rust
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("sample.wav").unwrap();
    let mut buffer = [0u8; 44]; // WAV 헤더는 44바이트

    file.read_exact(&mut buffer).unwrap();

    let header = WavHeader {
        chunk_id: buffer[0..4].try_into().unwrap(),
        chunk_size: u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
        format: buffer[8..12].try_into().unwrap(),
        subchunk1_id: buffer[12..16].try_into().unwrap(),
        subchunk1_size: u32::from_le_bytes(buffer[16..20].try_into().unwrap()),
        audio_format: u16::from_le_bytes(buffer[20..22].try_into().unwrap()),
        num_channels: u16::from_le_bytes(buffer[22..24].try_into().unwrap()),
        sample_rate: u32::from_le_bytes(buffer[24..28].try_into().unwrap()),
        byte_rate: u32::from_le_bytes(buffer[28..32].try_into().unwrap()),
        block_align: u16::from_le_bytes(buffer[32..34].try_into().unwrap()),
        bits_per_sample: u16::from_le_bytes(buffer[34..36].try_into().unwrap()),
        subchunk2_id: buffer[36..40].try_into().unwrap(),
        subchunk2_size: u32::from_le_bytes(buffer[40..44].try_into().unwrap()),
    };

    println!("{:#?}", header);
}
```

## ✅ 정리

| 항목                         | 설명                                                   |
|------------------------------|--------------------------------------------------------|
| buffer[start..end]           | 바이트 슬라이스에서 원하는 범위 추출                   |
| try_into().unwrap()          | 슬라이스를 고정 크기 배열로 변환                       |
| from_le_bytes()              | 리틀 엔디안 바이트 배열을 숫자 타입으로 변환           |
| #[repr(C)]                   | 구조체의 메모리 레이아웃을 C 스타일로 고정              |


## 🎧 WAV 오디오 데이터를 i16으로 파싱하는 흐름
### 1. WAV 헤더 읽기 (44바이트)
```rust
use std::fs::File;
use std::io::{Read, BufReader};

let mut file = BufReader::new(File::open("sample.wav")?);
let mut header = [0u8; 44];
file.read_exact(&mut header)?;
```


###  2. subchunk2_size 추출 → 오디오 데이터 크기
```rust
let data_size = u32::from_le_bytes(header[40..44].try_into().unwrap());
```


### 3 . 오디오 데이터 읽기
```rust
let mut audio_data = vec![0u8; data_size as usize];
file.read_exact(&mut audio_data)?;
```


### 4. 바이트 → i16 샘플로 변환
```rust
let samples: Vec<i16> = audio_data
    .chunks_exact(2) // 16비트 = 2바이트
    .map(|chunk| i16::from_le_bytes(chunk.try_into().unwrap()))
    .collect();
```

- chunks_exact(2): 2바이트씩 자름
- from_le_bytes: 리틀 엔디안 기준으로 i16으로 변환

## ✅ 전체 예제 요약
```rust
use std::fs::File;
use std::io::{Read, BufReader};

fn main() -> std::io::Result<()> {
    let mut file = BufReader::new(File::open("sample.wav")?);

    let mut header = [0u8; 44];
    file.read_exact(&mut header)?;

    let data_size = u32::from_le_bytes(header[40..44].try_into().unwrap());
    let mut audio_data = vec![0u8; data_size as usize];
    file.read_exact(&mut audio_data)?;

    let samples: Vec<i16> = audio_data
        .chunks_exact(2)
        .map(|chunk| i16::from_le_bytes(chunk.try_into().unwrap()))
        .collect();

    println!("샘플 개수: {}", samples.len());
    println!("첫 샘플: {}", samples[0]);

    Ok(())
}
```

## ⚠️ 주의사항

| 항목             | 설명                                         |
|------------------|----------------------------------------------|
| i16              | 16비트 PCM 오디오에만 해당됨 (`chunks_exact(2)`) |
| u8               | 8비트 WAV는 `u8`로 처리해야 함 (`chunks_exact(1)`) |
| 샘플 순서        | 스테레오일 경우 [L, R, L, R, ...] 순서로 나열됨 |
| 엔디안           | 대부분의 WAV는 리틀 엔디안 → `from_le_bytes()` 사용 |
| 포맷 확인 필요   | 헤더의 `audio_format`, `bits_per_sample` 확인 필수 |

---



