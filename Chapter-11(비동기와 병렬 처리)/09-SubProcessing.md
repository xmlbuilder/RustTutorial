# subprocess
subprocess는 시스템 프로그래밍이나 공학적 계산에서 자주 등장하는 개념,
CAD나 해석 시스템을 만드는 사람에게는 아주 중요한 도구.

🧠 Subprocess란?
**Subprocess(서브프로세스)**는
_현재 실행 중인 프로그램이 외부 프로그램이나 명령어를 별도의 프로세스로 실행하는 것을 말함.
즉, 부모 프로세스가 자식 프로세스를 생성해서 특정 작업을 맡기는 구조.

## 🔧 핵심 특징
- 독립된 실행 흐름: 부모와 별도로 실행되며, 자체 메모리 공간을 가짐
- 표준 입출력 연결 가능: stdin, stdout, stderr를 통해 데이터 주고받기 가능
- 시간 제한 가능: 일정 시간 후 종료하거나, 타임아웃 설정 가능
- 자원 격리: 메모리 충돌이나 예외가 부모에게 직접 영향을 주지 않음
- 멀티 OS 지원: Windows, Linux, macOS 모두에서 사용 가능


## 📦 설치 방법
Cargo.toml에 다음을 추가하면 됨:
```
[dependencies]
tokio = { version = "1", features = ["full"] }
sysinfo = "0.30"
```



## 🦀 Rust 예제: 
- subprocess 실행 + stdout 수신 + 타임아웃 종료
```rust
use tokio::process::Command;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. subprocess 실행
    let mut child = Command::new("ping")
        .arg("localhost") // 예시: ping 명령어
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    // 2. stdout 읽기 준비
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let mut reader = BufReader::new(stdout).lines();

    // 3. 일정 시간 동안 메시지 수신
    let duration = Duration::from_secs(5);
    let result = timeout(duration, async {
        while let Some(line) = reader.next_line().await? {
            println!("stdout: {}", line);
        }
        Ok::<_, anyhow::Error>(())
    }).await;

    // 4. 타임아웃 발생 시 프로세스 종료
    match result {
        Ok(_) => println!("Subprocess completed within time."),
        Err(_) => {
            println!("Timeout reached. Killing subprocess...");
            child.kill().await?;
        }
    }

    Ok(())
}
```


## ✅ 주요 포인트
- tokio::process::Command → 비동기 subprocess 실행
- BufReader::lines() → stdout을 비동기로 읽음
- timeout() → 일정 시간 기다림
- child.kill() → 타임아웃 시 subprocess 종료


## macOS open

아래는 macOS에서 open 명령어를 Rust로 실행하면서 working directory를 설정하는 예제.  
Rust의 `std::process::Command`를 사용해서 구현.

### 🧪 예제: macOS에서 open 명령어 + working directory 설정
```rust
use std::process::Command;
use std::env;

fn main() -> std::io::Result<()> {
    // 원하는 working directory 설정
    let working_dir = "/Users/junghwan/Documents";

    // 실행할 파일 또는 앱
    let target = "MyApp.app"; // 또는 "myfile.pdf", "index.html" 등

    // open 명령어 실행
    let status = Command::new("open")
        .arg(target)
        .current_dir(working_dir) // working directory 설정
        .status()?; // 실행 결과 확인

    if status.success() {
        println!("Successfully opened: {}", target);
    } else {
        eprintln!("Failed to open: {}", target);
    }

    Ok(())
}
```


## ✅ 주요 포인트
- Command::new("open") → macOS에서 앱이나 파일 열기
- .current_dir(working_dir) → 해당 디렉토리에서 명령 실행
- target은 상대 경로 또는 앱 이름 가능 (.app, .pdf, .html 등)

## 💡 참고
- open은 macOS에서 GUI 앱을 실행하거나 파일을 연결된 앱으로 여는 데 사용됨
- working directory를 설정하면, 앱이 해당 디렉토리를 기준으로 파일을 찾거나 로그를 남길 수 있음
- 앱이 자체적으로 working directory를 무시하는 경우도 있으니, 앱의 동작 방식에 따라 달라질 수 있음

## LD_LIBRARY_PATH 설정
Linux에서 LD_LIBRARY_PATH 설정은 진짜 번거롭고 까다로운 부분 중 하나.  
Rust에서도 서브 프로세스를 실행할 때 환경 변수로 LD_LIBRARY_PATH를 지정할 수 있음,  
그래서 외부 라이브러리를 필요로 하는 프로그램을 안정적으로 구동할 수 있음.

## 🧪 Rust에서 LD_LIBRARY_PATH 설정하고 subprocess 실행
```rust
use std::process::Command;
use std::env;

fn main() -> std::io::Result<()> {
    // 라이브러리 경로 지정
    let lib_path = "/home/junghwan/my_libs";

    // 실행할 프로그램
    let program = "./my_binary";

    // subprocess 실행 + 환경 변수 설정
    let status = Command::new(program)
        .env("LD_LIBRARY_PATH", lib_path)
        .status()?; // 실행 결과 확인

    if status.success() {
        println!("프로그램이 성공적으로 실행되었습니다.");
    } else {
        eprintln!("프로그램 실행 실패.");
    }

    Ok(())
}
```


