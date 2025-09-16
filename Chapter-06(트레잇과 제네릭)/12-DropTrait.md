# Drop Trait
Drop 트레잇은 Rust의 자원 해제와 메모리 정리를 자동으로 처리하는 핵심 기능.
Drop 트레잇의 원리와 주의할 점까지 정리 + 해설.

## 🧠 Drop 트레잇이란?
Rust에서 Drop은 값이 스코프를 벗어나면서 메모리에서 삭제될 때 자동 호출되는 트레잇입니다.
### ✅ 정의
```rust
trait Drop {
    fn drop(&mut self);
}
```

- drop() 메서드는 값이 삭제되기 직전에 호출됩니다.
- 보통 파일 닫기, 네트워크 연결 해제, 로그 출력, 메모리 해제 등에 사용됩니다.

## 📦 Drop 트레잇의 사용 목적
Drop은 다음 두 경우에 사용됩니다:
- 값이 메모리에서 삭제될 때 추가 처리가 필요한 경우
- 범위를 벗어나기 전에 강제로 값을 삭제하고 싶은 경우

### 🔍 예제 ①: 스코프 기반 자동 삭제
```rust
struct DropExample { value: i32 }

impl Drop for DropExample {
    fn drop(&mut self) {
        println!("Drop example value is {}", self.value);
    }
}

fn main() {
    let _x = DropExample { value: 5 };
    {
        let _y = DropExample { value: 10 };
    }
    // 출력:
    // Drop example value is 10
    // Drop example value is 5
}
```


- _y는 내부 블록이 끝나면서 먼저 삭제됨
- _x는 main() 함수가 끝나면서 삭제됨
- Rust는 스코프 종료 순서대로 drop 호출

### ❗ 예제 ②: drop() 호출 후 값 접근 시 에러
```rust
fn main() {
    let x = DropExample { value: 5 };
    drop(x); // 명시적으로 삭제
    let _y = x.value; // ❌ 에러 발생
}
```

🔥 에러 메시지 요약
error[E0382]: use of moved value: `x`


- DropExample은 Copy 트레잇을 구현하지 않았기 때문에 drop(x) 호출 시 소유권이 이동
- 이후 x.value를 접근하려 하면 이미 삭제된 값에 접근하려는 것이므로 컴파일 에러 발생


### 🧪 해결 방법
- Clone을 구현해서 복사본을 만들거나
- Option<T>로 감싸고 .take()로 안전하게 꺼내는 방식도 가능
```rust
struct DropExample { value: i32 }

impl Drop for DropExample {
    fn drop(&mut self) {
        println!("Drop example value is {}", self.value);
    }
}

fn main() {
    let mut x = Some(DropExample { value: 5 });
    let val = x.take().unwrap().value;
    println!("Value taken: {}", val);
}
```



## ✅ Drop 트레잇 요약

| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| 목적               | 값이 삭제될 때 자동으로 호출되는 정리 로직                           |
| 호출 시점          | 스코프 종료 또는 `drop()` 명시 호출                                   |
| 주의사항           | drop 호출 후 해당 값은 더 이상 접근 불가 (소유권 이동됨)             |
| 대표 사용 예       | 파일 닫기, 로그 출력, 메모리 해제, 네트워크 연결 종료 등              |
| 에러 방지 방법     | `Clone` 구현, `Option<T>`로 감싸기, `Ref`로 접근                      |

---

# 핵심 포인트
```rust
let x = DropExample { value: 5 };
```
- DropExample { value: 10 }는 새로운 구조체 인스턴스 생성
- value: 10은 i32 타입 → Copy 트레잇을 구현하고 있어서 복사됨
- _y는 이 새 인스턴스의 **소유자(owner)**가 됨
- 이 인스턴스는 _y가 스코프를 벗어날 때 drop() 호출됨

## 🔍 소유권 이전과 비교
```rust
let a = DropExample { value: 10 };
let b = a; // ✅ 여기서는 소유권이 a → b로 이동
```

