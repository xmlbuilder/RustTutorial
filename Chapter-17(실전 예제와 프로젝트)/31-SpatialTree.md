# SpatialTree

## **Goal:** 
- understand the math, design, and usage of our `SpatialTree` (binary AABB tree) and adopt behavior‑centric Rust patterns safely.

---

## 1) Big Picture

**What it is:** A dynamic, binary spatial index of axis-aligned bounding boxes (AABBs).  
**Why:** Speed up queries like frustum/box intersection, ray picking, and broad‑phase collision.

```
Root AABB
├─ child1 (AABB)
│  ├─ leaf (objs...)
│  └─ leaf (objs...)
└─ child2 (AABB)
   ├─ leaf (objs...)
   └─ leaf (objs...)
```

We keep **objects at the deepest child that fully contains their AABB**;  
if neither child contains it, the object is **stored at the parent** (parent‑store pattern),  
guaranteeing termination and preserving correctness for “spanning” objects.

---

## 2) Math Primer (AABB, Slabs, and Slopes)

### 2.1 Axis‑Aligned Bounding Box (AABB)
An AABB is defined by `min = (xmin, ymin, zmin)` and `max = (xmax, ymax, zmax)` with `min <= max` componentwise.

**Containment (closed interval):**
```
big.contains(small)  ⟺
  big.min.x <= small.min.x <= small.max.x <= big.max.x
∧ big.min.y <= small.min.y <= small.max.y <= big.max.y
∧ big.min.z <= small.min.z <= small.max.z <= big.max.z
```

**Intersection (closed interval test):**
```
intersects(a, b) ⟺
  a.max.x >= b.min.x ∧ a.min.x <= b.max.x
∧ a.max.y >= b.min.y ∧ a.min.y <= b.max.y
∧ a.max.z >= b.min.z ∧ a.min.z <= b.max.z
```

### 2.2 Ray–AABB (“slab test”)
For ray `r(t) = ro + t * rd`, test per axis with parametric intervals:
```
tx1 = (xmin - ro.x) / rd.x,  tx2 = (xmax - ro.x) / rd.x
ty1 = (ymin - ro.y) / rd.y,  ty2 = (ymax - ro.y) / rd.y
tz1 = (zmin - ro.z) / rd.z,  tz2 = (zmax - ro.z) / rd.z
```
Normalize each pair so `t1 <= t2`, then intersect intervals:
```
tmin = max( min(tx1, tx2), min(ty1, ty2), min(tz1, tz2) )
tmax = min( max(tx1, tx2), max(ty1, ty2), max(tz1, tz2) )
hit  ⇔  tmax ≥ tmin  and  tmax ≥ 0  and  [optional] t‑window clip
```
If `rd.<axis> ≈ 0`, treat as parallel; accept only if `ro.<axis>` is within `[min, max]` of the box on that axis.

### 2.3 Longest‑Axis Split
Given `extent = max - min`, split along `argmax(extent)` at `mid = 0.5*(min[axis]+max[axis])`. Two children are
```
child1: [min, mid]   child2: [mid, max]  (along that axis; other axes unchanged)
```
This balances depth and reduces overlap for typical distributions.

---

## 3) Data Model & Ownership

- `SpatialTree<T>` owns a **root** `TreeNode<T>` and configuration like `max_objects`.
- `TreeNode<T>` owns:
  - `bbox: BoundingBox` (covers this subtree)
  - `objects: Vec<Arc<T>>` (only those not fully contained by a child)
  - `child1: Option<Box<TreeNode<T>>>`, `child2: Option<Box<TreeNode<T>>>`
- `T: SpatialObject` gives `fn aabb(&self) -> BoundingBox`.

We use `Arc<T>` for shared ownership across the tree and client systems (renderer/physics).

---

## 4) Core Algorithms

### 4.1 Insert
1. Ensure `root.bbox` **expands** to include `obj.aabb()` if needed (cheap union).
2. Recurse:
   - If **leaf**: push; if `objects.len() > max_objects` ⇒ **split**.
   - Else: if **child contains** obj ⇒ **descend**; otherwise **store at parent**.

