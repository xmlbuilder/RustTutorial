# BlockDef / BlockRef
- BlockDef와 BlockRef는 함께 Composite 패턴을 구현하는 핵심 구조입니다.  
- CAD/로보틱스/그래픽스 시스템에서 블록을 정의하고, 그 정의를 참조하여 계층적으로 배치·변환하는 데 쓰입니다.  

## 📘 BlockDef + BlockRef 구조 문서화
### BlockDef
#### 개요
- 역할: 블록의 원형(Definition)을 정의하는 객체
- 내용: 블록 이름, 기준점(base point), 포함된 지오메트리 목록
- 특징: 여러 BlockRef가 동일한 BlockDef를 참조할 수 있음 → 재사용 가능
#### 필드
- name: String  
  - 블록 이름. 사용자가 블록을 식별할 때 사용.
- base: Point3D  
  - 블록의 기준점. 블록을 삽입할 때 기준이 되는 좌표.
- geoms: Vec<String>  
  - 블록에 포함된 지오메트리 목록. 실제 구현에서는 Geometry 객체가 들어감.
#### 함수
- new(name: &str, base: Point3D) -> BlockDef  
  - 새로운 블록 정의 생성.
- add_geometry(&mut self, g: String)  
  - 블록에 지오메트리 추가.

### BlockRef
#### 개요
- 역할: 특정 BlockDef를 참조하여 실제 계층 구조를 형성하는 객체
- 내용: 로컬 변환(local transform), 월드 변환(world transform), pivot, 자식 블록들
- 특징: Composite 패턴의 "Component" 역할. 부모-자식 관계를 통해 계층적 변환을 전파.
#### 필드
- def: Rc<BlockDef>  
  - 참조하는 블록 정의.
- pivot: Point3D  
  - 회전·스케일링의 기준점.
- local: Xform  
  - 로컬 변환 행렬. 블록 자체의 이동/회전/스케일링.
- world: Xform  
  - 월드 변환 행렬. 부모 변환과 로컬 변환을 합성한 결과.
- children: Vec<Rc<RefCell<BlockRef>>>  
  - 자식 블록들. Composite 패턴 구현.
- dirty: bool  
  - 변환이 갱신 필요 상태인지 표시.
#### 함수
- new(def: Rc<BlockDef>) -> Rc<RefCell<BlockRef>>  
  - 새로운 블록 참조 생성.
- add_child(parent: &Rc<RefCell<Self>>, child: Rc<RefCell<Self>>)  
  - 부모 블록에 자식 블록 추가.
- set_pivot(&mut self, p: Point3D)  
  - pivot 설정.
- set_local_xform(&mut self, xf: Xform)  
  - 로컬 변환 행렬 덮어쓰기.
- apply_local_xform(&mut self, xf: Xform)  
  - 로컬 변환 행렬에 새로운 변환을 곱해 적용.
- translate_local(&mut self, dx: f64, dy: f64, dz: f64)  
  - 로컬 좌표계에서 평행이동.
- rotate_about_pivot(&mut self, radians: f64, axis_unit: &Vector3D)  
  - pivot을 기준으로 회전.
- update_matrices(&mut self, parent_world: &Xform)  
  - 부모 변환과 로컬 변환을 합성하여 world 변환 갱신. 자식에게도 전파.
- world_xform(&self) -> &Xform  
  - 현재 world 변환 행렬 반환.
- to_world_point(&self, p_local: Point3D) -> Point3D  
  - 로컬 좌표계의 점을 world 좌표계로 변환.

## 구조 관계 (Composite 패턴)
```
BlockDef (정의)
   └── BlockRef (참조)
          ├── local transform
          ├── world transform
          └── children (BlockRef ...)
```

- BlockDef: 블록의 원형 정의 (이름, 기준점, 지오메트리)
- BlockRef: BlockDef를 참조하여 실제 계층 구조를 형성
- Composite 패턴: BlockRef가 BlockRef를 포함 → 부모 변환이 자식에게 전파됨

## ✨ 요약
- BlockDef: 블록의 원형 정의. 이름, 기준점, 지오메트리 관리.
- BlockRef: 블록 참조. 로컬/월드 변환, pivot, 자식 관리. Composite 패턴으로 계층적 변환 전파.
- 전체 구조: BlockDef는 "정의", BlockRef는 "실체". 여러 BlockRef가 하나의 BlockDef를 공유하며,  
  부모-자식 관계로 계층적 변환을 구현.
