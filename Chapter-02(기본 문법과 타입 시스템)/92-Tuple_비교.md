# tuple `==` 
- Rust의 == 비교가 tuple 내부의 타입이 모두 PartialEq 또는 Eq를 구현할 때만  
    자동으로 동작
- Rust의 tuple 비교는 **사전식(lexicographical)** 비교가 아니라  
    **모든 요소가 각각 == 인지 확인하는 구조적 비교** 이다.

- 즉, (a1, a2, ..., an) == (b1, b2, ..., bn) 은 다음과 같이 평가된다:
    - $(a1==b1)\wedge (a2==b2)\wedge \cdots \wedge (an==bn)$
    - float가 있든 없든, 비교 가능한 타입만 있으면 이 규칙이 그대로 적용된다.

## 🔍 Rust에서 tuple의 기본 == 동작 방식
### 1. 모든 요소가 PartialEq이면 tuple도 자동으로 PartialEq
- Rust는 tuple에 대해 다음과 같은 trait 구현을 자동으로 제공한다:
    - (T1, T2, ..., Tn): PartialEq
- 단, 모든 Ti가 PartialEq일 때만
    - 따라서 (usize, usize) 같은 tuple은 자동으로 비교 가능하다.

### 2. 비교 방식: 구조적(element-wise) 비교
- Rust는 tuple을 비교할 때 각 요소를 순서대로 == 비교한다.
- 예:
```rust
(1usize, 10usize) == (1usize, 10usize)  // true
(1usize, 10usize) == (1usize, 20usize)  // false
(1usize, 10usize) == (2usize, 10usize)  // false
```

- float가 없다고 해서 특별한 최적화나 다른 규칙이 적용되는 것은 아니다.

### 3. float이 있을 때만 특별한 점이 생김
- f32, f64는 NaN 때문에 Eq를 구현하지 못한다.
- 그래서 float이 tuple에 들어가면:
    - tuple은 PartialEq은 가능
    - 하지만 Eq는 불가능
- 예:
```rust
(1usize, 1.0f64) == (1usize, 1.0f64)  // OK
// 하지만 Eq는 구현되지 않음
```

- 반면 float이 없으면 tuple은 Eq도 자동 구현된다.

## ✔ 결론
- float이 없는 tuple에서 Rust의 기본 == 는 다음처럼 동작한다:
  - 모든 요소가 PartialEq → tuple도 PartialEq
  - 모든 요소가 Eq → tuple도 Eq
  - 비교는 각 요소를 순서대로 == 비교
  - 특별한 최적화나 다른 규칙은 없음
- 즉, (usize, usize, ...) 같은 tuple은 완전히 **정상적인 구조적 비교** 만 수행한다.


---


## 1. ==가 먼저 하는 일: 전부 eq 호출로 풀린다
- a == b 는 문법이 아니라 trait 메서드 호출로 완전히 풀리는 문법 설탕.
- a == b
```rust
// 는 컴파일 시에 대략 이렇게 해석된다:
PartialEq::eq(&a, &b)
```

- != 도 마찬가지로:
- a != b
```rust
// 는
PartialEq::ne(&a, &b)
// 기본 구현은 단순히 !self.eq(other) 
```

- 즉, tuple 비교도 결국은 tuple 타입에 대해 구현된 impl PartialEq for (T1, T2, ...)의  
    **eq 메서드 호출**.

## 2. 표준 라이브러리의 tuple PartialEq 구현 형태
- 표준 라이브러리에는 (0~12개 정도까지) 각 길이에 대해 **직접 작성된 impl**들이 들어 있다.
```rust
impl<T1, T2> PartialEq for (T1, T2)
where
    T1: PartialEq,
    T2: PartialEq,
{
    fn eq(&self, other: &(T1, T2)) -> bool {
        self.0 == other.0 && self.1 == other.1
        // 즉, 다시 각 요소의 PartialEq::eq 호출로 풀림
    }
}
```

