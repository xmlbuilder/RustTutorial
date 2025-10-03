# zip / unzip
Rust의 zip과 unzip은 반복자(iterator)를 다룰 때 정말 강력한 도구.  
두 개의 컬렉션을 짝지어 묶거나, 묶인 쌍을 다시 분리할 때 사용됩니다.  
아래에 개념 설명 + 샘플 코드 + 주의사항까지 깔끔하게 정리.  

## 🔗 zip: 두 반복자를 쌍으로 묶기
### ✅ 개념
- 두 반복자를 병렬로 순회하면서 (a, b) 형태의 튜플로 묶습니다.
- 길이가 다르면 짧은 쪽 기준으로 잘립니다.

#### 🧪 예제
```rust
fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![85, 92, 78];
    let pairs: Vec<(&str, i32)> = names.iter().zip(scores.iter()).map(|(n, s)| (*n, *s)).collect();
    println!("{:?}", pairs); // [("Alice", 85), ("Bob", 92), ("Charlie", 78)]
}
```

- zip()은 (&str, &i32)를 반환하므로 *n, *s로 역참조 필요
- collect()로 Vec<(T1, T2)>로 변환

### 🔄 unzip: 튜플 반복자를 두 벡터로 분리
#### ✅ 개념
- (a, b) 형태의 반복자를 받아서
`Vec<A>`와 `Vec<B>`로 각각 분리합니다.

#### 🧪 예제

```rust
fn main() {
    let pairs = vec![("Alice", 85), ("Bob", 92), ("Charlie", 78)];

    let (names, scores): (Vec<&str>, Vec<i32>) = pairs.into_iter().unzip();

    println!("{:?}", names);  // ["Alice", "Bob", "Charlie"]
    println!("{:?}", scores); // [85, 92, 78]
}
```

#### 🧪 샘플: 두 벡터를 zip해서 합산
```rust
fn main() {
    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30];

    // zip으로 묶고, map으로 더한 후, collect로 벡터로 수집
    let summed: Vec<i32> = a.iter()
                            .zip(b.iter())
                            .map(|(x, y)| x + y)
                            .collect();

    println!("{:?}", summed); // [11, 22, 33]
}
```

#### 🔍 설명
- a.iter().zip(b.iter()): (1, 10), (2, 20), (3, 30) 형태로 묶음
- .map(|(x, y)| x + y): 각 쌍을 더함
- .collect(): 결과를 Vec<i32>로 수집

#### ✅ 참고
- iter()는 참조를 반환하므로 x와 y는 &i32 타입 → x + y는 자동 역참조됨
- into_iter()를 쓰면 move가 발생하므로 String, 구조체 등에서는 주의 필요

- into_iter().unzip()은 튜플을 자동으로 분리해줍니다
- 타입 추론이 어려울 경우 (Vec<T1>, Vec<T2>) 명시 필요


### ⚠️ 주의사항: zip / unzip / 반복자
| 항목       | 관련 메서드 또는 개념     | 설명 또는 주의사항                                 |
|------------|----------------------------|----------------------------------------------------|
| 길이 불일치 | `zip()` vs `zip_longest`   | Rust의 `zip()`은 짧은 쪽 기준으로 잘림 (`zip_longest` 없음) |
| 반복자 종류 | `iter()` vs `into_iter()`  | `iter()`는 참조, `into_iter()`는 값 이동            |
| 분리        | `unzip()`                  | 튜플 반복자를 두 벡터로 분리, 타입 명시 필요         |



## 🧠 실전 팁
- zip()은 enumerate()와 함께 쓰면 인덱스와 값 묶기 가능
- unzip()은 map()으로 만든 튜플을 분리할 때 유용
- zip().map().collect()는 Rust에서 가장 흔한 데이터 변환 패턴 중 하나

---

# zip_with, multi_zip, itertools::izip!

Rust의 반복자 패턴은 Python의 zip, enumerate, map과 매우 유사하면서도
타입 안정성과 성능 면에서 훨씬 정교하게 설계되어 있음.  
아래에 zip_with, multi_zip, itertools::izip! 같은 고급 zip 패턴을 깔끔하게 정리.

## 🧠 고급 zip 패턴 in Rust
### 1. ✅ zip_with: 직접 zip을 구현하는 방식
Rust에는 zip_with라는 이름의 메서드는 없지만,  
zip().map() 조합으로 직접 zip-with 기능을 구현할 수 있음.
```rust
fn main() {
    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30];

    let sum: Vec<i32> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
    println!("{:?}", sum); // [11, 22, 33]
}
```

