## 📌 Rust에서 상수 선언하기 (const)

Rust에서 상수는 프로그램 전체에서 변하지 않는 값을 표현할 때 사용합니다.  
상수를 선언할 때는 반드시 타입을 명시해야 하며, const 키워드를 사용합니다.

const NAME: Type = value;

## ✅ 예제
```csharp
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    println!("{}", THRESHOLD);
    println!("{}", is_big(5));
}
```
## 실행 결과
```
10
false
```

THRESHOLD는 i32 타입 상수이며 값은 10으로 고정됩니다.

is_big(5) → 5가 10보다 크지 않으므로 false.

## ✅ 추가 예제 (PI 상수)
```csharp
const PI: f32 = 3.141592;

fn main() {
    println!("PI : {PI}.");
}
```
실행 결과
PI : 3.141592.


PI는 f32 타입 상수.

소수점이 포함된 실수 상수를 사용할 때는 f32 또는 f64로 지정해야 합니다.

## 📖 정리

- const는 반드시 타입을 지정해야 한다.
- 전역/지역 어디서든 선언 가능하다.
- const는 항상 컴파일 타임에 결정되는 값이어야 한다.
- 변경할 수 없으며, 프로그램 실행 중 값이 달라질 수 없다.

---

# Rust: const vs static (그리고 static mut)

## const

- 컴파일 타임 상수: 값이 항상 컴파일 시에 계산되어 사용 위치에 인라인됩니다.
- 주소(메모리 위치)가 없음: 한 군데에 “놓여 있는 값”이 아니라, 쓰인 곳마다 값이 박힙니다.
- 항상 불변: 바꿀 수 없음.
- 용도: 패턴 매칭, 배열 길이, 제네릭의 const 파라미터 등.

```rust
pub const THRESHOLD: i32 = 10;
fn is_big(n: i32) -> bool { n > THRESHOLD } // 값이 인라인됨
```

## static
**런타임에 단 하나의 인스턴스(고정된 주소)** 가 존재합니다(전역 변수).  
기본은 불변(static 자체는 mut 아님). 내부를 바꾸려면 UnsafeCell/Mutex 등으로 “안쪽 가변성”을 써야 함.    
외부 크레이트에서 심볼로 링크되어 같은 주소를 가리킵니다.  

```rust
pub static ANSWER: i32 = 42;
fn addr() -> *const i32 { &ANSWER as *const i32 } // 항상 같은 주소
```

### static mut (주의!)

전역 가변 변수. 데이터 레이스 위험 → 모든 접근이 unsafe.  
멀티스레드에서 정의상 UB(미정의 동작) 가능. 실무에서는 피해야 하고,  
`static + Mutex/Atomic/OnceLock` 같은 안전 래퍼를 사용.  

```rust
static mut COUNTER: u64 = 0;

fn bump_unsafe() {
    unsafe { COUNTER += 1; } // 안전하지 않음!
}
```

### 안전한 대안들
```rust
use std::sync::{Mutex, OnceLock}; // 1.70+는 OnceLock 안정화  

static LOGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn push_log(s: String) {
    let v = LOGS.get_or_init(|| Mutex::new(Vec::new()));
    v.lock().unwrap().push(s);
}
```

## C#과의 대응 관계
|개념  |시점/바인딩 |	주소 고정 |	변경 가능 |	주 사용법 |
|-----|-----------|---------|-----------|-------------|
| C#  const |	컴파일 타임: 호출 사이트에 값 베이크|	주소 개념 없음|	불가 |	숫자/문자 상수 |
| C# static readonly|런타임: 타입 로드/생성자 시|초기화	고정(타입 당 1개)|필드 자체 재할당 불가(참조/내용은 타입에 따라)|전역 불변(주소는 하나)|

## Rust와 매핑하면:

### C# const ≈ Rust const
둘 다 컴파일 타임 값이 인라인됨.
라이브러리의 값을 바꾸면 소비자 재컴파일이 필요(안 그러면 옛날 값이 박혀 있음).

### C# static readonly ≈ Rust static(불변)
둘 다 런타임에 한 인스턴스/한 주소가 존재.
동적 링크 시, 라이브러리 교체로 값이 달라지면 소비자 재컴파일 없이도 새 값을 볼 여지가 있음(심볼 참조).  
→ Rust에서도 pub static FOO: i32 = 123;를 동적 링크로 가져오면 주소를 통해 접근합니다.  

