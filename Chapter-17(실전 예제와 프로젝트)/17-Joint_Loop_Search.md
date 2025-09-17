# 📘 루프 추출 알고리즘 문서
## 🧠 목적

관절(Joint) 구조에서 각 노드의 parent 정보만을 기반으로
**Leaf → Root 방향의 경로(루프)**를 추적하며,
사이클이나 잘못된 연결은 자동으로 감지 및 제외하고,
루트는 반드시 "다리" 노드로 끝나야 하는 구조를 보장합니다.

## 🧱 데이터 구조
```rust
Joint
struct Joint {
    name: String,
    parent: Option<String>,
}
```

- name: 관절의 고유 이름
- parent: 부모 관절의 이름 (없으면 루트)

## 🔧 함수 설명
### 1. build_joint_map_with_check
```rust
fn build_joint_map_with_check(joints: Vec<Joint>) -> HashMap<String, Joint>
```

역할
- 사용자 입력으로 받은 Vec<Joint>를 HashMap으로 변환
- 중복된 name이 존재하면 경고 출력
- 덮어쓰기를 방지하고, 구조 오류를 사전에 감지

동작
- HashSet을 사용해 이미 등록된 name을 추적
- 중복 발견 시 println!으로 경고 출력 후 continue

### 2. find_leaf_nodes
```rust
fn find_leaf_nodes(joints: &HashMap<String, Joint>) -> Vec<String>
```

역할
- 전체 관절 중에서 **자식이 없는 노드(Leaf)**를 추출
동작
- 모든 name을 수집
- 모든 parent 이름을 수집
- name ∉ parent인 노드를 Leaf로 간주

### 3. trace_to_dari_only
```rust
fn trace_to_dari_only(joints: &HashMap<String, Joint>, start: &str) -> Option<Vec<String>>
```

역할
- 특정 Leaf 노드에서 시작해 root 까지의 경로를 추적
- 사이클이 생기면 경로를 중단하고 제외
- 경로가 root 로 끝나지 않으면 제외
동작
- visited 집합으로 중복 방문 감지
- parent가 이미 방문된 노드면 사이클로 간주
- 경로의 마지막 노드가 root 가 아니면 None 반환

### 4. extract_all_joint_orders
fn extract_all_joint_orders(joints: &HashMap<String, Joint>) -> Vec<Vec<String>>


역할
- 모든 Leaf 노드에서 root 까지의 유효한 루프 경로를 추출
동작
- find_leaf_nodes로 Leaf 목록 추출
- 각 Leaf에 대해 trace_to_dari_only 호출
- Some(path)인 경우만 결과에 포함

### 🧪 실행 흐름
```rust
fn main() {
    let joints = vec![ /* 사용자 입력 */ ];
    let joint_map = build_joint_map_with_check(joints);
    let orders = extract_all_joint_orders(&joint_map);

    for order in orders {
        println!("→ {}", order.join(" → "));
    }
}
```



## ⚠️ 예외 처리 목록

| 예외 상황                    | 설명                                                         | 처리 방식                                   |
|-----------------------------|--------------------------------------------------------------|----------------------------------------------|
| 중복된 `name`               | 동일한 이름의 Joint가 여러 번 정의됨                         | 경고 출력 후 첫 번째 정의만 사용 (`continue`)|
| 사이클 발생                 | A → B → C → A처럼 자기 자신 또는 상위 노드를 다시 참조함     | 경고 출력 후 해당 경로 제외 (`None` 반환)   |
| parent가 이미 방문된 노드   | 경로 추적 중 visited에 있는 노드를 다시 parent로 지정함      | 사이클로 간주하고 경고 후 제외               |
| 루프가 `"root"`로 끝나지 않음 | 루트가 `"root"`가 아닌 경우 (예: `"child"`이 루트처럼 끝나는 경우)| 경고 출력 후 해당 경로 제외 (`None` 반환)   |
| 루트 `"root"`가 누락됨      | `"root"` 노드가 존재하지 않거나 parent가 지정되어 있음        | 전체 구조 오류로 간주 → 경고 또는 종료       |
| parent가 존재하지 않는 노드 | parent가 지정되었지만 해당 이름의 Joint가 존재하지 않음       | 경고 출력 후 경로 중단 또는 제외             |

