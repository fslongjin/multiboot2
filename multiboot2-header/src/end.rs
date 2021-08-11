use crate::{HeaderTagFlag, HeaderTagType, StructAsBytes};
use core::mem::size_of;

/// Terminates a list of optional tags
/// in a multiboot2 header.
#[derive(Copy, Clone, Debug)]
#[repr(C, packed(8))]
pub struct EndHeaderTag {
    // u16 value
    typ: HeaderTagType,
    // u16 value
    flags: HeaderTagFlag,
    size: u32,
}

impl EndHeaderTag {
    pub const fn new() -> Self {
        EndHeaderTag {
            typ: HeaderTagType::End,
            flags: HeaderTagFlag::Required,
            size: size_of::<Self>() as u32,
        }
    }

    pub fn typ(&self) -> HeaderTagType {
        self.typ
    }
    pub fn flags(&self) -> HeaderTagFlag {
        self.flags
    }
    pub fn size(&self) -> u32 {
        self.size
    }
}

impl StructAsBytes for EndHeaderTag {}
