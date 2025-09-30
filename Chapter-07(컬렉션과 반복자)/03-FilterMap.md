# filter / filter_map
Rust에서 자주 사용되는 이터레이터 메서드인 filter와 filter_map의 차이점과 사용 방식에 대한 설명입니다.

## 🧠 filter vs filter_map 비교 요약
| 메서드        | 조건 검사 방식             | 반환 타입             | 사용 목적                          | 예시 상황                          |
|---------------|------------------------------|-----------------------|------------------------------------|-------------|
|`filter`   |`bool` 반환 조건식 사용      |`Iterator<Item=T>`|요소를 걸러내기만 할 때        |`filter(|x| x > 10)`           |
|`filter_map`|`Option<T>` 반환 조건 + 변환|`Iterator<Item=U>`|조건 검사와 동시에 값 추출/변환|`filter_map(|x| Some(x.len()))`|

## 예제 코드
```rust
struct Person {
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: i32,
}

fn main() {
    let mut persons: Vec<Person> = Vec::new();
    persons.push(Person {
        first_name: "Asnim".to_string(),
        last_name: None,
        age: 1,
    });
    persons.push(Person {
        first_name: "Fahim".to_string(),
        last_name: Some("Ansari".to_string()),
        age: 2,
    });
    persons.push(Person {
        first_name: "Shahul".to_string(),
        last_name: None,
        age: 6,
    });
    persons.push(Person {
        first_name: "Mujeeb".to_string(),
        last_name: Some("Rahuman".to_string()),
        age: 6,
    });
    let ages_of_people_with_second_name_using_seperate_filter_map: Vec<i32> = persons
        .iter()
        .filter(|p| p.last_name.is_some())
        .map(|p| p.age)
        .collect();
    println!("{:?}", ages_of_people_with_second_name)
}


let ages_of_people_with_second_name_using_seperate_filter_map: Vec<i32> = persons
        .iter()
        .filter_map(|p| p.last_name.map(|_| p.age))
        .collect();

let ages_of_people_with_second_name_using_seperate_filter_map: Vec<i32> = persons
.iter()
.filter_map(|p| match p.last_name { 
    Some(_) => Some(p.age),
    None => None
})
.collect();        

```


## 🔍 예제 분석
### ✅ filter + map 조합
```rust
let ages: Vec<i32> = persons
    .iter()
    .filter(|p| p.last_name.is_some()) // last_name이 Some인 사람만 필터링
    .map(|p| p.age)                    // 나이만 추출
    .collect();
```

- 2단계 처리: 먼저 조건 필터링 → 그 다음 값 추출
- filter는 bool을 반환하는 조건식이 필요

### ✅ filter_map 단일 처리
```rust
let ages: Vec<i32> = persons
    .iter()
    .filter_map(|p| p.last_name.map(|_| p.age))
    .collect();
```

또는
```rust
let ages: Vec<i32> = persons
    .iter()
    .filter_map(|p| match p.last_name {
        Some(_) => Some(p.age),
        None => None,
    })
    .collect();
```

- 1단계 처리: 조건 검사와 값 추출을 동시에 수행
- filter_map은 Option<T>을 반환 → Some(val)만 남김

## ✅ 어떤 걸 써야 할까?
| 상황 또는 목적                     | 적합한 메서드     | 설명                                      |
|----------------------------------|-------------------|-------------------------------------------|
| 조건만 걸러내고 원본 그대로 사용     | `filter()`         | `bool` 조건으로 요소를 선택               |
| 조건에 따라 값을 추출하거나 변환     | `filter_map()`     | `Option<T>`을 반환하여 값 추출과 필터링 동시 수행 |



## 💡 실전 팁
- filter_map은 Option<T>을 다룰 때 매우 유용합니다.
- Result<T, E>를 다룰 때는 filter_map(Result::ok)처럼 사용할 수 있습니다.
- filter_map은 성능상 filter + map보다 약간 더 효율적일 수 있습니다 (중간 이터레이터 생략).

---

#  group_by, partition, fold
Rust의 고급 이터레이터 메서드인 group_by, partition, fold까지 함께 정리.

## 🧠 고급 이터레이터 메서드 요약
| 메서드       | 설명                                      | 반환 타입 또는 구조             | 사용 예시 또는 목적                     |
|--------------|-------------------------------------------|----------------------------------|----------------------------------------|
| `filter`     | 조건을 만족하는 요소만 남김               | `Iterator<Item=T>`              | 특정 조건으로 요소 걸러내기             |
| `filter_map` | 조건 검사 + 값 추출/변환                  | `Iterator<Item=U>`              | `Option<T>` 기반 값 추출               |
| `group_by`   | 인접한 요소를 그룹화                      | `GroupBy<K, V>` (external crate) | 연속된 값 묶기 (e.g. 날짜별 로그)       |
| `partition`  | 조건에 따라 두 그룹으로 분할              | `(Vec<T>, Vec<T>)`              | 성공/실패, 참/거짓 그룹 나누기         |
| `fold`       | 누적 계산 수행                            | `T` (초기값과 동일한 타입)      | 합계, 누적, 집계 등                     |



