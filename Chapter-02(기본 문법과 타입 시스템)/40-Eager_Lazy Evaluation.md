
# 🦀 Rust: 즉시 vs 지연 실행
## ✅ 즉시 실행
- vec![], HashMap, HashSet 등은 즉시 생성
- collect() 호출 시 전체 결과를 메모리에 저장
```rust
let v = vec![1, 2, 3, 4, 5];
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect(); // 즉시 실행
```

## ✅ 지연 실행
- iter(), map(), filter()는 모두 lazy
- collect()를 호출하기 전까지는 아무 것도 계산되지 않음
```rust
let lazy = (1..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 3); // lazy iterator

for val in lazy {
    println!("{}", val); // 이 시점에 계산됨
}
```

---

# 🔁 C++: 즉시 vs 지연 실행
## ✅ 즉시 실행
- std::vector, std::map, std::set 등 STL 컨테이너는 즉시 실행
- std::transform, std::copy, std::accumulate 등은 즉시 결과 생성
```cpp
std::vector<int> v = {1, 2, 3, 4, 5};
std::vector<int> doubled;
std::transform(v.begin(), v.end(), std::back_inserter(doubled), [](int x) { return x * 2; });
```

→ 모든 결과가 즉시 계산되어 doubled에 저장됨

## ✅ 지연 실행 (Lazy Evaluation)
- C++20부터 ranges 라이브러리로 지연 실행 가능
- views::transform, views::filter 등은 lazy
```cpp
#include <ranges>
auto lazy_view = std::views::iota(1, 10)
               | std::views::filter([](int x) { return x % 2 == 0; })
               | std::views::transform([](int x) { return x * 3; });
```

→ lazy_view는 실제로 반복할 때만 계산됨



## 단계별 실행 절차
아래는 C++20의 std::ranges를 활용한 lazy_view를 main 함수에서 호출하고 결과가 처리되는 방식을 단계별로 설명한 예제.

## ✅ 전체 코드 예시
```cpp
#include <iostream>
#include <ranges>

int main() {
    auto lazy_view = std::views::iota(1, 10)
                   | std::views::filter([](int x) { return x % 2 == 0; })
                   | std::views::transform([](int x) { return x * 3; });

    for (int val : lazy_view) {
        std::cout << val << " ";
    }
    std::cout << std::endl;
}
```

## 🔍 실행 흐름 설명
### 1. std::views::iota(1, 10)
- 생성: 1, 2, 3, ..., 9 → lazy range (아직 계산 안 됨)
### 2. filter: x % 2 == 0
- 조건: 짝수만 통과 → 2, 4, 6, 8
### 3. transform: x * 3
- 변환: 6, 12, 18, 24
### 4. for 루프에서 값 꺼낼 때마다
- iota → filter → transform → val 생성
- 하나씩 계산됨 (lazy evaluation)

### 🧠 결과 출력
```
6 12 18 24
```

## ✅ 핵심 요약
| 단계         | 설명                                      |
|--------------|-------------------------------------------|
| 선언         | `lazy_view`는 계산되지 않은 파이프라인       |
| 반복 시작    | `for` 루프에서 값 하나씩 꺼낼 때 계산 시작   |
| 처리 흐름    | `iota` → `filter` → `transform` → 출력       |
| 실행 시점    | 반복자가 요청될 때마다 지연 실행됨           |


## 🧠 요약 비교
| 언어   | 즉시 실행 예시                        | 지연 실행 예시                             |
|--------|----------------------------------------|---------------------------------------------|
| Rust   | `.collect()`                           | `.iter().map().filter()`                    |
| C++    | `std::vector`, `std::transform`        | `std::views::transform`, `std::views::filter` |

---
