use std::sync::Arc;
use crate::session::Session;
use crate::tx_stream::TxStream;

pub trait DItem: std::fmt::Debug + Send + Sync {
    fn key(&self) -> i32;
    fn item_type(&self) -> u16;
    fn table_type(&self) -> u16;
    fn serialize(&self, stream: &mut dyn TxStream, session: &Session);
}



#[derive(Clone, Debug)]
pub struct Cursor {
    pub data: Arc<dyn DItem>,
    pub visible: bool,
    pub temp_data: u16,
    pub param_data: u8,
    pub param: usize,
}

impl Cursor {
    pub fn new(data: Arc<dyn DItem>) -> Self {
        Cursor {
            data,
            visible: true,
            temp_data: 0,
            param_data: 0,
            param: 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.visible
    }


    pub fn key(&self) -> i32 {
        self.data.key()
    }

    pub fn item_type(&self) -> u16 {
        self.data.item_type()
    }

    pub fn table_type(&self) -> u16 {
        self.data.table_type()
    }

    pub fn set_visible(&mut self, v: bool) {
        self.visible = v;
    }

    pub fn set_temp_data(&mut self, d: u16) {
        self.temp_data = d;
    }

    pub fn set_param_data(&mut self, d: u8) {
        self.param_data = d;
    }

    pub fn set_param(&mut self, p: usize) {
        self.param = p;
    }
}