# map
Rust의 map 메서드는 정말 강력한 도구. 
이터레이터, Option, Result 등 다양한 타입에서 사용되며, 데이터 변환의 핵심 역할.

## 🧠 기본 개념: map 메서드란?
- 정의: map은 이터레이터나 래퍼 타입(Option, Result)에 대해 각 요소에 함수를 적용하여 새로운 값을 생성하는 메서드입니다.
- 특징:
- 지연 평가: 이터레이터에서는 실제로 .next()를 호출할 때까지 실행되지 않음
- 소유권 이동: Option, Result에서는 원래 값을 소비함

### 🧵 1. 이터레이터에서의 map
#### ✅ 기본 사용
```rust
let a = [1, 2, 3];
let mut iter = a.iter().map(|x| 2 * x);
```

- iter()는 참조를 반환 → x는 &i32
- into_iter()는 값 자체를 반환 → x는 i32
#### ✅ 조건 분기 + 문자열 생성
```rust
let ret = (1..=max).into_iter()
    .map(|i| match (i % 3, i % 5) {
        (0, 0) => format!("{i} - case1\n"),
        (0, _) => format!("{i} - case2\n"),
        (_, 0) => format!("{i} - case3\n"),
        _      => format!("{i} - case4\n"),
    })
    .collect::<Vec<String>>()
    .join("");
```


- map으로 각 숫자에 대해 문자열 생성
- collect::<Vec<String>>()으로 벡터로 변환
- join("")으로 하나의 문자열로 합침

#### 샘플 예제

```rust
fn map_test1(max: i32) {
    let ret  = (1..=max).into_iter()
        .map(|i| match (i %3, i%5) {
            (0, 0) => format!("{} - case1\n", i),
            (0, _) => format!("{} - case2\n", i),
            (_, 0) => format!("{} - case3\n", i),
            (_, _) => format!("{} - case4\n", i),
        }).collect::<Vec<String>>().join("");
    println!("{}", ret);
}

// Map은 최종적으로 생성하는 값이 문자열 벡터이기 때문에  collecet에게 반환값을 알려 준 것이다.

fn main() {
    map_test1(10);
}
/*
결과물
1 - case4
2 - case4
3 - case2
4 - case4
5 - case3
6 - case2
7 - case4
8 - case4
9 - case2
10 - case3
*/


fn main() {
    let a = [1, 2, 3];
    let mut iter = a.iter().map(|x| 2 * x);

    assert_eq!(
        iter.next(), Some(2)
    );
    assert_eq!(
        iter.next(), Some(4)
    );
    assert_eq!(
        iter.next(), Some(6)
    );
    assert_eq!(
        iter.next(), None
    );
}
```

### 🔄 2. iter() vs into_iter() 차이
```rust
let nums = vec![1, 2, 3];
nums.iter().map(|i| format!("{}", i));      // i: &i32
nums.into_iter().map(|i| format!("{}", i)); // i: i32
```

- iter()는 참조를 반환 → 원본을 유지
- into_iter()는 값을 소비 → 원본 사용 불가

#### 샘플 예제
```rust
fn ownership(nums : Vec<i32>){
    let ret = nums.iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join(", ");
    println!("{}", ret);

    let ret = nums.into_iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join(", ");
    println!("{}", ret);
}


fn main() {
    let array = vec![1, 2, 3, 4, 5, 6];
    ownership(array);
}

```

### 🎯 3. Option에서의 map
let som_number = Some(5.0);
let double = som_number.map(|x| x * 2.0); // Some(10.0)


- map은 Some(x) → Some(f(x))으로 변환
- None은 그대로 유지
#### 샘플예제
```rust
fn main() {
    let som_number = Some(5.0);
    let none_number: Option<f64> = None;
    let double_some = som_number.map(|x| x * 2.0);
    let double_none = none_number.map(|x| x * 2.0);
    println!("Double Some: {:?}", double_some);
    println!("Double None: {:?}", double_none);
}
```

⚠️ 소유권 주의
```rust
let maybe_some_string = Some(String::from("Hello World!"));
let maybe_some_len = maybe_some_string.map(|s| s.len());
// println!("{:?}", maybe_some_string); // ❌ 컴파일 에러
```

- String은 소유권을 가진 타입
- map은 내부 값을 소비하므로 이후 원본 사용 불가

## 📚 확장: Result, filter_map, map_or_else
- Result<T, E>도 map 사용 가능 → Ok(x)만 변환
- filter_map: Option<T>을 반환하는 함수로 None은 제거
- map_or_else: None일 때 대체값을 계산하는 클로저 사용
자세한 예제는 Hackintosh Rao의 Rust map/filter_map 설명에서 확인할 수 있어요.

