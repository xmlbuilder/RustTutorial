# Tuple을 Map키로 사용하기

Rust에서 튜플은 기본적으로 **사전순 정렬(lexicographical order)** 을 따르기 때문에,  
우리가 원하는 정렬 우선순위를 만들기 위해서는 튜플의 구성 순서 자체를 조작하면 됩니다.

## 🧠 기본 튜플 정렬 방식
Rust에서 (a, b, c) 형태의 튜플은 Ord가 구현되어 있으면 다음 순서로 비교됩니다:
- a를 먼저 비교
- 같으면 b를 비교
- 그래도 같으면 c를 비교
즉, 튜플의 앞쪽 요소가 우선순위를 결정합니다.

```rust
let mut items = vec![
    (2, "apple"),
    (1, "banana"),
    (2, "banana"),
];

items.sort(); // 결과: [(1, "banana"), (2, "apple"), (2, "banana")]
```

## 🔧 원하는 방식으로 정렬하려면?
### ✅ 1. 튜플 순서 바꾸기
```rust
let mut items = vec![
    ("apple", 2),
    ("banana", 1),
    ("banana", 2),
];
items.sort(); // 문자열 우선 정렬됨
```

### ✅ 2. 정렬 기준을 커스텀 클로저로 지정
```rust
items.sort_by(|a, b| a.1.cmp(&b.1)); // 숫자 기준으로 정렬
```

### ✅ 3. 정렬 우선순위를 튜플로 만들어서 `sort_by_key`
```rust
let mut items = vec![
    ("banana", 2),
    ("apple", 1),
    ("banana", 1),
];

items.sort_by_key(|item| (item.1, item.0)); // 숫자 우선, 그다음 문자열
```


## ✨ 실전 예: 복합 우선순위 정렬
```rust
#[derive(Debug)]
struct Task {
    priority: u8,
    deadline: u32,
    name: String,
}
```
```rust
let mut tasks = vec![
    Task { priority: 2, deadline: 100, name: "Write".into() },
    Task { priority: 1, deadline: 200, name: "Read".into() },
    Task { priority: 1, deadline: 100, name: "Code".into() },
];
```
```rust
tasks.sort_by_key(|t| (t.priority, t.deadline)); // priority → deadline 순으로 정렬
```


## 💡 요약 – 튜플 기반 정렬 우선순위 지정
| 정렬 방식             | 설명                                      |
|----------------------|-------------------------------------------|
| `sort_by` / `sort_by_key` | 클로저 또는 키 추출 방식으로 정렬 기준 지정 |
| `(기준1, 기준2, …)`   | 튜플로 복합 기준 지정 → 앞쪽 요소가 우선순위 |

---

# HashMap / BTreeMap Tuple Key

HashMap이나 BTreeMap에 키로 들어갈 때는 우리가 직접 정렬을 호출하는 게 아니라,  
키 자체의 비교 기준에 따라 자동으로 정렬되거나 해시 충돌이 처리되기 때문에, 원하는 우선순위를 만들려면 키 구조 자체를 설계해야 함.

## 🧠 핵심: 키로 들어가는 튜플은 `비교 기준` 을 내포한 구조여야 한다
Rust에서 HashMap은 해시 기반이라 정렬은 없지만, BTreeMap은 키의 Ord 구현에 따라 자동 정렬됩니다.

## ✅ 튜플 키는 앞쪽 요소부터 비교됨
```rust
let mut map = BTreeMap::new();
map.insert((priority, timestamp), value);
```

- priority가 먼저 비교되고
- 같으면 timestamp가 비교됨
즉, 우선순위가 높은 기준을 튜플의 앞쪽에 배치해야 원하는 정렬이 됩니다.

## 🔧 원하는 우선순위를 만들기 위한 튜플 설계 전략
| 목적                      | 튜플 구조                          | 설명                                      |
|---------------------------|------------------------------------|-------------------------------------------|
| 우선순위 → 시간 순         | `(priority, timestamp)`            | 높은 우선순위 먼저, 같으면 오래된 순       |
| 날짜 → 이름 순            | `(date, name)`                     | 날짜 기준 정렬, 같으면 이름순              |
| 사용자 등급 → 활동량 순    | `(user_level, activity_score)`     | 등급 높은 사용자 먼저, 같으면 활동 많은 순 |

- 이 구조는 BTreeMap뿐 아니라 Vec 정렬에도 그대로 적용 가능.


