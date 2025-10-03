# Drop

## 🧠 Drop이란?
- Drop은 어떤 값이 스코프를 벗어날 때 자동으로 호출되는 정리 로직을 정의하는 트레이트예요.
- 주로 파일 닫기, 메모리 해제, 네트워크 연결 종료 같은 자원 정리에 사용됩니다.
```rust
struct SmartPointer {
    data: String,
}

impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("Dropping SmartPointer with data `{}`!", self.data);
    }
}
```
이 예제에서 SmartPointer가 스코프를 벗어나면 drop()이 자동 호출되어 메시지를 출력합니다.


## 🔍 주요 특징

| 항목           | 설명                                                                 |
|------------------------|----------------------------------------------------------------------|
| `drop()`               | 값이 스코프를 벗어날 때 자동 호출되는 정리 함수                      |
| `Drop::drop()`         | 직접 호출 불가, 대신 `std::mem::drop()`을 사용해 명시적 drop 가능     |
| `std::mem::drop()`     | 값을 즉시 drop 처리하고 이후 접근 불가                                |
| `Copy`와 `Drop`은 양립 불가 | `Copy` 타입은 drop 시점을 예측할 수 없기 때문에 `Drop`을 구현할 수 없음 |



## ✅ std::mem::drop()과의 차이
Rust에서는 Drop::drop()을 직접 호출할 수 없고,
명시적으로 값을 drop하고 싶을 때는 std::mem::drop()을 사용해야 해요.
```rust
let c = SmartPointer { data: String::from("early drop") };
std::mem::drop(c); // 여기서 drop() 호출됨
println!("c는 이미 drop됨");
```


## 🧠 실전에서 언제 쓰나?
- Box<T>, Vec<T>, File 등 자원을 소유하는 타입은 내부적으로 Drop을 구현
- 커스텀 타입에서 자원 정리 로직을 명시적으로 넣고 싶을 때 직접 구현
- 예: impl Drop for TcpConnection { ... }

## ⚠️ 주의할 점
- Drop은 panic 중에도 호출됨 → 정리 로직은 반드시 안전하게 작성해야 함
- Drop이 구현된 타입은 Copy를 붙일 수 없음 → move만 가능


## Drop 관련 소유권·복사 한 장 요약
- Copy가 없으면: 값 전달 = move (원본 못 씀)
- Copy가 있으면: 값 전달 = bit-copy (원본 계속 사용 가능)
- Drop이 있으면 Copy 불가.
- 수학용 작은 POD(모두 f64 등) 타입은 보통 #[derive(Copy, Clone)].

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion { pub x:f64, pub y:f64, pub z:f64, pub w:f64 }
```

## 1) Drop를 구현하는 방법

Drop는 “스코프를 벗어날 때 자동으로 호출되는 소멸자”예요.  
보통 리소스 해제(FFI 핸들, 파일, 메모리 등) 에서만 필요합니다. 
Quaternion처럼 f64만 들고 있는 POD 타입에는 보통 필요 없습니다.

```rust
// Copy는 빼야 함! (Drop 있는 타입은 Copy가 될 수 없음)
#[derive(Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f64, pub y: f64, pub z: f64, pub w: f64,
}

