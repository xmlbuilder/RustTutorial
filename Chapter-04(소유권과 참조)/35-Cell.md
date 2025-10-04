# Cell

Rust에서 Cell<T>는 **내부 가변성(Interior Mutability)**을 제공하는 핵심 타입 중 하나입니다.  
즉, 불변 참조(&T)를 통해서도 값을 변경할 수 있게 해주는 구조입니다.

## ✅ Cell<T> 핵심 요약
| 항목       | 설명                                                                 |
|------------|----------------------------------------------------------------------|
| 정의       | `std::cell::Cell<T>` — 내부 가변성을 제공하는 컨테이너 타입         |
| 목적       | 불변 참조(`&T`)로도 내부 값을 변경할 수 있도록 허용                 |
| 제약       | `T: Copy` 타입만 저장 가능                                          


## 🔍 예제
```rust
use std::cell::Cell;

struct Counter {
    count: Cell<u32>,
}

fn main() {
    let counter = Counter { count: Cell::new(0) };
    counter.count.set(10); // 내부 값 변경
    println!("Count: {}", counter.count.get()); // 출력: 10
}
```

- counter는 불변으로 선언됐지만
- Cell 덕분에 내부 count는 변경 가능

---

# Cell / RefCell

이제 Cell<T>와 RefCell<T>의 차이점을 구조적으로 정리해볼게요.  
둘 다 **Interior Mutability (내부 가변성)** 을 제공하지만,  
사용 방식, 제약 조건, 성능 특성이 다릅니다.  

✅ Cell<T> vs RefCell<T> 차이 요약
| 항목         | Cell<T>                                | RefCell<T>                                 |
|--------------|-----------------------------------------|---------------------------------------------|
| 목적         | Copy 타입의 값 변경                     | 복잡한 구조체나 Vec 등 참조 타입 변경       |
| 내부 접근    | `.get()` / `.set()`                     | `.borrow()` / `.borrow_mut()`               |
| 제약         | `T: Copy` 타입만 가능                   | 어떤 타입이든 가능                          |
| 체크 방식    | 컴파일 타임                             | 런타임 (Ref 추적, 중복 borrow 시 panic)     |
| 성능         | 매우 빠름 (복사 기반)                   | 약간 느림 (런타임 체크 오버헤드 있음)       |
| 사용 예시    | 카운터, 플래그, 단일 값                 | Vec, HashMap, 구조체 내부 필드              |



## 🔍 예시 비교
### Cell<T> — 단순 값 변경
```rust
use std::cell::Cell;

let x = Cell::new(5);
x.set(10);
println!("{}", x.get()); // 출력: 10
```

### RefCell<T> — 복잡한 구조 변경
```rust
use std::cell::RefCell;

let v = RefCell::new(vec![1, 2]);
v.borrow_mut().push(3);
println!("{:?}", v.borrow()); // 출력: [1, 2, 3]

```





