# 🧠 Copy vs Clone 트레잇 요약
| 트레잇   | 목적                          | 동작 방식               | 특징 및 제약사항                          |
|----------|-------------------------------|--------------------------|-------------------------------------------|
| `Copy`   | 값의 **얕은 복사** (shallow copy) | 자동 복사 (`let y = x`) | 오버라이딩 불가, 모든 필드가 `Copy`여야 함 |
| `Clone`  | 값의 **깊은 복사** (deep copy)   | 명시적 호출 (`x.clone()`) | 오버라이딩 가능, 자원 복사에 적합           |



## ✅ Copy 트레잇
- 스택에 저장되는 값에 적합 (예: i32, bool, f64)
- Drop 트레잇이 구현된 타입은 Copy 불가
- #[derive(Copy, Clone)]로 자동 구현 가능
- 모든 필드가 Copy를 구현해야 전체 타입에 Copy 적용 가능
### 예시
```rust
#[derive(Debug, Copy, Clone)]
struct MyStruct {
    x: i32,
    y: i32,
}

fn main() {
    let a = MyStruct { x: 1, y: 2 };
    let b = a; // 자동 복사
    println!("{:?}", a); // 가능
    println!("{:?}", b);
}
```



## ✅ Clone 트레잇
- 힙에 저장된 자원을 복사할 때 사용 (예: String, Vec<T>)
- Copy보다 유연하며, 오버라이딩 가능
- #[derive(Clone)] 또는 impl Clone으로 구현
- 명시적으로 .clone() 호출 필요
### 예시
```rust
#[derive(Debug)]
struct MyStruct {
    x: i32,
    y: i32,
}

impl Clone for MyStruct {
    fn clone(&self) -> Self {
        println!("clone is called!");
        MyStruct { x: self.x, y: self.y }
    }
}

fn main() {
    let a = MyStruct { x: 1, y: 2 };
    let b = a.clone(); // 명시적 복사
    println!("{:?}", a);
    println!("{:?}", b);
}
```


## 🧪 Copy vs Clone: 어떤 걸 써야 할까?
| 상황 또는 목적                         | 추천 트레잇 |
|--------------------------------------|-------------|
| 단순한 스칼라 값만 포함된 타입         | `Copy`      |
| 힙 자원을 포함한 타입 (예: String, Vec) | `Clone`     |
| 복사 동작을 커스터마이징하고 싶을 때    | `Clone`     |
| 자동 복사로 성능 최적화가 필요한 경우   | `Copy`      |



## 📌 참고: Copy 가능한 타입 예시
- i32, u64, bool, f64
- (i32, i32) 같은 Copy 타입으로만 구성된 튜플
- char, usize, &T (immutable reference)
❌ String, Vec<T>, Box<T> 등은 Copy 불가 → Clone 필요


# Copy / Clone 성능 
Rust의 Copy와 Clone 트레잇을 중심으로 성능, 메모리 모델, 라이프타임과의 관계까지 확장한 정리.  
단순한 트레잇 설명을 넘어서, Rust의 안전성과 효율성 철학을 이해하는 데 핵심이 되는 내용.

## ⚡ 1. 성능 비교: Copy vs Clone
| 항목           | Copy                          | Clone                             |
|----------------|-------------------------------|-----------------------------------|
| 복사 방식      | `let b = a`                   | `let b = a.clone()`               |
| 복사 비용      | 매우 저렴 (스택 메모리 복사)   | 상대적으로 비쌈 (힙 메모리 복사 포함 가능) |
| 호출 방식      | 자동                           | 명시적                            |
| 오버라이딩 가능 | ❌ 불가능                       | ✅ 가능                            |
| 사용 대상      | 스칼라 값, Copy 가능한 구조체  | 힙 자원 포함 타입, 커스텀 복사 필요 시 |


