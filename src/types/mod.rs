mod peer_id;
pub mod request;
pub mod response;

pub use peer_id::{ConnectionOrigin, PeerId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Version {
    V1 = 1,
}

impl Version {
    pub fn new(version: u16) -> crate::Result<Self> {
        match version {
            1 => Ok(Version::V1),
            _ => Err(anyhow::anyhow!("invalid version {}", version)),
        }
    }

    pub fn to_u16(self) -> u16 {
        self as u16
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::V1
    }
}

pub type HeaderMap = std::collections::HashMap<String, String>;

pub mod header {
    pub const CONTENT_TYPE: &str = "content-type";
    pub const STATUS_MESSAGE: &str = "status-message";
}
