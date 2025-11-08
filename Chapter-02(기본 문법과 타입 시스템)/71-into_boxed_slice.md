# into_boxed_slice
`into_boxed_slice` 는 Rust에서 Vec<T>를 힙에 고정된 배열(Box<[T]>)로 변환할 때 사용하는 메서드입니다.  
아래에 자세히 설명.

## 📦 Vec<T> → Box<[T]> 변환
```rust
let vec = vec![1, 2, 3];
let boxed: Box<[i32]> = vec.into_boxed_slice();
```
- `Vec<T>` 는 가변적이고 동적으로 크기 조절 가능한 컨테이너
- `Box<[T]>` 는 고정된 크기의 힙 배열로, 더 이상 크기 변경이 불가능함
- `into_boxed_slice()` 는 Vec을 `소유권을 넘기면서` `Box<[T]>` 로 변환

## 🧠 왜 쓰는가?
- 불변 배열로 고정하고 싶을 때: 더 이상 크기를 바꾸지 않을 경우
- 메모리 최적화: Vec은 capacity를 여유 있게 잡지만 `Box<[T]>` 는 정확한 크기만 사용
- API 요구: 어떤 함수가 Box<[T]>를 요구할 때

## 🔍 특징 요약

| 항목               | Vec<T>                                      | Box<[T]>                                      |
|--------------------|---------------------------------------------|-----------------------------------------------|
| 크기 변경 가능     | 가능 (push, pop 등)                         | 불가능 (고정된 크기)                          |
| 메모리 위치        | 힙 (heap)                                   | 힙 (heap)                                     |
| 소유권 이동        | 그대로 이동 가능                            | `into_vec()`으로 다시 Vec으로 변환 가능       |
| 주요 용도          | 동적 배열, 반복적 추가/삭제                 | 고정 배열, API 전달, 메모리 최적화            |
| 변환 방법          | `vec.into_boxed_slice()`                    | `boxed.into_vec()`                            |
| 사용 예            | `let vec = vec![1, 2, 3];`                  | `let boxed = vec.into_boxed_slice();`         |


## 🧪 실전 예시
```rust
fn take_boxed(data: Box<[u8]>) {
    println!("Length: {}", data.len());
}

fn main() {
    let vec = vec![10, 20, 30];
    let boxed = vec.into_boxed_slice();
    take_boxed(boxed);
}
```
- take_boxed()는 Box<[u8]>를 요구하므로, Vec<u8>을 into_boxed_slice()로 변환해야 합니다.


## 🧩 관련 메서드 비교

| 메서드               | 변환 흐름                         |
|----------------------|-----------------------------------|
| `into_boxed_slice()`   | Vec<T> → Box<[T]>                |
| `into_vec()`           | Box<[T]> → Vec<T>                |
| `Box::new([1,2,3])`    | [T; N] → Box<[T]> (고정 배열 박싱) |

## 📌 설명 요약
- `into_boxed_slice()`
    - Vec<T>를 힙에 고정된 배열(Box<[T]>)로 변환
    - 크기 변경 불가, 메모리 최적화
- `into_vec()`
    - Box<[T]>를 다시 Vec<T>로 변환
    - 크기 변경 가능, 유연한 조작 가능
- `Box::new([1,2,3])`
    - 고정된 배열 [T; N]을 직접 박싱
    - `Box<[T; N]>` 타입이며, `Box<[T]>` 와는 다름

---
