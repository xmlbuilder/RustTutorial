# as_*
## 1. as_deref_mut가 정확히 하는 일
- 시그니처(Option 기준):
```rust
impl<T> Option<T>
where
    T: std::ops::DerefMut,
{
    pub fn as_deref_mut(&mut self) -> Option<&mut T::Target>;
}
```

- 해석하면:
    - 입력: &mut Option<T>
    - 조건: T: DerefMut<Target = U> 인 어떤 타입 (예: Box<U>, &mut U, Rc<RefCell<U>>는 아님)
    - 출력: Option<&mut U> (즉, “안에 있는 걸 deref한 뒤 다시 &mut로 빼 줌”)
- 직관적으로는:
- **안에 DerefMut 가능한 뭔가가 들어 있는 Option이 있을 때, 그 안쪽의 실제 값에 대한 &mut 참조만 빼서 쓰고 싶다.**

- 예시:
```rust
let mut x: Option<Box<i32>> = Some(Box::new(10));

// as_deref_mut 없으면: Option<&mut Box<i32>>
if let Some(b) = x.as_mut() {
    **b = 20;
}

// as_deref_mut 사용: Option<&mut i32>
if let Some(v) = x.as_deref_mut() {
    *v = 30;
}
```

- 여기서 중요한 포인트:
    - as_mut() → Option<&mut Box<i32>>
    - as_deref_mut() → Option<&mut i32> (Box 내부로 한 번 더 들어감)
    - 둘 다 소유권 이동(moving)을 일으키지 않고 단지 borrow만 한다.

## 2. 왜 Option<&mut f64>에서 as_deref_mut를 쓰게 되는가
- 잘못된 코드 패턴은:
```rust
fn f(gauss: Option<&mut f64>) {
    if let Some(gv) = gauss { *gv = 0.0; }
    if let Some(gv) = gauss.as_deref_mut() { *gv = det; } // 에러
}
```

- 여기서 문제는:
- if let Some(gv) = gauss → Option<&mut f64>에서 내부 &mut f64를 move 해버림
- 그래서 gauss는 **부분적으로 move된 값** 이 돼서,  
    이후 gauss.as_deref_mut() 호출 시 borrow 에러가 터짐.
- as_deref_mut 자체는 잘못이 없고,  
    그 전에 한 번이라도 if let Some(x) = gauss를 썼다는 사실이 문제의 근원이다.
- Option<&mut T> 는 Copy가 아니기 때문에, 이 타입을 쓸 땐 항상 이렇게 해야 안전하다:
```rust
if let Some(gv) = gauss.as_deref_mut() {
    *gv = 0.0;
}
```

- 이렇게 하면:
    - gauss는 그대로 남아 있고
    - 내부의 &mut f64 만 잠깐 빌려와서 쓰고 돌려줌
    - move가 아니라 borrow라서, 이후에도 gauss를 다시 사용할 수 있음.

## 3. as_deref vs as_deref_mut 차이
- 둘 다 Deref/DerefMut 트레이트 기반의 sugar.
```rust
fn as_deref(self: &Option<T>) -> Option<&U>;
```
```rust
fn as_deref_mut(self: &mut Option<T>) -> Option<&mut U>;
```

- as_deref:
    - 읽기 전용 빌려오기 &Option<T> → Option<&U>
    - 예: Option<String> → Option<&str>
- as_deref_mut:
    - 가변 빌려오기 &mut Option<T> → Option<&mut U>
    - 예: &mut Option<Box<Foo>> → Option<&mut Foo>
- 둘 다 안에 있는 걸 deref + 래핑해서 리턴할 뿐이고, 소유권을 옮기지 않는다.

## 4. 다른 중요한 as_* 계열 정리
- Rust 표준 라이브러리에서 자주 쓰이는 as_* 몇 개만 뽑으면:

### 4.1. as_ref / as_mut
- Option/Result에서 가장 많이 쓰이는 기본기.
```rust
impl<T> Option<T> {
    fn as_ref(&self) -> Option<&T>;
    fn as_mut(&mut self) -> Option<&mut T>;
}
```
```rust
impl<T, E> Result<T, E> {
    fn as_ref(&self) -> Result<&T, &E>;
    fn as_mut(&mut self) -> Result<&mut T, &mut E>;
}
```
- 소유권 move 없이 내부를 빌려와서 쓰고 싶을 때.
- 예:
```rust
let mut opt: Option<String> = Some("hi".to_string());

if let Some(s) = opt.as_mut() {
    s.push_str(" there");
}
// opt는 여전히 Some("hi there") 상태로 남아 있음.
```


