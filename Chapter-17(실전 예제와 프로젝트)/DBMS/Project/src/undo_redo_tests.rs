#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::item::{DItem, Cursor};
    use crate::session::Session;
    use crate::item_factory::{item_factory, item_factory_mut};

    #[derive(Debug)]
    struct MyItem {
        key: i32,
        item_type: u16,
        table_type: u16,
    }

    impl DItem for MyItem {
        fn key(&self) -> i32 { self.key }
        fn item_type(&self) -> u16 { self.item_type }
        fn table_type(&self) -> u16 { self.table_type }
        fn serialize(&self, _stream: &mut dyn crate::tx_stream::TxStream, _session: &crate::session::Session) {}
    }

    #[test]
    fn test_insert_remove_undo_redo() {
        // 아이템 타입 등록
        let factory = item_factory_mut();
        factory.lock().unwrap().register_type(
            100, // item_type
            10,  // table_type
            Arc::new(|key| Arc::new(MyItem {
                key,
                item_type: 100,
                table_type: 10,
            })),
            Arc::new(|_item| {}),
        );

        println!("test-1");

        // 세션 생성 및 테이블 등록
        let mut session = Session::new();
        session.register_table(10, 100);
        let table = session.get_table_mut(10).unwrap();

        println!("test0");

        // 삽입
        let cursor = table.insert(42, factory).unwrap();
        assert_eq!(cursor.key(), 42);
        assert!(table.get(42).is_some());

        println!("test1");

        {
            table.remove(42);
            assert!(table.get(42).is_none());
            println!("test2");

            table.tx.commit(); // 또는 session.commit_all();
        }
        // 삭제


        {
            // Undo
            session.undo_all();
            assert!(session.get_table(10).unwrap().get(42).is_some());
            println!("test3");

        }

        {
            // Redo
            session.redo_all();
            assert!(session.get_table(10).unwrap().get(42).is_none());

            println!("test4");
        }


    }
}
