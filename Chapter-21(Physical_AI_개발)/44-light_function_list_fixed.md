
## 소스 코드
```rust
use std::fmt;
use crate::core::int_lock32::IntLock32;

pub type Callback      = fn(u64);
pub type CallbackParam = u64;

#[derive(Debug, Clone, Copy)]
struct Node {
    prev: Option<usize>,
    next: Option<usize>,
    func: Option<Callback>,
    param: CallbackParam,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            prev: None,
            next: None,
            func: None,
            param: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListResult {
    InUse,         // 락 실패
    Ok,            // 성공
    NotFoundOrBad, // 잘못된 인자 / 찾을 수 없음
    Full,          // 용량 초과
}

pub struct LightFunctionListFixed<const MAX: usize> {
    head: Option<usize>,
    tail: Option<usize>,
    free_list: Option<usize>,
    nodes: [Node; MAX],
    lock: IntLock32,
}

impl<const MAX: usize> fmt::Debug for LightFunctionListFixed<MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LightFunctionListFixed")
            .field("head", &self.head)
            .field("tail", &self.tail)
            .field("free_list", &self.free_list)
            .finish()
    }
}

impl<const MAX: usize> LightFunctionListFixed<MAX> {
    pub fn new() -> Self {
        // Node는 Copy + Default 이므로 [Default; MAX] 가능
        let mut nodes = [Node::default(); MAX];

        // free_list 초기화: 0..MAX-1을 단일 연결 리스트로 연결
        let mut free_list: Option<usize> = None;
        for i in 0..MAX {
            nodes[i].next = free_list;
            nodes[i].prev = None;
            nodes[i].func = None;
            nodes[i].param = 0;
            free_list = Some(i);
        }

        Self {
            head: None,
            tail: None,
            free_list,
            nodes,
            lock: IntLock32::new(),
        }
    }

    pub const fn max_size() -> usize {
        MAX
    }

    fn allocate_node(&mut self) -> Option<usize> {
        let idx = self.free_list?;
        let next = self.nodes[idx].next;
        self.free_list = next;

        let n = &mut self.nodes[idx];
        n.prev = None;
        n.next = None;
        n.func = None;
        n.param = 0;

        Some(idx)
    }

    fn free_node(&mut self, idx: usize) {
        let n = &mut self.nodes[idx];
        n.func = None;
        n.param = 0;
        n.prev = None;
        n.next = self.free_list;
        self.free_list = Some(idx);
    }

    fn unlink_and_free(&mut self, idx: usize) {
        let (prev, next) = {
            let n = &self.nodes[idx];
            (n.prev, n.next)
        };

        if let Some(p) = prev {
            self.nodes[p].next = next;
        } else {
            self.head = next;
        }

        if let Some(nxt) = next {
            self.nodes[nxt].prev = prev;
        } else {
            self.tail = prev;
        }

        self.free_node(idx);
    }

    fn find_node_by_func(&self, func: Callback) -> Option<usize> {
        let mut cur = self.head;
        while let Some(i) = cur {
            if self.nodes[i].func == Some(func) {
                return Some(i);
            }
            cur = self.nodes[i].next;
        }
        None
    }

    pub fn add_function(&mut self, func: Callback, param: CallbackParam) -> ListResult {
        if func as usize == 0 {
            return ListResult::NotFoundOrBad;
        }
        if !self.lock.get_default_lock() {
            return ListResult::InUse;
        }

        let idx = match self.allocate_node() {
            Some(i) => i,
            None => {
                self.lock.return_default_lock();
                return ListResult::Full;
            }
        };

        {
            let node = &mut self.nodes[idx];
            node.func = Some(func);
            node.param = param;
            node.prev = self.tail;
            node.next = None;
        }

        if let Some(t) = self.tail {
            self.nodes[t].next = Some(idx);
        }
        self.tail = Some(idx);

        if self.head.is_none() {
            self.head = Some(idx);
        }

        self.lock.return_default_lock();
        ListResult::Ok
    }

    pub fn remove_function_pair(
        &mut self,
        func: Callback,
        param: CallbackParam,
    ) -> ListResult {
        if func as usize == 0 {
            return ListResult::NotFoundOrBad;
        }
        if !self.lock.get_default_lock() {
            return ListResult::InUse;
        }

        let mut cur = self.find_node_by_func(func);
        let mut rc = ListResult::NotFoundOrBad;

        while let Some(i) = cur {
            let n = &self.nodes[i];
            if n.func == Some(func) && n.param == param {
                self.unlink_and_free(i);
                rc = ListResult::Ok;
                break;
            }
            cur = n.next;
        }

        self.lock.return_default_lock();
        rc
    }

    pub fn remove_function(&mut self, func: Callback) -> ListResult {
        if func as usize == 0 {
            return ListResult::NotFoundOrBad;
        }
        if !self.lock.get_default_lock() {
            return ListResult::InUse;
        }

        let rc = if let Some(idx) = self.find_node_by_func(func) {
            self.unlink_and_free(idx);
            ListResult::Ok
        } else {
            ListResult::NotFoundOrBad
        };

        self.lock.return_default_lock();
        rc
    }

    pub fn is_in_list_pair(
        &mut self,
        func: Callback,
        param: CallbackParam,
    ) -> ListResult {
        if func as usize == 0 {
            return ListResult::NotFoundOrBad;
        }
        if !self.lock.get_default_lock() {
            return ListResult::InUse;
        }

        let mut cur = self.find_node_by_func(func);
        let mut rc = ListResult::NotFoundOrBad;

        while let Some(i) = cur {
            let n = &self.nodes[i];
            if n.func == Some(func) && n.param == param {
                rc = ListResult::Ok;
                break;
            }
            cur = n.next;
        }

        self.lock.return_default_lock();
        rc
    }

    pub fn is_in_list(&mut self, func: Callback) -> ListResult {
        if func as usize == 0 {
            return ListResult::NotFoundOrBad;
        }
        if !self.lock.get_default_lock() {
            return ListResult::InUse;
        }

        let rc = if self.find_node_by_func(func).is_some() {
            ListResult::Ok
        } else {
            ListResult::NotFoundOrBad
        };

        self.lock.return_default_lock();
        rc
    }

    pub fn empty_list(&mut self) -> bool {
        if !self.lock.get_default_lock() {
            return false;
        }

        let mut cur = self.head;
        while let Some(i) = cur {
            let next = self.nodes[i].next;
            self.free_node(i);
            cur = next;
        }

        self.head = None;
        self.tail = None;

        self.lock.return_default_lock();
        true
    }

    pub fn call_functions(&mut self, first_to_last: bool) -> bool {
        if !self.lock.get_default_lock() {
            return false;
        }

        if first_to_last {
            let mut cur = self.head;
            while let Some(i) = cur {
                let (f, p, next) = {
                    let n = &self.nodes[i];
                    (n.func, n.param, n.next)
                };
                if let Some(func) = f {
                    func(p);
                }
                cur = next;
            }
        } else {
            let mut cur = self.tail;
            while let Some(i) = cur {
                let (f, p, prev) = {
                    let n = &self.nodes[i];
                    (n.func, n.param, n.prev)
                };
                if let Some(func) = f {
                    func(p);
                }
                cur = prev;
            }
        }

        self.lock.return_default_lock();
        true
    }

    pub fn is_in_use(&self) -> bool {
        self.lock.is_locked_flag()
    }

    pub fn count(&mut self) -> usize {
        if !self.lock.get_default_lock() {
            return 0;
        }

        let mut cnt = 0;
        let mut cur = self.head;
        while let Some(i) = cur {
            cnt += 1;
            cur = self.nodes[i].next;
        }

        self.lock.return_default_lock();
        cnt
    }
}
```
## 샘플 코드
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::light_function_list_fixed::{LightFunctionListFixed, ListResult};

    fn cb_print(p: u64) {
        println!("cb_print: {}", p);
    }

    #[test]
    fn basic_add_call_count() {
        let mut fl: LightFunctionListFixed<4> = LightFunctionListFixed::new();

        let rc1 = fl.add_function(cb_print, 10);
        let rc2 = fl.add_function(cb_print, 20);
        assert_eq!(rc1, ListResult::Ok);
        assert_eq!(rc2, ListResult::Ok);

        assert_eq!(fl.count(), 2);
        assert!(fl.call_functions(true));  // 10, 20
        assert!(fl.call_functions(false)); // 20, 10
    }

    #[test]
    fn full_capacity() {
        let mut fl: LightFunctionListFixed<2> = LightFunctionListFixed::new();

        let rc1 = fl.add_function(cb_print, 1);
        let rc2 = fl.add_function(cb_print, 2);
        let rc3 = fl.add_function(cb_print, 3);

        assert_eq!(rc1, ListResult::Ok);
        assert_eq!(rc2, ListResult::Ok);
        assert_eq!(rc3, ListResult::Full);

        assert_eq!(fl.count(), 2);
    }

    #[test]
    fn remove_and_empty() {
        let mut fl: LightFunctionListFixed<4> = LightFunctionListFixed::new();

        fl.add_function(cb_print, 10);
        fl.add_function(cb_print, 20);
        fl.add_function(cb_print, 30);

        let rc = fl.is_in_list_pair(cb_print, 20);
        assert_eq!(rc, ListResult::Ok);

        let rc_rem = fl.remove_function_pair(cb_print, 20);
        assert_eq!(rc_rem, ListResult::Ok);

        let rc2 = fl.is_in_list_pair(cb_print, 20);
        assert_eq!(rc2, ListResult::NotFoundOrBad);

        let rc3 = fl.remove_function(cb_print);
        assert_eq!(rc3, ListResult::Ok);

        assert_eq!(fl.count(), 1);

        assert!(fl.empty_list());
        assert_eq!(fl.count(), 0);
    }
}
```
---
