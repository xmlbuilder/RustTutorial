# RwLock / RwLockWriteGuard
Rust의 동시성에서 자주 쓰이는 RwLock과 RwLockWriteGuard에 대해 자세히 설명.  
이 둘은 읽기/쓰기 락을 안전하게 관리하기 위한 도구입니다.

## 🔐 RwLock<T>란?
RwLock<T>는 **읽기-쓰기 락(Read-Write Lock)**을 제공하는 구조체입니다. 내부에 T 타입의 데이터를 가지고 있으며, 다음을 보장합니다:
- 여러 **읽기 접근(read)**은 동시에 허용됩니다.
- 단 하나의 **쓰기 접근(write)**만 허용됩니다. 쓰는 동안에는 읽기도 막힙니다.
```rust
use std::sync::RwLock;
let lock = RwLock::new(5); // 내부 값은 5
```


## ✍️ RwLockWriteGuard<'a, T>란?
RwLockWriteGuard는 RwLock에서 쓰기 락을 획득했을 때 반환되는 구조체입니다.  
이 구조체를 통해 내부 값을 가변 참조로 접근할 수 있어요.
```rust
let mut write_guard = lock.write().unwrap(); // RwLockWriteGuard 반환
*write_guard += 1; // 내부 값 수정


- write() 메서드는 RwLockWriteGuard<T>를 반환합니다.
- write_guard가 스코프를 벗어나면 자동으로 락이 해제됩니다.

🔗 둘을 어떻게 연결해서 사용하는가?
✅ 기본 사용 흐름
use std::sync::RwLock;

fn main() {
    let data = RwLock::new(vec![1, 2, 3]);

    {
        let mut write_guard = data.write().unwrap(); // 쓰기 락 획득
        write_guard.push(4); // 데이터 수정
    } // write_guard가 drop되면서 락 해제

    {
        let read_guard = data.read().unwrap(); // 읽기 락 획득
        println!("{:?}", *read_guard); // [1, 2, 3, 4]
    }
}


🔄 흐름 요약
| 호출 대상     | 반환 타입           |
|---------------|---------------------|
| RwLock<T>     |                     |
| .write()      | RwLockWriteGuard    |
| .read()       | RwLockReadGuard     |



🧠 주의할 점
- RwLock은 멀티스레드 환경에서 안전하게 공유하려면 Arc<RwLock<T>>로 감싸야 합니다.
- write()와 read()는 Result를 반환하므로 unwrap() 또는 ?로 처리해야 합니다.
- RwLockWriteGuard는 락이 걸린 동안 다른 스레드 접근을 막기 때문에 오래 잡고 있으면 성능에 영향을 줄 수 있어요.
