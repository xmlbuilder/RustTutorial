# Fixed Size Memory Pool

이 코드는 Rust로 작성된 **고정 크기 메모리 풀(Fixed Size Memory Pool)**입니다. 
즉, 일정 크기의 블록을 반복적으로 빠르게 할당/반환할 수 있도록 설계된 커스텀 메모리 관리자입니다. 
아래에 구조, 동작 방식, 문제점, 그리고 테스트 코드를 자세히 설명.

## 🧠 구조 설명
### 🔹 FixedSizeMemMgr
고정 크기 블록을 관리하는 메모리 풀입니다.
- alloc_size: 사용자가 요청한 블록 크기
- block_size: 실제 할당되는 블록 크기 (정렬 고려)
- chunk_bytes: 한 번에 확보하는 청크 크기
- state: 내부 상태 (free list + chunk 저장소)
### 🔹 FreeNode
- 각 블록을 연결하는 단일 연결 리스트 노드입니다.
- next: 다음 free 블록을 가리키는 포인터 (Option<NonNull<FreeNode>>)

## ⚙️ 주요 동작
- new()
    - 초기 설정: 블록 크기 정렬, 최소 크기 보장
    - FreeNode 크기와 정렬을 고려해 block_size 계산
- make_new_chunk_locked()
    - chunk_bytes만큼 메모리를 확보 (Box<[u8]>)
    - block_size 단위로 쪼개서 FreeNode로 초기화
    - 각 블록을 free_head에 연결 (freelist 구성)
- alloc_raw()
    - freelist에서 하나 꺼내서 사용자에게 반환
    - 없으면 새 청크를 만들어서 freelist 채움
- free_raw(p)
    - 사용자가 반환한 포인터를 freelist에 다시 연결
- free_all_mem()
    - 모든 청크 메모리 해제
    - freelist 초기화

## ✅ 샘플 테스트 코드
```rust
fn main() {
    use std::sync::Arc;

    let mgr = Arc::new(FixedSizeMemMgr::new(64, 1024)); // 64바이트 블록, 1KB 청크

    let p1 = mgr.alloc_raw();
    let p2 = mgr.alloc_raw();
    let p3 = mgr.alloc_raw();

    unsafe {
        std::ptr::write_bytes(p1, 0xAA, mgr.alloc_size());
        std::ptr::write_bytes(p2, 0xBB, mgr.alloc_size());
        std::ptr::write_bytes(p3, 0xCC, mgr.alloc_size());
    }

    mgr.free_raw(p2);
    mgr.free_raw(p1);
    mgr.free_raw(p3);

    mgr.free_all_mem();
}
```

```rust
fn main() {
    // 블록 크기: 64바이트, 청크 크기: 1024바이트 → 한 청크당 약 16블록
    let mgr = Arc::new(FixedSizeMemMgr::new(64, 1024));

    let mut ptrs = Vec::new();

    // 40개 블록 할당 → 1024바이트 청크를 초과하므로 자동으로 여러 청크 생성됨
    for i in 0..40 {
        let p = mgr.alloc_raw();
        println!("Allocated block {} at {:p}", i, p);
        unsafe {
            std::ptr::write_bytes(p, i as u8, mgr.alloc_size());
        }
        ptrs.push(p);
    }

    println!("Total chunks allocated: {}", mgr.state.lock().unwrap().chunks.len());

    // 모든 블록 반환
    for p in ptrs {
        mgr.free_raw(p);
    }

    // 메모리 해제
    mgr.free_all_mem();
}
```


### ⚠️ 잠재적 문제점 및 개선 제안
| 항목                        | 문제 설명                                                                 | 개선 제안                                                                 |
|-----------------------------|---------------------------------------------------------------------------|---------------------------------------------------------------------------|
| Box::leak                   | `Box::leak` 없이 `Box<[u8]>`를 그대로 사용 → drop 시 자동 해제됨         | 메모리 누수를 의도한다면 `Box::leak` 명시적으로 사용하거나 lifetime 관리 강화 |
| unsafe                      | `ptr.add(offset)` 및 `(*node_ptr).next`는 unsafe 접근                     | `block_size` 정렬 보장, 접근 전 메모리 유효성 확인 필요                  |
| free_raw                    | 동일 포인터를 두 번 `free_raw`하면 freelist가 깨질 수 있음               | double-free 방지 로직 추가 또는 반환된 포인터 추적                       |
| alloc_raw + expect()        | freelist가 비어있을 때 panic 발생 가능 (`expect("chunk must exist")`)    | panic 대신 graceful fallback 또는 에러 반환 (`Result` 사용 등)           |


