# split / split_inclusive
split_inclusive()는 Rust의 문자열이나 슬라이스를 특정 조건 기준으로 나누되, 그 조건을 포함해서 분할하는 함수입니다.  
보통 split()은 조건을 기준으로 나누지만, split_inclusive()는 조건이 걸린 요소를 각 조각의 끝에 포함.

## 🔍 핵심 개념
| 함수               | 조건 포함 여부 | 입력       | 결과                          |
|--------------------|----------------|------------|-------------------------------|
| split()            | ❌ 포함 안 함   | "a,b,c"    | ["a", "b", "c"]               |
| split_inclusive()  | ✅ 포함함      | "a,b,c"    | ["a,", "b,", "c"]             |


## 🔧 예시: 문자열에서 쉼표 기준 분할
```rust
let s = "a,b,c";
let parts: Vec<&str> = s.split_inclusive(',').collect();
println!("{:?}", parts); // ["a,", "b,", "c"]
```

- 각 조각의 끝에 ,가 포함됨
- 로그 처리, CSV 파싱, 구분자 포함 텍스트 처리에 유용

## 🔧 예시: 슬라이스에서 조건 포함 분할
```rust
let data = [1, 2, 3, 0, 4, 5, 0, 6];
let chunks: Vec<&[i32]> = data.split_inclusive(|&x| x == 0).collect();
println!("{:?}", chunks);
// [[1, 2, 3, 0], [4, 5, 0], [6]]
```

- 0을 기준으로 나누되, 0을 각 조각의 끝에 포함
- 센서 데이터, 로그 이벤트, HIC 구간 추적 등에 활용 가능

## 💡 언제 쓰면 좋은가?
| 상황               | 이유                                                   |
|--------------------|--------------------------------------------------------|
| 로그 이벤트 분할    | 이벤트 종료 지점을 포함해서 정확한 구간 추적 가능       |
| CSV/구분자 포함 텍스트 | 구분자까지 포함해 각 필드를 완전하게 분리 가능         |
| 센서 구간 추적      | 특정 값 기준으로 나누되, 기준값을 각 구간 끝에 포함 가능 |
| HIC 계산 구간       | 시간 경계 포함해서 슬라이스 나누기 → 정확한 구간 분석   |
| 패턴 기반 분할      | 특정 조건이 발생한 지점까지 포함해 의미 있는 조각 생성   |


---

# 문자열이 아닌 곳에 응용 

Rust에서 split()과 split_inclusive()를 사용하는 다양한 예시를  
문자열과 슬라이스에 대해 각각 적용.

## 🧵 문자열에서 split()
```rust
let s = "a,b,c";
let parts: Vec<&str> = s.split(',').collect();
println!("{:?}", parts); // ["a", "b", "c"]
```

- 쉼표를 기준으로 나누되, 쉼표는 포함되지 않음

## 🧵 문자열에서 split_inclusive()
```rust
let s = "a,b,c";
let parts: Vec<&str> = s.split_inclusive(',').collect();
println!("{:?}", parts); // ["a,", "b,", "c"]
```

- 쉼표를 기준으로 나누되, 쉼표는 각 조각의 끝에 포함됨

## 🔢 슬라이스에서 split()
```rust
let data = [1, 2, 3, 0, 4, 5, 0, 6];
let chunks: Vec<&[i32]> = data.split(|&x| x == 0).collect();
println!("{:?}", chunks);
// [[1, 2, 3], [4, 5], [6]]
```

- 0을 기준으로 나누되, 0은 제외됨

## 🔢 슬라이스에서 split_inclusive()
```rust
let data = [1, 2, 3, 0, 4, 5, 0, 6];
let chunks: Vec<&[i32]> = data.split_inclusive(|&x| x == 0).collect();
println!("{:?}", chunks);
// [[1, 2, 3, 0], [4, 5, 0], [6]]
```

- 0을 기준으로 나누되, 0은 각 조각의 끝에 포함됨

---


## 🔧 Rust에서 문자열 토큰 분할은 이렇게 간단합니다

### cpp  코드
```cpp
bool tokenParam(std::string strParam, const std::string& strTok, std::vector<std::string>& aParam)
{
    aParam.clear();
    std::string str = std::move(strParam);
    regex reg(strTok);
    sregex_token_iterator iter(str.begin(), str.end(), reg, -1);
    sregex_token_iterator end;
    vector<string> vec(iter, end);
    for (const auto& a : vec)
    {
        if(!a.empty()){
            aParam.push_back(a);
        }

    }
   return true;
}
```

### ✅ 기본 split()
```rust
let s = "a,b,c";
let tokens: Vec<&str> = s.split(',').collect();
println!("{:?}", tokens); // ["a", "b", "c"]
```


### ✅ 정규식 기반 분할 (regex crate 사용)
```rust
use regex::Regex;

let s = "a,b;c|d";
let re = Regex::new(r"[;,|]").unwrap();
let tokens: Vec<&str> = re.split(s).filter(|t| !t.is_empty()).collect();
println!("{:?}", tokens); // ["a", "b", "c", "d"]
```

