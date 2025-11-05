# 🧩 ArrayPool & ArrayPoolContainer 설계 문서
## 📦 개요
ArrayPool과 ArrayPoolContainer는 다양한 타입의 배열 핸들러(ArrayHandler<T>)를 공통 인터페이스(ArrayComp)로 관리하기 위한 구조입니다.  
이 구조는 배열 크기 동기화, 버퍼 설정, 재할당, 클리어 등의 기능을 제공하며, 동적 타입 처리와 안전한 메모리 관리를 목표로 합니다.

## 🧠 핵심 컴포넌트

### ArrayComp (Trait)
```rust
pub trait ArrayComp {
    fn get_size(&self) -> usize;
    fn set_array_size(&mut self, size: usize);
    fn re_alloc(&mut self, size: usize);
    fn set_buffer_size(&mut self, size: usize);
    fn clear(&mut self);

    // 다운캐스팅 지원
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

### ArrayHandler<T>
- 제네릭 배열 핸들러
- Vec<T> 기반
- Default + Clone 제약
- ArrayComp 구현

### KeyIndexer
- Vec<i32> 기반 키 배열
- HashMap<i32, usize>로 인덱싱
- 배열 크기 동기화 기준 제공

## 🧱 구조 비교

| 항목             | ArrayPool                                | ArrayPoolContainer                         |
|------------------|-------------------------------------------|---------------------------------------------|
| 컴포넌트 저장 방식 | Weak<RefCell<dyn ArrayComp>>         | Rc<RefCell<dyn ArrayComp>>             |
| 소유권 유지 여부  | ❌ 외부 Rc가 drop되면 데이터 사라짐        | ✅ Pool이 직접 Rc를 보관하므로 유지됨       |
| 참조 방식         | Rc → Weak                                 | Rc 직접 보관                                |
| 접근 방식         | upgrade() 필요                            | Rc::as_ptr()로 직접 비교 가능               |
| 제거 방식         | upgrade() 후 포인터 비교                  | Rc::as_ptr()로 포인터 직접 비교             |
| 순환 참조 위험    | 없음 (Weak 사용)                          | 있음 (detach/clear로 관리 필요)             |

## 🔁 리팩토링 이유
기존 ArrayPool은 Weak 참조를 사용하여 외부에서 Rc가 drop되면 데이터가 사라지는 문제가 있었습니다.  
이를 해결하기 위해 ArrayPoolContainer는 Rc를 직접 보관하여 데이터를 영구적으로 유지할 수 있도록 설계되었습니다.

## 🧪 주요 메서드
### 컴포넌트 관리
```rust
fn attach_component(&mut self, comp: Rc<RefCell<dyn ArrayComp>>)
fn detach_component(&mut self, comp: Rc<RefCell<dyn ArrayComp>>)
fn detach_all_component(&mut self)
```

### 배열 동기화 및 관리
```rust
fn sync_array_size(&mut self) -> bool
fn re_alloc(&mut self, size: usize)
fn set_buffer_size(&mut self, size: usize)
fn clear(&mut self)
```

### 타입별 핸들러 접근
```rust
fn get_handler_at<T: 'static + Default + Clone>(&self, index: usize) -> Option<Rc<RefCell<ArrayHandler<T>>>>
fn get_handler_at_by_dyn(&self, index: usize) -> Option<Rc<RefCell<dyn ArrayComp>>>
```
## 🧪 사용 예시
```rust
let handler: Rc<RefCell<dyn ArrayComp>> =
    Rc::new(RefCell::new(ArrayHandler::<i32>::new()));

let mut pool = ArrayPoolContainer::new();
pool.attach_component(handler.clone());

pool.get_key_indexer().insert_key(1);
pool.sync_array_size();

handler.borrow_mut().set_value(0, 123);
println!("Value at 0: {}", handler.borrow().as_slice());
```
## 🧩 확장 방향
- 이름 기반 컴포넌트 관리
- 타입별 필터링 및 조회
- Subject 이벤트 연동
- ArrayHandler<T>에 대한 고속 매핑 및 캐시

## 📁 추천 파일 구조
```
src/
├── core/
│   ├── array_handler.rs
│   ├── array_pool.rs
│   ├── key_indexer.rs
│   └── subject.rs
tests/
├── arraypool_tests.rs
README.md
```

## 소스 코드
```rust
use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::core::array_handler::ArrayHandler;
use crate::core::key_indexer::KeyIndexer;
use crate::core::subject::Subject;

