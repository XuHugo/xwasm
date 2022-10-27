use crate::{module::Module, instruction::{Expr, Args}, opcodes::OpCode};
use num_enum::TryFromPrimitive;
use std::{convert::{TryFrom, TryInto}, ops::{Neg, BitAnd, BitOr, BitXor, Shl, Shr}};



#[derive(Copy, Clone)]
pub union StackValue2 {
    pub i32_: i32,
    pub i64_: i64,
    pub u32_: u32,
    pub u64_: u64,
    pub f32_: f32,
    pub f64_: f64,
}


#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum StackValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl From<i8> for StackValue {
	fn from(val: i8) -> Self {
		StackValue::I32(val as i32)
	}
}

impl From<i16> for StackValue {
	fn from(val: i16) -> Self {
		StackValue::I32(val as i32)
	}
}

impl From<i32> for StackValue {
    fn from(n: i32) -> Self {
        StackValue::I32(n)
    }
}

impl From<i64> for StackValue {
    fn from(n: i64) -> Self {
        StackValue::I64(n)
    }
}

impl From<u8> for StackValue {
	fn from(val: u8) -> Self {
		StackValue::I32(val as i32)
	}
}

impl From<u16> for StackValue {
	fn from(val: u16) -> Self {
		StackValue::I32(val as i32)
	}
}

impl From<u32> for StackValue {
    fn from(n: u32) -> Self {
        StackValue::I32(n as i32)
    }
}

impl From<u64> for StackValue {
    fn from(n: u64) -> Self {
        StackValue::I64(n as i64)
    }
}

impl From<f32> for StackValue {
    fn from(f: f32) -> Self {
        StackValue::F32(f)
    }
}

impl From<f64> for StackValue {
    fn from(f: f64) -> Self {
        StackValue::F64(f)
    }
}

impl From<bool> for StackValue {
    fn from(b: bool) -> Self {
        if b {
            StackValue::I32(1)
        }else{
            StackValue::I32(0)
        }
        
    }
}


impl From<StackValue> for i32 {
    fn from(n: StackValue) -> Self {
        match n {
            StackValue::I32(val)=> val,
            _=> panic!("the conversion from stackvalue to i32 error!"),
        }
    }
}

impl From<StackValue> for u32 {
    fn from(n: StackValue) -> Self {
        match n {
            StackValue::I32(val)=> val as u32,
            _=> panic!("the conversion from stackvalue to u32 error!"),
        }
    }
}

impl From<StackValue> for i64 {
    fn from(n: StackValue) -> Self {
        match n {
            StackValue::I64(val)=> val,
            _=> panic!("the conversion from stackvalue to i64 error!"),
        }
    }
}

impl From<StackValue> for u64 {
    fn from(n: StackValue) -> Self {
        match n {
            StackValue::I64(val)=> val as u64,
            _=> panic!("the conversion from stackvalue to u64 error!"),
        }
    }
}

impl From<StackValue> for f32 {
    fn from(n: StackValue) -> Self {
        match n {
            StackValue::F32(val)=> val,
            _=> panic!("the conversion from stackvalue to f32 error!"),
        }
    }
}

impl From<StackValue> for f64 {
    fn from(n: StackValue) -> Self {
        match n {
            StackValue::F64(val)=> val,
            _=> panic!("the conversion from stackvalue to f64 error!"),
        }
    }
}