## ✅ 예시: 성능 차이 측정
```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct BigData {
    values: Vec<i32>,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // 빠름

    let b1 = BigData { values: vec![1; 1000000] };
    let b2 = b1.clone(); // 느림 (힙 복사)
}
```

- Point는 스택에 저장되므로 복사 비용이 거의 없음
- BigData는 힙에 저장된 Vec을 복사하므로 성능에 영향

## 🧠 2. 메모리 모델과 Copy/Clone의 관계
Rust의 메모리 모델은 **소유권(ownership)**과 **이동(move)**을 중심으로 설계되어 있습니다.

### ✅ Copy가 가능한 타입
- 스택에 저장되는 프리미티브 타입: i32, bool, char, f64, usize
- Copy 가능한 타입으로만 구성된 튜플: (i32, i32)
- &T (불변 참조)

### ❌ Copy가 불가능한 타입
- 힙에 저장되는 타입: String, Vec<T>, Box<T>
- Drop 트레잇을 구현한 타입
- 복사 시 자원 소유권을 고려해야 하는 타입

### ✅ Clone은 힙 자원을 복사할 수 있음
- Vec, String, HashMap 등은 Clone을 통해 복사 가능
- Clone은 Drop과 함께 사용 가능

## ⏳ 3. 라이프타임과의 관계
Copy와 Clone은 라이프타임 자체를 변경하지는 않지만,
값의 소유권과 참조 방식에 따라 라이프타임 설계에 영향을 줍니다.

### ✅ Copy와 라이프타임
```rust
fn use_copy(x: i32) {
    let y = x; // x는 여전히 유효
}
```

- i32는 Copy → 소유권 이동 없음 → 라이프타임 유지
### ❌ Clone과 라이프타임
```rust
fn use_clone(s: String) {
    let s2 = s.clone(); // s와 s2는 별개의 힙 자원
}
```

- String은 Clone → 힙 자원 복사 → 라이프타임 분리 가능
## ✅ 참조와 라이프타임
```rust
fn use_ref<'a>(s: &'a String) {
    println!("{}", s);
}
```


- 참조를 사용할 경우 라이프타임 'a를 명시해야 안전성 보장

## 🧪 실전 팁 요약
| 상황 또는 목적                         | 추천 방식       | 설명                                      |
|--------------------------------------|----------------|-------------------------------------------|
| 스택 기반 값 복사                    | `Copy`         | 빠르고 자동으로 복사됨                     |
| 힙 자원 포함 타입 복사               | `Clone`        | 명시적으로 `.clone()` 호출 필요             |
| 복사 동작을 커스터마이징하고 싶을 때 | `Clone + impl` | `Clone`을 직접 구현하여 로깅 등 추가 가능     |
| 라이프타임 명시 없이 값 전달         | `Copy`         | 소유권 이동 없이 값 전달 가능               |
| 성능 최적화가 중요한 경우            | `Copy`         | 메모리 복사 비용이 낮아 빠른 처리 가능       |


---

# 주의 사항 (개념 잘못 이해)


# Clone / Copy
| 🧠 개념 정리: `Clone` vs `Copy` | 호출 방식     | 복사 방식        | 특징                          |
|----------------------------------|----------------|------------------|-------------------------------|
| `Clone`                          | `.clone()`     | 명시적 복사      | 깊은 복사 가능, 비용이 있을 수 있음 |
| `Copy`                           | 자동           | 암묵적 복사      | 경량 타입에 적합, move 대신 복사됨  |

- Copy는 Clone의 경량 버전으로, 소유권을 이동시키지 않고 자동 복사를 허용함
- Copy가 선언된 타입은 let b = a;처럼 대입하거나 함수 인자로 넘길 때 move가 아닌 복사가 일어남