- 길이가 3이면:
```rust
impl<T1, T2, T3> PartialEq for (T1, T2, T3)
where
    T1: PartialEq,
    T2: PartialEq,
    T3: PartialEq,
{
    fn eq(&self, other: &(T1, T2, T3)) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2 == other.2
    }
}
```

- 여기서 중요한 포인트:
    - 좌변 tuple 타입과 우변 tuple 타입이 동일해야 한다
    - (A, C) 와 (B, D) 처럼 **각 요소끼리는 서로 PartialEq** 이지만,  
        tuple **타입 자체가 다르면** 표준 라이브러리의 현재 구현으로는 PartialEq가 안 붙는다.
    - 그래서 impl<L1, L2, R1, R2> PartialEq<(R1, R2)> for (L1, L2) where L1:  
        PartialEq<R1>, ...  같은 **비대칭** 구현은 아직 없다.

## 3. 컴파일러가 생성하는 실제 비교 코드의 모양
- 예를 들어:
```rust
fn main() {
    let a: (usize, usize) = (1, 10);
    let b: (usize, usize) = (1, 20);

    if a == b {
        println!("same");
    }
}
```

- 이 코드는 개념적으로 다음과 같이 풀린다고 보면 된다:
```rust
fn main() {
    let a: (usize, usize) = (1, 10);
    let b: (usize, usize) = (1, 20);

    // 1단계: == 를 PartialEq::eq 호출로 변환
    if <(usize, usize) as PartialEq>::eq(&a, &b) {
        println!("same");
    }
}
```

- 그리고 <(usize, usize) as PartialEq>::eq 의 실제 본문은 위에서 본 것처럼:
```rust
fn eq(&self, other: &(usize, usize)) -> bool {
    self.0 == other.0 && self.1 == other.1
}
```

- 이 self.0 == other.0 도 다시:
```rust
<usize as PartialEq>::eq(&self.0, &other.0)
```

- 즉, 최종적으로는:
    - usize 두 개를 비교하는 PartialEq::eq 호출 두 번
    - 그 결과를 && 로 묶은 단순한 논리식 으로 컴파일된다.

## 4. float이 없을 때, 추가적인 “최적화”나 특수 규칙은 없다
- **usize 같은 비교 가능한 데이터만 있을 때, 컴파일러가 뭔가 더 특별한 코드를 만들어 주는가?** 
    - 규칙 자체는 전혀 특별하지 않다.
    - 그냥 **각 요소의 PartialEq::eq를 호출하고, 전부 true면 true**.
    - f32, f64가 없다는 이유만으로 다른 알고리즘을 쓰거나, 튜플 전체를 메모리 블록으로 비교하는  
    식의 별도 의미론을 적용하지 않는다.
- 다만 usize·u32 같은 단순 타입의 PartialEq::eq 구현은 매우 단순해서,  
    최적화 단계에서 인라인되고 레지스터 비교 한두 번으로 줄어들 가능성은 크다.
- 이건 **의미론** 이 아니라 **최적화** 영역

## 5. Eq 구현도 마찬가지로 “요소별 AND”
- Eq는 메서드가 없는 marker trait라서, tuple에 대해선 대략 이런 식으로 붙어 있다:
```rust
impl<T1, T2> Eq for (T1, T2)
where
    T1: Eq,
    T2: Eq,
{}
```

- 즉:
- 모든 요소가 Eq면 tuple도 Eq
- Eq 자체는 메서드를 추가로 정의하지 않으므로, 실제 비교 로직은 여전히  
    PartialEq::eq의 **요소별 AND** 그대로다.

## 6. 정리
- a == b → PartialEq::eq(&a, &b) 로 완전히 풀린다.
- tuple에 대한 PartialEq 구현은 각 길이별로 직접 정의되어 있고, self.i == other.i  
    를 &&로 모두 묶는 구조다.
- 각 요소의 == 역시 다시 그 타입의 PartialEq::eq 호출이다.
- float이 없다고 해서 의미론이 바뀌지는 않고, 단지 모든 요소가 Eq라면 tuple도 Eq가 되어  
    **완전한 동치 관계** 를 가진다는 점만 달라진다.
