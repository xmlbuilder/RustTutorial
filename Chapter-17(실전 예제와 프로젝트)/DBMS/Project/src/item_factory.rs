use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use once_cell::sync::Lazy; // ✅ 반드시 sync 버전
use crate::item::{DItem, Cursor};

pub type CreateCallback = Arc<dyn Fn(i32) -> Arc<dyn DItem> + Send + Sync>;

pub type DestroyCallback = Arc<dyn Fn(Arc<dyn DItem>) + Send + Sync>;

#[derive(Clone)]
pub struct TypeInfo {
    pub create: CreateCallback,
    pub destroy: DestroyCallback,
    pub item_type: u16,
    pub table_type: u16,
}

#[derive(Clone)]
pub struct ItemFactory {
    registry: HashMap<u16, TypeInfo>, // key: item_type
}



impl ItemFactory {
    pub fn new() -> Self {
        ItemFactory {
            registry: HashMap::new(),
        }
    }

    pub fn register_type(
        &mut self,
        item_type: u16,
        table_type: u16,
        create: CreateCallback,
        destroy: DestroyCallback,
    ) -> bool {
        if item_type == 0 || table_type == 0 || self.registry.contains_key(&item_type) {
            return false;
        }

        self.registry.insert(
            item_type,
            TypeInfo {
                create,
                destroy,
                item_type,
                table_type,
            },
        );
        true
    }

    pub fn create_item(&self, item_type: u16, key: i32) -> Option<Arc<dyn DItem>> {
        self.registry.get(&item_type).map(|info| (info.create)(key))
    }

    pub fn destroy_item(&self, item: Arc<dyn DItem>) {
        let item_type = item.item_type();
        if let Some(info) = self.registry.get(&item_type) {
            (info.destroy)(item);
        }
    }

    pub fn get_type_info(&self, item_type: u16) -> Option<&TypeInfo> {
        self.registry.get(&item_type)
    }
}


static FACTORY: Lazy<Mutex<ItemFactory>> = Lazy::new(|| Mutex::new(ItemFactory::new()));

pub fn item_factory() -> &'static Mutex<ItemFactory> {
    &FACTORY
}



pub fn item_factory_mut() -> &'static Mutex<ItemFactory> {
    &FACTORY
}

