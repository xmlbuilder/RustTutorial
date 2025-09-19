# thread channel
Rust의 Channel은 스레드 간에 안전하게 데이터를 주고받을 수 있도록 설계된 메시지 전달 방식.  
복잡한 동기화 없이도 여러 스레드가 협업할 수 있게 해주는 핵심 도구 중 하나.

## 🧵 Channel이란?
Rust에서 Channel은 하나의 스레드가 데이터를 보내고, 다른 스레드가 그 데이터를 받는 구조.  
마치 물이 흐르는 파이프처럼, 한쪽에서 데이터를 넣으면 다른 쪽에서 꺼낼 수 있음.
- Channel은 두 부분으로 나뉘어:
- Sender (tx): 데이터를 보내는 쪽
- Receiver (rx): 데이터를 받는 쪽
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // 채널 생성

    thread::spawn(move || {
        let msg = String::from("Hello from thread!");
        tx.send(msg).unwrap(); // 메시지 전송
    });

    let received = rx.recv().unwrap(); // 메시지 수신 (blocking)
    println!("Received: {}", received);
}
```


## 🔁 mpsc란?
Rust의 Channel은 mpsc 모듈을 통해 제공돼. 이는 Multiple Producer, Single Consumer의 약자로,  
여러 개의 Sender가 하나의 Receiver로 데이터를 보낼 수 있다.
- tx.clone()을 통해 Sender를 복제하면 여러 스레드에서 동시에 데이터를 보낼 수 있어.
- Receiver는 하나만 존재하고, 데이터를 순서대로 받아.

### ⏳ Blocking vs Non-blocking
- recv()는 blocking 방식으로, 메시지가 올 때까지 기다려.
- try_recv()는 non-blocking 방식으로, 메시지가 없으면 바로 에러를 반환해.
```rust
match rx.try_recv() {
    Ok(msg) => println!("Received: {}", msg),
    Err(_) => println!("No message yet"),
}
```


## 📦 Channel의 특징 요약
| 특징           | 설명                                                         |
|----------------|--------------------------------------------------------------|
| 안전성         | Rust의 소유권 시스템 덕분에 데이터 경쟁이 없음              |
| 유연성         | 여러 Sender 가능, Receiver는 하나                            |
| 동기화         | 메시지 전달로 자연스럽게 동기화됨                            |
| 사용 용도      | 스레드 간 통신, 작업 분산, 이벤트 처리 등에 활용 가능        |
| Blocking 방식  | `recv()`는 메시지가 올 때까지 기다림                         |
| Non-blocking   | `try_recv()`는 메시지가 없으면 즉시 에러 반환                |

Channel은 Rust의 철학인 **"공유 메모리보다 메시지를 공유하라"**를 잘 보여주는 도구.  
복잡한 락(lock) 없이도 스레드 간 협업이 가능하니, 병렬 프로그래밍을 할 때 정말 유용.

--- 

# GUI 연동

Rust에서도 대부분의 GUI 프레임워크는 메인 스레드에서만 UI를 업데이트할 수 있음  
이는 다른 언어에서도 일반적인 제약.  
Rust에서는 이를 안전하게 처리하기 위해 Channel, Arc<Mutex<T>, 또는 Callback Queue 같은 구조를 활용.
아래는 eframe/egui를 사용해서 백그라운드 스레드에서 진행 상황(progress)을 계산하고, 메인 스레드에서 GUI를 업데이트하는 예제:

## 🧪 예제: 백그라운드 스레드에서 Progress 업데이트
```rust
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct MyApp {
    progress: Arc<Mutex<f32>>,
}

impl Default for MyApp {
    fn default() -> Self {
        let progress = Arc::new(Mutex::new(0.0));
        let progress_clone = Arc::clone(&progress);

        // 백그라운드 스레드 생성
        thread::spawn(move || {
            for i in 0..=100 {
                {
                    let mut prog = progress_clone.lock().unwrap();
                    *prog = i as f32 / 100.0;
                }
                thread::sleep(Duration::from_millis(50));
            }
        });

        Self { progress }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let progress = *self.progress.lock().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Progress Example");
            ui.add(egui::ProgressBar::new(progress).show_percentage());
        });

        // UI를 계속 갱신하도록 요청
        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Progress Demo",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