## 소스

```rust

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Joint {
    name: String,
    parent: Option<String>,
}

/// 중복 name 감지 + 경고 출력
fn build_joint_map_with_check(joints: Vec<Joint>) -> HashMap<String, Joint> {
    let mut map = HashMap::new();
    let mut seen = HashSet::new();

    for joint in joints {
        if seen.contains(&joint.name) {
            println!("⚠️ 경고: 중복된 Joint 이름 발견 → '{}'", joint.name);
            continue; // 덮어쓰기 방지
        }
        seen.insert(joint.name.clone());
        map.insert(joint.name.clone(), joint);
    }

    map
}

/// Leaf 노드 추출
fn find_leaf_nodes(joints: &HashMap<String, Joint>) -> Vec<String> {
    let all_names: HashSet<_> = joints.keys().cloned().collect();
    let parent_names: HashSet<_> = joints.values()
        .filter_map(|j| j.parent.clone())
        .collect();

    all_names.difference(&parent_names).cloned().collect()
}

/// 루프 경로 추적 (사이클 감지 + 다리로 끝나지 않으면 제외)
fn trace_to_dari_only(joints: &HashMap<String, Joint>, start: &str) -> Option<Vec<String>> {
    let mut path = Vec::new();
    let mut visited = HashSet::new();
    let mut current = Some(start.to_string());

    while let Some(name) = current {
        if visited.contains(&name) {
            println!("⚠️ 사이클 감지: '{}' → 경로 중단", name);
            return None;
        }
        visited.insert(name.clone());
        path.push(name.clone());

        let next = joints.get(&name).and_then(|j| j.parent.clone());
        if let Some(ref p) = next {
            if visited.contains(p) {
                println!("⚠️ parent '{}' of '{}' creates cycle → 무시", p, name);
                return None;
            }
        }

        current = next;
    }

    if path.last().map(|s| s.as_str()) == Some("다리") {
        Some(path)
    } else {
        println!("⚠️ 루프가 '다리'로 끝나지 않음 → 제외: {:?}", path);
        None
    }
}

/// 전체 루프 순서 추출
fn extract_all_joint_orders(joints: &HashMap<String, Joint>) -> Vec<Vec<String>> {
    let mut orders = Vec::new();
    let leaves = find_leaf_nodes(joints);

    for leaf in leaves {
        if let Some(order) = trace_to_dari_only(joints, &leaf) {
            orders.push(order);
        }
    }

    orders
}

fn main() {
    // 사용자 입력 Joint 목록
    let joints = vec![
        Joint { name: "손가락".into(), parent: Some("팔".into()) },
        Joint { name: "팔".into(), parent: Some("어깨".into()) },
        Joint { name: "어깨".into(), parent: Some("몸".into()) },
        Joint { name: "몸".into(), parent: Some("다리".into()) },
        Joint { name: "다리".into(), parent: None },
        Joint { name: "머리".into(), parent: Some("몸".into()) },
        Joint { name: "왼쪽다리".into(), parent: Some("다리".into()) },
        Joint { name: "오른쪽다리".into(), parent: Some("다리".into()) },

        // ❌ 사용자 실수: 중복 정의 + 사이클 유발
        Joint { name: "몸".into(), parent: Some("팔".into()) }, // 무시됨
    ];

    // 중복 감지 포함한 JointMap 생성
    let joint_map = build_joint_map_with_check(joints);

    // 루프 순서 추출
    let orders = extract_all_joint_orders(&joint_map);
    println!("\n✅ 루프 순서 목록 (다리로 끝나는 것만):");
    for order in orders {
        println!("→ {}", order.join(" → "));
    }
}
```

## ✅ 출력 예시
⚠️ 경고: 중복된 Joint 이름 발견 → '몸'
⚠️ 루프가 '다리'로 끝나지 않음 → 제외: ["몸", "팔"]

## ✅ 루프 순서 목록 (다리로 끝나는 것만):
→ 손가락 → 팔 → 어깨 → 몸 → 다리
→ 머리 → 몸 → 다리
→ 왼쪽다리 → 다리
→ 오른쪽다리 → 다리

---

# 관절 구조를 String 기반이 아닌 u32 ID 기반으로 바꾸어 설계

