# 🧱 구조체란 무엇인가요?
Rust에서 **구조체(struct)**는 여러 개의 관련된 데이터를 하나의 사용자 정의 타입으로 묶는 방법입니다.  
객체지향 언어의 클래스와 비슷한 역할을 하며, 데이터를 더 명확하고 의미 있게 표현할 수 있게 해줍니다.

## 🔁 예제 비교로 이해하기
### 1. 일반 변수 사용
```rust
fn main() {
    let width = 20;
    let height = 30;
    println!("area = {}", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

- 단점: width와 height가 서로 관련된 값이라는 의미가 코드상에 드러나지 않음.
- 확장성 부족: 만약 사각형 외에 다른 도형을 추가하려면 관리가 어려워짐.

### 2. 튜플 사용
```rust
fn main() {
    let rect = (20, 30);
    println!("area = {}", area(rect));
}

fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
```

- 장점: 관련된 값을 하나로 묶음.
- 단점: rect.0, rect.1처럼 의미 없는 숫자 인덱스로 접근해야 함 → 가독성 저하.

### 3. 구조체 사용
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    println!("area = {}", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

- 장점:
- rect.width, rect.height처럼 의미 있는 이름으로 접근 가능.
- 확장성 뛰어남: 나중에 color, position 같은 필드를 추가하기 쉬움.
- 재사용성 향상: 여러 함수에서 Rectangle 타입을 일관되게 사용할 수 있음.

## 🧾 구조체 디버깅 출력
Rust에서는 구조체를 println!으로 출력하려면 Debug 트레이트를 명시적으로 추가해야 합니다.
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("rect = {:?}", rect); // Debug 출력
}
```

- #[derive(Debug)]는 컴파일러에게 이 구조체를 디버깅용으로 출력할 수 있게 해달라는 지시.
- {:?}는 Debug 포맷으로 출력하라는 의미.

## 🕵️‍♂️ dbg! 매크로로 디버깅
```rust
fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    dbg!(rect);
}
```

- dbg!는 파일명, 줄번호, 변수명까지 함께 출력해주는 디버깅 도구.
- 개발 중에 값이 제대로 들어갔는지 확인할 때 매우 유용.


## ✨ 구조체의 핵심 원리 요약

| 개념                     | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| 데이터 묶음              | 관련된 여러 값을 하나의 타입으로 묶어 표현                          |
| 가독성 향상              | `rect.width`, `rect.height`처럼 의미 있는 이름으로 필드 접근 가능     |
| 확장성                   | 새로운 필드 추가가 쉬워 유지보수에 유리                             |
| 재사용성                 | 여러 함수나 모듈에서 구조체 타입을 일관되게 사용 가능                |
| 디버깅 지원              | `#[derive(Debug)]`와 `dbg!` 매크로를 통해 구조체 내용을 쉽게 출력 가능 |

---



