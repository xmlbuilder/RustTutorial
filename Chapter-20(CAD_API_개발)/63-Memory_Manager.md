# Memory Manager 

## 소스 코드

```rust
use std::collections::{HashMap, HashSet};
use std::mem::{align_of, size_of};
use std::ptr::{self, NonNull};
use std::sync::{Arc, Mutex};

/// 내부 free-list 노드 (각 블록 선두에 들어감)
#[repr(C)]
struct FreeNode {
    next: Option<NonNull<FreeNode>>,
}
```
```rust

fn align_up(v: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    (v + (align - 1)) & !(align - 1)
}
```
```rust
struct FixedState {
    free_head: Option<NonNull<FreeNode>>,
    chunks: Vec<&'static mut [u8]>,
    allocated: HashSet<*mut u8>,
    freed: HashSet<*mut u8>,
}
```
```rust
pub struct FixedSizeMemMgr {
    alloc_size: usize,
    block_size: usize,
    chunk_bytes: usize,
    state: Mutex<FixedState>,
}
```
```rust
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
```
```rust
    fn make_new_chunk_locked(&self, st: &mut FixedState) {
        let boxed = vec![0u8; self.chunk_bytes].into_boxed_slice();
        let leaked: &'static mut [u8] = Box::leak(boxed);
        let ptr = leaked.as_mut_ptr();
        let len = leaked.len();
        let mut offset = 0;

        st.chunks.push(leaked);

        while offset + self.block_size <= len {
            let node_ptr = unsafe { ptr.add(offset) as *mut FreeNode };
            unsafe {
                (*node_ptr).next = st.free_head;
            }
            st.free_head = NonNull::new(node_ptr);
            st.allocated.insert(node_ptr as *mut u8);
            offset += self.block_size;
        }
    }
```
```rust
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
```
```rust
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
        unsafe {
            (*node_ptr).next = st.free_head;
        }
        st.free_head = NonNull::new(node_ptr);
        st.freed.insert(p);
    }
```
```rust
    pub fn free_all_mem(&self) {
        let mut st = self.state.lock().unwrap();
        st.chunks.clear();
        st.free_head = None;
        st.allocated.clear();
        st.freed.clear();
    }
```
```rust
    pub fn alloc_size(&self) -> usize {
        self.alloc_size
    }
```
```rust    
    pub fn block_size(&self) -> usize {
        self.block_size
    }
```
```rust    
    pub fn chunk_bytes(&self) -> usize {
        self.chunk_bytes
    }
```
```rust
    pub fn num_chunks(&self) -> usize {
        let st = self.state.lock().unwrap();
        st.chunks.len()
    }
}
```
```rust
pub struct FixedSizeSafeMemMgr {
    alloc_size: usize,
    block_size: usize,
    chunk_bytes: usize,
    state: Mutex<FixedState>,
}
```
```rust
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
```
```rust
    fn make_new_chunk_locked(&self, st: &mut FixedState) {
        let boxed = vec![0u8; self.chunk_bytes].into_boxed_slice();
        let leaked: &'static mut [u8] = Box::leak(boxed);
        let ptr = leaked.as_mut_ptr();
        let len = leaked.len();
        let mut offset = 0;

        st.chunks.push(leaked);

        while offset + self.block_size <= len {
            let node_ptr = unsafe { ptr.add(offset) as *mut FreeNode };
            unsafe {
                (*node_ptr).next = st.free_head;
            }
            st.free_head = NonNull::new(node_ptr);
            st.allocated.insert(node_ptr as *mut u8);
            offset += self.block_size;
        }
    }
```
```rust
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
```
```rust
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
        unsafe {
            (*node_ptr).next = st.free_head;
        }
        st.free_head = NonNull::new(node_ptr);
        st.freed.insert(p);
    }
```
```rust
    pub fn free_all_mem(&self) {
        let mut st = self.state.lock().unwrap();
        st.chunks.clear();
        st.free_head = None;
        st.allocated.clear();
        st.freed.clear();
    }

    pub fn alloc_size(&self) -> usize {
        self.alloc_size
    }
    pub fn block_size(&self) -> usize {
        self.block_size
    }
    pub fn chunk_bytes(&self) -> usize {
        self.chunk_bytes
    }
}
```
```rust
unsafe impl Send for FixedSizeSafeMemMgr {}
unsafe impl Sync for FixedSizeSafeMemMgr {}

/// 타입 소멸자 지움(erase)용 함수 포인터
type ErasedDropFn = unsafe fn(*mut u8);
```
```rust
#[warn(unsafe_op_in_unsafe_fn)]
unsafe fn drop_impl<T>(p: *mut u8) {
    unsafe {
        (p as *mut T).drop_in_place();
    }
}
```
```rust
//
// EnhancedMemMgr (공유 가능한 고정 크기 풀 + 객체 생성/파괴 도우미)
//
#[derive(Clone)]
pub struct EnhancedMemMgr {
    pool: Arc<FixedSizeMemMgr>,
}
```
```rust
impl EnhancedMemMgr {
    /// n_alloc_size: 사용자 블록 크기
    /// nodes_per_chunk: 한 청크에 들어갈 블록 개수
    pub fn new(n_alloc_size: usize, nodes_per_chunk: usize) -> Self {
        let probe = FixedSizeMemMgr::new(n_alloc_size, nodes_per_chunk);
        let chunk_bytes = (probe.block_size() * nodes_per_chunk).max(1024);
        let pool = Arc::new(FixedSizeMemMgr::new(n_alloc_size, chunk_bytes));
        Self { pool }
    }
```
```rust
    pub fn from_pool(pool: Arc<FixedSizeMemMgr>) -> Self {
        Self { pool }
    }
```
```rust
    pub fn arc_pool(&self) -> Arc<FixedSizeMemMgr> {
        Arc::clone(&self.pool)
    }
```
```rust
    pub fn alloc_raw(&self) -> *mut u8 {
        self.pool.alloc_raw()
    }
```
```rust
    pub fn free_raw(&self, p: *mut u8) {
        self.pool.free_raw(p)
    }
```
```rust
    pub fn free_all_mem(&self) {
        self.pool.free_all_mem()
    }
```
```rust
    /// 풀에서 블록을 받아 T를 배치 생성 (placement new)
    pub fn alloc_object<T>(&self, value: T) -> NonNull<T> {
        assert!(
            size_of::<T>() <= self.pool.alloc_size(),
            "alloc_size({}) < size_of::<T>({})",
            self.pool.alloc_size(),
            size_of::<T>()
        );
        let p = self.pool.alloc_raw();
        unsafe {
            let tp = p as *mut T;
            ptr::write(tp, value);
            NonNull::new(tp).expect("non-null")
        }
    }
```
```rust
    /// T 소멸 후 블록 반납
    pub unsafe fn free_object<T>(&self, obj: NonNull<T>) {
        unsafe {
            ptr::drop_in_place(obj.as_ptr());
        }
        self.pool.free_raw(obj.as_ptr() as *mut u8);
    }
}
unsafe impl Send for EnhancedMemMgr {}
```
```rust
//
// SafeMemMgr (공유 가능, 생성한 객체 전체/개별 관리)
//
#[derive(Clone)]
pub struct SafeMemMgr {
    pool: Arc<FixedSizeMemMgr>,
    objects: Arc<Mutex<Vec<(*mut u8, ErasedDropFn)>>>, // (ptr, drop_fn)
}
```
```rust
impl SafeMemMgr {
    pub fn new(alloc_size: usize, nodes_per_chunk: usize) -> Self {
        let probe = FixedSizeMemMgr::new(alloc_size, 1024);
        let chunk_bytes = (probe.block_size() * nodes_per_chunk).max(1024);
        let pool = Arc::new(FixedSizeMemMgr::new(alloc_size, chunk_bytes));
        Self {
            pool,
            objects: Arc::new(Mutex::new(Vec::new())),
        }
    }
```
```rust
    pub fn from_pool(pool: Arc<FixedSizeMemMgr>) -> Self {
        Self {
            pool,
            objects: Arc::new(Mutex::new(Vec::new())),
        }
    }
```
```rust
    pub fn arc_pool(&self) -> Arc<FixedSizeMemMgr> {
        Arc::clone(&self.pool)
    }
```
```rust
    pub fn alloc_object<T>(&self, value: T) -> NonNull<T> {
        assert!(
            size_of::<T>() <= self.pool.alloc_size(),
            "alloc_size({}) < size_of::<T>({})",
            self.pool.alloc_size(),
            size_of::<T>()
        );
        let p = self.pool.alloc_raw();
        unsafe {
            let tp = p as *mut T;
            ptr::write(tp, value);
            self.objects
                .lock()
                .unwrap()
                .push((tp as *mut u8, drop_impl::<T>));
            NonNull::new(tp).expect("non-null")
        }
    }
```
```rust
    pub unsafe fn free_object<T>(&self, obj: NonNull<T>) {
        let mut v = self.objects.lock().unwrap();
        if let Some(pos) = v.iter().position(|(p, _)| *p == obj.as_ptr() as *mut u8) {
            let (_p, drop_fn) = v.remove(pos);
            unsafe {
                drop_fn(obj.as_ptr() as *mut u8);
            }

            self.pool.free_raw(obj.as_ptr() as *mut u8);
        }
    }
```
```rust
    pub fn free_all_objects(&self) {
        let mut v = self.objects.lock().unwrap();
        for (p, drop_fn) in v.drain(..) {
            unsafe { drop_fn(p) };
            self.pool.free_raw(p);
        }
        // 풀 자체를 비우고 싶다면:
        self.pool.free_all_mem();
    }
}
```
```rust
unsafe impl Send for SafeMemMgr {}
```
```rust
//
// SafeMemMgrEx (공유 가능, 크기별 풀 자동관리 + 태그 그룹핑)
//
#[derive(Clone)]
pub struct SafeMemMgrEx {
    pools: Arc<Mutex<HashMap<usize, Arc<FixedSizeMemMgr>>>>, // size -> pool
    tagged: Arc<Mutex<HashMap<String, Vec<(usize, *mut u8, ErasedDropFn)>>>>, // tag -> [(size, ptr, drop)]
}
```
```rust
impl SafeMemMgrEx {
    pub fn new() -> Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
            tagged: Arc::new(Mutex::new(HashMap::new())),
        }
    }
```
```rust
    fn get_or_create_pool_locked(
        pools: &mut HashMap<usize, Arc<FixedSizeMemMgr>>,
        size: usize,
    ) -> Arc<FixedSizeMemMgr> {
        if let Some(p) = pools.get(&size) {
            return Arc::clone(p);
        }
        let probe = FixedSizeMemMgr::new(size, 1024);
        let chunk_bytes = (probe.block_size() * size).max(1024);
        let p = Arc::new(FixedSizeMemMgr::new(size, chunk_bytes));
        pools.insert(size, Arc::clone(&p));
        p
    }
```
```rust
    pub fn alloc_object<T>(&self, tag: &str, value: T) -> NonNull<T> {
        let size = size_of::<T>();
        // 풀 확보 및 배치 생성
        let tp = {
            let p = {
                let mut pools = self.pools.lock().unwrap();
                Self::get_or_create_pool_locked(&mut pools, size)
            };
            let raw = p.alloc_raw();
            unsafe {
                let tp = raw as *mut T;
                ptr::write(tp, value);
                tp
            }
        };
        // 태그 등록
        {
            let mut tagged = self.tagged.lock().unwrap();
            tagged
                .entry(tag.to_string())
                .or_default()
                .push((size, tp as *mut u8, drop_impl::<T>));
        }
        NonNull::new(tp).expect("non-null")
    }
```
```rust
    pub unsafe fn free_object<T>(&self, tag: &str, obj: NonNull<T>) {
        let size = size_of::<T>();
        let mut tagged = self.tagged.lock().unwrap();
        if let Some(vec) = tagged.get_mut(tag) {
            if let Some(pos) = vec
                .iter()
                .position(|(s, p, _)| *s == size && *p == obj.as_ptr() as *mut u8)
            {
                let (_s, p_u8, drop_fn) = vec.remove(pos);
                unsafe {
                    drop_fn(p_u8);
                }
                let pools = self.pools.lock().unwrap();
                if let Some(pool) = pools.get(&size) {
                    pool.free_raw(p_u8);
                }
            }
            if vec.is_empty() {
                tagged.remove(tag);
            }
        }
    }
```
```rust
    pub fn free_objects_by_tag(&self, tag: &str) {
        let vec_opt = {
            let mut tagged = self.tagged.lock().unwrap();
            tagged.remove(tag)
        };

        if let Some(vec) = vec_opt {
            let pools = self.pools.lock().unwrap();
            for (size, p_u8, drop_fn) in vec {
                unsafe { drop_fn(p_u8) };
                if let Some(pool) = pools.get(&size) {
                    pool.free_raw(p_u8);
                }
            }
        }
    }
```
```rust
    pub fn free_all_objects(&self) {
        let all = {
            let mut tagged = self.tagged.lock().unwrap();
            let mut out = Vec::new();
            for (_tag, vec) in tagged.drain() {
                out.extend(vec);
            }
            out
        };
        {
            let mut pools = self.pools.lock().unwrap();
            for (size, p_u8, drop_fn) in all {
                unsafe { drop_fn(p_u8) };
                if let Some(pool) = pools.get(&size) {
                    pool.free_raw(p_u8);
                }
            }
            // 풀 자체까지 비우고 싶으면 아래 주석 해제
            for (_s, pool) in pools.iter() { pool.free_all_mem(); }
            pools.clear();
        }
    }
}

unsafe impl Send for SafeMemMgrEx {}
```

