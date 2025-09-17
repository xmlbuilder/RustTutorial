# Process Handling
Rust에서도 프로세스를 생성하고 종료하며, 시스템에 어떤 프로세스들이 실행 중인지 확인하는 기능을 충분히 다룰 수 있습니다.

## 🧠 1. 프로세스 생성: std::process::Command
Rust에서는 Command 구조체를 사용해 외부 프로그램을 실행할 수 있음.
```rust
use std::process::Command;

fn main() {
    let child = Command::new("echo")
        .arg("Hello, CAD!")
        .spawn()
        .expect("Failed to start process");

    println!("Started process with PID: {}", child.id());
}
```

- spawn()은 비동기 실행
- output()은 결과를 기다렸다가 반환
- status()는 종료 코드만 확인

## 🛑 2. 프로세스 종료: Child.kill()
```rust
use std::process::Command;

fn main() {
    let mut child = Command::new("sleep")
        .arg("10")
        .spawn()
        .expect("Failed to start");

    // 2초 후 강제 종료
    std::thread::sleep(std::time::Duration::from_secs(2));
    child.kill().expect("Failed to kill process");
}
```

- kill()은 해당 프로세스를 강제 종료
- wait()을 호출하면 종료될 때까지 기다림

## 🔍 3. 실행 중인 프로세스 확인 (Linux 기준)
Rust 자체로는 시스템 전체 프로세스를 나열하진 않지만, ps, pgrep 명령어를 호출해서 확인할 수 있어요:
```rust
use std::process::Command;

fn main() {
    let output = Command::new("ps")
        .arg("-e")
        .output()
        .expect("Failed to execute ps");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
```

- ps -e: 전체 프로세스 목록
- pgrep <name>: 특정 이름의 프로세스 PID 검색

## 🧠 실무 팁
| 기능 또는 목적             | 사용 방법 또는 API                                |
|----------------------------|----------------------------------------------------|
| 프로세스 생성              | `Command::new(...).spawn()`                        |
| 프로세스 종료 또는 대기    | `Child.kill()` / `Child.wait()`                   |
| 실행 중 프로세스 조회      | `ps`, `pgrep` 명령어 호출 → `Command`로 실행       |
| 결과 파싱 및 출력          | `output.stdout` → `String::from_utf8_lossy(...)`  |


## ✅ 1. Command + wait() + 채널 기반 메시지 전달
Rust의 으로 솔버를 실행하고, 별도 스레드에서 기다렸다가 메시지를 보내는 방식이 가장 직관적입니다.

### 🛠️ 예제: 솔버 완료 후 메시지 전달
```rust
use std::process::Command;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let status = Command::new("long_solver")
            .arg("--run")
            .status()
            .expect("Failed to start solver");

        if status.success() {
            tx.send("솔버 완료!").unwrap();
        } else {
            tx.send("솔버 실패!").unwrap();
        }
    });

    // 다른 작업 수행 중...
    println!("다른 업무 수행 중...");

    // 메시지 수신
    let msg = rx.recv().unwrap();
    println!("알림: {}", msg);
}
```

- thread::spawn으로 솔버를 백그라운드에서 실행
- mpsc::channel로 메시지를 메인 스레드에 전달
- recv()는 블로킹 방식 → 메시지가 올 때까지 기다림

