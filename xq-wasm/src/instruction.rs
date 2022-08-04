
use crate::module::{LabelIndex, FuncIndex, TypeIndex, LocalIndex, GlobalIndex};

pub trait Expre {
    fn new(&self){
        todo!()
    }
}

pub type Expr = Vec<Instruction>;

pub struct Instruction{
    pub opcode: u8,
    args: Args,
}

pub enum Args{
    None,
    Zero(u32),

    MemArg{
        offset:u32, 
        align:u32
    },
    BlockArgs{
        bt:BlockType, 
        instrs:Vec<Instruction>
    },
    IfArgs{
        bt:BlockType, 
        instrs1:Vec<Instruction>, 
        instrs2:Vec<Instruction>
    }, 
    BrArgs(LabelIndex),
    BrTableArgs{
        labels: Vec<LabelIndex>,
        default: LabelIndex,
    },
    CallArgs(FuncIndex),
    CallIndirectArgs(TypeIndex),

    LocalArgs(LocalIndex),
    GocalArgs(GlobalIndex),

    I32ConstArgs(i32),
    I64ConstArgs(i64),
    F32ConstArgs(f32),
    F64ConstArgs(f64),
    TruncSatArgs(u8),
}

pub struct MemArg {
    pub offset: u32,
    pub align:  u32,
}

type BlockType = i32;

pub struct BlockArgs {
    bt: BlockType,
    instrs: Vec<Instruction>
}

pub struct IfArgs {
    bt: BlockType,
    instr1: Vec<Instruction>,
    instr2: Vec<Instruction>,
}

pub struct BrTableArgs {
    labels: Vec<LabelIndex>,
    default: LabelIndex,
}