# SpatialTree
SpatialTree.md 문서를 기반으로 전체 구조를 정리하고, 주요 함수들의 동작과 관련 수식.  
코드 검토도 포함해서 논리 흐름과 안전성까지 점검.

## 🌲 SpatialTree 구조 요약
### 📐 핵심 타입
| 타입 이름         | 설명                                      |
|------------------|-------------------------------------------|
| `HasAABB`          | 객체가 AABB(BoundingBox)를 제공하는 트레이트 |
| `BoundingBox`      | 공간 내 경계 박스를 나타내는 구조체         |
| `TreeNode<T>`      | AABB, 객체 리스트, 자식 노드를 포함하는 트리 노드 |
| `SpatialTree<T>`   | 루트 노드와 최대 객체 수를 포함한 공간 분할 트리 |
| `SpatialTreeStats` | 트리 통계 정보: 노드 수, 리프 수, 깊이, 객체 수 등 |

### 🔧 주요 메서드 기능
| 메서드 이름                 | 설명                                                             |
|----------------------------|------------------------------------------------------------------|
| `insert`                     | 객체를 트리에 삽입. 루트 밖이면 AABB 확장 후 재귀 삽입          |
| `query`                      | AABB 범위 내 객체를 탐색하여 결과 벡터에 추가                    |
| `intersects_box`             | 주어진 AABB와 교차하는 객체가 있는지 여부 반환                  |
| `remove_arc`                 | `Arc::ptr_eq`로 객체를 식별하여 트리에서 제거                    |
| `update`                     | 객체를 제거 후 다시 삽입하여 AABB 갱신                          |
| `ray_query`                  | 광선과 AABB 교차하는 객체를 탐색하여 결과 벡터에 추가           |
| `traverse_nodes`             | DFS 방식으로 모든 노드를 순회                                    |
| `traverse_nodes_and_objects`| 노드와 그에 속한 객체를 모두 순회                               |
| `stats`                      | 트리의 통계 정보 계산 (노드 수, 리프 수, 깊이 등)               |
| `insert_recursive`           | 리프에 객체 삽입 또는 분할 수행                                 |
| `split_node`                 | 가장 긴 축 기준으로 노드를 2분할                                |
| `query_recursive`            | 재귀적으로 AABB 범위 내 객체 탐색                               |
| `ray_query_recursive`        | 재귀적으로 광선과 교차하는 객체 탐색                            |
| `ray_aabb_intersect`         | 광선과 AABB의 교차 여부 계산 (slab 방식)                         |

## ✏️ 수식 표현
### 1. AABB 분할 기준
가장 긴 축을 기준으로 중간값 계산:

$$
\mathrm{mid}=\frac{\mathrm{min}[axis]+\mathrm{max}[axis]}{2}
$$

## 2. 평균 객체 수
### 리프당 평균 객체 수:

$$
\mathrm{avg\\_ objs\\_ per\\_ leaf}=\frac{\mathrm{objects}}{\mathrm{leaves}}
$$

## 3. Ray-AABB 교차 판정
### 각 축에 대해 slab 교차 범위 계산:

$$
t_a=\frac{mn-ro_x}{rd_x},\quad t_b=\frac{mx-ro_x}{rd_x}
$$

### 정렬 후:

$$
t_0=\max (t_0,\min (t_a,t_b)),\quad t_1=\min (t_1,\max (t_a,t_b))
$$

### 최종 교차 조건:

$$
t_1\geq t_0\quad \mathrm{and}\quad t_1\geq 0
$$

## 📦 문맥 속에서의 mn, mx
```rust
fn check_axis(ro: f64, rd: f64, mn: f64, mx: f64, t0: &mut f64, t1: &mut f64) -> bool
```
- 이 함수는 **광선(ray)** 이 AABB의 한 축(x, y, z)에서 **슬래브(slab)**와 교차하는지를 검사하는 부분.
    - ro: ray origin (광선 시작점)의 해당 축 좌표
    - rd: ray direction (광선 방향)의 해당 축 성분
    - mn: AABB의 해당 축에서의 최솟값 (min bound)
    - mx: AABB의 해당 축에서의 최댓값 (max bound)

### 📐 예시
예를 들어 x축에서:  
- AABB의 x 범위가 $[1.0,3.0]$ 이면
    - mn = 1.0, mx = 3.0
- 광선이 ro_x=0.0에서 시작해서 rd_x=1.0 방향으로 진행하면
    - 광선이 x축에서 AABB와 교차하는지 확인할 수 있음

### 🧠 왜 필요한가?
이 값들을 이용해 **광선과 AABB의 교차 구간 $[t_0, t_1]$** 을 계산:  

$$
t_a=\frac{mn-ro}{rd},\quad t_b=\frac{mx-ro}{rd}
$$

$$
t_0=\max (t_0,\min (t_a,t_b)),\quad t_1=\min (t_1,\max (t_a,t_b))
$$

이걸 x, y, z축 각각에 대해 반복해서  
모든 축에서 교차해야 최종적으로 AABB와 광선이 교차한다고 판단할 수 있음.


