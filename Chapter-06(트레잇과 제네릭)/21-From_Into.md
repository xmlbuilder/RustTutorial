# Into

Into::into는 Rust에서 타입 변환을 위한 표준 방식 중 하나입니다.

## 🔄 Into란 무엇인가?
Into는 Rust의  트레잇입니다. 한 타입을 다른 타입으로 변환할 수 있도록 정의된 인터페이스예요.
```rust
pub trait Into<T> {
    fn into(self) -> T;
}
```

즉, into() 메서드를 호출하면 self를 T 타입으로 변환합니다.

## 🧠 핵심 개념
- Into는 소유권을 이동하면서 변환합니다 (self를 소비).
- Into는 From 트레잇이 구현되어 있으면 자동으로 구현됩니다.
- Into는 제네릭 함수에서 타입 추론을 유도하거나 강제할 때 유용합니다.

## 🧪 예제
### 1. 기본 타입 변환
```rust
let s: String = "hello".into(); // &str → String
```

이건 내부적으로 String::from("hello")와 동일합니다.

## 2. 사용자 정의 타입 변환
```rust
struct MyInt(i32);

impl From<i32> for MyInt {
    fn from(n: i32) -> Self {
        MyInt(n)
    }
}

fn main() {
    let x: MyInt = 42.into(); // i32 → MyInt
}
```

여기서 From<i32>를 구현했기 때문에 Into<MyInt>도 자동으로 구현됩니다.


## 🧩 From vs Into
|  트레잇 | 변환 방향 | 구현 위치 | 사용 방식 |
|-----------|------------|----------------------|----------------|
| `From<T>` | `T → Self` | 변환 **대상 타입**에 구현 | `T::from(x)` |
| `Into<T>` | `Self → T` | 변환 **원본 타입**에 구현 (`From`이 있으면 자동 구현됨) | `x.into()` |


### 즉, From을 구현하면 Into는 자동으로 따라옵니다, 하지만 그 반대는 아닙니다.


## ⚠️ 주의할 점
- into()는 타입 추론이 어려울 수 있습니다. 이럴 땐 명시적으로 타입을 지정해야 합니다:
```rust
let x = 42;
let y = Into::<MyInt>::into(x); // 명시적 타입 지정
```

또는:
```rust
let y: MyInt = x.into(); // 변수 타입으로 유도
```


## 💡 언제 쓰면 좋을까?
- 제네릭 함수에서 다양한 입력 타입을 받아 처리할 때
- API에서 유연한 타입 변환을 허용하고 싶을 때
- From을 구현한 타입에 대해 .into()를 통해 간결한 변환을 하고 싶을 때


---

# From

From 트레잇은 Rust에서 타입 간 변환을 정의하는 가장 기본적인 방식입니다.  
Into와 짝을 이루며, 변환 대상 타입이 스스로 "나는 이런 타입으로부터 만들어질 수 있어!"라고 선언하는 구조.

## 🔄 From 트레잇이란?
```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}
```

- T → Self로 변환하는 방법을 정의합니다.
- 변환은 실패하지 않는(infallible) 것이 전제입니다.
- From을 구현하면 자동으로 Into도 구현됩니다.

## 🧪 예제: 기본 타입 변환
```rust
let s: String = String::from("hello"); // &str → String
```


### 이건 사실상 "hello".into()와 동일합니다. String이 From<&str>을 구현했기 때문이죠.

## 🧪 예제: 사용자 정의 타입
```rust
struct MyInt(i32);

impl From<i32> for MyInt {
    fn from(n: i32) -> Self {
        MyInt(n)
    }
}

fn main() {
    let x = MyInt::from(42);      // 명시적 From
    let y: MyInt = 42.into();     // 자동 Into
}
```


### From<i32>을 구현하면 Into<MyInt>도 자동으로 구현되므로 into()도 사용할 수 있어요.


## 🧠 From vs Into 비교
| 트레잇 비교 | 변환 방향 | 구현 위치 | 사용 방식 |
|----------------|------------|-------------|-------------|
| `From<T>`       | `T → Self` | 변환 **대상 타입**에 구현 | `T::from(x)` |
| `Into<T>`       | `Self → T` | 변환 **원본 타입**에 구현 (보통 자동 구현됨) | `x.into()` |




## 💡 언제 From을 구현하나요?
- 타입 간 변환이 명확하고 실패하지 않을 때
- API에서 다양한 입력을 받아 처리하고 싶을 때
- Into를 자동으로 얻고 싶을 때

---

# 이터레이터 체이닝을 활용해 데이터를 변환 (Into::into)

Rust에서 이터레이터 체이닝을 활용해 데이터를 변환하고 합산하는 매우 idiomatic한 방식.
🧠 전체 구조
```rust
self.data.iter().copied().map(Into::into).sum()
```

이 코드는 self.data라는 벡터(Vec<T>)의 요소들을 복사 → 변환 → 합산하는 흐름입니다.

## 🔍 단계별 분석
### 1. self.data.iter()
- &self.data의 불변 참조 이터레이터를 생성합니다.
- 타입: std::slice::Iter<'_, T>
- 반환되는 값은 &T (참조)

### 2. .copied()
- &T → T로 복사합니다.
- T: Copy 트레잇이 구현되어 있어야 사용 가능
- 타입: T (값)
🔸 이 단계에서 참조를 값으로 바꾸므로 이후 .map()에서 T를 직접 다룰 수 있게 됩니다.


### 3. .map(Into::into)
- 각 T 값을 Into<f64>을 통해 f64로 변환합니다.
- Into::into는 Fn(T) -> f64로 동작
- T가 Into<f64>를 구현하고 있어야 합니다
🔸 이건 map(|x| x.into())와 동일한 표현입니다.


### 4. .sum()
- 이터레이터의 모든 f64 값을 합산합니다.
- 타입: f64
- 내부적으로 fold(0.0, |acc, x| acc + x)와 유사하게 동작

## ✅ 전체 흐름 요약
| 단계               | 입력 타입 | 동작 설명              | 출력 타입 |
|--------------------|-----------|-------------------------|-----------|
| `iter()`           | `&[T]`    | 슬라이스의 참조 이터레이터 생성 | `Iter<'_, T>` |
| `copied()`         | `&T`      | 참조를 값으로 복사 (`T: Copy`) | `T`       |
| `map(Into::into)`  | `T`       | `T`를 `f64`로 변환 (`Into<f64>`) | `f64`     |
| `sum()`            | `f64`     | 모든 값을 합산             | `f64`     |



## 💡 예시
```rust
let data = vec![1u32, 2, 3];
let total: f64 = data.iter().copied().map(Into::into).sum();
println!("{}", total); // 출력: 6.0
```

여기서 u32는 Into<f64>를 구현하고 있으므로 변환이 가능합니다.

----