```


## 🔍 설명
- Arc<Mutex<f32>>를 사용해 스레드 간 안전하게 공유되는 진행률 값을 관리해.
- thread::spawn으로 백그라운드 작업을 수행하고, 진행률을 0.0 → 1.0까지 업데이트.
- ctx.request_repaint()를 통해 egui에게 UI를 계속 갱신하라고 요청함.
- ProgressBar 위젯을 사용해 시각적으로 진행률을 표시.

---
# 구조체 공유

Rust에서는 Arc<Mutex<T>>를 사용하면 **구조체(struct)**도 안전하게 여러 스레드에서 공.  
T가 구조체든 벡터든 맵이든 상관없이, Mutex는 그 데이터를 하나의 스레드만 접근할 수 있도록 보호하고,  
Arc는 그 데이터를 여러 스레드가 참조할 수 있도록 공유.

## 🧱 구조체 공유 예제

예를 들어, 아래처럼 AppState라는 구조체를 여러 스레드에서 공유:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct AppState {
    progress: u32,
    message: String,
}

fn main() {
    let state = Arc::new(Mutex::new(AppState {
        progress: 0,
        message: String::from("Starting..."),
    }));

    let mut handles = vec![];

    for i in 0..5 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let mut data = state_clone.lock().unwrap();
            data.progress += 10;
            data.message = format!("Thread {} updated progress", i);
            println!("Thread {}: {:?}", i, *data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final state: {:?}", *state.lock().unwrap());
}
```


## 🔍 핵심 포인트
- Arc::clone(&state)로 각 스레드에 공유된 참조를 넘겨줌.
- Mutex::lock()으로 안전하게 구조체에 접근하고 수정.
- 구조체 내부에 여러 필드가 있어도 문제 없이 접근 가능.

## 🧠 왜 구조체 공유가 중요한가?
- GUI 앱에서 백그라운드 작업이 진행 상황, 로그, 상태 등을 업데이트할 때
- 서버에서 클라이언트 요청을 처리하며 공통 상태를 유지할 때
- 게임에서 여러 엔티티가 공유된 월드 상태를 참조할 때


---

# 🛠️ Rust에서 BackgroundWorker 스타일 구현
Rust에서는 다음을 조합해서 BackgroundWorker와 유사한 구조를 만들 수 있어:
- std::thread::spawn → 백그라운드 작업 실행
- std::sync::mpsc::channel → 진행률 및 완료 이벤트 전달
- Arc<Mutex<T>> → 상태 공유
- GUI 프레임워크 (egui, iced, gtk-rs 등) → 메인 스레드에서 UI 업데이트

## ✅ 예제: 진행률 보고 + 완료 이벤트
```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Progress {
    percent: u32,
    message: String,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Progress>();
    let is_done = Arc::new(Mutex::new(false));
    let is_done_clone = Arc::clone(&is_done);

    // 백그라운드 작업 시작
    thread::spawn(move || {
        for i in 0..=100 {
            let msg = Progress {
                percent: i,
                message: format!("진행 중... {}%", i),
            };
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }

        // 완료 상태 설정
        let mut done = is_done_clone.lock().unwrap();
        *done = true;
    });

    // 메인 스레드에서 진행률 수신
    loop {
        if let Ok(progress) = rx.recv() {
            println!("Progress: {}% - {}", progress.percent, progress.message);
        }

        let done = *is_done.lock().unwrap();
        if done {
            println!("작업 완료!");
            break;
        }
    }
}
```