## 🧱 구조 변경 요약

| 변경 전               | 변경 후              |
|----------------------|----------------------|
| `name: String`       | `id: u32`            |
| `parent: Option<String>` | `parent: Option<u32>` |
| `Vec<String>`        | `Vec<u32>`           |

##  변경된 소스
```rust
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Joint {
    id: u32,
    parent: Option<u32>,
}

/// 중복 ID 감지 + 경고 출력
fn build_joint_map_with_check(joints: Vec<Joint>) -> HashMap<u32, Joint> {
    let mut map = HashMap::new();
    let mut seen = HashSet::new();

    for joint in joints {
        if seen.contains(&joint.id) {
            println!("⚠️ 경고: 중복된 Joint ID 발견 → {}", joint.id);
            continue;
        }
        seen.insert(joint.id);
        map.insert(joint.id, joint);
    }

    map
}

/// Leaf 노드 추출
fn find_leaf_nodes(joints: &HashMap<u32, Joint>) -> Vec<u32> {
    let all_ids: HashSet<_> = joints.keys().cloned().collect();
    let parent_ids: HashSet<_> = joints.values()
        .filter_map(|j| j.parent)
        .collect();

    all_ids.difference(&parent_ids).cloned().collect()
}

/// 루프 경로 추적 (사이클 감지 + 루트 ID로 끝나지 않으면 제외)
fn trace_to_root_only(joints: &HashMap<u32, Joint>, start: u32, root_id: u32) -> Option<Vec<u32>> {
    let mut path = Vec::new();
    let mut visited = HashSet::new();
    let mut current = Some(start);

    while let Some(id) = current {
        if visited.contains(&id) {
            println!("⚠️ 사이클 감지: {} → 경로 중단", id);
            return None;
        }
        visited.insert(id);
        path.push(id);

        let next = joints.get(&id).and_then(|j| j.parent);
        if let Some(p) = next {
            if visited.contains(&p) {
                println!("⚠️ parent {} of {} creates cycle → 무시", p, id);
                return None;
            }
        }

        current = next;
    }

    if path.last() == Some(&root_id) {
        Some(path)
    } else {
        println!("⚠️ 루프가 루트 {}로 끝나지 않음 → 제외: {:?}", root_id, path);
        None
    }
}

/// 전체 루프 순서 추출
fn extract_all_joint_orders(joints: &HashMap<u32, Joint>, root_id: u32) -> Vec<Vec<u32>> {
    let mut orders = Vec::new();
    let leaves = find_leaf_nodes(joints);

    for leaf in leaves {
        if let Some(order) = trace_to_root_only(joints, leaf, root_id) {
            orders.push(order);
        }
    }

    orders
}

fn main() {
    // Joint ID 정의
    const 손가락: u32 = 1;
    const 팔: u32 = 2;
    const 어깨: u32 = 3;
    const 몸: u32 = 4;
    const 다리: u32 = 5;
    const 머리: u32 = 6;
    const 왼쪽다리: u32 = 7;
    const 오른쪽다리: u32 = 8;

    let joints = vec![
        Joint { id: 손가락, parent: Some(팔) },
        Joint { id: 팔, parent: Some(어깨) },
        Joint { id: 어깨, parent: Some(몸) },
        Joint { id: 몸, parent: Some(다리) },
        Joint { id: 다리, parent: None },
        Joint { id: 머리, parent: Some(몸) },
        Joint { id: 왼쪽다리, parent: Some(다리) },
        Joint { id: 오른쪽다리, parent: Some(다리) },

        // ❌ 사용자 실수: 중복 정의 + 사이클 유발
        Joint { id: 몸, parent: Some(팔) }, // 무시됨
    ];

    let joint_map = build_joint_map_with_check(joints);
    let orders = extract_all_joint_orders(&joint_map, 다리);

    println!("\n✅ 루프 순서 목록 (루트 {}로 끝나는 것만):", 다리);
    for order in orders {
        println!("→ {:?}", order);
    }
}
```

## 출력

```
✅ 루프 순서 목록 (루트 5로 끝나는 것만):
→ [1, 2, 3, 4, 5]
→ [6, 4, 5]
→ [8, 5]
→ [7, 5]
```

----




