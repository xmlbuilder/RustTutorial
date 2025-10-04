# tokio_futures
tokio_futures는 Rust의 비동기 생태계에서 중요한 역할을 하는 라이브러리 중 하나입니다.  
하지만 요즘은 대부분의 기능이 tokio와 futures 크레이트에 통합되어 있어서 tokio_futures 자체는 덜 사용되는 경향이 있습니다.  
그래도 그 개념과 역할을 이해하는 건 매우 중요.

## 🧠 기본 개념: Futures란?
- Future는 아직 완료되지 않은 비동기 작업을 나타내는 타입입니다.
- JavaScript의 Promise, Java의 CompletableFuture와 유사한 개념이지만 Rust에서는 제로 코스트 추상화와 메모리 안전성을 핵심으로 설계되어 있습니다.
- .await를 통해 Future가 완료될 때까지 기다릴 수 있습니다.

## ⚙️ Tokio와 Futures의 관계
Tokio는 Rust에서 가장 널리 사용되는 비동기 런타임입니다. 이 런타임은 Future를 실행하기 위한 환경을 제공합니다.
- tokio는 내부적으로 futures 크레이트를 기반으로 동작합니다.
- tokio_futures는 과거에 tokio와 futures 간의 호환성을 돕기 위해 만들어졌습니다.
- 현재는 대부분의 기능이 tokio 자체에 통합되어 있어 tokio_futures는 더 이상 필수적이지 않습니다.

## 🔍 주요 기능들 (과거 기준)
tokio_futures는 다음과 같은 기능들을 제공했습니다:
| 기능       | 설명                                                                 |
|------------|----------------------------------------------------------------------|
| FutureExt  | Future에 다양한 확장 메서드를 제공하는 trait. 예: map, then, boxed 등 |
| Stream     | 비동기적으로 여러 값을 순차적으로 생성하는 타입                      |
| Sink       | 비동기적으로 값을 소비하는 타입. Stream과 반대 개념                   |
| compat     | std::future와 futures 간의 호환성을 위한 어댑터                       |



## 🧪 예시 코드
```rust
use tokio::runtime::Runtime;
use tokio::task;
use futures::future;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let fut1 = async { 1 };
        let fut2 = async { 2 };
        let result = future::join(fut1, fut2).await;
        println!("Result: {:?}", result);
    });
}
```

## 두 개의 Future를 동시에 실행하고 결과를 기다리는 방식입니다.

### 🧩 최신 대안
- 대부분의 경우 tokio와 futures 크레이트만으로 충분합니다.
- tokio::task::JoinSet, tokio::select!, tokio::spawn_blocking 등 고급 기능들이 tokio에 직접 포함되어 있어 더 강력하고 안전한 비동기 프로그래밍이 가능합니다.

```rust
use tokio::io::{stdin, stdout};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use futures::io::copy;

#[tokio::main]
async fn main() {
    let mut input = stdin().compat();
    let mut output = stdout().compat_write();

    copy(&mut input, &mut output).await.unwrap();
}
```

이 코드는 **Tokio 기반의 비동기 입출력(IO)**을 사용해서 **표준 입력(stdin)**으로부터 데이터를 읽고, **표준 출력(stdout)**으로 그대로 복사하는 프로그램입니다.  
즉, 사용자가 입력한 내용을 그대로 출력하는 에코 프로그램이에요.  
하지만 중요한 포인트는, 이 코드가 Tokio와 futures 간의 호환성 문제를 해결하기 위해 compat 어댑터를 사용한다는 점입니다.

## 🧠 핵심 목적

- tokio::io::stdin()과 tokio::io::stdout()은 Tokio의 비동기 IO 타입입니다.
- futures::io::copy()는 futures 크레이트의 함수로, AsyncRead와 AsyncWrite 트레잇을 구현한 타입을 요구합니다.
- 그런데 tokio의 IO 타입은 futures의 트레잇을 직접 구현하지 않기 때문에 호환되지 않습니다.
- 그래서 compat()과 compat_write()를 사용해 Tokio 타입을 futures 타입처럼 보이게 변환합니다.

```
$ cargo run
hello world
hello world
```
---

## compat
compat()의 진짜 가치는 서로 다른 비동기 생태계를 연결할 때 드러납니다.  

## 🚧 실전에서 꼭 필요한 시나리오들
### 1. 서드파티 라이브러리와의 통합
예를 들어, async-compression, async-tungstenite, hyper, reqwest 같은 라이브러리 중 일부는 futures::io 기반으로 만들어졌습니다.  
그런데 내 프로젝트는 tokio 기반이라면?
#### 🔧 compat()을 사용하면 tokio의 TcpStream을 futures::AsyncRead로 변환해서 그대로 사용할 수 있습니다.

```rust
use tokio::net::TcpListener;
use tokio_util::compat::TokioAsyncReadCompatExt;
use futures::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let mut compat_stream = stream.compat(); // 핵심!
    let mut buffer = [0; 20];
    let n = compat_stream.read(&mut buffer).await.unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
}
```

