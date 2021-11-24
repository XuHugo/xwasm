use crate::{constants::*, traits::*, types::*};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, collections::*, string::String, vec::Vec};
#[cfg(not(feature = "std"))]
use core::{marker, mem::MaybeUninit, slice};
#[cfg(feature = "std")]
use std::{collections::*, marker, mem::MaybeUninit, slice};

macro_rules! repeat_macro {
    ($f:ident, $n:expr) => ($f!($n));
    ($f:ident, $n:expr, $($ns:expr), *) => {
        $f!($n);
        repeat_macro!($f, $($ns), *);
    };
}

impl<X:Serial, Y:Serial> Serial for (X, Y) {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        self.0.serial(out)?;
        self.1.serial(out)
    }
}

impl<X: Deserial, Y:Deserial> Deserial for (X, Y) {
    fn deserial<R: Read>(source: &mut R) -> ParaseResult<Self>{
        let x = X::deserial(source)?;
        let y = Y::deserial(source)?;
        Ok((x, y))
    }
}

impl Serial for u8 {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {out.write_u8(*self)}
}

impl Deserial for u8 {
    fn deserial<R: Read>(source: &mut R) -> ParseResult<Self> {source.read_u8()}
}


