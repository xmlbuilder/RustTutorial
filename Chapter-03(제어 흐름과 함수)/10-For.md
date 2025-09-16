# for
for는 Rust에서 가장 안전하고 직관적인 반복 방식으로, **컬렉션이나 범위(iterator)**를 순회할 때 사용됩니다.

## 🔁 기본 구조
for 변수 in 반복가능한값 {
    // 실행할 코드
}


- 반복가능한값은 배열, 벡터, 범위(0..5) 등 Iterator를 구현한 타입이면 모두 가능해요.

## 📦 배열 순회
```rust
fn main() {
    let xs = [1, 2, 3, 4, 5];
    for x in xs {
        println!("x = {}", x);
    }
}
```
## 출력 결과
```
x = 1
x = 2
x = 3
x = 4
x = 5
```

- 배열의 각 요소를 자동으로 순회합니다.
- 인덱스를 직접 관리할 필요가 없어요.

## 🔢 범위 순회
```rust
fn main() {
    for i in 0..5 {
        println!("i = {i}");
    }
    println!("Finished");
}
```
## 출력 결과
```
i=0
i=1
i=2
i=3
i=4
Finished
```

- 0..5는 0부터 4까지 (5는 포함되지 않음).
- 0..=4 또는 1..=100처럼 =을 붙이면 끝값도 포함됩니다.

## 📏 배열 길이 기반 순회
``` rust
fn main() {
    let xs = [1, 2, 3, 4, 5];
    let l = xs.len();
    for i in 0..l {
        println!("i = {i}");
    }
    println!("Finished");
}
```

### 출력 결과
```
i = 0
i = 1
i = 2
i = 3
i = 4
Finished
```

- 배열의 인덱스를 직접 순회할 때 유용합니다.

## 🔄 역순 순회
```rust
fn main() {
    let xs = [1, 2, 3, 4, 5];
    let l = xs.len();
    for i in (0..l).rev() {
        println!("i = {i}");
    }
    println!("Finished");
}
```

### 출력 결과
```
i = 4
i = 3
i = 2
i = 1
i = 0
Finished
```

- .rev()를 붙이면 역순으로 반복합니다.

## 🧠 .iter() 사용
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

- .iter()는 배열을 참조 형태로 순회합니다.
- 값이 아닌 참조(&element)로 접근하므로 원본을 수정하지 않아요.

## 🧩 for 반복문 요약
| 기능 또는 패턴         | 설명                                                                 |
|------------------------|----------------------------------------------------------------------|
| 배열 순회              | `for x in array`                                                     |
| 범위 순회              | `for i in 0..n`, `for i in 1..=100`                                   |
| 길이 기반 인덱스 순회  | `for i in 0..array.len()`                                             |
| 역순 순회              | `for i in (0..n).rev()`                                               |
| 참조 순회              | `for x in array.iter()`                                               |
| 가장 안전한 반복 방식  | ✅ 인덱스 오류 없이 자동으로 순회                                     |

----


## 🔢 .. vs ..= 차이점
for i in 1..5 {
    println!("{i}");
}

- 1..5는 1부터 4까지 (5는 포함되지 않음)
for i in 1..=5 {
    println!("{i}");
}

- 1..=5는 1부터 5까지 (5도 포함됨)


```rust
//1부터 100까지
for n in 1..101 {
//1부터 100까지
for n in 1..=100 {
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

## 🧠 요약 표 (Markdown)
| 범위 표현 | 포함 여부         | 예시 출력       |
|-----------|-------------------|-----------------|
| `a..b`    | `a` 이상 `b` 미만 | `a, a+1, ..., b-1` |
| `a..=b`   | `a` 이상 `b` 이하 | `a, a+1, ..., b`   |



## ✅ 언제 ..=를 써야 할까?
- 마지막 값까지 포함하고 싶을 때
- 예: 1..=100 → 1부터 100까지 모두 출력
- 예: for i in 0..=arr.len() - 1 → 배열의 마지막 인덱스까지 접근

---