## ✅ 전체 구조 요약

| 구조체 이름            | 설명                                                                 |
|------------------------|----------------------------------------------------------------------|
| `FixedSizeMemMgr`      | 고정 크기 블록을 관리하는 기본 메모리 풀. free-list 기반 할당/해제 지원 |
| `FixedSizeSafeMemMgr`  | `FixedSizeMemMgr`의 `Send + Sync` 안전 버전. 멀티스레드 환경 대응       |
| `EnhancedMemMgr`       | `FixedSizeMemMgr` 기반 객체 생성/소멸 지원. `alloc_object`, `free_object` 제공 |
| `SafeMemMgr`           | 생성된 객체를 추적하여 안전하게 해제. `free_all_objects` 지원           |
| `SafeMemMgrEx`         | 크기별 풀 자동 생성 + 태그 기반 객체 그룹 관리. `free_objects_by_tag` 지원 |


## 🔍 기능 점검 요약
- Free-list 기반 메모리 관리: FreeNode를 이용한 블록 재사용
- 동기화 처리: Mutex로 내부 상태 보호
- double-free 및 외부 포인터 방지: allocated, freed 추적
- 객체 생성/소멸 지원: alloc_object, free_object에서 placement new와 drop_in_place 사용
- 태그 기반 그룹 관리: SafeMemMgrEx에서 tag로 객체 그룹화 및 일괄 해제 가능
- 전체적으로 C++에서의 custom allocator 패턴을 Rust의 안전성과 타입 시스템에 맞게 잘 옮기셨습니다. 

