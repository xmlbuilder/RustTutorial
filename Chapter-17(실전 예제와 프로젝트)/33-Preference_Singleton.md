# Preference Singleton

Rust는 전역 상태를 기본적으로 지양하지만, 설정값이나 환경값처럼 읽기/쓰기 가능한 전역 싱글톤이 필요한 경우가 있습니다.  
이때 스레드 안전성을 유지하면서도 간결하고 현대적인 방식으로 구현하는 방법을 소개합니다.  


## ✅ 기본 방식: OnceLock + RwLock 조합
Rust 1.70부터 안정화된 은 초기화 시점 제어를 위한 도구입니다.  
여기에 RwLock을 결합하면 읽기/쓰기 락을 통한 동시성 제어가 가능합니다.  

```rust
//! Thread-safe global Preferences singleton.

use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone)]
pub struct Preferences {
    global_mesh: f64,
    local_mesh_sizes: Vec<f64>,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            global_mesh: 0.0,
            local_mesh_sizes: Vec::new(),
        }
    }
}

/// The singleton storage (initialized on first use).
static PREFS: OnceLock<RwLock<Preferences>> = OnceLock::new();

/// Get the global RwLock<Preferences>, initializing if needed.
#[inline]
fn prefs_lock() -> &'static RwLock<Preferences> {
    PREFS.get_or_init(|| RwLock::new(Preferences::default()))
}

/* ---------- Public API (copy-based, ergonomic) ---------- */

/// Get current global_mesh (copy).
#[inline]
pub fn get_global_mesh() -> f64 {
    let guard = prefs_lock().read().expect("poisoned RwLock");
    guard.global_mesh
}

/// Set global_mesh.
#[inline]
pub fn set_global_mesh(val: f64) {
    let mut guard = prefs_lock().write().expect("poisoned RwLock");
    guard.global_mesh = val;
}

/// Replace entire local_mesh_sizes vector.
#[inline]
pub fn set_local_mesh_sizes(v: Vec<f64>) {
    let mut guard = prefs_lock().write().expect("poisoned RwLock");
    guard.local_mesh_sizes = v;
}

/// Get a cloned copy of local_mesh_sizes.
#[inline]
pub fn get_local_mesh_sizes() -> Vec<f64> {
    let guard = prefs_lock().read().expect("poisoned RwLock");
    guard.local_mesh_sizes.clone()
}

/// Append one size to local_mesh_sizes.
#[inline]
pub fn push_local_mesh_size(size: f64) {
    let mut guard = prefs_lock().write().expect("poisoned RwLock");
    guard.local_mesh_sizes.push(size);
}

/// Clear local_mesh_sizes.
#[inline]
pub fn clear_local_mesh_sizes() {
    let mut guard = prefs_lock().write().expect("poisoned RwLock");
    guard.local_mesh_sizes.clear();
}

/* ---------- Power users: scoped read/write access ---------- */

/// Execute a read-only closure with &Preferences.
#[inline]
pub fn with_read<F, R>(f: F) -> R
where
    F: FnOnce(&Preferences) -> R,
{
    let guard = prefs_lock().read().expect("poisoned RwLock");
    f(&*guard)
}

/// Execute a write closure with &mut Preferences.
#[inline]
pub fn with_write<F, R>(f: F) -> R
where
    F: FnOnce(&mut Preferences) -> R,
{
    let mut guard = prefs_lock().write().expect("poisoned RwLock");
    f(&mut *guard)
}

/* ---------- Optional: initialization helpers ---------- */

/// Reinitialize preferences to default (use carefully).
#[inline]
pub fn reset_to_default() {
    let mut guard = prefs_lock().write().expect("poisoned RwLock");
    *guard = Preferences::default();
}

/// Initialize with specific values (idempotent-style setter).
#[inline]
pub fn init_if_unset(global_mesh: f64, locals: Vec<f64>) {
    // Ensures OnceLock is created, then set fields.
    let mut guard = prefs_lock().write().expect("poisoned RwLock");
    guard.global_mesh = global_mesh;
    guard.local_mesh_sizes = locals;
}



mod prefs;
use prefs::*;

fn main() {
    // 초기화
    init_if_unset(5.0, vec![0.5, 1.0, 2.0]);

    // 읽기
    let g = get_global_mesh();
    println!("global_mesh = {g}");

    // 쓰기
    set_global_mesh(7.5);
    push_local_mesh_size(3.0);

    // 일괄 교체
    set_local_mesh_sizes(vec![1.2, 1.6, 2.4]);

    // 고급: 클로저로 직접 접근
    with_write(|p| {
        p.global_mesh *= 2.0;
        p.local_mesh_sizes.retain(|&x| x >= 1.5);
    });

    let locals = get_local_mesh_sizes();
    println!("local_mesh_sizes = {locals:?}");
}
```

