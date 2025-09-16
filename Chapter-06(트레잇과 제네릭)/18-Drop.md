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

