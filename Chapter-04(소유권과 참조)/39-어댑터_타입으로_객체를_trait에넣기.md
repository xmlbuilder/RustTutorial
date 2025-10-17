# 🧩 핵심 개념: 어댑터 타입으로 C 객체를 Rust 트리에 넣기

### 1. 기존 C 구조체: ON_Object
- C에서 넘어온 구조체 (ON_Object)는 Rust에서 직접 다루기엔 안전하지 않거나 기능이 부족할 수 있음.
- 예를 들어, ON_Object는 HasAABB 트레잇을 구현하지 않아서 트리에 넣을 수 없음.

### 2. 어댑터 타입: OnObjectAdapter<'a>(&'a ON_Object)
- 이건 래퍼 구조체. ON_Object를 감싸서 Rust에서 필요한 트레잇(HasAABB)을 구현할 수 있게 해줌.
- 즉, 기존 타입을 확장하거나 연결해주는 가교 역할을 합니다.
- 
```rust
struct OnObjectAdapter<'a>(&'a ON_Object);

impl<'a> HasAABB for OnObjectAdapter<'a> {
    fn aabb(&self) -> AABB {
        // ON_Object의 데이터를 기반으로 AABB 계산
    }
}
```
- 이렇게 하면 SpatialTree 같은 구조에 OnObjectAdapter를 넣을 수 있음.
- 실제 데이터는 ON_Object지만, 트리 입장에서는 HasAABB를 구현한 객체로 인식됨.

### 3. C FFI 포인터 처리: NonNull + lifetime
- C에서 넘어온 포인터는 *mut ON_Object 같은 raw pointer일 수 있음.
- 이걸 Rust에서 안전하게 쓰려면 NonNull<ON_Object>로 감싸고,
`'a` 같은 **수명(lifetime)** 을 붙여서 메모리 안전성을 보장.

```rust
use std::ptr::NonNull;

struct OnObjectAdapter<'a> {
    ptr: NonNull<ON_Object>,
    _marker: PhantomData<&'a ON_Object>,
}
```

- PhantomData는 `'a` 수명을 트래킹하기 위한 도구예요.
- 이렇게 하면 Rust가 **이 포인터는 `'a` 동안 유효하다** 는 걸 알 수 있음.

## ✅ 요약
| 목적                     | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| 어댑터 타입              | `OnObjectAdapter<'a>(&'a ON_Object)` → 트레잇 구현을 위한 래퍼        |
| 트레잇 확장              | `HasAABB` 구현 → 트리에 넣을 수 있게 만듦                            |
| C 포인터 안전 처리       | `NonNull + lifetime + PhantomData` → FFI 포인터를 Rust에서 안전하게 사용 |

---

# PhantomData

PhantomData는 Rust에서 타입이나 수명 정보를 컴파일러에게 알려주는 용도로 쓰이는 **제로 크기 타입(zero-sized type)** 임.
직접 데이터를 담진 않지만, 타입 시스템과 수명 추적에 중요한 역할을 합니다.

### 🧠 왜 `'a` 수명을 트래킹해야 할까?
예를 들어 C FFI에서 넘어온 포인터를 NonNull<T>로 감싸면,  
Rust는 이 포인터가 얼마나 오래 살아야 하는지 알 수 없음.  
그런데 우리가 `'a` 수명 동안만 유효하다고 알고 있다면,  
컴파일러에게 그 정보를 명시적으로 알려줘야 함.  

### ✅ PhantomData<&'a T>로 수명 트래킹하는 방식
```rust
use std::ptr::NonNull;
use std::marker::PhantomData;

struct OnObjectAdapter<'a> {
    ptr: NonNull<ON_Object>,
    _marker: PhantomData<&'a ON_Object>,
}
```

- PhantomData<&'a ON_Object>는 **이 타입은 `'a` 동안만 유효한 ON_Object를 참조한다** 는 의미.
- 실제로 `&'a` ON_Object를 저장하지 않지만, 컴파일러는 이 수명을 기반으로 borrow checker를 작동 시킴.

### 🔍 어떻게 작동하냐면…
- PhantomData<&`'a` T>는 읽기 참조 수명을 추적합니다.
- PhantomData<&`'a` mut T>는 가변 참조 수명을 추적합니다.
- 이 정보를 통해 Rust는 이 타입이 언제까지 안전하게 사용될 수 있는지를 판단합니다.

## ✨ 요약
| 역할                     | 설명                                                                 |
|--------------------------|----------------------------------------------------------------------|
| `PhantomData<&'a T>`     | `'a` 수명 동안 참조가 유효함을 컴파일러에게 알림                     |
| 실제 데이터 없음         | 메모리에는 아무것도 저장하지 않음 (zero-sized)                       |
| 수명 추적 목적           | FFI 포인터, unsafe 구조체, 트레잇 구현 시 안전성 보장에 사용          |


---

