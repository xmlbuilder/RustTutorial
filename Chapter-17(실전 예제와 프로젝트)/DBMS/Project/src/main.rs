use std::ptr::NonNull;
use std::sync::Arc;
use crate::dbutil::{format_string_args, get_db_temp_path, replace_all};
use crate::guid::Guid;
use crate::hashset::HashSetTable;
use crate::item::{Cursor, DItem};
use crate::item_factory::item_factory_mut;
use crate::mem_pool::MemPool;
use crate::session::Session;
use crate::transaction::Transaction;

mod guid;
mod dbutil;
mod mem_pool;
mod item;
mod item_factory;
mod hashset;
mod tx_delta_list;
mod tx_stream;
mod tx_manager;
mod table;
mod session;
mod transaction;
mod define;
mod undo_redo;

#[derive(Debug)]
struct MyNode {
    value: i32,
    next: Option<NonNull<MyNode>>,
}

#[derive(Debug)]
struct MyItem {
    key: i32,
    item_type: u16,
    table_type: u16,
}

impl DItem for MyItem {
    fn key(&self) -> i32 {
        self.key
    }

    fn item_type(&self) -> u16 {
        self.item_type
    }

    fn table_type(&self) -> u16 {
        self.table_type
    }

    fn serialize(&self, _stream: &mut dyn crate::tx_stream::TxStream, _session: &crate::session::Session) {
        // 직렬화 로직은 필요 시 구현
    }
}

fn main() {

    let factory = item_factory_mut();
    {
        let guid = Guid::new();
        println!("GUID: {}", guid.to_string());

        let parsed = Guid::from_string(&guid.to_string()).unwrap();
        assert_eq!(guid, parsed);
    }


    {
        let original = "Hello, World!";
        let replaced = replace_all(original, "World", "Rust");
        println!("{}", replaced); // Hello, Rust!

        let temp_path = get_db_temp_path();
        println!("임시 경로: {}", temp_path.display());

    }


    {
        let mut pool = MemPool::<MyNode>::new(std::mem::size_of::<MyNode>(), 1024 * 1024);

        let node_ptr = pool.alloc();
        unsafe {

            node_ptr.as_ptr().write(MyNode { value: 42, next: None });
            println!("Node: {:?}", *node_ptr.as_ptr());
        }
        pool.dealloc(node_ptr);
    }


    {

        let mut table = HashSetTable::new(10, 100);

        let item = Arc::new(MyItem {
            key: 42,
            item_type: 100,
            table_type: 10,
        });

        let cursor = Cursor::new(item);
        table.insert(cursor);

        if let Some(found) = table.find(42) {
            println!("Found {} items with key 42", found.len());
        }

    }







    {
        println!("================== 11번 테스트 =================================");
        let mut session = Session::new();
        session.register_table(10, 100);

        let table = session.get_table_mut(10).unwrap();
        table.insert(42, factory);
        table.remove(42);

        session.undo_all(); // 삭제 취소
        session.redo_all(); // 다시 삭제

    }

    {

        println!("================== 11번 테스트 =================================");
        let mut session = Session::new();
        session.register_table(10, 100);

        {

            let table = session.get_table_mut(10).unwrap();
            table.insert(42, factory);
            let mut tx = Transaction::new(&mut session);
            tx.commit(); // 명시적 커밋
        }

        {

            let table = session.get_table_mut(10).unwrap();
            table.remove(42);
            let mut tx = Transaction::new(&mut session);
            tx.commit();
            // rollback 생략 → Drop에서 자동 undo
        }

    }

}