## ✅ 코드 검토 요약
- insert_recursive와 split_node는 객체 분할 로직이 명확하고 안전하게 구성됨
- Arc<T>를 사용한 객체 공유는 ptr_eq로 정확히 비교됨
- BoundingBox 관련 메서드는 외부 모듈에 의존하므로 해당 구현이 필요함
- ray_aabb_intersect는 slab 방식으로 정확한 광선 교차 판정을 수행함
- stats()는 리프 히스토그램과 평균 계산까지 포함되어 있어 분석에 유용함

---

## 소스 코드
```rust
use crate::math::boundingbox::BoundingBox;
use crate::math::prelude::Point3D;
use crate::math::vector3d::Vector3D;
use std::sync::Arc;

pub trait HasAABB: Send + Sync {
    fn aabb(&self) -> BoundingBox;
}
```
```rust
#[derive(Debug, Default, Clone)]
pub struct SpatialTreeStats {
    pub nodes: usize,
    pub leaves: usize,
    pub max_depth: usize,
    pub objects: usize,
    pub parent_stored: usize,
    pub avg_objs_per_leaf: f64,
    pub leaf_hist: Vec<usize>, // index = leaf에 든 객체 개수, 값 = 그런 리프 수
}
```
```rust
#[derive(Debug)]
pub struct TreeNode<T: HasAABB> {
    pub bbox: BoundingBox,
    pub objects: Vec<Arc<T>>, // 부모 보관 패턴 가능
    pub child1: Option<Box<TreeNode<T>>>,
    pub child2: Option<Box<TreeNode<T>>>,
}
```
```rust
impl<T: HasAABB> TreeNode<T> {
    fn is_leaf(&self) -> bool {
        self.child1.is_none() && self.child2.is_none()
    }

    fn bbox_obj_count(&self) -> usize {
        self.objects.len()
    }
}
```
```rust
#[derive(Debug)]
pub struct SpatialTree<T: HasAABB> {
    pub root: Box<TreeNode<T>>,
    pub max_objects: usize, // 리프가 이 개수를 넘으면 분할
}
```
```rust
impl<T: HasAABB> SpatialTree<T> {
    pub fn new(root_bbox: BoundingBox) -> Self {
        Self {
            root: Box::new(TreeNode {
                bbox: root_bbox,
                objects: Vec::new(),
                child1: None,
                child2: None,
            }),
            max_objects: 10,
        }
    }
```
```rust
    pub fn root_bbox(&self) -> &BoundingBox {
        &self.root.bbox
    }
```
```rust    
    pub fn root_bbox_mut(&mut self) -> &mut BoundingBox {
        &mut self.root.bbox
    }
```
```rust
    pub fn expand_to_include(&mut self, new_bbox: &BoundingBox) {
        self.root.bbox.union_mut(new_bbox);
    }
```
```rust
    /// Insert(bbox, obj) + 루트 밖이면 확장
    pub fn insert(&mut self, obj: Arc<T>) {
        let bb = obj.aabb();
        if !self.root.bbox.includes(&bb) {
            // 루트 박스 확장
            self.root.bbox.union_with(&bb);
        }
        let root = &mut self.root; // 루트만 &mut 대여
        Self::insert_recursive(root, obj, self.max_objects);
    }
```
```rust
    pub fn query(&self, q: &BoundingBox, out: &mut Vec<Arc<T>>) {
        self.query_recursive(&self.root, q, out);
    }
```
```rust
    pub fn intersects_box(&self, q: &BoundingBox) -> bool {
        let mut hit = false;
        self.traverse_nodes(|n| {
            if !BoundingBox::intersects(&n.bbox, q) {
                return;
            }
            if n.objects
                .iter()
                .any(|o| BoundingBox::intersects(&o.aabb(), q))
            {
                hit = true;
            }
        });
        hit
    }
```
```rust
    pub fn remove_arc(&mut self, target: &Arc<T>) -> bool {
        let mut removed = false;
        fn walk<T: HasAABB>(n: &mut TreeNode<T>, target: &Arc<T>, removed: &mut bool) {
            n.objects.retain(|o| {
                let keep = !Arc::ptr_eq(o, target);
                if !keep {
                    *removed = true;
                }
                keep
            });
            if let Some(c1) = n.child1.as_mut() {
                walk(c1, target, removed);
            }
            if let Some(c2) = n.child2.as_mut() {
                walk(c2, target, removed);
            }
        }
        walk(&mut self.root, target, &mut removed);
        removed
    }
```
```rust
    pub fn update(&mut self, target: &Arc<T>) {
        // 단순 정책: 지우고 다시 삽입(객체가 aabb()에서 최신 박스를 제공한다고 가정)
        if self.remove_arc(target) {
            self.insert(target.clone());
        } else {
            // 못 찾았으면 그냥 삽입
            self.insert(target.clone());
        }
    }
```
```rust
    pub fn ray_query(
        &self,
        ro: Point3D,
        rd: Vector3D,
        tmin: f64,
        tmax: f64,
        out: &mut Vec<Arc<T>>,
    ) {
        out.clear();
        self.ray_query_recursive(&self.root, ro, rd, tmin, tmax, out);
    }
```
```rust
    pub fn traverse_nodes<F: FnMut(&TreeNode<T>)>(&self, mut f: F) {
        fn dfs<T: HasAABB, F: FnMut(&TreeNode<T>)>(n: &TreeNode<T>, f: &mut F) {
            f(n);
            if let Some(c1) = n.child1.as_ref() {
                dfs(c1, f);
            }
            if let Some(c2) = n.child2.as_ref() {
                dfs(c2, f);
            }
        }
        dfs(&self.root, &mut f);
    }
```
```rust
    pub fn traverse_nodes_and_objects<FN, FO>(&self, mut node_cb: FN, mut obj_cb: FO)
    where
        FN: FnMut(&TreeNode<T>),
        FO: FnMut(&Arc<T>),
    {
        fn dfs<T: HasAABB, FN, FO>(n: &TreeNode<T>, node_cb: &mut FN, obj_cb: &mut FO)
        where
            FN: FnMut(&TreeNode<T>),
            FO: FnMut(&Arc<T>),
        {
            node_cb(n);
            for o in &n.objects {
                obj_cb(o);
            }
            if let Some(c1) = n.child1.as_ref() {
                dfs(c1, node_cb, obj_cb);
            }
            if let Some(c2) = n.child2.as_ref() {
                dfs(c2, node_cb, obj_cb);
            }
        }
        dfs(&self.root, &mut node_cb, &mut obj_cb);
    }
```
```rust
    pub fn stats(&self) -> SpatialTreeStats {
        let mut s = SpatialTreeStats::default();

        self.traverse_nodes(|n| {
            s.nodes += 1;
            let c = n.bbox_obj_count();
            s.objects += c;
            if n.is_leaf() {
                s.leaves += 1;
                if s.leaf_hist.len() <= c {
                    s.leaf_hist.resize(c + 1, 0);
                }
                s.leaf_hist[c] += 1;
            } else {
                s.parent_stored += c;
            }
        });
```
```rust
        // 최대 깊이
        fn depth<T: HasAABB>(n: &TreeNode<T>, d: usize) -> usize {
            if n.is_leaf() {
                d
            } else {
                let d1 = n.child1.as_ref().map(|c| depth(c, d + 1)).unwrap_or(d);
                let d2 = n.child2.as_ref().map(|c| depth(c, d + 1)).unwrap_or(d);
                d1.max(d2)
            }
        }
        s.max_depth = depth(&self.root, 1);
        s.avg_objs_per_leaf = if s.leaves > 0 {
            s.objects as f64 / s.leaves as f64
        } else {
            0.0
        };
        s
    }
```
```rust
    fn insert_recursive(node: &mut TreeNode<T>, obj: Arc<T>, max_objects: usize) {
        if node.is_leaf() {
            node.objects.push(obj);
            if node.objects.len() > max_objects {
                Self::split_node(node); // ⬅️ self 없이 호출
            }
            return;
        }

        let bb = obj.aabb();
        if let Some(c1) = node.child1.as_mut() {
            if c1.bbox.includes(&bb) {
                Self::insert_recursive(c1, obj, max_objects);
                return;
            }
        }
        if let Some(c2) = node.child2.as_mut() {
            if c2.bbox.includes(&bb) {
                Self::insert_recursive(c2, obj, max_objects);
                return;
            }
        }
        // 자식 둘 다 못 담으면 부모에 보관(부모 보관 패턴)
        node.objects.push(obj);
    }
```
```rust
    fn split_node(node: &mut TreeNode<T>) {
        // 가장 긴 축 기준으로 2분할
        let axis = node.bbox.max_extent_axis(); // 네 boundingbox.rs에 맞춰 구현되어 있을 거라 가정
        let mid = 0.5 * (node.bbox.min()[axis] + node.bbox.max()[axis]);

        let mut b1 = node.bbox;
        let mut b2 = node.bbox;
        b1.max_mut(axis, mid);
        b2.min_mut(axis, mid);

        let mut c1 = Box::new(TreeNode {
            bbox: b1,
            objects: Vec::new(),
            child1: None,
            child2: None,
        });
        let mut c2 = Box::new(TreeNode {
            bbox: b2,
            objects: Vec::new(),
            child1: None,
            child2: None,
        });

        // 기존 오브젝트 재분배
        let old = std::mem::take(&mut node.objects);
        for o in old {
            let bb = o.aabb();
            if c1.bbox.includes(&bb) {
                c1.objects.push(o);
            } else if c2.bbox.includes(&bb) {
                c2.objects.push(o);
            } else {
                node.objects.push(o);
            } // 둘 다 못 담으면 부모에 남김
        }

        node.child1 = Some(c1);
        node.child2 = Some(c2);
    }
```
```rust
    fn query_recursive(&self, node: &TreeNode<T>, q: &BoundingBox, out: &mut Vec<Arc<T>>) {
        if !BoundingBox::intersects(&node.bbox, q) {
            return;
        }

        for o in &node.objects {
            if BoundingBox::intersects(&o.aabb(), q) {
                out.push(o.clone());
            }
        }
        if let Some(c1) = node.child1.as_ref() {
            self.query_recursive(c1, q, out);
        }
        if let Some(c2) = node.child2.as_ref() {
            self.query_recursive(c2, q, out);
        }
    }
```
```rust
    fn ray_query_recursive(
        &self,
        node: &TreeNode<T>,
        ro: Point3D,
        rd: Vector3D,
        tmin: f64,
        tmax: f64,
        out: &mut Vec<Arc<T>>,
    ) {
        if !Self::ray_aabb_intersect(ro, rd, &node.bbox, tmin, tmax) {
            return;
        }

        for o in &node.objects {
            if Self::ray_aabb_intersect(ro, rd, &o.aabb(), tmin, tmax) {
                out.push(o.clone());
            }
        }
        if let Some(c1) = node.child1.as_ref() {
            self.ray_query_recursive(c1, ro, rd, tmin, tmax, out);
        }
        if let Some(c2) = node.child2.as_ref() {
            self.ray_query_recursive(c2, ro, rd, tmin, tmax, out);
        }
    }
```
```rust
    pub fn ray_aabb_intersect(
        ro: Point3D,      // ray origin
        rd: Vector3D,     // ray dir
        bb: &BoundingBox, // AABB
        tmin: f64,        // enter
        tmax: f64,        // exit
    ) -> bool {
        #[inline]
        fn check_axis(rox: f64, rdx: f64, mn: f64, mx: f64, t0: &mut f64, t1: &mut f64) -> bool {
            const EPS: f64 = 1e-15;
            if rdx.abs() < EPS {
                // 평행: 원점이 slab 내부에 있어야 통과
                if rox < mn || rox > mx {
                    return false;
                }
                return true; // t 구간은 그대로 유지
            }
            let inv = 1.0 / rdx;
            let mut ta = (mn - rox) * inv;
            let mut tb = (mx - rox) * inv;
            if ta > tb {
                std::mem::swap(&mut ta, &mut tb);
            }
            *t0 = t0.max(ta);
            *t1 = t1.min(tb);
            *t0 <= *t1
        }

        let minv = bb.min(); // 만약 필드라면: let minv = bb.min;
        let maxv = bb.max(); // 필드라면: let maxv = bb.max;

        let mut t0 = tmin;
        let mut t1 = tmax;

        if !check_axis(ro.x, rd.x, minv.x, maxv.x, &mut t0, &mut t1) {
            return false;
        }
        if !check_axis(ro.y, rd.y, minv.y, maxv.y, &mut t0, &mut t1) {
            return false;
        }
        if !check_axis(ro.z, rd.z, minv.z, maxv.z, &mut t0, &mut t1) {
            return false;
        }
        t1 >= t0 && t1 >= 0.0
    }
}
```


