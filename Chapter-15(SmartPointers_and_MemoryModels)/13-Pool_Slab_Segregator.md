# Pool / Slab/ Segregator
Rust에서 풀(Pool), 슬랩(Slab), 세그리게이터(Segregator)는 메모리 할당과 관리에 관련된 고급 개념으로,  
성능과 안전성을 높이기 위한 전략입니다.  
이들은 특히 시스템 프로그래밍이나 고성능 서버 개발에서 중요한 역할을 하며,  
Rust의 메모리 안전성과 제로 비용 추상화를 활용해 효율적인 메모리 관리를 가능하게 합니다.  

## 🧠 개념 설명

### 1. 풀(Pool)
- 메모리 풀은 미리 할당된 메모리 블록을 저장해두고 필요할 때 재사용하는 방식입니다.
- Rust에서는 object_pool, typed-arena, pool 같은 크레이트를 통해 구현됩니다.
- 장점: 할당/해제 비용 감소, 캐시 친화적, 메모리 단편화 방지.
- 사용 예시: 게임 엔진, 네트워크 서버 등에서 객체를 자주 생성/삭제할 때.
- 
### 2. 슬랩(Slab)
- 슬랩은 고정 크기의 객체를 저장하는 배열 기반 구조입니다.
- Rust에서는 slab 크레이트가 대표적이며, 인덱스를 통해 객체에 접근합니다.
- 특징:
- 객체가 고정된 크기일 때 효율적.
- 인덱스 기반 접근으로 안전하고 빠름.
- 사용 예시: TCP 연결 관리, 세션 테이블 등에서 자주 사용.
- 
### 3. 세그리게이터(Segregator)
- 세그리게이터는 서로 다른 크기의 메모리 요청을 분리해 처리하는 전략입니다.
- Rust의 moka, heapless, allocator-api2 같은 라이브러리에서 사용됩니다.
- 원리:
- 작은 객체는 작은 블록에서, 큰 객체는 큰 블록에서 할당.
- 내부적으로 여러 풀을 관리해 최적화.
- 장점: 메모리 낭비 최소화, 할당 속도 향상.

## 🛠️ Rust에서의 활용
| 전략        | Rust 크레이트 예시               | 주요 용도                          |
|-------------|----------------------------------|------------------------------------|
| Pool        | object_pool, typed-arena         | 반복적인 객체 생성/삭제            |
| Slab        | slab                             | 고정 크기 객체 관리, 인덱스 접근   |
| Segregator  | allocator-api2, heapless         | 다양한 크기 객체의 효율적 할당     |


## 🏊 1. Pool 예제 (object_pool 사용)
```rust
use object_pool::Pool;

#[derive(Default)]
struct MyObject {
    data: u32,
}

fn main() {
    let pool = Pool::new( || MyObject::default() );
    // 객체를 풀에서 가져오기
    let mut obj = pool.pull();
    obj.data = 42;
    println!("사용된 객체의 값: {}", obj.data);
    // obj는 drop되면 자동으로 풀로 반환됨
}
```

- 설명: Pool은 객체를 미리 만들어두고 재사용합니다. pull()로 가져오고, 사용 후 자동 반환됩니다.
- 장점: 할당 비용 절감, 성능 향상.

## 🧱 2. Slab 예제 (slab 사용)
```rust
use slab::Slab;

fn main() {
    let mut slab = Slab::new();

    let key = slab.insert("Hello, Slab!");
    println!("슬랩에서 가져온 값: {}", slab[key]);

    slab.remove(key); // 제거
}
```

- 설명: Slab은 고정 크기 슬롯에 데이터를 저장하고 인덱스로 접근합니다.
- 장점: 빠른 접근, 메모리 단편화 없음.

## 🧩 3. Segregator 예제 (allocator-api2 기반)
```rust
use allocator_api2::alloc::{Global, Allocator, Layout};

fn main() {
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ptr = Global.allocate(layout).unwrap().as_ptr();

    unsafe {
        std::ptr::write_bytes(ptr, 0xAB, 64); // 메모리 초기화
    }
    println!("64바이트 메모리 블록을 할당하고 초기화했습니다.");
    unsafe {
        Global.deallocate(ptr, layout); // 해제
    }
}
```

- 설명: 다양한 크기의 메모리를 직접 할당/해제하는 방식입니다.
- 장점: 세밀한 제어, 커스텀 할당 전략 가능.

각 방식은 상황에 따라 적절히 선택해야 함. 예를 들어:
- Pool: 자주 생성/삭제되는 객체
- Slab: 고정 크기 구조체를 인덱스로 관리
- Segregator: 다양한 크기의 메모리를 직접 다룰 때

---

# 전환 테스트

## 🧠 기존 구조: Vec<Arc<T>>
- Vec: 순차적 저장
- Arc: 여러 스레드에서 안전하게 공유
- 용도: 다수의 공유 객체를 저장하고 반복적으로 접근하거나 수정