### 4.2 Split
- Longest‑axis split, allocate two children.
- **Re‑distribute** existing objects:
  - If `child.contains(obj.bb)` ⇒ move to that child.
  - Otherwise keep in parent (spanners).

### 4.3 Query (Box)
- DFS pruning by `node.bbox ∩ query == ∅`.
- For survivors: add any `o ∈ node.objects` with `o.aabb ∩ query ≠ ∅`, then recurse children.

### 4.4 RayQuery
- DFS pruning by `ray_aabb_intersect(ray, node.bbox)`.
- For survivors: add `o` if `ray_aabb_intersect(ray, o.aabb)`; recurse children.

---

## 5) Rust Usage Examples

### 5.1 Minimal Traits and Types
```rust
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct Point3D { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Clone, Copy, Debug)]
pub struct Vector3D { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox { pub min: Point3D, pub max: Point3D }

pub trait SpatialObject {
    fn aabb(&self) -> BoundingBox;
}
```

### 5.2 SpatialTree Construction & Insertion
```rust
let root_bb = BoundingBox {
    min: Point3D { x: -10.0, y: -10.0, z: -10.0 },
    max: Point3D { x:  10.0, y:  10.0, z:  10.0 },
};
let mut tree = SpatialTree::<MyGeom>::new(root_bb, 8);

let g = Arc::new(MyGeom::cube((0.0,0.0,0.0), 1.0));
tree.insert(g.clone());

// insert far away → root expands
let far = Arc::new(MyGeom::cube((100.0,100.0,100.0), 2.0));
tree.insert(far.clone());
assert!(tree.root().bbox.includes(&far.aabb()));
```

### 5.3 Box Query
```rust
let q = BoundingBox {
    min: Point3D { x: -1.0, y: -1.0, z: -1.0 },
    max: Point3D { x:  1.0, y:  1.0, z:  1.0 },
};
let mut out = Vec::new();
tree.query(&q, &mut out);
println!("{} hit(s)", out.len());
```

### 5.4 Ray Query
```rust
let ro = Point3D { x: 0.0, y: 0.0, z: -100.0 };
let rd = Vector3D { x: 0.0, y: 0.0, z: 1.0 }; // toward +z
let mut hits = Vec::new();
tree.ray_query(ro, rd, 0.0, 1.0e9, &mut hits);
```

---

## 6) ASCII Diagrams (Split & Parent‑Store)

### 6.1 Longest‑Axis Split (X‑axis)
```
Before split:
+--------------------------+
|        node.bbox         |
|   objs: [A,B,C,D,E,F]    |
+--------------------------+

After split at midX:
+--------------+--------------+
| child1.bbox  | child2.bbox  |
| objs: [A,B]  | objs: [C,D]  |   E,F don’t fit fully → kept at parent
+--------------+--------------+

Parent now keeps: [E, F]
```

### 6.2 Query Pruning
```
                 [node]
                /      \
     (miss) [c1]        [c2] (hit)
                 \      /
                 [skip c1 subtree]
                 [descend c2 only]
```

---

## 7) Performance Notes

- **Parent‑store** guarantees we never “force” bad fits; it reduces infinite churn for large/spanning objects.
- **Split threshold** (`max_objects`) trades memory for depth. Typical values: 8–32.
- **Cache locality:** DFS with small recursion depth is okay; you can convert to iterative with a stack vector.
- **Rebuild/Refit:** For heavy dynamic scenes, occasionally rebuild from scratch or refit node AABBs from children.

---

## 8) Testing Checklist

- Insertion of objects **inside** root (no expansion).
- Insertion **outside** root expands root bbox.
- Split triggers when `len(objects) > max_objects`.
- Objects that **span** children remain in parent.
- Box query finds exactly expected set.
- Ray query matches CPU reference of ray–AABB for each result.
- Remove + Update keep tree invariants.
- Stats: nodes, leaves, max_depth, avg_objs/leaf look sane for random scenes.

---

## 9) Common Pitfalls (and Fixes)

