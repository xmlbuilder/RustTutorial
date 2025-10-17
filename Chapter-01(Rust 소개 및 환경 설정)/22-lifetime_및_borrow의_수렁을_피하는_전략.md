# lifetime 및 borrow의 수렁을 피하는 전략

## 1) 수렁을 피하는 대원칙 7

- 동사만 먼저, 타입/라이프타임은 나중
- 시그니처를 일단 **값 중심** 으로 씀. 참조는 최대한 미루고, 가능하면 Clone + Cow/Arc로 시작합니다.
- 모든 부수효과는 Ctx 하나로 트레이트는 &self(불변) + &mut Ctx만 받게 하면 lifetime 전파가 거의 없습니다.
  
```rust
trait Step { fn run(&self, input: &Input, ctx: &mut Ctx) -> Result<Output>; }
```

### Trait Object(런타임 다형)부터 시작
- 제네릭로 시작하면 lifetime/타입 전파가 폭발.

#### 초판은:
  
```rust
type BoxStep = Box<dyn Step>; // or Rc<dyn Step>
struct Pipeline { steps: Vec<BoxStep> }
```
- 성능 병목이 보일 때만 제네릭로 올립니다.

### ID/핸들 패턴 (Arena/ECS식)
- 구조체 참조를 트레이트에 넣지 말고, ID로 참조하고 실제 데이터는 Ctx/아레나가 소유:
```rust
struct MeshId(u32);
struct Ctx { meshes: Arena<Mesh>, /* ... */ }
trait Op { fn apply(&self, id: MeshId, ctx: &mut Ctx); } // ← 수명 프리
```

### 객체 안전(Object-safe) 규칙을 지켜라
- 트레이트 오브젝트로 쓸 거면: 제네릭 메서드/Self: Sized/연관 상수 남발 금지.
- 필요하면 “두 개의 트레이트”로 나눠서, 바깥은 오브젝트-세이프, 안쪽은 제네릭로.
- 콜백은 HRTB로 받기 (수명 오염 방지)

```rust
fn visit<F>(&self, f: F) where for<'a> F: FnMut(&'a Node) { /* ... */ }
```
- 이렇게 하면 호출자 수명이 구현으로 새어들지 않아요.
- 초기에는 ‘소유’를 두껍게 &[T]보다 Arc<[T]> / Cow<'_, T> / SmallVec 등을 써서 설계 확정 전까지 수명 의존을 줄입니다.

## 2) 나쁜/좋은 트레이트 시그니처 비교 (수명 폭탄 방지)

### ❌ 나쁨: 수명을 외부로 새게 만듦
```rust
trait Intersect<'a> {
    fn hit(&'a self, a: &'a Mesh, b: &'a Mesh) -> bool; // 'a가 전체를 지배
}
```

### ✅ 좋음: 컨텍스트에 소유/캐시 모음, 트레이트는 얇게
```rust
struct Ctx<'s> { meshes: &'s Arena<Mesh>, scratch: Scratch, /* ... */ }
trait Intersect {
    fn hit(&self, a: MeshId, b: MeshId, ctx: &mut Ctx) -> bool;
}
```
- 외부 수명이 트레이트 시그니처에 등장하지 않으니 트레이트는 ‘수명 중립’.
- 실제 참조는 ctx.meshes.get(a) 안에서만 잠깐 빌려 씀.

## 3) **초판은 런타임**, 병목만 제네릭 단계적 업그레이드
### 1단계: 빠른 조립 (런타임 다형)
```rust
trait Step { fn run(&self, input: &Input, ctx: &mut Ctx) -> Output; }
struct Pipeline { steps: Vec<Box<dyn Step>>; }
```
### 2단계: 성능 민감 구간만 제네릭로
```rust
trait StepImpl { fn run_impl(&self, input: &Input, ctx: &mut Ctx) -> Output; }
struct Pipeline<T: StepImpl> { steps: Vec<T> } // 핵심 루프만 monomorphization
```
### 3단계: 하이브리드
- 상위 조립은 dyn Step, 내부 뜨거운 루프는 T: Kernel로 분리.

## 4) 수명 줄이는 6가지 패턴 (코드 조각)