## 🏊 1. Pool로 대체
### ✅ 목적
- 객체를 재사용하고 할당 비용을 줄이기 위함
- Arc 없이도 단일 스레드 내에서 효율적
### 🔧 대체 방법
```rust
use object_pool::Pool;

#[derive(Default)]
struct MyData {
    value: u32,
}

fn main() {
    let pool = Pool::new(|| MyData::default());

    let mut handles = Vec::new();

    for _ in 0..10 {
        let obj = pool.pull(); // Arc 없이 직접 객체를 가져옴
        handles.push(obj);
    }

    for obj in handles {
        println!("value = {}", obj.value);
    }
}
```            

- 주의: Arc가 없으므로 멀티스레드 공유가 필요하면 Arc<Pool<T>>로 감싸야 함

## 🧱 2. Slab으로 대체
### ✅ 목적
- 고정 크기 객체를 인덱스로 관리
- 빠른 접근과 제거 가능
### 🔧 대체 방법
```rust
use slab::Slab;
use std::sync::Arc;

#[derive(Default)]
struct MyData {
    value: u32,
}

fn main() {
    let mut slab: Slab<Arc<MyData>> = Slab::new();

    let key = slab.insert(Arc::new(MyData { value: 42 }));
    let another = slab.insert(Arc::new(MyData { value: 99 }));

    println!("key1 = {}", slab[key].value);
    println!("key2 = {}", slab[another].value);

    slab.remove(key);
}
```

- 장점: Vec<Arc<T>>처럼 인덱스로 접근 가능하면서 제거도 효율적
- 슬랩은 인덱스 기반이므로 usize 키를 통해 객체를 추적 가능

## 🧩 3. Segregator로 대체
### ✅ 목적
- 다양한 크기의 객체를 효율적으로 할당
- 커스텀 할당 전략이 필요한 경우
### 🔧 대체 방법
```rust
use allocator_api2::alloc::{Global, Allocator, Layout};
use std::ptr::NonNull;

fn main() {
    let layout = Layout::new::<[u32; 10]>(); // Vec<Arc<T>> 대신 고정 배열
    let ptr = Global.allocate(layout).unwrap();
    unsafe {
        let slice = ptr.as_ptr() as *mut [u32; 10];
        (*slice)[0] = 123;
        println!("첫 번째 값: {}", (*slice)[0]);
        Global.deallocate(ptr, layout);
    }
}
```       

- 주의: 저수준 메모리 제어가 필요하므로 안전성 확보가 중요
- 멀티스레드 공유는 직접 Arc로 감싸거나 별도 전략 필요

## 🧮 요약 비교
| 전략        | 대체 구조               | 공유 가능 | 인덱스 접근 | 안전성 | 특징                         |
|-------------|--------------------------|------------|--------------|--------|------------------------------|
| Pool        | Pool<T>, Arc<Pool<T>>    | ✅         | ❌           | ✅     | 객체 재사용, 빠른 할당/반환     |
| Slab        | Slab<Arc<T>>             | ✅         | ✅           | ✅     | 인덱스 기반 접근, 빠른 삭제     |
| Segregator  | allocator-api2           | ❌ (직접 구현 필요) | ❌           | ❌     | 저수준 제어, unsafe 필요       |


---

# 실전 예제

```rust
struct MyData<T> {
    data: Vec<Arc<T>>,
}
```

Pool, Slab, 또는 Segregator를 활용해서 Vec<Arc<T>> 대신 더 효율적인 메모리 관리 방식으로 바꿈.  
즉, MyData 내부의 data 필드를 대체하는 방식.

## 🔄 대체 전략별 구조
### 🏊 1. Pool로 대체
```rust
use object_pool::Pool;
use std::sync::Arc;

struct MyData<T> {
    pool: Pool<Arc<T>>,
}
impl<T: Default> MyData<T> {
    fn new() -> Self {
        Self {
            pool: Pool::new(|| Arc::new(T::default())),
        }
    }
    fn get(&self) -> Arc<T> {
        self.pool.pull().clone()
    }
}
```

- 핵심: Vec<Arc<T>> 대신 Pool<Arc<T>>를 사용해 객체를 재사용
- 장점: 할당 비용 감소, 캐시 친화적
- 단점: 순차적 접근은 직접 구현해야 함

### 🧱 2. Slab로 대체
```rust
use slab::Slab;
use std::sync::Arc;

struct MyData<T> {
    slab: Slab<Arc<T>>,
}

impl<T> MyData<T> {
    fn new() -> Self {
        Self {
            slab: Slab::new(),
        }
    }
    fn insert(&mut self, value: T) -> usize {
        self.slab.insert(Arc::new(value))
    }
    fn get(&self, key: usize) -> Option<&Arc<T>> {
        self.slab.get(key)
    }
}
```