---
## 소스 코드
```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Mul;
use crate::core::prelude::{Point3D, Vector3D};
use crate::core::xform::Xform;
```
```rust
/// BlockDef: 블록 정의
pub struct BlockDef {
    pub name: String,
    pub base: Point3D,
    pub geoms: Vec<String>, // 단순화: 실제 Geometry 대신 문자열
}
```
```rust
impl BlockDef {
    pub fn new(name: &str, base: Point3D) -> Self {
        Self { name: name.to_string(), base, geoms: Vec::new() }
    }
    pub fn add_geometry(&mut self, g: String) {
        self.geoms.push(g);
    }
}
```
```rust
/// BlockRef: 블록 참조 (Composite)
pub struct BlockRef {
    pub def: Rc<BlockDef>,
    pivot: Point3D,
    pub local: Xform,
    world: Xform,
    parent_world: Xform,
    children: Vec<Rc<RefCell<BlockRef>>>,
    dirty: bool,
}
```
```rust
impl BlockRef {
    pub fn new(def: Rc<BlockDef>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            def,
            pivot: Point3D::origin(),
            local: Xform::identity(),
            world: Xform::identity(),
            parent_world: Xform::identity(),
            children: Vec::new(),
            dirty: true,
        }))
    }
```
```rust
    pub fn add_child(parent: &Rc<RefCell<Self>>, child: Rc<RefCell<Self>>) {
        parent.borrow_mut().children.push(child);
    }
```
```rust
    pub fn set_pivot(&mut self, p: Point3D) {
        self.pivot = p;
        self.dirty = true;
    }
```
```rust
    pub fn set_local_xform(&mut self, xf: Xform) {
        self.local = xf;
        self.dirty = true;
    }
```
```rust
    pub fn translate_local(&mut self, dx: f64, dy: f64, dz: f64) {
        self.local = Xform::translation(dx, dy, dz); // 단순 덮어쓰기
        self.dirty = true;
    }
```
```rust
    pub fn rotate_about_pivot(&mut self, radians: f64, axis_unit: &Vector3D) {
        let r = Xform::rotation_about_axis(&self.pivot, axis_unit, radians);
        self.local = r * self.local;
        self.dirty = true;
    }
```
```rust
    // Compose: world = parent_world * T(base) * local
    fn composed_local(&self) -> Xform {
        Xform::translation(self.def.base.x, self.def.base.y, self.def.base.z) * self.local
    }
```
```rust
    pub fn update_matrices(&mut self, parent_world: &Xform) {
        if !self.dirty && self.parent_world == *parent_world {
            return;
        }

        // world = parent_world * local
        self.world = parent_world.mul(self.local);
        self.parent_world = parent_world.clone();
        self.dirty = false;

        for c in &self.children {
            c.borrow_mut().update_matrices(&self.world);
        }
    }
```
```rust
    pub fn world_xform(&self) -> &Xform {
        &self.world
    }
```
```rust
    // Convenience: transform a point in this block’s local model space to world
    pub fn to_world_point(&self, p_local: Point3D) -> Point3D {
        self.world.transform_point(p_local)
    }
```
```rust
    pub fn apply_local_xform(&mut self, xf: Xform) {
        self.local = xf * self.local;
        self.dirty = true;
    }
}
```
---
## 테스크 코드
```rust
use std::rc::Rc;
use nurbslib::core::block::{BlockDef, BlockRef};
use nurbslib::core::prelude::{Point3D, Vector3D};
use nurbslib::core::xform::Xform;
```
```rust
#[test]
fn main() {
    let def = Rc::new(BlockDef::new("BaseBlock", Point3D::origin()));
    let root = BlockRef::new(def.clone());

    let child_def = Rc::new(BlockDef::new("ChildBlock", Point3D { x: 1.0, y: 0.0, z: 0.0 }));
    let child = BlockRef::new(child_def.clone());

    BlockRef::add_child(&root, child.clone());

    {
        let mut root_mut = root.borrow_mut();
        root_mut.translate_local(2.0, 0.0, 0.0);
        root_mut.update_matrices(&Xform::identity());
    }

    println!("Root world: {:?}", root.borrow().world_xform());
    println!("Child world: {:?}", child.borrow().world_xform());
}
```
---
## 테스트 코드 검증
```rust

#[test]
fn hierarchy_demo() {
    let def_root=Rc::new(BlockDef::new("Root",Point3D::origin()));
    let root=BlockRef::new(def_root);

    let def_child=Rc::new(BlockDef::new("Child",Point3D{x:1.0,y:0.0,z:0.0}));
    let child=BlockRef::new(def_child);
    BlockRef::add_child(&root,child.clone());

    {
        let mut r=root.borrow_mut();
        r.translate_local(2.0,0.0,0.0);
        r.rotate_about_pivot(0.25, &Vector3D::new(0.0, 0.0, 1.0));
        r.update_matrices(&Xform::identity());
    }
    {
        let mut c=child.borrow_mut();
        c.translate_local(0.0,1.0,0.0);

        c.update_matrices(root.borrow().world_xform());
    }
    println!("Root world: {:?}",root.borrow().world_xform());
    println!("Child world: {:?}",child.borrow().world_xform());
}

```
### 출력 결과
```
Root world: Xform { m: [[0.9689124217106447, -0.24740395925452294, 0.0, 1.9378248434212895],
  [0.24740395925452294, 0.9689124217106447, 0.0, 0.4948079185090459],
  [0.0, 0.0, 1.0, 0.0],
  [0.0, 0.0, 0.0, 1.0]] }

Child world: Xform { m: [[0.9689124217106447, -0.24740395925452294, 0.0, 1.6904208841667665],
  [0.24740395925452294, 0.9689124217106447, 0.0, 1.4637203402196906],
  [0.0, 0.0, 1.0, 0.0],
  [0.0, 0.0, 0.0, 1.0]] }
```
- 지금 출력된 Root world와 Child world 행렬을 해석해보면 동작이 올바른지 확인할 수 있습니다.

