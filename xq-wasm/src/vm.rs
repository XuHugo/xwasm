use crate::{module::Module, instruction::Expr, opcodes::OpCode};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;



#[derive(Copy, Clone)]
pub union StackValue {
    pub i32_: i32,
    pub i64_: i64,
    pub u32_: u32,
    pub u64_: u64,
    pub f32_: f32,
    pub f64_: f64,
}

impl From<i32> for StackValue {
    fn from(i32_: i32) -> Self {
        Self {
            i32_,
        }
    }
}

impl From<u32> for StackValue {
    fn from(u32_: u32) -> Self {
        Self {
            u32_,
        }
    }
}

impl From<i64> for StackValue {
    fn from(i64_: i64) -> Self {
        Self {
            i64_,
        }
    }
}

impl From<u64> for StackValue {
    fn from(u64_: u64) -> Self {
        Self {
            u64_,
        }
    }
}

impl From<f32> for StackValue {
    fn from(f32_: f32) -> Self {
        Self {
            f32_,
        }
    }
}

impl From<f64> for StackValue {
    fn from(f64_: f64) -> Self {
        Self {
            f64_,
        }
    }
}

pub struct OperandStack{
    stack:Vec<StackValue>,
    sp: usize,
}

impl OperandStack{
    fn with_limit(limit: usize)->Self{
        OperandStack{
            stack:Vec::with_capacity(limit),
            sp:0,
        }
    }

    fn pop(&mut self)->StackValue{
        self.stack.pop().unwrap()
    }
    fn push(&mut self, sv:StackValue){
        self.stack.push(sv)
    }
    unsafe fn popS32(&mut self)->i32{
        self.pop().i32_
    }

    unsafe fn popS64(&mut self)->i64{
        self.pop().i64_
    }
}


pub struct Interpreter{
    operandstack: OperandStack,
    module: Module,
}

impl Interpreter{
    pub fn new(module: Module)->Self{
        Interpreter{
            operandstack:OperandStack::with_limit(10000),
            module,
        }
    }

    pub fn execFunc(){

    }

