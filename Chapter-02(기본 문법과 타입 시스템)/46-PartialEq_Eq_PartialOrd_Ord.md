# PartialEq / Eq / PartialOrd / Ord
아래는 요청하신 PartialEq, Eq, PartialOrd, Ord에 대한 핵심 정리입니다.
실무에서 타입 비교와 정렬을 설계할 때 꼭 알아야 할 부분만 간결하게 정리:

## ✅ PartialEq & Eq
| Trait      | 핵심 기능           |
|------------|---------------------|
| PartialEq  | `eq`, `ne` 메서드 → `==`, `!=` 연산자 지원 |
| Eq         | `PartialEq` 기반 → 완전 등가 보장 (반사성 필요) |

### 예시
```rust
struct Key {
    id: u32,
    metadata: Option<String>,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
```

- metadata는 비교하지 않음 → 부분 등가
- Eq는 구현하지 않음 → 반사성 보장 안 됨
서로 다른 타입 간 비교
```rust
impl PartialEq<u32> for Key {
    fn eq(&self, other: &u32) -> bool {
        self.id == *other
    }
}
```

- PartialEq<T>는 타입 간 비교 가능
- Eq는 타입이 같아야만 구현 가능

### ✅ PartialOrd & Ord
| Trait       | 핵심 기능                     |
|-------------|-------------------------------|
| PartialOrd  | `partial_cmp` 메서드 → `<`, `<=`, `>`, `>=` 연산자 지원 |
| Ord         | `cmp` 메서드 → 항상 `Ordering` 반환 |

### 예시
```rust
#[derive(Eq, PartialEq)]
struct Citation {
    author: String,
    year: u32,
}

impl PartialOrd for Citation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.author.partial_cmp(&other.author) {
            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
            author_ord => author_ord,
        }
    }
}
```

- author 기준으로 먼저 비교, 같으면 year 비교
- Ord를 구현하려면 cmp()로 항상 Ordering을 반환해야 함

## 🔚 요약: 비교 관련 Trait
| Trait        | 연산자 지원       | 반환 타입         |
|--------------|-------------------|-------------------|
| PartialEq    | `==`, `!=`        | `bool`            |
| Eq           | (PartialEq 기반)  | 완전 등가 보장     |
| PartialOrd   | `<`, `<=`, `>`, `>=` | `Option<Ordering>` |
| Ord          | (전체 순서 보장)  | `Ordering`        |


---
# 반사성

“반사성(reflexivity)”이라는 용어는 수학과 컴퓨터 과학에서 관계의 성질을 설명할 때 자주 등장하지만,  
Rust나 일반 프로그래밍에서는 처음 들으면 낯설 수 있음.

## 🔍 반사성(reflexivity)이란?
자기 자신과의 관계가 항상 참(True)인 성질을 말합니다.
예시로 설명하면:
- 어떤 값 a가 있을 때
→ a == a가 항상 참이면, 그 비교는 반사적이라고 말합니다.

## ✅ Rust에서 반사성이 중요한 이유
- Eq trait은 반사성을 보장해야만 구현 가능합니다.
- 즉, Eq를 구현한 타입은
→ a == a가 항상 true여야 해요.
### 예시
```rust
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
```


- 여기서 Point는 Eq를 구현했기 때문에
→ 어떤 p: Point에 대해 p == p는 항상 true여야 함

### ⚠️ 반사성이 깨질 수 있는 경우
- NaN (Not a Number) 같은 값은
→ NaN == NaN이 false이기 때문에
→ Eq를 구현할 수 없고, PartialEq만 가능함

## 🔚 요약: Eq vs PartialEq
| Trait       | 반사성 예시        |
|-------------|--------------------|
| Eq          | `a == a` 항상 참   |
| PartialEq   | `NaN == NaN`은 거짓 |


---

# PartialEq fro f64

아래는 f64 타입에 대해 **허용 오차(tolerance)**를 적용한 PartialEq 구현 예시입니다.
두 값이 10e-5 이내로 차이가 나면 같다고 판단하는 방식:
```rust
const TOLERANCE: f64 = 1e-5;

#[derive(Debug)]
struct Approximate(f64);

impl PartialEq for Approximate {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < TOLERANCE
    }
}

fn main() {
    let a = Approximate(1.00001);
    let b = Approximate(1.00002);

    println!("a == b: {}", a == b); // true
}
```


## ✅ 핵심 포인트
- abs()로 절대값 차이를 구하고
→ TOLERANCE보다 작으면 같다고 판단
- Eq는 구현하지 않음 → 반사성은 만족하지만, NaN 같은 예외를 고려하면 안전하게 PartialEq만

Rust에서는 표준 타입(f64, i32 등)에 대해 직접 impl을 추가하는 것이 막혀 있습니다.  
이건 Rust의 고의적인 안전 설계.

## 🔒 왜 직접 impl이 막혀 있을까?
| 원칙 또는 이유     | 설명 |
|--------------------|------|
| Orphan Rule        | trait과 타입 중 하나는 현재 crate에 있어야 `impl` 가능 |
| 타입 안정성        | 표준 타입은 외부에서 수정 불가 → 충돌 방지 |
| API 일관성         | 핵심 타입은 Rust 팀이 관리 → 예측 가능한 동작 보장 |
| 충돌 방지          | 여러 crate에서 같은 타입에 다른 `impl`을 추가하면 혼란 발생 |