- 핵심: Vec<Arc<T>> → Slab<Arc<T>>
- 장점: 인덱스 기반 접근, 빠른 삽입/삭제
- 단점: 순서 보장 없음

### 🧩 3. Segregator로 대체 (저수준 제어)
```rust
use allocator_api2::alloc::{Allocator, Global, Layout};
use std::ptr::NonNull;

struct MyData<T> {
    ptrs: Vec<NonNull<T>>,
    layout: Layout,
}

impl<T> MyData<T> {
    fn new() -> Self {
        let layout = Layout::new::<T>();
        Self {
            ptrs: Vec::new(),
            layout,
        }
    }

    fn allocate(&mut self, value: T) {
        let ptr = Global.allocate(self.layout).unwrap().cast::<T>();
        unsafe {
            ptr.as_ptr().write(value);
        }
        self.ptrs.push(ptr);
    }

    fn get(&self, index: usize) -> &T {
        unsafe { self.ptrs[index].as_ref() }
    }
}
```

- 핵심: 직접 메모리 할당으로 Vec<Arc<T>> 대체
- 장점: 세밀한 제어
- 단점: unsafe 사용, 수동 해제 필요

## 🧮 요약

| 전략        | 대체 구조             | 공유 가능 | 순차 접근 | 안전성 | 특징                     |
|-------------|------------------------|------------|--------------|--------|--------------------------|
| Pool        | Pool<Arc<T>>           | ✅         | ❌ (직접 구현) | ✅     | 객체 재사용, 빠른 할당     |
| Slab        | Slab<Arc<T>>           | ✅         | ❌ (인덱스 기반) | ✅     | 인덱스 접근, 빠른 삽입/삭제 |
| Segregator  | Vec<NonNull<T>>        | ❌         | ✅           | ❌     | 저수준 제어, unsafe 필요   |


추천:
- Slab<Arc<T>>는 가장 자연스럽고 안전한 대체입니다.
- Pool<Arc<T>>는 객체 재사용이 중요한 경우에 적합합니다.
- Segregator는 성능이 극도로 중요한 저수준 시스템에서만 추천.

---

## 🏊 1. Pool 예제 (순회, 수정, 삭제)
```rust
use object_pool::Pool;

#[derive(Default, Debug)]
struct MyData {
    value: u32,
}

fn main() {
    let pool = Pool::new(|| MyData::default());
    let mut items = Vec::new();

    // 삽입
    for i in 0..5 {
        let mut obj = pool.pull();
        obj.value = i;
        items.push(obj);
    }

    // 순회
    for item in &items {
        println!("value = {}", item.value);
    }

    // 수정
    for item in &mut items {
        item.value *= 10;
    }

    // 삭제 (drop 시 자동 반환)
    items.clear();
}
```

- ✅ 순회: for item in &items
- ✅ 수정: item.value = ...
- ✅ 삭제: clear()로 반환


## 🧱 2. Slab 예제 (순회, 수정, 삭제)
```rust
use slab::Slab;
use std::sync::Arc;

#[derive(Debug)]
struct MyData {
    value: u32,
}

fn main() {
    let mut slab: Slab<Arc<MyData>> = Slab::new();

    // 삽입
    for i in 0..5 {
        slab.insert(Arc::new(MyData { value: i }));
    }

    // 순회
    for (key, item) in slab.iter() {
        println!("key = {}, value = {}", key, item.value);
    }

    // 수정 (Arc는 불변이므로 새로 교체)
    for key in slab.iter().map(|(k, _)| k).collect::<Vec<_>>() {
        slab[key] = Arc::new(MyData { value: slab[key].value * 10 });
    }

    // 삭제
    slab.remove(2); // 특정 키 삭제
}
```

- ✅ 순회: slab.iter()
- ✅ 수정: slab[key] = Arc::new(...)
- ✅ 삭제: slab.remove(key)

## 🧩 3. Segregator 예제 (순회, 수정, 삭제)
```rust
use allocator_api2::alloc::{Allocator, Global, Layout};
use std::ptr::NonNull;

#[derive(Debug)]
struct MyData {
    value: u32,
}

fn main() {
    let layout = Layout::new::<MyData>();
    let mut ptrs: Vec<NonNull<MyData>> = Vec::new();

    // 삽입
    for i in 0..5 {
        let ptr = Global.allocate(layout).unwrap().cast::<MyData>();
        unsafe {
            ptr.as_ptr().write(MyData { value: i });
        }
        ptrs.push(ptr);
    }

    // 순회
    for ptr in &ptrs {
        unsafe {
            println!("value = {}", ptr.as_ref().value);
        }
    }

    // 수정
    for ptr in &ptrs {
        unsafe {
            let data = ptr.as_ptr();
            (*data).value *= 10;
        }
    }

    // 삭제
    for ptr in ptrs {
        unsafe {
            Global.deallocate(ptr.cast(), layout);
        }
    }
}
```

