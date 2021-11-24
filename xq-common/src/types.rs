
use crate::constants;
#[cfg(not(feature = "std"))]
use alloc::{string::String, string::ToString, vec::Vec};
#[cfg(not(feature = "std"))]
use core::{convert, fmt, iter, ops, str};
#[cfg(feature = "std")]
use std::{convert, fmt, iter, ops, str};

#[cfg(feature = "fuzz")]
use arbitrary::Arbitrary;
#[cfg(feature = "derive-serde")]
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use serde::ser::SerializeTuple;

pub const ACCOUNT_ADDRESS_SIZE: usize = 44;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "fuzz", derive(Arbitrary))]
pub struct Amount{
    pub micro_gtu: u64,
}

#[cfg(feature = "derive-serde")]
impl SerdeSerialize for Amount {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error>{
        ser.serialize_str(&self.micro_gtu.to_string())
    }
}

#[cfg(feature = "derive-serde")]
impl<'de> SerdeDeserialize<'de> for Amount{
    fn deserialize<D: serde::de::Deserializer<'de>>(des:D) -> Result<Self,D::Error>{
        let s = String::deserialize(des)?;
        let micro_gtu = s.parse::<u64>().map_err(|e| serde::de::Error::custom(format!("{}", e)))?;
        Ok(Amount{
            micro_gtu,
        })
    }
}