----

# 예제 코드

```rust
use std::sync::Arc;

#[derive(Debug, Clone)]
struct MyObject {
    bbox: BoundingBox,
}
```
```rust
impl HasAABB for MyObject {
    fn aabb(&self) -> BoundingBox {
        self.bbox.clone()
    }
}
```

## 🧪 1. 트리 생성 및 객체 삽입
```rust
fn main() {
    let root_bb = BoundingBox::new([0.0; 3], [10.0; 3]);
    let mut tree = SpatialTree::new(root_bb);

    let obj = Arc::new(MyObject {
        bbox: BoundingBox::new([1.0; 3], [2.0; 3]),
    });

    tree.insert(obj);
}
```


## 🔍 2. AABB 범위 쿼리
```rust
fn main() {
    let root_bb = BoundingBox::new([0.0; 3], [10.0; 3]);
    let mut tree = SpatialTree::new(root_bb);

    let obj = Arc::new(MyObject {
        bbox: BoundingBox::new([1.0; 3], [2.0; 3]),
    });
    tree.insert(obj.clone());

    let mut results = Vec::new();
    let query_bb = BoundingBox::new([0.0; 3], [5.0; 3]);
    tree.query(&query_bb, &mut results);

    println!("Found {} objects", results.len());
}
```


## 💥 3. 객체 제거 및 갱신
```rust
fn main() {
    let root_bb = BoundingBox::new([0.0; 3], [10.0; 3]);
    let mut tree = SpatialTree::new(root_bb);

    let obj = Arc::new(MyObject {
        bbox: BoundingBox::new([1.0; 3], [2.0; 3]),
    });
    tree.insert(obj.clone());

    let removed = tree.remove_arc(&obj);
    println!("Removed: {}", removed);

    tree.update(&obj); // 위치가 바뀌었을 경우
}
```


