# clone_from_slice / copy_from_slice

clone_from_slice는 Rust에서 **한 슬라이스의 내용을 다른 슬라이스로 복제(clone)**할 때 사용하는 메서드.
단순한 복사처럼 보이지만, 이 메서드는 Clone 트레이트를 구현한 타입에 대해 안전하게 복제를 수행.

## 🧠 기본 개념
```rust
fn clone_from_slice(&mut self, src: &[T])
```

- self: 복사 대상 (&mut [T])
- src: 복사 원본 (&[T])
- T: 반드시 Clone을 구현해야 함
두 슬라이스의 길이가 같아야 하며, 그렇지 않으면 panic이 발생합니다.


## ✅ 사용 예시
```rust
let mut dst = vec![0; 5];
let src = vec![1, 2, 3, 4, 5];

dst.clone_from_slice(&src);

assert_eq!(dst, src);
```


- dst는 &mut [T]로 변환 가능하고
- src는 &[T]로 전달됨
- 내부적으로 src[i].clone()을 반복해서 dst[i]에 복제함

## 🔍 비교: clone_from_slice vs copy_from_slice
| 메서드 | 요구 트레이트 | 동작 방식            | 성능 특성     |
|---------------------------------------------|----------------|----------------------|----------------|
| `clone_from_slice`                          | `Clone`        | 깊은 복사 (`clone()`) | 유연하지만 느림 |
| `copy_from_slice`                           | `Copy`         | 얕은 복사 (`memcpy`) | 매우 빠름       |


clone_from_slice는 더 일반적이고 유연하지만,
copy_from_slice는 더 빠르고 제한적이에요.


## ⚠️ 주의사항
- 길이가 다르면 panic 발생
- 이미 초기화된 self의 요소는 drop되지 않음 → 기존 값은 덮어쓰기됨
- T가 Clone을 구현하지 않으면 컴파일 에러

## ✨ 언제 쓰면 좋은가?
- Vec<T>나 Box<[T]> 같은 슬라이스 기반 타입에
다른 슬라이스의 값을 복제하고 싶을 때
- T가 Clone만 있고 Copy는 없는 경우
예: String, Vec<u8>, Arc<T>, Box<T> 등



## 🧩 기본 비교
| 비교 항목               | `copy_from_slice`         | `clone_from_slice`              |
|------------------------|----------------------------|----------------------------------|
| 트레이트 요구사항       | `T: Copy`                  | `T: Clone`                       |
| 복사 방식               | 얕은 복사 (`memcpy`)       | 깊은 복사 (`clone()`)           |
| 사용 가능한 타입 예시   | `u8`, `i32`, `bool`, `f64` | `String`, `Vec<T>`, `Box<T>`, `Arc<T>` |
| 슬라이스 길이 불일치 시 | panic 발생                 | panic 발생                       |
| 성능 특성               | 매우 빠름                  | 유연하지만 느림                  |



## ⚠️ 에러가 나는 이유: Copy vs Clone
### 🔹 copy_from_slice는 Copy 타입만 가능
```rust
let mut dst: [String; 3] = Default::default();
let src = [String::from("a"), String::from("b"), String::from("c")];

dst.copy_from_slice(&src); // ❌ 컴파일 에러: String은 Copy가 아님
```

- String은 힙에 데이터를 가지고 있어서 얕은 복사로는 안전하지 않음
- Copy는 비트 단위 복사만 허용 → String, Vec, Box 등은 Copy 불가

### 🔹 clone_from_slice는 Clone만 있으면 OK
```rust
let mut dst: [String; 3] = Default::default();
let src = [String::from("a"), String::from("b"), String::from("c")];

dst.clone_from_slice(&src); // ✅ 정상 작동
```

- clone()은 각 요소를 깊은 복사하므로 안전하게 복제 가능
- 내부적으로 dst[i] = src[i].clone()이 반복됨

### 🔍 실제 상황에서 흔히 겪는 패턴
```rust
let mut dst = vec![String::new(); 3];
let src = vec![String::from("a"), String::from("b"), String::from("c")];

dst.copy_from_slice(&src); // ❌ 에러 발생
dst.clone_from_slice(&src); // ✅ 정상 작동
```

이럴 때 에러 메시지는 보통:
```
the trait bound String: Copy is not satisfied
```

## 🧠 정리: 언제 어떤 걸 써야 할까?
| 타입 제약 (`T`)         | 사용 가능한 메서드       |
|------------------------|--------------------------|
| `T: Copy`              | `copy_from_slice`        |
| `T: Clone`             | `clone_from_slice`       |
| `T: !Copy + Clone`     | `clone_from_slice`       |
| `T: !Clone`            | ❌ 사용 불가 (컴파일 에러) |



## ✨ 추가 팁
- Copy는 자동 구현되지만, Clone은 명시적 구현이 필요함
- Copy와 Drop은 양립 불가 → Drop이 있는 타입은 Copy 불가
- clone_from_slice는 기존 값을 덮어쓰기만 하므로, 이전 값의 drop은 자동 처리됨

---