- Python의 zip(a, b) + map(lambda x, y: x + y)와 동일한 효과
- 이게 사실상 Rust의 zip_with 패턴입니다

### 2. ✅ multi_zip: 여러 반복자 zip
기본 zip()은 두 개까지만 지원되지만,
세 개 이상 zip하려면 중첩하거나 itertools를 사용해야 합니다.

- itertools는 Rust 표준 라이브러리에는 포함되어 있지 않고, **외부 크레이트(라이브러리)** 로 설치해야 합니다.

#### 📦 설치 방법
Cargo.toml에 아래처럼 추가해주면 됩니다:
```
[dependencies]
itertools = "0.11"
```

```rust
use itertools::multizip;

fn main() {
    let a = vec![1, 2];
    let b = vec![10, 20];
    let c = vec![100, 200];

    for (x, y, z) in multizip((a, b, c)) {
        println!("{} + {} + {} = {}", x, y, z, x + y + z);
    }
}
```
- multizip((a, b, c))은 (x, y, z) 튜플로 묶어줍니다

### 3. ✅ itertools::izip!: 매크로 기반 zip
izip!은 반복자 zip을 매크로 형태로 간결하게 표현할 수 있어요.
```rust
use itertools::izip;

fn main() {
    let names = vec!["Alice", "Bob"];
    let scores = vec![85, 92];
    let grades = vec!['B', 'A'];

    for (name, score, grade) in izip!(names, scores, grades) {
        println!("{}: {} ({})", name, score, grade);
    }
}
```

- izip!은 multizip보다 더 간결하고 직관적
- 반복자뿐 아니라 Vec, 슬라이스도 바로 사용 가능

### 🔑 핵심 요약: 고급 zip 패턴

| 패턴          | 역할 또는 기능           | 관련 크레이트 또는 대상 | 비고 또는 특징                     |
|---------------|--------------------------|--------------------------|------------------------------------|
| `zip().map()` | 두 반복자를 zip 후 연산 수행 | 기본 반복자               | Rust의 `zip_with` 역할             |
| `multizip()`  | 여러 반복자를 튜플로 zip   | `itertools`              | 반복자 기반 zip                    |
| `izip!`       | 매크로 기반 zip 표현       | `Vec`, `slice`, `itertools` | 가장 간결하고 직관적 zip 표현      |

Rust는 반복자 조합이 강력해서 Python 스타일 zip 패턴을 더 안전하고 빠르게 구현할 수 있음.

---

# structure remind

Vec<(&str, i32)>는 참조 기반 튜플이고,  
Vec<(String, i32)>는 소유권을 가진 구조 기반 튜플입니다.  
구조체로 보관하려면 후자가 맞고, 더 나아가면 명시적인 구조체 타입으로 감싸는 게 가장 좋습니다.

## ✅ 튜플 → 구조체로 보관하는 방식
### 1. 튜플 형태
```rust
let pairs: Vec<(String, i32)> = vec![
    ("Alice".to_string(), 85),
    ("Bob".to_string(), 92),
];
```

- String은 heap을 소유하므로 구조체에 넣기 적합
- &str은 참조라서 lifetime 문제가 생길 수 있음

### 2. 구조체 형태
```rust
struct Score {
    name: String,
    value: i32,
}

let pairs: Vec<Score> = vec![
    Score { name: "Alice".into(), value: 85 },
    Score { name: "Bob".into(), value: 92 },
];
```

- 구조체로 감싸면 의미가 명확하고 유지보수가 쉬움
- 필드 이름 덕분에 .name, .value로 접근 가능


### 🔑 핵심 요약: &str vs String

| 타입     | 소유권 여부 | 메모리 위치 | 특징 또는 용도                     |
|----------|--------------|--------------|------------------------------------|
| `&str`   | ❌ 없음       | stack에서 참조 | 고정된 문자열, 빠르지만 lifetime 필요 |
| `String` | ✅ 있음       | heap에 저장   | 가변 문자열, 구조체에 넣기 적합      |


결론적으로 Vec<(String, i32)>는 구조체로 보관할 때의 중간 단계이고,
Vec<Score>처럼 명시적인 구조체로 감싸는 게 가장 Rust다운 방식


