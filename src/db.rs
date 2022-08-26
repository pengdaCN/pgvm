use serde::de::DeserializeOwned;
use serde::Serialize;
use sled::{Db, Tree};

pub trait ExtKv {
    fn store<K: AsRef<[u8]>, V: Serialize>(&self, key: K, value: &V) -> sled::Result<()>;
    fn load<K: AsRef<[u8]>, V: DeserializeOwned>(&self, key: K) -> sled::Result<Option<V>>;
}

macro_rules! impl_ext_kv {
    ($ty:ident) => {
        impl ExtKv for $ty {
            fn store<K: AsRef<[u8]>, V: Serialize>(&self, key: K, value: &V) -> sled::Result<()> {
                self.insert(key, bincode::serialize(value).unwrap())
                    .map(|_| ())
            }

            fn load<K: AsRef<[u8]>, V: DeserializeOwned>(&self, key: K) -> sled::Result<Option<V>> {
                let v = self.get(key)?;

                Ok(v.map(|x| bincode::deserialize(x.as_ref()).unwrap()))
            }
        }
    };
}

impl_ext_kv!(Db);
impl_ext_kv!(Tree);
