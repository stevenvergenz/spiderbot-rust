use core::marker::PhantomData;
use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    IntoBytes,
    FromZeros,
    TryFromBytes,
};

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct Flags<T>(u8, PhantomData<T>)
    where T: Immutable + TryFromBytes + IntoBytes;
impl<T> Flags<T> where T: Immutable + TryFromBytes + IntoBytes {
    pub fn has(&self, flag: T) -> bool {
        let bytes = flag.as_bytes();
        self.0 & bytes[0] == bytes[0]
    }

    pub fn has_all(&self, flags: Self) -> bool {
        self.0 & flags.0 == flags.0
    }
}