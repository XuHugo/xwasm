
use crate::module::LabelIndex;

pub trait Expr {
    fn new(&self){
        todo!()
    }
}

pub struct Instruction{
    opcode: u8,
    args: Args,
}

pub enum Args{
    None,
    MemArg,
    BlockArgs,
    IfArgs,
    BrTableArgs,
    u32,
    u64,
    i32,
    i64,
    f32,
    f64,
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