- DropExample은 Copy를 구현하지 않았기 때문에 a는 이후에 사용할 수 없음
- 이게 **소유권 이전(move)**의 대표적인 예

## ✅ 복사 가능한 필드가 있다고 해서 구조체 전체가 Copy 되는 건 아님
```rust
struct DropExample { value: i32 } // i32는 Copy지만 DropExample은 Copy 아님
```

- Rust에서는 Drop 트레잇을 구현한 타입은 자동으로 Copy를 구현할 수 없음
- 즉, DropExample은 Copy가 아니므로 복사 대신 move가 발생함

## 🧪 확인 예제
```rust
fn main() {
    let x = DropExample { value: 5 };
    let y = x; // move 발생
    println!("{}", y.value); // ✅
    // println!("{}", x.value); // ❌ 컴파일 에러: x는 move됨
}
```


## ✅ 구조체 생성 vs 소유권 이전

| 코드                              | 설명                                      |
|-----------------------------------|-------------------------------------------|
| `let _y = DropExample { value: 10 };` | 새 인스턴스 생성. 소유권 이전 아님         |
| `let b = a;`                      | `a`의 소유권을 `b`로 이동 (move 발생)     |
| `i32`                             | Copy 타입이므로 값 복사 가능               |
| `DropExample`                     | Drop 구현 시 Copy 불가 → move로 동작       |


---

# 소유권 이전 
소유권 이전(move)을 안전하게 제어하기 위한 패턴이에요.
핵심은 Option<T>를 활용해서 DropExample의 인스턴스를 감싸고,
필요할 때 take()로 소유권을 꺼내는 방식을 사용하는 거죠.

## 🧠 코드 분석
```rust
let mut x = Some(DropExample { value: 5 });
let val = x.take().unwrap().value;
println!("Value taken: {}", val);
```

## ✅ 동작 흐름
- x는 Option<DropExample> 타입 → 값이 Some(...)으로 감싸져 있음
- x.take()는 x에서 값을 꺼내고, x를 None으로 바꿈
- unwrap()으로 Some을 열고, 내부의 DropExample을 가져옴
- .value로 내부 필드에 접근

## 🔐 왜 이렇게 하는가?
- DropExample은 Drop을 구현했기 때문에 Copy가 불가능
- 일반적으로 let val = x.value;처럼 직접 접근하면 move가 발생해서 이후 x는 사용할 수 없음
- Option<T>로 감싸면 값을 꺼내는 시점을 제어할 수 있음
- take()는 내부 값을 move하면서 꺼내고, 원래 변수는 None으로 남음

## 🧪 시각적 흐름
```
Before take():
x = Some(DropExample { value: 5 })

After take():
x = None
val = DropExample { value: 5 }
```

- x는 더 이상 원래 값을 갖고 있지 않음 → 안전하게 소유권 이전 완료

## 📦 이 패턴이 유용한 상황

| 상황 또는 목적                          | 설명                                                                 |
|-----------------------------------------|----------------------------------------------------------------------|
| 소유권을 안전하게 이전하고 싶을 때      | `take()`를 사용하면 값의 move를 명시적으로 제어할 수 있음             |
| 값이 스코프 밖으로 나가기 전에 조작할 때| `Option<T>`로 감싸면 삭제 전에 원하는 로직을 실행할 수 있음           |
| 내부 상태를 초기화하거나 재설정할 때     | `take()` 호출 후 `Option<T>`는 `None`이 되어 상태 초기화가 가능        |
| Drop을 구현한 타입의 자원 해제를 제어할 때 | `Option<T>`로 감싸면 drop 시점을 명확하게 조절할 수 있음              |




## ✅ Option<T> + take() 패턴 요약
- `Option<T>`로 감싸면 값의 존재 여부와 소유권을 함께 관리할 수 있음
- `take()`는 내부 값을 move하면서 꺼내고, 원래 변수는 `None`으로 초기화됨
- Drop을 구현한 타입에서 소유권 이전을 안전하게 제어할 수 있는 실전 패턴









