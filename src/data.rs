use crate::errors::{Error, Reason, Result};
use serde::{Deserialize, Serialize};
use sled::Db as Database;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

#[derive(Debug, Ord, Eq, Deserialize, Serialize)]
pub struct Version {
    pub name: String,
    pub arch: String,
    pub os: String,
    pub v1: i32,
    pub v2: Option<i32>,
    pub v3: Option<i32>,
    pub unstable_v4: Option<UnstableVersion>,
    pub size: i32,
    pub sha256: String,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "go{}", self.v1)?;

        for x in vec![&self.v2, &self.v3] {
            if let Some(x) = x {
                write!(f, ".{}", *x)?;
            }
        }

        if let Some(ref x) = self.unstable_v4 {
            write!(f, "{x}")?;
        }

        write!(f, ".{}-{}", self.os, self.arch)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.v1 == other.v1
            && self.v2 == other.v2
            && self.v3 == other.v3
            && self.unstable_v4 == other.unstable_v4
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        macro_rules! cmp {
            ($v1:expr, $v2:expr) => {
                let x = $v1.partial_cmp($v2);
                if let Some(x) = x {
                    if !matches!(x, Ordering::Equal) {
                        return Some(x);
                    }
                }
            };
        }

        cmp!(&self.v1, &other.v1);
        cmp!(&self.v2, &other.v2);
        cmp!(&self.v3, &other.v3);

        if self.unstable_v4.is_none() && other.unstable_v4.is_some() {
            return Some(Ordering::Greater);
        }
        if self.unstable_v4.is_some() && other.unstable_v4.is_none() {
            return Some(Ordering::Less);
        }

        cmp!(&self.unstable_v4.as_ref(), &other.unstable_v4.as_ref());

        Some(Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, Clone, Ord, Eq, Deserialize, Serialize)]
pub enum UnstableVersion {
    RC(i32),
    Beta(i32),
}

impl Display for UnstableVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self {
                UnstableVersion::RC(_) => {
                    "rc"
                }
                UnstableVersion::Beta(_) => {
                    "beta"
                }
            },
            match self {
                UnstableVersion::RC(x) => {
                    *x
                }
                UnstableVersion::Beta(x) => {
                    *x
                }
            }
        )
    }
}

impl PartialOrd for UnstableVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            UnstableVersion::RC(x) => match other {
                UnstableVersion::RC(o) => x.cmp(o).into(),
                UnstableVersion::Beta(_) => Some(Ordering::Greater),
            },
            UnstableVersion::Beta(x) => match other {
                UnstableVersion::RC(_) => Some(Ordering::Less),
                UnstableVersion::Beta(o) => x.cmp(o).into(),
            },
        }
    }
}

pub struct Db {
    db: Database,
}

impl Db {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db = sled::open(path)?;

        Ok(Self { db })
    }

    pub fn store(&self, vers: Vec<Version>) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{UnstableVersion, Version};
    use std::mem::transmute;

    #[test]
    fn ord() {
        let r1 = UnstableVersion::RC(1);
        let r2 = UnstableVersion::RC(2);
        let b1 = UnstableVersion::Beta(1);
        let b2 = UnstableVersion::Beta(2);

        let v1 = Version {
            name: "".to_string(),
            arch: "".to_string(),
            os: "".to_string(),
            v1: 1,
            v2: 8.into(),
            v3: 6.into(),
            unstable_v4: None,
            size: 0,
            sha256: "".to_string(),
        };

        let v2 = Version {
            name: "".to_string(),
            arch: "".to_string(),
            os: "".to_string(),
            v1: 1,
            v2: 8.into(),
            v3: 6.into(),
            unstable_v4: b2.clone().into(),
            size: 0,
            sha256: "".to_string(),
        };

        assert_eq!(v1 > v2, true);
        assert_eq!(v2 < v1, true);
        assert_eq!(v2 > v1, false);
    }
}
