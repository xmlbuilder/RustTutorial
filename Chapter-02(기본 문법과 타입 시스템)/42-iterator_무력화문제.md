# iterator 무력화 문제
C++의 std::vector나 std::map에서 순회 중에 요소를 삭제하면 iterator가 무력화되거나 undefined behavior가 발생하는 문제,
Rust는 이걸 언어 구조와 안전성 모델로 원천적으로 차단.

## 🔥 C++ STL의 문제점
| 컨테이너        | 삭제 방식             | 문제점 및 주의사항                                      |
|-----------------|-----------------------|---------------------------------------------------------|
| vector          | vec.erase(it); it++   | `it`가 무효화됨 → 다음 요소 접근 시 undefined behavior 가능성 |
| map             | map.erase(it++)       | 일부 구현에서는 안전하지만, 순회 흐름이 복잡해지고 실수 유발 가능 |
| set             | set.erase(it++)       | `map`과 유사하지만, 여전히 iterator 무효화 주의 필요       |
| unordered_map   | erase(it++)           | 내부 해시 구조로 인해 iterator 무효화 가능성 더 높음       |
| list            | list.erase(it++)      | 비교적 안전하지만, 순회 흐름이 꼬일 수 있음                 |

C++은 이런 문제를 개발자 책임으로 방치하고 있음.
그래서 “잘 쓰면 괜찮지만, 실수하면 치명적”인 구조.

## 🛡️ Rust의 접근 방식: 안전이 기본값
Rust는 이런 문제를 컴파일 타임에 원천적으로 막습니다.
### 1. 소유권과 빌림 시스템
- Vec, HashMap 등에서 순회 중에 요소를 삭제하려면 mutable iterator를 명시적으로 사용해야 함
- 동시에 읽고 쓰는 건 불가능 → 컴파일 에러
### 2. retain() 방식으로 안전하게 삭제
```rust
let mut v = vec![1, 2, 3, 4, 5];
v.retain(|&x| x != 3); // ✅ 안전하게 조건 기반 삭제
```

- 내부적으로 안전하게 처리되며 iterator 무효화 없음
### 3. drain()으로 삭제 + 순회
```rust
fn main() {
    let mut numbers = vec![10, 15, 20, 25, 30];

    // 20보다 큰 값만 제거하면서 출력
    for removed in numbers.drain_filter(|x| *x > 20) {
        println!("삭제된 값: {}", removed);
    }

    println!("남은 값: {:?}", numbers);
}
```

- drain()은 요소를 제거하면서도 소유권을 넘겨주기 때문에 안전
### 4. into_iter()는 소비 기반
- into_iter()는 컬렉션을 소비하면서 순회 → 삭제와 충돌 없음

## 🧠 핵심 철학
“순회 중에 삭제하고 싶으면, 그 의도를 명확히 표현하고, 컴파일러가 그걸 안전하게 보장할 수 있어야 한다.”


## 🔍 예시: HashMap에서 삭제
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);

let keys_to_remove: Vec<_> = map.iter()
    .filter(|(_, &v)| v == 2)
    .map(|(k, _)| *k)
    .collect();

for k in keys_to_remove {
    map.remove(k); // ✅ 안전하게 삭제
}
```

- 순회 중 삭제는 별도의 키 목록을 만들어서 처리
- iterator 무효화 없음, 컴파일러가 안전성 보장

## 💡 결론
Rust는 C++이 “개발자 책임”으로 넘긴 iterator 무효화 문제를 언어 차원에서 구조적으로 막아버리는 언어.

---

