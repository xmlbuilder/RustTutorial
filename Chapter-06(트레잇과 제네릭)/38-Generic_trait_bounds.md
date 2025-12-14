## Generic trait bounds
- 제네릭 트레이트 바운드는 **이 타입이 최소한 어떤 능력을 가져야 한다**를 선언, 
- 제네릭 코드가 안전하고 효율적으로 동작하도록 보장합니다. 
- 단순한 문법 이상의 의미가 있고, 타입 시스템·트레이트 시스템·모노모픽 컴파일 전략과 깊게 연결됩니다.

### Core concepts
- 제약의 목적:
    - 제네릭으로 **아무 타입이나** 받으면 내부에서 할 수 있는 일이 없습니다. 
    - 트레이트 바운드로 타입이 제공해야 하는 연산(복사, 비교, 포맷, 수학 등)을 명시. 
    - 컴파일 타임에 검증하고 최적화된 코드 생성이 가능해집니다.
- 선언 위치:
    - 인라인 제약: `fn f<T: Copy + Debug>(x: T) { ... }`
    - where 절: `fn f<T>(x: T) where T: Copy + Debug { ... }`
    - 긴 제약이나 여러 제약을 가독성 좋게 정리할 때 유용합니다.
- 모노모픽 코드 생성:
    - 제네릭 함수는 사용 시점마다 타입별로 특수화되어 별도의 머신 코드가 생성됩니다. 
    - 바운드가 정확할수록 최적화(인라인, 제거 등)에 유리합니다.

### Bound varieties and patterns
- 기본 트레이트 바운드:
    - Copy/Clone: 값 복제 가능. 소유권 제약을 단순화합니다.
    - PartialEq/Eq, PartialOrd/Ord: 비교 가능. 정렬, 검색, 조건 분기 등에 필요합니다.
    - Debug/Display: 포맷 출력 가능. 로깅과 에러 메시지에서 필수적입니다.
    - Hash: 해시 가능. 해시맵 키로 쓰려면 Eq + Hash가 필요합니다.
- 연산자 관련 트레이트:
    - Add/Sub/Mul/Div/Rem: 산술 연산 지원.
    - AddAssign 등 할당 연산: 누적 갱신 패턴에서 유용합니다.
- Output 연관 타입: Add<Output = T>처럼 결과 타입을 동일하게 강제할 수 있습니다.
- 연관 타입(associated types):
    - 트레이트가 타입을 내장해 제공하는 경우 그 타입에도 제약을 겁니다.
```rust
trait IntoIter { type Item; fn next(&mut self) -> Option<Self::Item>; }
fn consume<I>(mut it: I)
where
    I: IntoIter,
    I::Item: Debug,
{ /* ... */ }
```
- 슈퍼트레이트(supertraits):
    - 트레이트 A가 B를 요구하면 trait A: B {}로 표현합니다. 
    - A를 바운드로 쓰면 자동으로 B도 필요해집니다.
- trait Pretty: Display {}
```rust
fn show<T: Pretty>(x: T) { /* Display도 자동 포함 */ }
```
- 자동 트레이트(auto traits):
    - Send, Sync, Unpin 등은 구현 여부가 구조적으로 결정됩니다. 
    - 동시성 경계에서 T: Send + Sync는 매우 흔한 제약입니다.
- 라이프타임과 HRTB:
    - 라이프타임 바운드: T: 'a는 T가 'a보다 오래 산다는 뜻. 
    - 참조를 담는 제네릭 타입에서 중요합니다.
    - HRTB (Higher-Ranked Trait Bounds): 모든 라이프타임에 대해 성립해야 할 때 사용합니다.
```rust
fn call_with_str<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{ /* 'a가 어떤 라이프타임이어도 가능 */ }
```
- 인터페이스 합성:
    - 여러 트레이트를 조합해 풍부한 계약을 만듭니다.
```rust
fn process<T>(x: T)
where
    T: Copy + Hash + Eq + Debug + Send,
{ /* 분산 캐시 키로도 안전하게 사용 */ }
```

### Where to apply bounds
- 함수/메서드 시그니처: 가장 흔한 곳입니다.
```rust
fn min<T>(a: T, b: T) -> T
    where T: PartialOrd
{ if a < b { a } else { b } }
```
- impl 블록: 타입 전체의 기능을 특정 조건에서만 제공.
```rust
impl<T> MyVec<T>
where T: Clone
{
    fn duplicate(&self) -> Self { /* ... */ }
}
```
- 제네릭 타입 정의: 타입 파라미터 자체의 계약을 명확히 함.
```rust
struct Wrapper<T: Debug>(T);
```
- 트레이트 구현의 조건부 제공(blanket/conditional impl):
```rust
impl<T> From<Vec<T>> for MyList<T>
    where T: Clone
{ /* Vec<T>를 MyList<T>로 변환 */ }
```

### Advanced topics and caveats
- Coherence와 Orphan rule:
    - 당신이 소유하지 않은 타입과 당신이 소유하지 않은 트레이트의 조합은 임플을 추가할 수 없습니다. 
    - 라이브러리 간 충돌을 방지하기 위한 규칙입니다.
- Specialization(부분 특수화):
    - 특정 타입에 대해 더 구체적인 구현을 제공하는 기능은 아직 안정화되지 않았습니다. 
    - 현재는 제약과 수동 분기로 대체합니다.
- Negative bounds(부정 바운드):
    - **T가 X를 구현하지 않아야 한다** 같은 제약은 지원되지 않습니다. 
    - 보통 API 설계로 회피합니다.
- Bounds 최소화:
    - 불필요한 바운드는 제약을 과도하게 만들고 재사용성을 떨어뜨립니다. 
    - 필요한 연산만 정확히 요구. 
    - 예를 들어 “읽기만” 필요하면 Clone 대신 Borrow나 참조로 충분할 수 있습니다.
- 성능 관점:
    - 바운드로 인해 제네릭이 모노모픽 컴파일되어 인라이닝과 제거 최적화가 가능해집니다. 
    - 반면 트레이트 객체(dyn Trait)는 런타임 디스패치를 사용하므로 유연하지만 약간의 오버헤드가 있습니다. 
- Const generics와의 결합:
    - 크기나 고정 파라미터를 타입 수준으로 올릴 때도 바운드를 활용합니다.
```rust
fn sum<const N: usize, T>(arr: [T; N]) -> T
where T: Copy + Default + Add<Output = T>
{ /* ... */ }
```

- Practical guidance
- 필요한 연산만 요구:
    - 출력만 필요 → Display 또는 Debug
    - 비교만 필요 → PartialOrd
    - 해시맵 키 → Eq + Hash
    - 수학 연산 → Add/Sub/...와 결과 타입 제약(Output = T)
    - 부동소수점 → num_traits::Float 같은 도메인 트레이트
- 도메인 트레이트 만들기:
    - 자주 쓰는 제약을 프로젝트 트레이트로 묶어 가독성과 일관성을 유지.
```rust
trait Scalar: Copy + PartialOrd + Add<Output=Self> + Sub<Output=Self> {}
impl<T> Scalar for T where T: Copy + PartialOrd + Add<Output=Self> + Sub<Output=Self> {}
```
- 테스트로 계약 검증:
    - 경계 조건(정렬, 비교, 산술)이 의도대로 동작하는지 파라미터화를 통해 확인.

---