### 4.2. String/Vec 관련: as_str, as_bytes, as_slice, as_mut_slice
- String::as_str(&self) -> &str
- String::as_bytes(&self) -> &[u8]
- Vec<T>::as_slice(&self) -> &[T]
- Vec<T>::as_mut_slice(&mut self) -> &mut [T]
- 이건 “소유한 컨테이너를 슬라이스 뷰로 바꿔서 쓰는” 전형적인 패턴.
```rust
let mut v = vec![1, 2, 3];
let s: &mut [i32] = v.as_mut_slice();
s[0] = 10;
// v == [10, 2, 3]
```

### 4.3. 포인터/원시 관련: as_ptr, as_mut_ptr
- slice::as_ptr(&self) -> *const T
- slice::as_mut_ptr(&mut self) -> *mut T
- 이건 주로 FFI나 unsafe 코드에서 쓰이고, NURBS/수치코드에서도 C API랑 붙일 때  
    유용할 수도 있음.

## 5. 언제 뭐를 써야 하는지 감각 정리
- 정리하면:
    - Option/Result 내부를 빌려 쓰고 싶다 → as_ref, as_mut
    - Deref 가능한 타입(예: Box, String)의 내부를 바로 빌려 쓰고 싶다
        - 읽기 전용 → as_deref
        - 가변 → as_deref_mut
    - Vec / String / slice → as_slice, as_mut_slice, as_str, as_bytes
    - 소유권을 절대 move시키고 싶지 않다 → as_* 계열을 먼저 떠올리기
- 특히 지금처럼:
    - 함수 파라미터가 Option<&mut f64> 일 때는  
        무조건 as_deref_mut 패턴으로만 다뤄야 한다
        - if let Some(x) = opt / match opt 로 꺼내면 곧바로 partial move 지옥.

## 6. 상황에 딱 맞는 안전 패턴 예시
- 지금 principal curvature 코드에서 쓰면 딱 이런 식:
```rust
fn set_zero(
    gauss: &mut Option<&mut f64>,
    mean: &mut Option<&mut f64>,
    k1: &mut Option<&mut f64>,
    k2: &mut Option<&mut f64>,
) {
    if let Some(gv) = gauss.as_deref_mut() { *gv = 0.0; }
    if let Some(mv) = mean.as_deref_mut() { *mv = 0.0; }
    if let Some(v1) = k1.as_deref_mut()   { *v1 = 0.0; }
    if let Some(v2) = k2.as_deref_mut()   { *v2 = 0.0; }
}
```

- 이 패턴으로 쓰면:
    - 소유권 이동 없음
    - borrow 문제 없음
    - 여러 번 호출해도 안전

- 특히 as_ref, as_mut, as_deref, as_deref_mut 같은 것들은  
    언제 어떤 걸 써야 하는지 감이 안 오기 쉬움.
- 그래서 패턴별로, 언제 어떤 as_ 를 써야 하는지*, 그리고 왜 필요한지를 아주 직관적으로 정리.

### 🌟 Rust의 as_* 계열은 **소유권(move) 없이 내부를 빌려오기 위한 도구들**
- Rust의 핵심은 move vs borrow.
    - if let Some(x) = option → 내부 값을 move
    - option.as_ref() → 내부 값을 borrow (&)
    - option.as_mut() → 내부 값을 borrow (&mut)
    - option.as_deref() → 내부의 내부까지 borrow (&)
    - option.as_deref_mut() → 내부의 내부까지 borrow (&mut)
- 즉, as_* 계열은 move를 피하고 싶을 때 쓰는 도구.

