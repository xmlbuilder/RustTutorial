# fsWatcher
이 코드는 Rust로 작성된 파일 시스템 감시 매니저(FsWatchManager)를 구현한 예제입니다.  
notify 크레이트를 활용해 디렉토리나 파일의 변경 이벤트를 감지하고, 옵저버 패턴을 통해 외부에 알리는 구조입니다.  
아래에 전체 구조와 사용된 주요 기법들을 정리.

## 📄 FsWatchManager 문서화 및 기술 설명
### 🧩 주요 구성 요소
### 1. FileChangeKind (열거형)
- 파일 변경 이벤트의 종류를 정의합니다.
- notify::EventKind를 기반으로 Created, Modified, Removed, Renamed, Other로 분류합니다.
- as_u64() 메서드를 통해 각 종류를 숫자로 매핑하여 메시지 전송 시 활용합니다.

### 2. FileChangeEvent (구조체)
- notify::Event를 간결하게 정리한 구조입니다.
- 변경된 경로(paths)와 변경 종류(kind)를 포함합니다.
- from_notify_event() 메서드는 notify::Event를 FileChangeEvent로 변환합니다.

### 3. FsWatchManager (싱글톤 매니저)
- 파일 시스템 감시를 담당하는 핵심 구조체입니다.
- 내부 구성:
    - RecommendedWatcher: notify 감시 객체
    - Subject: 옵저버 패턴을 위한 주체
    - rx: 이벤트 수신 채널
    - current_event: 현재 이벤트를 공유하기 위한 Arc<Mutex<>> 핸들
- 주요 메서드:
    - watch_dir(), watch_file(): 감시 대상 등록
    - process_pending_events(): 이벤트 수신 및 옵저버에게 알림
    - attach_observer(): 옵저버 등록
    - current_event_handle(): 옵저버와 공유할 이벤트 핸들 반환

### 4. LoggingFileObserver (예제 옵저버)
- Observer 트레이트를 구현한 구조체
- update_observer() 메서드에서 이벤트를 받아 로그 출력

### 5. FS_WATCH_MANAGER (싱글톤 인스턴스)
- once_cell::sync::Lazy와 parking_lot::RwLock을 활용해 전역에서 접근 가능한 싱글톤으로 구현

## 🛠️ 사용된 주요 기법 및 라이브러리
### ✅ 옵저버 패턴
- Subject와 Observer를 통해 이벤트 발생 시 등록된 옵저버에게 알림을 전파
- notify() 메서드로 메시지와 파라미터를 전달

### ✅ 싱글톤 패턴
- Lazy<RwLock<FsWatchManager>>를 통해 전역에서 접근 가능한 단일 인스턴스 구현
- RwLock을 사용해 읽기/쓰기 동시 접근을 제어

### ✅ 채널 기반 이벤트 처리
- crossbeam_channel::unbounded()로 비동기 이벤트 수신
- try_recv()로 이벤트를 폴링 방식으로 처리

### ✅ 파일 시스템 감시
- notify 크레이트의 RecommendedWatcher를 사용
- Config::default().with_poll_interval()로 폴링 간격 설정

### ✅ 동기화 및 공유
- Arc<Mutex<>>를 통해 이벤트 정보를 여러 쓰레드/옵저버와 안전하게 공유

## 🧪 테스트 및 실행 예제
### 테스트 코드 (#[cfg(test)])
- setup_file_watcher() 테스트 함수에서 감시 대상 디렉토리를 등록하고 10초간 이벤트를 처리
### 메인 함수
- fs_watcher_test()에서 D:\Temp 디렉토리를 감시하고 무한 루프로 이벤트를 처리

## 📌 확장 아이디어
- 다양한 옵저버 등록 (예: DB 저장, UI 반영)
- 이벤트 필터링 및 디바운싱
- 이벤트 로그 파일 저장
- GUI 연동 (예: Tauri, egui)

## ✅ 현재 구조가 Linux/macOS에서도 작동 가능한 이유
### 1. notify 크레이트는 크로스 플랫폼 지원
- notify는 Linux (inotify), macOS (FSEvents), Windows (ReadDirectoryChangesW) 등 다양한 OS의 파일 시스템 이벤트를 추상화하여 제공합니다.
- RecommendedWatcher는 각 플랫폼에 맞는 최적의 백엔드를 자동으로 선택합니다.
### 2. 경로 처리에 std::path::Path 사용
- Path와 PathBuf는 플랫폼 독립적인 경로 표현을 제공하므로, 코드 자체는 OS에 종속되지 않습니다.
- 단, 문자열 리터럴로 경로를 지정할 때는 OS에 맞는 형식 ("D:\\Temp" vs "/tmp" 등)을 사용해야 합니다.
### 3. 멀티스레딩 및 동기화는 플랫폼 독립적
- Arc, Mutex, RwLock, crossbeam_channel 등은 모두 Rust 표준 라이브러리 또는 크로스 플랫폼 크레이트에서 제공되므로 OS에 관계없이 동작합니다.