- ✅ 순회: unsafe { ptr.as_ref() }
- ✅ 수정: unsafe { (*ptr.as_ptr()).value = ... }
- ✅ 삭제: Global.deallocate(...)

---

# 구조체 안에서 데이터 관리 

아래는 각각의 구조체에 대해 순회, 수정, 삭제를 수행하는 예제입니다.  
모두 MyData<T> 형태를 기반으로 하며, T는 MyItem이라는 구조체로 가정합니다:
```rust
#[derive(Debug)]
struct MyItem {
    value: u32,
}
```

## 🏊 1. Pool<Arc<T>> 기반
```rust
use object_pool::Pool;
use std::sync::Arc;

#[derive(Debug)]
struct MyItem {
    value: u32,
}

struct MyData {
    pool: Pool<Arc<MyItem>>,
    items: Vec<Arc<MyItem>>, // 별도 저장용
}

impl MyData {
    fn new() -> Self {
        let pool = Pool::new(|| Arc::new(MyItem { value: 0 }));
        Self { pool, items: Vec::new() }
    }

    fn insert(&mut self, val: u32) {
        let mut item = self.pool.pull();
        Arc::get_mut(&mut item).unwrap().value = val;
        self.items.push(item.clone());
    }

    fn iterate(&self) {
        for item in &self.items {
            println!("value = {}", item.value);
        }
    }

    fn modify(&mut self) {
        for item in &mut self.items {
            if let Some(mut_ref) = Arc::get_mut(item) {
                mut_ref.value *= 10;
            }
        }
    }

    fn clear(&mut self) {
        self.items.clear(); // drop → pool로 반환
    }
}
```


## 🧱 2. Slab<Arc<T>> 기반
```rust
use slab::Slab;
use std::sync::Arc;

#[derive(Debug)]
struct MyItem {
    value: u32,
}

struct MyData {
    slab: Slab<Arc<MyItem>>,
}

impl MyData {
    fn new() -> Self {
        Self { slab: Slab::new() }
    }

    fn insert(&mut self, val: u32) -> usize {
        self.slab.insert(Arc::new(MyItem { value: val }))
    }

    fn iterate(&self) {
        for (key, item) in self.slab.iter() {
            println!("key = {}, value = {}", key, item.value);
        }
    }

    fn modify(&mut self) {
        let keys: Vec<usize> = self.slab.iter().map(|(k, _)| k).collect();
        for key in keys {
            let old = &self.slab[key];
            self.slab[key] = Arc::new(MyItem { value: old.value * 10 });
        }
    }

    fn remove(&mut self, key: usize) {
        self.slab.remove(key);
    }
}
```


## 🧩 3. Vec<NonNull<T>> + Layout 기반 (Segregator 스타일)
```rust
use allocator_api2::alloc::{Allocator, Global, Layout};
use std::ptr::NonNull;

#[derive(Debug)]
struct MyItem {
    value: u32,
}

struct MyData {
    ptrs: Vec<NonNull<MyItem>>,
    layout: Layout,
}

impl MyData {
    fn new() -> Self {
        let layout = Layout::new::<MyItem>();
        Self { ptrs: Vec::new(), layout }
    }

    fn insert(&mut self, val: u32) {
        let ptr = Global.allocate(self.layout).unwrap().cast::<MyItem>();
        unsafe {
            ptr.as_ptr().write(MyItem { value: val });
        }
        self.ptrs.push(ptr);
    }

    fn iterate(&self) {
        for ptr in &self.ptrs {
            unsafe {
                println!("value = {}", ptr.as_ref().value);
            }
        }
    }

    fn modify(&mut self) {
        for ptr in &self.ptrs {
            unsafe {
                (*ptr.as_ptr()).value *= 10;
            }
        }
    }

    fn clear(&mut self) {
        for ptr in self.ptrs.drain(..) {
            unsafe {
                Global.deallocate(ptr.cast(), self.layout);
            }
        }
    }
}
```

## 🧮 비교 요약
| 구조체 타입           | 순회 방식                  | 수정 방식                  | 삭제 방식                  | 안전성 | 특징                     |
|------------------------|-----------------------------|-----------------------------|-----------------------------|--------|--------------------------|
| Pool<Arc<T>>           | for item in &Vec<_>         | Arc::get_mut() 또는 교체     | Vec::clear()                | ✅     | 객체 재사용, 자동 반환     |
| Slab<Arc<T>>           | slab.iter()                 | Arc::new(...)로 교체         | slab.remove(key)            | ✅     | 인덱스 기반 관리, 빠른 삭제 |
| Vec<NonNull<T>>        | unsafe { ptr.as_ref() }     | unsafe { *ptr = ... }       | Global.deallocate(...)      | ❌     | 저수준 제어, 수동 해제 필요 |