## 📘 함수별 설명
- 아래는 주요 함수들의 역할과 용도를 정리한 표입니다:

### 🧩 메모리 매니저 함수표

| 함수 이름                  | 소속 구조체           | 역할 및 설명 |
|---------------------------|------------------------|--------------|
| `new`                     | 모든 구조체            | 초기화. 블록 크기, 청크 크기 계산 및 상태 설정 |
| `alloc_raw`               | `FixedSizeMemMgr` 외   | 블록 하나를 할당. free-list에서 꺼냄 |
| `free_raw`                | `FixedSizeMemMgr` 외   | 블록을 반환. double-free 및 외부 포인터 검사 포함 |
| `free_all_mem`            | `FixedSizeMemMgr` 외   | 모든 메모리 청크와 상태 초기화 |
| `alloc_size`, `block_size`, `chunk_bytes` | `FixedSizeMemMgr` | 설정된 크기 정보 반환 |
| `num_chunks`              | `FixedSizeMemMgr`      | 현재 청크 개수 반환 |
| `alloc_object<T>`         | `EnhancedMemMgr`, `SafeMemMgr`, `SafeMemMgrEx` | 블록에 객체를 배치 생성 (placement new) |
| `free_object<T>`          | `EnhancedMemMgr`, `SafeMemMgr`, `SafeMemMgrEx` | 객체 소멸 후 블록 반환 |
| `free_all_objects`        | `SafeMemMgr`, `SafeMemMgrEx` | 생성된 모든 객체 소멸 및 반환 |
| `free_objects_by_tag`     | `SafeMemMgrEx`         | 특정 태그에 속한 객체들만 소멸 및 반환 |
| `arc_pool`, `from_pool`   | `EnhancedMemMgr`, `SafeMemMgr` | 풀 공유 및 외부 풀로부터 생성 |
| `get_or_create_pool_locked` | `SafeMemMgrEx`       | 크기별 풀 자동 생성 및 캐싱 |

----


## 🧪 테스트 방법 제안
- Rust에서는 #[test]를 활용한 단위 테스트가 가능합니다. 예를 들어:
```rust
#[test]
fn test_alloc_and_free() {
    let mgr = FixedSizeMemMgr::new(64, 1024);
    let p1 = mgr.alloc_raw();
    let p2 = mgr.alloc_raw();
    assert_ne!(p1, ptr::null_mut());
    assert_ne!(p2, ptr::null_mut());
    mgr.free_raw(p1);
    mgr.free_raw(p2);
}
```

- 또는 객체 생성/소멸 테스트:
```rust
#[test]
fn test_object_lifecycle() {
    let mgr = EnhancedMemMgr::new(64, 16);
    let obj = mgr.alloc_object(String::from("Hello"));
    unsafe { mgr.free_object(obj); }
}
```
- 실제 내용물 접근
```rust
#[test]
fn test_object_lifecycle() {
    let mgr = EnhancedMemMgr::new(64, 1024);
    let mut obj = mgr.alloc_object(String::from("Hello"));
    println!("obj: {:?}", obj);

    // ✅ 포인터에서 참조 얻기
    let s: &String = unsafe { obj.as_ref() };
    println!("String 내용: {}", s);

    // ✅ 포인터에서 가변 참조 얻기 (필요 시)
        let s_mut: &mut String = unsafe { obj.as_mut() };
        s_mut.push_str(" world");

    println!("String 내용: {}", s_mut);

    unsafe { mgr.free_object(obj); }
}
```
-  출력 결과
```
obj: 0x1d126a09700
String 내용: Hello
String 내용: Hello world
```

## 🧠 결론
이 메모리 매니저는 C++의 low-level allocator 패턴을 Rust의 안전성과 타입 시스템에 맞게 훌륭히 재현한 구조입니다.
free-list, chunked allocation, placement new, drop tracking, tag-based grouping까지 모두 구현되어 있어 실전에서도 충분히 활용 가능합니다.

--- 

# 샘플 코드와 단위 테스트 코드

아래는 각 구조체별로 사용자용 샘플 코드와 단위 테스트 코드를 하나씩 정리한 것입니다.