## 🎯 언제 어떤 as_* 를 써야 하는지 한눈에 보는 표
| 상황(What you want)                           | 메서드(as_*)        | 결과 타입(Result)          | 설명(When to use)                                      |
|-----------------------------------------------|----------------------|-----------------------------|---------------------------------------------------------|
| Option<T> 내부를 읽기 전용으로 빌리고 싶다     | as_ref()             | Option<&T>                  | move 없이 내부를 &T 로 보고 싶을 때                    |
| Option<T> 내부를 가변으로 빌리고 싶다          | as_mut()             | Option<&mut T>              | move 없이 내부를 &mut T 로 보고 싶을 때                |
| Deref 가능한 타입(Box<T>, &T 등)의 내부를 빌림 | as_deref()           | Option<&U>                  | 내부의 내부를 &U 로 보고 싶을 때                       |
| DerefMut 가능한 타입(Box<T>, &mut T 등)        | as_deref_mut()       | Option<&mut U>              | 내부의 내부를 &mut U 로 보고 싶을 때                   |
| Vec<T> 를 슬라이스로 보고 싶다                 | as_slice()           | &[T]                        | Vec을 읽기 전용 slice 로 변환                          |
| Vec<T> 를 가변 슬라이스로 보고 싶다            | as_mut_slice()       | &mut [T]                    | Vec을 가변 slice 로 변환                               |
| String 을 &str 로 보고 싶다                    | as_str()             | &str                        | String → &str 변환                                      |
| String 을 &[u8] 로 보고 싶다                   | as_bytes()           | &[u8]                       | UTF-8 바이트 단위 접근                                  |

- as_ref / as_mut       → Option 내부를 borrow
- as_deref / as_deref_mut → 내부의 내부(Box, &mut 등)를 borrow
- as_slice / as_mut_slice → Vec을 slice로 변환
- as_str / as_bytes     → String을 view로 변환



### 🔥 왜 as_deref_mut() 가 필요한가?
- 예를 들어 네 함수는 이런 타입을 받지?
```rust
gauss: Option<&mut f64>
```

- 이타입은 Copy가 아니다.
- 그래서 아래 코드는 move를 일으킨다:
```rust
if let Some(gv) = gauss { *gv = 0.0; }   // ❌ move 발생
```
- move가 발생하면 이후에 gauss를 다시 사용할 수 없어서 에러가 터진다.
- 그래서 Rust는 이렇게 하라고 한다:
```rust
if let Some(gv) = gauss.as_deref_mut() { *gv = 0.0; }   // ✅ borrow만 발생
```

- 즉,
- Option<&mut T> 를 move하지 않고 내부의 &mut T만 빌려오고 싶을 때  
    as_deref_mut()가 유일한 정답


### 🌈 예제로 감 잡기
#### 1) Option<T> → 내부를 읽기 전용으로 보고 싶다
```rust
let opt: Option<String> = Some("hello".into());

if let Some(s) = opt.as_ref() {
    println!("{}", s);   // &String
}
```

#### 2) Option<T> → 내부를 수정하고 싶다
```rust
let mut opt: Option<String> = Some("hello".into());

if let Some(s) = opt.as_mut() {
    s.push_str(" world");
}
```

#### 3) Option<Box<T>> → Box 안의 T를 보고 싶다
```rust
let mut opt: Option<Box<i32>> = Some(Box::new(10));

if let Some(v) = opt.as_deref_mut() {
    *v = 20;   // &mut i32
}
```

### 4) Option<&mut T> → 내부 T를 수정하고 싶다 (네 상황)
```rust
fn f(gauss: &mut Option<&mut f64>) {
    if let Some(gv) = gauss.as_deref_mut() {
        *gv = 0.0;
    }
}
```


## 🧠 핵심 요약 (이거만 기억하면 된다)
- ✔ Option<T> 내부를 move 없이 보고 싶다 → as_ref() / as_mut()
- ✔ Option<Box<T>> 또는 Option<&mut T> 내부의 진짜 T를 보고 싶다
    - as_deref() / as_deref_mut()
- ✔ Option<&mut T> 는 절대 if let Some(x) = option 쓰면 안 된다
    - move 발생 → 이후 사용 불가 → 에러 폭발

## 🎉 초간단 기억법
- **Option은 as_로 다뤄라**.
- 내부를 빌릴 때는 as_ref/as_mut,
- 내부의 내부를 빌릴 때는 as_deref/as_deref_mut.”
- 이 한 줄만 기억해도 Rust의 Option 처리에서 90%는 해결된다.

---

## Vec / String
- Option 중심으로 설명했는데, Rust에는 Vec, String, slice, Box, Rc, Arc 등  
    다양한 타입에 as_* 계열이 존재해서 헷갈리기 딱 좋음.
