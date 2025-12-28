## 많이 틀리는 mut 사용법

### 🎯 왜 `let mut knots = kv.knots;` 가 문제인가?
- 이 코드는 **복사(copy)** 가 아니라 **move**.
- 즉:
```rust
let mut knots = kv.knots;
```

- 이렇게 하면:
  - kv.knots(Vec<f64>)의 소유권이 knots로 이동
  - 원래 kv.knots는 비어 있는 상태가 됨
  - 이후 knots를 수정해도 KnotVector 내부는 전혀 바뀌지 않음
- 그래서 너는 knots를 수정했는데 정작 kv.knots는 그대로라서 **왜 값이 안 바뀌지?** 같은 문제가 생기는 것임.

### 🎯 올바른 방식: **참조(reference)** 로 가져오기
```rust
let knots = &mut kv.knots;
```

- 이건 소유권을 가져오는 게 아니라, 원본 Vec을 직접 가리키는 포인터를 가져오는 것.
- 그래서 knots를 수정하면
  - kv.knots가 그대로 수정됨
  - 우리가 원하는 in-place 수정이 정확히 일어남

### 🔥 이 둘의 차이를 한 줄로 요약하면
| Rust 코드                      | 의미                                         |
|-------------------------------|----------------------------------------------|
| `let mut knots = kv.knots;`   | Vec의 **소유권을 가져옴(move)** → 원본 변경 안 됨 |
| `let knots = &mut kv.knots;`  | Vec을 **빌려옴(mutable borrow)** → 원본이 직접 수정됨 |

- Rust는 이 둘을 엄격하게 구분하기 때문에 헷갈리면 바로 버그가 생김.

### 🎨 비유로 설명하면
- ❌ let mut knots = kv.knots;
  - **냉장고(kv.knots)를 통째로 가져와서 내 집(knots)으로 옮겨버림**
  - 냉장고는 원래 집(kv)에서 사라짐.

- ✔ let knots = &mut kv.knots;
  - **냉장고는 그대로 두고, 냉장고 문만 열어서 안에 있는 걸 꺼내 쓰는 것**
  - 원본은 그대로 있고, 안의 내용만 바뀜.

### 😊 앞으로 절대 헷갈리지 않게 기억하는 법
- ✔ 규칙 1
  - 점(.) 뒤에 있는 필드를 그대로 쓰면 move
```
let x = kv.knots; // move
```

- ✔ 규칙 2
- & 또는 &mut를 붙이면 borrow
```rust
let x = &kv.knots;     // immutable borrow
let x = &mut kv.knots; // mutable borrow
```

- ✔ 규칙 3
  - Vec을 수정하고 싶으면 반드시 &mut로 가져오기

### 🎉 결론
- Rust가 원래 이런 부분이 헷갈리게 되어 있음.
- 하지만 지금처럼 정확히 짚고 넘어가면 앞으로는 이런 move/borrow 실수 거의 안 할 것임.

---

