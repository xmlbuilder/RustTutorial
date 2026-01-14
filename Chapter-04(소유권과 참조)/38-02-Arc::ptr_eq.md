# Arc::ptr_eq 이해하기

- `Arc::ptr_eq` 는 Rust에서 두 Arc<T>가 같은 힙 객체를 가리키는지 확인할 때 사용하는 함수입니다.  
- 즉, 두 Arc가 내용이 같아 보여도, 실제로 같은 메모리 주소를 공유하는지를 판단함.  

## 🔍 왜 쓰나요?
- Arc<T>는 참조 카운트 기반 스마트 포인터라서, 여러 개가 같은 데이터를 공유할 수 있음.
- 하지만 `==` 연산자는 내용이 같은지만 비교하고, 같은 객체인지는 확인하지 않음.
- `ptr_eq` 는 진짜 동일한 객체인지를 확인하는 데 사용됩니다.

## 사용법
- `Arc::ptr_eq(&a, &c)`

## ✅ 사용 예시
```rust
use std::sync::Arc;

#[derive(Debug)]
struct Data {
    value: i32,
}
```
```rust
fn main() {
    let a = Arc::new(Data { value: 42 });
    let b = Arc::clone(&a); // b는 a와 같은 객체를 가리킴
    let c = Arc::new(Data { value: 42 }); // c는 내용은 같지만 다른 객체

    assert!(Arc::ptr_eq(&a, &b)); // true: 같은 객체
    assert!(!Arc::ptr_eq(&a, &c)); // false: 다른 객체
}
```

## 🧠 언제 유용할까?
- 트리나 그래프 구조에서 노드 식별할 때
- 캐시나 풀에서 객체 재사용 여부 판단할 때
- 업데이트/삭제 시 동일 객체인지 확인할 때

---

# 실전 예제

`Arc::ptr_eq`를 실전에서 어떻게 활용할 수 있는지, 트리 구조에서 객체를 삽입하고 삭제하는 예제.  
특히 SpatialTree 같은 구조에서 객체의 동일성 판단에 매우 유용합니다.

## 🧩 시나리오: 공간 트리에 객체 삽입 후 삭제
- 우리는 SpatialTree라는 공간 인덱스 구조를 가지고 있고,
- 여기에 GameObject를 Arc로 감싸서 삽입합니다.
- 나중에 삭제할 때, 같은 객체인지 확인하려면 `Arc::ptr_eq` 를 사용합니다.

## 🧪 실전 예제
```rust
use std::sync::Arc;

#[derive(Debug)]
struct GameObject {
    name: String,
}

struct SpatialTree {
    objects: Vec<Arc<GameObject>>,
}

impl SpatialTree {
    fn new() -> Self {
        Self { objects: Vec::new() }
    }

    fn insert(&mut self, obj: Arc<GameObject>) {
        self.objects.push(obj);
    }

    fn remove(&mut self, target: &Arc<GameObject>) {
        self.objects.retain(|obj| !Arc::ptr_eq(obj, target));
    }

    fn list(&self) {
        for obj in &self.objects {
            println!("Object: {}", obj.name);
        }
    }
}

fn main() {
    let mut tree = SpatialTree::new();

    let obj1 = Arc::new(GameObject { name: "Player".into() });
    let obj2 = Arc::new(GameObject { name: "Enemy".into() });

    tree.insert(obj1.clone());
    tree.insert(obj2.clone());

    println!("Before removal:");
    tree.list();

    // Remove obj1 using ptr_eq
    tree.remove(&obj1);

    println!("\nAfter removal:");
    tree.list();
}
```


### ✅ 결과
```
Before removal:
Object: Player
Object: Enemy

After removal:
Object: Enemy
```

## 🔍 왜 ptr_eq가 중요한가?
- `Arc::clone()` 을 여러 번 해도 내용은 같지만 객체는 동일하다는 보장이 필요할 때,
- `==` 는 내용 비교이므로, 동일한 객체인지 판단하려면 `ptr_eq 를 써야 해요.
- 특히 ECS(Entity-Component-System), 트리, 그래프, 캐시, 풀 구조에서 객체 식별에 매우 유용합니다.

---

## 🔍 다시 정리하면
```rust
let obj = Arc::new(GameObject { name: "Player".into() });
tree.insert(obj.clone()); // 같은 Arc → 같은 포인터
tree.insert(obj.clone()); // 또 같은 Arc → 같은 포인터

