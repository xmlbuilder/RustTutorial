# 참조 조합 + slice 조합

- Rust를 쓰다 보면 **참조 조합(&, &mut) + slice 조합([T], [&T], [&mut T])** 이 너무 많아서  
    **이걸 언제 어떻게 만들어서 함수에 넘겨야 하지?** 라는 혼란이 생김.
- 그래서 한 번 전체적으로 정리해두면 Rust가 훨씬 단순하게 느껴진다.
- 아래는 Rust에서 가장 많이 쓰는 참조/슬라이스 조합 4가지를  
    Vec / array / iter_mut / iter로 어떻게 만들어서 함수에 넘기는지 완전 정리한 표다.

| 타입 형태              | 의미 설명                                      | Vec/array에서 만드는 방법                                 | 함수 인자 형태 예시                         |
|------------------------|------------------------------------------------|------------------------------------------------------------|----------------------------------------------|
| &[&T]                 | 불변 참조들의 불변 슬라이스                    | let v: Vec<&T> = items.iter().collect();                  | fn f(xs: &[&T])                              |
| &mut [&mut T]         | 가변 참조들의 가변 슬라이스                    | let mut v: Vec<&mut T> = items.iter_mut().collect();      | fn f(xs: &mut [&mut T])                      |
| &[&mut T]             | 가변 참조들의 불변 슬라이스                    | let v: Vec<&mut T> = items.iter_mut().collect();          | fn f(xs: &[&mut T])                          |
| &mut [&T]             | 불변 참조들의 가변 슬라이스 (거의 안 씀)       | let mut v: Vec<&T> = items.iter().collect();              | fn f(xs: &mut [&T])                          |

## 🔍 각 타입이 실제로 언제 쓰이는지
### 1) &[&T]
- 불변 참조들의 불변 슬라이스.
    - 여러 개의 &T를 읽기만 할 때
    - 가장 흔함
- 만드는 방법:
```rust
let refs: Vec<&T> = items.iter().collect();
f(&refs[..]);
```


### 2) &mut [&mut T]
- 가변 참조들의 가변 슬라이스.
    - 여러 개의 &mut T를 함수에서 수정해야 할 때
    - NURBS surface 리스트를 수정하는 함수가 딱 이 형태
- 만드는 방법:
```rust
let mut refs: Vec<&mut T> = items.iter_mut().collect();
f(&mut refs[..]);
```


### 3) &[&mut T]
- 가변 참조들의 불변 슬라이스.
    - 여러 개의 &mut T를 넘기지만
    - 함수 내부에서는 슬라이스 자체를 수정하지 않고, 각 요소만 수정할 때
- 만드는 방법:
```rust
let refs: Vec<&mut T> = items.iter_mut().collect();
f(&refs[..]);
```


### 4) &mut [&T]
- 불변 참조들의 가변 슬라이스.
    - 거의 안 쓰임
    - 슬라이스 순서를 바꾸거나 swap할 때만 필요
- 만드는 방법:
```rust
let mut refs: Vec<&T> = items.iter().collect();
f(&mut refs[..]);
```


## 🔥 핵심 요약 (진짜 중요한 부분)
- ✔ 여러 개의 &mut T를 함수에 넘기고 싶다
    - Vec<&mut T> + &mut slice
```rust
let mut refs: Vec<&mut T> = items.iter_mut().collect();
f(&mut refs[..]);
```

- ✔ 여러 개의 &T를 함수에 넘기고 싶다
    - Vec<&T> + slice
```rust
let refs: Vec<&T> = items.iter().collect();
f(&refs[..]);
```

- ✔ iter_mut → &mut T
- ✔ iter → &T
- 이 두 개만 기억하면 80% 해결된다.

