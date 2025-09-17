# Shared Memory

## 🧠 핵심 개념: Shared Memory IPC
- 공유 메모리는 운영체제가 제공하는 기능으로, 두 개 이상의 프로세스가 동일한 물리 메모리 영역을 접근할 수 있게 해줍니다.
- Rust에서는 이를 위해 메모리 맵핑(mmap), FFI 기반 C 라이브러리, 또는 전용 크레이트를 사용합니다.

## 🧪 실무용 크레이트 예시
### 1.  크레이트
- IpcSharedMemory 구조체를 통해 공유 메모리 영역을 생성하고 전달 가능
- 안전하게 사용하려면 단일 읽기/쓰기 구조를 유지해야 함
- 예: 한 프로세스가 IpcSharedMemory를 생성하고, 다른 프로세스에 전달하여 읽기
### 2.  크레이트
- 메모리 맵핑을 통해 파일 또는 익명 메모리를 공유
- unsafe 코드가 필요할 수 있음 → 신중하게 사용
### 3.  (FFI 기반)
- 고성능 IPC를 위한 C++ 기반 라이브러리
- Rust에서 FFI로 사용 가능
- 싱글 프로듀서-싱글 컨슈머 구조에 적합

## ✅ 실무 예시 흐름
- 프로세스 A가 공유 메모리 생성 (IpcSharedMemory, mmap, etc.)
- 해당 메모리 핸들을 프로세스 B에 전달 (소켓, 파일 디스크립터 등)
- 프로세스 B가 해당 메모리를 매핑하여 접근
- 두 프로세스 간에 동기화 메커니즘 필요 (예: 뮤텍스, 세마포어)

## ⚠️ 주의할 점
- Rust는 기본적으로 프로세스 간 공유 메모리를 직접 지원하지 않음
- 대부분의 구현은 운영체제 기능 + unsafe 코드 + 외부 크레이트 조합
- 메모리 동기화와 안전성 확보가 핵심

---

# memmap2

 Rust에서 가장 선호되는 Shared Memory IPC 방식은 실무 기준으로 보면  크레이트를 사용하는 메모리 맵핑 기반 접근 방식입니다.  
 이 방식은 운영체제의 메모리 매핑 기능을 활용해 파일 또는 익명 메모리 영역을 여러 프로세스가 공유할 수 있게 해줍니다.

## ✅ 왜 memmap2가 선호될까?

| 항목 또는 장점         | 설명 또는 효과                                      |
|------------------------|-----------------------------------------------------|
| `unsafe` 최소화        | 메모리 매핑은 위험하지만 `memmap2`는 안전한 래퍼 제공 |
| 크로스 플랫폼 지원     | Windows, Linux, macOS 모두에서 동작 가능             |
| 파일 기반 공유         | 파일을 통해 여러 프로세스가 동일 메모리 접근 가능     |
| 성능 우수              | 디스크 I/O 없이 메모리처럼 빠르게 읽고 쓰기 가능       |
| 간단한 API             | `Mmap`, `MmapMut`로 직관적인 읽기/쓰기 구현 가능       |



## 🛠️ 공유 메모리 생성 및 접근 예제
### 1. Cargo.toml에 크레이트 추가
```
[dependencies]
memmap2 = "0.5"
```


### 2. 공유 메모리 생성 (쓰기 프로세스)
```rust
use std::fs::OpenOptions;
use std::io::Write;
use memmap2::MmapMut;

fn main() -> std::io::Result<()> {
    // 공유 파일 생성
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("shared.dat")?;

    // 크기 지정 (예: 128바이트)
    file.set_len(128)?;

    // 메모리 매핑 (쓰기 가능)
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    // 데이터 쓰기
    mmap[..5].copy_from_slice(b"Hello");

    // flush로 디스크에 반영 (다른 프로세스가 읽을 수 있게)
    mmap.flush()?;

    Ok(())
}
```


### 3. 공유 메모리 읽기 (다른 프로세스)
```rust
use std::fs::File;
use memmap2::Mmap;

fn main() -> std::io::Result<()> {
    // 공유 파일 열기
    let file = File::open("shared.dat")?;

    // 메모리 매핑 (읽기 전용)
    let mmap = unsafe { Mmap::map(&file)? };

    // 데이터 읽기
    let msg = std::str::from_utf8(&mmap[..5]).unwrap();
    println!("읽은 메시지: {}", msg); // → Hello

    Ok(())
}
```