### C# static 가변 + 락/Interlocked ≈ Rust static + Mutex/Atomic

전역 가변 상태는 락/원자 연산으로 안전하게 관리.  
**Rust static mut**는 C#에 직접 대응 없음. C# 전역 가변이라도 안전 장치를 쓰지만, Rust static mut은 언어 차원에서 unsafe로 표시됩니다.  
“DLL 런타임 때 달라지는가?” (버전 교체/재빌드 관점)  

## C#
const: 호출 사이트에 값이 박히므로 라이브러리의 const 값을 바꾸고 소비자만 교체하면 옛 값이 유지됩니다. 소비자 재컴파일 필요.  
static readonly: 런타임 바인딩이라 라이브러리만 교체해도 새 값이 반영됩니다(참조가 라이브러리 쪽 심볼을 가리킴).  

## Rust
const: 컴파일 타임 인라인. 소비자 크레이트가 재컴파일되어야 새 값이 반영됩니다. (Cargo는 의존 버전이 바뀌면 보통 재빌드합니다.)  
static: 런타임 심볼로 참조(특히 동적 링크 시). 라이브러리 교체로 값이 바뀌면 재컴파일 없이도 새 값이 보일 수 있습니다.  
다만 Rust 생태계는 주로 **정적 링크(rlib → 최종 바이너리로 모노리식 링크)**가 일반적이라, 실무에서 “교체만으로 갱신” 시나리오는 드뭅니다.  

## 요약: 개념적으로는 C#과 동일한 차이가 존재합니다.

“베이크되는 값” = const
“런타임에 한 주소” = static

전역 가변은 static mut 금지, Mutex/Atomic/OnceLock 사용이 권장.

## 보너스: 언제 무엇을 쓰나?

수학 상수, 패턴/배열 크기, 제네릭 const 인자 → const  
전역 테이블/버퍼/캐시(불변 데이터) → static  
전역 초기화가 필요한 리소스(런타임 정보 필요) → static + OnceLock/OnceCell  
전역 가변 카운터/플래그 → static + Atomic* (예: AtomicU64)  
전역 가변 컬렉션 → static + Mutex<T> / RwLock<T>  

---

# 참고 소스

## 1) const — 컴파일 타임 상수
```rust
const THRESHOLD: i32 = 10;
const PI: f32 = 3.141592;

fn main() {
    let n = 7;
    println!("THRESHOLD = {THRESHOLD}");
    println!("PI = {PI}");
    println!("is_big({n}) = {}", n > THRESHOLD);
}
```

## 2) static — 전역 불변, 고정 주소
```rust
static BANNER: &str = "hello, static!";

fn main() {
    let a = BANNER.as_ptr();
    let b = BANNER.as_ptr();
    println!("{BANNER} (same addr? {})", a == b); // 항상 같은 주소
}
```

## 3) static + OnceLock — 런타임 1회 초기화 (안전)

Rust 1.70+ 표준 OnceLock.

```rust
use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static str {
    CONFIG.get_or_init(|| {
        // 파일/환경변수/계산 등 런타임 초기화 가능
        "mode=release;workers=8".to_string()
    })
}

fn main() {
    println!("{}", get_config());
    println!("{}", get_config()); // 재사용
}
```


## 4) static + 원자타입 — 전역 카운터 (락 없이, 스레드 안전)

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static HITS: AtomicU64 = AtomicU64::new(0);

fn hit() {
    HITS.fetch_add(1, Ordering::Relaxed);
}

fn main() {
    for _ in 0..5 { hit(); }
    println!("hits = {}", HITS.load(Ordering::Relaxed));
}
```

## 5) static + Mutex — 전역 컬렉션(가변) 안전하게
```rust
use std::sync::{Mutex, OnceLock};

static LOGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn push_log(msg: impl Into<String>) {
    let m = LOGS.get_or_init(|| Mutex::new(Vec::new()));
    m.lock().unwrap().push(msg.into());
}

fn main() {
    push_log("start");
    push_log("processing");
    push_log("done");
    let logs = LOGS.get().unwrap().lock().unwrap().clone();
    println!("logs = {:?}", logs);
}
```

## 6) static + RwLock — 읽기 많고 쓰기 적을 때
```rust
use std::sync::{RwLock, OnceLock};

static DATA: OnceLock<RwLock<Vec<i32>>> = OnceLock::new();

