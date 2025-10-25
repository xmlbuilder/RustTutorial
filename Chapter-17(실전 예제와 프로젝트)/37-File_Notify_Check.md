# notify crate

Rust에서는 notify 크레이트를 사용하면 파일 시스템 변경을 감시할 수 있음.  
이 라이브러리는 다양한 OS에서 작동하며, 생성/삭제/수정 이벤트를 실시간으로 감지할 수 있습니다.

## 🔭 Rust에서 파일 변경 감시: notify 크레이트
### ✅ 주요 특징
- 크로스 플랫폼 지원: Linux(inotify), macOS(FSEvents/kqueue), Windows(ReadDirectoryChangesW)
- 비동기 및 디바운스 지원: 빠른 이벤트 처리와 중복 제거 가능
- 간단한 API: 콜백 기반으로 이벤트 처리

## 📦 설치 방법 (Cargo.toml)
```
[dependencies]
notify = "8.1.0"
```
- 필요 시 features = ["serde"] 옵션으로 이벤트 직렬화도 가능


## 🧪 기본 사용 예제
```rust
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch("D:\\Temp".as_ref(), RecursiveMode::Recursive).expect("TODO: panic message");
    println!("Watching...");
    for res in rx {
        match res {
            Ok(event) => println!("이벤트 발생: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

}
```
- RecursiveMode::Recursive는 하위 디렉토리까지 감시
- DebouncedEvent는 이벤트 중복을 줄여줌

## ⚠️ 주의사항
- WSL, Docker, NFS 환경에서는 일부 이벤트가 감지되지 않을 수 있음 → PollWatcher로 대체 가능
- 대용량 디렉토리는 감시 한계에 도달할 수 있음 → Linux에서는 fs.inotify.max_user_watches 설정 필요

---
