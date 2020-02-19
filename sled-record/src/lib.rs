pub use sled_record_derive::*;

pub trait Record: Sized {
    const TABLE_NAME: &'static str;

    fn write_key_bytes(&self, buf: &mut Vec<u8>) -> Result<(), bincode::Error>;

    fn write_value_bytes(&self, buf: &mut Vec<u8>) -> Result<(), bincode::Error>;

    fn from_kv(key_bytes: &sled::IVec, value_bytes: &sled::IVec) -> Result<Self, bincode::Error>;
}

pub trait DbExt {
    fn persist<R: Record>(&self, record: &R) -> Result<(), bincode::Error>;
}

impl DbExt for sled::Db {
    fn persist<R: Record>(&self, record: &R) -> Result<(), bincode::Error> {
        let mut key_buf = Vec::new();

        record
            .write_key_bytes(&mut key_buf)
            .expect("TODO error handling");

        let mut value_buf = Vec::new();

        record
            .write_value_bytes(&mut value_buf)
            .expect("TODO error handling");

        let tree = self.open_tree(R::TABLE_NAME).expect("TODO: error handling");

        tree.insert(key_buf, value_buf)
            .expect("TODO: error handling");

        Ok(())
    }
}
