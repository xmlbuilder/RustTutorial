# Directory Watcher
Rust에서는 라는 크레이트를 사용하면 특정 디렉토리나 파일의 변경 사항을 실시간으로 감시하고 알림을 받을 수 있는 구조를 아주 쉽게 구현할 수 있음.  
이 크레이트는 내부적으로 OS별 감시 시스템을 사용합니다:
- Windows → ReadDirectoryChangesW
- Linux → inotify
- macOS → FSEvents

## 🛠️ 예제: 디렉토리 변경 감시 + 알림 출력
```rust
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    // 이벤트 수신 채널 생성
    let (tx, rx) = channel();

    // 감시자 생성 (2초 debounce)
    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();

    // 감시할 디렉토리 지정
    watcher.watch("C:\\MyCAD\\SolverOutput", RecursiveMode::Recursive).unwrap();

    println!("디렉토리 변경 감시 시작...");

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path) => println!("파일 생성됨: {:?}", path),
                DebouncedEvent::Write(path) => println!("파일 수정됨: {:?}", path),
                DebouncedEvent::Remove(path) => println!("파일 삭제됨: {:?}", path),
                DebouncedEvent::Rename(from, to) => println!("파일 이름 변경: {:?} → {:?}", from, to),
                _ => println!("기타 이벤트: {:?}", event),
            },
            Err(e) => println!("감시 오류: {:?}", e),
        }
    }
}
```



## ✅ 주요 기능
| 항목 또는 API             | 설명 또는 역할                                      |
|---------------------------|-----------------------------------------------------|
| `watch()`                 | 감시할 디렉토리 또는 파일을 등록                    |
| `RecursiveMode::Recursive`| 하위 디렉토리까지 포함하여 변경 사항 감시            |
| `DebouncedEvent`          | 생성, 수정, 삭제, 이름 변경 등 다양한 이벤트 타입    |
| `rx.recv()`               | 이벤트 수신 대기 (블로킹 방식)                      |


## 🧠 실무 팁
| 항목 또는 도구       | 설명 또는 역할                                         |
|----------------------|--------------------------------------------------------|
| `notify`             | 파일/디렉토리 변경 감지용 크레이트 (OS별 감시 API 활용) |
| `Duration`           | 이벤트 debounce 간격 설정 → 과도한 이벤트 방지          |
| `notify-rust`        | 시스템 알림 표시 (Windows/macOS/Linux 지원)             |
| `channel` + `recv()` | 이벤트 수신 및 처리 루프 구성                           |



---

# File Watcher

단일 파일도 감시할 수 있습니다.  
notify 크레이트는 디렉토리뿐 아니라 특정 파일 하나만 지정해서 변경 사항을 감지할 수 있어요.  
예를 들어, C:\\MyCAD\\SolverOutput\\result.txt 같은 파일이 생성되거나 수정될 때만 반응하도록 설정할 수 있습니다.

## 🛠️ 단일 파일 감시 예제
```rust
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;

fn main() {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();

    // 단일 파일 감시
    watcher.watch(Path::new("C:\\MyCAD\\SolverOutput\\result.txt"), RecursiveMode::NonRecursive).unwrap();

    println!("파일 변경 감시 시작...");

    loop {
        match rx.recv() {
            Ok(event) => println!("이벤트 발생: {:?}", event),
            Err(e) => println!("감시 오류: {:?}", e),
        }
    }
}
```


## ✅ 핵심 포인트
| 항목 또는 API               | 설명 또는 역할                                      |
|-----------------------------|-----------------------------------------------------|
| `RecursiveMode::NonRecursive` | 단일 파일 감시 시 사용. 하위 디렉토리 감시는 제외됨   |
| `watch(Path::new(...))`     | 감시 대상 파일 또는 디렉토리 경로 지정               |
| `DebouncedEvent::Write`     | 파일이 수정될 때 발생하는 이벤트                     |

## 🧠 실무 팁
- 파일이 수정될 때마다 자동으로 파싱하거나 UI에 반영하는 구조로 확장 가능
- notify는 OS별로 최적화된 감시 API를 사용하므로 성능도 우수
- 파일이 너무 자주 바뀌면 Duration을 늘려서 이벤트 폭주 방지

---

