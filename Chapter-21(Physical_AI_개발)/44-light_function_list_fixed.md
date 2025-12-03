## 📄 임베디드용 LightFunctionListFixed 설계 문서
### 1. 설계 의도
- 임베디드 장비에서 동작하는 경량 함수 리스트 컨테이너를 제공하기 위해 설계.
- 동적 메모리 할당 없이 **고정 크기 배열(MAX)** 을 기반으로 동작.
- **락(IntLock32)** 을 이용해 간단한 동시성 제어를 지원.
- 콜백 함수와 파라미터를 저장하고, 리스트 형태로 관리하며, 순차 호출 가능.

### 2. 구성 요소
- 2.1 IntLock32
    - 단순한 불리언 플래그 기반 락 구조체.
    - 주요 메서드:
    - get_default_lock() → 락 획득 시도, 성공하면 true 반환.
    - return_default_lock() → 락 반환.
    - is_locked_flag() → 현재 락 상태 확인.
    - 목적: 멀티스레드 환경이 아닌 임베디드 환경에서 간단한 임계영역 보호.
- 2.2 Node
    - 리스트의 기본 단위.
    - 필드:
    - prev, next → 양방향 연결 리스트 인덱스.
    - func → 콜백 함수 포인터.
    - param → 콜백 파라미터(u64).
- 2.3 LightFunctionListFixed<MAX>
    - 고정 크기(MAX) 배열 기반 함수 리스트.
    - 주요 필드:
    - head, tail → 리스트의 시작과 끝.
    - free_list → 사용 가능한 노드 인덱스 관리.
    - nodes[MAX] → 실제 노드 배열.
    - lock → IntLock32 기반 락.

### 3. 동작 방식
- 초기화(new)
    - nodes 배열을 초기화하고, 모든 노드를 free_list에 연결.
    - head/tail은 None으로 시작.
- 노드 할당/해제
    - allocate_node() → free_list에서 하나 꺼내 사용.
    - free_node(idx) → 노드를 free_list로 반환.
    - unlink_and_free(idx) → 리스트에서 제거 후 free_list로 반환.
- 함수 추가(add_function)
    - 락 획득 → free_list에서 노드 할당 → 콜백과 파라미터 저장 → tail에 연결 → 락 반환.
    - 용량 초과 시 ListResult::Full 반환.
- 함수 제거(remove_function, remove_function_pair)
    - 특정 함수 또는 함수+파라미터 쌍을 찾아 제거.
    - 노드를 free_list로 반환.
- 조회(is_in_list, is_in_list_pair)
    - 특정 함수 또는 함수+파라미터가 리스트에 존재하는지 확인.
- 호출(call_functions)
    - head→tail 또는 tail→head 방향으로 순차적으로 콜백 실행.
    - 파라미터는 u64로 전달.
- 기타
    - empty_list() → 전체 리스트 비우기.
    - count() → 현재 리스트에 등록된 함수 개수.
    - is_in_use() → 락 상태 확인.

4. 장점
- 임베디드 친화적: 동적 메모리 없음, 고정 배열 기반.
- 경량화: 단순한 구조체와 함수 포인터만 사용.
- 락 지원: IntLock32로 간단한 임계영역 보호.
- 양방향 리스트: 앞뒤 순회 가능.
- **결과 코드(enum ListResult)** 로 상태를 명확히 반환.

### 5. 샘플 사용
```rust
fn cb_print(p: u64) {
    println!("cb_print: {}", p);
}

fn main() {
    let mut fl: LightFunctionListFixed<4> = LightFunctionListFixed::new();

    fl.add_function(cb_print, 10);
    fl.add_function(cb_print, 20);

    assert_eq!(fl.count(), 2);

    fl.call_functions(true);  // head→tail: 10, 20
    fl.call_functions(false); // tail→head: 20, 10

    fl.remove_function_pair(cb_print, 20);
    assert_eq!(fl.count(), 1);

    fl.empty_list();
    assert_eq!(fl.count(), 0);
}
```


### ✅ 결론
이 구조는 임베디드 장비에서 콜백 함수 리스트를 관리하기에 적합합니다.
- 고정 크기 배열 기반 → 메모리 예측 가능
- 락 지원 → 간단한 동시성 보호
- 함수 등록/제거/호출 기능 제공
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
```rust
```
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
```rust
```
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

