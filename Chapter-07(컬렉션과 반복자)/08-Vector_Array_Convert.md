# Collection 간 호환 (Vector & Array)

Rust에서 Vector ↔ Array 변환, Map에서 key/value로 array/vector 사용,  
그리고 컬렉션 간 데이터 전달 기법를 다룸

## 🧠 Vector ↔ Array 변환
### 1. Vector → Array
#### ✅ 복사 (정적 크기)
```rust
let v = vec![1, 2, 3];
let a: [i32; 3] = [v[0], v[1], v[2]]; // 수동 복사
```

#### ✅ reference
```rust
let v = vec![1, 2, 3];
let slice: &[i32] = &v[..]; // &[T]는 array처럼 사용 가능
```

```rust
let slice: &[i32] = &v
```
&v[..]와 &v는 대부분의 경우 동일하게 작동하지만,  
의미와 동작 방식에는 미묘한 차이가 있습니다.

##### ✅ &v[..] vs &v 차이점
| 표현       | 의미 또는 대상       | 실제 타입     | 동작 방식 또는 변환 과정     | 비고 또는 추천 상황             |
|------------|----------------------|----------------|-------------------------------|----------------------------------|
| `&v[..]`   | 전체 슬라이스 참조    | `&[i32]`       | 명시적 슬라이스 생성          | 슬라이스 범위 지정 시 유용       |
| `&v`       | `Vec` 참조            | `&Vec<i32>`    | `Deref` 통해 `&[i32]`로 변환됨 | 대부분 자동 변환되므로 편리함    |

##### 🔍 왜 둘 다 작동할까?
Rust는 Vec<T>에 대해 Deref<Target = [T]>를 구현해놨기 때문에
&Vec<T>는 자동으로 &[T]로 간접 변환(deref coercion) 됩니다.
즉, &v는 내부적으로 &*v → &v[..]로 바뀌는 거예요.

##### ✅ 언제 &v[..]를 쓰는 게 좋을까?
- 타입이 명확해야 할 때 (&[T]로 고정)
- 슬라이스 범위를 조절할 때 (&v[1..], &v[..2] 등)
- Vec이 아닌 다른 슬라이스와 비교하거나 함수 인자로 넘길 때

##### 💬 결론
&v와 &v[..]는 대부분 동일하게 작동하지만,
&v[..]는 명시적 슬라이스이고, &v는 간접 슬라이스입니다.
타입이 중요한 상황에서는 &v[..]가 더 안전하고 명확.

#### ✅ 가변 참조
```rust
let mut v = vec![1, 2, 3];
let slice_mut: &mut [i32] = &mut v[..];
slice_mut[0] = 10;
```

Rust에서는 [T; N]은 고정 크기이므로
Vec<T> → [T; N]은 크기 확인이 필요함

### 2. Array → Vector
#### ✅ 복사
```rust
let a = [1, 2, 3];
let v = a.to_vec(); // Vec<i32>
```

### ✅ reference
```rust
let a = [1, 2, 3];
let slice: &[i32] = &a;
```


### ✅ 가변 참조
```rust
let mut a = [1, 2, 3];
let slice_mut: &mut [i32] = &mut a;
slice_mut[1] = 99;
```


## 🧠 Map에서 key/value가 array 또는 vector일 때
### 1. Key가 array or vector
❌ 일반적으로 불가능
- [T; N]는 Eq + Hash가 구현되어 있으면 가능
- Vec<T>는 Hash가 구현되어 있어도 mutable이므로 위험

```rust
use std::collections::HashMap;
let mut map: HashMap<[i32; 3], &str> = HashMap::new();
map.insert([1, 2, 3], "point");
```
Vec<T>를 key로 쓰는 건 권장되지 않음
대신 Arc<Vec<T>>나 Box<[T]>로 고정화해서 쓰는 방식이 있음


### 2. Value가 array or vector
#### ✅ 복사
```rust
let mut map: HashMap<&str, Vec<i32>> = HashMap::new();
map.insert("a", vec![1, 2, 3]);
```

#### ✅ 참조
```rust
let v = vec![1, 2, 3];
let map: HashMap<&str, &[i32]> = HashMap::from([("a", &v[..])]);
```

