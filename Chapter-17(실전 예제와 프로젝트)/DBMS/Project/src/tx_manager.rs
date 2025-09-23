use crate::define::TxAction;
use crate::tx_delta_list::TxDeltaList;

#[derive(Default, Clone)]
pub struct TxManager {
    undo_stack: Vec<TxDeltaList>,
    redo_stack: Vec<TxDeltaList>,
    current: TxDeltaList,
}

impl TxManager {
    pub fn new() -> Self {
        TxManager {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current: TxDeltaList::new(),
        }
    }

    /// 현재 트랜잭션에 액션 추가
    pub fn add(&mut self, action: TxAction) {
        self.current.add(action);
    }

    /// 커밋: 현재 변경사항을 undo 스택에 저장
    pub fn commit(&mut self) {
        if self.current.count() > 0 {
            self.undo_stack.push(std::mem::take(&mut self.current));
            self.redo_stack.clear(); // 커밋 시 redo 초기화
        }
    }

    /// Undo: 마지막 변경사항을 되돌림
    pub fn undo(&mut self) -> Option<TxDeltaList> {
        if let Some(delta) = self.undo_stack.pop() {
            self.redo_stack.push(delta.clone());
            Some(delta)
        } else {
            None
        }
    }

    /// Redo: 마지막 undo를 다시 적용
    pub fn redo(&mut self) -> Option<TxDeltaList> {
        if let Some(delta) = self.redo_stack.pop() {
            self.undo_stack.push(delta.clone());
            Some(delta)
        } else {
            None
        }
    }

    /// 전체 초기화
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current.clear();
    }

    /// 현재 트랜잭션 액션 수
    pub fn current_count(&self) -> usize {
        self.current.count()
    }

    pub fn has_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn has_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