## 핵심 구조
```rust
static PREFS: OnceLock<RwLock<Preferences>> = OnceLock::new();

fn prefs_lock() -> &'static RwLock<Preferences> {
    PREFS.get_or_init(|| RwLock::new(Preferences::default()))
}
```
- OnceLock은 최초 접근 시 한 번만 초기화됨
- RwLock은 읽기 다중, 쓰기 단일 락을 제공
- API는 get_*, set_*, with_read, with_write 등으로 구성
### 장점
- 표준 라이브러리만 사용 (추가 의존성 없음)
- 초기화 시점 제어 가능
- 락을 직접 제어할 수 있어 유연함
### 단점
- 호출자가 read() / write()를 직접 호출해야 하므로 락 처리에 대한 이해가 필요

---

# once_cell / parking_lot 을 이용

`once_cell` 의 `sync::Lazy` 를 써서 초기화/락을 전부 내부로 숨긴 전역 싱글톤을 만듬.  
호출자는 `get_*`/`set_*` 같은 함수만 쓰면 되고, 직접 `lock`/`unlock` 할 일 없음.

## Cargo.toml
```
[dependencies]
once_cell = "1.19"
# 선택: 더 빠르고 poison 없음. std RwLock 대신 쓰고 싶다면:
parking_lot = "0.12"
```

## 🚀 개선 방식: once_cell::sync::Lazy + RwLock
Lazy는 OnceLock보다 더 간결한 API를 제공합니다.  
초기화와 전역 접근을 한 줄로 처리할 수 있어 실용적입니다.

- src/prefs.rs
```rust
use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Preferences {
    global_mesh: f64,
    local_mesh_sizes: Vec<f64>,
}

impl Default for Preferences {
    fn default() -> Self {
        Self { global_mesh: 0.0, local_mesh_sizes: Vec::new() }
    }
}

// 전역 싱글톤 (Lazy가 최초 사용 시 1회 초기화)
static PREFS: Lazy<RwLock<Preferences>> = Lazy::new(|| RwLock::new(Preferences::default()));

#[inline] fn prefs() -> &'static RwLock<Preferences> { &PREFS }

/* ---- 바깥에서 락을 몰라도 되는 API ---- */

#[inline]
pub fn get_global_mesh() -> f64 {
    prefs().read().unwrap().global_mesh
}

#[inline]
pub fn set_global_mesh(val: f64) {
    prefs().write().unwrap().global_mesh = val;
}

#[inline]
pub fn get_local_mesh_sizes() -> Vec<f64> {
    prefs().read().unwrap().local_mesh_sizes.clone()
}

#[inline]
pub fn set_local_mesh_sizes(v: Vec<f64>) {
    prefs().write().unwrap().local_mesh_sizes = v;
}

#[inline]
pub fn push_local_mesh_size(size: f64) {
    prefs().write().unwrap().local_mesh_sizes.push(size);
}

#[inline]
pub fn clear_local_mesh_sizes() {
    prefs().write().unwrap().local_mesh_sizes.clear();
}

/* ---- 필요 시 클로저 접근(락을 여전히 숨김) ---- */

#[inline]
pub fn with_read<F, R>(f: F) -> R
where F: FnOnce(&Preferences) -> R {
    let g = prefs().read().unwrap();
    f(&*g)
}

#[inline]
pub fn with_write<F, R>(f: F) -> R
where F: FnOnce(&mut Preferences) -> R {
    let mut g = prefs().write().unwrap();
    f(&mut *g)
}

/* ---- 리셋/초기화 ---- */

#[inline]
pub fn reset_to_default() {
    *prefs().write().unwrap() = Preferences::default();
}

#[inline]
pub fn init_if_unset(global_mesh: f64, locals: Vec<f64>) {
    let mut g = prefs().write().unwrap();
    if g.local_mesh_sizes.is_empty() && g.global_mesh == 0.0 {
        g.global_mesh = global_mesh;
        g.local_mesh_sizes = locals;
    }
}
```
## 기본형 구현
```rust
static PREFS: Lazy<RwLock<Preferences>> = Lazy::new(|| RwLock::new(Preferences::default()));
```
- Lazy는 최초 접근 시 자동 초기화
- prefs().read().unwrap() / prefs().write().unwrap()으로 접근

### 장점
- 락을 숨긴 API 설계 가능 → 호출자는 get_*, set_*만 사용
- 초기화가 더 간단하고 직관적

### 단점
- unwrap() 사용으로 락 poison 처리에 대한 방어가 약함
- 여전히 std::sync::RwLock은 성능이 다소 낮고 panic 시 poison 발생

---

## ⚡ 고급형: once_cell::Lazy + parking_lot::RwLock
고성능 락 구현체로, std::sync::RwLock보다 빠르고 poison-free입니다.  

