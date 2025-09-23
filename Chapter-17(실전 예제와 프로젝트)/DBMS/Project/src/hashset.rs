use std::collections::HashMap;
use std::sync::Arc;

use crate::item::{Cursor, DItem};

pub struct HashSetTable {
    pub table_type: u16,
    pub item_type: u16,
    pub items: HashMap<i32, Vec<Cursor>>, // key → list of items
}


impl HashSetTable {
    pub fn new(table_type: u16, item_type: u16) -> Self {
        HashSetTable {
            table_type,
            item_type,
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, cursor: Cursor) {
        let key = cursor.key();
        self.items.entry(key).or_default().push(cursor);
    }

    pub fn remove(&mut self, key: i32) -> Option<Vec<Cursor>> {
        if let Some(list) = self.items.remove(&key) {
            Some(list)
        } else {
            None
        }
    }


    pub fn find(&self, key: i32) -> Option<&Vec<Cursor>> {
        self.items.get(&key)
    }

    pub fn find_mut(&mut self, key: i32) -> Option<&mut Vec<Cursor>> {
        self.items.get_mut(&key)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn all_items(&self) -> impl Iterator<Item = &Cursor> {
        self.items.values().flat_map(|v| v.iter())
    }
}

impl HashSetTable {
    pub fn find_visible(&self, key: i32) -> Option<&Cursor> {
        self.items.get(&key)?.iter().find(|c| c.visible)
    }

    pub fn find_alive(&self, key: i32) -> Option<&Cursor> {
        self.items.get(&key)?.iter().find(|c| c.is_alive())
    }
}