---

## 🔍 Vec vs Slab 핵심 차이
| 항목             | Vec<T>                        | Slab<T>                         |
|------------------|-------------------------------|----------------------------------|
| 삭제 방식         | remove(i), swap_remove        | remove(key)                      |
| 인덱스 유지       | ❌ 삭제 시 인덱스 밀림 가능       | ✅ 삭제해도 인덱스 재사용 가능     |
| 삽입 성능         | 빠름 (push)                   | 빠름 (insert)                    |
| 삭제 성능         | 느림 (retain, swap_remove 등) | 빠름 (O(1) remove)               |
| 순서 보장         | ✅ 삽입 순서 유지               | ❌ 순서 보장 안 됨                |
| 메모리 재사용     | ❌ 삭제된 공간 재사용 어려움     | ✅ 슬롯 재사용으로 메모리 효율적   |
| 인덱스 접근       | ✅ vec[i]                      | ✅ slab[key]                      |
| 용도             | 순차적 데이터 처리에 적합       | ID 기반 객체 관리에 적합          |

## 🧠 Slab의 주요 장점
### 1. 인덱스 안정성
- Vec은 요소를 제거하면 인덱스가 바뀌거나 밀립니다.
- Slab은 제거된 슬롯을 내부적으로 재사용하므로 인덱스가 고정되고 재사용 가능합니다.
- 예: TCP 연결 관리, 세션 테이블 등에서 연결 ID를 안정적으로 유지할 수 있음.
 
### 2. 빠른 삭제
- Vec은 swap_remove()나 retain()을 써야 하므로 비용이 발생합니다.
- Slab은 remove(key)로 O(1) 삭제가 가능.

### 3. 메모리 재사용
- Slab은 삭제된 슬롯을 자동으로 재활용하므로 메모리 단편화가 적고 할당 비용이 낮음.
  
### 4. 인덱스 기반 접근
- Slab은 usize 키를 통해 객체에 접근하므로 빠르고 안전한 인덱스 관리가 가능합니다.

## 🧪 언제 Slab이 유리한가?
- 많은 객체를 생성/삭제하며 인덱스를 유지해야 할 때
- ID 기반으로 객체를 추적해야 할 때
- 순서보다 빠른 접근성과 삭제가 중요할 때

## 🧩 예시: TCP 연결 관리
```rust
use slab::Slab;

struct Connection {
    id: usize,
    data: String,
}

fn main() {
    let mut connections: Slab<Connection> = Slab::new();
    let conn_id = connections.insert(Connection {
        id: 0,
        data: "Hello".into(),
    });
    connections.remove(conn_id); // 삭제 후 슬롯 재사용 가능
}
```


## ✅ 요약
- Vec은 순서 중심, Slab은 슬롯 기반 인덱스 관리에 특화
- Slab은 삭제와 재사용이 빠르고 안전하며, ID 기반 시스템에 적합

---

# Pool 활용

아래는 Pool<T>를 제대로 활용하는 구조를 보여주는 예제입니다.  
핵심은 Arc<T> 같은 공유 포인터 없이, 값 자체를 재사용하는 방식.  
이 구조는 짧은 생명주기의 객체를 반복적으로 생성/삭제하는 상황에서 특히 유용합니다.  

## 🧱 예제: Pool<T>를 활용한 메시지 처리기
```rust
use object_pool::Pool;

#[derive(Debug, Default)]
struct Message {
    id: u32,
    payload: String,
}

struct MessageHandler {
    pool: Pool<Message>,
    active: Vec<Message>,
}

impl MessageHandler {
    fn new() -> Self {
        let pool = Pool::new(|| Message::default());
        Self {
            pool,
            active: Vec::new(),
        }
    }

    fn receive(&mut self, id: u32, payload: &str) {
        let mut msg = self.pool.pull();
        msg.id = id;
        msg.payload.clear();
        msg.payload.push_str(payload);
        self.active.push(msg);
    }

    fn process(&mut self) {
        for msg in &self.active {
            println!("Processing message {}: {}", msg.id, msg.payload);
        }
    }

    fn clear(&mut self) {
        self.active.clear(); // 메시지 객체는 drop되며 pool로 반환됨
    }
}
```

## 🔍 핵심 포인트
| 항목               | 설명                                      |
|--------------------|-------------------------------------------|
| Pool<Message>      | 객체를 미리 생성해 pull/push로 재사용         |
| msg.payload.clear()| 기존 객체의 필드를 초기화해 재사용 가능하게 만듦 |
| active.clear()     | drop 시 자동으로 pool에 반환됨               |
| Arc<T> 없음        | 참조 카운트 관리 불필요, 값 자체를 재사용       |