이런 식으로 서드파티 라이브러리와의 경계에서 호환성을 확보할 수 있어요.

### 2. 레거시 코드 마이그레이션
기존 프로젝트가 futures 0.1 또는 tokio 0.1 기반이라면, 최신 async/await로 마이그레이션할 때 compat은 필수 도구입니다.
예전 코드를 전부 뜯어고치지 않고, 점진적으로 최신화할 수 있게 해줍니다.


### 3. 혼합 런타임 환경
어떤 라이브러리는 tokio를 요구하고, 다른 라이브러리는 async-std나 futures를 요구할 때, compat을 통해 중간 브릿지 역할을 할 수 있습니다.  
이건 특히 복잡한 네트워크 서버나 데이터 파이프라인에서 유용하죠.

## 🧠 요약하면
compat()은 단순한 입출력 변환이 아니라, Rust 비동기 생태계의 연결고리입니다.
- 라이브러리 간 호환성 확보
- 레거시 코드 점진적 업그레이드
- 복잡한 시스템에서 런타임 혼합 대응
이런 상황에서는 없으면 개발이 막히고, 있으면 길이 열리는 그런 도구예요.


이 코드는 **비동기적으로 연결을 수락(accept)**하는 것이지, 데이터를 "다 읽을 때까지 기다리는" 건 아닙니다.

### 🧠 listener.accept().await의 의미
```rust
let (stream, _) = listener.accept().await.unwrap();
```

- 이 줄은 클라이언트가 연결을 시도할 때까지 기다리는 작업입니다.
- 즉, TcpListener는 새로운 연결 요청이 들어올 때까지 비동기적으로 대기하고,
- 연결이 들어오면 TcpStream을 반환해서 그 스트림을 통해 데이터를 주고받을 수 있게 됩니다.
#### ✅ 이 시점에서는 연결만 수락한 상태이고, 데이터를 읽거나 쓰는 작업은 아직 시작되지 않았어요.


### 📥 그럼 "일부를 받으면서 기다린다"는 건 언제일까?
그건 stream.read() 또는 stream.read_exact() 같은 메서드를 사용할 때입니다.  
예를 들어:
```rust
use tokio::io::AsyncReadExt;

let mut buffer = [0u8; 1024];
let n = stream.read(&mut buffer).await.unwrap();
```

- 이 코드는 클라이언트가 데이터를 보내기 전까지 기다립니다.
- 하지만 버퍼 크기만큼 다 채울 때까지 기다리는 건 아니고, 일부만 와도 read()는 반환될 수 있어요.
- 즉, 부분 수신이 가능하고, 반복적으로 read()를 호출해서 전체 메시지를 수신하는 방식이 일반적입니다.

## 🔄 요약

| 메서드         | 설명                                                                 |
|----------------|----------------------------------------------------------------------|
| accept().await | 클라이언트의 연결 요청을 비동기적으로 기다림. 연결이 오면 TcpStream 반환 |
| read().await   | 연결된 스트림에서 데이터를 일부라도 수신하면 즉시 반환. 전체 수신 아님     |



데이터를 다 읽으려면 반복적으로 읽기를 시도하면서 버퍼에 계속 쌓아야 합니다.  
일반적으로는 Vec<u8> 같은 동적 버퍼를 사용해서 데이터를 누적. 

## 📦 전체 데이터를 읽는 예제 (TcpStream 기반)
```rust
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("서버가 127.0.0.1:8080에서 대기 중...");

    let (mut stream, _) = listener.accept().await?;
    println!("클라이언트 연결됨!");

    let mut buffer = Vec::new(); // 데이터를 누적할 버퍼
    let mut temp = [0u8; 1024];  // 임시 읽기 버퍼

    loop {
        let n = stream.read(&mut temp).await?;
        if n == 0 {
            println!("클라이언트가 연결을 종료했습니다.");
            break;
        }
        buffer.extend_from_slice(&temp[..n]); // 읽은 데이터를 누적
        println!("{} 바이트 수신됨", n);
    }

    println!("전체 수신 데이터: {:?}", String::from_utf8_lossy(&buffer));
    Ok(())
}
```


### 🔍 설명
- temp는 고정 크기의 임시 버퍼입니다. 한 번에 최대 1024바이트를 읽습니다.
- read()는 일부만 읽고 반환될 수 있으므로, 반복적으로 호출해야 전체 데이터를 받을 수 있습니다.
- buffer.extend_from_slice()를 통해 데이터를 계속 누적합니다.
- read()가 0을 반환하면, 상대방이 연결을 종료했다는 뜻입니다.

### 🧠 실전 팁
- 프로토콜 기반 처리: 만약 메시지 길이가 정해져 있거나, 헤더에 길이 정보가 있다면 그걸 먼저 읽고 정확히 그 길이만큼 읽는 방식이 더 안전합니다.
- 타임아웃 처리: tokio::time::timeout()을 사용해서 너무 오래 기다리지 않도록 설정할 수 있음.
- 스트림 파싱: tokio_util::codec을 사용하면 TCP 스트림을 프레임 단위로 파싱할 수 있어서 더 구조적인 처리도 가능합니다.

