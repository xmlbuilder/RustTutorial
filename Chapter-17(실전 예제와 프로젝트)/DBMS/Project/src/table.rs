use crate::item::{Cursor, DItem};
use crate::item_factory::{ItemFactory};
use crate::hashset::HashSetTable;
use crate::tx_manager::TxManager;

use std::sync::Mutex;
use crate::define::TxAction;

pub struct Table {
    pub table_type: u16,
    pub item_type: u16,
    pub items: HashSetTable,
    pub tx: TxManager,
}

impl Table {
    pub fn new(table_type: u16, item_type: u16) -> Self {
        Table {
            table_type,
            item_type,
            items: HashSetTable::new(table_type, item_type),
            tx: TxManager::new(),
        }
    }

    /// 아이템 삽입
    pub fn insert(&mut self, key: i32, factory: &Mutex<ItemFactory>) -> Option<Cursor> {
        let factory = factory.lock().ok()?;
        let item = factory.create_item(self.item_type, key)?;
        let cursor = Cursor::new(item);

        self.items.insert(cursor.clone());
        self.tx.add(TxAction::Insert(cursor.clone())); // undo 시 삭제
        Some(cursor)
    }

    /// 아이템 삭제
    pub fn remove(&mut self, key: i32) -> bool {
        if let Some(cursors) = self.items.remove(key) {
            for cursor in cursors {
                self.tx.add(TxAction::Remove(cursor)); // undo 시 복원
            }
            true
        } else {
            false
        }
    }

    /// 아이템 조회
    pub fn get(&self, key: i32) -> Option<&Cursor> {
        self.items.find_visible(key)
    }

    /// 전체 초기화
    pub fn clear(&mut self) {
        self.items.clear();
        self.tx.clear();
    }

    /// Undo
    pub fn undo(&mut self) {
        if let Some(mut delta) = self.tx.undo() {
            for action in delta.iter_mut() {
                match action {
                    TxAction::Insert(cursor) => {
                        self.items.insert(cursor.clone()); // 삭제 취소
                        *action = TxAction::Cancelled;
                    }
                    TxAction::Remove(cursor) => {
                        self.items.remove(cursor.key()); // 삽입 취소
                        *action = TxAction::Cancelled;
                    }
                    TxAction::Modify { before, .. } => {
                        self.items.insert(before.clone()); // 수정 취소
                        *action = TxAction::Cancelled;
                    }
                    TxAction::Cancelled => {}
                }
            }
        }
    }

    /// Redo
    pub fn redo(&mut self) {
        if let Some(mut delta) = self.tx.redo() {
            for action in delta.iter_mut() {
                match action {
                    TxAction::Insert(cursor) => {
                        self.items.remove(cursor.key()); // 다시 삭제
                        *action = TxAction::Cancelled;
                    }
                    TxAction::Remove(cursor) => {
                        self.items.insert(cursor.clone()); // 다시 삽입
                        *action = TxAction::Cancelled;
                    }
                    TxAction::Modify { after, .. } => {
                        self.items.insert(after.clone()); // 다시 수정
                        *action = TxAction::Cancelled;
                    }
                    TxAction::Cancelled => {}
                }
            }
        }
    }
}