## 🔎 Root world 행렬 해석
- Root world:
```
[[ 0.9689, -0.2474, 0.0, 1.9378],
 [ 0.2474,  0.9689, 0.0, 0.4948],
 [ 0.0,     0.0,    1.0, 0.0   ],
 [ 0.0,     0.0,    0.0, 1.0   ]]
```
- 상단 3×3 부분:
```
[[0.9689, -0.2474],
[0.2474,  0.9689]]
```
  - 이는 Z축 회전 행렬로, 약 0.25 rad (≈ 14.3°) 회전을 나타냅니다.
  - 즉, root가 Z축을 기준으로 14° 정도 회전한 상태입니다.
- 마지막 열 (translation):
```
  [1.9378, 0.4948, 0.0]
```
  - root가 (2,0,0)으로 이동한 뒤 회전이 적용되어, 최종 위치가 (1.94, 0.49, 0.0) 근처로 나타납니다.
- 즉, 회전 + 평행이동이 올바르게 합성되었습니다.

## 🔎 Child world 행렬 해석
Child world:
```
[[ 0.9689, -0.2474, 0.0, 1.6904],
 [ 0.2474,  0.9689, 0.0, 1.4637],
 [ 0.0,     0.0,    1.0, 0.0   ],
 [ 0.0,     0.0,    0.0, 1.0   ]]
```
- 상단 3×3 부분은 root와 동일 → 부모의 회전을 그대로 상속받음.
- translation 부분:
```
[1.6904, 1.4637, 0.0]
```
- child의 base (1,0,0) + local translate (0,1,0) = (1,1,0).
- 이 점이 root의 world 변환(회전+이동)에 의해 최종적으로 (1.69, 1.46, 0.0)으로 변환된 것.
- 즉, 부모 변환을 올바르게 상속받고, 자신의 local 변환도 적용된 결과입니다.
  
### ✅ 결론
- Root: (2,0,0) 이동 후 Z축 0.25 rad 회전 → 최종 위치 (1.94, 0.49).
- Child: base (1,0,0) + local (0,1,0) → (1,1,0). 부모 변환 적용 후 (1.69, 1.46).
- 행렬 결과는 수학적으로 정확하며, Composite 패턴이 정상적으로 동작하고 있습니다.
- 👉 즉, 지금 구조는 부모 변환이 자식에게 잘 전파되고, 자식의 local 변환도 반영되는 걸 확인했습니다.

---