- filter(|t| !t.is_empty())는 C++의 if(!a.empty())에 해당
- Regex::split()은 sregex_token_iterator보다 훨씬 직관적

## 💡 요약 비교
| 기능               | C++ 방식                  | Rust 방식                          |
|--------------------|---------------------------|------------------------------------|
| 정규식 기반 분할    | sregex_token_iterator      | Regex::split()                     |
| 빈 토큰 제거        | if(!a.empty())             | .filter(|t| !t.is_empty())         |
| 결과 벡터에 저장    | vector<string> + push_back | .collect::<Vec<_>>()               |
| 전체 코드 길이      | 10줄 이상                  | 2~3줄                              |

---

# 🧩 split_first()란?

split_first()는 slice에서 첫 번째 요소와 나머지를 분리하는 메서드입니다.  
```rust
fn split_first(&self) -> Option<(&T, &[T])>
```
- &self: 대상 slice (&[T])
- 반환값: Option<(첫 요소 참조, 나머지 slice)>
- 빈 slice면 None, 그 외엔 Some((first, rest))

## ✅ 예시
```rust
let arr = [10, 20, 30];
if let Some((first, rest)) = arr.split_first() {
    println!("First: {}", first);       // 10
    println!("Rest: {:?}", rest);       // [20, 30]
}
```

- first: &10
- rest: &[20, 30]

### ❗ 주의할 점
- slice를 소비하지 않음 → 원본은 그대로 유지됨
- 참조를 반환 → first는 &T, rest는 &[T]
- 빈 slice면 None → 반드시 Option 처리 필요

---

# 기타 함수 정리

split_first()는 iterator 스타일의 재귀적 분해를 염두에 둔 함수.  
Rust의 slice 처리 철학이 잘 드러나는 함수들이라서,  
같이 쓰이는 split_last(), split_at(), chunks()도 정리.  

## 🧩 split_first()
```rust
fn split_first(&self) -> Option<(&T, &[T])>
```

- 앞에서부터 하나 꺼내고 나머지 반환
- 빈 slice면 None
- 재귀적 처리, 패턴 매칭에 유용
```rust
let arr = [1, 2, 3];
if let Some((head, tail)) = arr.split_first() {
    println!("{head}, {:?}", tail); // 1, [2, 3]
}
```


## 🧩 split_last()
```rust
fn split_last(&self) -> Option<(&T, &[T])>
```

- 뒤에서부터 하나 꺼내고 나머지 반환
- split_first()의 반대 방향
- 스택처럼 처리할 때 유용
- 
```rust
let arr = [1, 2, 3];
if let Some((last, rest)) = arr.split_last() {
    println!("{last}, {:?}", rest); // 3, [1, 2]
}
```


## 🧩 split_at()
```rust
fn split_at(&self, mid: usize) -> (&[T], &[T])
```

- 지정한 인덱스 기준으로 앞/뒤로 나눔
- panic 발생 가능 → 인덱스 범위 주의
- 
```rust
let arr = [1, 2, 3, 4];
let (left, right) = arr.split_at(2);
println!("{:?}, {:?}", left, right); // [1, 2], [3, 4]
```


## 🧩 chunks()
```rust
fn chunks(&self, size: usize) -> Chunks<'_, T>
```

- slice를 고정 크기 블록으로 나눔
- 마지막 chunk는 size보다 작을 수 있음
```rust
let arr = [1, 2, 3, 4, 5];
for chunk in arr.chunks(2) {
    println!("{:?}", chunk); // [1, 2], [3, 4], [5]
}
```

---

# Borrow에 응용

Rust에서 mut를 쓰는 건 단순히 값을 바꾸기 위한 게 아니라,  
소유권과 빌림(borrowing) 문제를 해결하는 핵심 도구로 쓰이는 경우가 많음.  
특히 slice나 iterator를 다룰 때, split_first() 같은 함수와 함께 쓰면  
재귀적 처리나 상태 변경이 가능.  

## 🔍 mut가 소유권 문제를 해결하는 방식
### 1. 가변 참조로 빌림을 허용
``rust
fn consume(slice: &mut &[i32]) {
    if let Some((first, rest)) = slice.split_first() {
        println!("Consuming: {}", first);
        *slice = rest; // slice를 앞으로 이동
    }
}
```

- &mut &[i32]는 slice 자체를 가변 참조로 빌림
- *slice = rest로 slice의 내부를 바꿀 수 있음  
    → 이게 바로 iterator처럼 slice를 소비하는 방식

### 2. 재귀적 처리에 유리
```rust
fn walk(slice: &mut &[i32]) {
    while let Some((first, rest)) = slice.split_first() {
        println!("Next: {}", first);
        *slice = rest;
    }
}
```

- slice를 점점 줄여가면서 순회  
  → mut 없으면 *slice = rest가 불가능  
  → 결국 소유권 문제로 막히게 됨  

### 3. Iterator와 유사한 흐름
- split_first()는 iterator처럼 앞에서 하나씩 꺼내는 구조
- mut를 쓰면 slice를 줄여가면서 상태를 유지할 수 있음  
  → 소유권을 넘기지 않고도 반복 처리 가능

---