## 📦 요약
이 코드는 매우 효율적인 고정 크기 메모리 풀을 구현한 예입니다.  
NonNull, Mutex, Box<[u8]>, unsafe를 적절히 활용해 성능과 안전성을 균형 있게 다루고 있어요.  
다만 unsafe 사용 시 메모리 유효성, double-free, lifetime 문제에 대한 방어 로직이 추가되면 더 견고해질 수 있습니다.

FixedSizeMemMgr는 성능 중심으로 설계되어 있지만, 메모리 안정성을 강화하려면 몇 가지 방어 로직을 추가하는 것이 좋습니다. 
아래는 다음 세 가지 문제에 대한 개선을 반영한 수정 버전입니다:

## ✅ 개선 목표
- 메모리 유효성: 반환된 포인터가 실제로 할당된 블록인지 확인
- Double-free 방지: 이미 반환된 포인터를 다시 free하지 않도록 추적
- Lifetime 안정성: Box::leak을 활용해 메모리 lifetime을 'static으로 고정

## 🔧 개선된 구조
```rust
use std::collections::HashSet;
use std::ptr::NonNull;
use std::sync::Mutex;
use std::mem::{size_of, align_of};

#[repr(C)]
struct FreeNode {
    next: Option<NonNull<FreeNode>>,
}

struct FixedState {
    free_head: Option<NonNull<FreeNode>>,
    chunks: Vec<&'static mut [u8]>,
    allocated: HashSet<*mut u8>, // 추적용
    freed: HashSet<*mut u8>,     // double-free 방지용
}

pub struct FixedSizeMemMgr {
    alloc_size: usize,
    block_size: usize,
    chunk_bytes: usize,
    state: Mutex<FixedState>,
}

impl FixedSizeMemMgr {
    pub fn new(n_alloc_size: usize, n_chunk_bytes: usize) -> Self {
        assert!(n_alloc_size > 0);
        assert!(n_chunk_bytes >= 1024);

        let need = size_of::<FreeNode>();
        let align = align_of::<FreeNode>().max(align_of::<usize>());
        let block_size = align_up(n_alloc_size.max(need), align);

        Self {
            alloc_size: n_alloc_size,
            block_size,
            chunk_bytes: n_chunk_bytes,
            state: Mutex::new(FixedState {
                free_head: None,
                chunks: Vec::new(),
                allocated: HashSet::new(),
                freed: HashSet::new(),
            }),
        }
    }

    fn make_new_chunk_locked(&self, st: &mut FixedState) {
        let boxed = vec![0u8; self.chunk_bytes].into_boxed_slice();
        let leaked: &'static mut [u8] = Box::leak(boxed);
        let ptr = leaked.as_mut_ptr();
        let len = leaked.len();
        let mut offset = 0;

        st.chunks.push(leaked);

        while offset + self.block_size <= len {
            let node_ptr = unsafe { ptr.add(offset) as *mut FreeNode };
            unsafe { (*node_ptr).next = st.free_head; }
            st.free_head = NonNull::new(node_ptr);
            st.allocated.insert(node_ptr as *mut u8);
            offset += self.block_size;
        }
    }

    pub fn alloc_raw(&self) -> *mut u8 {
        let mut st = self.state.lock().unwrap();
        if st.free_head.is_none() {
            self.make_new_chunk_locked(&mut st);
        }
        let node = st.free_head.take().expect("chunk must exist");
        let next = unsafe { (*node.as_ptr()).next };
        st.free_head = next;
        let p = node.as_ptr() as *mut u8;
        st.freed.remove(&p); // 할당되었으므로 freed 목록에서 제거
        p
    }

    pub fn free_raw(&self, p: *mut u8) {
        if p.is_null() {
            return;
        }

        let mut st = self.state.lock().unwrap();

        // 유효한 블록인지 확인
        if !st.allocated.contains(&p) {
            panic!("Invalid pointer: not from this allocator");
        }

        // double-free 방지
        if st.freed.contains(&p) {
            panic!("Double free detected");
        }

        let node_ptr = p as *mut FreeNode;
        unsafe { (*node_ptr).next = st.free_head; }
        st.free_head = NonNull::new(node_ptr);
        st.freed.insert(p);
    }

    pub fn free_all_mem(&self) {
        let mut st = self.state.lock().unwrap();
        st.chunks.clear();
        st.free_head = None;
        st.allocated.clear();
        st.freed.clear();
    }

    pub fn alloc_size(&self) -> usize { self.alloc_size }
    pub fn block_size(&self) -> usize { self.block_size }
    pub fn chunk_bytes(&self) -> usize { self.chunk_bytes }
}

fn align_up(x: usize, align: usize) -> usize {
    (x + align - 1) & !(align - 1)
}
```

