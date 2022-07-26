use std::convert::TryFrom;

use anyhow::bail;



#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

impl TryFrom<u8> for ValueType{
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error>{
        match value{
            0x7F => Ok(ValueType::I32),
            0x7E => Ok(ValueType::I64),
            0x7D => Ok(ValueType::F32),
            0x7C => Ok(ValueType::F64),
            _ => bail!("Unknown value type {}", value),
        }
    }
}

const FUNC_REF: i32 = 0x70;
pub(crate) const FUNC_TAG: u8 = 0x60;

const MUT_CONST: u8 = 0;
const MUT_VAR: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub tag: u8,
    pub parameters: Vec<ValueType>,
    pub results:     Vec<ValueType>,
}

#[derive(Debug, Copy, Clone)]
pub struct Limits {
    pub tag: u8,
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug, Copy, Clone)]
pub struct TableType {
    pub elem_type: u8,
    pub limits: Limits,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryType {
    pub limits: Limits,
}

#[derive(Debug)]
pub struct GlobalType {
    pub init:    ValueType,
    pub mutable: bool,
}

