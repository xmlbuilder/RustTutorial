use std::collections::HashSet;
use crate::define::TxAction;
use crate::item::Cursor;

#[derive(Default, Clone)]
pub struct TxDeltaList {
    pub actions: Vec<TxAction>,
    pub keys: HashSet<i32>,
}

impl TxDeltaList {
    pub fn new() -> Self {
        TxDeltaList {
            actions: Vec::new(),
            keys: HashSet::new(),
        }
    }

    /// TxAction 추가 (중복 키 방지)
    pub fn add(&mut self, action: TxAction) {
        let key = match &action {
            TxAction::Insert(c) => c.key(),
            TxAction::Remove(c) => c.key(),
            TxAction::Modify { after, .. } => after.key(),
            TxAction::Cancelled => return, // 무시
        };

        if self.keys.insert(key) {
            self.actions.push(action);
        }
    }

    /// 전체 초기화
    pub fn clear(&mut self) {
        self.actions.clear();
        self.keys.clear();
    }

    /// TxAction 수
    pub fn count(&self) -> usize {
        self.actions.len()
    }

    /// 읽기 전용 반복자
    pub fn iter(&self) -> impl Iterator<Item = &TxAction> {
        self.actions.iter()
    }

    /// 가변 반복자
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TxAction> {
        self.actions.iter_mut()
    }

    /// 살아있는 커서만 반환
    pub fn find_alive(&self) -> Vec<&Cursor> {
        self.actions.iter().filter_map(|action| {
            match action {
                TxAction::Insert(c) | TxAction::Remove(c) => {
                    if c.is_alive() {
                        Some(c)
                    } else {
                        None
                    }
                }
                TxAction::Modify { after, .. } => {
                    if after.is_alive() {
                        Some(after)
                    } else {
                        None
                    }
                }
                TxAction::Cancelled => None,
            }
        }).collect()
    }

    /// 키로 커서 찾기
    pub fn find_by_key(&self, key: i32) -> Option<&Cursor> {
        self.actions.iter().find_map(|action| {
            match action {
                TxAction::Insert(c) | TxAction::Remove(c) => {
                    if c.key() == key {
                        Some(c)
                    } else {
                        None
                    }
                }
                TxAction::Modify { after, .. } => {
                    if after.key() == key {
                        Some(after)
                    } else {
                        None
                    }
                }
                TxAction::Cancelled => None,
            }
        })
    }
}