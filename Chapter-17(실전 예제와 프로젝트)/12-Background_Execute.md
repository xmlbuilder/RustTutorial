# Background Execute

## ✅ 1. 기본 구조: 루프 + 시간 체크
Rust의 std::time::SystemTime 또는 chrono를 사용해서 현재 시간을 체크하고 조건이 맞으면 작업 수행하는 방식입니다.

```rust
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread::sleep;

fn main() {
    loop {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // 예: 매일 13:30:00에 실행
        let target_time = 13 * 3600 + 30 * 60;

        if now % 86400 == target_time {
            println!("지정된 시간입니다! 작업 실행 중...");
            // 여기에 원하는 작업 수행
        }

        sleep(Duration::from_secs(1)); // CPU 과다 사용 방지
    }
}
```


## ✅ 2. 고급 방식: cron 스타일 스케줄링
Rust에서는  스타일의 스케줄링을 구현할 수 있는 크레이트도 있어요. 예를 들어 를 사용하면 아래처럼 설정할 수 있습니다:
```rust
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();

    // 매일 13:30에 실행
    sched.add(Job::new("30 13 * * *".parse().unwrap(), || {
        println!("지정된 시간입니다! 작업 실행 중...");
        // 원하는 작업 수행
    }));

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}
```


## ✅ 3. 완전한 데몬화: daemonize 크레이트
Rust에서 진짜 데몬 프로세스를 만들고 싶다면  크레이트를 사용할 수 있어요. 이걸 사용하면:
- 백그라운드에서 실행
- stdout/stderr 로그 파일 지정
- PID 파일 생성
- 권한 설정 등 가능


## 🧠 실무 팁

| 항목 또는 도구                  | 설명 또는 역할                                               |
|----------------------------------|--------------------------------------------------------------|
| `SystemTime`, `chrono`, `job_scheduler` | 시간 기반 작업 스케줄링 (예: 매일 13:30에 자동 실행)             |
| `daemonize`                      | 백그라운드 데몬 프로세스 생성 (로그, PID, 권한 설정 등 지원)     |
| `notify`, `tokio`, `async`       | 파일 변경 감지, 비동기 이벤트 처리, 실시간 반응 구조 구현 가능     |
| `log`, `env_logger`, `tracing`   | 로그 기록, 디버깅, 상태 추적 등 시스템 운영에 필요한 로깅 기능     |

이 구조는 CAD 프로젝트에서 야간 자동 계산, 주기적 백업, 결과 파일 정리 같은 작업을 자동화하는 데 아주 유용합니다.

## ✨ 예시: C vs Rust 비교
### C (간단한 ls 실행)
```cpp
#include <unistd.h>
int main() {
    execlp("ls", "ls", NULL);
    return 0;
}
```

### Rust (동일한 작업)
```rust
use std::process::Command;
fn main() {
    Command::new("ls").status().unwrap();
}
```

Rust는 단순히 “쉽다”는 걸 넘어서, 실수를 방지하면서도 성능을 유지하는 구조를 갖추고 있음.