## 🔍 이 구조의 특징
| 구성 요소          | 역할 및 설명                                                  |
|-------------------|---------------------------------------------------------------|
| `thread::spawn`   | 백그라운드 스레드를 생성하여 작업을 비동기적으로 수행함       |
| `mpsc::channel`   | 메인 스레드와 백그라운드 스레드 간 메시지(진행률 등) 전달     |
| `Arc<Mutex<bool>>`| 작업 완료 여부를 여러 스레드에서 안전하게 공유하기 위한 구조  |
| `rx.try_recv()` / `recv()` | 메인 스레드에서 메시지를 수신하여 UI 또는 상태를 갱신함 |


## 🧩 GUI에 통합하려면?
예를 들어 egui를 사용한다면, rx.try_recv()를 update() 함수 안에서 호출하고, ctx.request_repaint()로 UI를 계속 갱신.  
이 방식은 C#의 ProgressChanged와 RunWorkerCompleted 이벤트를 대체할 수 있어.

GUI 없이도 GUI 통합을 흉내 내는 샘플. 
여기서는 GUI가 없으니까, 메인 스레드에서 UI를 업데이트하는 부분을 println!으로 대체.  
백그라운드 스레드에서 작업을 수행하고, 진행률을 mpsc::channel을 통해 메인 스레드로 보내는 구조.

## 🧪 Rust 샘플: GUI 없이 진행률 업데이트 시뮬레이션
```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Progress {
    percent: u32,
    message: String,
}

fn main() {
    // 채널 생성: 백그라운드 → 메인 스레드
    let (tx, rx) = mpsc::channel::<Progress>();

    // 완료 여부 공유 변수
    let is_done = Arc::new(Mutex::new(false));
    let is_done_clone = Arc::clone(&is_done);

    // 백그라운드 작업 시작
    thread::spawn(move || {
        for i in 0..=100 {
            let msg = Progress {
                percent: i,
                message: format!("진행 중... {}%", i),
            };
            tx.send(msg).unwrap(); // 진행률 전송
            thread::sleep(Duration::from_millis(50));
        }

        // 완료 상태 설정
        let mut done = is_done_clone.lock().unwrap();
        *done = true;
    });

    // 메인 스레드: GUI 업데이트 시뮬레이션
    loop {
        // GUI에서 진행률 수신 및 표시
        if let Ok(progress) = rx.try_recv() {
            println!("[GUI] Progress: {}% - {}", progress.percent, progress.message);
        }

        // 완료 여부 확인
        let done = *is_done.lock().unwrap();
        if done {
            println!("[GUI] 작업 완료!");
            break;
        }

        // GUI 프레임 갱신 시뮬레이션
        thread::sleep(Duration::from_millis(16)); // 약 60FPS
    }
}
```


## 🧩 설명
- Progress 구조체는 진행률과 메시지를 담고 있어.
- 백그라운드 스레드는 Progress를 생성해서 tx.send()로 전송.
- 메인 스레드는 rx.try_recv()로 메시지를 수신하고 println!으로 GUI처럼 출력.
- thread::sleep(16ms)는 GUI 프레임 갱신을 흉내 내는 용도야.

---

# eframe / egui

eframe과 egui는 Rust에서 GUI를 만들기 위한 핵심 라이브러리들.

## 🚀 설치 방법
### 1. Cargo.toml에 추가
[dependencies]
eframe = "0.27"

eframe은 egui를 포함한 프레임워크.  
egui만 따로 쓰고 싶다면 egui = "0.27"만 넣어도 돼.


## 🧱 기본 코드 구조
```rust
use eframe::{egui, epi};

struct MyApp;

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "My egui App"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, JungHwan!");
            ui.label("이건 egui로 만든 GUI야.");
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|_cc| Box::new(MyApp)),
    );
}
```


## 🧪 실행 방법
```
cargo run
```

GUI 창이 뜨면서 "Hello, JungHwan!"이라는 텍스트가 표시.


🧠 참고 사항
- eframe은 egui를 기반으로 만든 프레임워크로, **웹(WASM)**과 네이티브 데스크탑 앱 모두 지원해.
- 웹 앱으로 빌드하려면 wasm32-unknown-unknown 타겟을 설치해야 해:

```
rustup target add wasm32-unknown-unknown
```