## ⚠️ Linux/macOS에서 주의할 점

| 항목                 | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| 경로 형식            | Windows: `D:\\Temp` → Linux/macOS: `/tmp`, `/home/user/...` 등       |
| 경로 권한            | 감시 대상 디렉토리에 읽기 권한 필요. 필요 시 `chmod` 또는 `sudo` 사용 |
| notify 백엔드 차이   | Linux: `inotify`, macOS: `FSEvents` 사용. 이벤트 감지 방식이 다름     |
| 이벤트 종류 차이     | macOS는 `Rename` 이벤트가 `Modify(Name)`로 들어올 수 있음            |
| 경로 인코딩          | Windows: UTF-16, Unix 계열: UTF-8. 경로 문자열 처리 시 인코딩 주의    |
| 파일 시스템 특성     | NFS, SMB 등 일부 파일 시스템은 이벤트 감지 제한 또는 지연 가능성 있음 |
| 폴링 설정            | `with_poll_interval()` 값 조정 필요할 수 있음                         |
| 경로 구분자          | Windows: `\`, Unix 계열: `/` — `Path`/`PathBuf` 사용 시 자동 처리됨   |

---

## 🛠️ Linux/macOS에서 사용하려면?
- 경로 수정
```rust
watcher.watch_dir("/tmp", true)?; // 예: Linux/macOS
```
- 권한 확인
- 감시할 디렉토리에 대한 읽기 권한이 있는지 확인하세요.
- sudo 권한이 필요한 경우도 있습니다.
- 테스트 시 경로 변경
```rust
#[test]
fn setup_file_watcher() {
    ...
    watcher.watch_dir("/tmp", true)?; // 또는 적절한 경로
    ...
}
```
- 이벤트 종류 확인
- notify의 이벤트 종류가 OS마다 다를 수 있으므로, FileChangeKind::from_event_kind()에서 로그를 찍어 확인해보는 것이 좋습니다.

## ✅ 결론
- 구조적으로는 완전히 크로스 플랫폼입니다.
- 경로만 OS에 맞게 바꾸면 Linux/macOS에서도 문제없이 작동합니다.
- 다만, 이벤트 종류나 감지 방식의 차이는 테스트를 통해 확인하고 필요 시 보완하는 것이 좋습니다.

----

## 소스 코드
```rust
use crate::core::observer::Observer;
use std::sync::{Arc, Weak};
pub struct Subject {
    observers: Vec<Weak<dyn Observer>>,
}

impl Subject {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn attach_observer(&mut self, obs: &Arc<dyn Observer>) {
        // 중복 방지
        let ptr = Arc::as_ptr(obs) as *const ();
        let exists = self.observers.iter().any(|w| {
            if let Some(s) = w.upgrade() {
                (Arc::as_ptr(&s) as *const ()) == ptr
            } else {
                false
            }
        });
        if !exists {
            self.observers.push(Arc::downgrade(obs));
        }
    }

    pub fn detach_observer(&mut self, obs: &Arc<dyn Observer>) {
        let ptr = Arc::as_ptr(obs) as *const ();
        self.observers.retain(|w| {
            if let Some(s) = w.upgrade() {
                (Arc::as_ptr(&s) as *const ()) != ptr
            } else {
                false
            }
        });
    }

    pub fn clear_observers(&mut self) {
        self.observers.clear();
    }

    pub fn notify(&mut self, msg: u32, wparam: u64, lparam: u64) -> i32 {
        // dead weak 정리
        self.observers.retain(|w| w.upgrade().is_some());
        println!("observers len: {}", self.observers.len());
        for w in &self.observers {
            println!("observer: w {:?}", w);
            if let Some(rc) = w.upgrade() {
                println!("observer: rc");
                let r = rc.update_observer(self as *const Subject, msg, wparam, lparam);
                if r != 0 {
                    return r;
                }
            }
        }
        0
    }
}
```
```rust
impl Default for Subject {
    fn default() -> Self {
        Self::new()
    }
}
```
```rust
use crate::core::subject;