### ✅ 실전 예: 이벤트 우선순위 큐
```rust
#[derive(Debug)]
struct Event {
    priority: u8,
    timestamp: u64,
    name: String,
}
```
```rust
let mut queue = BTreeMap::new();
let event = Event { priority: 1, timestamp: 100, name: "Start".into() };
queue.insert((event.priority, event.timestamp), event);
```
- BTreeMap은 (priority, timestamp) 기준으로 자동 정렬됨
- priority가 낮을수록 먼저 처리됨

## 💡 요약 – 튜플 키와 맵 구조의 비교 기준

| 키 구조      | 비교 기준 / 요구 트레이트 |
|--------------|----------------------------|
| `(a, b, c)`  | 앞쪽 요소부터 순차 비교     |
| `BTreeMap`   | `Ord` 구현 필요 → 자동 정렬 |
| `HashMap`    | `Eq` + `Hash` 구현 필요     |

---


# 공간은 index로 바꾸어서 key로 활용
 핵심 아이디어: 공간을 격자로 나누고 인덱스를 키로 사용
## ✅ 흐름 요약
- 실수 좌표 → 정수 인덱스 변환
```rust
let i = (x / cell_size).floor() as i32;
let j = (y / cell_size).floor() as i32;
let k = (z / cell_size).floor() as i32;
```
- 튜플 키 생성
```rust
let key = (i, j, k);
```
- HashMap 또는 BTreeMap에 저장 및 조회
```rust
let mut space = HashMap::new();
space.insert(key, value);
```

## 🔧 장점
- 정확한 위치 비교 대신 근사 셀 단위로 처리 → 부동소수점 문제 회피
- 튜플 키는 `Eq + Hash` 자동 구현됨 → HashMap에서 바로 사용 가능
- 정렬이 필요하면 BTreeMap으로 전환 가능 → Ord도 자동 구현됨

## ✨ 실전 활용 예 – 튜플 키 기반 공간 인덱싱
| 분야           | 활용 방식                                      |
|----------------|-------------------------------------------------|
| 게임 엔진      | 월드 좌표를 셀 단위로 나눠서 객체 배치 및 충돌 처리 |
| 물리 시뮬레이션 | 입자 위치를 셀로 나눠서 근접 계산 최적화           |
| 3D 캐싱        | 렌더링된 블록을 셀 단위로 저장 및 조회             |
| 공간 인덱싱    | GIS 좌표를 격자화하여 빠른 검색 구현               |
| 로봇 경로 탐색 | 환경을 셀로 나눠서 장애물 탐지 및 경로 계획         |


## 💡 요약 – 셀 기반 공간 키 설계

| 개념        | 설명                                 |
|-------------|--------------------------------------|
| `floor(x / cell_size)` | 좌표를 셀 인덱스로 정규화             |
| `(i, j, k)` | 셀 인덱스를 튜플로 묶어 키로 사용         |
| `HashMap`   | `Eq` + `Hash` 필요 → 빠른 조회 가능     |
| `BTreeMap`  | `Ord` 필요 → 자동 정렬 가능             |


---

# HashMap 효율성 증가

지금처럼 단순히 (i, j, k) 인덱스를 기반으로 데이터 저장과 조회만 필요하다면 HashMap이 훨씬 효율적이고 적합한 선택.

## ✅ 왜 HashMap이 더 적합한가?
| 조건 또는 구조     | 설명                                      |
|--------------------|-------------------------------------------|
| `BTreeMap`         | 자동 정렬됨 → 정렬이 필요할 때 적합         |
| `HashMap`          | 빠른 조회 가능 → 정렬 불필요할 때 적합      |
| `(i, j, k)` 튜플 키 | `Eq` + `Hash` 자동 구현 → 바로 사용 가능     |
| 목적이 조회 중심    | `HashMap`이 성능과 단순성에서 더 유리함     |


## 🔧 실전 예
```rust
use std::collections::HashMap;

let mut space: HashMap<(i32, i32, i32), Data> = HashMap::new();

let i = (x / cell_size).floor() as i32;
let j = (y / cell_size).floor() as i32;
let k = (z / cell_size).floor() as i32;

space.insert((i, j, k), data);
```
- Data는 원하는 구조체나 값
- (i, j, k)는 셀 인덱스 → 키로 사용

## 💡 요약 – 맵 선택 기준과 튜플 키 활용

| 목적 또는 구조     | 추천 맵     | 요구 트레이트 |
|--------------------|-------------|----------------|
| 빠른 조회, 정렬 불필요 | `HashMap`   | `Eq` + `Hash`   |
| 자동 정렬 필요        | `BTreeMap`  | `Ord`           |
| 튜플 키 `(i, j, k)`   | 둘 다 가능  | 자동 트레이트 구현됨 |

---


