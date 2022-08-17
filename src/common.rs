use openssl::sha::Sha256;
use std::io::Write;
use std::ops::{Deref, DerefMut};

pub struct WriteSha256(Sha256);

impl WriteSha256 {
    pub fn new(hasher: Sha256) -> Self {
        Self(hasher)
    }
    pub fn into_sha256(self) -> Sha256 {
        self.0
    }
}

impl Deref for WriteSha256 {
    type Target = Sha256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WriteSha256 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Write for WriteSha256 {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.update(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
