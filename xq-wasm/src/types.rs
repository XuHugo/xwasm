

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

const FUNC_REF: i32 = 0x70;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub tag: u8,
    pub parameters: Vec<ValueType>,
    pub result:     Vec<ValueType>,
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

#[derive(Debug, Copy, Clone)]
pub struct Limits {
    pub tag: u8,
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug)]
pub struct Global {
    pub init:    ValueType,
    pub mutable: bool,
}