## 전체 코드
```rust

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quaternion {
    /// self 다음에 next 회전 적용 (R_{self} ∘ R_{next} 아님! 이름 그대로 '순서')
    pub fn then(self, next: Self) -> Quaternion {
        self * next // q1 다음 q2 ⇒ q1*q2
    }
}

#[test]
fn then_helpers_match_manual_composition() {
    let qx = Quaternion::from_axis_angle_deg(Vector3D::new(1.0,0.0,0.0), 90.0);
    let qz = Quaternion::from_axis_angle_deg(Vector3D::new(0.0,0.0,1.0), 90.0);
    let v  = Vector3D::new(1.0,0.0,0.0);

    let q_then = qx.then(qz);              // qx 후 qz
    let r1 = q_then.rotate_vector(v);

    let r2 = qz.rotate_vector(qx.rotate_vector(v));
    assert!((r1.x-r2.x).abs()<1e-9 && (r1.y-r2.y).abs()<1e-9 && (r1.z-r2.z).abs()<1e-9);

    // Transform도 동일한 의미로 작동
    let Rx = qx.to_transform();
    let Rz = qz.to_transform();
    let t_then = Rx.then(&Rz);              // Rx 후 Rz  →  Rz * Rx
    let r3 = t_then.transform_vector3d(&v);
    assert!((r3.x-r2.x).abs()<1e-9 && (r3.y-r2.y).abs()<1e-9 && (r3.z-r2.z).abs()<1e-9);
    }

```


## 🔍 지금 코드에서 일어나는 일
```rust
impl Quaternion {
    pub fn then(self, next: Self) -> Quaternion {
        self * next
    }
}
```

- self는 Quaternion 타입
- Quaternion은 #[derive(Clone, Copy)] 되어 있음
- 따라서 self는 move되지 않고 복사됨
즉, qx.then(qz)를 호출해도 qx는 여전히 유효하고 이후에도 사용할 수 있어요.

✅ 예시로 확인
```rust
let qx = Quaternion::from_axis_angle_deg(...);
let qz = Quaternion::from_axis_angle_deg(...);

let q_then = qx.then(qz); // self = qx, next = qz

let again = qx.rotate_vector(...); // ✅ qx는 여전히 유효
```

- qx는 then() 호출 후에도 살아 있음
- 만약 Copy가 없었다면 qx는 move되고 이후 사용 불가

## 📦 왜 Copy가 가능한가?
### Rust에서 Copy는 다음 조건을 만족해야 합니다:
- 모든 필드가 Copy여야 함
- Drop trait을 구현하지 않아야 함
### Quaternion은 f64만 포함하므로:
- f64는 Copy
- 따라서 Quaternion도 Copy 가능

## 🧠 요약 
|     조건               | self 소비 방식         | 동작 설명                          |
|---------------------------|------------------------|-------------------------------------|
| `#[derive(Clone, Copy)]`  | `self`                 | 값이 자동 복사됨 (move 아님)        |
| `Copy`                    | `self`                 | 암묵적 복사, 이후에도 원본 사용 가능 |
| `Clone`                   | `.clone()`             | 명시적 복사 필요, 원본은 move됨     |

---


# Copy trait가 필요한 이유

## 코드
```rust
impl Distance for Vector2D {
    fn distance(&self, other: &Self) -> f64 {
        (*other - *self).length()
    }
}
```
## 에러 코드
```
112 |         (*other - *self).length()
    |         -^^^^^^---------
    |         ||
    |         |move occurs because `*other` has type `Vector2D`, which does not implement the `Copy` trait
    |         `*other` moved due to usage in operator
    |

```

## 이 코드에서 Copy 트레잇이 필요한 이유는 바로 이 부분 때문:
```rust
(*other - *self).length()
```

여기서 *other와 *self는 Vector2D 타입의 값을 꺼내는 동작.
즉, &Vector2D를 Vector2D로 **역참조(dereference)**해서 Sub 연산을 수행.