#### ✅ 가변 참조
```rust
let mut v = vec![1, 2, 3];
let map: HashMap<&str, &mut [i32]> = HashMap::from([("a", &mut v[..])]);
```
---
#### lifetime 참고 
i32처럼 Copy 타입이라면 &[i32] 슬라이스를 참조해도 특별한 lifetime 명시 없이 잘 작동하지만,
구조체 같은 heap 자원을 포함한 타입이 들어오면 lifetime을 명시해줘야 할 경우가 많습니다.

#### ✅ 왜 구조체는 lifetime 명시가 필요할까?
- 구조체는 보통 String, Vec, Box 같은 heap 자원을 포함하고 있고
이를 참조하면 그 자원이 얼마나 오래 살아야 하는지를 컴파일러가 알아야 함.
- &[MyStruct]처럼 구조체 슬라이스를 참조할 경우,
슬라이스 내부의 참조가 살아있는 범위를 명시해야 안전하게 컴파일됩니다.

#### 🔍 예시
##### ✅ Copy 타입 (OK)
```rust
let v = vec![1, 2, 3];
let map: HashMap<&str, &[i32]> = HashMap::from([("a", &v[..])]); // OK

```

##### ⚠️ 구조체 타입 (lifetime 필요)
```rust
struct Person {
    name: String,
}

fn make_map<'a>(v: &'a [Person]) -> HashMap<&'a str, &'a [Person]> {
    HashMap::from([("a", v)]) // ✅ lifetime 명시 필요
}
```

- 여기서 'a는 v와 HashMap 내부 참조가 같은 생명주기를 공유한다는 뜻.
- 만약 생략하면 컴파일러가 “이 참조가 얼마나 살아야 하는지 모르겠어” 하고 에러를 냅니다.

##### 🔑 핵심 요약: 참조와 소유권

| 타입 또는 상황     | 참조 가능 여부     | 복사 또는 이동 방식 | 비고 또는 주의사항              |
|--------------------|--------------------|----------------------|---------------------------------|
| `i32`, `bool`      | 쉽게 참조 가능     | `Copy`로 자동 복사됨 | 소유권 이동 없이 사용 가능       |
| `String`, `Vec`    | 참조 가능하지만 주의 | `Clone` 또는 `move` 필요 | heap 자원이므로 lifetime 명시 필요 |

Rust는 참조가 안전하게 유지되는지를 컴파일러가 엄격하게 체크하기 때문에구조체를 참조할 땐 lifetime을 명시하는 습관이 정말 중요합니다.

---


### 🧠 Collection 간 데이터 전달 기법

| 기법         | 방식        | 예시 코드                          | 설명                                      |
|--------------|-------------|------------------------------------|-------------------------------------------|
| `clone()`    | 복사        | `let v2 = v.clone();`              | 깊은 복사. heap 데이터까지 복제됨         |
| `to_vec()`   | 복사        | `let v = slice.to_vec();`          | slice → Vec 변환. 복사 발생               |
| `into_iter()`| 소유권 이동 | `for x in v.into_iter()`           | `v`의 소유권을 소비하며 순회              |
| `iter()`     | 참조        | `for x in v.iter()`                | `&v`로 순회. 원본은 변경되지 않음         |
| `iter_mut()` | 가변 참조   | `for x in v.iter_mut()`            | `&mut v`로 순회. 원소 수정 가능           |
| `extend()`   | 복사/참조   | `v.extend(&other_vec);`            | 다른 컬렉션의 요소를 복사해 추가          |
| `drain()`    | 소유권 이동 | `for x in v.drain(..)`             | 요소를 꺼내면서 원본에서 제거             |
| `split_at()` | 참조 분할   | `let (a, b) = slice.split_at(2);`  | slice를 두 개로 나눔. 참조 기반           |
| `as_slice()` | 참조        | `v.as_slice()`                     | Vec → slice 변환. 참조 기반               |


## 🔍 고급 전달 방식
- Rc<Vec<T>>, Arc<Vec<T>> → 공유 참조
- Box<[T]> → heap에 고정된 array
- Cow<[T]> → 복사-on-write (읽기 전용 → 수정 시 복사)

## ✅ 결론
Rust에서 컬렉션 간 데이터 전달은 소유권, 참조, 복사, 가변성을 명확히 구분해야 안전하면서도 효율적인 구조를 만들 수 있음.