- src/prefs.rs
```rust
use once_cell::sync::Lazy;
use parking_lot::RwLock;

#[derive(Debug, Clone, Default)]
pub struct Preferences {
    pub global_mesh: f64,
    pub local_mesh_sizes: Vec<f64>,
}

static PREFS: Lazy<RwLock<Preferences>> = Lazy::new(|| RwLock::new(Preferences::default()));
#[inline] fn prefs() -> &'static RwLock<Preferences> { &PREFS }

pub fn get_global_mesh() -> f64 { prefs().read().global_mesh }
pub fn set_global_mesh(v: f64) { prefs().write().global_mesh = v; }

pub fn get_local_mesh_sizes() -> Vec<f64> { prefs().read().local_mesh_sizes.clone() }
pub fn set_local_mesh_sizes(v: Vec<f64>) { prefs().write().local_mesh_sizes = v; }
pub fn push_local_mesh_size(x: f64) { prefs().write().local_mesh_sizes.push(x); }
pub fn clear_local_mesh_sizes() { prefs().write().local_mesh_sizes.clear(); }

pub fn with_read<F, R>(f: F) -> R where F: FnOnce(&Preferences)->R { f(&*prefs().read()) }
pub fn with_write<F, R>(f: F) -> R where F: FnOnce(&mut Preferences)->R { f(&mut *prefs().write()) }

pub fn reset_to_default() { *prefs().write() = Preferences::default(); }
```

### (옵션) 더 편한 쓰기: global_mesh만 원자형으로
global_mesh는 잦은 갱신이 예상되면 AtomicF64(안전한 사용을 위해 load/store에 Ordering)로 바꾸면 RwLock 없이도 빠르게 업데이트 가능.  
다만 Vec는 여전히 락 필요.
```rust
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicF64, Ordering};

#[derive(Debug, Default)]
struct Preferences {
    local_mesh_sizes: Vec<f64>,
}

static GLOBAL_MESH: AtomicF64 = AtomicF64::new(0.0);
static PREFS: Lazy<RwLock<Preferences>> = Lazy::new(|| RwLock::new(Preferences::default()));

pub fn get_global_mesh() -> f64 { GLOBAL_MESH.load(Ordering::Relaxed) }
pub fn set_global_mesh(v: f64) { GLOBAL_MESH.store(v, Ordering::Relaxed) }

pub fn get_local_mesh_sizes() -> Vec<f64> { PREFS.read().local_mesh_sizes.clone() }
pub fn set_local_mesh_sizes(v: Vec<f64>) { PREFS.write().local_mesh_sizes = v; }
pub fn push_local_mesh_size(x: f64) { PREFS.write().local_mesh_sizes.push(x); }
```

원자형은 락 프리라 편하지만, fetch_add 같은 연산에서 일관성 수준(Ordering)을 선택해야 함.  
대부분 Relaxed로 충분하고, 교차 스레드 가시성이 중요하면 SeqCst를 쓰면 돼.

### 사용 예
```rust
use your_crate::prefs::*;

fn main() {
    set_global_mesh(5.0);
    push_local_mesh_size(0.5);
    push_local_mesh_size(1.0);

    with_write(|p| {
        // 여전히 락은 내부에서만 사용됨
        p.local_mesh_sizes.retain(|&x| x >= 0.8);
    });

    println!("gm = {}", get_global_mesh());
    println!("locals = {:?}", get_local_mesh_sizes());
}
```
### 구현 예시
```rust
static PREFS: Lazy<RwLock<Preferences>> = Lazy::new(|| RwLock::new(Preferences::default()));
```
- parking_lot::RwLock은 .read() / .write()에서 unwrap() 없이 사용 가능
- API는 동일하지만 성능과 안정성이 향상됨
### 장점
- 락 성능 우수
- panic-safe (poison 없음)
- 코드가 더 간결함

---

### 🧭 결론: 어떤 방식이 가장 좋을까?
| 방식                        | 특징                                      | 장점                                  | 추천 상황                          |
|-----------------------------|-------------------------------------------|----------------------------------------|-------------------------------------|
| OnceLock + RwLock           | 표준 라이브러리만 사용, 직접 락 제어      | 의존성 없음, 초기화 시점 제어 가능     | 락 제어가 필요한 고급 사용자        |
| once_cell::Lazy + RwLock    | 간결한 초기화, unwrap 필요                | 코드 간결, API 숨김 가능               | 일반적인 설정값 관리                |
| Lazy + parking_lot::RwLock  | 빠르고 안전, poison-free                  | 고성능, panic-safe, 코드 더 간결       | 실서비스, 고성능 요구               |
| AtomicF64                   | 단일 값에 대해 락 없이 접근 가능          | 락 프리, 빠름                           | 자주 갱신되는 숫자 필드에 적합      |

---