## 🔍 왜 Copy가 필요한가?
Rust는 기본적으로 **값을 move(이동)**하려고 함.
*self나 *other는 Vector2D 값을 꺼내는 것이므로,
이 값을 - 연산에 넘기면 move가 발생합니다.
하지만 self와 other는 &Vector2D — 즉, 참조 타입이기 때문에
값을 move하면 이후에 다시 사용할 수 없게 됨.
그래서 Rust는 이 값을 안전하게 꺼내려면 Copy 트레잇이 구현되어 있어야 한다고 요구.

## ✅ 해결 방법
### 방법 1: Vector2D에 Copy 트레잇을 구현
```rust
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}
```

f64는 이미 Copy이므로 Vector2D도 Copy를 안전하게 구현할 수 있어요


### 방법 2: clone()을 사용해서 명시적 복사
```
(other.clone() - self.clone()).length()
```

이건 Copy 없이도 작동하지만, 성능상 Copy가 더 효율적이고 idiomatic이에요


## ✨ 요약: `Distance` 트레잇 구현 시 필요한 개념
| 표현       | 필요 조건 또는 설명             |
|------------|----------------------------------|
| `*self`    | `Copy` 트레잇이 있어야 안전하게 값 복사 가능 |
| `Copy`     | `Sub` 연산에서 값 이동 없이 사용하려면 필요 |
| `Clone`    | `Copy`가 없을 경우 명시적 복사에 사용 (`clone()`) |

----


# 🧠 왜 Clone()이 필요한가?

## 이런 구조로 Point2D를 설계한다면
```rust
#[derive(Clone)]
struct Point2D {
    label: String,
    x: f64,
    y: f64,
}
```

Rust의 Copy는 **얕은 복사(shallow copy)**만 수행.
즉, 스택에 있는 값만 복사하고, 힙에 있는 데이터는 공유된 포인터로 남아요.
반면 Clone은 **사용자가 정의한 방식으로 깊은 복사(deep copy)**를 수행할 수 있음.

## 🔍 예시 비교
### ✅ Copy 가능한 구조체
```rust
#[derive(Copy, Clone)]
struct Point2D {
    x: f64,
    y: f64,
}
```

- f64는 Copy이므로 Point2D도 Copy 가능
- *self나 let p = point;처럼 자유롭게 복사 가능


### ❌ Copy 불가능한 구조체 (예: String 포함)
```rust
#[derive(Clone)]
struct LabelledPoint {
    label: String,
    x: f64,
    y: f64,
}
```

- String은 힙을 사용하는 타입이라 Copy 불가
- 복사하려면 point.clone()을 명시적으로 호출해야 함

## ✨ 요약: 언제 Clone()이 필요한가?
| 구조체 구성 요소     | 복사 방식       | 필요한 트레잇 |
|----------------------|----------------|----------------|
| 모두 `Copy` 타입     | 얕은 복사       | `Copy`         |
| 힙 데이터 포함       | 깊은 복사 필요  | `Clone`        |
| 사용자 정의 복사 로직 | 커스터마이징 가능 | `Clone`        |


## 🔍 핵심 개념
```rust
(*other - *self).length()
```

이 표현에서 *other와 *self는 Vector2D 값을 꺼내는 동작인데,
이 값들을 Sub 연산에 넘기면 move가 발생합니다.
Rust는 move가 가능한 타입이면 그대로 이동시키고,
Clone이 선언되어 있어도 자동으로 복사하지 않아요.

### ✅ 복사가 필요한 경우엔 명시적으로 clone() 호출
```rust
(other.clone() - self.clone()).length()
```

이렇게 해야만 Rust는 Clone 트레잇을 사용해서 깊은 복사를 수행합니다.

## ✨ 요약: 자동 호출 여부
| 상황                          | 설명                                       |
|-------------------------------|--------------------------------------------|
| `Clone` 선언만 있음           | 자동 호출 ❌ → `clone()` 직접 호출해야 함     |
| `Copy` 선언 있음              | 자동 복사 ✅ → `*self`, `let x = y` 가능     |
| `Clone` + `Copy` 둘 다 있음   | `Copy` 우선 적용 → 자동 복사됨              |