## ✅ 2. try_recv() + sleep()으로 비동기 폴링
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // 긴 작업 시뮬레이션
        thread::sleep(Duration::from_secs(5));
        tx.send("솔버 완료!").unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("알림: {}", msg);
                break;
            }
            Err(_) => {
                println!("솔버 진행 중...");
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
```

- try_recv()는 메시지가 없으면 바로 반환
- sleep()으로 CPU 사용량 조절
- UI나 콘솔에서 진행 상태를 표시할 수 있음

## ✅ 3. 고급 방식: OS 알림, WebSocket, DB 트리거 등
- OS 알림: 데스크탑 앱이라면 notify-rust 크레이트로 시스템 알림 가능
- WebSocket: 서버-클라이언트 구조라면 실시간 메시지 푸시 가능
- 파일/DB 트리거: 솔버가 결과 파일을 쓰면, 감시 프로세스가 이를 감지해 알림

## 🧠 실무 팁
| 기능 또는 방식           | 설명 또는 사용 예시                              |
|--------------------------|--------------------------------------------------|
| `mpsc::channel` + `recv()` | 백그라운드 스레드에서 메시지를 보내고 수신 대기     |
| `try_recv()` + `sleep()`   | 비동기 폴링 방식으로 상태를 주기적으로 확인         |
| `notify-rust`             | 데스크탑 OS 알림을 띄워 사용자에게 즉시 전달        |
| `flag` 기반 공유 메모리    | 외부 프로세스가 상태 플래그를 설정 → 감시 프로세스가 읽음 |

---

# 외부 프로그램 실행하기
Rust의 std::process::Command API를 사용하면 외부 프로그램을 실행하고, 인자를 전달하고, 표준 출력(stdout)을 캡처하고, 
**종료 코드(return code)** 까지 받을 수 있음.

## 🛠️ 예제: Solver.exe 실행 + stdout 캡처 + 종료 코드 확인
```rust
use std::process::Command;

fn main() {
    let output = Command::new("C:\\Program\\Solver.exe")
        .args(&["data1", "data2", "data3"])
        .output()
        .expect("Solver 실행 실패");

    // 표준 출력 캡처
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Solver 출력:\n{}", stdout);

    // 종료 코드 확인
    match output.status.code() {
        Some(0) => println!("Solver 정상 종료"),
        Some(code) => println!("Solver 에러 종료 (코드: {})", code),
        None => println!("Solver가 종료 코드를 반환하지 않음"),
    }
}
```


## ✅ 설명
🧠 실무 팁

| 항목 또는 API             | 설명 또는 예시                                   |
|---------------------------|--------------------------------------------------|
| `.args(&[...])`           | 인자 전달 → `["data1", "data2", "data3"]`        |
| `.output()`               | 프로세스 실행 후 stdout/stderr 캡처 및 종료 대기 |
| `output.stdout`           | 표준 출력 결과 → `String::from_utf8_lossy(...)` |
| `output.status.code()`    | 종료 코드 확인 → `0`: 성공, 그 외: 에러 코드     |

## 🧠 실무 팁
| 항목 또는 API               | 설명 또는 사용 예시                                      |
|-----------------------------|----------------------------------------------------------|
| `.args(&["arg1", "arg2", ...])` | 외부 프로그램에 인자 전달 (`data1`, `data2`, `data3`)       |
| `.spawn()` + `Child.wait()`     | 비동기 실행 후 종료까지 기다림 (실시간 처리에 적합)         |
| `.output().stdout` + `String::from_utf8_lossy` | 표준 출력 캡처 및 문자열 변환                          |
| `.output().stderr`             | 표준 에러 메시지 캡처 (디버깅 및 오류 처리에 유용)         |

----

# Standard Output / Input

Rust에서는 외부 프로세스를 실행하면서 표준 출력(stdout)을 실시간으로 스트리밍할 수 있습니다.
 즉, 솔버가 아직 실행 중이어도 출력되는 내용을 줄 단위로 받아서 바로 처리하거나 화면에 표시할 수 있음.

## ✅ 핵심 기술: Command::spawn() + Stdio::piped() + BufReader::lines()
이 조합을 사용하면 프로세스가 출력하는 내용을 실시간으로 읽을 수 있습니다.

###  🛠️ 실시간 출력 스트리밍 예제
```rust
use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

fn main() {
    let mut child = Command::new("C:\\Program\\Solver.exe")
        .args(&["data1", "data2", "data3"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Solver 실행 실패");

    let stdout = child.stdout.take().expect("stdout 가져오기 실패");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        match line {
            Ok(content) => println!("Solver 출력: {}", content),
            Err(e) => eprintln!("읽기 오류: {}", e),
        }
    }

    let status = child.wait().expect("Solver 종료 대기 실패");
    println!("Solver 종료 코드: {:?}", status.code());
}
```


## 🧠 실무 팁

| 항목 또는 API         | 설명 또는 역할                                      |
|------------------------|-----------------------------------------------------|
| `Stdio::piped()`       | 표준 출력/에러를 Rust 코드로 가져올 수 있게 설정       |
| `BufReader::lines()`   | 출력 스트림을 줄 단위로 실시간 읽기 가능               |
| `.spawn()`             | 외부 프로세스를 비동기 실행 (즉시 반환됨)              |
| `.wait()`              | 프로세스 종료까지 대기 (종료 코드 확인 가능)           |
| `.stdout.take()`       | `Child` 구조체에서 stdout 핸들을 가져와 스트리밍 처리   |


## 📦 확장 아이디어
- stderr도 Stdio::piped()로 설정하면 에러 메시지도 실시간으로 읽을 수 있어요
- 출력 내용을 로그 파일에 저장하거나 UI에 실시간 반영 가능
- 출력이 너무 많을 경우 BufReader 대신 io::copy()로 스트리밍 처리 가능


----

# 🧰 Windows에서 프로세스 리스트 가져오기

## 1.  크레이트 사용
```
# Cargo.toml
[dependencies]
sysinfo = "0.29"
```
```rust
use sysinfo::{System, SystemExt, ProcessExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_processes();

    for (pid, process) in sys.processes() {
        println!("PID: {} | Name: {}", pid, process.name());
    }
}
```

- sysinfo는 cross-platform 지원
- CPU, 메모리, 디스크, 네트워크 정보도 함께 조회 가능

## 2. Windows API 직접 호출 (고급 방식)
Rust에서 Windows API를 직접 호출하려면 windows 크레이트를 사용해 CreateToolhelp32Snapshot, 
Process32First, Process32Next를 호출해야 합니다. 
이 방식은 C++에서 하던 것과 거의 동일한 흐름.

## 🔍 참고: PowerShell에서는 이렇게도 가능
```
Get-Process | Select-Object Name, Id
```

→ Rust에서 PowerShell을 호출해 결과를 파싱하는 방식도 가능하지만, sysinfo 크레이트가 훨씬 깔끔합니다.

## 🧠 요약: Rust에서 프로세스 리스트 가져오기

| 방법 또는 크레이트     | 장점                          | 단점                          | 용도 및 특징                          |
|------------------------|-------------------------------|-------------------------------|---------------------------------------|
| `sysinfo`              | 크로스 플랫폼, 사용 간편       | 세부 정보 제한 가능성         | 프로세스 이름, PID, CPU/메모리 사용량 등 |
| Windows API 직접 호출 | 세부 정보 완벽 제어 가능       | 복잡한 unsafe 코드 필요        | `CreateToolhelp32Snapshot` 등 사용     |
| PowerShell 호출        | 빠른 구현 가능                 | 외부 의존성, 파싱 필요         | `Get-Process` 결과를 Rust에서 파싱     |


---
