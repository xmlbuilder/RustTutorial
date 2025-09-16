# 구조체 정리

- 관련있는 값들을 이름을 붙어 모아 구성하는 타입
- impl 블록으로 메소드나 연관 함수를 만들어 사용
- 메소드의 첫번재 파라미터는 self, &self, &mut self 모두 가능

## 샘플 예제
### 1. 구조체 정의
```rust
//구조체 정의
struct User {
    name: String,
    email: String,
    active:bool,
}

fn main() {
   let user: User = User{
        name: String::from("정중환"),
        email : String::from("junghwan.jeong@gmail.com"),
        active: true
   };

   println!("User name  = {}", user.name); //User name  = 정중환
   println!("User email  = {}", user.email); //User email  = junghwan.jeong@gmail.com
   println!("User.active = {}", user.active); //User.active = true
}
```

### 2. 지금은 구조체의 데이터를 변경할 수 없다.
```rust
fn main() {

   let user: User = User{
        name: String::from("정중환"),
        email : String::from("junghwan.jeong@gmail.com"),
        active: true
   };

   println!("User name  = {}", user.name);
   println!("User email  = {}", user.email);
   println!("User.active = {}", user.active);
   //user.email = String::from("test@gmail.com"); //Compile error
}
```

### 3. mut를 넣어서 수정 가능하도록 변경 할 수 있다.
```rust
fn main() {
    let mut user: User = User{
         name: String::from("정중환"),
         email : String::from("junghwan.jeong@gmail.com"),
         active: true
    };
    println!("User name  = {}", user.name);
    println!("User email  = {}", user.email);
    println!("User.active = {}", user.active);
 
    user.email = String::from("test@gmail.com");
    println!("User email  = {}", user.email);
 
 }
 ```
 

### 4. 인스턴스를 생성하는 부분을 따로 함수로 분리 할 수 있다.
```rust
fn builder_user(name: String, email: String) -> User {
    User {
        name: name,
        email: email,
        active: true
    }
}

fn main() {
   let mut user: User = builder_user(String::from("정중환"), String::from("junghwan.jeong@gmail.com"));
   println!("User name  = {}", user.name);
   println!("User email  = {}", user.email);
   println!("User.active = {}", user.active);

   user.email = String::from("test@gmail.com");
   println!("User email  = {}", user.email);
}
```

### 5. 인스턴트 복사하면 얇은 복사가 된다.
//소유권 문제에 주의 하라
```rust
fn main() {
    let user: User = builder_user(String::from("정중환"), String::from("junghwan.jeong@gmail.com"));
    let user2 =  User {
        name: user.name, //얇은 복사가 된다  - 소유권을 분실한다.
        email: user.email, //얇은 복사가 된다 - 소유권을 분실한다.
        active: user.active
    };
    //println!("user1.name = {}", user.name); //Compile error - 소유권 문제 발생

    let user3 = User {
        active: true,
        ..user2  //소유권 분실이 발생한다.
    };
    //println!("user2.name = {}", user2.name); //Compile error - 소유권이 이전 되었다.
}
```

## 🧠 구조체 심화 요약

| 개념                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| 구조체 정의                   | 관련 있는 값들을 이름을 붙여 하나의 타입으로 구성 (`struct`)         |
| 필드 접근                     | `user.name`, `user.email`처럼 명확한 이름으로 접근 가능              |
| 불변 구조체                   | 기본적으로 구조체는 불변이며, 수정하려면 `mut` 키워드 필요           |
| 가변 구조체                   | `let mut user`로 선언하면 필드 값 수정 가능                          |
| 생성 함수 분리                | `fn builder_user(...) -> User`로 인스턴스 생성 로직을 함수로 분리 가능 |
| 얕은 복사와 소유권           | 구조체 필드가 `String` 같은 소유권 타입일 경우, 복사 시 소유권 이동 발생 |
| 구조체 업데이트 문법 (`..`)   | 기존 인스턴스를 기반으로 일부 필드만 변경하여 새 인스턴스 생성 가능   |
| `impl` 블록                  | 구조체에 메서드나 연관 함수 정의 가능 (`self`, `&self`, `&mut self`) 사용 |

---
