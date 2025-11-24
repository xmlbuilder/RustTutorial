# expcted `!` (never type)

지금 Rust 컴파일러가 말하는 핵심은 let ... else 구문에서 else 블록은 반드시 **탈출(diverge)** 해야 한다는 점입니다.  
즉, else 안에서는 return, break, continue, panic! 같은 흐름을 끝내는 문장이 와야 하고,  
단순히 false 같은 값을 반환하는 건 허용되지 않습니다.

```
error[E0308]: `else` clause of `let...else` does not diverge
  --> src\core\circle.rs:72:83
   |
72 |           let (Some(pln1), Some(pln2), Some(pln3) ) = (plane0, plane1, plane2) else {
   |  ___________________________________________________________________________________^
73 | |             false
74 | |         };
   | |_________^ expected `!`, found `bool`
   |
```

## 🔎 왜 에러가 나는가?
```rust
let (Some(pln1), Some(pln2), Some(pln3)) = (plane0, plane1, plane2) else { false };
```

- let ... else는 패턴 매칭 실패 시 현재 함수/블록을 빠져나가야 한다는 규칙이 있습니다.
- 그런데 else { false }는 단순히 bool 값을 반환하려고 하니,  
  컴파일러가 **여기서는 ! (never type, 즉 절대 반환하지 않는 타입)가 필요하다** 고 에러를 내는 것임.

## ✅ 해결 방법
### 1. 함수에서 바로 return하기
```rust
let (Some(pln1), Some(pln2), Some(pln3)) = (plane0, plane1, plane2) else {
    return false; // 함수가 bool을 반환한다면 이렇게
};
```

### 2. 패닉으로 처리하기
```rust
let (Some(pln1), Some(pln2), Some(pln3)) = (plane0, plane1, plane2) else {
    panic!("plane 값이 None 입니다");
};
```

### 3. match 구문으로 바꾸기
```rust
match (plane0, plane1, plane2) {
    (Some(pln1), Some(pln2), Some(pln3)) => {
        // 정상 처리
    }
    _ => return false, // 실패 시 처리
}
```
## 🧠 정리
- let ... else는 패턴 매칭 실패 시 반드시 흐름을 종료해야 한다.
- 따라서 `else { false }` 는 불가능하고, `else { return false; }` 또는 match를 써야 합니다.

---

# never type

Rust에서 ! 타입은 흔히 never type이라고 부릅니다. 이름 그대로 **절대 값이 존재하지 않는 타입** 을 의미.

## 🔎 정의
- !는 값을 가질 수 없는 타입입니다.
- 즉, 이 타입의 값은 절대 만들어질 수 없고, 프로그램 흐름이 그 지점에서 **끝난다(diverge)** 는 뜻.

### 📌 언제 쓰이나?
- 함수에서 절대 반환하지 않는 경우
```rust
fn forever() -> ! {
    loop { } // 무한 루프, 절대 끝나지 않음
}
```
- 패닉 발생
```rust
fn crash() -> ! {
    panic!("에러 발생!");
}
```
- 프로그램 흐름 종료
- return, break, continue 같은 제어문도 내부적으로는 ! 타입을 사용합니다.
- 예: return false;는 함수 전체를 끝내므로 else 블록에서 허용됩니다.

### 🧠 왜 필요한가?
- let ... else 같은 구문에서 패턴 매칭 실패 시 반드시 `흐름을 종료` 해야 한다는 규칙이 있습니다.
- 그래서 else { false }는 안 되고, else { return false; }처럼 함수 자체를 끝내야 합니다.
- 이때 return false;는 함수의 반환 타입(bool)을 맞추면서 동시에 흐름을 종료하기 때문에 올바른 코드가 됩니다.

## ✅ 요약
- ! = never type, 값이 존재하지 않는 타입
- 의미: **여기서 프로그램은 절대 정상적으로 계속되지 않는다**
- 사용 예: panic!, 무한 루프, return, break, continue

---