## 🧪 사용 예
```rust
fn main() {
    let mut handler = MessageHandler::new();

    handler.receive(1, "Hello");
    handler.receive(2, "World");
    handler.process();

    handler.clear(); // 메시지 객체는 pool로 반환됨

    handler.receive(3, "Reused");
    handler.process();
}
```

## 출력:
```
Processing message 1: Hello
Processing message 2: World
Processing message 3: Reused
```


## 💡 언제 유용한가?
- 게임 엔진에서 Bullet, Particle, Entity 같은 객체 재사용
- 네트워크 서버에서 Request, Response 객체 반복 처리
- 실시간 시스템에서 할당/해제 비용을 줄이고 싶을 때


## ⚡ 메모리 재할당 없이 객체 재사용
```rust
let mut msg = self.pool.pull();
msg.id = id;
msg.payload.clear();
msg.payload.push_str(payload);
```

## 🔍 왜 효율적인가?
- msg는 이미 할당된 Message 객체를 풀에서 가져온 것이기 때문에,
- payload.clear()는 기존 String의 버퍼를 유지한 채 내용만 비움
- push_str()는 기존 버퍼가 충분하면 재할당 없이 문자열을 추가
- 결과적으로 힙 메모리 재할당이 거의 발생하지 않음

## 🧠 이점 요약
| 항목               | 설명                                      |
|--------------------|-------------------------------------------|
| Pool               | 객체를 재사용하여 할당/해제 비용 절감         |
| String / Vec       | 내부 버퍼를 clear 후 재사용 가능, 재할당 없음 |
| new / drop         | 반복 시 비용 발생, 캐시 미스 가능성 있음      |


## 🧪 비교: 일반적인 방식 vs Pool 방식
| 방식            | 객체 생성 방식         | 메모리 재사용 | 성능 특성             | 추천 용도                      |
|-----------------|------------------------|----------------|------------------------|-------------------------------|
| Arc::new(...)   | 매번 새 객체 생성       | ❌              | 느림, 힙 재할당 발생     | 공유 참조가 필요한 구조         |
| Pool.pull()     | 기존 객체를 pull로 재사용 | ✅              | 빠름, 캐시 친화적        | 반복 생성/삭제가 많은 시스템     |



## 🔁 Pool의 순환 구조
```rust
let mut item = pool.pull();
// 사용 후 drop → 내부적으로 pool에 반환됨
```

## 🔍 동작 흐름
- 초기 생성: Pool::new(|| T::default())로 객체 생성 전략 정의
- pull() 호출: pool에서 사용 가능한 객체를 꺼냄 (없으면 새로 생성)
- 사용: 필요한 필드만 수정해서 사용
- drop(): 사용이 끝나면 자동으로 pool에 반환됨
- 다음 pull(): 이전에 반환된 객체를 재사용

## 🧠 핵심 이점
| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| new                | 객체를 매번 생성하지 않고 pull로 재사용 → 할당/해제 비용 절감             |
| String / Vec       | 내부 버퍼를 clear 후 재사용 → 힙 메모리 재할당 없음, 성능 향상             |
| drop → 반환        | 사용 후 drop되면 pool로 자동 반환 → 객체 순환 구조로 메모리 효율적 관리     |
| 캐시 친화적 구조     | 동일한 메모리 위치를 반복 사용 → CPU 캐시 활용도 높아짐                    |


## 🧪 예시: 3개 pull → 3개 재사용
```rust
let mut pool = Pool::new(|| Message::default());

let a = pool.pull();
let b = pool.pull();
let c = pool.pull();

// 사용 후 drop
drop(a);
drop(b);
drop(c);

// 이후 pull은 a, b, c를 재사용
let reused = pool.pull(); // a 또는 b 또는 c 중 하나
```

## 💡 요약
Pool은 "객체를 해제하지 않고, pull한 만큼 갖고 있다가 그 갯수만큼 돌려 쓰는" 구조.  
이 덕분에 할당 없이 빠르게 객체를 순환할 수 있고, GC 없는 Rust 환경에서 매우 효율적.

---

# 새로운 데이터를 만들고 돌려 구조에 사용
Pool에서 꺼낸 객체를 사용해 완성된 데이터를 만들고, 그 후 Pool에 객체를 돌려주는 구조는 고성능 시스템에서 자주 쓰이는 패턴.  
아래에 그 흐름과 구현 방법을 정리.

## 🔁 목적: Pool 객체를 가공 → 새 객체로 복사 → Pool에 반환
### 예시 상황
- Pool<Message>에서 Message를 꺼냄
- Message를 가공해서 ProcessedMessage로 변환
- Message는 더 이상 필요 없으므로 Pool에 반환