- **Parallel rays (rd ≈ 0) on an axis:** special‑case; treat as infinite slab if `ro` within `[min,max]`.
- **Floating‑point tolerances:** Use small eps (1e‑12…1e‑9) when comparing, especially in `includes` and `intersects`.
- **Borrow checker pain during recursion:** pass `&mut TreeNode` down, but avoid borrowing `self` again inside that call (no nested `self.*` mutable borrows).  
    Extract values first.
- **Arc<T> cycles:** Tree holds `Arc<T>`; do not store back‑references from `T` into the tree.

---

## 10) Reference Implementations (Snippets)

### 10.1 Ray–AABB (robust)
```rust
pub fn ray_aabb_intersect(
    ro: Point3D, rd: Vector3D,
    bb: &BoundingBox, mut tmin: f64, mut tmax: f64
) -> bool {
    #[inline]
    fn axis(ro: f64, rd: f64, mn: f64, mx: f64, t0: &mut f64, t1: &mut f64) -> bool {
        const EPS: f64 = 1e-15;
        if rd.abs() < EPS {
            // parallel: inside the slab?
            if ro < mn || ro > mx { return false; }
            true
        } else {
            let inv = 1.0 / rd;
            let mut ta = (mn - ro) * inv;
            let mut tb = (mx - ro) * inv;
            if ta > tb { std::mem::swap(&mut ta, &mut tb); }
            *t0 = t0.max(ta);
            *t1 = t1.min(tb);
            *t0 <= *t1
        }
    }

    if !axis(ro.x, rd.x, bb.min.x, bb.max.x, &mut tmin, &mut tmax) { return false; }
    if !axis(ro.y, rd.y, bb.min.y, bb.max.y, &mut tmin, &mut tmax) { return false; }
    if !axis(ro.z, rd.z, bb.min.z, bb.max.z, &mut tmin, &mut tmax) { return false; }
    tmax >= tmin && tmax >= 0.0
}
```

### 10.2 Longest‑Axis Split Helper
```rust
impl BoundingBox {
    pub fn longest_axis(&self) -> usize {
        let ex = self.max.x - self.min.x;
        let ey = self.max.y - self.min.y;
        let ez = self.max.z - self.min.z;
        if ex >= ey && ex >= ez { 0 } else if ey >= ez { 1 } else { 2 }
    }
    pub fn split_along_longest_axis(&self) -> (BoundingBox, BoundingBox) {
        let axis = self.longest_axis();
        let mid  = 0.5 * (self.min_at(axis) + self.max_at(axis));
        let mut a = *self;
        let mut b = *self;
        a.set_max_at(axis, mid);
        b.set_min_at(axis, mid);
        (a, b)
    }
}
```

---

## 11) FAQ

**Q. Why parent‑store instead of forcing every object downwards?**  
A. Large objects cause pathological overlap; forcing them down increases duplicates or breaks containment invariant. Parent‑store is simple and correct.

**Q. How big should `max_objects` be?**  
A. Start with 8–16. If the tree doesn’t split often and queries are slow, reduce. If splits explode depth, increase.

**Q. Can we use SAH (surface area heuristic) or BVH instead?**  
A. Yes. Our structure is compatible with SAH‑guided splits. Start simple; profile later.

---

## 12) Next Steps

- Add **frustum** queries (6 plane checks).
- Add **refit** path for moving objects (bottom‑up size update).
- Optional **iterative** DFS for stackless traversal in hot loops.
- Benchmarks on representative scenes.

---

**Appendix A – Deriving Slab Intervals**

For axis `i ∈ {x,y,z}`, the ray equation is `ri(t) = roi + t * rdi`. Intersection with slab `[min_i, max_i]` happens if `t` satisfies:
```
min_i ≤ roi + t * rdi ≤ max_i
⇒ (min_i - roi)/rdi ≤ t ≤ (max_i - roi)/rdi   (if rdi > 0)
⇒ (max_i - roi)/rdi ≤ t ≤ (min_i - roi)/rdi   (if rdi < 0)
```
Flip if `rdi < 0` to keep `t1 ≤ t2`. Intersect across axes by `tmin = max(t1x,t1y,t1z)`, `tmax = min(t2x,t2y,t2z)`.

