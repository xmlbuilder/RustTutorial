# Allocator

## 🧠 Allocator란? (일반적인 개념)

Allocator는 **메모리를 어떻게, 어디에, 얼마나 할당할지 결정하는 전략** 입니다.  

Rust는 기본적으로 안전한 메모리 관리를 위해 **스택(stack)** 과 **힙(heap)** 을 구분해서 사용.  
하지만 어떤 데이터를 저장할지, 얼마나 저장할지 모를 때는 동적으로 메모리를 할당해야 합니다.. 이때 필요한 게 바로 Allocator입니다.  

## 🧪 아주 간단한 비유
- 📦 스택: 정해진 크기의 선반. 미리 크기를 알고 있어야 함. 빠르고 간단함.
- 📦 힙: 창고에서 공간을 빌려오는 방식. 크기를 나중에 정할 수 있지만, 관리가 복잡함.
- Allocator는 이 창고에서 얼마나 공간을 빌릴지, 어떻게 정리할지를 알려주는 규칙.

## 🧪 Rust 예제
Rust 표준 라이브러리에는 Box, Vec, String 같은 타입들이 있는데, 이들은 내부적으로 힙에 메모리를 할당. 예를 들어:  
```rust
fn main() {
    let x = Box::new(42); // 정수 42를 힙에 저장
    let v = vec![1, 2, 3, 4]; // 힙에 4개의 정수 저장
    println!("x = {}, v = {:?}", x, v);
}
```
여기서 Box와 Vec은 내부적으로 **Rust의 글로벌 할당자(Global Allocator)** 를 사용해서 메모리를 할당.  
Rust는 기본적으로 `std::alloc::Global` 이라는 기본 할당자를 사용하지만, 필요하면 사용자 정의 Allocator도 만들 수 있습니다.

## 🧪 실전 예: 사용자 정의 Allocator 만들기 (Rust 1.68+)
```rust
use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("Allocating {} bytes", layout.size());
        System.alloc(layout) // 기본 시스템 할당자 사용
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        println!("Deallocating {} bytes", layout.size());
        System.dealloc(ptr, layout)
    }
}
```
```rust
#[global_allocator]
static A: MyAllocator = MyAllocator;

fn main() {
    let v = vec![1, 2, 3]; // 할당 시 로그 출력됨
    println!("{:?}", v);
}
```

- 이 예제는 모든 힙 할당을 감시하는 사용자 정의 할당자를 설정한 것임.
- 실제로는 게임 엔진, 임베디드 시스템, 고성능 컴퓨팅 등에서 커스텀 메모리 전략을 위해 사용됨.

## ✅ 요약
| 구성 요소             | 설명 또는 역할                                                       |
|----------------------|----------------------------------------------------------------------|
| `Allocator` (Rust)   | 메모리를 어떻게 할당할지 정의하는 트레이트. `alloc`, `dealloc` 메서드 포함 |
| `std::alloc::Global` | Rust의 기본 할당자. 대부분의 힙 타입(`Box`, `Vec`, `String`)에서 자동 사용됨 |
| 사용자 정의 할당자    | `GlobalAlloc` 트레이트를 구현하여 커스텀 메모리 전략을 정의할 수 있음     |

---

```rust
#[global_allocator] 
static A: MyAllocator = MyAllocator; 
```

이렇게 선언하면 Rust 전체 프로그램에서 사용하는 `기본 힙 할당자(Global Allocator)` 가 바뀝니다.

## 🔍 무슨 일이 일어나냐면…
Rust는 기본적으로 `std::alloc::Global` 이라는 시스템 할당자를 사용해서 Box, Vec, String 같은 힙 기반 타입의 메모리를 관리합니다.
그런데 위처럼 `#[global_allocator]` 를 선언하면, 프로그램 전체에서 사용하는 힙 메모리 할당 전략이 사용자 정의 할당자로 대체됩니다.

## ✅ 예시
```rust
use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        println!("🔧 Allocating {} bytes", layout.size());
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        println!("🧹 Deallocating {} bytes", layout.size());
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static A: MyAllocator = MyAllocator;

fn main() {
    let v = vec![1, 2, 3]; // 이 벡터의 메모리 할당은 MyAllocator를 통해 이루어짐
}
```

- 이 코드를 실행하면 vec!이 내부적으로 MyAllocator를 통해 메모리를 할당하고 해제합니다.
- 즉, 모든 힙 할당이 커스텀 로직을 거치게 됩니다.