### 🔁 흐름 정리
```rust
tree.insert(obj.clone());      // 객체 삽입
tree.remove_arc(&obj);         // 객체 제거
tree.update(&obj);             // 제거 후 다시 삽입
```

#### 그런데 update() 내부를 보면:
```rust
pub fn update(&mut self, target: &Arc<T>) {
    if self.remove_arc(target) {
        self.insert(target.clone());
    } else {
        self.insert(target.clone());
    }
}
```

- 즉, 무조건 다시 삽입합니다. 그래서:
    - remove_arc()로 객체를 제거하고
    - insert()로 다시 같은 객체를 넣으니
    - 결과적으로 트리 상태는 원래와 동일하게 보임

### 🔍 왜 Arc::ptr_eq로 비교했는데도 다시 삽입된 객체가 같을까?
Arc::ptr_eq(&a, &b)는 두 Arc가 같은 힙 메모리를 가리키는지 확인합니다.  
obj.clone()은 같은 힙을 가리키는 새로운 Arc를 만들기 때문에 ptr_eq는 여전히 true가 됩니다.  
그래서 remove_arc(&obj)는 정확히 해당 객체를 찾아서 제거하고,  
insert(obj.clone())은 같은 객체를 다시 삽입하는 것임.

## ✅ 결론
- before와 after가 같아 보이는 건 정상적인 동작
- update()는 객체의 AABB가 바뀌었을 때를 대비한 함수이므로, AABB가 바뀌지 않으면 시각적으로 변화가 없을 수 있음

