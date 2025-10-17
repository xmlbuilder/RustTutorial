# Arc::ptr_eq 이해하기

Arc::ptr_eq는 Rust에서 두 Arc<T>가 같은 힙 객체를 가리키는지 확인할 때 사용하는 함수입니다.  
즉, 두 Arc가 내용이 같아 보여도, 실제로 같은 메모리 주소를 공유하는지를 판단함.  

## 🔍 왜 쓰나요?
- Arc<T>는 참조 카운트 기반 스마트 포인터라서, 여러 개가 같은 데이터를 공유할 수 있음.
- 하지만 == 연산자는 내용이 같은지만 비교하고, 같은 객체인지는 확인하지 않음.
- ptr_eq는 진짜 동일한 객체인지를 확인하는 데 사용됩니다.

## ✅ 사용 예시
```rust
use std::sync::Arc;

#[derive(Debug)]
struct Data {
    value: i32,
}

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

Arc::ptr_eq를 실전에서 어떻게 활용할 수 있는지, 트리 구조에서 객체를 삽입하고 삭제하는 예제.  
특히 SpatialTree 같은 구조에서 객체의 동일성 판단에 매우 유용합니다.

## 🧩 시나리오: 공간 트리에 객체 삽입 후 삭제
- 우리는 SpatialTree라는 공간 인덱스 구조를 가지고 있고,
- 여기에 GameObject를 Arc로 감싸서 삽입합니다.
- 나중에 삭제할 때, 같은 객체인지 확인하려면 Arc::ptr_eq를 사용합니다.

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
- Arc::clone()을 여러 번 해도 내용은 같지만 객체는 동일하다는 보장이 필요할 때,
- ==는 내용 비교이므로, 동일한 객체인지 판단하려면 ptr_eq를 써야 해요.
- 특히 ECS(Entity-Component-System), 트리, 그래프, 캐시, 풀 구조에서 객체 식별에 매우 유용합니다.

---
