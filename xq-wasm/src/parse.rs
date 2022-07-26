use std::{
    convert::TryFrom,
    io::{Cursor, Read}
};
use anyhow::{bail, ensure, Result, Ok};

use crate::{
    types::{FunctionType, FUNC_TAG, ValueType, TableType, Limits, MemoryType, GlobalType, EmptyExpr}, 
    module::{TypeSection,FunctionSection,TableSection, MemorySection, GlobalSection, Global, ExportSection, Export}
};

pub const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
pub const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

pub type ParseResult<A> = anyhow::Result<A>;

pub fn decode(code:Vec<u8>)->ParseResult<()>{
    let len = code.len() as u64;
    let cursor = &mut Cursor::new(code);
    {
        let mut buf = [0u8; 4];
        cursor.read_exact(&mut buf)?;
        ensure!(buf == MAGIC, "error magic !");
        cursor.read_exact(&mut buf)?;
        ensure!(buf == VERSION, "error version !");
    }
    println!("len = {}", len);

    while cursor.position() < len  {
        let mut buf = [0u8; 1];
        cursor.read_exact(&mut buf)?;
        println!("secid={}",buf[0]);
        match buf[0] {
            0 => decode_custom(cursor),
            1 => {decode_type(cursor);},
            2 => {decode_import(cursor);},
            3 => {decode_function(cursor);},
            4 => {decode_table(cursor);},
            5 => {decode_memory(cursor);},
            6 => {decode_global(cursor);},
            7 => decode_export(cursor),
            8 => decode_start(cursor),
            9 => decode_element(cursor),
            10 => decode_code(cursor),
            11 => decode_data(cursor),
            id => bail!("Unknown section id {}", id),
        };

        // let section = UnparsedSection::parse(EMPTY_CTX, code)?;
        // ensure!(
        //     section.section_id == SectionId::Custom || section.section_id > last_section,
        //     "Section out of place."
        // );
        // if section.section_id != SectionId::Custom {
        //     last_section = section.section_id
        // }
        // match section.section_id {
        //     SectionId::Custom => custom.push(section),
        //     SectionId::Type => ty = Some(section),
        //     SectionId::Import => import = Some(section),
        //     SectionId::Function => func = Some(section),
        //     SectionId::Table => table = Some(section),
        //     SectionId::Memory => memory = Some(section),
        //     SectionId::Global => global = Some(section),
        //     SectionId::Export => export = Some(section),
        //     SectionId::Start => start = Some(section),
        //     SectionId::Element => element = Some(section),
        //     SectionId::Code => code = Some(section),
        //     SectionId::Data => data = Some(section),
        // }
    }

    Ok(())

}

pub trait Decode {
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> where Self: Sized;
}

impl Decode for u8{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<u8>{
        let mut buf = [0u8; 1];
        cursor.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl Decode for u32{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<u32>{
        let num32 = leb128::read::unsigned(&mut cursor.take(5))?;
        Ok(u32::try_from(num32)?)
    }
}

impl <A: Decode> Decode for Vec<A>{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let len = u32::decode(cursor).unwrap();
        let mut out = Vec::with_capacity(len as usize);
        for _ in 0..len{
            out.push(A::decode(cursor).unwrap());
        }
        Ok(out)
    }
}

impl Decode for ValueType{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> {
        let byte = u8::decode(cursor).unwrap();
        if let core::result::Result::Ok(x) = ValueType::try_from(byte){
            Ok(x)
        }else{
            bail!("Unknown ValueType {}",byte)
        }
    }
}

impl Decode for FunctionType{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<FunctionType> {
        let tag = u8::decode(cursor).unwrap();
        ensure!( tag == FUNC_TAG, "error function type !");
        let parameters = Vec::<ValueType>::decode(cursor).unwrap();
        let results = Vec::<ValueType>::decode(cursor).unwrap();
        Ok(FunctionType{
            tag,
            parameters,
            results,
        })
    }
}

impl Decode for Limits{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> {
        let tag = u8::decode(cursor).unwrap();
        let min = u32::decode(cursor).unwrap();
        match tag {
            0x00 => Ok(Limits{
                tag,
                min,
                max:None,
            }),

            0x01 => {
                let max = u32::decode(cursor).unwrap();
                ensure!(min<=max,"min > max.");
                Ok(Limits{
                    tag,
                    min,
                    max:Some(max),
                })
            }

            _ => bail!("error tag {:#x}", tag),
        }
    }
}

impl Decode for TableType{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> {
        let elem_type = u8::decode(cursor).unwrap();
        let limits = Limits::decode(cursor).unwrap();
        Ok(TableType{
            elem_type,
            limits,
        })
    }
}

impl Decode for MemoryType{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let limits = Limits::decode(cursor).unwrap();
        Ok(MemoryType{
            limits,
        })
    }
}