## 🧪 테스트 포인트
- ✅ 유효하지 않은 포인터를 free_raw()에 넘기면 panic
- ✅ 같은 포인터를 두 번 free_raw()하면 panic
- ✅ alloc_raw()로 할당된 포인터만 free_raw() 가능
- ✅ free_all_mem() 호출 시 모든 상태 초기화
좋은 질문이에요 JungHwan님! C++에서는 커스텀 메모리 풀을 만들고 operator new(size_t) 또는 std::allocator를 통해 객체를 직접 생성하죠. Rust에서도 비슷한 방식으로 메모리 풀에서 직접 객체를 생성할 수 있습니다. 다만 Rust는 안전성과 명시적인 제어를 중요시하기 때문에 방식이 조금 다릅니다.


---

# 🧠 Rust에서 메모리 풀 기반 객체 생성 방식
Rust에서는 다음과 같은 방식으로 메모리 풀에서 객체를 생성:

1. alloc_raw() → 포인터 확보
2. ptr::write() → 객체를 직접 생성
3. ptr::drop_in_place() → 수동으로 drop 처리

## 🔧 예시: 메모리 풀에서 구조체 생성
```rust
use std::ptr;
use std::mem;
use std::sync::Arc;

#[derive(Debug)]
struct MyStruct {
    a: u32,
    b: u64,
}

fn main() {
    let pool = Arc::new(FixedSizeMemMgr::new(mem::size_of::<MyStruct>(), 1024));

    // 1. 메모리 풀에서 raw 포인터 확보
    let raw = pool.alloc_raw() as *mut MyStruct;

    // 2. 객체를 직접 생성 (placement new처럼)
    unsafe {
        ptr::write(raw, MyStruct { a: 10, b: 20 });
    }

    // 3. 사용
    unsafe {
        println!("Created: {:?}", *raw);
    }

    // 4. 수동 drop (필요 시)
    unsafe {
        ptr::drop_in_place(raw);
    }

    // 5. 메모리 반환
    pool.free_raw(raw as *mut u8);
}
```



## ✅ 이 방식의 특징
| 단계             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| alloc_raw()      | 메모리 풀에서 초기화되지 않은 고정 크기 블록을 확보함                |
| ptr::write()     | 확보한 블록에 객체를 직접 생성 (C++의 placement new와 유사)          |
| drop_in_place()  | 객체를 수동으로 drop 처리하여 자원 해제 (Rust의 drop 호출을 직접 수행) |
| free_raw()       | 메모리 풀에 블록을 반환하여 재사용 가능 상태로 만듦                   |



## 🛡️ 안전하게 쓰려면?
- unsafe를 사용할 수밖에 없지만, 메모리 풀 내부에서 유효성 체크와 double-free 방지 로직을 넣으면 안전하게 사용할 수 있어요.
- Box::from_raw()을 활용하면 Rust의 drop 시스템과 통합할 수도 있습니다:
let boxed = unsafe { Box::from_raw(raw) };
// boxed는 drop 시 자동으로 메모리 해제됨 (단, 메모리 풀과 충돌 주의)


하지만 이 방식은 메모리 풀이 Box와 호환되는 구조일 때만 사용해야 해요.

----