impl Drop for Quaternion {
    fn drop(&mut self) {
        // 여기서 리소스 해제/로그 등 수행 (패닉은 피하세요!)
        #[cfg(debug_assertions)]
        eprintln!("drop Quaternion: {:.3},{:.3},{:.3}|{:.3}", self.x, self.y, self.z, self.w);
    }
}
```

## 중요한 규칙

- Drop을 구현하면 그 타입은 더 이상 Copy가 될 수 없습니다.
- 값 전달(by value)은 진짜 move가 됩니다(원본 변수 사용 불가).
- Drop에서 패닉하지 마세요(더블 패닉 → abort 가능).
- 의도적으로 누수하고 싶다면 std::mem::forget(self) (권장X).

## 2) Copy가 없어졌을 때 생기는 영향과 대처

현재 프로젝트는 Mul이 소유값 × 소유값 시그니처였죠:
```rust
impl std::ops::Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output { /* ... */ }
}
```
Copy가 없으니 self * next는 두 피연산자를 move합니다.
원본을 계속 쓰고 싶다면 아래 중 하나를 선택하세요.

### 방법 A: API를 참조로 받고 clone() 사용 (간단)
POD라서 clone() 비용은 비트 복사 수준이에요.
impl Quaternion {
    // 소유권 보존: 참조로 받고, 내부에서 clone()으로 값 생성
    pub fn then(&self, next: &Self) -> Self {
        self.clone() * next.clone()
    }
}

### 방법 B: 연산자 오버로드를 “참조 × 참조”로도 지원
(기존 구현을 재사용하거나 수식 한 번 더 적기)
use std::ops::Mul;
impl<'a, 'b> Mul<&'b Quaternion> for &'a Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: &'b Quaternion) -> Quaternion {
        // 1) 간단 위임: clone 해서 기존 Mul(Self, Self) 재사용
        self.clone() * rhs.clone()
        // 2) 또는 여기서 직접 수식 계산해도 됨 (중복 수식이 싫으면 1번이 낫습니다)
    }
}


그럼 self * next 대신 &self * &next도 쓸 수 있고,
then도 이렇게 깔끔:
```rust
impl Quaternion {
    pub fn then(&self, next: &Self) -> Self {
        self * next  // 참조 × 참조 → 위에서 만든 Mul<&,&>가 호출됨
    }
}
```
### 3) 언제 Drop이 진짜 유용한가?

#### FFI 자원 래핑: 예) C 라이브러리 핸들 해제
```rust
pub struct NativeQuat { raw: *mut ffi::Quat }
impl Drop for NativeQuat {
    fn drop(&mut self) { unsafe { ffi::quat_free(self.raw) } }
}
```

#### 파일/소켓/락: 스코프 종료 시 자동 해제

순수 수치 타입(사원수/행렬/벡터)에는 보통 Drop 불필요하고,
성능과 사용성 측면에서 Copy 유지가 더 좋습니다.

### 4) 요약

Drop을 붙이면 Copy를 포기해야 하고, 값 전달은 move가 됩니다.
소유권 보존이 필요하면 API를 &self/&Self로 바꾸고 내부에서 clone()
또는 참조용 Mul 구현을 추가하세요.
수치용 구조체는 보통 Drop 없이 #[derive(Copy, Clone)]가 권장 패턴입니다.

---

# Drop / Clone

Rust에서 Drop이 구현된 타입은 Copy를 할 수 없고, 대신 Clone만 가능.  
이건 단순한 문법 제한이 아니라, 메모리 구조와 동작 방식의 근본적인 차이 때문.  

## 비교 항목
| 항목                     | Copy                                | Clone                                 |
|--------------------------|--------------------------------------|----------------------------------------|
| 복사 방식                | 비트 단위 복사 (shallow copy)       | 사용자 정의 복사 (deep copy 가능)      |
| 동작 시점                | 컴파일 타임                          | 런타임                                 |
| 비용                     | 거의 없음                            | 비용 있음 (힙 할당, 복사 등)           |
| Drop 트레이트 영향       | Drop이 있으면 Copy 불가              | Drop이 있어도 Clone 가능               |
| 사용자 정의 가능 여부    | 불가능                                | 가능 (`clone()` 메서드로 정의)         |
| 메모리 할당 여부         | 새 메모리 할당 없음                  | 새 메모리 할당 가능                    |
| 대표 예시 타입           | `i32`, `bool`, `char`, `usize` 등    | `String`, `Vec<T>`, `Box<T>` 등        |

## ✅ 예시로 이해해보면
### Copy 타입 (예: i32, f64, bool, usize)
```rust
let a = 42;
let b = a; // b는 a의 복사본, 둘 다 독립적으로 존재
```

- 그냥 비트 단위 복사만 일어남
- 메모리 구조는 동일, 별도 할당 없음
### Clone 타입 (예: String, Vec<T>, 사용자 정의 타입)
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // s2는 s1의 복사본, 새 메모리에 저장됨
```

- clone()은 내부적으로 새 힙 메모리 할당 후 데이터 복사
- s1과 s2는 서로 다른 메모리 주소를 가짐