### 🧪 예제 코드
```rust
use object_pool::Pool;

#[derive(Debug, Default)]
struct Message {
    id: u32,
    payload: String,
}

#[derive(Debug)]
struct ProcessedMessage {
    id: u32,
    content: String,
}

struct Processor {
    pool: Pool<Message>,
}

impl Processor {
    fn new() -> Self {
        Self {
            pool: Pool::new(|| Message::default()),
        }
    }

    fn process(&mut self, id: u32, raw: &str) -> ProcessedMessage {
        // 1. Pool에서 객체 pull
        let mut msg = self.pool.pull();

        // 2. 데이터 가공
        msg.id = id;
        msg.payload.clear();
        msg.payload.push_str(raw);

        // 3. 완성된 데이터를 복사
        let result = ProcessedMessage {
            id: msg.id,
            content: msg.payload.clone(), // 깊은 복사
        };

        // 4. msg는 drop되며 Pool로 자동 반환
        result
    }
}
```


## 🔍 핵심 포인트
| 단계     | 설명                                                                 |
|----------|----------------------------------------------------------------------|
| pull()   | Pool에서 기존 객체를 꺼냄 → 새로 할당하지 않고 재사용 가능               |
| 필드 수정| 기존 객체의 필드를 초기화하거나 덮어써서 원하는 상태로 가공               |
| clone()  | 필요한 데이터만 복사해서 새 객체로 생성 → Pool 객체와 분리됨              |
| drop     | 사용이 끝난 Pool 객체는 drop 시 자동으로 Pool에 반환됨 → 다음 pull에서 재사용 |


## 🧠 이점
- 할당 최소화: Message는 재사용되고, ProcessedMessage만 새로 생성됨
- 메모리 효율: String이나 Vec 같은 필드는 내부 버퍼를 재사용
- 성능 향상: 반복적인 작업에서 힙 할당/해제 비용 제거

## 💡 팁
- clone()은 필요한 필드만 복사하도록 최소화하는 게 좋음
- ProcessedMessage가 Copy 가능한 구조라면 더 빠르게 처리 가능
- 이 패턴은 파싱, 디코딩, 렌더링, 네트워크 처리 등에서 자주 쓰임

---

## 🔍 상황 비교
### ✅ active에 push 하지 않은 경우
```rust
let mut msg = self.pool.pull();
// msg는 지역 변수로만 존재 → 참조 카운트 1
// 함수 끝에서 drop → Pool에 반환됨
```

### ❌ self.active.push(msg) 한 경우
```rust
self.active.push(msg);
// msg는 active에 들어감 → 참조 카운트 증가
// drop되어도 Pool에 반환되지 않음 (참조 남아 있음)
```

### ✅ self.active.push(msg.clone()) 한 경우
```rust
self.active.push(msg.clone());
// clone된 객체는 Pool과 무관
// msg는 지역 변수로만 존재 → drop 시 Pool에 반환됨
```

### 🧠 요약
| 객체          | 참조 상태           | Pool 반환 시점         | 설명                                      |
|---------------|---------------------|-------------------------|-------------------------------------------|
| msg           | 지역 변수만 존재     | 함수 끝에서 drop됨      | 참조 카운트 1 → drop 시 Pool에 자동 반환     |
| msg (push됨)  | 외부에 참조 남음     | 반환되지 않음           | 참조 카운트 >1 → Pool에 반환되지 않음       |
| msg.clone()   | 새 객체 생성         | Pool과 무관              | 복사본은 Pool과 관계 없음, 독립적으로 사용 가능 |

### 🧠 왜 Vec을 같이 끌고 다녀야 할까?
Pool<T>는 객체를 재사용하기 위한 저장소일 뿐,  
"지금 사용 중인 객체들을 추적하거나 보관하는 기능은 없습니다."  
그래서 다음과 같은 이유로 Vec<T>나 Vec<Reusable<T>> 같은 별도 컨테이너가 필요:  
| 목적                  | 설명                                                                 |
|-----------------------|----------------------------------------------------------------------|
| 사용 중 객체 추적       | Pool은 단순히 꺼내고 돌려주는 역할만 함 → 현재 사용 중인 객체는 직접 관리해야 함 |
| 반환 시점 제어         | 객체가 drop되어야 Pool에 반환됨 → Vec에서 제거되어야 drop 발생             |
| 일괄 처리 및 순회       | 여러 객체를 한 번에 처리하거나 순회하려면 Vec 같은 컨테이너가 필요             |
| 상태 보존 및 접근       | 작업 중인 객체들의 상태를 유지하고 접근하려면 별도 저장소가 필요               |

### 🔧 구조 예시
```rust
struct MySystem {
    pool: Pool<MyItem>,
    active: Vec<MyItem>, // 사용 중인 객체들
}
```
- pool.pull() → 객체 꺼냄
- active.push(item) → 사용 중인 객체 저장
- active.clear() → drop 발생 → Pool로 반환

