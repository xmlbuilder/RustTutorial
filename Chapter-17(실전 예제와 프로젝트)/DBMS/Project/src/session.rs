use std::collections::HashMap;
use crate::table::Table;

pub struct Session {
    pub tables: HashMap<u16, Table>, // key: table_type
}


impl Session {
    pub fn new() -> Self {
        Session {
            tables: HashMap::new(),
        }
    }

    /// 테이블 등록
    pub fn register_table(&mut self, table_type: u16, item_type: u16) -> bool {
        if self.tables.contains_key(&table_type) {
            return false;
        }
        let table = Table::new(table_type, item_type);
        self.tables.insert(table_type, table);
        true
    }

    /// 테이블 조회
    pub fn get_table(&self, table_type: u16) -> Option<&Table> {
        self.tables.get(&table_type)
    }

    pub fn get_table_mut(&mut self, table_type: u16) -> Option<&mut Table> {
        self.tables.get_mut(&table_type)
    }
}


impl Session {
    /// 전체 undo
    pub fn undo_all(&mut self) {
        for table in self.tables.values_mut() {
            table.undo();
        }
    }

    /// 전체 redo
    pub fn redo_all(&mut self) {
        for table in self.tables.values_mut() {
            table.redo();
        }
    }

    /// 전체 초기화
    pub fn clear_all(&mut self) {
        for table in self.tables.values_mut() {
            table.clear();
        }
    }
}


impl Session {
    pub fn table_types(&self) -> Vec<u16> {
        self.tables.keys().cloned().collect()
    }

    pub fn table_count(&self) -> usize {
        self.tables.len()
    }
}