impl From<StackValue> for bool {
    fn from(b: StackValue) -> Self {
        match b {
            StackValue::I32(val)=> val !=0,
            _=> panic!("the conversion from stackvalue to bool error!"),
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
    fn pop_as<T>(&mut self)->T
    where
        T:From<StackValue>
    {
        let value = self.pop();
        value.into()
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

    pub fn execFunc(&mut self, idx:usize){
        match self.module.code{
            Some(ref mut  c)=>{
                let code = c[idx].clone();
                self.execCode(code.expr);

            },
            None=>{},
        }

        //self.module.invoke_export(func_name)
    }

    pub fn execCode(&mut self, instructions: Expr){
        for instruction in instructions.iter(){
            match OpCode::try_from(instruction.opcode).unwrap(){
                OpCode::Unreachable=>{
                    panic!("unreachable");
                },
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
                OpCode::Drop=>{
                    self.operandstack.pop();
                },
                OpCode::Select=>{
                    let right = self.operandstack.pop();
                    let mid = self.operandstack.pop();
                    let left = self.operandstack.pop();

                    let val = if right.into() {left} else {mid};
                    self.operandstack.push(val);
                },

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
                OpCode::I32Const =>{
                    if let Args::I32ConstArgs(arg) =instruction.args{
                        self.operandstack.push(arg.into())
                    }
                },
                OpCode::I64Const =>{
                    if let Args::I64ConstArgs(arg) =instruction.args{
                        self.operandstack.push(arg.into())
                    }
                },
                OpCode::F32Const =>{
                    if let Args::F32ConstArgs(arg) =instruction.args{
                        self.operandstack.push(arg.into())
                    }
                },
                OpCode::F64Const =>{
                    if let Args::F64ConstArgs(arg) =instruction.args{
                        self.operandstack.push(arg.into())
                    }
                },

                OpCode::I32Eqz =>{
                    let v = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((v==0).into())
                },
                OpCode::I32Eq =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((right==left).into())
                },
                OpCode::I32Ne =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((right!=left).into())
                },
                OpCode::I32LtS =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<right).into())
                },
                OpCode::I32LtU =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<right).into())
                },
                OpCode::I32GtS =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>right).into())
                },
                OpCode::I32GtU =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>right).into())
                },
                OpCode::I32LeS =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<=right).into())
                },
                OpCode::I32LeU =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<=right).into())
                },
                OpCode::I32GeS =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>=right).into())
                },
                OpCode::I32GeU =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>=right).into())
                },
                OpCode::I64Eqz =>{
                    let v = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((v==0).into())
                },
                OpCode::I64Eq =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((right==left).into())
                },
                OpCode::I64Ne =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((right!=left).into())
                },
                OpCode::I64LtS =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<right).into())
                },
                OpCode::I64LtU =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<right).into())
                },
                OpCode::I64GtS =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>right).into())
                },
                OpCode::I64GtU =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>right).into())
                },
                OpCode::I64LeS =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<=right).into())
                },
                OpCode::I64LeU =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<=right).into())
                },
                OpCode::I64GeS =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>=right).into())
                },
                OpCode::I64GeU =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>=right).into())
                },
                OpCode::F32Eq =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((right==left).into())
                },
                OpCode::F32Ne =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((right!=left).into())
                },
                OpCode::F32Lt =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<right).into())
                },
                OpCode::F32Gt =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>right).into())
                },
                OpCode::F32Le =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<=right).into())
                },
                OpCode::F32Ge =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>=right).into())
                },
                OpCode::F64Eq =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((right==left).into())
                },
                OpCode::F64Ne =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((right!=left).into())
                },
                OpCode::F64Lt =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<right).into())
                },
                OpCode::F64Gt =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>right).into())
                },
                OpCode::F64Le =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left<=right).into())
                },
                OpCode::F64Ge =>{
                    let right = self.operandstack.pop();
                    let left = self.operandstack.pop();
                    self.operandstack.push((left>=right).into())
                },

                OpCode::I32Clz =>{
                    let v = self.operandstack.pop_as::<i32>();
                    self.operandstack.push(v.leading_zeros().into())
                },
                OpCode::I32Ctz =>{
                    let v = self.operandstack.pop_as::<i32>();
                    self.operandstack.push(v.trailing_zeros().into())
                },
                OpCode::I32Popcnt =>{
                    let v = self.operandstack.pop_as::<i32>();
                    self.operandstack.push(v.count_ones().into())
                },
                OpCode::I32Add =>{
                    let right = self.operandstack.pop_as::<i32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((right+left).into())
                },
                OpCode::I32Sub =>{
                    let right = self.operandstack.pop_as::<i32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left-right).into())
                },
                OpCode::I32Mul =>{
                    let right = self.operandstack.pop_as::<i32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left*right).into())
                },
                OpCode::I32DivS =>{
                    let right = self.operandstack.pop_as::<i32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left/right).into())
                },
                OpCode::I32DivU =>{
                    let right = self.operandstack.pop_as::<u32>();
                    let left = self.operandstack.pop_as::<u32>();
                    self.operandstack.push((left/right).into())
                },
                OpCode::I32RemS =>{
                    let right = self.operandstack.pop_as::<i32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left%right).into())
                },
                OpCode::I32RemU =>{
                    let right = self.operandstack.pop_as::<u32>();
                    let left = self.operandstack.pop_as::<u32>();
                    self.operandstack.push((left%right).into())
                },
                OpCode::I32And =>{
                    let right = self.operandstack.pop_as::<i32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left.bitand(right)).into())
                },
                OpCode::I32Or =>{
                    let right = self.operandstack.pop_as::<i32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left.bitor(right)).into())
                },
                OpCode::I32Xor =>{
                    let right = self.operandstack.pop_as::<i32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left.bitxor(right)).into())
                },
                OpCode::I32Shl =>{
                    let right = self.operandstack.pop_as::<u32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left.wrapping_shl(right)).into())
                },
                OpCode::I32ShrS =>{
                    let right = self.operandstack.pop_as::<u32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left.wrapping_shr(right)).into())
                },
                OpCode::I32ShrU =>{
                    let right = self.operandstack.pop_as::<u32>();
                    let left = self.operandstack.pop_as::<u32>();
                    self.operandstack.push((left.wrapping_shr(right)).into())
                },
                OpCode::I32Rotl =>{
                    let right = self.operandstack.pop_as::<u32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left.rotate_left(right)).into())
                },
                OpCode::I32Rotr =>{
                    let right = self.operandstack.pop_as::<u32>();
                    let left = self.operandstack.pop_as::<i32>();
                    self.operandstack.push((left.rotate_right(right)).into())
                },
                OpCode::I64Clz =>{
                    let v = self.operandstack.pop_as::<i64>();
                    self.operandstack.push(v.leading_zeros().into())
                },
                OpCode::I64Ctz =>{
                    let v = self.operandstack.pop_as::<i64>();
                    self.operandstack.push(v.trailing_zeros().into())
                },
                OpCode::I64Popcnt =>{
                    let v = self.operandstack.pop_as::<i64>();
                    self.operandstack.push(v.count_ones().into())
                },
                OpCode::I64Add =>{
                    let right = self.operandstack.pop_as::<i64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((right+left).into())
                },
                OpCode::I64Sub =>{
                    let right = self.operandstack.pop_as::<i64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left-right).into())
                },
                OpCode::I64Mul =>{
                    let right = self.operandstack.pop_as::<i64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left*right).into())
                },
                OpCode::I64DivS =>{
                    let right = self.operandstack.pop_as::<i64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left/right).into())
                },
                OpCode::I64DivU =>{
                    let right = self.operandstack.pop_as::<u64>();
                    let left = self.operandstack.pop_as::<u64>();
                    self.operandstack.push((left/right).into())
                },
                OpCode::I64RemS =>{
                    let right = self.operandstack.pop_as::<i64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left%right).into())
                },
                OpCode::I64RemU =>{
                    let right = self.operandstack.pop_as::<u64>();
                    let left = self.operandstack.pop_as::<u64>();
                    self.operandstack.push((left%right).into())
                },
                OpCode::I64And =>{
                    let right = self.operandstack.pop_as::<i64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((right+left).into())
                },
                OpCode::I64Or =>{
                    let right = self.operandstack.pop_as::<i64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left.bitor(right)).into())
                },
                OpCode::I64Xor =>{
                    let right = self.operandstack.pop_as::<i64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left.bitxor(right)).into())
                },
                OpCode::I64Shl =>{
                    let right = self.operandstack.pop_as::<u64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left.wrapping_shl(right as u32)).into())
                },
                OpCode::I64ShrS =>{
                    let right = self.operandstack.pop_as::<u64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left.wrapping_shr(right as u32)).into())
                },
                OpCode::I64ShrU =>{
                    let right = self.operandstack.pop_as::<u64>();
                    let left = self.operandstack.pop_as::<u64>();
                    self.operandstack.push((left.wrapping_shr(right as u32)).into())
                },
                OpCode::I64Rotl =>{
                    let right = self.operandstack.pop_as::<u64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left.rotate_left(right as u32)).into())
                },
                OpCode::I64Rotr =>{
                    let right = self.operandstack.pop_as::<u64>();
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left.rotate_right(right as u32)).into())
                },
                OpCode::F32Abs =>{
                    let v = self.operandstack.pop_as::<f32>();
                    self.operandstack.push(v.abs().into())
                },
                OpCode::F32Neg =>{
                    let v = self.operandstack.pop_as::<f32>();
                    self.operandstack.push(v.neg().into())
                },
                OpCode::F32Ceil =>{
                    let v = self.operandstack.pop_as::<f32>();
                    self.operandstack.push(v.ceil().into())
                },
                OpCode::F32Floor =>{
                    let v = self.operandstack.pop_as::<f32>();
                    self.operandstack.push(v.floor().into())
                },
                OpCode::F32Trunc =>{
                    let v = self.operandstack.pop_as::<f32>();
                    self.operandstack.push(v.trunc().into())
                },
                OpCode::F32Nearest =>{
                    let f = self.operandstack.pop_as::<f32>();
                    let fround = f.round();
                    if (f - fround).abs() == 0.5 && fround % 2.0 != 0.0 {
                        self.operandstack.push(f.trunc().into())
                    } else {
                        self.operandstack.push(fround.into())
                        
                    }
                },
                OpCode::F32Sqrt =>{
                    let v = self.operandstack.pop_as::<f32>();
                    self.operandstack.push(v.sqrt().into())
                },
                OpCode::F32Add =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((right+left).into())
                },
                OpCode::F32Sub =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left-right).into())
                },
                OpCode::F32Mul =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left*right).into())
                },
                OpCode::F32Div =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left/right).into())
                },
                OpCode::F32Min =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left.min(right)).into())
                },
                OpCode::F32Max =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left.max(right)).into())
                },
                OpCode::F32CopySign =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left.copysign(right)).into())
                },
                OpCode::F64Abs =>{
                    let v = self.operandstack.pop_as::<f64>();
                    self.operandstack.push(v.abs().into())
                },
                OpCode::F64Neg =>{
                    let v = self.operandstack.pop_as::<f64>();
                    self.operandstack.push(v.neg().into())
                },
                OpCode::F64Ceil =>{
                    let v = self.operandstack.pop_as::<f64>();
                    self.operandstack.push(v.ceil().into())
                },
                OpCode::F64Floor =>{
                    let v = self.operandstack.pop_as::<f64>();
                    self.operandstack.push(v.floor().into())
                },
                OpCode::F64Trunc =>{
                    let v = self.operandstack.pop_as::<f64>();
                    self.operandstack.push(v.trunc().into())
                },
                OpCode::F64Nearest =>{
                    let f = self.operandstack.pop_as::<f64>();
                    let fround = f.round();
                    if (f - fround).abs() == 0.5 && fround % 2.0 != 0.0 {
                        self.operandstack.push(f.trunc().into())
                    } else {
                        self.operandstack.push(fround.into())
                        
                    }
                },
                OpCode::F64Sqrt =>{
                    let v = self.operandstack.pop_as::<f64>();
                    self.operandstack.push(v.sqrt().into())
                },
                OpCode::F64Add =>{
                    let right = self.operandstack.pop_as::<f64>();
                    let left = self.operandstack.pop_as::<f64>();
                    self.operandstack.push((right+left).into())
                },
                OpCode::F64Sub =>{
                    let right = self.operandstack.pop_as::<f64>();
                    let left = self.operandstack.pop_as::<f64>();
                    self.operandstack.push((left-right).into())
                },
                OpCode::F64Mul =>{
                    let right = self.operandstack.pop_as::<f64>();
                    let left = self.operandstack.pop_as::<f64>();
                    self.operandstack.push((left*right).into())
                },
                OpCode::F64Div =>{
                    let right = self.operandstack.pop_as::<f64>();
                    let left = self.operandstack.pop_as::<f64>();
                    self.operandstack.push((left/right).into())
                },
                OpCode::F64Min =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left.min(right)).into())
                },
                OpCode::F64Max =>{
                    let right = self.operandstack.pop_as::<f32>();
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left.max(right)).into())
                },
                OpCode::F64CopySign =>{
                    let right = self.operandstack.pop_as::<f64>();
                    let left = self.operandstack.pop_as::<f64>();
                    self.operandstack.push((left.copysign(right)).into())
                },

                OpCode::I32WrapI64 =>{
                    let left = self.operandstack.pop_as::<i64>();
                    self.operandstack.push((left as i32).into())
                },
                OpCode::I32TruncF32S =>{
                    let left = self.operandstack.pop_as::<f32>();
                    left.try_truncate_into();
                    self.operandstack.push((left as i32).into())
                },
                OpCode::I32TruncF32U =>{
                    let left = self.operandstack.pop_as::<f32>();
                    self.operandstack.push((left as i32).into())
                },
                OpCode::I32TruncF64S =>{
                    let left = self.operandstack.pop_as::<f64>();
                    self.operandstack.push((left as i32).into())
                },
                OpCode::I32TruncF64U =>{
                    let left = self.operandstack.pop_as::<f64>();
                    self.operandstack.push((left as i32).into())
                },
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