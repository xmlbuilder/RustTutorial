# 🧠 FnOnce란?
한 번만 호출 가능한 함수형 트레잇
즉, 클로저가 **자기 내부의 값을 소비(move)** 하기 때문에 다시 호출할 수 없는 함수를 의미.


## 🔍 Fn, FnMut, FnOnce 비교
| 트레잇     | 호출 가능 횟수     | 내부 상태 변경 | 캡처 방식 |
|------------|--------------------|----------------|-----------|
| `Fn`       | 여러 번 가능        | ❌ 불변         | `&T`      |
| `FnMut`    | 여러 번 가능        | ✅ 가변         | `&mut T`  |
| `FnOnce`   | **한 번만 가능**    | ✅ 소비(move)   | `T`       |

### ✅ 예시로 기억해보자
```rust
fn call_once<F>(f: F)
where
    F: FnOnce(),
{
    f(); // 한 번만 호출
}

fn main() {
    let name = String::from("JungHwan");

    let say_hi = move || {
        println!("Hi, {}", name); // name을 move해서 가져감
    };

    call_once(say_hi); // OK
    // call_once(say_hi); // ❌ 다시 호출 불가
}
```

- move 클로저는 name을 소유권 이동해서 가져감
- 그래서 say_hi는 한 번만 호출 가능 → FnOnce

##  📘 결론
FnOnce는 소유권을 move해서 내부 값을 소비하는 클로저를 위한 트레잇.
그래서 한 번만 호출 가능하고,
Rust의 소유권 시스템과 함수형 스타일이 결합된 대표적인 개념.

----