## ✅ 해결 방법: 새 타입으로 감싸기
```rust
struct Approximate(f64);
impl PartialEq for Approximate {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-5
    }
}
```

- f64에 직접 impl은 불가능하지만
→ Approximate(f64)처럼 newtype 패턴을 쓰면 가능
- 이 방식은 타입 안전성과 커스터마이징을 동시에 확보할 수 있어요

## 🧠 Orphan Rule 요약
| 조건                          | impl 가능 여부 |
|-------------------------------|----------------|
| 내가 만든 타입 + 표준 trait   | ✅ 가능         |
| 표준 타입 + 내가 만든 trait   | ✅ 가능         |
| 표준 타입 + 표준 trait        | ❌ 불가능       |


## ✅ 예시로 이해하기
| 구현 시도                    | Trait 출처 | 타입 출처 | 가능 여부 |
|-----------------------------|------------|-----------|-----------|
| impl MyTrait for f64        | 내 crate   | 표준 타입 | ✅ 가능    |
| impl PartialEq for f64      | 표준 trait | 표준 타입 | ❌ 불가능  |
| impl PartialEq for MyType   | 표준 trait | 내 crate  | ✅ 가능    |

---

# impl MyTrait for f64 허용 이유

impl MyTrait for f64가 가능한 이유는 From, Into 같은 trait을 사용자 정의 타입에 적용하기 위한 확장성도 있지만,
핵심은 Rust의 Orphan Rule이 허용하는 범위 내에서 표준 타입에 대해 내가 만든 trait은 자유롭게 구현할 수 있도록 설계되어 있기 때문.

## 🔍 왜 impl MyTrait for f64는 허용될까?
| 조건 또는 목적             | 설명 |
|----------------------------|------|
| Orphan Rule 허용           | Trait 또는 Type 중 하나가 내 crate에 있으면 `impl` 가능 |
| 내 trait을 표준 타입에 적용 | `impl MyTrait for f64`는 내가 만든 trait이므로 허용됨 |
| 타입 변환 목적             | `impl From<f64> for MyUnit`으로 변환 로직 작성 가능 |
| 도메인 확장                | 표준 타입에 내 도메인 기능을 추가할 수 있음 (예: 단위 변환, 수치 연산 등) |


## ✅ 예시: 내 trait을 f64에 적용
```rust
trait MyTrait {
    fn describe(&self) -> String;
}

impl MyTrait for f64 {
    fn describe(&self) -> String {
        format!("{} (float)", self)
    }
}
```

- 이건 내 crate에서 만든 trait이기 때문에
→ 표준 타입 f64에 자유롭게 impl 가능

### ⚠️ 반대로는 불가능
```rust
// ❌ 불가능: 표준 trait + 표준 타입
impl PartialEq for f64 { ... } // 컴파일 에러
```

- PartialEq도 표준, f64도 표준 → 둘 다 외부 것이므로 impl 불가

## 🔚 요약: impl 가능 여부
| 구현 시도                   | Trait 출처 | 타입 출처 | 가능 여부 |
|----------------------------|------------|-----------|-----------|
| impl MyTrait for f64       | 내 crate   | 표준 타입 | ✅ 가능    |
| impl From<f64> for MyType  | 표준 trait | 내 crate  | ✅ 가능    |
| impl PartialEq for f64     | 표준 trait | 표준 타입 | ❌ 불가능  |

## ✅ 그래서 어떻게 해야 할까?
### 1. Newtype 패턴으로 감싸기
```rust
struct Approximate(f64);

impl PartialEq for Approximate {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-5
    }
}
```

- Approximate는 내가 만든 타입이므로
→ 표준 trait PartialEq, Ord, Hash 등을 자유롭게 구현 가능
### 2. 내 trait을 f64에 붙이기
```rust
trait MyFloatOps {
    fn is_approx_eq(&self, other: &Self, tol: f64) -> bool;
}

impl MyFloatOps for f64 {
    fn is_approx_eq(&self, other: &Self, tol: f64) -> bool {
        (self - other).abs() < tol
    }
}
```

- 내가 만든 trait이므로
→ 표준 타입 f64에 붙이는 건 허용됨

## 🔍 Rust vs C# 확장성 비교
| 언어   | 확장 방식               | 적용 대상      | 구현 방식 예시               |
|--------|-------------------------|----------------|------------------------------|
| C#     | static extension method | 표준 타입      | `public static bool IsApproxEqual(this double a, ...)` |
| Rust   | trait 기반 확장         | 표준 타입      | `impl MyTrait for f64 { ... }` |


## ✅ C# 예시
```csharp
public static class FloatExtensions {
    public static bool IsApproxEqual(this double a, double b, double tol = 1e-5) {
        return Math.Abs(a - b) < tol;
    }
}
```

- 1.0.IsApproxEqual(1.00001)처럼 호출 가능

## ✅ Rust 대응
```rust
trait ApproxEq {
    fn approx_eq(&self, other: &Self, tol: f64) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self, tol: f64) -> bool {
        (self - other).abs() < tol
    }
}
```
- a.approx_eq(b, 1e-5)처럼 호출 가능

## 🧠 Rust의 철학적 차이
- Rust는 trait 기반 확장이므로
    → 명시적 trait import (use)가 필요할 수 있음
- C#은 global scope에서 자동 확장됨
