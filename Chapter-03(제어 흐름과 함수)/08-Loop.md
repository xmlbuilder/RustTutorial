# Loop
Rust의 loop 문은 매우 강력하면서도 유연한 반복 구조입니다.

## 🌀 기본 loop 사용 예제
```rust
fn main() {
    let mut counter = 0;
    loop {
        println!("continue");
        counter += 1;
        if counter == 3 {
            break;
        }
    };
    println!("counter = {counter}");
}
```

## 🔍 설명
- loop는 무한 루프를 생성합니다.
- break를 사용하여 루프를 종료할 수 있습니다.
- counter가 3이 되면 루프를 빠져나오고, 마지막에 counter 값을 출력합니다.
- 출력 결과는:
```
continue
continue
continue
counter = 3
```


## 🧠 loop를 식(expression)으로 사용하기
```rust
fn main() {
    let mut counter = 0;
    let y = loop {
        println!("continue");
        counter += 1;
        if counter == 3 {
            break counter;
        }
    };
    println!("y = {y}"); // 3
}
```

### 🔍 설명
- loop는 식으로도 사용 가능하며, break에 값을 넘기면 그 값이 루프의 결과가 됩니다.
- `break counter;`는 `counter` 값을 `loop의 결과`로 반환합니다.
- 변수 y는 3이 됩니다.

## 🧭 중첩 루프와 이름 붙이기
```rust
fn main(){
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");
    'first_loop: loop {
        counter += 1;
        println!("The counter is now {}", counter);
        if counter > 9 {
            println!("Now entering the second loop.");
            'second_loop: loop {
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
}
```

## 🔍 설명
- 루프에 '이름:을 붙이면 특정 루프를 명시적으로 break할 수 있습니다.
- `break 'first_loop;`는 바깥 루프를 종료합니다.
- 이 구조는 복잡한 중첩 루프에서 제어 흐름을 명확하게 할 때 유용합니다.

🧩 요약
| 개념        | 설명                                                                 |
|-------------|----------------------------------------------------------------------|
| `loop`      | 무한 반복을 생성하는 기본 루프 구조                                  |
| `break`     | 루프를 종료시키는 키워드                                             |
| `loop` + `break` | `break`에 값을 넘겨 루프를 식(expression)으로 사용 가능             |
| `'label:`   | 중첩 루프에서 특정 루프를 명시적으로 종료할 수 있도록 이름을 붙임     |


## 🔁 Rust 반복문 비교 요약
| 반복문 종류 | 설명                                                                 | 사용 예시 |
|-------------|----------------------------------------------------------------------|-----------|
| `loop`      | 무한 반복을 생성. `break`로 종료. `break`에 값을 넘기면 식으로 사용 가능 | `loop { ... }` |
| `while`     | 조건이 참일 동안 반복. 조건이 거짓이 되면 종료                         | `while condition { ... }` |
| `for`       | 반복 가능한 값들(iterator)을 순회. 가장 안전하고 일반적인 반복 방식     | `for x in 0..5 { ... }` |



### 🌀 loop 예시
```rust
fn main() {
    let mut count = 0;
    loop {
        println!("count = {}", count);
        count += 1;
        if count == 3 {
            break;
        }
    }
}
```

- 무한 루프를 생성하고 break로 종료.
- loop는 식으로도 사용 가능:
```rust
let result = loop {
    if some_condition {
        break 42;
    }
};
```


### 🔄 while 예시
```rust
fn main() {
    let mut count = 0;
    while count < 3 {
        println!("count = {}", count);
        count += 1;
    }
}
```

- 조건이 참일 때만 반복.
- 조건이 처음부터 거짓이면 한 번도 실행되지 않음.

### 🔢 for 예시
```rust
fn main() {
    for i in 0..3 {
        println!("i = {}", i);
    }
}
```