### (a) 입력은 값/경량복사
```rust
#[derive(Clone)]
struct Command { name: Arc<str>, args: Arc<[u8]> }
trait Handle { fn handle(&self, cmd: Command, ctx: &mut Ctx); }
```

### (b) GAT로 이터레이터/뷰 노출 (필요할 때만)
```rust
trait Scene {
    type Iter<'a>: Iterator<Item = &'a Node> where Self: 'a;
    fn nodes(&self) -> Self::Iter<'_>;
}
```

### (c) 빌더로 ‘구성’과 ‘수행’ 분리
```rust
struct PipelineBuilder { steps: Vec<Box<dyn Step>> }
impl PipelineBuilder {
    fn add(mut self, s: impl Step + 'static) -> Self { self.steps.push(Box::new(s)); self }
    fn build(self) -> Pipeline { Pipeline { steps: self.steps } }
}
```

### (d) 에러 정책은 enum으로 중앙집중
```rust
enum Policy { Strict, BestEffort }
struct Ctx { policy: Policy, errors: Vec<anyhow::Error> }
```

### (e) 스케줄/이벤트 파이프는 ‘데이터→행위’ 분리
```rust
enum Event { Mouse{..}, Key{..} }
trait Listener { fn on(&self, ev: &Event, ctx: &mut Ctx); }
```

### (f) 어댑터/새장(Sealed)로 외부 확장 제어
```rust
mod sealed { pub trait Sealed {} }
pub trait Bounded: sealed::Sealed { fn bbox(&self) -> Aabb; }
```

## 5) “되돌리기 쉬운” 안전장치
- Facade 유지: 기존 API는 얇은 래퍼로 남겨두고 내부만 바꿔치기. 실패해도 외부 영향 최소화.
- Feature flag: cargo feature="behavior_pipeline"로 새/구 동작 전환.
- Contract Test 세트: 예전/새 구현에 같은 테스트 벡터를 돌려 비교(스냅샷 테스트 추천).
- Migration 단계 문서: public 변경점/대체법/폐기기한을 README에 명시.

## 6) 커밋 전 10문 체크리스트

- 트레이트 메서드에 수명 파라미터가 꼭 필요한가? (대부분 아니어야 함)
- 트레이트가 객체 안전인가? (dyn로 쓸 수 있는가)
- 부수효과는 전부 Ctx로만 흐르는가?
- 입력/출력은 값/경량 소유로 가능한가?
- 제네릭 채택 이유가 성능 근거가 있는가?
- 어댑터/파이프라인으로 구성이 테스트 가능한가?
- 실패/로그/메트릭이 Ctx에 일원화되어 있는가?
- 교체 가능성(전략 변경) 을 고려해 트레이트가 충분히 작고 단일 책임인가?
- 작은 합성으로 큰 행동을 만들 수 있는가?
- 계측 포인트(시간/할당/카운터)가 있는가?

## 7) 미니 예시: 공간 인덱스(데이터 중심 → 행위 중심)

### 데이터 중심(OOP식)
```rust
struct Object { bbox: Aabb, kind: Kind /* ... */ }
fn insert(tree: &mut Tree, o: &Object) { /* kind별 분기 */ }
```

### 행위 중심
```rust
trait Bounded { fn bbox(&self) -> HasAABB; }
struct Spatial { items: Vec<Box<dyn Bounded>>; }
impl Spatial {
    fn insert(&mut self, b: impl Bounded + 'static) { self.items.push(Box::new(b)); }
    fn query(&self, q: HasAABB) -> impl Iterator<Item=&dyn Bounded> {
        self.items.iter().map(|b| &**b).filter(move |b| b.bbox().intersects(q))
    }
}
```
- 수명 없음, 트레이트 얇음, 조립 쉬움.
- 나중에 성능 필요 시 Spatial<T: Bounded>로 확장.


--- 

# 🧩 프로젝트 이름: collide

## 목표: 
다양한 객체를 공간에 삽입하고, 충돌 여부를 검사하는 시스템을  
행위 중심 트레이트 + 컨텍스트 기반으로 설계