## 🧠 실무 팁

| 항목 또는 목적            | 설명 또는 적용 방식                          |
|---------------------------|-----------------------------------------------|
| 공유 파일 경로            | `shared.dat` 같은 파일 기반 메모리 공유       |
| 익명 공유 메모리          | `memfd_create`, `/dev/shm` (Linux 전용)       |
| 동기화 메커니즘           | `Mutex`, `Atomic`, `semaphore` 등으로 충돌 방지 |
| 추천 크레이트             | `memmap2` → 안전하고 범용적인 mmap 지원       |

---

# int/float 넘기기

Shared Memory를 통해 integer, float 같은 원시 값도 다른 프로세스에 넘길 수 있습니다. 
Rust에서는 이를 위해 메모리 매핑된 버퍼에 바이너리 형식으로 값을 직접 쓰고 읽는 방식을 사용합니다. 
아래에 memmap2 기반으로 실제 숫자 데이터를 공유하는 방법을 설명.

## 🧩 핵심 개념
- i32, f64 같은 값은 바이트 배열로 변환해서 공유 메모리에 저장해야 함
- Rust의 to_le_bytes() / from_le_bytes()를 사용하면 안전하게 변환 가능
- 공유 메모리는 memmap2로 생성하고, 바이트 단위로 접근

## 🛠️ 예제: 숫자 값을 공유 메모리에 쓰고 읽기
### 1. 쓰기 프로세스 (integer + float 저장)
```rust
use std::fs::OpenOptions;
use memmap2::MmapMut;

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("shared.dat")?;

    file.set_len(16)?; // i32(4) + f64(8) + 여유

    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    let int_val: i32 = 42;
    let float_val: f64 = 3.1415;

    // 바이트로 변환해서 저장
    mmap[0..4].copy_from_slice(&int_val.to_le_bytes());
    mmap[4..12].copy_from_slice(&float_val.to_le_bytes());

    mmap.flush()?; // 디스크에 반영

    Ok(())
}
```


### 2. 읽기 프로세스 (integer + float 복원)
```rust
use std::fs::File;
use memmap2::Mmap;

fn main() -> std::io::Result<()> {
    let file = File::open("shared.dat")?;
    let mmap = unsafe { Mmap::map(&file)? };

    let int_val = i32::from_le_bytes(mmap[0..4].try_into().unwrap());
    let float_val = f64::from_le_bytes(mmap[4..12].try_into().unwrap());

    println!("읽은 값: int = {}, float = {}", int_val, float_val);

    Ok(())
}
```

## 🧠 실무 팁

| 타입     | 변환 방법                                 | 바이트 크기 | 비고                         |
|----------|-------------------------------------------|-------------|------------------------------|
| `i32`    | `.to_le_bytes()` / `.from_le_bytes()`     | 4 bytes     | 정수 값 공유에 사용           |
| `f64`    | `.to_le_bytes()` / `.from_le_bytes()`     | 8 bytes     | 실수 값 공유에 사용           |
| `bool`   | `u8`로 변환 (`0` 또는 `1`)                | 1 byte      | `true` → `1`, `false` → `0`   |


- Endian: 대부분의 시스템은 little-endian → to_le_bytes() 사용
- 정렬: 여러 값을 저장할 때는 바이트 오프셋을 정확히 계산해야 함
- 동기화: 동시에 읽고 쓰는 경우 Atomic 또는 Mutex로 보호 필요


---

# 동기화

프로세스 간 동기화는 정말 까다롭고, 잘못하면 데이터 레이스, 교착 상태, 성능 저하까지 일어날 수 있음. 
그래서 실무에서는 말씀하신 것처럼 일방향 흐름 + 플래그 기반 설계가 훨씬 안정적이고 관리하기 쉬운 방식.

## ✅ 왜 플래그 기반 일방향 구조가 선호될까?
| 항목 또는 이유             | 설명 또는 효과                                 |
|----------------------------|------------------------------------------------|
| 동기화 부담 감소           | `Mutex`, `Semaphore` 없이도 안전하게 동작 가능 |
| 구조가 단순함              | 읽기/쓰기 역할이 명확하게 분리됨               |
| 디버깅이 쉬움              | `flag` 상태만 보면 현재 흐름 파악 가능         |
| 실시간 처리에 적합         | 빠른 반응과 낮은 지연 → CAD, 센서, 렌더링에 유리 |