# 📦 C++ 메모리 풀 기반 객체 생성/소멸 예제
## 🔧 1. 메모리 풀 클래스 정의
```cpp
#include <iostream>
#include <vector>
#include <cassert>

class MemoryPool {
public:
    MemoryPool(size_t blockSize, size_t blockCount)
        : blockSize_(blockSize), blockCount_(blockCount) {
        pool_ = ::operator new(blockSize_ * blockCount_);
        for (size_t i = 0; i < blockCount_; ++i) {
            void* ptr = static_cast<char*>(pool_) + i * blockSize_;
            freeList_.push_back(ptr);
        }
    }

    ~MemoryPool() {
        ::operator delete(pool_);
    }

    void* allocate() {
        assert(!freeList_.empty() && "Memory pool exhausted");
        void* ptr = freeList_.back();
        freeList_.pop_back();
        return ptr;
    }

    void deallocate(void* ptr) {
        freeList_.push_back(ptr);
    }

private:
    void* pool_;
    size_t blockSize_;
    size_t blockCount_;
    std::vector<void*> freeList_;
};
```


## 🧱 2. 사용자 클래스 정의
```cpp
class MyObject {
public:
    MyObject(int x) : x_(x) {
        std::cout << "MyObject(" << x_ << ") constructed\n";
    }

    ~MyObject() {
        std::cout << "MyObject(" << x_ << ") destructed\n";
    }

    void print() const {
        std::cout << "Value: " << x_ << "\n";
    }

private:
    int x_;
};
```


## 🚀 3. 메모리 풀에서 객체 생성/소멸
```cpp
int main() {
    MemoryPool pool(sizeof(MyObject), 10); // 10개 블록 준비

    // 객체 생성 (placement new)
    void* raw = pool.allocate();
    MyObject* obj = new (raw) MyObject(42);

    obj->print();

    // 객체 소멸
    obj->~MyObject();
    pool.deallocate(raw);

    return 0;
}
```


## ✅ 핵심 요약
| 단계             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| allocate()       | 메모리 풀에서 고정 크기 블록 하나를 확보함                           |
| new (ptr) T()    | 확보한 블록에 placement new를 사용해 객체를 직접 생성함              |
| obj->~T()        | 명시적으로 소멸자를 호출하여 객체의 자원을 해제함                    |
| deallocate()     | 사용한 블록을 메모리 풀에 반환하여 재사용 가능 상태로 만듦           |


---

# Thread Safe C++ Memory Pool

C++에서 메모리 풀을 직접 구현하고 사용하는 가장 기본적인 패턴이에요. 


## 🧠 핵심 개념
- std::mutex로 allocate()와 deallocate() 보호
- placement new로 객체 생성
- 명시적 소멸자 호출로 객체 파괴
- 메모리 풀은 고정 크기 블록을 관리

## 🔧 Thread-safe MemoryPool 예제
```cpp
#include <iostream>
#include <vector>
#include <mutex>
#include <cassert>

class ThreadSafeMemoryPool {
public:
    ThreadSafeMemoryPool(size_t blockSize, size_t blockCount)
        : blockSize_(blockSize), blockCount_(blockCount) {
        pool_ = ::operator new(blockSize_ * blockCount_);
        for (size_t i = 0; i < blockCount_; ++i) {
            void* ptr = static_cast<char*>(pool_) + i * blockSize_;
            freeList_.push_back(ptr);
        }
    }

    ~ThreadSafeMemoryPool() {
        ::operator delete(pool_);
    }

    void* allocate() {
        std::lock_guard<std::mutex> lock(mutex_);
        assert(!freeList_.empty() && "Memory pool exhausted");
        void* ptr = freeList_.back();
        freeList_.pop_back();
        return ptr;
    }

    void deallocate(void* ptr) {
        std::lock_guard<std::mutex> lock(mutex_);
        freeList_.push_back(ptr);
    }

private:
    void* pool_;
    size_t blockSize_;
    size_t blockCount_;
    std::vector<void*> freeList_;
    std::mutex mutex_;
};
```


## 🧱 사용자 클래스
```cpp
class MyObject {
public:
    MyObject(int x) : x_(x) {
        std::cout << "MyObject(" << x_ << ") constructed\n";
    }

    ~MyObject() {
        std::cout << "MyObject(" << x_ << ") destructed\n";
    }

    void print() const {
        std::cout << "Value: " << x_ << "\n";
    }

private:
    int x_;
};
```


