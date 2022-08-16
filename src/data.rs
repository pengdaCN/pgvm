use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use sled::Db as Database;

use crate::db::ExtKv;
use crate::errors::Result;

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

impl Version {
    pub fn short_version(&self) -> String {
        let mut s = self.v1.to_string();
        if let Some(x) = self.v2 {
            s.write_str(&format!(".{x}")).unwrap();
        }
        if let Some(x) = self.v3 {
            s.write_str(&format!(".{x}")).unwrap();
        }
        if let Some(x) = self.unstable_v4.as_ref() {
            s.write_str(&x.to_string()).unwrap();
        }

        s
    }
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

    pub fn store(&self, mut vers: Vec<Version>) -> Result<()> {
        vers.sort();
        vers.reverse();

        let (os, arch, versions) = Self::compute_meta(&vers);

        self.db.drop_tree("version")?;
        let tree = self.db.open_tree("version")?;

        tree.store("meta_os", &os)?;
        tree.store("meta_arch", &arch)?;
        tree.store("meta_versions", &versions)?;

        for x in vers.iter() {
            tree.store(x.to_string(), x)?;
        }

        Ok(())
    }

    fn compute_meta(vers: &Vec<Version>) -> (HashSet<String>, HashSet<String>, Vec<String>) {
        let mut os = HashSet::new();
        let mut arch = HashSet::new();
        let mut short_version = Vec::<String>::new();

        vers.iter().for_each(|x| {
            if !os.contains(&x.os) {
                os.insert(x.os.clone());
            }

            if !arch.contains(&x.arch) {
                arch.insert(x.arch.clone());
            }

            match short_version.last() {
                Some(v) => {
                    if v.ne(&x.short_version()) {
                        short_version.push(x.short_version());
                    }
                }
                None => {
                    short_version.push(x.short_version());
                }
            }
        });

        short_version.reverse();

        (os, arch, short_version)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{UnstableVersion, Version};

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