## 🧱 1. 핵심 개념 정리
| 개념      | 설명 또는 핵심 메서드                      |
|-----------|--------------------------------------------|
| `Bounded` | `bbox()` 메서드를 통해 공간 영역을 제공     |
| `Spatial` | `Bounded` 객체를 삽입하고 쿼리하는 인덱스   |
| `Ctx`     | 모든 상태, 캐시, 로그, 충돌 결과를 담는 컨텍스트 |
| `MeshId`  | `Ctx` 내부의 `Arena<Mesh>`에서 참조하는 핸들 |
| `Step`    | `run(input, ctx)` 메서드를 가진 파이프라인 단계 |


## 📦 프로젝트 구조
```
collide/
├── src/
│   ├── main.rs
│   ├── ctx.rs
│   ├── traits/
│   │   ├── bounded.rs
│   │   ├── step.rs
│   ├── impls/
│   │   ├── mesh.rs
│   │   ├── spatial.rs
│   │   └── steps/
│   │       ├── logger.rs
│   │       ├── inserter.rs
│   │       └── collider.rs
│   └── pipeline.rs
├── Cargo.toml
```


## 🧪 2. 기본 타입 정의
- src/ctx.rs
```rust
use generational_arena::Arena;
use crate::impls::mesh::Mesh;

#[derive(Default)]
pub struct Ctx {
    pub meshes: Arena<Mesh>,
    pub logs: Vec<String>,
    pub collisions: Vec<(u32, u32)>,
}
```

- src/traits/bounded.rs
```rust
use crate::impls::mesh::Aabb;

pub trait Bounded {
    fn bbox(&self) -> Aabb;
}
```

- src/traits/step.rs
```rust
use crate::ctx::Ctx;

pub trait Step {
    fn run(&self, input: &str, ctx: &mut Ctx);
}
```


##🧪 3. 객체 정의 및 트레이트 구현
- src/impls/mesh.rs
```rust
#[derive(Clone)]
pub struct Aabb {
    pub min: [f32; 2],
    pub max: [f32; 2],
}

impl Aabb {
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.min[0] <= other.max[0] &&
        self.max[0] >= other.min[0] &&
        self.min[1] <= other.max[1] &&
        self.max[1] >= other.min[1]
    }
}

pub struct Mesh {
    pub id: u32,
    pub bbox: Aabb,
}

impl crate::traits::bounded::Bounded for Mesh {
    fn bbox(&self) -> Aabb {
        self.bbox.clone()
    }
}
```


## 🧪 4. 공간 인덱스
- src/impls/spatial.rs
```rust
use crate::traits::bounded::Bounded;

pub struct Spatial {
    pub items: Vec<Box<dyn Bounded>>,
}

impl Spatial {
    pub fn insert(&mut self, b: impl Bounded + 'static) {
        self.items.push(Box::new(b));
    }

    pub fn query(&self, q: &dyn Bounded) -> impl Iterator<Item = &dyn Bounded> {
        let target = q.bbox();
        self.items.iter().map(|b| &**b).filter(move |b| b.bbox().intersects(&target))
    }
}
```


## 🧪 5. 파이프라인 단계 구현
- src/impls/steps/logger.rs
```rust
use crate::traits::step::Step;
use crate::ctx::Ctx;

pub struct Logger;
impl Step for Logger {
    fn run(&self, input: &str, ctx: &mut Ctx) {
        ctx.logs.push(format!("Received: {}", input));
    }
}
```

- src/impls/steps/inserter.rs
```rust
use crate::traits::step::Step;
use crate::ctx::Ctx;
use crate::impls::mesh::{Mesh, Aabb};

pub struct Inserter;
impl Step for Inserter {
    fn run(&self, input: &str, ctx: &mut Ctx) {
        let id = ctx.meshes.insert(Mesh {
            id: ctx.meshes.len() as u32,
            bbox: Aabb { min: [0.0, 0.0], max: [1.0, 1.0] },
        });
        ctx.logs.push(format!("Inserted mesh {}", id.index()));
    }
}
```

- src/impls/steps/collider.rs
```rust
use crate::traits::step::Step;
use crate::ctx::Ctx;

pub struct Collider;
impl Step for Collider {
    fn run(&self, _input: &str, ctx: &mut Ctx) {
        let ids: Vec<_> = ctx.meshes.iter().map(|(id, m)| (id.index(), m.bbox.clone())).collect();
        for (i, a) in ids.iter() {
            for (j, b) in ids.iter() {
                if i < j && a.intersects(b) {
                    ctx.collisions.push((*i, *j));
                }
            }
        }
    }
}
```