### 🔧 확인 테스트 팁
AABB를 바꿔보면 변화가 보임:
```rust
let mut obj = Arc::new(MyObject {
    bbox: BoundingBox::new(Point3D::from([1.0; 3]), Point3D::from([2.0; 3])),
});
tree.insert(obj.clone());
```
```rust
// AABB를 바꾼 새 객체로 교체
let moved = Arc::new(MyObject {
    bbox: BoundingBox::new(Point3D::from([5.0; 3]), Point3D::from([6.0; 3])),
});
tree.update(&moved);
```
- 이렇게 하면 after 출력에서 AABB 위치가 바뀐 걸 확인할 수 있음.


## 🔦 4. 광선 기반 쿼리
```rust
fn main() {
    let root_bb = BoundingBox::new(Point3D::from([0.0; 3]), Point3D::from([10.0; 3]));
    let mut tree = SpatialTree::new(root_bb);

    let obj = Arc::new(MyObject {
        bbox: BoundingBox::new(Point3D::from([1.0; 3]), Point3D::from([2.0; 3])),
    });
    tree.insert(obj.clone());

    let origin = Point3D::new(1.5, 1.5, -5.0); // x, y를 객체 안으로
    let direction = Vector3D::new(0.0, 0.0, 1.0);
    let mut hits = Vec::new();

    tree.ray_query(origin, direction, 0.0, 100.0, &mut hits);
    println!("Ray hit {} objects", hits.len());
}
```

## 📊 5. 트리 통계 확인
```rust
fn main() {
    let root_bb = BoundingBox::new([0.0; 3], [10.0; 3]);
    let mut tree = SpatialTree::new(root_bb);

    for i in 0..5 {
        let obj = Arc::new(MyObject {
            bbox: BoundingBox::new([i as f64; 3], [(i + 1) as f64; 3]),
        });
        tree.insert(obj);
    }

    let stats = tree.stats();
    println!(
        "Nodes: {}, Leaves: {}, Max Depth: {}, Avg objs/leaf: {:.2}",
        stats.nodes, stats.leaves, stats.max_depth, stats.avg_objs_per_leaf
    );
}
```

### 🧱 전체 흐름 요약
- 이 코드는 다음을 수행합니다:
    - 루트 AABB를 생성하고 트리를 초기화
    - 5개의 객체를 생성해서 트리에 삽입
    - 트리의 통계를 계산하고 출력

### 🔍 코드 설명
#### 1️⃣ 루트 AABB 생성
```rust
let root_bb = BoundingBox::new([0.0; 3], [10.0; 3]);
let mut tree = SpatialTree::new(root_bb);
```
- 루트 박스는 (0,0,0) ~ (10,10,10) 범위
- SpatialTree::new()으로 트리 생성

#### 2️⃣ 객체 삽입 루프
```rust
for i in 0..5 {
    let obj = Arc::new(MyObject {
        bbox: BoundingBox::new([i as f64; 3], [(i + 1) as f64; 3]),
    });
    tree.insert(obj);
}
```
- i = 0부터 i = 4까지 총 5개의 객체 생성
- 각 객체의 AABB는 (i, i, i) ~ (i+1, i+1, i+1)
- 모두 루트 박스 안에 있으므로 삽입 시 루트 확장은 발생하지 않음
- Arc로 감싸서 트리에 안전하게 공유 삽입

#### 3️⃣ 통계 출력
```rust
let stats = tree.stats();
println!(
    "Nodes: {}, Leaves: {}, Max Depth: {}, Avg objs/leaf: {:.2}",
    stats.nodes, stats.leaves, stats.max_depth, stats.avg_objs_per_leaf
);
```
- stats()는 다음을 계산:
- nodes: 전체 노드 수
- leaves: 리프 노드 수
- max_depth: 트리의 최대 깊이
- avg_objs_per_leaf: 리프당 평균 객체 수

