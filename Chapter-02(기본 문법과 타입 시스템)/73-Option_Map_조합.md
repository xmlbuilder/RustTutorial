# Option  + Map 조합

## 샘플 예시
```rust
let mut scores: HashMap<&str, Option<u32>> = HashMap::new();
let result = scores.get(name).and_then(|opt| opt.map(|score| score + 10));
```

## 🧠 표현: `|opt| opt.map(|score| score + 10)`

이건 클로저(익명 함수)를 정의한 것이고, `Option<Option<u32>>` 같은 중첩된 Option을 다룰 때 자주 사용됩니다.

### 🔹 |opt| ... — 클로저 정의
- opt는 Option<u32> 타입의 값입니다.
- 이 클로저는 Option<u32>를 받아서 내부 값을 처리합니다.
### 🔹 opt.map(...) — Option의 map 메서드
- map은 Option이 Some(value)일 때만 내부 값을 꺼내서 클로저를 적용합니다.
- None이면 아무 것도 하지 않고 그대로 None을 반환합니다.
즉, opt.map(|score| score + 10)의 의미는:
- opt가 Some(score)일 경우 → Some(score + 10)으로 변환
- opt가 None일 경우 → 그대로 None

## 🔍 전체 흐름 예시
```rust
let scores: HashMap<&str, Option<u32>> = ...;
let result = scores.get("Alice").and_then(|opt| opt.map(|score| score + 10));
```
- scores.get("Alice") → Option<&Option<u32>>
- and_then은 Option<&Option<u32>>에서 Option<u32>로 한 단계 벗겨냄
- opt.map(...)은 내부 u32 값을 꺼내서 +10 처리

## ✅ Option 처리 요약

| 표현 | 동작 설명 |
|------|------------|
| `opt.map(f)` | `opt`가 `Some(x)`일 때 `f(x)`를 적용하여 `Some(f(x))` 반환. `None`이면 그대로 `None` |
| `None.map(f)` | 아무 것도 하지 않고 `None` 반환 |
| `and_then(\|opt\| opt.map(...))` | 중첩된 `Option<Option<T>>`에서 내부 `Option<T>`를 꺼내고 `map`으로 처리. 최종적으로 `Option<U>` 반환 |

---



