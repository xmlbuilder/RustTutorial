use crate::item::Cursor;

// 트랜잭션 상태 플래그
#[derive(Clone, Debug)]
pub enum TxAction {
    Insert(Cursor),  // undo: remove
    Remove(Cursor),  // undo: insert
    Modify {
        before: Cursor,
        after: Cursor,
    },
    Cancelled, // 트랜잭션이 되돌려진 후 상태 초기화용
}

// 시스템 제한값
pub const MAX_TABLE: usize = 256;
pub const MAX_ITEM_TYPE: usize = 1024;

// 기타 상태 플래그
pub const STATUS_VISIBLE: u8 = 0x01;
pub const STATUS_HIDDEN: u8 = 0x02;