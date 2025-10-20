
# 🦀 Rust: 즉시 vs 지연 실행
## ✅ 즉시 실행: collect(), for 루프, count(), sum() 등
- collect()는 iterator를 소비하며 결과를 즉시 메모리에 저장
- for 루프도 iterator를 소비하며 즉시 실행됨
- 이 시점부터는 더 이상 lazy가 아님
```rust
let v = vec![1, 2, 3, 4, 5];
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect(); // 즉시 실행
```
- map()은 lazy지만 collect()가 호출되면서 모든 요소가 계산됨

## 💤 지연 실행: iter(), map(), filter() 등
- Iterator 체인은 lazy하게 구성됨
- collect()나 for 루프 등 **소비자(consuming adaptor)**가 호출되기 전까지는 아무 것도 실행되지 않음
```rust
let lazy = (1..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 3); // 아직 실행 안 됨

for val in lazy {
    println!("{}", val); // 이 시점에 계산됨
}
```
- filter와 map은 필요할 때마다 한 요소씩 계산
- 메모리 효율적이고 계산 지연 가능

## 🧠 핵심 요약: Rust의 Iterator 실행 방식
| 표현                    | 실행 시점       | 설명                                       |
|-------------------------|------------------|--------------------------------------------|
| `collect()`             | ✅ 즉시 실행     | 모든 요소를 계산하고 컬렉션으로 저장함     |
| `for` 루프              | ✅ 즉시 실행     | 요소를 하나씩 계산하며 소비함              |
| `iter()`, `map()`, `filter()` | ❌ 지연 실행 | 소비자가 호출되기 전까지 계산되지 않음     |



## 💡 실무 팁
- 대량 데이터 처리 시에는 lazy iterator로 메모리 절약 가능
- 필터링 후 일부만 사용할 경우 take(), find() 등과 함께 lazy하게 처리
- collect()는 결과를 저장해야 할 때만 사용

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

# Iterator 구현 / peekable, fuse, inspect
Rust의 Iterator는 lazy하게 동작하며, next() 호출을 통해 하나씩 값을 생성합니다.  
고급 어댑터인 peekable, fuse, inspect는 이 흐름을 제어하거나 확장하는 데 사용됩니다.

## 🔍 Iterator의 내부 동작과 next() 흐름
- 모든 Iterator는 next() 메서드를 구현해야 합니다.
- next()는 Option<Item>을 반환하며, Some(value) 또는 None으로 반복 종료를 나타냅니다.
- for 루프, collect(), find() 등은 내부적으로 next()를 반복 호출합니다.
```rust
let mut iter = vec![1, 2, 3].into_iter();
while let Some(val) = iter.next() {
    println!("{}", val); // 1, 2, 3 출력
}
```


## 🧠 고급 어댑터 설명
### 1️⃣ peekable()
- 다음 값을 **미리 확인(peek)**할 수 있게 해주는 어댑터
- peek()은 값을 소비하지 않고 참조만 반환
```rust
let mut iter = [1, 2, 3].iter().peekable();
println!("{:?}", iter.peek()); // Some(&1)
println!("{:?}", iter.next()); // Some(1)
```

- 여러 번 peek()해도 next()가 호출되기 전까지는 값이 유지됨

### 2️⃣ fuse()
- 반복자가 None을 반환한 이후에도 계속 None만 반환하도록 고정시킴
- 일부 반복자는 None 이후에도 다시 Some을 반환할 수 있는데, 이를 방지
```rust
let mut iter = [1].iter().fuse();
println!("{:?}", iter.next()); // Some(1)
println!("{:?}", iter.next()); // None
println!("{:?}", iter.next()); // None (보장됨)
```

- 실무에서는 Iterator를 여러 번 소비할 때 안전하게 쓰기 위해 사용

### 3️⃣ inspect()
- 각 요소를 중간에 엿보고(side-effect) 처리할 수 있음
- 디버깅, 로깅, 통계 수집 등에 유용
```rust
let v: Vec<_> = (1..5)
    .inspect(|x| println!("처리 중: {}", x))
    .map(|x| x * 2)
    .collect();
```

- inspect()는 값을 변경하지 않으며, 체인 중간에 끼워 넣을 수 있음

## 📌 실전 팁
- peekable()은 파서 구현이나 조건부 반복에 유용
- fuse()는 반복자가 예외적으로 Some을 다시 반환할 수 있는 경우에 안전망 역할
- inspect()는 로깅이나 디버깅에 매우 유용하며, 성능에 큰 영향을 주지 않음


## ✅ 1. Iterator 직접 구현하기
Rust에서 Iterator를 직접 구현하려면 trait Iterator를 구현하고 next() 메서드를 정의해야 합니다.
### 🔧 예제: 0부터 시작하는 짝수 생성기
```rust
struct EvenNumbers {
    current: usize,
}

impl Iterator for EvenNumbers {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current += 2;
        Some(result)
    }
}

fn main() {
    let mut evens = EvenNumbers { current: 0 };
    for n in evens.take(5) {
        println!("{}", n); // 0, 2, 4, 6, 8
    }
}
```

- next()는 호출될 때마다 새로운 값을 반환
- take(5)는 처음 5개만 가져오는 어댑터

## 🔁 2. 반복 제어 어댑터들
### 🔹 step_by(n)
- 지정한 간격으로 건너뛰며 반복
```rust
for i in (0..10).step_by(2) {
    println!("{}", i); // 0, 2, 4, 6, 8
}
```


### 🔹 enumerate()
- 각 요소에 인덱스를 붙여 (index, value) 튜플로 반환
```rust
for (i, val) in ["a", "b", "c"].iter().enumerate() {
    println!("{}: {}", i, val);
}
```


### 🔹 zip()
- 두 개의 iterator를 병렬로 묶어 (a, b) 튜플로 반환
```rust
let names = ["JungHwan", "Minji"];
let scores = [95, 88];

for (name, score) in names.iter().zip(scores.iter()) {
    println!("{}: {}", name, score);
}
```


###  🔹 cycle()
```rust
- 반복자를 무한히 반복
for val in [1, 2, 3].iter().cycle().take(7) {
    println!("{}", val); // 1, 2, 3, 1, 2, 3, 1
}
```

- take()와 함께 사용하면 원하는 개수만 반복 가능

## 🧠 실전 팁
- Iterator 직접 구현은 커스텀 로직이 필요한 경우 유용
- zip은 여러 데이터 소스를 병렬로 처리할 때
- enumerate는 인덱스 기반 처리에 필수
- cycle은 애니메이션, UI, 게임 루프 등에서 반복 효과에 활용

---