## 📦 요약표: map 사용법
| 타입        | 사용 예시                        | 특징 또는 반환 구조       |
|-------------|-----------------------------------|----------------------------|
| `Iterator`  | `.map(|x| x * 2)`                 | `collect()`으로 결과 수집  |
| `Option`    | `.map(|x| x + 1)`                 | `Some`만 변환, `None` 유지 |
| `Result`    | `.map(|x| x.to_string())`         | `Ok`만 변환, `Err` 유지    |
| `filter_map`| `.filter_map(|x| x.ok())`         | `None` 제거, `Some`만 유지 |


--- 

Rust의 map, fold, filter, flat_map은 이터레이터를 다룰 때 가장 자주 쓰이는 핵심 메서드. 
Option 타입에서 유용한 map_or, map_or_else까지 포함해서 차이점과 실전 예제를 체계적으로 정리.

## 🧠 핵심 차이점 요약
| 메서드         | 기능 요약                                 |
|----------------|--------------------------------------------|
| `map`          | 각 요소를 변환. 길이는 그대로 유지         |
| `filter`       | 조건을 만족하는 요소만 유지. 길이 줄어듦   |
| `fold`         | 누적 계산. 하나의 값으로 축약              |
| `flat_map`     | 중첩된 이터레이터를 평탄화                 |
| `map_or`      


## 🔍 실전 예제 비교
### 1️⃣ map – 값 변환
```rust
let nums = vec![1, 2, 3];
let doubled: Vec<_> = nums.iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

### 2️⃣ filter – 조건 필터링
```rust
let even: Vec<_> = nums.iter().filter(|&&x| x % 2 == 0).collect();
// [2]
```

### 3️⃣ fold – 누적 계산
```rust
let sum: i32 = nums.iter().fold(0, |acc, &x| acc + x);
// 6
```


### 4️⃣ flat_map – 중첩 제거
```rust
let words = vec!["hello world", "rust lang"];
let chars: Vec<_> = words.iter()
    .flat_map(|s| s.split_whitespace())
    .collect();
// ["hello", "world", "rust", "lang"]
```



### 🎯 Option에서의 map_or, map_or_else
#### ✅ map_or
```rust
let maybe_num = Some(5);
let result = maybe_num.map_or(0, |x| x * 2);
// 10

let none_num: Option<i32> = None;
let result = none_num.map_or(0, |x| x * 2);
// 0
```

- None일 경우 기본값 0 반환
#### ✅ map_or_else
```rust
let maybe_text = Some("hello");
let result = maybe_text.map_or_else(|| "default".to_string(), |s| s.to_uppercase());
// "HELLO"

let none_text: Option<&str> = None;
let result = none_text.map_or_else(|| "default".to_string(), |s| s.to_uppercase());
// "default"
```


- None일 경우 클로저로 기본값 계산 → 비용이 큰 계산에 적합

#### 🧪 실전 예제: map + filter + fold 조합
```rust
let nums = vec![1, 2, 3, 4, 5];
let result: i32 = nums.iter()
    .map(|x| x * 2)        // [2, 4, 6, 8, 10]
    .filter(|x| x > &5)    // [6, 8, 10]
    .fold(0, |acc, x| acc + x); // 24
```

- map으로 변환 → filter로 조건 적용 → fold로 합산

## 📦 요약표

| 메서드         | 적용 대상                | 설명 / 특징                     | 반환 구조 예시         | 사용 목적 예시            |
|----------------|--------------------------|----------------------------------|-------------------------|---------------------------|
| `map`          | `Iterator`, `Option`, `Result` | 각 요소를 변환                   | `[T]`, `Some(T)`, `Ok(T)` | 값 변환                   |
| `filter`       | `Iterator`               | 조건을 만족하는 요소만 유지     | `[T]`                   | 조건 필터링               |
| `fold`         | `Iterator`               | 누적 계산                        | `T`                     | 합산, 누적, 축약          |
| `flat_map`     | `Iterator`               | 중첩된 이터레이터를 평탄화      | `[T]`                   | 다차원 → 단일 구조 변환   |
| `map_or`       | `Option`                 | `None`일 경우 기본값 반환        | `T`                     | 기본값 + 변환             |
| `map_or_else`  | `Option`                 | `None`일 경우 클로저로 기본값 계산 | `T`                     | 지연 계산 + 변환          |

---