### 📊 예측되는 출력 예시
- 만약 max_objects = 10이라면:
    - 5개 객체는 루트 리프에 그대로 저장됨
    - 분할은 발생하지 않음
    - 출력 예시:
```
Nodes: 1, Leaves: 1, Max Depth: 1, Avg objs/leaf: 5.00
```


✅ 핵심 포인트
- 이 예제는 트리의 삽입과 통계 계산을 확인하는 데 적합
- max_objects를 낮추면 분할이 발생하고 max_depth가 증가함
- stats()는 구조 분석과 디버깅에 매우 유용한 도구

----


## 🧪 테스트 설명
### 1️⃣ insert_and_split_happens
- 목적: 객체 삽입 시 max_objects를 초과하면 노드가 분할되는지 확인
- 동작:
    - max_objects = 2로 설정 → 3개 객체 삽입 시 분할 유도
    - 객체 3개는 서로 다른 공간에 위치
    - TreeNode::split_node()가 호출되어 child1, child2가 생성되어야 함
- 검증:
    - tree.root.child1.is_some() → 분할 발생 확인
    - 자식 또는 부모에 총 3개 객체가 분산되어 있어야 함

### 2️⃣ query_returns_expected
- 목적: AABB 범위 쿼리 시 교차하는 객체만 정확히 반환되는지 확인
- 동작:
    - 3개의 객체 삽입: a, b, c
    - 쿼리 박스는 a, b와 교차하지만 c와는 교차하지 않음
- 검증:
    - hits에 a, b의 ID가 포함되어야 함
    - c의 ID는 포함되면 안 됨

### 3️⃣ ray_query_hits_expected_boxes
- 목적: 광선 쿼리로 AABB와 교차하는 객체만 정확히 탐지되는지 확인
- 동작:
    - x축 방향으로 진행하는 광선 설정
    - a, b, c는 x축 상에 위치 → 교차
    - d는 y, z축 위쪽에 위치 → 교차하지 않음
- 검증:
    - hits에 a, b, c의 ID가 포함되어야 함
    - d의 ID는 포함되면 안 됨

### 4️⃣ insert_outside_root_expands_root_bbox
- 목적: 루트 AABB 밖에 있는 객체 삽입 시 루트 박스가 확장되는지 확인
- 동작:
    - 루트 박스는 (0,0,0)~(1,1,1)
    - 객체는 (100,100,100)~(101,101,101) → 루트 밖
    - insert() 시 BoundingBox::union_with()로 루트 확장
- 검증:
    - tree.root.bbox.includes(&far.aabb()) → 확장 확인

### 5️⃣ grazing_and_parallel_ray_cases
- 목적: 경계 스치기 및 평행 레이의 엣지 케이스 처리 확인
- 동작:
    - 첫 번째 객체는 x=1 평면에 붙어 있음 → 광선이 스치면 교차해야 함
    - 두 번째 광선은 y축 방향으로 진행하지만 y=10에서 시작 → 박스 범위 밖
- 검증:
    - 첫 번째 광선은 hits에 객체 포함 → 스치기 허용
    - 두 번째 광선은 hits2가 비어 있어야 함 → 평행이지만 범위 밖이면 무시


### 🧱 보조 구조체: Dummy
- Dummy는 HasAABB를 구현한 테스트용 객체
- id로 식별 가능
- BoundingBox를 직접 생성해 위치 지정 가능

### ✏️ 수식 참고: Ray-AABB 교차
- 슬래브 방식으로 각 축에 대해 교차 범위 계산:

$$
t_a=\frac{mn-ro}{rd},\quad t_b=\frac{mx-ro}{rd}
$$

$$
t_0=\max (t_0,\min (t_a,t_b)),\quad t_1=\min (t_1,\max (t_a,t_b))
$$

$$
\mathrm{교차\  조건:\  }t_1\geq t_0\wedge t_1\geq 0
$$