## 🔍 각 메서드 상세 설명 & 예제
### ✅ group_by (from itertools crate)
```rust
use itertools::Itertools;

let data = vec![1, 1, 2, 2, 2, 3];
for (key, group) in &data.into_iter().group_by(|x| *x) {
    println!("{}: {:?}", key, group.collect::<Vec<_>>());
}
```

- 인접한 동일 값을 묶음
- group_by는 itertools 크레이트에서 제공됨
- 비슷한 날짜, 상태, 키 값으로 묶을 때 유용

### ✅ partition
```rust
let nums = vec![1, 2, 3, 4, 5, 6];
let (even, odd): (Vec<_>, Vec<_>) = nums.into_iter().partition(|x| x % 2 == 0);
println!("Even: {:?}, Odd: {:?}", even, odd);
```


- 조건에 따라 두 그룹으로 나눔
- 반환값은 (Vec<T>, Vec<T>)
- 성공/실패, 유효/무효 등 이진 분류에 적합

### ✅ fold
```rust
let nums = vec![1, 2, 3, 4];
let sum = nums.iter().fold(0, |acc, x| acc + x);
println!("Sum: {}", sum);
```


- 초기값부터 시작해 누적 계산
- fold(init, |acc, item| ...)
- 합계, 평균, 누적 문자열 등 다양한 집계에 사용

## 🧠 언제 어떤 걸 써야 할까?
| 상황 또는 목적                     | 적합한 메서드     | 설명                                      |
|----------------------------------|-------------------|-------------------------------------------|
| 조건만 걸러내고 원본 그대로 사용     | `filter`          | `bool` 조건으로 요소를 선택               |
| 조건에 따라 값을 추출하거나 변환     | `filter_map`      | `Option<T>` 기반으로 값 추출과 필터링 동시 수행 |
| 인접한 값들을 그룹화하고 싶을 때     | `group_by`        | 연속된 값들을 키 기준으로 묶음 (`itertools` 필요) |
| 조건에 따라 두 그룹으로 나누고 싶을 때 | `partition`       | `true`/`false` 조건으로 두 벡터로 분할     |
| 누적 계산 또는 집계를 수행할 때      | `fold`            | 초기값부터 누적하며 계산 수행             |

---

## 🧠 핵심 개념: filter_map과 filter (chain 사용)
### 🔹 filter_map
- Option<T>을 반환하는 클로저를 받아서
- Some(value)만 남기고 None은 제거
- 동시에 Some(value)의 값을 꺼내서 반환
```rust
let result: Vec<i32> = vec![Some(1), None, Some(3)]
    .into_iter()
    .filter_map(|x| x)
    .collect();
// result = [1, 3]
```

### 🔹 filter
- bool을 반환하는 클로저를 받아서
- true인 값만 남기고 false는 제거
```rust
let result: Vec<i32> = vec![1, 2, 3, 4]
    .into_iter()
    .filter(|x| x % 2 == 0)
    .collect();
// result = [2, 4]
```


## 🔗 함께 쓰는 이유
```rust
for header in (0..header_count)
    .filter_map(|i| loader.get_header_by_index(i)) // Option<String> → String
    .filter(|h| !h.is_empty())                     // 빈 문자열 제거
{
    // header는 유효한 String
}
```

## 🔄 동작 순서
- 0..header_count 범위를 반복
- 각 i에 대해 loader.get_header_by_index(i) 호출 → Option<String>
- filter_map이 Some(h)만 남기고 None은 제거
- filter가 h.is_empty()가 아닌 것만 남김
- 최종적으로 header는 빈 문자열이 아닌 유효한 값들만 반복됨

###  🧪 예시로 이해하기
```rust
let headers = vec![Some("Name"), None, Some(""), Some("Age")];

for header in headers
    .into_iter()
    .filter_map(|h| h.map(|s| s.to_string())) // Option<&str> → Option<String>
    .filter(|h| !h.is_empty())
{
    println!("Header: {}", header);
}
```

### 출력 결과:
```
Header: Name
Header: Age
```


## 💡 팁: 가독성 높이기
긴 체이닝은 가독성이 떨어질 수 있으니, 중간에 변수로 분리해도 좋음:
```rust
let valid_headers = (0..header_count)
    .filter_map(|i| loader.get_header_by_index(i))
    .filter(|h| !h.is_empty());

for header in valid_headers {
    // ...
}
```
## 실전 예제
```rust
    let header_count = loader.header_count();
    for header in (0..header_count)
        .filter_map(|i| loader.get_header_by_index(i))
        .filter(|h| !h.is_empty())
    {
        if let Some(raw) = loader.get_column(header) {
            let src = TArray::from(raw.clone());
            let mut tgt = TArray::from(vec![]);
            exec_sae_filter(&src, &mut tgt, 0.0001, 300.0);
            container.insert(header.clone(), tgt);
        }
    }
```