pub trait Observer: Send + Sync {
    fn update_observer(
        &self,
        subject: *const subject::Subject,
        msg: u32,
        w_param: u64,
        l_param: u64,
    ) -> i32;
}
```
```rust
use crate::core::observer::Observer;
use crate::core::subject::Subject;

use notify::event::ModifyKind;
use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher,
};

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, TryRecvError, unbounded};

pub const MSG_FS_EVENT: u32 = 0x1000_0001;

/// 파일/디렉토리 변경 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileChangeKind {
    Created,
    Modified,
    Removed,
    Renamed,
    Other,
}

impl FileChangeKind {
    fn from_event_kind(kind: &EventKind) -> Self {
        match kind {
            EventKind::Create(_) => FileChangeKind::Created,
            EventKind::Modify(ModifyKind::Name(_)) => FileChangeKind::Renamed,
            EventKind::Modify(_) => FileChangeKind::Modified,
            EventKind::Remove(_) => FileChangeKind::Removed,
            _ => FileChangeKind::Other,
        }
    }

    fn as_u64(self) -> u64 {
        match self {
            FileChangeKind::Created => 1,
            FileChangeKind::Modified => 2,
            FileChangeKind::Removed => 3,
            FileChangeKind::Renamed => 4,
            FileChangeKind::Other => 0,
        }
    }
}
```
```rust
/// notify 이벤트를 정리한 구조
#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    pub paths: Vec<PathBuf>,
    pub kind: FileChangeKind,
}

impl FileChangeEvent {
    fn from_notify_event(ev: Event) -> Vec<FileChangeEvent> {
        let kind = FileChangeKind::from_event_kind(&ev.kind);
        if ev.paths.is_empty() {
            return vec![FileChangeEvent {
                paths: Vec::new(),
                kind,
            }];
        }
        vec![FileChangeEvent {
            paths: ev.paths,
            kind,
        }]
    }
}
```
```rust
/// Directory / File Watcher 싱글톤
pub struct FsWatchManager {
    subject: Subject,
    watcher: RecommendedWatcher,
    rx: Receiver<notify::Result<Event>>,
    /// 현재 처리 중인 이벤트 (Observer와 공유)
    current_event: Arc<Mutex<Option<FileChangeEvent>>>,
}

impl FsWatchManager {
    fn new_internal() -> Self {
        // crossbeam 채널 생성
        let (tx, rx): (
            Sender<notify::Result<Event>>,
            Receiver<notify::Result<Event>>,
        ) = unbounded();

        // notify watcher 생성 (클로저에서 tx로 이벤트 전달)
        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )
        .expect("Failed to create RecommendedWatcher");

