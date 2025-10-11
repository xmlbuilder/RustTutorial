# SpatialTree Onboarding Guide

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