### 🧠 대안이 있을까?
| 구조               | 특징                                      | 장점                                      | 단점                                      |
|--------------------|-------------------------------------------|-------------------------------------------|-------------------------------------------|
| Vec<Reusable<T>>   | Pool에서 꺼낸 객체를 직접 보관             | 반환 자동화, Pool과 연동 쉬움              | API가 번거롭고, 직접 순회 시 불편함         |
| Slab<T>            | 인덱스 기반으로 객체를 저장하고 관리       | 빠른 접근, ID 기반 추적 가능               | 반환은 수동 관리 필요, 재사용은 직접 구현해야 함 |
| Arena              | 대량 할당 후 전체 해제하는 메모리 블록     | 할당 빠름, 단순 구조                       | 개별 반환 불가, 전체 해제만 가능             |


## ✅ 요약
Pool을 쓰면 객체 재사용은 쉬워지지만,  
**지금 사용 중인 객체들을 직접 관리해야 하므로 Vec 같은 컨테이너가 필요** 함.  
이건 번거롭지만, 성능과 메모리 효율을 위해 감수할 만한 구조적 선택.  

---

## 🧠 왜 Arena가 편한가?
| 항목                 | 설명                                                                 |
|----------------------|----------------------------------------------------------------------|
| 대량 할당 최적화       | 많은 객체를 빠르게 할당 가능, 개별 할당보다 훨씬 효율적                    |
| 해제 타이밍 단순화     | Arena 자체를 drop하면 모든 객체가 한꺼번에 해제됨                          |
| 생명주기 관리 불필요   | 객체 간 참조가 복잡해도 Arena가 전체를 소유하므로 따로 lifetime 관리가 필요 없음 |
| API 사용이 직관적      | `arena.alloc(...)` 한 줄로 객체 생성 가능 → Pool이나 Vec 관리 불필요         |


## 🔧 예시: typed-arena 사용
```rust
use typed_arena::Arena;

#[derive(Debug)]
struct Node {
    value: u32,
}

fn main() {
    let arena = Arena::new();

    let a = arena.alloc(Node { value: 1 });
    let b = arena.alloc(Node { value: 2 });

    println!("a = {:?}", a);
    println!("b = {:?}", b);
} // arena drop → a, b 모두 해제됨

```
## ⚠️ 단점도 있어요

| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| 개별 해제 불가       | Arena에 할당된 객체는 개별적으로 drop되지 않음 → 리소스 관리에 주의 필요         |
| 장기 참조 어려움     | Arena가 drop되면 모든 객체가 사라지므로 Arena보다 오래 살아야 하는 참조는 위험함 |
| 동시성 제한         | 대부분의 Arena는 `!Send`, `!Sync` → 멀티스레드 환경에서는 사용 불가             |
| 메모리 회수 시점 고정 | Arena 전체를 drop해야 메모리 회수됨 → 유연한 해제가 어려움                     |


## ✅ 요약
Arena는 단순하고 빠른 메모리 관리를 원할 때 최고의 선택.  
특히 단일 생명주기 안에서 대량 객체를 생성/사용/폐기하는 구조에서는 정말 편함.

### 🔧 구조 예시: Arena Singleton
```rust
use once_cell::sync::Lazy;
use std::cell::RefCell;
use typed_arena::Arena;

#[derive(Debug)]
struct Node {
    value: u32,
}

// 단일 스레드 전제이므로 RefCell로 내부 mutability 확보
static ARENA: Lazy<RefCell<Arena<Node>>> = Lazy::new(|| RefCell::new(Arena::new()));

fn create_node(val: u32) -> &'static Node {
    ARENA.borrow_mut().alloc(Node { value: val })
}
```
- ARENA는 사실상 GC 힙처럼 동작하는 전역 Arena
- create_node()는 new처럼 객체를 생성하지만, 메모리는 Arena가 관리
- C#/Java 개발자에게는 **GC 힙에 객체 올리고 참조만 하면 된다** 는 느낌

### ✅ 언제 유용한가?
- 파서, 트리, 그래프 등 복잡한 참조 구조를 빠르게 만들고 버릴 때
- 단일 생명주기 내에서 대량 객체를 생성/사용/폐기하는 연산
- GC 언어 개발자에게 Rust의 메모리 모델을 부드럽게 소개할 때

### ⚠️ 단일 스레드 전제는 꼭 명확히
- typed_arena::Arena<T>는 !Send, !Sync이므로 멀티스레드에서는 사용 불가
- RefCell도 단일 스레드에서만 안전

## 💡 요약
Arena는 GC 언어 개발자에게 친숙한 메모리 모델을 제공하면서도,  
Rust의 성능과 안전성을 유지할 수 있는 훌륭한 도구입니다.  
단일 스레드 + 제한된 연산 범위에서 Singleton으로 쓰면 정말 깔끔하고 강력.  

---