- 실제 생성되는 코드는 **요소별 비교 + 논리 AND** 로 생각하면 된다.

---

## Eq와 PartialEq 차이

- Rust가 Eq와 PartialEq라는 두 개의 다른 이름을 굳이 만든 이유는 단순히 **NaN 때문**  
    이 아니라, 수학적 의미론을 정확하게 모델링하기 위해서 임. 
- 이걸 이해하면 Rust의 타입 시스템이 얼마나 엄격하고 정교한지 훨씬 잘 보인다.

## 🔎 왜 이름이 PartialEq와 Eq인가?
- 핵심은 Rust가 **수학적 동치 관계(equivalence relation)** 를 엄격하게 구분하기 때문이다.
- 수학에서 “동치 관계”는 세 가지 성질을 만족해야 한다:

| 성질        | 의미                     | 예시 표현          |
|-------------|---------------------------|---------------------|
| Reflexive   | 자기 자신과 같다          | a == a              |
| Symmetric   | a == b 이면 b == a        | a == b, b == a      |
| Transitive  | a == b, b == c → a == c   | a == b, b == c, a == c |

- 이 세 가지를 모두 만족하는 비교만 완전한 동치 관계라고 부른다.
- Rust의 Eq는 바로 이 완전한 동치 관계를 의미한다.

### 🧩 그런데 float는 이 조건을 깨버린다
- f32, f64는 IEEE-754 규격을 따르는데, 여기서 NaN은 자기 자신과도 같지 않다.
```rust
assert!(f32::NAN != f32::NAN);
```

- 즉, 반사성(reflexive)이 깨진다.
- 이 순간 float의 ==는 더 이상 **동치 관계** 가 아니다.
- 그래서 Rust는 float에 대해:
    - PartialEq는 구현 가능
    - Eq는 구현 불가능
- 이렇게 구분한다.

### 🧠 그래서 이름이 이렇게 붙었다
- ✔ PartialEq
    **부분적(partial) 동치 관계** 를 의미한다.
    - 반사성, 대칭성, 추이성이 보장되지 않을 수 있다
    - 비교는 가능하지만, 완전한 동치 관계가 아니다
    - float처럼 NaN이 있는 타입이 여기에 해당
- 즉, **비교는 되지만, 수학적으로 완전한 동치가 아니다** 라는 뜻.

- ✔ Eq
- **완전한(equivalence) 동치 관계** 를 의미한다.
    - 반사성, 대칭성, 추이성이 모두 성립해야 한다
    - Eq는 메서드가 없는 marker trait
        - 단지 **이 타입은 완전한 동치 관계를 가진다** 는 선언
- usize, u32, bool, char, String, 그리고 이런 것들로 구성된 tuple 등이 여기에 해당한다.

### 📌 왜 굳이 이렇게 엄격하게 구분할까?
- Rust는 타입 시스템을 통해 논리적 오류를 방지하려고 한다.
    - 예를 들어, 해시맵의 key는 반드시 Eq를 만족해야 한다.
    - 왜냐하면 해시맵은 key 비교가 완전한 동치 관계라는 전제 위에서 동작하기 때문.
- 만약 float을 key로 허용하면?
    - NaN != NaN 때문에 같은 key가 두 개 생기는 이상한 상황이 발생
    - 해시맵의 불변식이 깨짐
    - 논리적 버그가 생김
- 그래서 Rust는 float을 Eq로 만들지 않는다. 이건 “안전성”을 위한 매우 중요한 설계.

### 🎯 결론
- Rust가 PartialEq와 Eq라는 두 이름을 만든 이유는:
    - 수학적 동치 관계를 엄격하게 모델링하기 위해
    - float처럼 반사성이 깨지는 타입을 구분하기 위해
    - 해시맵 같은 자료구조의 논리적 안전성을 보장하기 위해
- 즉, 동치 관계의 성질을 타입 시스템에 반영한 매우 의도적인 설계라고 보면 된다.

---

