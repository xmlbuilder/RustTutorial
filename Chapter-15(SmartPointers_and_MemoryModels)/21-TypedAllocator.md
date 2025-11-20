# TypedAllocator 테스트 코드 설명 및 실무적 활용 방안

이 테스트 코드의 용도는 Rust에서 커스텀 메모리 할당기인 TypedAllocator<T>가 정상적으로 객체를 할당하고 관리하는지 검증하는 것입니다.  
아래에 핵심 목적과 동작 원리를 정리.

## 테스트 코드

```rust
#[cfg(test)]
mod type_allocator_test {
    use geometry::core::typed_allocator::TypedAllocator;

    #[derive(Debug)]
    struct MyStruct {
        x: u32,
        y: u64,
    }

    #[test]
    fn test_type_alloc_basic() {
        let allocator = TypedAllocator::<MyStruct>::new(1024);
        let mut objects = Vec::new();
        for i in 0..5 {
            let obj = allocator.alloc(MyStruct {
                x: i,
                y: i as u64 * 10,
            });
            println!("Allocated {}: {:?}", i, obj);
            objects.push(obj); // Drop 은 Vec 이 소멸될 때 자동 호출됨
        }
        println!("All objects allocated.");
    }
}
```

## 🧪 테스트 목적
- TypedAllocator<MyStruct>를 사용해 MyStruct 타입의 객체를 메모리에 할당
- 할당된 객체들이 올바르게 생성되고, Drop 시 자동 해제되는지 확인
- 메모리 풀 크기(1024) 내에서 여러 객체를 할당해도 문제가 없는지 테스트

## 🔧 핵심 동작 흐름
- 할당기 생성
```rust
let allocator = TypedAllocator::<MyStruct>::new(1024);
```
- TypedAllocator는 내부적으로 고정 크기의 메모리 풀을 생성
- MyStruct 타입 전용으로 작동
- 객체 할당 반복
```rust
for i in 0..5 {
    let obj = allocator.alloc(MyStruct { x: i, y: i as u64 * 10 });
    objects.push(obj);
}
```
- alloc()을 통해 MyStruct 인스턴스를 할당
- 할당된 객체는 Vec에 저장되어 Drop 시 자동 해제됨
- 디버깅 출력

```rust
println!("Allocated {}: {:?}", i, obj);
```

- 할당된 객체의 내용을 출력하여 확인

## 📦 TypedAllocator의 일반적인 용도
- 성능 최적화: 반복적으로 같은 타입을 할당할 때 힙 대신 풀 메모리 사용
- 메모리 제어: 커스텀 할당 전략을 통해 메모리 사용량 예측 가능
- 게임/시뮬레이션: 대량의 구조체를 빠르게 생성/삭제해야 하는 경우

## ✅ 테스트 결과 기대
- 콘솔에 5개의 MyStruct 객체가 정상적으로 출력됨
- 메모리 누수 없이 Vec이 drop될 때 객체도 해제됨
- TypedAllocator가 타입 안정성과 메모리 안전성을 유지함

---

## ✅ 실무에서 쓰일 수 있는 경우
### 1. 고성능 시뮬레이션 / 게임 엔진
- 수천~수백만 개의 객체를 빠르게 생성/삭제해야 할 때
- 예: 물리 시뮬레이션에서 입자, 충돌체, 메시 노드 등
- TypedAllocator는 힙 할당보다 빠르고 캐시 친화적
### 2. 실시간 시스템 / 임베디드 환경
- 메모리 할당/해제 비용을 예측 가능하게 만들고 싶을 때
- 예: 항공/자동차 제어 시스템, 로봇 제어 루프
### 3. 메모리 풀 기반 구조 설계
- 특정 타입만 반복적으로 할당할 때 메모리 낭비 줄이기
- 예: 네트워크 패킷 구조체, 로그 버퍼, 이벤트 큐
### 4. 커스텀 메모리 관리가 필요한 라이브러리
- 예: ECS(Entity Component System), 그래픽 렌더러, DB 엔진

### ❌ 일반적인 실무에서는 잘 안 쓰이는 경우
- 웹 백엔드, 모바일 앱, 데스크탑 GUI 등에서는 대부분 Box, Vec, Rc, Arc 등 표준 할당기로 충분합니다.
- Rust의 안전한 메모리 모델 덕분에 커스텀 할당기를 직접 쓸 필요가 줄어듦