- 0..3은 0, 1, 2를 의미하는 범위.
- for는 iterator 기반으로 안전하고 깔끔한 반복을 제공.

## 🧠 언제 어떤 반복문을 써야 할까?
| 상황 또는 목적                     | `loop` 사용 | `while` 사용 | `for` 사용 |
|----------------------------------|-------------|--------------|------------|
| 무한 반복이 필요할 때            | ✅          | ❌           | ❌         |
| 반복 중 특정 시점에 값을 반환할 때| ✅ (`break`) | ❌           | ❌         |
| 조건 기반 반복이 필요할 때       | ❌          | ✅           | ❌         |
| 반복 횟수가 정해져 있지 않을 때  | ✅          | ✅           | ❌         |
| 컬렉션이나 범위를 순회할 때      | ❌          | ❌           | ✅         |
| 가장 안전하고 직관적인 반복 방식 | ❌          | ❌           | ✅         |
| 중첩 루프에서 특정 루프 종료     | ✅ (`'label`) | ❌           | ❌         |


---


# Loop를 만든 이유
굳이 loop를 만든 이유는 표현력과 제어 흐름의 유연성 때문. 
단순히 반복만 하는 게 아니라, loop는 Rust의 철학인 안전성과 명확한 제어 흐름을 더 잘 반영할 수 있도록 설계.

## 🔍 왜 loop가 필요한가?

### 1. 표현식으로 사용 가능
Rust는 모든 것이 표현식(expression)이라는 철학을 가지고 있음.  
loop는 break에 값을 넘겨서 루프 자체를 하나의 값으로 사용할 수 있음.
```rust
let result = loop {
    if condition {
        break 42;
    }
};
println!("result = {}", result); // 42
```

➡️ while은 이런 식으로 값을 반환할 수 없음.

### 2. 무한 루프를 명확하게 표현
while true도 무한 루프지만, loop는 의도적으로 무한 루프를 만들겠다는 명확한 표현.
```rust
loop {
    // 명시적 무한 루프
}
```

➡️ 읽는 사람에게 "이건 일부러 무한 루프를 만든 거야"라는 의도를 전달할 수 있음.

### 3. 중첩 루프에서 제어 흐름 지정 가능
loop는 'label:을 붙여서 중첩 루프에서 특정 루프를 종료할 수 있음.
```rust
'outer: loop {
    loop {
        break 'outer;
    }
}
```

➡️ while이나 for에서는 이런 제어가 불가능하거나 복잡해짐.

### 4. 미세한 제어가 필요한 경우
loop는 조건 없이 시작되기 때문에, 반복 조건을 루프 내부에서 자유롭게 구성할 수 있음.  
예를 들어, match, if, return, continue, break 등을 조합해서 복잡한 흐름을 만들 수 있음.

## 🧠 요약
| 기능 또는 상황                  | `loop`                          | `while`                        | `while true`                  |
|-------------------------------|----------------------------------|--------------------------------|-------------------------------|
| 무한 루프 표현                 | ✅ 명시적 무한 루프              | ❌ 조건 기반 반복               | ✅ 조건을 `true`로 설정한 루프 |
| 반복 조건 사용                 | ❌ 조건 없음                     | ✅ 조건 필요                    | ✅ 조건이 항상 true            |
| 루프 결과 반환 (`break val`)  | ✅ 가능                          | ❌ 불가능                       | ❌ 불가능                      |
| 중첩 루프 제어 (`'label`)     | ✅ `'label:`으로 특정 루프 제어 | ❌ 불가능                       | ❌ 불가능                      |
| 의도 표현 명확성              | ✅ 무한 루프임을 명확히 표현     | ✅ 조건 기반임을 표현           | ⚠️ 무한 루프지만 의도가 흐릿함 |
| 사용 추천 상황                | 상태 머신, 제어 흐름 설계 등    | 조건 반복이 필요한 경우        | 무한 루프가 필요하지만 간단할 때 |


---