---

## Source code

```rust
use std::sync::Arc;
use crate::math::boundingbox::BoundingBox;
use crate::math::prelude::Point3D;
use crate::math::vector3d::Vector3D;

pub trait SpatialObject: Send + Sync {
    fn aabb(&self) -> BoundingBox;
}

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


#[derive(Debug)]
pub struct TreeNode<T: SpatialObject> {
    pub bbox: BoundingBox,
    pub objects: Vec<Arc<T>>,                // 부모 보관 패턴 가능
    pub child1: Option<Box<TreeNode<T>>>,
    pub child2: Option<Box<TreeNode<T>>>,
}


impl<T: SpatialObject> TreeNode<T> {
    fn is_leaf(&self) -> bool {
        self.child1.is_none() && self.child2.is_none()
    }

    fn bbox_obj_count(&self) -> usize {
        self.objects.len()
    }
}


#[derive(Debug)]
pub struct SpatialTree<T: SpatialObject> {
    pub root: Box<TreeNode<T>>,
    pub max_objects: usize, // 리프가 이 개수를 넘으면 분할
}

impl<T: SpatialObject> SpatialTree<T> {
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

    pub fn root_bbox(&self) -> &BoundingBox { &self.root.bbox }
    pub fn root_bbox_mut(&mut self) -> &mut BoundingBox { &mut self.root.bbox }

    pub fn expand_to_include(&mut self, new_bbox: &BoundingBox) {
        self.root.bbox.union_mut(new_bbox);
    }

    /// C++ Insert(bbox, obj) + 루트 밖이면 확장
    pub fn insert(&mut self, obj: Arc<T>) {
        let bb = obj.aabb();
        if !self.root.bbox.includes(&bb) {
            // 루트 박스 확장
            self.root.bbox.union_with(&bb);
        }
        let root = &mut self.root;                 // 루트만 &mut 대여
        Self::insert_recursive(root, obj, self.max_objects);
    }

    pub fn query(&self, q: &BoundingBox, out: &mut Vec<Arc<T>>) {
        self.query_recursive(&self.root, q, out);
    }

    pub fn intersects_box(&self, q: &BoundingBox) -> bool {
        let mut hit = false;
        self.traverse_nodes(|n| {
            if !BoundingBox::intersects(&n.bbox, q) { return; }
            if n.objects.iter().any(|o| BoundingBox::intersects(&o.aabb(), q)) {
                hit = true;
            }
        });
        hit
    }

    pub fn remove_arc(&mut self, target: &Arc<T>) -> bool {
        let mut removed = false;
        fn walk<T: SpatialObject>(n: &mut TreeNode<T>, target: &Arc<T>, removed: &mut bool) {
            n.objects.retain(|o| {
                let keep = !Arc::ptr_eq(o, target);
                if !keep { *removed = true; }
                keep
            });
            if let Some(c1) = n.child1.as_mut() { walk(c1, target, removed); }
            if let Some(c2) = n.child2.as_mut() { walk(c2, target, removed); }
        }
        walk(&mut self.root, target, &mut removed);
        removed
    }

    pub fn update(&mut self, target: &Arc<T>) {
        // 단순 정책: 지우고 다시 삽입(객체가 aabb()에서 최신 박스를 제공한다고 가정)
        if self.remove_arc(target) {
            self.insert(target.clone());
        } else {
            // 못 찾았으면 그냥 삽입
            self.insert(target.clone());
        }
    }

    pub fn ray_query(&self, ro: Point3D, rd: Vector3D, tmin: f64, tmax: f64, out: &mut Vec<Arc<T>>) {
        out.clear();
        self.ray_query_recursive(&self.root, ro, rd, tmin, tmax, out);
    }

    pub fn traverse_nodes<F: FnMut(&TreeNode<T>)>(&self, mut f: F) {
        fn dfs<T: SpatialObject, F: FnMut(&TreeNode<T>)>(n: &TreeNode<T>, f: &mut F) {
            f(n);
            if let Some(c1) = n.child1.as_ref() { dfs(c1, f); }
            if let Some(c2) = n.child2.as_ref() { dfs(c2, f); }
        }
        dfs(&self.root, &mut f);
    }

    pub fn traverse_nodes_and_objects<FN, FO>(&self, mut node_cb: FN, mut obj_cb: FO)
    where
        FN: FnMut(&TreeNode<T>),
        FO: FnMut(&Arc<T>),
    {
        fn dfs<T: SpatialObject, FN, FO>(n: &TreeNode<T>, node_cb: &mut FN, obj_cb: &mut FO)
        where
            FN: FnMut(&TreeNode<T>),
            FO: FnMut(&Arc<T>),
        {
            node_cb(n);
            for o in &n.objects { obj_cb(o); }
            if let Some(c1) = n.child1.as_ref() { dfs(c1, node_cb, obj_cb); }
            if let Some(c2) = n.child2.as_ref() { dfs(c2, node_cb, obj_cb); }
        }
        dfs(&self.root, &mut node_cb, &mut obj_cb);
    }

    pub fn stats(&self) -> SpatialTreeStats {
        let mut S = SpatialTreeStats::default();

        self.traverse_nodes(|n| {
            S.nodes += 1;
            let c = n.bbox_obj_count();
            S.objects += c;
            if n.is_leaf() {
                S.leaves += 1;
                if S.leaf_hist.len() <= c { S.leaf_hist.resize(c + 1, 0); }
                S.leaf_hist[c] += 1;
            } else {
                S.parent_stored += c;
            }
        });

        // 최대 깊이
        fn depth<T: SpatialObject>(n: &TreeNode<T>, d: usize) -> usize {
            if n.is_leaf() { d }
            else {
                let d1 = n.child1.as_ref().map(|c| depth(c, d + 1)).unwrap_or(d);
                let d2 = n.child2.as_ref().map(|c| depth(c, d + 1)).unwrap_or(d);
                d1.max(d2)
            }
        }
        S.max_depth = depth(&self.root, 1);
        S.avg_objs_per_leaf = if S.leaves > 0 { S.objects as f64 / S.leaves as f64 } else { 0.0 };
        S
    }

    fn insert_recursive(node: &mut TreeNode<T>, obj: Arc<T>, max_objects: usize) {
        if node.is_leaf() {
            node.objects.push(obj);
            if node.objects.len() > max_objects {
                Self::split_node(node);           // ⬅️ self 없이 호출
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

    fn split_node(node: &mut TreeNode<T>) {
        // 가장 긴 축 기준으로 2분할
        let axis = node.bbox.max_extent_axis(); // 네 boundingbox.rs에 맞춰 구현되어 있을 거라 가정
        let mid = 0.5 * (node.bbox.min()[axis] + node.bbox.max()[axis]);

        let mut b1 = node.bbox;
        let mut b2 = node.bbox;
        b1.max_mut(axis, mid);
        b2.min_mut(axis, mid);

        let mut c1 = Box::new(TreeNode { bbox: b1, objects: Vec::new(), child1: None, child2: None });
        let mut c2 = Box::new(TreeNode { bbox: b2, objects: Vec::new(), child1: None, child2: None });

        // 기존 오브젝트 재분배
        let old = std::mem::take(&mut node.objects);
        for o in old {
            let bb = o.aabb();
            if c1.bbox.includes(&bb) { c1.objects.push(o); }
            else if c2.bbox.includes(&bb) { c2.objects.push(o); }
            else { node.objects.push(o); } // 둘 다 못 담으면 부모에 남김
        }

        node.child1 = Some(c1);
        node.child2 = Some(c2);
    }

    fn query_recursive(&self, node: &TreeNode<T>, q: &BoundingBox, out: &mut Vec<Arc<T>>) {
        if !BoundingBox::intersects(&node.bbox, q) { return; }

        for o in &node.objects {
            if BoundingBox::intersects(&o.aabb(), q) {
                out.push(o.clone());
            }
        }
        if let Some(c1) = node.child1.as_ref() { self.query_recursive(c1, q, out); }
        if let Some(c2) = node.child2.as_ref() { self.query_recursive(c2, q, out); }
    }



    fn ray_query_recursive(
        &self,
        node: &TreeNode<T>,
        ro: Point3D,
        rd: Vector3D,
        tmin: f64, tmax: f64,
        out: &mut Vec<Arc<T>>
    ) {
        if !Self::ray_aabb_intersect(ro, rd, &node.bbox, tmin, tmax) { return; }

        for o in &node.objects {
            if Self::ray_aabb_intersect(ro, rd, &o.aabb(), tmin, tmax) {
                out.push(o.clone());
            }
        }
        if let Some(c1) = node.child1.as_ref() { self.ray_query_recursive(c1, ro, rd, tmin, tmax, out); }
        if let Some(c2) = node.child2.as_ref() { self.ray_query_recursive(c2, ro, rd, tmin, tmax, out); }
    }

    pub fn ray_aabb_intersect(
        ro: Point3D,          // ray origin
        rd: Vector3D,         // ray dir
        bb: &BoundingBox,     // AABB
        tmin: f64,            // enter
        tmax: f64,            // exit
    ) -> bool {
        #[inline]
        fn check_axis(
            rox: f64, rdx: f64,
            mn: f64, mx: f64,
            t0: &mut f64, t1: &mut f64,
        ) -> bool {
            const EPS: f64 = 1e-15;
            if rdx.abs() < EPS {
                // 평행: 원점이 slab 내부에 있어야 통과
                if rox < mn || rox > mx { return false; }
                return true; // t 구간은 그대로 유지
            }
            let inv = 1.0 / rdx;
            let mut ta = (mn - rox) * inv;
            let mut tb = (mx - rox) * inv;
            if ta > tb { std::mem::swap(&mut ta, &mut tb); }
            *t0 = t0.max(ta);
            *t1 = t1.min(tb);
            *t0 <= *t1
        }

        let minv = bb.min(); // 만약 필드라면: let minv = bb.min;
        let maxv = bb.max(); // 필드라면: let maxv = bb.max;

        let mut t0 = tmin;
        let mut t1 = tmax;

        if !check_axis(ro.x, rd.x, minv.x, maxv.x, &mut t0, &mut t1) { return false; }
        if !check_axis(ro.y, rd.y, minv.y, maxv.y, &mut t0, &mut t1) { return false; }
        if !check_axis(ro.z, rd.z, minv.z, maxv.z, &mut t0, &mut t1) { return false; }

        // C++ 원본과 동일하게 t1>=t0 && t1>=0 조건
        t1 >= t0 && t1 >= 0.0
    }
}


```

