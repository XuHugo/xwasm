

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

const FUNC_REF: i32 = 0x70;
const FUNC_TAG: i32 = 0x60;

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






#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpCode {
    // Control instructions
    End,
    Nop,
    Unreachable,
    Block(BlockType),
    Loop(BlockType),
    If {
        ty: BlockType,
    },
    Else,
    Br(LabelIndex),
    BrIf(LabelIndex),
    BrTable {
        labels:  Vec<LabelIndex>,
        default: LabelIndex,
    },
    Return,
    Call(FuncIndex),
    CallIndirect(TypeIndex),

    // Parametric instructions
    Drop,
    Select,

    // Variable instructions
    LocalGet(LocalIndex),
    LocalSet(LocalIndex),
    LocalTee(LocalIndex),
    GlobalGet(GlobalIndex),
    GlobalSet(GlobalIndex),

    // Memory instructions
    I32Load(MemArg),
    I64Load(MemArg),
    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),
    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,

    // Numeric instructions
    I32Const(i32),
    I64Const(i64),

    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    I32WrapI64,
    I64ExtendI32S,
    I64ExtendI32U,
}