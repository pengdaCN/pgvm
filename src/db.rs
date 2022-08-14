use serde::{Serialize, Deserialize};
use sled::Db;

pub trait ExtKv {
    fn store<K: AsRef<[u8]>, V: Serialize>(&self, key: K, value: &V) -> sled::Result<()>;
    fn load<'a, K: AsRef<[u8]>, V: Deserialize<'a>>(&self, key: K) -> sled::Result<V>;
}

impl ExtKv for Db {
    fn store<K: AsRef<[u8]>, V: Serialize>(&self, key: K, value: &V) -> sled::Result<()> {
        self.insert(key, bincode::serialize(value).unwrap()).map(|_| ())
    }

    fn load<'a, K: AsRef<[u8]>, V: Deserialize<'a>>(&self, key: K) -> sled::Result<Option<V>> {
        let v = self.get(key)?;

        Ok(v.map(|x| {
            bincode::deserialize(x.as_ref()).unwrap()
        }))
    }
}