# Static 구조체 메서드
impl 블록 안에서 self를 받지 않는 함수가 어떤 역할을 하는지, 그리고 어떻게 활용되는지를 중심으로 정리.

## 🧱 Static 구조체 메서드 (연관 함수)란?
Rust에서 구조체에 정의된 메서드는 크게 두 가지로 나뉩니다:
| 메서드 유형       | 특징 및 설명                                                       |
|-------------------|---------------------------------------------------------------------|
| 인스턴스 메서드   | `self`, `&self`, `&mut self`를 첫 번째 인자로 받음. 구조체 인스턴스에 대해 동작 |
| 연관 함수 (Static) | `self`를 받지 않음. 구조체 이름으로 직접 호출하며 주로 생성자 역할 수행       |



## 🧠 연관 함수의 특징
- impl 블록 안에서 정의되지만 self를 인자로 받지 않음
- 구조체 인스턴스를 생성하거나, 구조체와 관련된 기능을 제공
- 호출 시 구조체 이름으로 직접 호출: Rectangle::square(30)
- 일반적으로 생성자 역할을 하며, new() 또는 커스텀 생성자 이름을 사용

## 🔍 예제 분석
```rust
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // 연관 함수: self를 받지 않음
    fn squre(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // 인스턴스 메서드: &self를 받음
    fn area(self: &Self) -> i32 {
        self.width * self.height
    }
}
```

### ✅ squre(size: i32) 함수
- self를 받지 않기 때문에 연관 함수
- Rectangle 구조체의 인스턴스를 생성하는 생성자 역할
- 호출 방식: Rectangle::squre(50)
- 오타 주의: squre → square가 더 일반적인 이름입니다
### ✅ area(&self) 메서드
- 구조체 인스턴스를 참조하여 넓이를 계산
- self를 불변 참조로 받아 읽기만 수행

### 🧪 사용 예시
```rust
fn main() {
    let rect = Rectangle::squre(30); // 연관 함수로 인스턴스 생성
    println!("Rectangle = {:?}", rect);
    println!("Area = {}", rect.area()); // 인스턴스 메서드 호출
}
```

### 출력 결과:
```
Rectangle = Rectangle { width: 30, height: 30 }
Area = 900
```


## 📌 요약: 구조체 메서드와 연관 함수
| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| `impl`             | 구조체에 메서드 및 연관 함수를 정의하는 블록                         |
| `fn square()`      | 연관 함수. `self`를 받지 않으며 구조체 이름으로 호출. 생성자 역할 수행 |
| `fn area(&self)`   | 인스턴스 메서드. 구조체 인스턴스의 필드를 참조하여 동작               |
| 호출 예시          | `Rectangle::square(30)` → 연관 함수 호출<br>`rect.area()` → 인스턴스 메서드 호출 |

---



