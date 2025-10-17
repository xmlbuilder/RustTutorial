# OOP → 행위 중심으로 바꾸기

## OOP → 행위 중심 “매핑표”
- 상속 트리 → trait 집합 + 구성체(필드로 전략 보관)
- 가상 함수 오버라이드 → dyn Trait(런타임) / 제네릭 T: Trait(컴파일타임)
- 공통 상위 클래스(ON_Object) → “빈 베이스” 제거, 능력 트레이트로 분해  
    예: trait Bounded { fn bbox(&self) -> BoundingBox; }
- 거대 매치/스위치 → 트레이트 폴리몰피즘으로 분산
- 옵저버/신호 → 경량 이벤트 버스 or Signal/subscribe(이전 예제)
- 싱글톤 → 컨텍스트 주입(명시적으로 &mut Ctx 전달)


## 1️⃣ 상속 트리 → trait 집합 + 구성체(필드로 전략 보관)
### OOP 방식
```cpp
class Shape {
  virtual void draw();
};

class Circle : public Shape {
  void draw() override;
};
```

### Rust 방식
```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
impl Drawable for Circle {
    fn draw(&self) {
        println!("Draw Circle");
    }
}

struct Renderer {
    strategy: Box<dyn Drawable>,
}

impl Renderer {
    fn render(&self) {
        self.strategy.draw();
    }
}
```

- ✅ 상속 대신 trait로 행위를 추상화하고,
- ✅ 전략 객체를 필드로 보관해 동작을 구성


## 2️⃣ 가상 함수 오버라이드 → dyn Trait(런타임) / T: Trait(컴파일타임)
### 런타임 방식 (동적 디스패치)
```rust
fn render_all(items: Vec<Box<dyn Drawable>>) {
    for item in items {
        item.draw(); // vtable 호출
    }
}
```

### 컴파일타임 방식 (정적 디스패치)
```rust
fn render_all<T: Drawable>(items: &[T]) {
    for item in items {
        item.draw(); // 인라인 최적화 가능
    }
}
```
- ✅ dyn Trait은 OOP의 가상 함수와 유사
- ✅ T: Trait은 C++ 템플릿처럼 빠르고 안전


## 3️⃣ 공통 상위 클래스 → “빈 베이스” 제거, 능력 트레이트로 분해
### OOP 방식
```rust
class ON_Object {
  virtual BoundingBox bbox();
};
```

### Rust 방식
```rust
trait Bounded {
    fn bbox(&self) -> BoundingBox;
}

struct Mesh;
impl Bounded for Mesh {
    fn bbox(&self) -> BoundingBox { /* ... */ }
}
```
- ✅ “능력” 단위로 트레이트 분해
- ✅ 필요할 때만 구현 → 조합 유연성 증가


## 4️⃣ 거대 매치/스위치 → 트레이트 폴리몰피즘으로 분산

### OOP 방식
```cpp
switch (obj.type) {
  case CIRCLE: draw_circle(obj); break;
  case RECT: draw_rect(obj); break;
}
```

### Rust 방식
```rust
trait Drawable {
    fn draw(&self);
}

fn render_all(items: Vec<Box<dyn Drawable>>) {
    for item in items {
        item.draw(); // 분산된 구현 호출
    }
}
```
- ✅ match 대신 트레이트 구현으로 분산
- ✅ 유지보수성과 확장성 향상


## 5️⃣ 옵저버/신호 → 경량 이벤트 버스 or Signal/subscribe
### Rust 방식 (간단한 Signal)
```rust
struct Signal<T> {
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> Signal<T> {
    fn subscribe<F: Fn(&T) + 'static>(&mut self, f: F) {
        self.subscribers.push(Box::new(f));
    }

    fn emit(&self, value: T) {
        for sub in &self.subscribers {
            sub(&value);
        }
    }
}
```
- ✅ 옵저버 패턴을 명시적으로 구현
- ✅ Fn 트레잇으로 콜백 처리


## 6️⃣ 싱글톤 → 컨텍스트 주입 (&mut ctx)
### OOP 방식
```cpp
Logger::instance().log("msg");
```

###  Rust 방식
```rust
struct Context {
    logs: Vec<String>,
}

fn log(ctx: &mut Context, msg: &str) {
    ctx.logs.push(msg.to_string());
}
```

- ✅ 전역 대신 명시적 주입
- ✅ 테스트 가능성, 추적 가능성 향상


## ✨ 전체 요약
| OOP 개념               | Rust 전환 방식                          |
|------------------------|-----------------------------------------|
| 상속 트리              | `trait` 집합 + 전략 필드 구성체         |
| 가상 함수              | `dyn Trait` / `T: Trait`                |
| 공통 베이스 클래스     | 능력 기반 트레이트 분해 (`Bounded`, 등) |
| 거대 switch/match      | 트레이트 폴리몰피즘으로 분산            |
| 옵저버/신호            | Signal 구조체 + subscribe               |
| 싱글톤                 | 명시적 `&mut Context` 주입              |

---