## 🧱 1. FixedSizeMemMgr 샘플
### 📦 사용 예제
```rust
use memmgr::FixedSizeMemMgr;

fn main() {
    let mgr = FixedSizeMemMgr::new(64, 1024); // 64바이트 블록, 1KB 청크
    let p1 = mgr.alloc_raw();
    let p2 = mgr.alloc_raw();

    println!("Allocated pointers: {:?}, {:?}", p1, p2);

    mgr.free_raw(p1);
    mgr.free_raw(p2);
}
```
### 출력 결과
```
Allocated pointers: 0x1c57d4be540, 0x1c57d4be500
```

### 왜 주소 차이가 40 일까?
#### 🔍 코드 설정
```rust
let mgr = FixedSizeMemMgr::new(64, 1024); // 블록 크기: 64바이트
```
- 이 설정은 64바이트 단위로 메모리를 할당한다는 의미입니다.
- 즉, alloc_raw()를 호출하면 64바이트 크기의 블록을 하나 반환합니다.
- 📌 실제로는 0x40 = 64바이트입니다.
```
Allocated pointers: 0x1c57d4be540, 0x1c57d4be500
```
- 두 주소의 차이: 0x1c57d4be540 - 0x1c57d4be500 = 0x40 = 64
- 즉, 64바이트 간격으로 정확히 떨어져 있음 → 정상적인 동작입니다!

#### ✅ 결론
- FixedSizeMemMgr는 내부적으로 연속된 메모리 블록을 64바이트 단위로 관리합니다.
- alloc_raw()는 다음 사용 가능한 블록을 반환하므로, 주소 차이가 정확히 64바이트 나는 것이 맞습니다.
- 이건 오히려 메모리 풀이 정확하게 정렬되고 최적화되어 있다는 증거예요.


## 🧱 2. FixedSizeSafeMemMgr 샘플
### 📦 사용 예제
```rust
use memmgr::FixedSizeSafeMemMgr;

fn main() {
    let mgr = FixedSizeSafeMemMgr::new(128, 2048);
    let p = mgr.alloc_raw();
    println!("Allocated: {:?}", p);
    mgr.free_raw(p);
}
```

### ✅ 테스트 코드
```rust
#[test]
fn test_fixed_size_safe_mem_mgr() {
    let mgr = FixedSizeSafeMemMgr::new(128, 2048);
    let p = mgr.alloc_raw();
    assert!(!p.is_null());
    mgr.free_raw(p);
}
```


## 🧱 3. EnhancedMemMgr 샘플
### 📦 사용 예제
```rust
use memmgr::EnhancedMemMgr;

fn main() {
    let mgr = EnhancedMemMgr::new(64, 16);
    let obj = mgr.alloc_object(String::from("Hello, world!"));
    unsafe {
        mgr.free_object(obj);
    }
}
```
### ✅ 테스트 코드
```rust
#[test]
fn test_enhanced_mem_mgr_object() {
    let mgr = EnhancedMemMgr::new(64, 16);
    let obj = mgr.alloc_object(String::from("Test"));
    unsafe {
        mgr.free_object(obj);
    }
}
```


## 🧱 4. SafeMemMgr 샘플
### 📦 사용 예제
```rust
use memmgr::SafeMemMgr;

fn main() {
    let mgr = SafeMemMgr::new(64, 16);
    let obj = mgr.alloc_object(String::from("Tracked"));
    unsafe {
        mgr.free_object(obj);
    }
    mgr.free_all_objects(); // 모든 객체 해제
}
```

### ✅ 테스트 코드
```rust
#[test]
fn test_safe_mem_mgr_tracking() {
    let mgr = SafeMemMgr::new(64, 16);
    let obj = mgr.alloc_object(String::from("Tracked"));
    unsafe {
        mgr.free_object(obj);
    }
    mgr.free_all_objects();
}
```


## 🧱 5. SafeMemMgrEx 샘플
### 📦 사용 예제
```rust
use memmgr::SafeMemMgrEx;

fn main() {
    let mgr = SafeMemMgrEx::new();
    let obj1 = mgr.alloc_object("group1", String::from("Tagged A"));
    let obj2 = mgr.alloc_object("group1", String::from("Tagged B"));

    unsafe {
        mgr.free_object("group1", obj1);
    }

    mgr.free_objects_by_tag("group1"); // group1에 속한 나머지 객체 해제
}
```

### ✅ 테스트 코드
```rust
#[test]
fn test_safe_mem_mgr_ex_tagged() {
    let mgr = SafeMemMgrEx::new();
    let obj = mgr.alloc_object("test", String::from("Tagged"));
    unsafe {
        mgr.free_object("test", obj);
    }
    mgr.free_objects_by_tag("test");
}
```

---

# Geom 과 연계

MemoryManager를 Geom의 구조체들과 함께 사용하는 목적은 Point3D, Vector3D, Point4D 같은 수치 구조체들을 빠르게 생성하고 관리하면서 메모리 할당 비용을 줄이고,  
객체 추적 및 해제를 안전하게 처리하기 위함입니다.
아래는 SafeMemMgr를 활용하여 Point3D와 Vector3D 객체를 생성하고,  
연산 후 안전하게 해제하는 실전 예제 코드입니다.

## 🧪 Geom + MemoryManager 연동 샘플

### 코드
```rust
use memmgr::SafeMemMgr;
use geom::{Point3D, Vector3D};
use std::ptr::NonNull;

#[test]
fn point3d_vector3d_test() {
    // 메모리 매니저 생성: Point3D와 Vector3D는 3개의 f64 → 24바이트
    let mem_mgr = SafeMemMgr::new(24, 64); // 블록 크기 24, 청크당 64개

    // Point3D 객체 생성
    let p1 = mem_mgr.alloc_object(Point3D::new(1.0, 2.0, 3.0));
    let p2 = mem_mgr.alloc_object(Point3D::new(4.0, 5.0, 6.0));

    // Vector3D 객체 생성
    let v1;
    unsafe {
        v1 = mem_mgr.alloc_object(Vector3D::from_points( p1.as_ref() , p2.as_ref() ));
    }

    // 연산 수행
    let dot;
    unsafe {
        dot = v1.as_ref().dot(v1.as_ref());
    }

    println!("Dot product of v1 with itself: {}", dot);

    // 객체 해제
    unsafe {
        mem_mgr.free_object(p1);
        mem_mgr.free_object(p2);
        mem_mgr.free_object(v1);
    }

    // 전체 객체 해제 (추적된 객체가 있다면)
    mem_mgr.free_all_objects();
}
```
### 출력
```
Dot product of v1 with itself: 27
```