fn main() {
    let store = DATA.get_or_init(|| RwLock::new(vec![1, 2, 3]));
    {
        let r = store.read().unwrap();
        println!("sum = {}", r.iter().sum::<i32>());
    }
    {
        let mut w = store.write().unwrap();
        w.push(4);
    }
    println!("len = {}", store.read().unwrap().len());
}
```

## 7) (안티패턴) static mut — 피하세요, 대안 포함
// 안티패턴: unsafe 필요 + 데이터 레이스 위험
```rust
static mut COUNTER_BAD: u64 = 0;

fn bump_bad() {
    unsafe { COUNTER_BAD += 1; } // ❌ 피해야 함
}

// 대안: Atomic 사용
use std::sync::atomic::{AtomicU64, Ordering};
static COUNTER_GOOD: AtomicU64 = AtomicU64::new(0);

fn bump_good() {
    COUNTER_GOOD.fetch_add(1, Ordering::Relaxed);
}

fn main() {
    bump_bad(); // 컴파일은 되지만 권장 X
    bump_good();
    println!("good = {}", COUNTER_GOOD.load(Ordering::Relaxed));
}
```
---


---

# 다시 정리
Rust에서 const와 static은 불변성과 메모리 수명에 관련된 중요한 개념입니다.

## 🧠 핵심 개념 요약
| 개념       | 설명                                                         |
|------------|--------------------------------------------------------------|
| `const`    | - 컴파일 타임에 값이 결정됨<br>- 불변이며 복사 가능<br>- 스코프에 따라 수명 결정 |
| `static`   | - 프로그램 전체에서 하나만 존재하는 전역 변수<br>- 기본적으로 `'static` 수명 |
| `'static`  | - 프로그램 전체 수명 동안 유효한 참조<br>- 프로모션된 `const` 값은 `'static` 참조 가능 |
| `static mut` | - 가변 전역 변수<br>- `unsafe` 블록에서만 접근 가능<br>- `const`에서는 `&mut` 참조 불가 |


## 🔍 예제 분석
### 1. Promotion과 'static 수명
```rust
const BIT1: u32 = 1 << 0;
const BIT2: u32 = 1 << 1;
const BITS: [u32; 2] = [BIT1, BIT2];
const STRING: &'static str = "bitstring";
```
- BIT1, BIT2, BITS는 모두 컴파일 타임에 결정되는 상수.
- STRING은 'static 수명을 가진 문자열 리터럴. 문자열 리터럴은 기본적으로 'static입니다.
- 이 값들은 프로모션(promoted) 되어 'static 수명을 가지므로 다른 'static 참조에 안전하게 사용 가능.
### 2. 구조체에 'static 참조 사용
```rust
struct BitsNStrings<'a> {
    mybits: [u32; 2],
    mystring: &'a str,
}

const BITS_N_STRINGS: BitsNStrings<'static> = BitsNStrings {
    mybits: BITS,
    mystring: STRING,
};
```

- BitsNStrings는 'a 수명을 가진 참조를 포함하는 구조체.
- BITS_N_STRINGS는 'static 수명을 가진 상수로 선언되었고, 내부 값들도 모두 'static이므로 문제 없음.

### ⚠️ const에서 mutable 참조는 금지
```rust
static mut S: u8 = 0;
const C: &u8 = unsafe { &mut S }; // ✅ OK: 불변 참조로 사용

static S: AtomicU8 = AtomicU8::new(0);
const C: &AtomicU8 = &S; // ✅ OK: AtomicU8은 내부적으로 안전하게 공유 가능

static mut S: u8 = 0;
const C: &mut u8 = unsafe { &mut S }; // ❌ ERROR: const에는 가변 참조 불가
```

- const는 가변 참조(&mut)를 포함할 수 없습니다. 이는 Rust의 안전성 모델을 위반하기 때문입니다.
- static mut은 unsafe 블록에서 접근 가능하지만, 그 참조를 const로 가변하게 저장하는 것은 금지됩니다.

## ✅ 정리
- const는 컴파일 타임에 결정되며, 'static 수명을 가질 수 있음 (프로모션될 경우).
- static은 전역 변수로, 'static 수명을 기본적으로 가짐.
- const는 가변 참조를 포함할 수 없음.
- 구조체에 'static 참조를 넣고 const로 선언하려면 내부 값들도 'static이어야 함.