## 🧪 6. 파이프라인 조립
- src/pipeline.rs
```rust
use crate::traits::step::Step;
use crate::ctx::Ctx;

pub struct Pipeline {
    pub steps: Vec<Box<dyn Step>>,
}

impl Pipeline {
    pub fn run(&self, input: &str, ctx: &mut Ctx) {
        for step in &self.steps {
            step.run(input, ctx);
        }
    }
}
```


## 🧪 7. 실행 예시
- src/main.rs
```rust
mod ctx;
mod traits {
    pub mod bounded;
    pub mod step;
}
mod impls {
    pub mod mesh;
    pub mod spatial;
    pub mod steps {
        pub mod logger;
        pub mod inserter;
        pub mod collider;
    }
}
mod pipeline;

use crate::impls::steps::*;
use crate::pipeline::Pipeline;
use crate::ctx::Ctx;

fn main() {
    let pipeline = Pipeline {
        steps: vec![
            Box::new(logger::Logger),
            Box::new(inserter::Inserter),
            Box::new(collider::Collider),
        ],
    };

    let mut ctx = Ctx::default();
    pipeline.run("spawn", &mut ctx);

    println!("Logs: {:?}", ctx.logs);
    println!("Collisions: {:?}", ctx.collisions);
}
```


## ✅ 이 프로젝트가 실전 연습에 좋은 이유
| 항목                        | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| `Step`, `Bounded`, `Intersect` | 동사 중심 트레이트로 행위 분리. 각 기능은 작고 단일 책임을 가짐         |
| `&self`, `&mut Ctx`, `input: &str` | 수명 전파 최소화. 트레이트는 얇고 수명 중립. 테스트/조합이 쉬움         |
| `Ctx`                        | 모든 부수효과(로그, 충돌, 캐시 등)를 집중 관리. 상태 추적이 명확함       |
| `Box<dyn Step>`             | 런타임 조립 가능. 빠른 프로토타이핑과 유연한 구성 실험에 적합             |
| `MeshId`, `Arena`           | 참조 대신 ID 사용. 수명 폭탄 방지 + 구조체 간 결합도 낮춤                |
| `Pipeline`                  | 조립부와 실행부 분리. 순서 변경, 조건 분기, 테스트가 쉬운 구조            |
| `Pipeline<T: StepImpl>`     | 성능 병목 구간만 제네릭으로 확장 가능. 런타임 → 정적 디스패치 전환 유연   |

---

# 🧠 구조 분석

```rust
impl Iterator<Item = &dyn Bounded>
```
- impl Iterator<…>: 이터레이터를 반환하는 구현체 타입
- Item = &dyn Bounded: 이 이터레이터가 반환하는 항목의 타입은 &dyn Bounded
- 즉, Bounded 트레이트를 구현한 객체에 대한 불변 참조

## ✅ 의미 요약
“Bounded 트레이트를 구현한 객체들을 참조하는 이터레이터를 반환한다”  
예를 들어, 공간 인덱스에서 query() 메서드가 이런 타입을 반환한다면:  
```rust
fn query(&self, q: Aabb) -> impl Iterator<Item = &dyn Bounded>
```

- 이건 q와 충돌하는 모든 객체를 반복하면서
- 각 객체를 &dyn Bounded로 반환해줌

## 🧪 예시
```rust
let spatial = Spatial::new();
spatial.insert(Box::new(MyObject));
spatial.insert(Box::new(OtherObject));

for obj in spatial.query(some_bbox) {
    println!("{:?}", obj.bbox());
}
```
- obj는 &dyn Bounded
- bbox()는 Bounded 트레이트의 메서드

## ✨ 장점
| 항목                        | 설명                                                                 |
|-----------------------------|----------------------------------------------------------------------|
| `Bounded`                   | 공간 정보를 추상화하는 트레이트. 다양한 타입을 동일 인터페이스로 처리 가능 |
| `&dyn Bounded`              | 객체 안전한 참조. 다양한 타입을 반복하면서 `bbox()` 호출 가능             |
| `Vec<Box<dyn Bounded>`, `Arena<Mesh>` | 다양한 컨테이너에서 `Bounded` 구현체를 저장하고 반복 가능. 유연한 구성 가능 |

---