### ✅ 테스트 코드 예시
```rust
#[test]
fn test_geom_with_memory_manager() {
    let mgr = SafeMemMgr::new(24, 1025);
    let p1 = mgr.alloc_object(Point3D::new(0.0, 0.0, 0.0));
    let p2 = mgr.alloc_object(Point3D::new(1.0, 0.0, 0.0));

    let v;
    unsafe {
        v = mgr.alloc_object(Vector3D::from_points(&p1.as_ref(), &p2.as_ref()));

        println!("v.as_ref().length(): {:?}", v.as_ref().length());
        assert!(v.as_ref().length() > 0.0);
    }

    unsafe {
        mgr.free_object(p1);
        mgr.free_object(p2);
        mgr.free_object(v);
    }
}
```
### 출력
```
v.as_ref().length(): 1.0
```


### 🎯 이 예제의 핵심 포인트

| 기능                         | 설명                                                                 |
|------------------------------|----------------------------------------------------------------------|
| `SafeMemMgr::alloc_object<T>` | 메모리 풀에서 객체를 생성하고 추적. `placement new` 방식으로 안전하게 생성 |
| `free_object<T>`             | 객체를 소멸시키고 해당 블록을 메모리 풀에 반환. double-free 방지 포함     |
| `Vector3D::from_points`      | 두 `Point3D`를 이용해 방향 벡터 생성. 기하학적 연산의 기본 구성 요소       |
| `dot`, `length`              | 벡터의 내적 및 길이 계산. 수치적 연산의 정확도 검증에 사용됨              |
| `free_all_objects`           | 추적된 모든 객체를 소멸시키고 메모리 반환. 일괄 해제 기능 제공             |


## 🧪 Geom 구조체 연산 + MemoryManager 샘플
```rust
use memmgr::SafeMemMgr;
use geom::{Point3D, Point4D, Matrix4, Segment3D, Vector3D};
use std::ptr::NonNull;

fn main() {
    // 메모리 매니저 생성 (Point4D는 4×f64 = 32바이트)
    let mgr = SafeMemMgr::new(32, 64);

    // Point4D 두 개 생성
    let p4a = mgr.alloc_object(Point4D::new(1.0, 2.0, 3.0, 1.0));
    let p4b = mgr.alloc_object(Point4D::new(4.0, 5.0, 6.0, 1.0));

    // 두 점 사이 보간
    let mid = Point4D::h_lerp(&p4a.as_ref(), &p4b.as_ref(), 0.5);
    println!("Midpoint (Point4D): {:?}", mid);

    // Matrix4 변환 행렬 생성 (단순 스케일 행렬 예시)
    let scale_matrix = Matrix4::new_scaling(2.0);
    let transformed = p4a.as_ref().transform(&scale_matrix);
    println!("Transformed Point4D: {:?}", transformed);

    // Segment3D 생성
    let p1 = Point3D::new(0.0, 0.0, 0.0);
    let p2 = Point3D::new(1.0, 1.0, 1.0);
    let segment = Segment3D::new(p1, p2);

    // 외부 점 생성 및 투영
    let external = Point3D::new(1.0, 0.0, 0.0);
    let projected = external.project_to_segment(&segment);
    println!("Projected Point3D: {:?}", projected);

    // 메모리 해제
    unsafe {
        mgr.free_object(p4a);
        mgr.free_object(p4b);
    }
    mgr.free_all_objects();
}
```
### 출력
```
Midpoint (Point4D): Point4D { x: 2.5, y: 3.5, z: 4.5, w: 1.0 }
Transformed Point4D: Point4D { x: 2.0, y: 4.0, z: 6.0, w: 1.0 }
Projected Point3D: Point3D { x: 0.3333333333333333, y: 0.3333333333333333, z: 0.3333333333333333 }
```


### ✅ 테스트 코드 예시
```rust
#[test]
fn test_point4d_matrix_segment_interaction() {
    let mgr = SafeMemMgr::new(32, 32);
    let p4a = mgr.alloc_object(Point4D::new(1.0, 2.0, 3.0, 1.0));
    let p4b = mgr.alloc_object(Point4D::new(4.0, 5.0, 6.0, 1.0));

    let mid;
    unsafe {
        mid = Point4D::h_lerp(&p4a.as_ref(), &p4b.as_ref(), 0.5);
    }

    assert_eq!(mid.x, 2.5);
    assert_eq!(mid.w, 1.0);

    let scale = Matrix4::new_scaling(2.0);
    let transformed;
    unsafe {
        transformed = p4a.as_ref().transform(&scale);
    }

    assert_eq!(transformed.x, 2.0);

    let seg = Segment3D::new(Point3D::new(0.0, 0.0, 0.0), Point3D::new(0.0, 1.0, 0.0));
    let pt = Point3D::new(1.0, 0.5, 0.0);
    let proj = pt.project_to_segment(&seg);
    assert!(proj.y >= 0.0 && proj.y <= 1.0);

    unsafe {
        mgr.free_object(p4a);
        mgr.free_object(p4b);
    }
    mgr.free_all_objects();
}
```


### 📌 요약

| 기능                        | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| `Point4D::h_lerp`           | 두 Point4D 사이를 선형 보간하여 중간 점을 계산함 (homogeneous interpolation) |
| `Matrix4::new_scaling`      | 3D 공간에서 스케일 행렬을 생성함. 각 축 방향으로 크기 조절 가능             |
| `Point4D::transform`        | Point4D를 4×4 행렬(Matrix4)을 통해 변환. 회전, 이동, 스케일 등을 적용         |
| `Segment3D::new`            | 두 Point3D를 연결하여 3D 선분(Segment3D)을 생성함                          |
| `Point3D::project_to_segment` | 외부 Point3D를 주어진 Segment3D에 수직으로 투영하여 가장 가까운 점을 계산함 |


