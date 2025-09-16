# 🧠 Boolean 타입이란?
Rust에서 bool 타입은 참(true) 또는 거짓(false) 두 가지 값만을 가질 수 있는 기본 데이터 타입입니다.  
조건문, 논리 연산, 제어 흐름 등에서 자주 사용되죠.
```rust
fn main() {
    let t = true;           // 타입 추론으로 bool로 인식
    let f: bool = false;    // 명시적으로 bool 타입 지정
}
```


## ⚙️ 특징
- 크기: bool 타입은 메모리에서 1바이트(8비트)를 차지합니다.
- 비트 패턴:
- true → 0x01
- false → 0x00
- 유효하지 않은 비트 패턴을 가지면 정의되지 않은 동작(undefined behavior)이 발생할 수 있습니다.

## 🔍 사용 예시
### 조건문
```rust
let is_active = true;
if is_active {
    println!("활성 상태입니다.");
}
```

### 논리 연산
```rust
let a = true;
let b = false;

println!("{}", a && b); // false
println!("{}", a || b); // true
println!("{}", !a);     // false
```


## 🧩 관련 트레잇
bool 타입은 다음과 같은 트레잇을 자동으로 구현합니다:
- Clone
- Copy
- Sized
- Send
- Sync
즉, 복사하거나 스레드 간에 안전하게 공유할 수 있어요.

---


## ✅ 조건문 (if, else if, else)
```rust
fn main() {
    let is_logged_in = true;

    if is_logged_in {
        println!("환영합니다!");
    } else {
        println!("로그인이 필요합니다.");
    }
}
```

### 🔄 여러 조건 처리
```rust
fn main() {
    let score = 85;

    if score >= 90 {
        println!("A등급");
    } else if score >= 80 {
        println!("B등급");
    } else {
        println!("재시험 필요");
    }
}
```


### 🎯 match 문
match는 Rust의 강력한 패턴 매칭 도구예요. bool 타입에도 아주 잘 어울립니다.
```rust
fn main() {
    let is_dark_mode = true;

    match is_dark_mode {
        true => println!("다크 모드 활성화됨"),
        false => println!("라이트 모드 사용 중"),
    }
}
```


## 🧩 고급 패턴 매칭 예제
### 1. Option<bool>과 함께 쓰기
```rust
fn main() {
    let setting: Option<bool> = Some(true);

    match setting {
        Some(true) => println!("설정이 켜졌습니다."),
        Some(false) => println!("설정이 꺼졌습니다."),
        None => println!("설정이 지정되지 않았습니다."),
    }
}
```



### 2. 튜플 패턴 매칭
```rust
fn main() {
    let user = ("JungHwan", true);

    match user {
        (name, true) => println!("{name}님은 활성 사용자입니다."),
        (name, false) => println!("{name}님은 비활성 사용자입니다."),
    }
}
```

### 3. if let을 활용한 간결한 매칭
```rust
fn main() {
    let is_ready: Option<bool> = Some(true);

    if let Some(true) = is_ready {
        println!("준비 완료!");
    }
}

```
---