## 🚀 사용 예제 (main)
```cpp
int main() {
    ThreadSafeMemoryPool pool(sizeof(MyObject), 10);

    // 객체 생성
    void* raw = pool.allocate();
    MyObject* obj = new (raw) MyObject(100);

    obj->print();

    // 객체 소멸
    obj->~MyObject();
    pool.deallocate(raw);

    return 0;
}
```


## ✅ 핵심 요약
| 단계             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| allocate()       | mutex로 보호된 메모리 풀에서 블록 하나 확보                          |
| new (ptr) T()    | placement new로 객체 생성                                            |


---
# FixedSizeSafeMemMgr

Rust로 구현한 FixedSizeSafeMemMgr입니다. 기존의 FixedSizeMemMgr를 기반으로 하되,  
다음과 같은 thread-safe 방어 로직을 강화한 버전:


## ✅ 개선 사항 요약
| 항목       | 설명                                      |
|------------|-------------------------------------------|
| Mutex      | 내부 상태(`FixedState`)를 `Mutex`로 감싸 멀티스레드 안전성 확보 |
| HashSet    | 할당된 포인터와 반환된 포인터를 추적하여 double-free 및 유효성 검사 |
| 'static    | `Box::leak`을 사용해 메모리 lifetime을 `'static`으로 고정하여 raw pointer 안정성 확보 |


## 🧱 FixedSizeSafeMemMgr 구현
```rust
use std::collections::HashSet;
use std::mem::{align_of, size_of};
use std::ptr::NonNull;
use std::sync::Mutex;

#[repr(C)]
struct FreeNode {
    next: Option<NonNull<FreeNode>>,
}

struct FixedState {
    free_head: Option<NonNull<FreeNode>>,
    chunks: Vec<&'static mut [u8]>,
    allocated: HashSet<*mut u8>,
    freed: HashSet<*mut u8>,
}

pub struct FixedSizeSafeMemMgr {
    alloc_size: usize,
    block_size: usize,
    chunk_bytes: usize,
    state: Mutex<FixedState>,
}

impl FixedSizeSafeMemMgr {
    pub fn new(n_alloc_size: usize, n_chunk_bytes: usize) -> Self {
        assert!(n_alloc_size > 0);
        assert!(n_chunk_bytes >= 1024);

        let need = size_of::<FreeNode>();
        let align = align_of::<FreeNode>().max(align_of::<usize>());
        let block_size = align_up(n_alloc_size.max(need), align);

        Self {
            alloc_size: n_alloc_size,
            block_size,
            chunk_bytes: n_chunk_bytes,
            state: Mutex::new(FixedState {
                free_head: None,
                chunks: Vec::new(),
                allocated: HashSet::new(),
                freed: HashSet::new(),
            }),
        }
    }

    fn make_new_chunk_locked(&self, st: &mut FixedState) {
        let boxed = vec![0u8; self.chunk_bytes].into_boxed_slice();
        let leaked: &'static mut [u8] = Box::leak(boxed);
        let ptr = leaked.as_mut_ptr();
        let len = leaked.len();
        let mut offset = 0;

        st.chunks.push(leaked);

        while offset + self.block_size <= len {
            let node_ptr = unsafe { ptr.add(offset) as *mut FreeNode };
            unsafe { (*node_ptr).next = st.free_head; }
            st.free_head = NonNull::new(node_ptr);
            st.allocated.insert(node_ptr as *mut u8);
            offset += self.block_size;
        }
    }

    pub fn alloc_raw(&self) -> *mut u8 {
        let mut st = self.state.lock().unwrap();
        if st.free_head.is_none() {
            self.make_new_chunk_locked(&mut st);
        }
        let node = st.free_head.take().expect("chunk must exist");
        let next = unsafe { (*node.as_ptr()).next };
        st.free_head = next;
        let p = node.as_ptr() as *mut u8;
        st.freed.remove(&p);
        p
    }

    pub fn free_raw(&self, p: *mut u8) {
        if p.is_null() {
            return;
        }

        let mut st = self.state.lock().unwrap();

        if !st.allocated.contains(&p) {
            panic!("Invalid pointer: not from this allocator");
        }

        if st.freed.contains(&p) {
            panic!("Double free detected");
        }

        let node_ptr = p as *mut FreeNode;
        unsafe { (*node_ptr).next = st.free_head; }
        st.free_head = NonNull::new(node_ptr);
        st.freed.insert(p);
    }

