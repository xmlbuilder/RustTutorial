use crate::item::Cursor;
use crate::guid::Guid;
use std::fs::File;
use std::hash::Hasher;
use std::io::{Read, Write, BufReader, BufWriter};
use std::sync::{Arc, Mutex};
use crate::define::TxAction;
use crate::item_factory::{item_factory, ItemFactory};

pub trait TxStream {
    fn write_guid(&mut self, guid: &Guid);
    fn read_guid(&mut self) -> Option<Guid>;

    fn write_u32(&mut self, value: u32);
    fn read_u32(&mut self) -> Option<u32>;

    fn flush(&mut self);
    fn write_action(&mut self, action: &TxAction);
    fn read_action(&mut self, item_type: u16, factory: &Mutex<ItemFactory>) -> Option<TxAction>;
}



pub struct FileTxStream {
    writer: BufWriter<File>,
    reader: Option<BufReader<File>>,
}

impl FileTxStream {
    pub fn new_write(path: &str) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(FileTxStream {
            writer: BufWriter::new(file),
            reader: None,
        })
    }

    pub fn new_read(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        Ok(FileTxStream {
            writer: BufWriter::new(File::create("/dev/null")?), // dummy writer
            reader: Some(BufReader::new(file)),
        })
    }

    pub fn write_u16(&mut self, value: u16) {
        self.writer.write_all(&value.to_le_bytes()).unwrap();
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let mut buf = [0u8; 2];
        self.reader.as_mut()?.read_exact(&mut buf).ok()?;
        Some(u16::from_le_bytes(buf))
    }
}


impl TxStream for FileTxStream {
    fn write_guid(&mut self, guid: &Guid) {
        self.write_u32(guid.data1);
        self.write_u16(guid.data2);
        self.write_u16(guid.data3);
        self.writer.write_all(&guid.data4).unwrap();
    }

    fn read_guid(&mut self) -> Option<Guid> {
        let data1 = self.read_u32()?;
        let data2 = self.read_u16()?;
        let data3 = self.read_u16()?;
        let mut data4 = [0u8; 8];
        self.reader.as_mut()?.read_exact(&mut data4).ok()?;
        Some(Guid { data1, data2, data3, data4 })
    }

    fn write_u32(&mut self, value: u32) {
        self.writer.write_all(&value.to_le_bytes()).unwrap();
    }

    fn read_u32(&mut self) -> Option<u32> {
        let mut buf = [0u8; 4];
        self.reader.as_mut()?.read_exact(&mut buf).ok()?;
        Some(u32::from_le_bytes(buf))
    }




    fn write_action(&mut self, action: &TxAction) {
        match action {
            TxAction::Insert(cursor) => {
                self.write_u32(cursor.key() as u32);
                self.writer.write_all(&[0x01]).unwrap(); // 상태: Insert
                self.writer.write_all(&[cursor.param_data]).unwrap();
                self.write_u32(cursor.param as u32);
            }
            TxAction::Remove(cursor) => {
                self.write_u32(cursor.key() as u32);
                self.writer.write_all(&[0x02]).unwrap(); // 상태: Remove
                self.writer.write_all(&[cursor.param_data]).unwrap();
                self.write_u32(cursor.param as u32);
            }
            TxAction::Modify { after, .. } => {
                self.write_u32(after.key() as u32);
                self.writer.write_all(&[0x03]).unwrap(); // 상태: Modify
                self.writer.write_all(&[after.param_data]).unwrap();
                self.write_u32(after.param as u32);
            }
            TxAction::Cancelled => {
                // 생략하거나 특별한 마커로 기록
                self.writer.write_all(&[0xFF]).unwrap(); // 상태: Cancelled
            }
        }
    }


    fn read_action(&mut self, item_type: u16, factory: &Mutex<ItemFactory>) -> Option<TxAction> {
        let key = self.read_u32()? as i32;

        let mut status = [0u8; 1];
        let mut param_data = [0u8; 1];

        self.reader.as_mut()?.read_exact(&mut status).ok()?;
        self.reader.as_mut()?.read_exact(&mut param_data).ok()?;

        let param = self.read_u32()? as usize;

        let factory = factory.lock().ok()?;
        let item = factory.create_item(item_type, key)?;
        let mut cursor = Cursor::new(item);

        cursor.param_data = param_data[0];
        cursor.param = param;

        match status[0] {
            0x01 => Some(TxAction::Insert(cursor)), // 삭제된 항목 → 복원
            0x02 => Some(TxAction::Remove(cursor)), // 삽입된 항목 → 삭제
            0x03 => {
                // 수정된 항목 → 수정 복원
                // 이 경우 before/after를 따로 읽어야 함 (추가 구조 필요)
                None // 또는 수정 로직 구현
            }
            0xFF => Some(TxAction::Cancelled), // 취소된 항목
            _ => None,
        }
    }





    fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
}