    pub fn execCode(instructions: Expr){
        for instruction in instructions.iter(){
            match OpCode::try_from(instruction.opcode).unwrap(){
                OpCode::Unreachable=>{},
                OpCode::Nop=>{},
                OpCode::Block=>{},
                OpCode::Loop=>{},
                OpCode::If=>{},
                OpCode::Else=>{},
                OpCode::End=>{},
                OpCode::Br=>{},
                OpCode::BrIf=>{},
                OpCode::BrTable=>{},
                OpCode::Return=>{},
                OpCode::Call=>{},
                OpCode::CallIndirect=>{},

    // Parametric instructions
    OpCode::Drop=>{},
    OpCode::Select=>{},

    // Variable instructions
    OpCode::LocalGet=>{},
    OpCode::LocalSet=>{},
    OpCode::LocalTee=>{},
    OpCode::GlobalGet=>{},
    OpCode::GlobalSet=>{},

    // Memory instructions
    OpCode::I32Load=>{},
    OpCode::I64Load =>{},
    OpCode::F32Load =>{},
    OpCode::F64Load =>{},
    OpCode::I32Load8S =>{},
    OpCode::I32Load8U =>{},
    OpCode::I32Load16S =>{},
    OpCode::I32Load16U =>{},
    OpCode::I64Load8S =>{},
    OpCode::I64Load8U =>{},
    OpCode::I64Load16S =>{},
    OpCode::I64Load16U=>{},
    OpCode::I64Load32S =>{},
    OpCode::I64Load32U =>{},
    OpCode::I32Store =>{},
    OpCode::I64Store =>{},
    OpCode::F32Store =>{},
    OpCode::F64Store =>{},
    OpCode::I32Store8 =>{},
    OpCode::I32Store16 =>{},
    OpCode::I64Store8 =>{},
    OpCode::I64Store16 =>{},
    OpCode::I64Store32 =>{},
    OpCode::MemorySize =>{},
    OpCode::MemoryGrow =>{},

    // Numeric instructions
    OpCode::I32Const =>{},
    OpCode::I64Const =>{},
    OpCode::F32Const =>{},
    OpCode::F64Const =>{},

    OpCode::I32Eqz =>{},
    OpCode::I32Eq =>{},
    OpCode::I32Ne =>{},
    OpCode::I32LtS =>{},
    OpCode::I32LtU =>{},
    OpCode::I32GtS =>{},
    OpCode::I32GtU =>{},
    OpCode::I32LeS =>{},
    OpCode::I32LeU =>{},
    OpCode::I32GeS =>{},
    OpCode::I32GeU =>{},
    OpCode::I64Eqz =>{},
    OpCode::I64Eq =>{},
    OpCode::I64Ne =>{},
    OpCode::I64LtS =>{},
    OpCode::I64LtU =>{},
    OpCode::I64GtS =>{},
    OpCode::I64GtU =>{},
    OpCode::I64LeS =>{},
    OpCode::I64LeU =>{},
    OpCode::I64GeS =>{},
    OpCode::I64GeU =>{},
    OpCode::F32Eq =>{},
    OpCode::F32Ne =>{},
    OpCode::F32Lt =>{},
    OpCode::F32Gt =>{},
    OpCode::F32Le =>{},
    OpCode::F32Ge =>{},
    OpCode::F64Eq =>{},
    OpCode::F64Ne =>{},
    OpCode::F64Lt =>{},
    OpCode::F64Gt =>{},
    OpCode::F64Le =>{},
    OpCode::F64Ge =>{},

    OpCode::I32Clz =>{},
    OpCode::I32Ctz =>{},
    OpCode::I32Popcnt =>{},
    OpCode::I32Add =>{},
    OpCode::I32Sub =>{},
    OpCode::I32Mul =>{},
    OpCode::I32DivS =>{},
    OpCode::I32DivU =>{},
    OpCode::I32RemS =>{},
    OpCode::I32RemU =>{},
    OpCode::I32And =>{},
    OpCode::I32Or =>{},
    OpCode::I32Xor =>{},
    OpCode::I32Shl =>{},
    OpCode::I32ShrS =>{},
    OpCode::I32ShrU =>{},
    OpCode::I32Rotl =>{},
    OpCode::I32Rotr =>{},
    OpCode::I64Clz =>{},
    OpCode::I64Ctz =>{},
    OpCode::I64Popcnt =>{},
    OpCode::I64Add =>{},
    OpCode::I64Sub =>{},
    OpCode::I64Mul =>{},
    OpCode::I64DivS =>{},
    OpCode::I64DivU =>{},
    OpCode::I64RemS =>{},
    OpCode::I64RemU =>{},
    OpCode::I64And =>{},
    OpCode::I64Or =>{},
    OpCode::I64Xor =>{},
    OpCode::I64Shl =>{},
    OpCode::I64ShrS =>{},
    OpCode::I64ShrU =>{},
    OpCode::I64Rotl =>{},
    OpCode::I64Rotr =>{},
    OpCode::F32Abs =>{},
    OpCode::F32Neg =>{},
    OpCode::F32Ceil =>{},
    OpCode::F32Floor =>{},
    OpCode::F32Trunc =>{},
    OpCode::F32Nearest =>{},
    OpCode::F32Sqrt =>{},
    OpCode::F32Add =>{},
    OpCode::F32Sub =>{},
    OpCode::F32Mul =>{},
    OpCode::F32Div =>{},
    OpCode::F32Min =>{},
    OpCode::F32Max =>{},
    OpCode::F32CopySign =>{},
    OpCode::F64Abs =>{},
    OpCode::F64Neg =>{},
    OpCode::F64Ceil =>{},
    OpCode::F64Floor =>{},
    OpCode::F64Trunc =>{},
    OpCode::F64Nearest =>{},
    OpCode::F64Sqrt =>{},
    OpCode::F64Add =>{},
    OpCode::F64Sub =>{},
    OpCode::F64Mul =>{},
    OpCode::F64Div =>{},
    OpCode::F64Min =>{},
    OpCode::F64Max =>{},
    OpCode::F64CopySign =>{},

    OpCode::I32WrapI64 =>{},
    OpCode::I32TruncF32S =>{},
    OpCode::I32TruncF32U =>{},
    OpCode::I32TruncF64S =>{},
    OpCode::I32TruncF64U =>{},
    OpCode::I64ExtendI32S =>{},
    OpCode::I64ExtendI32U =>{},
    OpCode::I64TruncF32S =>{},
    OpCode::I64TruncF32U =>{},
    OpCode::I64TruncF64S =>{},
    OpCode::I64TruncF64U =>{},
    OpCode::F32ConvertI32S =>{},
    OpCode::F32ConvertI32U =>{},
    OpCode::F32ConvertI64S =>{},
    OpCode::F32ConvertI64U =>{},
    OpCode::F32DemoteF64 =>{},
    OpCode::F64ConvertI32S =>{},
    OpCode::F64ConvertI32U =>{},
    OpCode::F64ConvertI64S =>{},
    OpCode::F64ConvertI64U =>{},
    OpCode::F64PromoteF32 =>{},
    OpCode::I32ReinterpretF32 =>{},
    OpCode::I64ReinterpretF64 =>{},
    OpCode::F32ReinterpretI32 =>{},
    OpCode::F64ReinterpretI64 =>{},
    OpCode::I32Extend8S =>{},
    OpCode::I32Extend16S =>{},
    OpCode::I64Extend8S =>{},
    OpCode::I64Extend16S =>{},
    OpCode::I64Extend32S =>{},
    OpCode::TruncSat =>{},
            }
        }

    }

}