비동기적으로 데이터를 읽을 때는 타임아웃 처리와 예상치 못한 연결 종료에 대한 에러 처리를 반드시 고려해야 합니다.  
아래에 실전적인 예제와 함께 설명.

## ⏱ 타임아웃 처리: tokio::time::timeout
timeout()은 특정 작업이 정해진 시간 안에 완료되지 않으면 에러를 반환합니다.
```rust
use tokio::time::{timeout, Duration};
use tokio::io::AsyncReadExt;

let result = timeout(Duration::from_secs(5), stream.read(&mut buffer)).await;
match result {
    Ok(Ok(n)) => {
        if n == 0 {
            println!("클라이언트가 연결을 종료했습니다.");
        } else {
            println!("{} 바이트 수신됨", n);
        }
    }
    Ok(Err(e)) => {
        println!("읽기 중 에러 발생: {}", e);
    }
    Err(_) => {
        println!("⏰ 타임아웃 발생: 5초 내에 데이터가 도착하지 않았습니다.");
    }
}
```

### 🔍 설명
- timeout()은 Result<Result<T, E>, Elapsed>를 반환합니다.
- Ok(Ok(n)): 정상적으로 데이터를 읽음
- Ok(Err(e)): 읽기 중 I/O 에러 발생
- Err(_): 타임아웃 발생

### 🔌 연결 종료 감지
TCP 연결이 끊기면 read()는 0 바이트를 반환합니다. 이건 "EOF"를 의미하며, 상대방이 shutdown() 또는 close()를 호출했을 때 발생합니다.
if n == 0 {
    println!("연결 종료 감지됨");
    break;
}

### 🧠 실전 팁
- 타임아웃은 반드시 설정하세요: 네트워크는 예측 불가능하므로, 무한 대기 상태를 방지해야 합니다.
- 에러 종류를 구분하세요: std::io::ErrorKind::WouldBlock, ConnectionReset, TimedOut 등 다양한 에러가 발생할 수 있습니다.
- 로그를 남기세요: 에러 발생 시 로그를 남기면 디버깅이 훨씬 쉬워집니다.
- 재시도 전략: 타임아웃이나 일시적 에러가 발생했을 때 재시도할지 여부를 결정하는 로직도 중요합니다.

## ✅ 전체 흐름 요약 (ASCII 표)
| 상황             | 처리 방식                                      |
|------------------|------------------------------------------------|
| 데이터 수신 지연 | timeout()으로 제한 시간 설정                   |
| 연결 종료        | read()가 0 반환 → 연결 종료로 간주             |
| 읽기 에러        | read()의 Err(e) → 로그 남기고 적절히 대응       |
| 타임아웃 발생    | timeout()의 Err(_) → 사용자 알림 or 재시도      |


read()가 0을 반환하는 경우는 에러가 아니라 정상적인 종료 신호, 즉 **EOF (End of File)**를 의미합니다.   
이건 네트워크 스트림에서도 마찬가지.

## ✅ n == 0의 의미
```rust
let n = stream.read(&mut buffer).await?;
```

- n > 0: 일부 데이터를 정상적으로 읽은 경우
- n == 0: 상대방이 연결을 정상적으로 종료했거나, 더 이상 읽을 데이터가 없음
- Err(e): 실제 에러 발생 (예: 연결 끊김, 타임아웃, I/O 오류 등)
📌 n == 0은 에러가 아니라 정상적인 EOF 처리입니다. 따라서 ? 연산자로 에러로 간주되지 않고, 직접 if n == 0으로 체크해야 합니다.


## 🔍 왜 헷갈릴 수 있을까?
Tokio의 AsyncReadExt::read()는 std::io::Read와 동일하게 usize를 반환합니다.  
이건 C의 read() 함수에서 유래된 방식인데, EOF를 명시적으로 표현하지 않고 0으로 표현하기 때문에 프로그래머가 직접 체크.
일부 개발자들은 이 방식이 null 포인터 문제처럼 실수하기 쉽다고 지적하며, Option<NonZeroUsize> 같은 더 명시적인 타입을 제안하기도 했습니다.

## 🧠 실전 처리 방식
```rust
match stream.read(&mut buffer).await {
    Ok(0) => {
        println!("EOF: 연결 종료 감지됨");
        break;
    }
    Ok(n) => {
        println!("{} 바이트 수신됨", n);
        // 데이터 처리 로직
    }
    Err(e) => {
        println!("에러 발생: {}", e);
        break;
    }
}
```

이렇게 하면 EOF와 에러를 명확히 구분할 수 있어요.

## ✨ 요약
| 반환값 | 의미                          | 처리 방식               |
|--------|-------------------------------|--------------------------|
| n > 0  | 일부 데이터 수신               | 계속 읽기                |
| n == 0 | EOF (연결 종료 또는 데이터 없음) | 루프 종료 or 연결 닫기   |
| Err(e) | 실제 에러 발생                | 로그 남기고 종료 or 재시도 |

---