    pub fn free_all_mem(&self) {
        let mut st = self.state.lock().unwrap();
        st.chunks.clear();
        st.free_head = None;
        st.allocated.clear();
        st.freed.clear();
    }

    pub fn alloc_size(&self) -> usize { self.alloc_size }
    pub fn block_size(&self) -> usize { self.block_size }
    pub fn chunk_bytes(&self) -> usize { self.chunk_bytes }
}

fn align_up(x: usize, align: usize) -> usize {
    (x + align - 1) & !(align - 1)
}
```


## 🧪 테스트 예제
```rust
fn main() {
    use std::sync::Arc;
    use std::thread;

    let mgr = Arc::new(FixedSizeSafeMemMgr::new(64, 1024));

    let handles: Vec<_> = (0..4).map(|i| {
        let mgr = mgr.clone();
        thread::spawn(move || {
            let p = mgr.alloc_raw();
            println!("Thread {} allocated {:p}", i, p);
            unsafe {
                std::ptr::write_bytes(p, i as u8, mgr.alloc_size());
            }
            mgr.free_raw(p);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("All threads completed safely.");
}
```


이제 이 구조는 멀티스레드 환경에서도 안전하게 작동하며, double-free와 유효하지 않은 포인터 접근을 방지합니다.

---

# 🧠 TypedAllocator<T>란?
- FixedSizeSafeMemMgr는 블록 단위로 raw pointer를 반환합니다.
- TypedAllocator<T>는 그 위에 타입 T를 올려서 객체 생성/소멸을 자동화합니다.
- 즉, T 타입의 객체를 안전하게 생성하고, drop 시 자동으로 메모리 풀에 반환되게 만드는 래퍼입니다.

## 🔧 구조 설계
### 1. TypedAllocator<T>: 타입 기반 메모리 풀
```rust
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::marker::PhantomData;

pub struct TypedAllocator<T> {
    pool: FixedSizeSafeMemMgr,
    _marker: PhantomData<T>,
}

impl<T> TypedAllocator<T> {
    pub fn new(chunk_bytes: usize) -> Self {
        let alloc_size = std::mem::size_of::<T>();
        Self {
            pool: FixedSizeSafeMemMgr::new(alloc_size, chunk_bytes),
            _marker: PhantomData,
        }
    }

    pub fn alloc(&self, value: T) -> TypedBox<T> {
        let raw = self.pool.alloc_raw() as *mut T;
        unsafe {
            ptr::write(raw, value);
        }
        TypedBox {
            ptr: raw,
            pool: &self.pool,
        }
    }
}
```


### 2. TypedBox<T>: Drop 자동 처리
```rust
use std::ops::{Deref, DerefMut};
use std::{fmt, ptr};
use std::marker::PhantomData;
use crate::core::memmgr_shared::FixedSizeSafeMemMgr;

pub struct TypedBox<'a, T> {
    ptr: *mut T,
    pool: &'a FixedSizeSafeMemMgr,
}

impl<'a, T> Deref for TypedBox<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<'a, T> DerefMut for TypedBox<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<'a, T> Drop for TypedBox<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(self.ptr);
        }
        self.pool.free_raw(self.ptr as *mut u8);
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for TypedBox<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f) // Deref를 통해 T의 Debug 출력 사용
    }
}
```

🧪 사용 예제

```rust

#[derive(Debug)]
struct MyStruct {
    x: u32,
    y: u64,
}

fn main() {
    let allocator = TypedAllocator::<MyStruct>::new(1024);

    let obj = allocator.alloc(MyStruct { x: 1, y: 2 });
    println!("Allocated: {:?}", obj);

    // Drop은 자동으로 호출됨
}

```

## ✅ 이 구조의 장점
| 항목                | 설명                                                                 |
|---------------------|----------------------------------------------------------------------|
| TypedAllocator<T>   | 타입 기반 메모리 풀로, T 타입 객체만 안전하게 생성 가능               |
| TypedBox<T>         | 객체를 감싸는 스마트 포인터로, Drop 시 자동으로 메모리 풀에 반환됨     |
| Deref               | `Deref` 구현으로 일반 객체처럼 `*obj`, `obj.x` 형태로 접근 가능         |
| FixedSizeSafeMemMgr | 내부적으로 `Mutex`로 보호되어 멀티스레드 환경에서도 안전하게 동작함     |




---