- 그래서 이번엔 타입별로, 언제 어떤 as_ 를 써야 하는지*, 왜 필요한지를 한 번에 정리.

### 🌟 1. Vec<T> 관련 as_* 정리
- ✔ as_slice()
    - Vec을 읽기 전용 슬라이스(&[T]) 로 보고 싶을 때
```rust
let v = vec![1, 2, 3];
let s: &[i32] = v.as_slice();
```

- ✔ as_mut_slice()
    - Vec을 가변 슬라이스(&mut [T]) 로 보고 싶을 때
```rust
let mut v = vec![1, 2, 3];
let s: &mut [i32] = v.as_mut_slice();
s[0] = 10;
```

- ✔ 언제 쓰나?
    - 함수가 &[T] 또는 &mut [T] 를 요구할 때
    - Vec을 슬라이스로 넘기고 싶을 때
    - Vec 내부를 안전하게 조작하고 싶을 때

### 🌟 2. String 관련 as_* 정리
- ✔ as_str()
    - String → &str
```rust
let s = String::from("hello");
let ss: &str = s.as_str();
```

- ✔ as_bytes()
    - String → &[u8]
```rust
let bytes = s.as_bytes();
```

- ✔ 언제 쓰나?
    - 문자열을 읽기 전용으로 넘길 때
    - UTF-8 바이트 단위로 처리할 때
    - &str API를 쓰고 싶을 때

### 🌟 3. slice 관련 as_* 정리
- ✔ as_ptr() / as_mut_ptr()
    - 슬라이스를 raw pointer로 변환
```rust
let s: &[i32] = &[1, 2, 3];
let p = s.as_ptr();        // *const i32
let mp = s.as_mut_ptr();   // *mut i32
```

- ✔ 언제 쓰나?
    - FFI(C API) 호출
    - unsafe 블록에서 직접 메모리 조작
    - 고성능 연산

### 🌟 4. Box<T> 관련 as_* 정리
- ✔ as_ref()
- Box<T> → &T
```rust
let b = Box::new(10);
let r: &i32 = b.as_ref();
```

- ✔ as_mut()
    - Box<T> → &mut T
```rust
let mut b = Box::new(10);
let r: &mut i32 = b.as_mut();
```

- ✔ 언제 쓰나?
    - Box 내부 값을 빌리고 싶을 때
    - 소유권(move) 없이 접근하고 싶을 때

### 🌟 5. Rc<T>, Arc<T> 관련 as_* 정리
- ✔ as_ref()
    - Rc<T> → &T
    - Arc<T> → &T
```rust
let rc = Rc::new(10);
let r: &i32 = rc.as_ref();
```

- ✔ 언제 쓰나?
    - Rc/Arc 내부 값을 읽기 전용으로 접근할 때
    - clone() 없이 borrow만 하고 싶을 때

### 🌟 6. Option<T> 관련 as_* 정리 (요약)
- ✔ as_ref()
    - Option<T> → Option<&T>
- ✔ as_mut()
    - Option<T> → Option<&mut T>
- ✔ as_deref()
    - Option<Box<T>> → Option<&T>
    - Option<&T> → Option<&T> (그대로)
- ✔ as_deref_mut()
    - Option<Box<T>> → Option<&mut T>
    - Option<&mut T> → Option<&mut T>

### 🌟 7. 언제 어떤 as_* 를 써야 하는지 “패턴”으로 정리
- 🔥 패턴 1: “소유권(move) 없이 내부를 보고 싶다”
    - as_ref() / as_mut()
- 🔥 패턴 2: “Box, &mut 같은 Deref 가능한 타입의 내부를 보고 싶다”
    - as_deref() / as_deref_mut()
- 🔥 패턴 3: “Vec을 슬라이스로 보고 싶다”
    - as_slice() / as_mut_slice()
- 🔥 패턴 4: “String을 &str로 보고 싶다”
    - as_str()
- 🔥 패턴 5: “raw pointer가 필요하다”
    - as_ptr() / as_mut_ptr()

## 🌈 초간단 기억법
- **as_ref/as_mut** → Option/T 내부를 borrow
- **as_deref/as_deref_mut** → 내부의 내부를 borrow
- **as_slice/as_str** → 컨테이너를 view로 변환


---