impl Decode for GlobalType{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let init = ValueType::decode(cursor).unwrap();
        let mutable = u8::decode(cursor).unwrap();
        Ok(GlobalType{
            init,
            mutable,
        })
    }
}

impl Decode for Global{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> {
        let types = GlobalType::decode(cursor).unwrap();
        println!("types:{:?}",types.clone());
        let expr = EmptyExpr::decode(cursor).unwrap();
        Ok(Global{
            types,
            expr:Box::new(expr),
        })
    }
}

impl Decode for EmptyExpr{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> {
        let mut buf = [0u8; 1];
        while buf[0] !=  0x0b{
            cursor.read_exact(&mut buf)?;
        }
        Ok(EmptyExpr)
    }
}

impl Decode for Export{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> {
        
    }
}

fn decode_custom(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_custom~");

}
fn decode_type(cursor: &mut Cursor<Vec<u8>>) -> Result<TypeSection>{
    println!("#decode_type:");
    let byte_count = u32::decode(cursor).unwrap();
    let type_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}-{}", byte_count,type_count);
    let mut types = Vec::new();
    for _i in 0..type_count{
        let value = FunctionType::decode(cursor).unwrap();
        println!("type[{}]:{:?}",_i,value);
        types.push(value);
    }
    Ok(TypeSection{types})
}
fn decode_import(cursor: &mut Cursor<Vec<u8>>) {
    println!("#decode_import~");
}
fn decode_function(cursor: &mut Cursor<Vec<u8>>)->Result<FunctionSection> {
    println!("#decode_function:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let types = Vec::<u32>::decode(cursor).unwrap();
    println!("types:{:?}",types);
    Ok(FunctionSection{types})
}
fn decode_table(cursor: &mut Cursor<Vec<u8>>)->Result<TableSection> {
    println!("#decode_table:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let table_type = Vec::<TableType>::decode(cursor).unwrap();
    println!("types:{:?}",table_type);
    Ok(TableSection{table_type})
}
fn decode_memory(cursor: &mut Cursor<Vec<u8>>)->Result<MemorySection>{
    println!("#decode_memory:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let memory_type = Vec::<MemoryType>::decode(cursor).unwrap();
    println!("types:{:?}",memory_type);
    Ok(MemorySection{memory_type})
}
fn decode_global(cursor: &mut Cursor<Vec<u8>>)->Result<GlobalSection>{
    println!("#decode_global:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let globals = Vec::<Global>::decode(cursor).unwrap();
    
    Ok(GlobalSection{
        globals
    })
}
fn decode_export(cursor: &mut Cursor<Vec<u8>>) -> Result<ExportSection>{
    println!("#decode_export:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let exports = Vec::<Export>::decode(cursor).unwrap();
    Ok(ExportSection{exports})
}
fn decode_start(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_start~");
}
fn decode_element(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_element~");
}
fn decode_code(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_code~");
}
fn decode_data(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_data~");
}