pub trait ArrayComp {
    fn get_size(&self) -> usize;
    fn set_array_size(&mut self, size: usize);
    fn re_alloc(&mut self, size: usize);
    fn set_buffer_size(&mut self, size: usize);
    fn clear(&mut self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```
```rust
impl<T: Default + Clone + 'static> ArrayComp for ArrayHandler<T> {
    fn get_size(&self) -> usize {
        self.get_size()
    }

    fn set_array_size(&mut self, size: usize) {
        self.set_array_size(size);
    }

    fn re_alloc(&mut self, size: usize) {
        if self.get_alloc_size() < size {
            self.set_array_size(size);
        }
    }

    fn set_buffer_size(&mut self, size: usize) {
        self.set_buffer_size(size);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

}
```
```rust
pub struct ArrayPool {
    subject: Subject,
    pub comps: Vec<Weak<RefCell<dyn ArrayComp>>>,
    key_indexer: KeyIndexer,
}
```
```rust
impl ArrayPool {
    pub fn new() -> Self {
        Self {
            subject: Subject::new(),
            comps: Vec::new(),
            key_indexer: KeyIndexer::default_new(),
        }
    }

    pub fn subject(&self) -> &Subject {
        &self.subject
    }
    pub fn subject_mut(&mut self) -> &mut Subject {
        &mut self.subject
    }

    pub fn get_key_indexer(&mut self) -> &mut KeyIndexer {
        &mut self.key_indexer
    }
    pub fn get_size(&self) -> usize {
        self.key_indexer.get_size()
    }

    pub fn attach_component(&mut self, comp: &Rc<RefCell<dyn ArrayComp>>) {
        let ptr = Rc::as_ptr(comp) as *const ();
        let exists = self.comps.iter().any(|w| {
            if let Some(s) = w.upgrade() {
                (Rc::as_ptr(&s) as *const ()) == ptr
            } else {
                false
            }
        });
        if !exists {
            self.comps.push(Rc::downgrade(comp));
        }
    }

    pub fn detach_component(&mut self, comp: &Rc<RefCell<dyn ArrayComp>>) {
        let ptr = Rc::as_ptr(comp) as *const ();
        self.comps.retain(|w| {
            if let Some(s) = w.upgrade() {
                (Rc::as_ptr(&s) as *const ()) != ptr
            } else {
                false
            }
        });
    }

    pub fn detach_all_component(&mut self) {
        self.comps.clear();
    }

    pub fn clear(&mut self) {
        self.compact();
        for w in &self.comps {
            if let Some(rc) = w.upgrade() {
                rc.borrow_mut().clear();
            }
        }
    }

    pub fn re_alloc(&mut self, size: usize) {
        self.compact();
        for w in &self.comps {
            if let Some(rc) = w.upgrade() {
                rc.borrow_mut().re_alloc(size);
            }
        }
    }

    pub fn set_buffer_size(&mut self, size: usize) {
        self.compact();
        for w in &self.comps {
            if let Some(rc) = w.upgrade() {
                rc.borrow_mut().set_buffer_size(size);
            }
        }
    }

    pub fn sync_array_size(&mut self) -> bool {
        self.compact();
        let size = self.key_indexer.get_size();
        for w in &self.comps {
            if let Some(rc) = w.upgrade() {
                if rc.borrow().get_size() < size {
                    rc.borrow_mut().set_array_size(size);
                }
            }
        }
        true
    }

    /// dead weak 정리
    pub fn compact(&mut self) {
        self.comps.retain(|w| w.upgrade().is_some());
    }
}

impl Default for ArrayPool {
    fn default() -> Self {
        Self::new()
    }
}

```
```rust
pub struct ArrayPoolContainer {
    subject: Subject,
    components: Vec<Rc<RefCell<dyn ArrayComp>>>,
    key_indexer: KeyIndexer,
}
```
```rust
impl ArrayPoolContainer {
    pub fn new() -> Self {
        Self {
            subject: Subject::new(),
            components: Vec::new(),
            key_indexer: KeyIndexer::default_new(),
        }
    }

    pub fn subject(&self) -> &Subject {
        &self.subject
    }

    pub fn subject_mut(&mut self) -> &mut Subject {
        &mut self.subject
    }

    pub fn get_key_indexer(&mut self) -> &mut KeyIndexer {
        &mut self.key_indexer
    }

    pub fn get_size(&self) -> usize {
        self.key_indexer.get_size()
    }

    pub fn attach_component(&mut self, comp: Rc<RefCell<dyn ArrayComp>>) {
        let ptr = Rc::as_ptr(&comp) as *const ();
        let exists = self.components.iter().any(|rc| Rc::as_ptr(rc) as *const () == ptr);
        if !exists {
            self.components.push(comp);
        }
    }

    pub fn detach_component(&mut self, comp: Rc<RefCell<dyn ArrayComp>>) {
        let ptr = Rc::as_ptr(&comp) as *const ();
        self.components.retain(|rc| Rc::as_ptr(rc) as *const () != ptr);
    }

    pub fn detach_all_component(&mut self) {
        self.components.clear();
    }

    pub fn clear(&mut self) {
        for rc in &self.components {
            rc.borrow_mut().clear();
        }
    }

    pub fn re_alloc(&mut self, size: usize) {
        for rc in &self.components {
            rc.borrow_mut().re_alloc(size);
        }
    }

    pub fn set_buffer_size(&mut self, size: usize) {
        for rc in &self.components {
            rc.borrow_mut().set_buffer_size(size);
        }
    }

    pub fn sync_array_size(&mut self) -> bool {
        let size = self.key_indexer.get_size();
        for rc in &self.components {
            if rc.borrow().get_size() < size {
                rc.borrow_mut().set_array_size(size);
            }
        }
        true
    }

    pub fn get_components(&self) -> &[Rc<RefCell<dyn ArrayComp>>] {
        &self.components
    }

    pub fn get_handler_at_by_type<T: 'static + Default + Clone>(&self, index: usize) -> Option<Rc<RefCell<ArrayHandler<T>>>> {
        let comp = self.components.get(index)?;
        let raw_ptr = Rc::as_ptr(comp) as *const RefCell<ArrayHandler<T>>;
        Some( unsafe { Rc::from_raw(raw_ptr).clone() } )
    }

    pub fn get_handler_at_by_dyn(&self, index: usize) -> Option<Rc<RefCell<dyn ArrayComp>>> {
        self.components.get(index).cloned()
    }

}

impl Default for ArrayPoolContainer {
    fn default() -> Self {
        Self::new()
    }
}
```

### 테스트 코드
```rust
#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use nurbslib::core::array_handler::ArrayHandler;
    use nurbslib::core::array_pool::{ArrayComp, ArrayPool, ArrayPoolContainer};
```
```rust
    #[test]
    fn test_array_pool1() {

        let handler: Rc<RefCell<dyn ArrayComp>> =
            Rc::new(RefCell::new(ArrayHandler::<i32>::new()));

        let mut pool = ArrayPool::new();
        pool.attach_component(&handler);

        // KeyIndexer 에 키 삽입
        pool.get_key_indexer().insert_key(10);
        pool.get_key_indexer().insert_key(20);

        // 배열 크기 동기화
        pool.sync_array_size();
        println!("Synced size: {}", handler.borrow().get_size());
    }
```
```rust
    #[test]
    fn test_array_pool2() {

        let handler : Rc<RefCell<dyn ArrayComp>> = Rc::new(RefCell::new(ArrayHandler::<i32>::new()));
        let mut pool = ArrayPool::new();
        pool.attach_component(&handler);

        if let Some(concrete) = handler.borrow_mut().as_any_mut().downcast_mut::<ArrayHandler<i32>>() {
            concrete.set_value(0, 42);
            concrete.set_value(1, 99);
            println!("Values: {:?}", concrete.as_slice());
        }
    }
```
```rust
    #[test]
    fn test_array_pool3() {


        let h1 : Rc<RefCell<dyn ArrayComp>> = Rc::new(RefCell::new(ArrayHandler::<i32>::with_buffer(8)));
        let h2 : Rc<RefCell<dyn ArrayComp>> = Rc::new(RefCell::new(ArrayHandler::<i32>::with_buffer(16)));

        let mut pool = ArrayPool::new();
        pool.attach_component(&h1);
        pool.attach_component(&h2);
        // 재할당
        pool.re_alloc(32);

        if let Some(concrete) = h1.borrow_mut().as_any_mut().downcast_mut::<ArrayHandler<i32>>() {
            println!("h1 alloc: {}", concrete.get_alloc_size());
        }

        if let Some(concrete) = h2.borrow_mut().as_any_mut().downcast_mut::<ArrayHandler<i32>>() {
            println!("h2 alloc: {}", concrete.get_alloc_size());
        }
    }
```
```rust
    #[test]
    fn test_array_pool4() {
        let mut pool = ArrayPool::new();
        let h1 : Rc<RefCell<dyn ArrayComp>> = Rc::new(RefCell::new(ArrayHandler::<i32>::new()));
        let h2 : Rc<RefCell<dyn ArrayComp>> = Rc::new(RefCell::new(ArrayHandler::<i32>::new()));
        pool.attach_component(&h1);
        pool.attach_component(&h2);
        // detach h1
        pool.detach_component(&h1);
        // compact 후 남은 컴포넌트 수 확인
        pool.compact();
        println!("Remaining components: {}", pool.comps.len());
    }
```
```rust
    #[test]
    fn test_array_pool5() {
        let handler : Rc<RefCell<dyn ArrayComp>> = Rc::new(RefCell::new(ArrayHandler::<i32>::new()));
        let mut pool = ArrayPool::new();
        pool.attach_component(&handler);

        // 값 설정
        if let Some(handle_i32) = handler.borrow_mut().as_any_mut().downcast_mut::<ArrayHandler<i32>>() {

            handle_i32.set_array(&[1, 2, 3, 4, 5]);
        }

        // 버퍼 크기 설정
        pool.set_buffer_size(100);

        // 클리어
        pool.clear();
        if let Some(handle_i32) = handler.borrow_mut().as_any_mut().downcast_mut::<ArrayHandler<i32>>() {

            println!("After clear: {:?}", handle_i32.as_slice());
        }
    }
```
```rust
    #[test]
    fn test_array_pool6() {
        let handler : Rc<RefCell<dyn ArrayComp>>  = Rc::new(RefCell::new(ArrayHandler::<i32>::new()));
        let mut pool = ArrayPool::new();
        pool.attach_component(&handler);

        // KeyIndexer 에 키 삽입
        for k in 0..10 {
            pool.get_key_indexer().insert_key(k);
        }

        // 자동 크기 동기화
        pool.sync_array_size();
        println!("Handler size after sync: {}", handler.borrow().get_size());
    }
```
```rust
    #[test]
    fn test_array_pool7() {
        let h_int : Rc<RefCell<dyn ArrayComp>>  = Rc::new(RefCell::new(ArrayHandler::<i32>::new()));
        let h_float : Rc<RefCell<dyn ArrayComp>>  = Rc::new(RefCell::new(ArrayHandler::<f32>::new())); // 트레잇 객체로 사용하려면 ArrayComponent 구현 필요

        let mut pool = ArrayPool::new();
        pool.attach_component(&h_int);
        // pool.attach_component(&h_float); // f32용 구현이 필요함

        pool.get_key_indexer().insert_key(1);
        pool.sync_array_size();

        println!("{:?}", pool.get_size());

    }
```
```rust
    #[test]
    fn test_array_pool8()
    {
        // 1. ArrayHandler 생성
        let handler : Rc<RefCell<dyn ArrayComp>>  = Rc::new(RefCell::new(ArrayHandler::<i32>::new()));

        // 2. ArrayPool 생성 및 컴포넌트 등록
        let mut pool = ArrayPool::new();
        pool.attach_component(&handler);

        // 3. KeyIndexer에 키 삽입 (크기 = 5)
        for key in 0..5 {
            pool.get_key_indexer().insert_key(key);
        }

        // 4. 배열 크기 동기화
        pool.sync_array_size();

        // 5. handler에 값 설정
        {
            if let Some(handle_i32) = handler.borrow_mut().as_any_mut().downcast_mut::<ArrayHandler<i32>>() {

                for i in 0..5 {
                    handle_i32.set_value(i, (i as i32 + 10)); // 10, 11, 12, 13, 14
                }
            }
        }

        // 6. ArrayPool을 통해 값 읽기
        {
            if let Some(rc) = pool.comps[0].upgrade() {
                if let Some(handle_i32) = rc.borrow_mut().as_any_mut().downcast_mut::<ArrayHandler<i32>>() {
                    println!("ArrayHandler values via ArrayPool:");
                    for (i, v) in handle_i32.as_slice().iter().enumerate() {
                        println!("  [{}] = {}", i, v);
                    }
                }

            }
        }
    }
```
```rust
    #[test]
    fn test_array_pool9() {
        let mut pool = ArrayPoolContainer::new();
        {
            let handler1: Rc<RefCell<dyn ArrayComp>> =
                Rc::new(RefCell::new(ArrayHandler::<i32>::new()));

            let handler2: Rc<RefCell<dyn ArrayComp>> =
                Rc::new(RefCell::new(ArrayHandler::<f64>::new()));

            pool.attach_component(handler1.clone());
            pool.attach_component(handler2.clone());


            if let Some(concrete) = handler1.borrow_mut().as_any_mut().downcast_mut::<ArrayHandler<i32>>() {
                concrete.set_value(0, 123);
                println!("Value at 0: {:?}", concrete.as_slice());
            }
            pool.detach_component(handler1);
        }

        pool.get_key_indexer().insert_key(1);
        pool.sync_array_size();


        if let Some(handler_i32) = pool.get_handler_at_by_type::<i32>(0) {
            let mut h = handler_i32.borrow_mut();
            h.set_value(0, 777);
            println!("i32 value at 0: {:?}", h.as_slice());
            //pool.detach_component(handler_i32);
        }

        if let Some(handler_f64) = pool.get_handler_at_by_type::<f64>(1) {
            let mut h = handler_f64.borrow_mut();
            h.set_value(0, 3.14);
            println!("f32 value at 0: {:?}", h.as_slice());
            //pool.detach_component(handler_f64);
        }
    }
}
```


