# format! / println!

## ✅ format! 기본 사용법
```rust
fn main() {
    let name = "JungHwan";
    let age = 30;
    let message = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", message);
}
```

### 출력 결과: 
```
My name is JungHwan and I am 30 years old.
```
- format!은 문자열을 만들어서 message 변수에 저장합니다.


## 🧪 다양한 포맷 옵션도 그대로 사용 가능
```rust
let pi = 3.14159;
let s1 = format!("{:.2}", pi);       // "3.14"
let s2 = format!("{:#x}", 255);      // "0xff"
let s3 = format!("{name:?}", name="Rust"); // "\"Rust\""
```
## 📦 실전 활용 예
```rust
fn build_error_message(code: i32, reason: &str) -> String {
    format!("Error {}: {}", code, reason)
}

fn main() {
    let msg = build_error_message(404, "Not Found");
    println!("{}", msg); // Error 404: Not Found
}
```

## 💡 요약

| 매크로       | 설명 또는 반환값             |
|--------------|------------------------------|
| `println!`   | 콘솔 표준 출력 (stdout)       |
| `format!`    | `String` 생성 (출력 X)        |
| `eprintln!`  | 콘솔 에러 출력 (stderr)       |
| `write!`     | 버퍼, 파일 등 출력 스트림에 기록 |

## ✅ 추가 설명
- println!: 화면에 출력할 때 사용
- format!: 문자열을 만들 때 사용 (로그, 템플릿 등)
- eprintln!: 에러 메시지 출력용, stderr로 분리됨
- write!: std::fmt::Write 또는 std::io::Write 구현체에 출력 (파일, TCP 스트림 등)

---

# println! 포맷

## ✅ 기본 예시: 수식 결과 출력
```rust
fn main() {
    let a = 5;
    let b = 3;
    println!("a + b = {}", a + b); // 출력: a + b = 8
    println!("a * b = {}", a * b); // 출력: a * b = 15
    println!("(a + b) * 2 = {}", (a + b) * 2); // 출력: (a + b) * 2 = 16
}
```
- {} 안에는 변수나 표현식의 결과를 넣을 수 있음.  
- 직접 연산을 넣는 건 안 되지만, 괄호로 감싸서 계산한 결과를 넣는 건 OK!


## 🧠 고급 예시: 함수 호출, 조건식도 가능
```rust
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let n = 4;
    println!("square of {} is {}", n, square(n)); // 출력: square of 4 is 16
    println!("is even? {}", n % 2 == 0); // 출력: is even? true
}
```


## ❌ 불가능한 것
```rust
// 잘못된 예시
println!("Sum: {a + b}"); // ❌ 컴파일 오류
```
- {} 안에 직접 수식을 넣는 건 안 되고, 반드시 수식 결과를 변수나 괄호로 전달.


## 💡 팁
- format!() 매크로도 동일하게 동작하므로 문자열을 만들 때도 수식 결과를 넣을 수 있음
- 복잡한 수식은 변수에 먼저 담고 출력하는 게 가독성에 좋음


## 🧾 println! 포맷 옵션 요약

| 포맷 옵션     | 설명                                 | 예시 코드 및 출력 결과                          |
|---------------|--------------------------------------|--------------------------------------------------|
| `{}`          | 기본 출력                             | `println!("x = {}", 42);` → `x = 42`             |
| `{:?}`        | 디버그 출력 (Debug trait 필요)        | `println!("{:?}", vec![1, 2, 3]);` → `[1, 2, 3]` |
| `{:#?}`       | Pretty 디버그 출력 (줄바꿈 포함)       | `println!("{:#?}", vec![1, 2, 3]);` → 줄바꿈된 리스트 |
| `{:.2}`       | 소수점 2자리까지 출력 (부동소수점)     | `println!("{:.2}", 3.14159);` → `3.14`           |
| `{:#x}`       | 16진수 출력 (접두어 포함)              | `println!("{:#x}", 255);` → `0xff`               |
| `{0}`         | 첫 번째 인자 출력                      | `println!("{0}, {0}", "Hi");` → `Hi, Hi`         |
| `{a}`         | 이름 있는 변수 출력                    | `let a = 10; println!("{a}");` → `10`            |
| `{a:?}`       | 이름 있는 변수 디버그 출력             | `let a = vec![1]; println!("{a:?}");` → `[1]`    |

## 🧪 실습 예제 모음
```rust
fn main() {
    let pi = 3.14159;
    let hex = 255;
    let list = vec![1, 2, 3];
    let name = "JungHwan";

    println!("기본 출력: {}", pi);                // 3.14159
    println!("소수점 제한: {:.2}", pi);           // 3.14
    println!("16진수 출력: {:#x}", hex);          // 0xff
    println!("디버그 출력: {:?}", list);          // [1, 2, 3]
    println!("Pretty 디버그: {:#?}", list);       // 줄바꿈 포함
    println!("이름 있는 변수: {name}");           // JungHwan
    println!("중복 인자: {0}, {0}", "Hi");        // Hi, Hi
    println!("디버그 이름 변수: {list:?}");       // [1, 2, 3]
}
```

## 💡 팁
- {:?}는 구조체, 벡터, 튜플 등 복잡한 타입을 출력할 때 유용
- format!()에서도 동일한 포맷 옵션을 사용할 수 있음
- Display와 Debug 트레잇을 직접 구현하면 커스텀 출력도 가능

---




