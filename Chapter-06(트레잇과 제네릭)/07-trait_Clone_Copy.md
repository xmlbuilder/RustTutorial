# trait 파생

## 🧬 #[derive]란?
Rust에서 #[derive(...)]는 컴파일러가 자동으로 트레이트 구현을 생성해주는 기능입니다.  
반복적인 코드를 줄이고, 표준 트레이트에 대해 빠르게 구현할 수 있도록 도와줍니다.

- #[derive]는 트레이트에 대한 기본 구현을 자동 생성합니다.
- 기본 동작과 다른 로직이 필요할 경우에는 직접 impl로 구현해야 합니다.
- 파생 가능한 주요 트레이트 목록:
- Eq, PartialEq: 값 비교
- Ord, PartialOrd: 정렬 및 순서 비교
- Clone: 복사본 생성
- Copy: 값 복사 (소유권 이동 없이)
- Default: 기본값 생성
- Debug: {:?} 포매팅 지원

### 🧪 예제 분석: #[derive(Clone)]
```rust
#[derive(Clone)]
struct Point {
    val: i32,
}

fn main() {
    let mut p1 = Point { val: 1 };
    let p2 = p1.clone(); // 복사본 생성
    p1.val = 2;
    println!("p1 val: {}, p2 val {}", p1.val, p2.val);
    // 출력: p1 val: 2, p2 val 1
}
```


####  ✅ 동작 설명
- #[derive(Clone)] 덕분에 Point 구조체는 Clone 트레이트를 자동으로 구현합니다.
- p2 = p1.clone()은 p1의 복사본을 생성하며, p1과 p2는 서로 독립적인 인스턴스가 됩니다.
- 이후 p1.val을 변경해도 p2.val에는 영향을 주지 않음.

### 🧠 추가 설명: `Clone` vs `Copy`

| 트레이트 | 특징 및 동작 방식                                                  |
|----------|--------------------------------------------------------------------|
| `Clone`  | `.clone()` 메서드를 명시적으로 호출해야 함                          |
|          | 힙 데이터를 포함한 복잡한 타입도 복사 가능                          |
|          | `#[derive(Clone)]`로 자동 구현 가능                                 |
|          | 복사 비용이 있을 수 있음                                            |
| `Copy`   | 복사가 자동으로 이루어짐 (명시적 호출 없이 대입만으로 복사됨)       |
|          | 스택에 저장되는 단순 값만 가능 (`i32`, `bool`, `char`, `f64` 등)    |
|          | 모든 필드가 `Copy`를 구현해야 구조체에 `Copy` 파생 가능              |
|          | `#[derive(Copy, Clone)]`로 함께 사용되는 경우가 많음                | | 
| Copy |  | 


## 📌 Copy가 가능한 기본 타입 (이미지 내용 기반)
- 정수형: u8, i16, u32, i64 등
- 불리언: bool
- 실수형: f32, f64
- 문자형: char
→ 구조체가 Copy를 파생하려면 모든 필드가 Copy를 구현하고 있어야 합니다.

## 🧩 직접 구현이 필요한 경우
기본 동작이 아닌 커스텀 로직이 필요할 경우, Clone을 직접 구현할 수 있습니다:
```rust
impl Clone for Point {
    fn clone(&self) -> Self {
        println!("Custom clone called!");
        Point { val: self.val }
    }
}
```

→ 이렇게 하면 .clone() 호출 시 사용자 정의 동작을 수행할 수 있어요.

### 🎯 정리: `#[derive]`, `Clone`, `Copy`

| 항목         | 설명                                                                 |
|--------------|----------------------------------------------------------------------|
| `#[derive]`   | 트레이트의 기본 구현을 자동으로 생성하는 애트리뷰트                   |
| `Clone`       | `.clone()` 메서드를 명시적으로 호출하여 복사본을 생성함               |
| `Copy`        | 복사가 자동으로 이루어짐 (`let b = a;` 형태로 사용 가능)              |
| 직접 구현 (`impl`) | 기본 동작과 다르게 하고 싶을 때 트레이트를 수동으로 구현함             |
| `Copy + Clone` | `Copy`를 파생하려면 `Clone`도 함께 구현되어 있어야 함 (`#[derive(Copy, Clone)]`) |