### 🔥 왜 Drop이 있으면 Copy를 못 쓰는가?
- Copy는 자동으로 복사되기 때문에,  
    → 복사된 값이 Drop을 두 번 호출할 위험이 있음
- 예: Box<T>는 Drop이 구현되어 있는데, Copy를 허용하면  
    → 같은 포인터를 두 번 해제하려고 해서 double free 발생 가능


## 🔍 비전문가 감각으로 보는 Copy vs Clone
| 관점                     | Copy                                | Clone                                 |
|--------------------------|--------------------------------------|----------------------------------------|
| 복사 방식                | 그냥 주소만 옮겨줌 (얕은 복사)       | 새 메모리 만들어서 복사 (깊은 복사 가능) |
| 메모리 할당              | 없음 (기존 메모리 그대로 사용)       | 있음 (새 힙 메모리 할당)               |
| 실행 비용                | 거의 없음                            | 있음 (복사 + 할당)                     |
| Drop 트레이트 영향       | 있으면 Copy 불가                     | 있어도 Clone 가능                      |
| 복사 시점                | 컴파일 타임                          | 런타임                                 |
| 예시                     | `i32`, `bool`, `char` 등             | `String`, `Vec<T>`, `Box<T>` 등        |

## 예시로 보면 더 명확:
```rust
let s1 = String::from("hello");
let s2 = s1; // move 발생, s1은 더 이상 사용 불가
```