```rust
#[derive(Debug, Clone)]
struct Dummy {
    id: u32,
    bb: BoundingBox,
}
```
```rust
impl Dummy {
    fn new(id: u32, min: (f64, f64, f64), max: (f64, f64, f64)) -> Self {
        let min_p = Point3D::new(min.0, min.1, min.2);
        let max_p = Point3D::new(max.0, max.1, max.2);
        Self {
            id,
            bb: BoundingBox::new(min_p, max_p),
        }
    }
}
```
```rust
impl HasAABB for Dummy {
    fn aabb(&self) -> BoundingBox {
        self.bb
    }
}
```
```rust
fn ray_aabb_intersect(
    ro: Point3D,
    rd: Vector3D,
    bb: &BoundingBox,
    t_min: f64,
    t_max: f64,
) -> bool {
    // 축별 슬래브 테스트
    fn check_axis(ro: f64, rd: f64, mn: f64, mx: f64, t0: &mut f64, t1: &mut f64) -> bool {
        if rd.abs() < 1e-15 {
            // 평행: 원점이 범위 밖이면 실패
            return !(ro < mn || ro > mx);
        }
        let inv = 1.0 / rd;
        let mut t_a = (mn - ro) * inv;
        let mut t_b = (mx - ro) * inv;
        if t_a > t_b {
            std::mem::swap(&mut t_a, &mut t_b);
        }
        *t0 = t0.max(t_a);
        *t1 = t1.min(t_b);
        *t0 <= *t1
    }

    let mut t0 = t_min;
    let mut t1 = t_max;

    if !check_axis(ro.x, rd.x, bb.min().x, bb.max().x, &mut t0, &mut t1) {
        return false;
    }
    if !check_axis(ro.y, rd.y, bb.min().y, bb.max().y, &mut t0, &mut t1) {
        return false;
    }
    if !check_axis(ro.z, rd.z, bb.min().z, bb.max().z, &mut t0, &mut t1) {
        return false;
    }
    t1 >= t0 && t1 >= 0.0
}
```
```rust
// ---- 헬퍼: 트리 만들기 -------------------------------------------------
fn make_tree(
    max_objects: usize,
    root_min: (f64, f64, f64),
    root_max: (f64, f64, f64),
) -> SpatialTree<Dummy> {
    let root = TreeNode {
        bbox: BoundingBox::new(
            Point3D::new(root_min.0, root_min.1, root_min.2),
            Point3D::new(root_max.0, root_max.1, root_max.2),
        ),
        objects: Vec::new(),
        child1: None,
        child2: None,
    };
    SpatialTree {
        root: Box::new(root),
        max_objects,
    }
}
```
```rust
// ---- 1) 삽입 + 분할 동작 테스트 --------------------------------------
#[test]
fn insert_and_split_happens() {
    // 작은 루트, 낮은 max_objects 로 빨리 분할 유도
    let mut tree = make_tree(2, (0.0, 0.0, 0.0), (10.0, 10.0, 10.0));

    // 루트 안쪽에서 서로 다른 하프 공간에 들어가도록 배치
    let objs = vec![
        Arc::new(Dummy::new(1, (1.0, 1.0, 1.0), (2.0, 2.0, 2.0))),
        Arc::new(Dummy::new(2, (1.5, 1.5, 1.5), (2.5, 2.5, 2.5))),
        Arc::new(Dummy::new(3, (8.0, 8.0, 8.0), (9.0, 9.0, 9.0))), // 3번째에서 분할 가능
    ];
    for o in objs {
        tree.insert(o);
    }

    // 루트가 분할되었는지 검사
    assert!(
        tree.root.child1.is_some() && tree.root.child2.is_some(),
        "root must split"
    );

    // 분할 후 부모에 남는 객체(둘 다 못 담는)도 있을 수 있으나
    // 최소한 자식 중 하나는 무언가를 가져야 함
    let c1_cnt = tree.root.child1.as_ref().unwrap().objects.len();
    let c2_cnt = tree.root.child2.as_ref().unwrap().objects.len();
    let parent_cnt = tree.root.objects.len();
    assert!(c1_cnt + c2_cnt + parent_cnt >= 3);
}
```
```rust
// ---- 2) 쿼리 테스트 (AABB vs AABB) ------------------------------------
#[test]
fn query_returns_expected() {
    let mut tree = make_tree(3, (0.0, 0.0, 0.0), (20.0, 20.0, 20.0));

    let a = Arc::new(Dummy::new(10, (1.0, 1.0, 1.0), (2.0, 2.0, 2.0)));
    let b = Arc::new(Dummy::new(11, (5.0, 5.0, 5.0), (6.0, 6.0, 6.0)));
    let c = Arc::new(Dummy::new(12, (15.0, 15.0, 15.0), (16.0, 16.0, 16.0)));
    tree.insert(a.clone());
    tree.insert(b.clone());
    tree.insert(c.clone());

    // 1~7 범위면 a,b는 교차, c는 제외
    let qbox = BoundingBox::new(Point3D::new(0.5, 0.5, 0.5), Point3D::new(7.0, 7.0, 7.0));
    let mut hits: Vec<Arc<Dummy>> = Vec::new();
    // 간단 Query 구현이 트리에 있다면 사용: tree.query(&qbox, &mut hits);
    // 여기서는 레퍼런스용으로 루트부터 DFS
    fn query_rec<T: HasAABB>(n: &TreeNode<T>, q: &BoundingBox, out: &mut Vec<Arc<T>>) {
        if !n.bbox.intersects_self(q) {
            return;
        }
        for o in &n.objects {
            if o.aabb().intersects_self(q) {
                out.push(o.clone());
            }
        }
        if let Some(c1) = n.child1.as_ref() {
            query_rec(c1, q, out);
        }
        if let Some(c2) = n.child2.as_ref() {
            query_rec(c2, q, out);
        }
    }
    query_rec(&tree.root, &qbox, &mut hits);

    let ids: Vec<u32> = hits.iter().map(|o| o.id).collect();
    assert!(ids.contains(&10));
    assert!(ids.contains(&11));
    assert!(!ids.contains(&12));
}
```
```rust
// ---- 3) RayQuery 테스트 -----------------------------------------------
#[test]
fn ray_query_hits_expected_boxes() {
    let mut tree = make_tree(2, (-10.0, -10.0, -10.0), (10.0, 10.0, 10.0));

    // x축 양의 방향으로 진행하는 레이와 겹치는 박스들
    let a = Arc::new(Dummy::new(1, (-9.0, -1.0, -1.0), (-8.0, 1.0, 1.0))); // 좌측
    let b = Arc::new(Dummy::new(2, (-1.0, -1.0, -1.0), (1.0, 1.0, 1.0))); // 중심
    let c = Arc::new(Dummy::new(3, (5.0, -1.0, -1.0), (6.0, 1.0, 1.0))); // 우측
    let d = Arc::new(Dummy::new(4, (0.0, 5.0, 5.0), (1.0, 6.0, 6.0))); // y,z 위쪽(레이와 비충돌)

    for o in [&a, &b, &c, &d] {
        tree.insert(o.clone());
    }

    let ro = Point3D::new(-10.0, 0.0, 0.0);
    let rd = Vector3D::new(1.0, 0.0, 0.0);

    // 트리 레이쿼리 (네 구현 사용 가능)
    fn ray_query_rec<T: HasAABB>(
        n: &TreeNode<T>,
        ro: Point3D,
        rd: Vector3D,
        tmin: f64,
        tmax: f64,
        out: &mut Vec<Arc<T>>,
    ) {
        if !ray_aabb_intersect(ro, rd, &n.bbox, tmin, tmax) {
            return;
        }
        for o in &n.objects {
            if ray_aabb_intersect(ro, rd, &o.aabb(), tmin, tmax) {
                out.push(o.clone());
            }
        }
        if let Some(c1) = n.child1.as_ref() {
            ray_query_rec(c1, ro, rd, tmin, tmax, out);
        }
        if let Some(c2) = n.child2.as_ref() {
            ray_query_rec(c2, ro, rd, tmin, tmax, out);
        }
    }

    let mut hits = Vec::<Arc<Dummy>>::new();
    ray_query_rec(&tree.root, ro, rd, 0.0, 1000.0, &mut hits);
    let mut ids: Vec<u32> = hits.iter().map(|o| o.id).collect();
    ids.sort_unstable();

    assert_eq!(ids, vec![1, 2, 3], "ray should hit a,b,c but not d");
}
```
```rust
// ---- 4) 루트 밖 객체 삽입 시 루트 확장 확인 -----------------------------
#[test]
fn insert_outside_root_expands_root_bbox() {
    let mut tree = make_tree(2, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0));
    let far = Arc::new(Dummy::new(99, (100.0, 100.0, 100.0), (101.0, 101.0, 101.0)));
    tree.insert(far.clone());

    // 확장되었어야 한다
    assert!(
        tree.root.bbox.includes(&far.aabb()),
        "root bbox must expand to include far object"
    );
}
```
```rust
// ---- 5) 경계 스치기/평행 레이 엣지 케이스 -------------------------------
#[test]
fn grazing_and_parallel_ray_cases() {
    let mut tree = make_tree(2, (-1.0, -1.0, -1.0), (1.0, 1.0, 1.0));
    let on_edge = Arc::new(Dummy::new(7, (1.0, -0.1, -0.1), (2.0, 0.1, 0.1))); // x=1 평면에 붙음
    tree.insert(on_edge.clone());

    // 레이가 x=1 평면을 스치는 경우
    let ro = Point3D::new(0.0, 0.0, 0.0);
    let rd = Vector3D::new(1.0, 0.0, 0.0);

    let mut hits = Vec::<Arc<Dummy>>::new();
    fn ray_rec<T: HasAABB>(n: &TreeNode<T>, ro: Point3D, rd: Vector3D, out: &mut Vec<Arc<T>>) {
        if !ray_aabb_intersect(ro, rd, &n.bbox, -1.0, 1000.0) {
            return;
        }
        for o in &n.objects {
            if ray_aabb_intersect(ro, rd, &o.aabb(), -1.0, 1000.0) {
                out.push(o.clone());
            }
        }
        if let Some(c1) = n.child1.as_ref() {
            ray_rec(c1, ro, rd, out);
        }
        if let Some(c2) = n.child2.as_ref() {
            ray_rec(c2, ro, rd, out);
        }
    }
    ray_rec(&tree.root, ro, rd, &mut hits);
    assert!(!hits.is_empty(), "grazing ray should count as a hit");

    // 평행한 레이(Y축 방향), 박스 Y범위 밖
    let ro2 = Point3D::new(0.0, 10.0, 0.0);
    let rd2 = Vector3D::new(0.0, 1.0, 0.0);
    let mut hits2 = Vec::<Arc<Dummy>>::new();
    ray_rec(&tree.root, ro2, rd2, &mut hits2);
    assert!(
        hits2.is_empty(),
        "parallel ray outside y-range should not hit"
    );
}
```

---
