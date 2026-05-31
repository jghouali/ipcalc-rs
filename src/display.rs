use crate::ip::IP;
use std::fmt;

impl fmt::Display for IP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IP::IPV4 { prefix, mask } => write!(
                f,
                "{}.{}.{}.{}/{}",
                (prefix >> 24) as u8,
                (prefix >> 16) as u8,
                (prefix >> 8) as u8,
                *prefix as u8,
                mask
            ),
            IP::IPV6 { prefix, mask } => write!(
                f,
                "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}/{}",
                (prefix >> 112) as u16,
                (prefix >> 96) as u16,
                (prefix >> 80) as u16,
                (prefix >> 64) as u16,
                (prefix >> 48) as u16,
                (prefix >> 32) as u16,
                (prefix >> 16) as u16,
                *prefix as u16,
                mask
            ),
        }
    }
}
