use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
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

#[derive(Debug)]
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