## ⚠️ 주의할 점

| 항목               | 설명                                                                 |
|--------------------|----------------------------------------------------------------------|
| `unsafe` 사용       | `GlobalAlloc` 트레이트 구현 시 `unsafe` 블록 필수. 메모리 안전성 직접 책임져야 함 |
| 전역 영향          | 프로그램 전체의 힙 할당 방식이 바뀌므로, 모든 `Vec`, `Box`, `String` 등에 영향 |
| 디버깅 어려움       | 잘못된 구현 시 런타임 오류, 메모리 누수, use-after-free 등 디버깅이 어려움     |
| 성능 저하 가능성    | 비효율적인 할당 전략은 전체 프로그램 성능에 악영향을 줄 수 있음               |
| 라이브러리 충돌 가능성 | 일부 라이브러리는 특정 할당자에 의존할 수 있어, 커스텀 할당자와 충돌 가능성 있음 |


## 🧩 요약

| 키워드 또는 구성 요소     | 설명 또는 역할                                                       |
|--------------------------|----------------------------------------------------------------------|
| `#[global_allocator]`    | Rust 전체 프로그램의 기본 힙 할당자를 사용자 정의 할당자로 교체하는 속성 |
| 힙 기반 타입들            | `Box`, `Vec`, `String`, `HashMap` 등은 모두 글로벌 할당자를 통해 메모리 할당 |
| 영향 범위                 | 프로그램 전역의 힙 할당 동작에 영향을 주며, 모든 힙 기반 타입에 적용됨     |

---

# nalgebra와 연계

nalgebra를 사용하는 간단한 예제와 실전 예제를 함께 소개.  

## 🧠 Rust에서의 Allocator란?
Rust는 타입 안정성과 메모리 안전성을 중시하는 언어입니다.  
Allocator는 Rust에서 타입과 차원에 따라 메모리를 안전하게 할당할 수 있는지를 컴파일 타임에 확인하기 위한 **트레이트(trait)** 입니다.
Rust 자체에는 Allocator라는 이름의 표준 트레이트가 있지만, 여기서 말하는 Allocator는  라이브러리에서 정의된 것으로,  
벡터나 행렬을 생성할 때 필요한 메모리 할당자입니다.

## 📌 핵심 개념
```rust
pub trait Allocator<T, D>
where
    D: Dim,
{
    // 메모리 할당 전략을 정의하는 트레이트
}
```

- T: 저장할 값의 타입 (예: f64)
- D: 벡터의 차원 (U1, U2, Dynamic 등)

## 🧪 심플 예제: 고정 차원 벡터
```rust
use nalgebra::{OVector, Vector3, DefaultAllocator};
use nalgebra::allocator::Allocator;

fn example_fixed_vector() {
    let v: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
    println!("{:?}", v);
}
```
- Vector3<f64>는 고정 크기 벡터
- Allocator<f64, U3>는 컴파일 타임에 자동으로 만족됨
- DefaultAllocator가 내부적으로 이를 구현

## 🧪 실전 예제: 제네릭 시스템 정의
```rust
use nalgebra::{OVector, Dim, DefaultAllocator};
use nalgebra::allocator::Allocator;

// 시스템 정의
struct MyODE;

impl<D: Dim> ode_solvers::System<f64, OVector<f64, D>> for MyODE
where
    DefaultAllocator: Allocator<f64, D>,
{
    fn system(&self, x: f64, y: &OVector<f64, D>, dy: &mut OVector<f64, D>) {
        dy[0] = x - y[0]; // 예시: y' = x - y
    }
}
```

- D는 벡터의 차원 (고정 또는 동적)
- Allocator 제약을 통해 OVector<f64, D>가 안전하게 생성됨
- 이 구조는 ODE 적분기에서 제네릭 시스템을 정의할 때 필수

## ✅ 요약

| 구성 요소           | 설명 또는 역할                                                                 |
|--------------------|----------------------------------------------------------------------------------|
| `Allocator`        | Rust 트레이트. 타입 `T`와 차원 `D`에 맞는 메모리 공간을 할당할 수 있는지 정의함 |
| `DefaultAllocator` | 대부분의 경우 자동으로 선택되는 기본 할당자. `Allocator`를 내부적으로 구현함     |
| `OVector`, `OMatrix` | 일반화된 벡터/행렬 타입. 차원에 따라 `Allocator`가 필요함                     |

---