Rust는 명시적 복사 철학을 따르기 때문에,
Clone은 “복사 가능”하다는 뜻이지 “자동 복사”는 아니에요.
그래서 clone()을 직접 호출해야 하고,
반면 Copy는 “자동 복사 가능”하다는 뜻이라 *self나 let x = y가 자유롭게 작동합니다.



## 🧠 핵심 개념: String은 왜 Copy가 안 되는가?
- String은 힙에 데이터를 저장하는 타입이에요. 내부적으로는:
- 포인터
- 길이
- 용량
이 세 가지를 가진 구조체입니다.
- Copy는 **비트 단위 복사(shallow copy)**만 허용하는 트레잇인데,
String처럼 힙 메모리를 소유하는 타입은 얕은 복사로는 안전하지 않아요.
만약 String을 Copy로 복사하면, 두 개의 String이 같은 힙 메모리를 가리키게 되고,
둘 다 drop 시에 **중복 해제(double free)**가 발생할 수 있어요. Rust는 이런 위험을 막기 위해 Copy를 금지합니다.


## ✅ 그래서 어떻게 해야 하나?
## 구조체에 String이 있을 경우:
- Copy는 불가능 ❌
- Clone은 가능 ✅ → 명시적으로 clone()을 호출해야 함
```rust
#[derive(Clone)]
struct LabelledPoint {
    label: String,
    x: f64,
    y: f64,
}

let p1 = LabelledPoint {
    label: "origin".to_string(),
    x: 0.0,
    y: 0.0,
};

let p2 = p1.clone(); // 깊은 복사 수행
```

이때 clone()은 String 내부의 힙 데이터를 새로 복사해서 안전하게 새로운 인스턴스를 만들어줍니다.


## ✨ 요약: String이 있을 때의 복사 전략
| 필드 타입 | Copy 가능 여부 | Clone 가능 여부 | 복사 방식         |
|-----------|----------------|------------------|------------------|
| `f64`, `bool`, `usize` 등 | ✅ 가능          | ✅ 가능           | 얕은 복사 (스택)  |
| `String`, `Vec<T>`, `Box<T>` 등 | ❌ 불가능       | ✅ 가능           | 깊은 복사 (힙)    |


## 🔥 왜 에러가 나는가?
- *self와 *other는 LabelledPoint 같은 구조체의 값을 꺼내는 동작
- 그런데 LabelledPoint가 String을 포함하고 있다면, 이 타입은 Copy가 불가능합니다
- 따라서 *self는 move가 발생하고, 이후에 self를 다시 사용하려 하면 컴파일 에러가 발생합니다

## ✅ 해결 방법
### 1. clone()을 명시적으로 호출
```rust
(other.clone() - self.clone()).length()
```

이 방식은 String을 포함한 구조체에서도 안전하게 작동합니다
Clone은 힙 데이터를 깊은 복사하므로 소유권 문제가 발생하지 않음


### 2. Copy를 붙이려면 String을 제거하거나 &str로 바꾸기
```rust
#[derive(Copy, Clone)]
struct Point2D {
    x: f64,
    y: f64,
}
```

Copy를 쓰고 싶다면 구조체는 스택 기반 타입만 포함
String 대신 &'static str 같은 참조 타입을 쓰면 Copy가 가능해질 수도 있음


## ✨ 요약
| 구조체 필드에 `String` 있음 | `*self` 사용 시 결과       | 해결 방법           |
|-----------------------------|----------------------------|---------------------|
| ✅ 있음                     | ❌ move 발생 → 컴파일 에러 | `clone()` 호출 필요 |
| ❌ 없음 (`Copy` 가능)       | ✅ 자동 복사됨              | 그대로 사용 가능     |


Rust는 이런 타입 안전성 덕분에 런타임 오류 없이 메모리를 관리할 수 있음.

---










