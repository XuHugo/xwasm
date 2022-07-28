use std::{
    convert::TryFrom,
    io::{Cursor, Read, SeekFrom, Seek}
};
use anyhow::{bail, ensure, Result, Ok};

use crate::{
    types::{FunctionType, FUNC_TAG, ValueType, TableType, Limits, MemoryType, GlobalType, EmptyExpr}, 
    module::{TypeSection,FunctionSection,TableSection, MemorySection, GlobalSection, Global, ExportSection, Export, ExportDescription, ExportTag,
        StartSection, ElementSection, CodeSection, Locals, DataSection, ImportSection, Import, ImportDescription, ImportTag, ImportDes}
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
            7 => {decode_export(cursor);},
            8 => {decode_start(cursor);},
            9 => {decode_element(cursor);},
            10 => {decode_code(cursor);},
            11 => {decode_data(cursor);},
            id => bail!("Unknown section id {}", id),
        };
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

impl Decode for String{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let len = u32::decode(cursor).unwrap();
        let start = cursor.position() as usize;
        let end = start + len as usize;
        cursor.seek(SeekFrom::Current(i64::from(len)))?;
        let str = std::str::from_utf8(&cursor.get_ref()[start..end]).unwrap().to_string();
        Ok(str)
    }
}

impl Decode for ExportTag{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> {
        let tag = u8::decode(cursor).unwrap();
        match tag {
            0x00 => Ok(ExportTag::ExportTagFunc),
            0x01 => Ok(ExportTag::ExportTagTable),
            0x02 => Ok(ExportTag::ExportTagMem),
            0x03 => Ok(ExportTag::ExportTagGlobal),
            other => bail!("error export tag {:#x}.", other),
        }
    }
}

impl Decode for ExportDescription{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let tag = ExportTag::decode(cursor).unwrap();
        let index = u32::decode(cursor).unwrap();
        Ok(ExportDescription{
            tag,
            index,
        })
    }
}

impl Decode for Export{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self> {
        let name = String::decode(cursor).unwrap();
        let description = ExportDescription::decode(cursor).unwrap();
        Ok(Export{
            name,
            description
        })
    }
}

impl Decode for ElementSection{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let table = u32::decode(cursor).unwrap();
        let offset = EmptyExpr::decode(cursor).unwrap();
        let init = Vec::<u32>::decode(cursor).unwrap();
        println!("types: id:{:?} init:{:?}",table, init);
        Ok(ElementSection{
            table,
            offset:Box::new(offset),
            init,
        })
    }
}

impl Decode for Locals{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let n = u32::decode(cursor).unwrap();
        let types = ValueType::decode(cursor).unwrap();
        Ok(Locals{
            n,
            types,
        })
    }
}

impl Decode for CodeSection{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let byte_count = u32::decode(cursor).unwrap();
        println!("code byte count:{:?}",byte_count);
        let start = cursor.position();
        let locals = Vec::<Locals>::decode(cursor).unwrap();
        let end = cursor.position();
        let expr_count = u64::from(byte_count) - (end - start);
        let expr_bytes = &cursor.get_ref()[end as usize..(end + expr_count) as usize];
        cursor.set_position(end + expr_count);
        Ok(CodeSection{
            locals,
            expr:Box::new(EmptyExpr),
        })
    }
}

impl Decode for DataSection{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let mem = u32::decode(cursor).unwrap();
        let offset = EmptyExpr::decode(cursor).unwrap();
        let init = Vec::<u8>::decode(cursor).unwrap();
        println!("types: id:{:?} init:{:?}",mem, init);
        Ok(DataSection{
            mem,
            offset:Box::new(offset),
            init,
        })
    }
}

impl Decode for ImportTag{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let tag = u8::decode(cursor).unwrap();
        match tag {
            0x00 => Ok(ImportTag::ImportTagFunc),
            0x01 => Ok(ImportTag::ImportTagTable),
            0x02 => Ok(ImportTag::ImportTagMem),
            0x03 => Ok(ImportTag::ImportTagGlobal),
            other => bail!("error import tag {:#x}.", other),
        }
    }
}

impl Decode for ImportDescription{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let tag = ImportTag::decode(cursor).unwrap();
        let mut importdes = ImportDes::Func(0);
        match tag{
            ImportTag::ImportTagFunc =>{
                let func = u32::decode(cursor).unwrap();
                importdes = ImportDes::Func(func);
            },
            ImportTag::ImportTagTable => {
                let table = TableType::decode(cursor).unwrap();
                importdes = ImportDes::Table(table);
            },
            ImportTag::ImportTagMem => {
                let mem = MemoryType::decode(cursor).unwrap();
                importdes = ImportDes::Mem(mem);
            },
            ImportTag::ImportTagGlobal => {
                let global = GlobalType::decode(cursor).unwrap();
                importdes = ImportDes::Global(global);
            },
        }
        Ok(ImportDescription{
            tag,
            importdes,
        })
    }
}

impl Decode for Import{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{
        let module = String::decode(cursor).unwrap();
        let name = String::decode(cursor).unwrap();
        let description = ImportDescription::decode(cursor).unwrap();
        Ok(Import{
            module,
            name,
            description,
        })
    }
}

// impl Decode for Vec<u8>{
//     fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<Self>{   
//         let len = u32::decode(cursor).unwrap();
//         let start = cursor.position() as usize;
//         let end = start + len as usize;
//         cursor.seek(SeekFrom::Current(i64::from(len)))?;
//         let str = cursor.get_ref()[start..end].to_vec();
//         Ok(str)
//     }
// }

fn decode_custom(cursor: &mut Cursor<Vec<u8>>) {
    println!("#decode_custom:");
    let byte_count = Vec::<u8>::decode(cursor).unwrap();
    // let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {:?}", byte_count);
    // let name = String::decode(cursor).unwrap();
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
fn decode_import(cursor: &mut Cursor<Vec<u8>>)->Result<ImportSection> {
    println!("#decode_import:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let imports = Vec::<Import>::decode(cursor).unwrap();
    println!("types:{:?}",imports);
    Ok(ImportSection{imports})
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
    println!("types:{:?}",exports);
    Ok(ExportSection{exports})
}
fn decode_start(cursor: &mut Cursor<Vec<u8>>)->Result<StartSection> {
    println!("#decode_start:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let func_index = u32::decode(cursor).unwrap();
    println!("types:{:?}",func_index);
    Ok(StartSection(func_index))
}
fn decode_element(cursor: &mut Cursor<Vec<u8>>)->Result<Vec<ElementSection>>{
    println!("#decode_element:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let element = Vec::<ElementSection>::decode(cursor).unwrap();
    Ok(element)
}
fn decode_code(cursor: &mut Cursor<Vec<u8>>)->Result<Vec<CodeSection>> {
    println!("#decode_code:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let code = Vec::<CodeSection>::decode(cursor).unwrap();
    Ok(code)
}
fn decode_data(cursor: &mut Cursor<Vec<u8>>)->Result<Vec<DataSection>> {
    println!("#decode_data:");
    let byte_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}", byte_count);
    let data = Vec::<DataSection>::decode(cursor).unwrap();
    Ok(data)
}