###   고차원 변환 + 배열 할당 예제
```rust
use memmgr::SafeMemMgr;
use geom::{Point3D, Point4D, Matrix4, Vector3D};
use std::ptr::NonNull;

#[test]
fn safe_mem_mgr_geom_test_case2() {
    // Point4D는 4×f64 = 32바이트 → 블록 크기 32, 청크당 16개
    let mgr = SafeMemMgr::new(32, 16);

    // ✅ 1. 고차원 변환: Point4D → Matrix4 변환 적용
    let p4 = mgr.alloc_object(Point4D::new(1.0, 2.0, 3.0, 1.0));
    let scale = Matrix4::new_scaling(2.0);
    let rotated;
    unsafe {
        rotated = p4.as_ref().transform(&scale);
    }

    println!("Transformed Point4D: {:?}", rotated);

    // ✅ 2. Point4D → Point3D로 투영
    let euclid = rotated.to_point();
    println!("Projected to Point3D: {:?}", euclid);

    // ✅ 3. 배열 할당: Point4D 10개를 메모리 풀에서 할당
    let mut points: Vec<NonNull<Point4D>> = Vec::new();
    for i in 0..10 {
        let pt = mgr.alloc_object(Point4D::new(i as f64, i as f64 * 2.0, i as f64 * 3.0, 1.0));
        points.push(pt);
    }

    // ✅ 4. 배열 순회하며 변환 적용
    let translation = Matrix4::translation(10.0, 0.0, 0.0);
    for pt in &points {
        let p;
        unsafe {
            p = pt.as_ref();
        }
        let moved = p.transform(&translation);
        println!("Moved Point4D: {:?}", moved);
    }

    // ✅ 5. 메모리 해제
    unsafe {
        mgr.free_object(p4);
        for pt in points {
            mgr.free_object(pt);
        }
    }
    mgr.free_all_objects();
}
```


### ✅ 테스트 코드 예시
```rust
#[test]
fn test_high_dimensional_transform_and_array_alloc() {
    let mgr = SafeMemMgr::new(32, 16);
    let p = mgr.alloc_object(Point4D::new(1.0, 1.0, 1.0, 1.0));
    let m = Matrix4::new_scaling(3.0);
    let t = p.as_ref().transform(&m);
    assert_eq!(t.x, 3.0);
    assert_eq!(t.w, 1.0);

    // 배열 할당
    let mut arr = Vec::new();
    for i in 0..5 {
        let pt = mgr.alloc_object(Point4D::new(i as f64, 0.0, 0.0, 1.0));
        arr.push(pt);
    }

    assert_eq!(arr.len(), 5);

    unsafe {
        mgr.free_object(p);
        for pt in arr {
            mgr.free_object(pt);
        }
    }
}
```


### 📌 요약

| 기능                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `Point4D::transform`          | 4D 점(Point4D)에 행렬(Matrix4)을 적용하여 회전, 이동, 스케일 등의 변환 수행 |
| `Point4D::to_point()`         | 동차 좌표(Point4D)를 3D 유클리드 좌표(Point3D)로 투영                   |
| `SafeMemMgr::alloc_object<T>` | 메모리 풀에서 객체를 안전하게 생성하고 추적                            |
| `Vec<NonNull<T>>`             | 여러 객체를 배열 형태로 안전하게 저장하고 반복 처리 가능                |
| `Matrix4::new_translation`    | 3D 공간에서 이동 변환 행렬 생성                                        |
| `free_object`, `free_all_objects` | 개별 또는 전체 객체를 소멸시키고 메모리 풀에 반환                        |



## 🧪 배열 할당 + 슬라이스 사용 예제
```rust
use memmgr::SafeMemMgr;
use geom::Point3D;
use std::ptr::{self, NonNull};

#[test]
fn safe_mem_mgr_array_test() {
    // Point3D는 3×f64 = 24바이트 → 블록 크기 24
    let mgr = SafeMemMgr::new(24, 64);

    // ✅ 1. 배열 크기 설정
    const N: usize = 5;

    // ✅ 2. 배열 포인터 생성
    let mut ptrs: [NonNull<Point3D>; N] = [
        mgr.alloc_object(Point3D::new(1.0, 0.0, 0.0)),
        mgr.alloc_object(Point3D::new(0.0, 1.0, 0.0)),
        mgr.alloc_object(Point3D::new(0.0, 0.0, 1.0)),
        mgr.alloc_object(Point3D::new(1.0, 1.0, 0.0)),
        mgr.alloc_object(Point3D::new(0.0, 1.0, 1.0)),
    ];

    // ✅ 3. 슬라이스로 접근
    let slice: &[NonNull<Point3D>] = &ptrs;
    for (i, p) in slice.iter().enumerate() {

        let pt;
        unsafe {
            pt = p.as_ref();
        }
        println!("Point {}: ({:.1}, {:.1}, {:.1})", i, pt.x, pt.y, pt.z);
    }

    // ✅ 4. 슬라이스 일부만 처리
    let sub_slice = &slice[1..4];
    let sum;
    unsafe {
        sum = sub_slice.iter().fold(Point3D::zero(), |acc, p| acc.add(p.as_ref()));
    }

    println!("Sum of sub-slice: {:?}", sum);

    // ✅ 5. 메모리 해제
    unsafe {
        for p in ptrs {
            mgr.free_object(p);
        }
    }
    mgr.free_all_objects();
}
```


### ✅ 테스트 코드 예시
```rust
#[test]
fn test_array_allocation_and_slice_usage() {
    let mgr = SafeMemMgr::new(24, 16);
    const N: usize = 3;
    let ptrs: [NonNull<Point3D>; N] = [
        mgr.alloc_object(Point3D::new(1.0, 0.0, 0.0)),
        mgr.alloc_object(Point3D::new(0.0, 1.0, 0.0)),
        mgr.alloc_object(Point3D::new(0.0, 0.0, 1.0)),
    ];

    let slice = &ptrs;
    let total;
    unsafe {
        total = slice.iter().fold(Point3D::zero(), |acc, p| acc.add(p.as_ref()));
    }

    assert!(total.x > 0.0 && total.y > 0.0 && total.z > 0.0);

    unsafe {
        for p in ptrs {
            mgr.free_object(p);
        }
    }
    mgr.free_all_objects();
}
```

### 📌 요약

| 기능                      | 설명                                                                 |
|---------------------------|----------------------------------------------------------------------|
| `[NonNull<T>; N]`         | 고정 크기 배열에 객체 포인터를 저장. 메모리 풀에서 직접 할당한 객체들을 담음 |
| `&[NonNull<T>]`           | 배열을 슬라이스로 변환하여 반복 처리, 부분 접근, 연산 등에 활용           |
| `iter().fold()`           | 슬라이스를 순회하며 누적 연산 수행. 예: 좌표 합산, 평균 계산 등            |
| `mgr.alloc_object()`      | 메모리 풀에서 객체를 생성하고 포인터 반환. `placement new` 방식 사용       |
| `mgr.free_object()`       | 개별 객체를 소멸시키고 메모리 풀에 반환. double-free 방지 포함             |
| `mgr.free_all_objects()`  | 추적된 모든 객체를 일괄 해제. 메모리 풀 정리 및 안전한 자원 관리            |