- String은 Copy가 안 되니까 → **소유권 이동(move)**이 일어남
- s1과 s2가 같은 힙 주소를 공유하면 → Drop이 두 번 호출될 수 있어서 위험
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // 새 힙 메모리에 "hello" 복사됨
```

- clone()은 새 메모리 할당 + 데이터 복사
- s1, s2는 서로 다른 힙 주소를 가리킴 → 안전하게 Drop 가능

## ✅ 그래서 Rust는 이렇게 설계
- Copy는 shallow copy만 허용 → Drop이 있으면 금지
- Clone은 deep copy 가능 → Drop 있어도 안전하게 사용 가능

---

# move 메모리 무력화

말씀하신 대로, s1 = s2는 메모리 입장에서는 shallow copy처럼 보이지만,
Rust는 s1을 “무력화”시키는 방식으로 안전성을 확보합니다.

## 🔍 메모리 관점: 실제로는 “복사”가 맞다
```rust
let s1 = String::from("hello");
let s2 = s1; // move 발생
```

- 힙에 "hello"가 저장되고, s1은 그 힙 주소를 가리킴
- s2 = s1은 그 주소를 그대로 복사함 → shallow copy
- 즉, 메모리 관점에서는 복사가 맞음

## 🔐 Rust가 s1을 “무력화”시키는 방식
Rust는 컴파일 타임에 s1을 사용 불가능하게 만드는 방식으로 안전성을 확보합니다:
### 1. 소유권 이동(move semantics)
- s1의 소유권이 s2로 이동하면  
    → s1은 더 이상 유효하지 않음
### 2. 컴파일러가 s1을 “사용 금지” 처리
- 이후 s1을 사용하려고 하면 컴파일 에러 발생:
```  
error[E0382]: borrow of moved value: `s1`
```

### 3. Drop은 단 한 번만 호출됨
- s2만 Drop 대상이 되고, s1은 더 이상 존재하지 않음
- 이로써 double free, dangling pointer, use-after-free를 원천적으로 차단

## ✅ 비전문가 감각으로 요약하면
“실제로는 주소 복사지만, Rust는 s1을 컴파일 타임에 죽여버려서 메모리 해제를 한 번만 하게 만든다.”

## 🔚 요약
| 관점             | 설명                                       |
|------------------|--------------------------------------------|
| 메모리 관점       | shallow copy (주소만 복사됨)               |
| Rust의 안전 전략 | s1을 컴파일 타임에 무력화 (사용 금지 처리) |
| Drop 호출        | 단 한 번만 발생 (s2에서만 호출됨)          |
| 위험 회피        | double free, use-after-free 방지           |


## 🔍 Rust의 소유권 무력화 메커니즘
### 1. 메모리 관점: shallow copy
- let s2 = s1;은 실제로는 s1의 내부 포인터를 그대로 복사함
- 즉, 힙 주소는 그대로 공유하는 구조 → shallow copy
### 2. 컴파일러 관점: 소유권 추적
- Rust는 타입 시스템과 소유권 트래킹을 통해  
    → s1이 더 이상 유효하지 않음을 컴파일 타임에 감지
- 이후 s1을 사용하려 하면 컴파일 에러:

```
error[E0382]: use of moved value: `s1`
```

### 3. 무력화 방식
- s1은 메모리에는 남아 있지만, 타입 시스템에서 “죽은 값”으로 간주
- 이건 마치 **“s1은 연결이 끊긴 유령 포인터”**처럼 취급됨
- 그래서 Drop은 s2에서만 호출되고, s1은 사용도 해제도 불가능

---

# drop(a)만 되는가
## 실제 코드
```rust
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("{} 삭제 중", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("B 블록에서 나가기");
        }
        println!("A 블록에서 나가기");
    }
    drop(a);
    println!("main에서 나가기");
}
```


겉보기엔 a.drop()도 될 것 같은데, Rust에서는 drop(a)만 허용되고 a.drop()은 금지됩니다.
이건 Rust의 안전성과 소유권 모델을 지키기 위한 의도적인 설계예요.

## 🔍 왜 a.drop()은 안 되고 drop(a)만 되는가?
### 1. Drop::drop은 직접 호출할 수 없도록 막혀 있음
- Rust는 Drop 트레이트의 drop() 메서드를  
    → 직접 호출하는 걸 금지합니다
- 즉, a.drop()은 컴파일 에러가 납니다:
```
error[E0040]: explicit use of destructor method
```

### 2. 이유: 안전성과 예측 가능성
- Drop은 소유권이 끝날 때 자동으로 호출되는 메서드예요
- 만약 a.drop()을 허용하면:
- a가 중간에 해제되고도 이후에 사용될 수 있음
- → use-after-free, double drop 같은 위험 발생
### 3. drop(a)는 안전한 방식
- drop(a)는 표준 라이브러리 함수로  
    → a의 소유권을 넘기고 즉시 drop을 호출함
- 이후 a는 사용 불가 상태가 되므로  
    → 컴파일 타임에 안전하게 막아줌

## ✅ 비전문가 감각으로 요약하면
“a.drop()은 내가 직접 해제하려는 거고,
drop(a)은 Rust한테 ‘얘 해제해줘’ 하고 넘기는 거다.
Rust는 내가 직접 해제하는 걸 막고,
소유권을 넘겨서 안전하게 해제하는 방식만 허용하는 거다.”


## 🔚 요약
| 방식        | 설명                                                   |
|-------------|--------------------------------------------------------|
| a.drop()    | ❌ 금지됨 — Drop은 직접 호출할 수 없고 컴파일 에러 발생 |
| drop(a)     | ✅ 허용됨 — 소유권을 넘겨 안전하게 메모리 해제함        |
| 이유        | Rust는 Drop을 자동 호출용으로 설계해 직접 호출을 막음   |


## 🔍 핵심 질문: a.drop()을 직접 호출하면, 스코프 끝날 때 또 drop이 호출될까?
### ✅ 이론적으로는 그럴 수 있어 보임
- a.drop()은 &mut self니까 → a는 여전히 살아 있음
- 스코프 끝나면 → Rust는 자동으로 drop을 호출하려고 할 것 같음
- 즉, drop이 두 번 호출될 수도 있는 구조처럼 보임
❌ 하지만 Rust는 그걸 허용하지 않음
- Rust는 Drop::drop()을 직접 호출하는 걸 금지함
- 이유는:
    → 직접 호출하면 drop이 두 번 호출될 위험이 있기 때문

## 🔐 Rust의 안전 전략
| 방식        | 동작 설명                                               |
|-------------|----------------------------------------------------------|
| drop(a)     | ✅ 소유권을 넘기고 안전하게 drop 호출 → 이후 a는 무력화됨 |
| a.drop()    | ❌ 금지됨 — drop이 두 번 호출될 위험이 있어 컴파일 에러 발생 |
| 안전 전략   | Drop은 자동 호출용으로 설계됨 → 직접 호출은 원천 차단     |

---