## 📌 Rust 참조/슬라이스 4종류 완전 정리
| 타입 형태        | 의미 설명                               | 함수에서 가능한 조작                         | Vec/array에서 만드는 방법                                   |
|------------------|-------------------------------------------|-----------------------------------------------|--------------------------------------------------------------|
| &[&T]            | 불변 참조들의 불변 슬라이스               | 요소 읽기만 가능                              | let v: Vec<&T> = items.iter().collect(); f(&v[..]);          |
| &mut [&mut T]    | 가변 참조들의 가변 슬라이스               | 슬라이스 재배열 + 각 요소 수정 가능           | let mut v: Vec<&mut T> = items.iter_mut().collect(); f(&mut v[..]); |
| &[&mut T]        | 가변 참조들의 불변 슬라이스               | 요소 수정 가능 (슬라이스 자체는 불변)         | let v: Vec<&mut T> = items.iter_mut().collect(); f(&v[..]);  |
| &mut [&T]        | 불변 참조들의 가변 슬라이스               | 슬라이스 재배열 가능 (요소는 수정 불가)       | let mut v: Vec<&T> = items.iter().collect(); f(&mut v[..]);  |


---
# 상세 설명

## 🔍 각 타입에서 “무엇을 조작할 수 있는지” 핵심 설명
- Rust에서 조작 가능 여부는 두 가지 축으로 결정돼:
    - 슬라이스 자체가 가변인지 (&mut [...])
    - 슬라이스 안의 요소가 가변인지 (&mut T)
- 이 두 가지를 조합하면 네 가지 경우가 나온다.

### 1) &[&T]
- 불변 슬라이스 + 불변 참조
    - 슬라이스 수정 불가
    - 요소 수정 불가
    - 읽기만 가능
- 예:
```rust
fn f(xs: &[&T]) {
    for x in xs {
        println!("{:?}", x);
    }
}
```


### 2) &mut [&mut T]
- 가변 슬라이스 + 가변 참조
    - 슬라이스 재배열 가능 (swap, sort_by, etc.)
    - 요소 수정 가능 (*xs[i] = ...)
    - 가장 강력한 형태
- 예:
```rust
fn f(xs: &mut [&mut T]) {
    xs.swap(0, 1);     // 슬라이스 조작 가능
    *xs[0] = new_val;  // 요소 조작 가능
}
```


### 3) &[&mut T]
- 불변 슬라이스 + 가변 참조
    - 슬라이스 재배열 불가
    - 요소 수정 가능
    - 요소만 바꾸고 싶을 때 사용
- 예:
```rust
fn f(xs: &[&mut T]) {
    *xs[0] = new_val;  // OK
    // xs.swap(0,1);   // ❌ 불변 슬라이스라 불가
}
```


### 4) &mut [&T]
- 가변 슬라이스 + 불변 참조
    - 슬라이스 재배열 가능
    - 요소 수정 불가
    - 순서만 바꾸고 싶을 때 사용
- 예:
```rust
fn f(xs: &mut [&T]) {
    xs.swap(0, 1);  // OK
    // *xs[0] = ... // ❌ 불변 참조라 불가
}
```


## 🔥 핵심 요약 (이해 포인트)
- ✔ 슬라이스가 &mut이면 → 슬라이스 구조 변경 가능
    - swap
    - sort
    - reorder
    - push/pop은 불가 (슬라이스는 길이 고정)
- ✔ 요소가 &mut이면 → 요소 값 변경 가능
- ✔ 둘 다 mut이면 → 슬라이스 + 요소 둘 다 조작 가능
- ✔ 둘 다 불변이면 → 읽기만 가능

## 🔥 실제로 Vec/array에서 어떻게 만들어지는가
- iter → &T
```rust
let refs: Vec<&T> = items.iter().collect();
```
- iter_mut → &mut T
```rust
let refs: Vec<&mut T> = items.iter_mut().collect();
```
- Vec → slice
- &refs[..]        // &[&T] 또는 &[&mut T]
- &mut refs[..]    // &mut [&T] 또는 &mut [&mut T]
---