---

# i32, f64 배열 활용

SafeMemMgr를 활용해 i32와 f64 배열을 메모리 풀에서 직접 할당하고 슬라이스로 사용하는 예제입니다.  
이 방식은 Rust의 placement new 스타일로 메모리 풀에 원시 타입 배열을 배치하고, 슬라이스로 안전하게  
접근하는 패턴을 보여줍니다.

## 🧪 i32 / f64 배열 할당 + 슬라이스 사용 예제
```rust
use memmgr::SafeMemMgr;
use std::ptr::{self, NonNull};

fn main() {
    // ✅ 1. i32 배열 할당: 10개 → 10 × 4 = 40바이트
    let i32_mgr = SafeMemMgr::new(40, 16);
    let i32_ptr = i32_mgr.alloc_object([0i32; 10]); // 초기화된 배열

    // ✅ 2. 슬라이스로 접근 및 수정
    let slice: &mut [i32; 10] = unsafe { i32_ptr.as_ptr().as_mut().unwrap() };
    for i in 0..slice.len() {
        slice[i] = (i * i) as i32;
    }

    println!("i32 배열 내용:");
    for val in slice.iter() {
        println!("  {}", val);
    }

    // ✅ 3. f64 배열 할당: 5개 → 5 × 8 = 40바이트
    let f64_mgr = SafeMemMgr::new(40, 16);
    let f64_ptr = f64_mgr.alloc_object([0.0f64; 5]);

    let f_slice: &mut [f64; 5] = unsafe { f64_ptr.as_ptr().as_mut().unwrap() };
    for i in 0..f_slice.len() {
        f_slice[i] = (i as f64).sqrt();
    }

    println!("f64 배열 내용:");
    for val in f_slice.iter() {
        println!("  {:.3}", val);
    }

    // ✅ 4. 메모리 해제
    unsafe {
        i32_mgr.free_object(i32_ptr);
        f64_mgr.free_object(f64_ptr);
    }

    i32_mgr.free_all_objects();
    f64_mgr.free_all_objects();
}
```


### ✅ 테스트 코드 예시
```rust
#[test]
fn test_i32_f64_array_allocation() {
    let i32_mgr = SafeMemMgr::new(40, 8);
    let f64_mgr = SafeMemMgr::new(40, 8);

    let i32_ptr = i32_mgr.alloc_object([1i32; 10]);
    let f64_ptr = f64_mgr.alloc_object([2.0f64; 5]);

    let i_slice: &[i32; 10] = unsafe { i32_ptr.as_ptr().as_ref().unwrap() };
    let f_slice: &[f64; 5] = unsafe { f64_ptr.as_ptr().as_ref().unwrap() };

    assert_eq!(i_slice[0], 1);
    assert_eq!(f_slice[0], 2.0);

    unsafe {
        i32_mgr.free_object(i32_ptr);
        f64_mgr.free_object(f64_ptr);
    }

    i32_mgr.free_all_objects();
    f64_mgr.free_all_objects();
}
```


### 📌 요약

| 기능                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `SafeMemMgr::alloc_object([T; N])` | 고정 크기 배열을 메모리 풀에 배치 생성. `[T; N]` 타입을 직접 할당함         |
| `NonNull<[T; N]>`             | 배열 포인터를 안전하게 저장. 슬라이스로 접근 가능                         |
| `as_ptr().as_mut()`           | 포인터를 슬라이스로 변환하여 읽기/쓰기 가능. 배열 요소 수정에 사용됨         |
| `free_object()`               | 개별 배열 객체를 소멸시키고 메모리 풀에 반환. 안전한 해제 처리 포함           |
| `free_all_objects()`          | 추적된 모든 배열 객체를 일괄 해제. 메모리 풀 정리 및 자원 관리에 유용         |

---

## ✅ Vec<T> vs Box<[T]> 성능 비교 요약:  
Vec<T>은 유연하고 기능이 많지만, Box<[T]>는 메모리 오버헤드가 적고 고정 크기 배열에 적합합니다.  
다차원 배열은 ndarray 크레이트를 활용하면 안전하고 효율적으로 처리할 수 있습니다.  

### 🚀 Vec<T> vs Box<[T]> 성능 비교  

| 항목               | `Vec<T>`                                | `Box<[T]>`                                 |
|--------------------|------------------------------------------|--------------------------------------------|
| 크기 변경          | `push`, `pop`, `resize` 가능              | 불가능 (고정 크기)                          |
| 메모리 구조        | 포인터 + 길이 + 용량 (3 word)             | 포인터 + 길이 (2 word)                      |
| 성능 특성          | 유연하지만 약간의 오버헤드 있음           | 더 작고 빠름 (특히 고정 크기일 때)          |
| 생성 방식          | `vec![...]`                               | `vec![...].into_boxed_slice()`              |
| 사용 용도          | 동적 배열, 리스트, 버퍼 등                 | 고정 크기 배열, 수치 계산, 캐시 최적화 등    |

- 예시 벤치마크 결과에 따르면, 작은 배열에서는 Vec<T>가 빠르고, 큰 배열에서는 Box<[T]>가 더 빠른 경우도 있음.


## 🧮 다차원 배열 처리: ndarray 크레이트 활용
- Rust에서 다차원 배열을 안전하고 효율적으로 다루려면  크레이트를 사용하는 것이 가장 좋습니다.
## ✨ 주요 기능
- Array1, Array2, Array3 등 다양한 차원 지원
- 슬라이싱, reshape, element-wise 연산
- mapv, dot, sum, mean 등 수치 연산 지원
## 📦 설치
```
[dependencies]
ndarray = "0.15"
```

### 🧪 예제 코드
```rust
use ndarray::Array;

#[test]
fn ndarray_basic() {
    // 1D 배열
    let arr1 = Array::from_vec(vec![1, 2, 3, 4, 5]);
    println!("1D array: {:?}", arr1);

    // 2D 배열
    let arr2 = Array::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();
    println!("2D array: {:?}", arr2);

    // 슬라이싱
    let slice = arr2.slice(ndarray::s![.., 1..]);
    println!("Sliced: {:?}", slice);

    // reshape
    let reshaped = arr2.into_shape((3, 2)).unwrap();
    println!("Reshaped: {:?}", reshaped);
}
```