## 🔍 Rust에서 실무적으로 더 자주 쓰이는 메모리 전략

| 전략                          | 설명                                                                 |
|-------------------------------|----------------------------------------------------------------------|
| `Vec<T>`                      | 가장 일반적인 동적 배열. 자동 확장, 안전하고 빠름.                   |
| `Box<T>`                      | 힙에 단일 객체를 저장. 트리 구조나 재귀 타입에 자주 사용됨.         |
| `Rc<T>`, `Arc<T>`             | 참조 카운팅 기반 공유. `Rc`는 단일 스레드, `Arc`는 멀티 스레드용.   |
| `Arena`, `Bump` (`typed-arena`, `bumpalo`) | 대량 객체를 빠르게 할당/해제. 트리, 파서, ECS 등에 적합. |
| `slab`, `pool`                | 고정 크기 객체를 반복적으로 할당. 네트워크, 이벤트 큐 등에 사용.     |


## ✍️ 결론
- TypedAllocator<T>는 특정 성능 요구가 있는 시스템에서 실무적으로 유용합니다.
- 하지만 일반적인 Rust 개발에서는 표준 할당기로 충분한 경우가 많습니다.
- 실무에서 쓸 일이 있다면, 보통은 성능 최적화, 메모리 예측성, 또는 GC 없는 환경이 핵심 이유입니다.

---

## 실무 적용 

아래는 TypedAllocator<T> 또는 유사한 메모리 풀 전략이 실무에서 실제로 쓰일 수 있는 Rust 기반 실전 예제입니다.  
상황은 게임 엔진의 물리 시뮬레이션에서 수천 개의 입자를 빠르게 생성/삭제하는 경우입니다.

## 🎮 실전 예제: 입자 시뮬레이션에서 메모리 풀 할당기 사용
### 📌 시나리오
- 매 프레임마다 수천 개의 입자를 생성하고, 일정 시간 후 제거됨
- 일반적인 Box나 Vec 기반 할당은 성능 저하 발생
- TypedAllocator<Particle>를 사용해 빠른 할당/해제 구현

### 🧱 구조체 정의
```rust
#[derive(Debug)]
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    lifetime: f32,
}
```

### 🧠 커스텀 할당기 사용 예시
```rust
use geometry::core::typed_allocator::TypedAllocator;

fn simulate_particles() {
    let mut allocator = TypedAllocator::<Particle>::new(10_000); // 최대 1만 개 입자

    let mut active_particles = Vec::new();

    for i in 0..5000 {
        let p = allocator.alloc(Particle {
            position: [i as f32 * 0.01, 0.0, 0.0],
            velocity: [0.0, 1.0, 0.0],
            lifetime: 3.0,
        });
        active_particles.push(p);
    }

    println!("Spawned {} particles", active_particles.len());
}
```

## ✅ 실무적 장점

| 항목             | 설명                                                                 |
|------------------|----------------------------------------------------------------------|
| 성능 향상         | 힙 할당보다 빠른 속도로 객체를 생성/해제할 수 있음                   |
| 메모리 예측성     | 할당/해제 비용이 일정하여 실시간 시스템에 적합                        |
| 캐시 효율성       | 연속된 메모리 블록을 사용하므로 CPU 캐시 친화적                       |
| 타입 안정성       | 특정 타입 전용으로 설계되어 런타임 오류 없이 안전하게 사용 가능        |
| Drop 자동 관리    | Rust의 소유권 시스템과 결합되어 메모리 누수 없이 안전하게 해제됨       |


## 🧪 확장 아이디어
- TypedAllocator<Enemy> → 적 객체 풀
- TypedAllocator<Bullet> → 총알 객체 풀
- TypedAllocator<Node> → 그래프 노드, 트리 구조 등

---


## 🚀 일반 for 루프 vs 커스텀 할당기 성능 비교

