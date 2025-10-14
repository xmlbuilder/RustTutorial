# const 배열  / 함수
Rust에서 const 배열을 만드는 건 아주 간단하지만 몇 가지 규칙이 있음.  
아래에 기본 문법과 함께 다양한 예제를 정리.

## ✅ 기본 문법
```rust
const NUMBERS: [i32; 3] = [1, 2, 3];
```
- const로 선언된 배열은 컴파일 타임에 값이 결정되어야 함
- [i32; 3]은 i32 타입의 요소 3개를 가진 배열

## ✅ 다양한 예제
### 🔹 문자열 배열
```rust
const COLORS: [&str; 3] = ["red", "green", "blue"];
```
- &str은 고정된 문자열 리터럴
- String은 heap에 저장되므로 const에서는 사용 불가

### 🔹 튜플 배열
```rust
const POINTS: [(i32, i32); 2] = [(0, 0), (10, 20)];
```

- 각 요소가 (i32, i32) 형태의 튜플
### 🔹 bool 배열
const FLAGS: [bool; 4] = [true, false, true, false];

| 항목    | 설명                                |
|---------|-------------------------------------|
| String  | ❌ 사용 불가 → 대신 `&str` 사용     |
| Vec     | ❌ 사용 불가 → 대신 고정 크기 배열  |
| const   | ✅ 컴파일 타임에 값이 결정되어야 함 |

---

# const fn

Rust에서는 const fn을 사용하면 컴파일 타임에 실행 가능한 함수를 정의할 수 있음.  
이걸 활용하면 const 배열이나 구조체 초기화에 동적 계산된 값을 넣을 수 있음.

## ✅ 기본 문법
```rust
const fn double(x: i32) -> i32 {
    x * 2
}
const VALUE: i32 = double(5); // 컴파일 타임에 계산됨
```
- double(5)은 런타임이 아니라 컴파일 타임에 계산됨
- VALUE는 10으로 고정됨

## ✅ const fn로 배열 만들기
```rust
const fn make_array() -> [i32; 3] {
    [1, 2, 3]
}

const NUMBERS: [i32; 3] = make_array();
```

- make_array()는 const fn이기 때문에 NUMBERS도 const로 선언 가능

## ✅ 반복 계산도 가능 (단, 제한 있음)
```rust
const fn square_array() -> [i32; 4] {
    [1 * 1, 2 * 2, 3 * 3, 4 * 4]
}

const SQUARES: [i32; 4] = square_array(); // [1, 4, 9, 16]
```
- 루프는 안 되지만 수식 기반 계산은 가능

## ⚠️ 제한 사항 (const fn에서 사용 불가 또는 제한되는 요소)
| 항목        | 설명                                      |
|-------------|-------------------------------------------|
| 반복문      | ❌ `for`, `while` 사용 불가               |
| 힙 자료구조 | ❌ `Vec`, `Box`, `String` 사용 불가       |
| 조건문      | ⚠️ `if`는 가능하지만 일부 제약 있음       |

## 🔍 보충 설명
- for, while은 런타임 제어 흐름이므로 const fn에서는 허용되지 않음
- Vec, Box, String은 힙 할당이 필요하므로 const에서는 불가능
- if는 단순한 분기에는 사용 가능하지만, 복잡한 조건이나 동적 흐름은 제한됨

---