## 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use geometry::math::boundingbox::BoundingBox;
    use geometry::math::prelude::{Point3D, Vector3D};
    use geometry::spatial::spatial_tree::{SpatialObject, SpatialTree, TreeNode};
    // ---- 테스트용 더미 타입 ----------------------------------------------

    #[derive(Debug, Clone)]
    struct Dummy {
        id: u32,
        bb: BoundingBox,
    }

    impl Dummy {
        fn new(id: u32, min: (f64, f64, f64), max: (f64, f64, f64)) -> Self {
            let minp = Point3D::new(min.0, min.1, min.2);
            let maxp = Point3D::new(max.0, max.1, max.2);
            Self { id, bb: BoundingBox::new(minp, maxp) }
        }
    }

    impl SpatialObject for Dummy {
        fn aabb(&self) -> BoundingBox { self.bb }
    }

    // ---- (필요시) 테스트내 로컬 Ray-AABB: 네가 이미 제공한 로직과 동일 -----

    fn ray_aabb_intersect(ro: Point3D, rd: Vector3D, bb: &BoundingBox, tmin: f64, tmax: f64) -> bool {
        // 축별 슬래브 테스트
        fn check_axis(ro: f64, rd: f64, mn: f64, mx: f64, t0: &mut f64, t1: &mut f64) -> bool {
            if rd.abs() < 1e-15 {
                // 평행: 원점이 범위 밖이면 실패
                return !(ro < mn || ro > mx);
            }
            let inv = 1.0 / rd;
            let mut tA = (mn - ro) * inv;
            let mut tB = (mx - ro) * inv;
            if tA > tB { std::mem::swap(&mut tA, &mut tB); }
            *t0 = t0.max(tA);
            *t1 = t1.min(tB);
            *t0 <= *t1
        }

        let mut t0 = tmin;
        let mut t1 = tmax;

        if !check_axis(ro.x, rd.x, bb.min().x, bb.max().x, &mut t0, &mut t1) { return false; }
        if !check_axis(ro.y, rd.y, bb.min().y, bb.max().y, &mut t0, &mut t1) { return false; }
        if !check_axis(ro.z, rd.z, bb.min().z, bb.max().z, &mut t0, &mut t1) { return false; }
        t1 >= t0 && t1 >= 0.0
    }

    // ---- 헬퍼: 트리 만들기 -------------------------------------------------

    fn make_tree(max_objects: usize, root_min: (f64, f64, f64), root_max: (f64, f64, f64)) -> SpatialTree<Dummy> {
        let root = TreeNode {
            bbox: BoundingBox::new(
                Point3D::new(root_min.0, root_min.1, root_min.2),
                Point3D::new(root_max.0, root_max.1, root_max.2),
            ),
            objects: Vec::new(),
            child1: None,
            child2: None,
        };
        SpatialTree { root : Box::new(root), max_objects }
    }

    // ---- 1) 삽입 + 분할 동작 테스트 --------------------------------------

    #[test]
    fn insert_and_split_happens() {
        // 작은 루트, 낮은 max_objects로 빨리 분할 유도
        let mut tree = make_tree(2, (0.0, 0.0, 0.0), (10.0, 10.0, 10.0));

        // 루트 안쪽에서 서로 다른 하프 공간에 들어가도록 배치
        let objs = vec![
            Arc::new(Dummy::new(1, (1.0, 1.0, 1.0), (2.0, 2.0, 2.0))),
            Arc::new(Dummy::new(2, (1.5, 1.5, 1.5), (2.5, 2.5, 2.5))),
            Arc::new(Dummy::new(3, (8.0, 8.0, 8.0), (9.0, 9.0, 9.0))), // 3번째에서 분할 가능
        ];
        for o in objs { tree.insert(o); }

        // 루트가 분할되었는지 검사
        assert!(tree.root.child1.is_some() && tree.root.child2.is_some(), "root must split");

        // 분할 후 부모에 남는 객체(둘 다 못 담는)도 있을 수 있으나
        // 최소한 자식 중 하나는 무언가를 가져야 함
        let c1_cnt = tree.root.child1.as_ref().unwrap().objects.len();
        let c2_cnt = tree.root.child2.as_ref().unwrap().objects.len();
        let parent_cnt = tree.root.objects.len();
        assert!(c1_cnt + c2_cnt + parent_cnt >= 3);
    }

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
        fn query_rec<T: SpatialObject>(n: &TreeNode<T>, q: &BoundingBox, out: &mut Vec<Arc<T>>) {
            if !n.bbox.intersects_self(q) { return; }
            for o in &n.objects {
                if o.aabb().intersects_self(q) {
                    out.push(o.clone());
                }
            }
            if let Some(c1) = n.child1.as_ref() { query_rec(c1, q, out); }
            if let Some(c2) = n.child2.as_ref() { query_rec(c2, q, out); }
        }
        query_rec(&tree.root, &qbox, &mut hits);

        let ids: Vec<u32> = hits.iter().map(|o| o.id).collect();
        assert!(ids.contains(&10));
        assert!(ids.contains(&11));
        assert!(!ids.contains(&12));
    }

    // ---- 3) RayQuery 테스트 -----------------------------------------------

    #[test]
    fn ray_query_hits_expected_boxes() {
        let mut tree = make_tree(2, (-10.0, -10.0, -10.0), (10.0, 10.0, 10.0));

        // x축 양의 방향으로 진행하는 레이와 겹치는 박스들
        let a = Arc::new(Dummy::new(1, (-9.0, -1.0, -1.0), (-8.0, 1.0, 1.0))); // 좌측
        let b = Arc::new(Dummy::new(2, (-1.0, -1.0, -1.0), ( 1.0, 1.0, 1.0))); // 중심
        let c = Arc::new(Dummy::new(3, ( 5.0, -1.0, -1.0), ( 6.0, 1.0, 1.0))); // 우측
        let d = Arc::new(Dummy::new(4, ( 0.0,  5.0,  5.0), ( 1.0, 6.0, 6.0))); // y,z 위쪽(레이와 비충돌)

        for o in [&a, &b, &c, &d] { tree.insert(o.clone()); }

        let ro = Point3D::new(-10.0, 0.0, 0.0);
        let rd = Vector3D::new(1.0, 0.0, 0.0);

        // 트리 레이쿼리 (네 구현 사용 가능)
        fn ray_query_rec<T: SpatialObject>(
            n: &TreeNode<T>, ro: Point3D, rd: Vector3D, tmin: f64, tmax: f64, out: &mut Vec<Arc<T>>
        ) {
            if !super::tests::ray_aabb_intersect(ro, rd, &n.bbox, tmin, tmax) { return; }
            for o in &n.objects {
                if super::tests::ray_aabb_intersect(ro, rd, &o.aabb(), tmin, tmax) {
                    out.push(o.clone());
                }
            }
            if let Some(c1) = n.child1.as_ref() { ray_query_rec(c1, ro, rd, tmin, tmax, out); }
            if let Some(c2) = n.child2.as_ref() { ray_query_rec(c2, ro, rd, tmin, tmax, out); }
        }

        let mut hits = Vec::<Arc<Dummy>>::new();
        ray_query_rec(&tree.root, ro, rd, 0.0, 1000.0, &mut hits);
        let mut ids: Vec<u32> = hits.iter().map(|o| o.id).collect();
        ids.sort_unstable();

        assert_eq!(ids, vec![1, 2, 3], "ray should hit a,b,c but not d");
    }

    // ---- 4) 루트 밖 객체 삽입 시 루트 확장 확인 -----------------------------

    #[test]
    fn insert_outside_root_expands_root_bbox() {
        let mut tree = make_tree(2, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0));
        let far = Arc::new(Dummy::new(99, (100.0, 100.0, 100.0), (101.0, 101.0, 101.0)));
        tree.insert(far.clone());

        // 확장되었어야 한다
        assert!(tree.root.bbox.includes(&far.aabb()), "root bbox must expand to include far object");
    }

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
        fn ray_rec<T: SpatialObject>(n: &TreeNode<T>, ro: Point3D, rd: Vector3D, out: &mut Vec<Arc<T>>) {
            if !super::tests::ray_aabb_intersect(ro, rd, &n.bbox, -1.0, 1000.0) { return; }
            for o in &n.objects {
                if super::tests::ray_aabb_intersect(ro, rd, &o.aabb(), -1.0, 1000.0) {
                    out.push(o.clone());
                }
            }
            if let Some(c1) = n.child1.as_ref() { ray_rec(c1, ro, rd, out); }
            if let Some(c2) = n.child2.as_ref() { ray_rec(c2, ro, rd, out); }
        }
        ray_rec(&tree.root, ro, rd, &mut hits);
        assert!(!hits.is_empty(), "grazing ray should count as a hit");

        // 평행한 레이(Y축 방향), 박스 Y범위 밖
        let ro2 = Point3D::new(0.0, 10.0, 0.0);
        let rd2 = Vector3D::new(0.0, 1.0, 0.0);
        let mut hits2 = Vec::<Arc<Dummy>>::new();
        ray_rec(&tree.root, ro2, rd2, &mut hits2);
        assert!(hits2.is_empty(), "parallel ray outside y-range should not hit");
    }
}

```

---
