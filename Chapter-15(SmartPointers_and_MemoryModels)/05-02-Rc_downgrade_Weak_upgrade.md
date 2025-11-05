# 🧠 `Rc::downgrade` 와 `Weak::upgrade` 란?
## 🔗 Rc::downgrade
- Rc<T> → Weak<T>로 변환합니다.
- Weak<T>는 Rc<T>의 비소유 참조입니다. 즉, 참조 카운트를 증가시키지 않아요.
- 순환 참조(cyclic reference)를 피할 때 유용합니다.

```rust
use std::rc::{Rc, Weak};

let rc = Rc::new(5);
let weak = Rc::downgrade(&rc); // weak는 rc를 참조하지만 소유하지 않음
```

## 🔄 Weak::upgrade
- Weak<T> → Option<Rc<T>>로 변환합니다.
- 참조 대상이 아직 살아 있다면 Some(Rc<T>)를 반환하고, 그렇지 않으면 None을 반환합니다.
```rust
if let Some(strong_rc) = weak.upgrade() {
    println!("Value: {}", strong_rc);
} else {
    println!("The value has been dropped.");
}
```
---

## 참고 소스
### `upgrade`
```rust
pub fn attach_component(&mut self, comp: &Rc<RefCell<dyn ArrayComponent>>) {
    let ptr = Rc::as_ptr(comp) as *const ();
    let exists = self.components.iter().any(|w| {
        if let Some(s) = w.upgrade() {
            (Rc::as_ptr(&s) as *const ()) == ptr
        } else {
            false
        }
    });
    if !exists {
        self.components.push(Rc::downgrade(comp));
    }
}
```
```rust
pub fn detach_component(&mut self, comp: &Rc<RefCell<dyn ArrayComponent>>) {
    let ptr = Rc::as_ptr(comp) as *const ();
    self.components.retain(|w| {
        if let Some(s) = w.upgrade() {
            (Rc::as_ptr(&s) as *const ()) != ptr
        } else {
            false
        }
    });
}
```
```rust
pub fn clear(&mut self) {
    self.compact();
    for w in &self.components {
        if let Some(rc) = w.upgrade() {
            rc.borrow_mut().clear();
        }
    }
}
```
```rust
pub fn re_alloc(&mut self, size: usize) {
    self.compact();
    for w in &self.components {
        if let Some(rc) = w.upgrade() {
            rc.borrow_mut().re_alloc(size);
        }
    }
}
```
```rust
pub fn set_buffer_size(&mut self, size: usize) {
    self.compact();
    for w in &self.components {
        if let Some(rc) = w.upgrade() {
            rc.borrow_mut().set_buffer_size(size);
        }
    }
}
```
```rust
pub fn sync_array_size(&mut self) -> bool {
    self.compact();
    let size = self.key_indexer.get_size();
    for w in &self.components {
        if let Some(rc) = w.upgrade() {
            if rc.borrow().get_size() < size {
                rc.borrow_mut().set_array_size(size);
            }
        }
    }
    true
}
```
```rust
/// dead weak 정리
fn compact(&mut self) {
    self.components.retain(|w| w.upgrade().is_some());
}
```

### `downgrade`
```rust
pub fn attach_component(&mut self, comp: &Rc<RefCell<dyn ArrayComponent>>) {
    let ptr = Rc::as_ptr(comp) as *const ();
    let exists = self.components.iter().any(|w| {
        if let Some(s) = w.upgrade() {
            (Rc::as_ptr(&s) as *const ()) == ptr
        } else {
            false
        }
    });
    if !exists {
        self.components.push(Rc::downgrade(comp));
    }
}
```
