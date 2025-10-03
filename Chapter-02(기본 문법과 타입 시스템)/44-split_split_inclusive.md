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