| 항목             | `for` + `Box::new()`                         | `TypedAllocator` 방식                                      |
|------------------|----------------------------------------------|------------------------------------------------------------|
| 할당 속도         | 느림 – 매번 힙에 새로 할당                   | 빠름 – 미리 확보된 메모리 풀에서 슬라이스처럼 할당         |
| 해제 비용         | 개별 Drop 호출 필요                          | Vec 등 컨테이너 Drop 시 일괄 해제                          |
| 메모리 단편화     | 발생 가능                                     | 거의 없음 (연속 메모리 사용)                              |
| 캐시 효율성       | 낮음 – 산발적 메모리 접근                    | 높음 – 연속된 메모리 블록 활용                            |
| 예측 가능성       | 낮음 – 할당/해제 비용이 상황에 따라 다름     | 높음 – 고정된 풀에서 일정한 비용으로 처리 가능             |


## 🧪 예시: 10만 개 객체 생성
```rust
// 일반 방식
let mut v = Vec::new();
for _ in 0..100_000 {
    v.push(Box::new(MyStruct { x: 1, y: 2 }));
}
```
```rust
// 커스텀 할당기 방식
let allocator = TypedAllocator::<MyStruct>::new(100_000);
let mut v = Vec::new();
for _ in 0..100_000 {
    v.push(allocator.alloc(MyStruct { x: 1, y: 2 }));
}
```

- 일반 방식: 매번 힙에 새로 할당 → 할당기 호출 + 메모리 단편화 발생
- 풀 방식: 미리 확보한 메모리에서 슬라이스처럼 빠르게 할당

## 📌 실무에서 “훨씬 빠른” 조건
- 할당/해제가 매우 빈번할 때
- 객체 수가 수천~수백만 개일 때
- 실시간 처리 요구 (게임, 시뮬레이션, 제어 시스템 등)
- GC 없는 환경에서 메모리 예측이 중요할 때

## ❗ 단, 주의할 점
- 풀 크기를 초과하면 panic 또는 undefined behavior 가능
- 일반적인 앱에서는 오히려 복잡도만 증가할 수 있음
- Rust의 표준 할당기는 대부분의 경우 충분히 빠르고 안전함

---

## 🧠 TypedAllocator<T> 내부 구현 원리
### 1. 📦 목적
- 특정 타입 T에 대해 반복적인 할당/해제를 빠르게 처리
- 힙 대신 미리 확보된 연속 메모리 블록을 사용
- Rust의 타입 시스템과 Drop을 활용해 안전한 메모리 관리

### 2. 🧱 내부 구조
```rust
struct TypedAllocator<T> {
    buffer: Box<[MaybeUninit<T>]>, // 고정 크기 메모리 풀
    next: usize,                   // 다음 할당 위치
    capacity: usize,               // 총 풀 크기
}
```

- MaybeUninit<T>: 초기화되지 않은 메모리 공간을 안전하게 다루기 위한 Rust 표준 타입
- Box<[MaybeUninit<T>]>: 힙에 연속된 메모리 블록을 미리 확보
- next: 현재 할당 위치를 추적

### 3. ⚙️ 할당 방식
```rust
fn alloc(&mut self, value: T) -> &mut T {
    assert!(self.next < self.capacity);
    let slot = &mut self.buffer[self.next];
    slot.write(value); // 안전하게 초기화
    self.next += 1;
    unsafe { slot.assume_init_mut() } // 초기화된 참조 반환
}
```

- write(value): MaybeUninit에 안전하게 값을 채움
- assume_init_mut(): 초기화된 참조를 안전하게 꺼냄
- 할당은 O(1) → 매우 빠름

### 4. 🧹 해제 방식
- Rust의 Drop은 Vec<&mut T> 같은 컨테이너가 소멸될 때 자동 호출됨
- 별도의 free() 호출 없이도 안전하게 메모리 해제 가능
- 필요 시 reset() 메서드로 풀을 초기화할 수 있음

### 5. ✅ 안전성 보장
- MaybeUninit<T>를 통해 초기화 여부를 명확히 구분
- alloc()에서만 초기화 → 외부에서 직접 접근 불가
- Rust의 소유권과 생명주기 시스템으로 메모리 누수 방지

### 📌 요약 흐름
```
[풀 생성] → [T 타입 메모리 블록 확보]
       ↓
[alloc()] → 초기화된 T 객체 반환
       ↓
[사용 후 Drop] → 자동 해제
```


### 🔍 실무에서의 장점
- 빠른 할당/해제 (O(1))
- 메모리 단편화 없음
- 캐시 친화적
- 타입 안정성 확보