---

# vec![value; count] 문법이 왜 Clone을 요구

Rust의 메모리 모델과 제네릭 동작을 이해하는 데 아주 중요한 포인트. 
vec![value; count] 문법이 왜 Clone을 요구하는지, 그리고 그 작동 원리를 자세히 설명.

## 🧠 vec![value; count]의 작동 원리
```rust
let pool = vec![None; pool_size];
```

이 문법은 다음과 같은 의미입니다:
“None이라는 값을 pool_size만큼 복제해서 벡터를 만든다.”

Rust는 이걸 내부적으로 이렇게 처리합니다:
```rust
let mut v = Vec::with_capacity(pool_size);
for _ in 0..pool_size {
    v.push(value.clone());
}
```

즉, value.clone()을 반복해서 호출하는 방식이에요.  
그래서 value가 Clone 트레이트를 구현하고 있어야 합니다.

## ❗ 왜 Clone이 필요할까?
Rust는 안전한 메모리 모델을 유지하기 위해, 값을 복사할 때 반드시 명시적인 복제 방법을 요구합니다.   Copy는 비트 복사만 가능한 타입에만 허용되고, Box, Vec, String 같은 힙 기반 타입은 반드시 Clone을 구현해야 복제할 수 있음.  
Option<Box<Item>>는 힙에 있는 데이터를 가리키는 포인터이기 때문에, 단순 복사(Copy)가 불가능하고, 복제하려면 Clone이 필요합니다.  

### ✅ 예시: Clone이 있는 경우
```rust
#[derive(Clone)]
struct Item {
    data: usize,
    next: Option<Box<Item>>,
}

let pool: Vec<Option<Box<Item>>> = vec![None; 10]; // OK
```


### ❌ 예시: Clone이 없는 경우
```rust
struct Item {
    data: usize,
    next: Option<Box<Item>>,
}

let pool: Vec<Option<Box<Item>>> = vec![None; 10]; // ❌ 오류 발생


오류 메시지:
the trait `Clone` is not implemented for `Box<Item>`
```


### ✅ 대안: resize_with() 사용
복제 없이도 None을 반복 생성하려면 이렇게 하면 됩니다:
let mut pool = Vec::with_capacity(pool_size);
pool.resize_with(pool_size, || None);

이 방식은 Clone 없이도 안전하게 초기화할 수 있어요.


### 🧠 왜 `#[derive]`가 구조 설계에 영향을 줄까?

| 트레이트     | 기능 설명                          | 구조적 파급 효과                              |
|--------------|------------------------------------|-----------------------------------------------|
| `Clone`      | 깊은 복제 수행                     | 불필요한 메모리 복제 발생 가능성              |
| `Copy`       | 비트 단위 복사                     | 수명 관리 무시 → `Drop` 무시됨                |
| `Debug`      | `{:?}` 포맷으로 출력 가능          | 민감 정보 노출 가능성, 로깅 범위 확대          |
| `PartialEq`  | `==` 비교 가능                     | 의미 없는 비교 허용 → 논리 오류 유발 가능     |
| `Default`    | 기본값 자동 생성                   | 필드 누락 감지 어려움 → 초기화 실수 유발 가능 |



## ✨ 요약
| 문법                          | 내부 동작 방식            | 요구 조건         |
|------------------------------|---------------------------|-------------------|
| `vec![value; count]`         | `value.clone()` 반복 호출 | `value: Clone` 필요 |
| `resize_with(count, || ...)` | 클로저를 `count`번 실행   | `Clone` 불필요      |

---




