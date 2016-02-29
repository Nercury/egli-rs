use std::fmt;

/// EGL version.
#[derive(Copy, Clone, Debug)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