tree.remove(&obj); // → 둘 다 제거됨 (같은 포인터)
```

> ✅ 이 경우엔 Arc::ptr_eq로 비교하면 둘 다 같은 포인터니까 한 번의 remove로 둘 다 제거.

## 하지만 아래처럼 내용은 같지만 다른 Arc를 넣으면?
```rust
let obj1 = Arc::new(GameObject { name: "Player".into() });
let obj2 = Arc::new(GameObject { name: "Player".into() }); // 다른 Arc, 같은 내용

tree.insert(obj1.clone());
tree.insert(obj2.clone());

tree.remove(&obj1); // → obj1만 제거됨, obj2는 남음
```

> ❌ 이 경우엔 Arc::ptr_eq(&obj1, &obj2)는 false이므로, remove는 obj1만 제거하고 obj2는 남아요.



## ✅ 해결 방법
### 1. 삽입할 때 받은 Arc를 저장해두기
```rust
let obj = Arc::new(GameObject { name: "Player".into() });
tree.insert(obj.clone());

// 나중에 정확히 같은 Arc로 제거
tree.remove(&obj);
```

### 2. 내용 기반 비교로 바꾸기 (주의: 성능·의도 차이)
```rust
fn remove_by_value(&mut self, target: &GameObject) {
    self.objects.retain(|obj| obj.as_ref() != target);
}
```
- 이건 PartialEq 기반 비교라서 내용이 같으면 제거됨
- 하지만 Arc::ptr_eq처럼 정확한 동일 객체 식별은 안 됨

| 비교 방식         | 기준             | 제거 범위                          | 사용 목적                          |
|------------------|------------------|------------------------------------|------------------------------------|
| `Arc::ptr_eq`     | 포인터 동일성     | 같은 `Arc` 인스턴스만 제거됨       | 객체 식별, 핸들 기반 트리/캐시     |
| `==` (`PartialEq`) | 내용 동일성       | 내용이 같은 모든 객체 제거 가능    | 의미적 중복 제거, 데이터 정리       |

## 🧠 왜 insert 중복 검사가 필요할까?
- 불필요한 메모리 낭비: 같은 객체가 여러 번 들어가면 공간 낭비
- 삭제/업데이트 시 혼란: 어떤 걸 지워야 할지 모호해짐
- 의도하지 않은 중복 처리: 시스템이 같은 객체를 여러 번 처리할 수 있음

## ✅ 해결 방법: insert()에서 중복 검사
### 1. Arc::ptr_eq 기반 중복 검사
```rust
fn insert(&mut self, obj: Arc<GameObject>) {
    if self.objects.iter().any(|o| Arc::ptr_eq(o, &obj)) {
        // 이미 같은 객체가 들어있음
        return;
    }
    self.objects.push(obj);
}
```

- **정확히 같은 객체(같은 포인터)** 만 막음
- 내용이 같지만 다른 객체는 허용

###  2. PartialEq 기반 내용 중복 검사
```rust
fn insert_by_value(&mut self, obj: Arc<GameObject>) {
    if self.objects.iter().any(|o| o.as_ref() == obj.as_ref()) {
        // 내용이 같은 객체가 이미 있음
        return;
    }
    self.objects.push(obj);
}
```

- 내용이 같으면 막음
- 포인터가 달라도 내용이 같으면 중복으로 간주

## 🧩 어떤 방식이 더 좋을까?
| 기준 항목         | Arc::ptr_eq                         | PartialEq (`==`)                    |
|------------------|--------------------------------------|-------------------------------------|
| 비교 기준         | 포인터 동일성                        | 내용 동일성                         |
| 제거 범위         | 같은 `Arc` 인스턴스만 제거됨         | 내용이 같은 모든 객체 제거 가능     |
| 중복 삽입 방지     | 같은 객체만 막을 수 있음              | 내용이 같으면 모두 막을 수 있음     |
| 성능              | 매우 빠름 (주소 비교)                | 느릴 수 있음 (내용 비교)            |
| 용도              | 핸들 기반 트리, 캐시, 객체 식별용     | 의미적 중복 제거, 데이터 정리용     |

---