        FsWatchManager {
            subject: Subject::new(),
            watcher,
            rx,
            current_event: Arc::new(Mutex::new(None)),
        }
    }

    /// Preferences 스타일 싱글톤 accessor
    pub fn instance() -> &'static RwLock<FsWatchManager> {
        &FS_WATCH_MANAGER
    }

    /// Observer 등록 (Subject에 위임)
    pub fn attach_observer(&mut self, obs: &Arc<dyn Observer>) {
        self.subject.attach_observer(obs);
    }

    /// 디렉토리 감시 추가
    pub fn watch_dir<P: AsRef<Path>>(&mut self, path: P, recursive: bool) -> NotifyResult<()> {
        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        self.watcher.watch(path.as_ref(), mode)
    }

    /// 파일 감시 추가
    pub fn watch_file<P: AsRef<Path>>(&mut self, path: P) -> NotifyResult<()> {
        self.watcher
            .watch(path.as_ref(), RecursiveMode::NonRecursive)
    }

    /// pending 이벤트를 모두 꺼내서 Observer들에게 통지
    pub fn process_pending_events(&mut self) {
        loop {
            match self.rx.try_recv() {
                Ok(Ok(ev)) => {
                    // println!("[FsWatchManager] raw event: {:?}", ev); // 디버그 필요하면 사용
                    let events = FileChangeEvent::from_notify_event(ev);
                    for ev2 in events {
                        {
                            // 현재 이벤트 설정
                            let mut lock = self.current_event.lock().unwrap();
                            *lock = Some(ev2.clone());
                        }

                        let kind_code = ev2.kind.as_u64();
                        let _ = self.subject.notify(MSG_FS_EVENT, kind_code, 0);

                        {
                            // 처리 후 비움 (선택 사항)
                            let mut lock = self.current_event.lock().unwrap();
                            *lock = None;
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("[FsWatchManager] watch error: {:?}", e);
                }
                Err(TryRecvError::Empty) => {
                    // 더 이상 이벤트 없음
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    eprintln!("[FsWatchManager] channel disconnected");
                    break;
                }
            }
        }
    }

    /// Observer 쪽에 넘겨줄 current_event 핸들
    pub fn current_event_handle(&self) -> Arc<Mutex<Option<FileChangeEvent>>> {
        Arc::clone(&self.current_event)
    }

    /// (필요하면 사용할 수 있는 helper)
    pub fn current_event(&self) -> Option<FileChangeEvent> {
        self.current_event.lock().unwrap().clone()
    }
}
```
```rust
/// 실제 싱글톤 인스턴스
pub static FS_WATCH_MANAGER: Lazy<RwLock<FsWatchManager>> =
    Lazy::new(|| RwLock::new(FsWatchManager::new_internal()));
```
```rust
/// 예제 Observer: 파일 변경 로그 출력
pub struct LoggingFileObserver {
    current_event: Arc<Mutex<Option<FileChangeEvent>>>,
}
```
```rust
impl LoggingFileObserver {
    pub fn new(current_event: Arc<Mutex<Option<FileChangeEvent>>>) -> Self {
        Self { current_event }
    }
}
```
```rust
impl Observer for LoggingFileObserver {
    fn update_observer(
        &self,
        _subject: *const Subject,
        msg: u32,
        w_param: u64,
        _l_param: u64,
    ) -> i32 {
        if msg == MSG_FS_EVENT {
            let kind_code = w_param;
            let ev_opt = self.current_event.lock().unwrap().clone();

            if let Some(ev) = ev_opt {
                println!(
                    "[LoggingFileObserver] msg={:#x}, kind={}, paths={:?}",
                    msg, kind_code, ev.paths
                );
            } else {
                println!(
                    "[LoggingFileObserver] msg={:#x}, kind={} (no current_event)",
                    msg, kind_code
                );
            }
        }
        0
    }
}
```
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    #[ignore] // 필요할 때만 직접 실행
    fn setup_file_watcher() {
        // 매니저에서 current_event 핸들을 가져와서 Observer에 주입
        let current_event_handle = {
            let mgr_lock = FsWatchManager::instance().read();
            mgr_lock.current_event_handle()
        };

        let observer: Arc<dyn Observer> = Arc::new(LoggingFileObserver::new(current_event_handle));

        {
            let watcher_lock = FsWatchManager::instance();
            let mut watcher = watcher_lock.write();

            watcher.attach_observer(&observer);
            watcher
                .watch_dir("D:\\Temp", true)
                .expect("failed to watch directory");
        }

        println!("Now watching D:\\Temp ...");

        let start = std::time::Instant::now();
        while start.elapsed().as_secs() < 10 {
            {
                let watcher_lock = FsWatchManager::instance();
                let mut watcher = watcher_lock.write();
                watcher.process_pending_events();
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
```
```rust
fn on_fs_watcher_dir<P: AsRef<Path>>(path: P) {
    // current_event 핸들 얻기
    let current_event_handle = {
        let mgr_lock = FsWatchManager::instance().read();
        mgr_lock.current_event_handle()
    };

    // Observer 생성
    let observer: Arc<dyn Observer> = Arc::new(LoggingFileObserver::new(current_event_handle));

    {
        let watcher_lock = FsWatchManager::instance();
        let mut watcher = watcher_lock.write();

        watcher.attach_observer(&observer);
        watcher
            .watch_dir(path.as_ref(), true)
            .expect("failed to watch directory");
    }

    println!("Now watching {:?}... (Ctrl+C to exit)", path.as_ref());

    loop {
        {
            let watcher_lock = FsWatchManager::instance();
            let mut watcher = watcher_lock.write();
            watcher.process_pending_events();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```


###  테스트 코드
```rust
fn main() {
    // current_event 핸들 얻기
    let current_event_handle = {
        let mgr_lock = FsWatchManager::instance().read();
        mgr_lock.current_event_handle()
    };

    // Observer 생성
    let observer: Arc<dyn Observer> = Arc::new(LoggingFileObserver::new(current_event_handle));

    {
        let watcher_lock = FsWatchManager::instance();
        let mut watcher = watcher_lock.write();

        watcher.attach_observer(&observer);
        watcher
            .watch_dir("D:\\Temp", true)
            .expect("failed to watch directory");
    }

    println!("Now watching D:\\Temp ... (Ctrl+C to exit)");

    loop {
        {
            let watcher_lock = FsWatchManager::instance();
            let mut watcher = watcher_lock.write();
            watcher.process_pending_events();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```

---
