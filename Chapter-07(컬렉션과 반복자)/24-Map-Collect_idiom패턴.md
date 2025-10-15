# iter().map(|(k, _)| k).collect::<Vec<_>>() 패턴 활용

`iter().map(|(k, _)| k).collect::<Vec<_>>()` 같은 패턴은 Vec나 배열에서도 충분히 활용 가능. 
다만 Vec과 배열은 (key, value) 쌍이 아니라 값만 있는 순차 컬렉션이기 때문에, map()의 대상이 달라질 뿐.

## ✅ Vec에서 map + collect 예제
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 각 요소를 제곱해서 새로운 Vec 생성
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();

    println!("{:?}", squares); // [1, 4, 9, 16, 25]
}
```

- iter() → 불변 참조
- map(|x| x * x) → 각 요소 처리
- collect() → 결과를 Vec으로 수집

## 🔄 Vec에서 인덱스 기반 수정 예제
```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    // 인덱스를 수집해서 수정
    for i in (0..numbers.len()).collect::<Vec<_>>() {
        numbers[i] *= 10;
    }
    println!("{:?}", numbers); // [10, 20, 30, 40, 50]
}
```

- 0..numbers.len() → 인덱스 생성
- collect::<Vec<_>>() → 복사본 확보 (수정 가능)
- for i in ... → 안전하게 수정

## 🧮 배열에서도 가능 (단, 고정 크기)
```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    let doubled: Vec<i32> = arr.iter().map(|x| x * 2).collect();

    println!("{:?}", doubled); // [2, 4, 6, 8, 10]
}
```
- 배열도 iter() 가능
- 결과는 Vec으로 수집  


## 🔄 배열에서 인덱스 기반 수정 예제
배열에서도 이 방식은 가능하지만 조건이 있음.  
핵심은 배열이 **가변(mutable)** 이어야 하고, numbers[i]처럼 인덱스로 접근할 수 있어야 한다는 점.
### ✅ 예제: 배열에서 map + collect로 인덱스 수정
```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];

    for i in (0..numbers.len()).collect::<Vec<_>>() {
        numbers[i] *= 10;
    }

    println!("{:?}", numbers); // [10, 20, 30, 40, 50]
}
```
🔍 설명
- `0..numbers.len()` → 인덱스 범위 생성
- `.collect::<Vec<_>>()` → 인덱스를 복사해 수정 가능
- `numbers[i] *= 10` → 배열 요소 수정

### ⚠️ 주의할 점
- 배열은 고정 크기이므로 push, remove 같은 동작은 불가능
- collect()는 인덱스를 복사해오는 것이므로 불변 순회 중 수정할 때 유용
- 이 방식은 Vec에서도 동일하게 작동합니다


### 💡 대안: iter_mut()로 더 간단하게
배열이나 Vec에서는 아래처럼 iter_mut()을 쓰는 게 더 일반적:
```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];

    for num in numbers.iter_mut() {
        *num *= 10;
    }

    println!("{:?}", numbers); // [10, 20, 30, 40, 50]
}
```

- 더 간결하고 안전한 방식
- collect() 없이 바로 수정 가능

## 🔍 요약
| 컬렉션 타입 | map + collect 사용 가능 | 수정 방식         | 삭제 방식         | 특징                         |
|-------------|--------------------------|--------------------|--------------------|------------------------------|
| Vec<T>      | ✅                        | 직접 인덱스 접근    | swap_remove, retain | 순서 보장, 유연한 구조         |
| [T; N]      | ✅ (iter()로 가능)        | 불가능 (불변 배열)  | 불가능             | 고정 크기, 제한적 활용         |
| Slab<T>     | ✅ (iter()로 키 추출)     | remove/insert      | remove(key)        | 인덱스 재사용, 빠른 삭제       |


---

## 🔍 왜 for + collect::<Vec<_>>()가 유리할까?
### ✅ 명시적인 인덱스
```rust
for i in (0..vec.len()).collect::<Vec<_>>() {
    // i는 usize, vec[i]로 직접 접근 가능
}
```
- 인덱스를 통해 이전/다음 요소, 짝수/홀수 조건, 조건부 삭제 등 다양한 로직을 쉽게 구현할 수 있음.
  
### ✅ 명확한 스코프
- for 루프는 스코프가 명확해서 루프 안에서 변수 선언, 조건 분기, break, continue 등을 자유롭게 사용할 수 있음.
- 반면 map()이나 for_each()는 표현식 기반이라 복잡한 흐름 제어에는 불리.

### 🧪 예시: 짝수 인덱스만 수정
```rust
fn main() {
    let mut vec = vec![10, 20, 30, 40, 50];
    for i in (0..vec.len()).collect::<Vec<_>>() {
        if i % 2 == 0 {
            vec[i] += 1;
        }
    }
    println!("{:?}", vec); // [11, 20, 31, 40, 51]
}
```
- 클로저로는 i를 직접 다루기 어렵지만, for 루프는 자유롭죠.

## 💡 결론

| 스타일        | 장점                                 | 단점                                 | 추천 상황                         |
|---------------|--------------------------------------|--------------------------------------|----------------------------------|
| for + index   | 명확한 흐름 제어, 조건 분기, 인덱스 접근 가능 | 코드가 약간 길어질 수 있음             | 복잡한 로직, 수정/삭제가 필요한 경우 |
| map / for_each| 간결하고 선언적, 함수형 스타일         | 인덱스 접근 불편, 흐름 제어 제한         | 단순 변환, 필터링, 수집 작업        |
----

# slab에 사용

slab을 순회하면서 각 항목을 수정하는 방식인데, map()과 collect()를 활용한 패턴이 조금 낯설 수 있음.  
아래에 단계별 설명과 함께 일반적인 예제 제공.

## 🧠 코드 분석: 단계별 설명
```rust
for key in slab.iter().map(|(k, _)| k).collect::<Vec<_>>() {
    slab[key] = Arc::new(MyData { value: slab[key].value * 10 });
}
```

## 🔍 단계별로 쪼개면:
- `slab.iter()`  
    → Slab의 모든 (key, value) 쌍을 반복자 형태로 반환.
- `.map(|(k, _)| k)`  
    → 각 (key, value) 쌍에서 `key` 만 추출.
- `.collect::<Vec<_>>()`  
    → 추출한 키들을 Vec<usize>로 수집.
- `for key in ...`  
    → 키들을 순회하면서 slab[key]에 접근.
- `slab[key] = Arc::new(...)`  
    → 해당 키의 값을 새로 할당 (수정).
## ✅ 왜 이렇게 쓰는가?
- `slab.iter()` 은 &self를 빌려오므로 직접 수정이 불가능.
- `collect()` 로 키를 복사해오면 이후에 `slab[key] = ...` 처럼 수정이 가능.
- 이 패턴은 불변 순회 → 수정을 안전하게 분리하는 데 유용.

## 🧪 일반적인 예제: 사용자 점수 수정
```rust
use slab::Slab;
use std::sync::Arc;

#[derive(Debug)]
struct User {
    name: String,
    score: u32,
}

fn main() {
    let mut users: Slab<Arc<User>> = Slab::new();
    // 초기 데이터 삽입
    users.insert(Arc::new(User { name: "Alice".into(), score: 50 }));
    users.insert(Arc::new(User { name: "Bob".into(), score: 70 }));
    users.insert(Arc::new(User { name: "Charlie".into(), score: 90 }));
    // 모든 사용자 점수 2배로 수정
    for key in users.iter().map(|(k, _)| k).collect::<Vec<_>>() {
        let old = &users[key];
        users[key] = Arc::new(User {
            name: old.name.clone(),
            score: old.score * 2,
        });
    }

    // 결과 출력
    for (key, user) in users.iter() {
        println!("{} (key {}): score = {}", user.name, key, user.score);
    }
}
```

## 🧩 팁
- 이 패턴은 HashMap, Vec, BTreeMap 등에서도 유사하게 활용 가능.
- collect::<Vec<_>>()는 불변 순회 중 수정이 필요할 때 매우 유용.
- Arc를 쓰는 경우에는 불변 객체를 새로 만들어 교체하는 방식이 일반적.

---