## as_slice 
- 바로 as_slice()로 네 가지 형태를 어떻게 만들고, 어떤 조작이 가능한지에 대한 정리.
- Rust에서 as_slice()는 사실 “Vec → &[T]” 변환기일 뿐.
- 즉:
    - Vec<T> → &[T]
    - Vec<&T> → &[&T]
    - Vec<&mut T> → &[&mut T]
- 그리고 as_mut_slice()는:
    - Vec<T> → &mut [T]
    - Vec<&T> → &mut [&T]
    - Vec<&mut T> → &mut [&mut T]
- 즉, Vec의 요소 타입이 무엇이냐에 따라 네 가지 조합이 자동으로 만들어진다.
- 그래서 네가 말한 네 가지 타입을 as_slice() / as_mut_slice()로 모두 만들 수 있음.

##  Rust 참조/슬라이스 4종류 + as_slice/as_mut_slice 정리
| 타입 형태    | 의미 설명       | as_slice / as_mut_slice로 만드는 방법   | 가능한 조작   |
|-------------|-------------------------|-------------------------------------|-----------------------|
|&[&T]        |불변 참조들의 불변 슬라이스 |let v: Vec<&T> = ...; v.as_slice() | 요소 읽기만 가능 |
|&mut[&mut T]|가변 참조들의 가변 슬라이스 |let mut v: Vec<&mut T> = ...; v.as_mut_slice() | 슬라이스 재배열 + 요소 수정 가능  |
|&[&mut T] |가변 참조들의 불변 슬라이스 |let v: Vec<&mut T> = ...; v.as_slice() | 요소 수정 가능 (슬라이스 자체는 불변)  |
|&mut [&T] |불변 참조들의 가변 슬라이스 |let mut v: Vec<&T> = ...; v.as_mut_slice() | 슬라이스 재배열 가능 (요소는 수정 불가)  |



## 🔥 핵심은 “Vec의 요소 타입”이 네 가지 조합을 결정한다
### 1) Vec<&T>
- as_slice() → &[&T]
- as_mut_slice() → &mut [&T]
### 2) Vec<&mut T>
- as_slice() → &[&mut T]
- as_mut_slice() → &mut [&mut T]
- 즉:
    - Vec의 요소가 &T인지 &mut T인지
    - 슬라이스가 &인지 &mut인지
- 이 두 가지 조합으로 네 가지 타입이 자동 생성된다.

## 🔥 실제 예제들
### ✔ 1) 여러 개의 불변 참조를 읽기만 하고 싶다 → &[&T]
```rust
let v: Vec<&T> = items.iter().collect();
let s: &[&T] = v.as_slice();
```


### ✔ 2) 여러 개의 가변 참조를 수정하고 싶다 → &mut [&mut T]
```rust
let mut v: Vec<&mut T> = items.iter_mut().collect();
let s: &mut [&mut T] = v.as_mut_slice();
```


### ✔ 3) 여러 개의 가변 참조를 넘기지만 슬라이스는 불변 → &[&mut T]
```rust
let v: Vec<&mut T> = items.iter_mut().collect();
let s: &[&mut T] = v.as_slice();
```


### ✔ 4) 여러 개의 불변 참조를 넘기지만 슬라이스는 가변 → &mut [&T]
```rust
let mut v: Vec<&T> = items.iter().collect();
let s: &mut [&T] = v.as_mut_slice();
```


## 🔥 왜 이게 강력하냐?
- 너는 이제 다음만 보면 바로 고칠 수 있어:
    - Vec의 요소 타입이 무엇인지
    - 슬라이스가 &인지 &mut인지
- 이 두 가지로 네 가지 조합이 자동으로 만들어지니까.
- 즉:
    - Vec<&mut T> → as_mut_slice() → &mut [&mut T]
    - Vec<&mut T> → as_slice() → &[&mut T]
    - Vec<&T> → as_slice() → &[&T]
    - Vec<&T> → as_mut_slice() → &mut [&T]

---
