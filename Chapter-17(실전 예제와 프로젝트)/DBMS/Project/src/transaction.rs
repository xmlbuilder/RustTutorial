use crate::session::Session;

pub struct Transaction<'a> {
    session: &'a mut Session,
    committed: bool,
}

impl<'a> Transaction<'a> {
    pub fn new(session: &'a mut Session) -> Self {
        Transaction {
            session,
            committed: false,
        }
    }

    /// 명시적 커밋
    pub fn commit(mut self) {
        self.session.clear_all(); // 트랜잭션 반영 후 초기화
        self.committed = true;
    }

    /// 명시적 롤백
    pub fn rollback(mut self) {
        self.session.undo_all();
        self.committed = true;
    }
}

impl<'a> Drop for Transaction<'a> {
    fn drop(&mut self) {
        if !self.committed {
            self.session.undo_all(); // 자동 롤백
        }
    }
}