## 🧠 구조 설계 예시
### 📦 공유 메모리 구조 (128 bytes)

| Offset 범위 | 타입   | 역할 또는 설명                     |
|-------------|--------|------------------------------------|
| 0           | `u8`   | 상태 플래그 (`0`: 비어 있음, `1`: 데이터 있음) |
| 1 ~ 4       | `i32`  | 정수 값 (예: 상태 코드, 좌표 등)   |
| 5 ~ 12      | `f64`  | 실수 값 (예: 센서 값, 위치 등)     |
| 13 ~ 127    | `u8[]` | 여유 공간 또는 확장용 데이터 영역   |



## 🔁 흐름 예시
- Writer 프로세스
- flag == 0이면 → 데이터 쓰기
- 값 저장 후 flag = 1로 설정
- Reader 프로세스
- flag == 1이면 → 데이터 읽기
- 읽은 후 flag = 0으로 초기화
이렇게 하면 서로 충돌 없이 안전하게 데이터를 교환할 수 있어요.
물론 이 구조도 완벽하진 않지만, 단방향 흐름에서는 Mutex 없이도 충분히 안정적.

## 🛠️ 실무 팁
- AtomicU8을 쓰면 flag를 스레드/프로세스 안전하게 관리 가능
- flag를 2로 설정해서 “읽는 중” 같은 상태도 표현 가능
- sleep()이나 poll()을 활용해 간단한 대기 루프도 구현 가능


---

# sleep / poll

sleep()이나 poll()을 활용한 간단한 대기 루프는 Shared Memory 구조에서 읽기/쓰기 타이밍을 맞추기 위한 가장 직관적인 방식.  
특히 flag 기반 일방향 구조에서는 “데이터가 준비될 때까지 기다렸다가 읽기” 같은 흐름을 구현할 때 아주 유용하죠.

## 🧩 기본 개념
- Writer: 데이터를 쓰고 flag = 1로 설정
- Reader: flag == 1이 될 때까지 기다렸다가 읽고, flag = 0으로 초기화
- 대기 루프: sleep() 또는 poll()을 활용해 CPU를 과도하게 사용하지 않도록 함

## 🛠️ 샘플 코드: sleep() 기반 대기 루프
```rust
use std::{fs::File, thread::sleep, time::Duration};
use memmap2::Mmap;
use std::sync::atomic::{AtomicU8, Ordering};

fn main() -> std::io::Result<()> {
    let file = File::open("shared.dat")?;
    let mmap = unsafe { Mmap::map(&file)? };

    loop {
        let flag = mmap[0]; // flag 위치는 0번 오프셋

        if flag == 1 {
            // 데이터 읽기
            let int_val = i32::from_le_bytes(mmap[1..5].try_into().unwrap());
            let float_val = f64::from_le_bytes(mmap[5..13].try_into().unwrap());

            println!("읽은 값: int = {}, float = {}", int_val, float_val);

            // flag 초기화
            let mmap_mut = unsafe { Mmap::map_mut(&file)? };
            mmap_mut[0] = 0;
            break;
        }

        // 데이터가 아직 없으면 잠깐 대기
        sleep(Duration::from_millis(10));
    }

    Ok(())
}

```

## 🧠 실무 팁

| 항목        | 설명 또는 역할                                  |
|-------------|--------------------------------------------------|
| `sleep()`   | 간단한 대기 루프 구현에 사용. CPU 사용량 낮춤     |
| `poll()`    | OS 이벤트 기반 대기 가능. 고급 IPC에 적합         |
| `AtomicU8`  | `flag`를 안전하게 읽고 쓰기 위한 동기화 도구      |
| `Duration`  | `sleep()` 간격 조절. 너무 짧으면 busy-wait 발생    |



## ✅ 확장 아이디어
- flag = 2 → 읽는 중, 3 → 에러 등으로 상태를 세분화
- loop 대신 while let이나 match로 상태 관리
- sleep()을 점진적으로 늘리는 exponential backoff도 가능