### 🧩 코드 전체 맥락
use ndarray::Array;

fn main() {
    let arr2 = Array::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6]).unwrap();
    let slice = arr2.slice(ndarray::s![.., 1..]);
    println!("Sliced: {:?}", slice);
}



### 🔍 단계별 설명
#### ① arr2 생성
```rust
Array::from_shape_vec((2, 3), vec![1, 2, 3, 4, 5, 6])
```
- arr2는 2행 3열짜리 배열입니다.
- 내부 데이터는 행 기준으로 채워집니다:
```
[[1, 2, 3],
 [4, 5, 6]]
```


#### ② 슬라이싱 구문
```rust
arr2.slice(ndarray::s![.., 1..])
```

- 이 부분이 핵심입니다. ndarray::s![.., 1..]는 슬라이스 매크로로, 행과 열을 각각 어떻게 자를지 지정합니다.

#### 📌 슬라이스 매크로 구성 설명 (`ndarray::s![.., 1..]`)

| 표현     | 의미                                      |
|----------|-------------------------------------------|
| `..`     | 모든 행 선택 (0부터 끝까지)                |
| `1..`    | 열 인덱스 1부터 끝까지 선택 (1열, 2열 등)  |

- 즉, 이 슬라이스는 다음을 선택합니다:
```
[[2, 3],
 [5, 6]]
```
#### ③ 결과 출력
```rust
println!("Sliced: {:?}", slice);
```

- 출력 결과는:
```
[[2, 3],
 [5, 6]]
```

#### 📌 요약
| 코드                         | 설명                                                                 |
|------------------------------|----------------------------------------------------------------------|
| `Array::from_shape_vec`      | shape과 벡터 데이터를 기반으로 다차원 배열 생성 (`Array2`, `Array3` 등)       |
| `ndarray::s![.., 1..]`       | 슬라이스 매크로. 모든 행(`..`)과 열 1부터 끝까지(`1..`) 선택               |
| `arr2.slice(...)`            | 지정된 슬라이스 범위로 배열 뷰 생성. 원본 배열은 변경되지 않음              |
| `println!("{:?}", slice)`    | 슬라이스된 배열 뷰를 디버그 형식으로 출력                                 |


### 📌 요약

| 기능                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `Vec<T>`                      | 동적 배열. 크기 변경 가능하며 유연하지만 메모리 오버헤드가 있음             |
| `Box<[T]>`                    | 고정 크기 힙 배열. 메모리 구조가 작고 빠르며 수치 계산에 적합               |
| `vec![...].into_boxed_slice()`| `Box<[T]>` 생성 방법. `Vec<T>`를 고정 배열로 변환                         |
| `ndarray::Array`              | 다차원 배열 처리용 크레이트. `Array1`, `Array2` 등 다양한 차원 지원         |
| `slice`, `reshape`, `mapv`    | `ndarray`에서 슬라이싱, 형태 변경, 요소별 연산을 수행하는 주요 메서드       |


# SafeMemMgr + ndarray
아래는 SafeMemMgr를 활용해 ndarray의 다차원 배열을 메모리 풀에서 직접 생성하고 사용하는 예제입니다.  
이 방식은 메모리 풀을 통해 배열 데이터를 안전하게 관리하면서, ndarray의 고급 수치 연산 기능을 활용하는 패턴을 보여줍니다.

## 🧪 SafeMemMgr × ndarray 다차원 배열 예제
```rust
use memmgr::SafeMemMgr;
use ndarray::{ArrayViewMut2, ShapeBuilder};
use std::ptr::NonNull;

#[test]
fn safe_mem_mgr_ndarray_test() {
    // ✅ 1. 메모리 매니저 생성: f64 3×3 배열 → 9 × 8 = 72바이트
    let mgr = SafeMemMgr::new(72, 16);

    // ✅ 2. 배열 데이터 할당
    let data_ptr = mgr.alloc_object([0.0f64; 9]); // 3×3 배열용 데이터

    // ✅ 3. ndarray 뷰 생성
    let raw_slice: &mut [f64; 9] = unsafe { data_ptr.as_ptr().as_mut().unwrap() };
    let mut view: ArrayViewMut2<f64> = ArrayViewMut2::from_shape((3, 3).strides((3, 1)), raw_slice).unwrap();

    // ✅ 4. 값 설정 및 연산
    for ((i, j), val) in view.indexed_iter_mut() {
        *val = (i + j) as f64;
    }

    println!("3×3 배열 내용:");
    println!("{:?}", view);

    // ✅ 5. 메모리 해제
    unsafe {
        mgr.free_object(data_ptr);
    }
    mgr.free_all_objects();
}
```

### 출력
```
3×3 배열 내용:
[[0.0, 1.0, 2.0],
 [1.0, 2.0, 3.0],
 [2.0, 3.0, 4.0]], shape=[3, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2
```


###  ✅ 테스트 코드 예시
```rust
#[test]
fn test_ndarray_with_memory_pool() {
    use ndarray::ArrayViewMut2;

    let mgr = SafeMemMgr::new(72, 8);
    let ptr = mgr.alloc_object([0.0f64; 9]);

    let slice: &mut [f64; 9] = unsafe { ptr.as_ptr().as_mut().unwrap() };
    let mut view = ArrayViewMut2::from_shape((3, 3).strides((3, 1)), slice).unwrap();

    view[[0, 0]] = 1.0;
    view[[1, 1]] = 2.0;
    view[[2, 2]] = 3.0;

    assert_eq!(view[[1, 1]], 2.0);

    unsafe {
        mgr.free_object(ptr);
    }
    mgr.free_all_objects();
}

```
### 📌 요약

| 기능                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `SafeMemMgr::alloc_object([f64; N])` | 고정 크기 f64 배열을 메모리 풀에서 안전하게 할당. ndarray 배열 데이터로 사용 가능 |
| `ArrayViewMut2::from_shape()` | 슬라이스를 2차원 ndarray 뷰로 변환. shape과 stride 지정 가능               |
| `view[[i, j]] = val`          | ndarray 스타일의 인덱싱으로 요소 값 설정. 반복, 연산, 시각화에 활용 가능     |
| `free_object()`               | 개별 배열 객체를 소멸시키고 메모리 풀에 반환. 안전한 해제 처리 포함           |
| `free_all_objects()`          | 추적된 모든 배열 객체를 일괄 해제. 메모리 풀 정리 및 자원 관리에 유용         |

---