## ✅ 주요 포인트
- .env("LD_LIBRARY_PATH", path) → 런타임에 환경 변수 설정
- Rust는 Command를 통해 환경 변수, working directory, stdin/stdout 제어 가능
- 이 방식은 멀티 OS 중에서도 Linux에 특화된 설정을 Rust에서 유연하게 처리할 수 있게 해줌


## 💡 추가 팁
- 여러 환경 변수를 동시에 설정하고 싶다면 .envs()를 사용하면 됨
- 만약 Rust 프로그램 자체가 외부 C 라이브러리를 FFI로 링크해야 한다면,  
build.rs에서 println!("cargo:rustc-link-search=native=...")로 처리하는 방식도 있음


## 🔧 Rust에서 subprocess 종료 (terminate) 하기
```rust
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("sleep")
        .arg("10") // 10초 동안 대기하는 예시
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start subprocess");

    // 2초 후 강제 종료
    std::thread::sleep(std::time::Duration::from_secs(2));
    let _ = child.kill(); // 여기서 terminate!
}
```

## 핵심 메서드:
- spawn() → 자식 프로세스 실행
- kill() → 자식 프로세스 종료
- wait() → 종료될 때까지 대기
- try_wait() → 비동기적으로 상태 확인

## 🧠 주의할 점
- kill()은 SIGKILL을 보내는 방식이라, 리소스 정리 없이 강제 종료됨
- Windows에서는 TerminateProcess에 해당하고, Unix에서는 SIGKILL
- 종료 후에는 wait() 또는 try_wait()로 좀비 프로세스 방지 필요



## 🛠️ sysinfo 크레이트로 시스템 정보 확인하기
```rust
use sysinfo::{System, SystemExt, ProcessExt};

fn main() {
    let mut sys = System::new_all(); // 모든 정보 초기화
    sys.refresh_all(); // 최신 정보로 갱신

    // 메모리 정보
    println!("Total memory: {} KB", sys.total_memory());
    println!("Used memory : {} KB", sys.used_memory());

    // CPU 사용률
    println!("CPU usage: {:.2}%", sys.global_cpu_info().cpu_usage());

    // 실행 중인 프로세스 목록
    for (pid, process) in sys.processes() {
        println!("[{}] {} - {:.2}%", pid, process.name(), process.cpu_usage());
    }
}
```

## 🔍 주요 기능 요약 (`sysinfo` 크레이트 기준)

| 항목               | 설명                                 | API                                           |
|--------------------|--------------------------------------|-----------------------------------------------|
| 전체 메모리        | 시스템의 총 메모리 용량 (KB 단위)     | `sys.total_memory()`                          |
| 사용 중 메모리     | 현재 사용 중인 메모리 (KB 단위)       | `sys.used_memory()`                           |
| CPU 사용률         | 전체 CPU의 평균 사용률 (%)            | `sys.global_cpu_info().cpu_usage()`           |
| 프로세스 목록      | 실행 중인 모든 프로세스 정보          | `sys.processes()`                             |
| 프로세스 이름      | 개별 프로세스의 이름                  | `process.name()`                              |
| 프로세스 CPU 사용률| 해당 프로세스의 CPU 사용률 (%)        | `process.cpu_usage()`                         |


---

## 🧠 1. Tokio + sysinfo 비동기 모니터링 샘플

아래는 tokio와 sysinfo를 연동해서  
주기적으로 시스템 상태를 비동기적으로 모니터링하는 전체 샘플 코드:
```rust
use sysinfo::{System, SystemExt, ProcessExt};
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
    let mut ticker = interval(Duration::from_secs(2)); // 2초마다 모니터링

    loop {
        ticker.tick().await;
        sys.refresh_all();

        println!("🧠 CPU Usage: {:.2}%", sys.global_cpu_info().cpu_usage());
        println!("💾 Memory Used: {} / {}", sys.used_memory(), sys.total_memory());

        // 상위 CPU 사용 프로세스 3개 출력
        let mut processes: Vec<_> = sys.processes().values().collect();
        processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

        for proc in processes.iter().take(3) {
            println!("🔧 [{}] {} - {:.2}%", proc.pid(), proc.name(), proc.cpu_usage());
        }

        println!("----------------------------------------");
    }
}

```


## 예시: tokio-cron-scheduler
```rust
use tokio_cron_scheduler::{JobScheduler, Job};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let sched = JobScheduler::new().await.unwrap();

    let job = Job::new_async("1/5 * * * * *", |_uuid, _l| {
        Box::pin(async move {
            println!("🕒 Job 실행됨: 5초마다 반복");
        })
    }).unwrap();

    sched.add(job).await.unwrap();
    sched.start().await.unwrap();

    tokio::signal::ctrl_c().await.unwrap(); // Ctrl+C로 종료
}
```



