use std::{
    convert::TryFrom,
    io::{Cursor, Read}
};
use anyhow::{bail, ensure, Result};

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
            1 => decode_type(cursor),
            2 => decode_import(cursor),
            3 => decode_function(cursor),
            4 => decode_table(cursor),
            5 => decode_memory(cursor),
            6 => decode_global(cursor),
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

impl Decode for u32{
    fn decode(cursor: &mut Cursor<Vec<u8>>)->Result<u32>{
        let num32 = leb128::read::unsigned(&mut cursor.take(5))?;
        Ok(u32::try_from(num32)?)
    }
}

fn decode_custom(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_custom~");

}
fn decode_type(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_type:");
    let byte_count = u32::decode(cursor).unwrap();
    let type_count = u32::decode(cursor).unwrap();
    println!("byte_count: {}-{}", byte_count,type_count);
}
fn decode_import(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_import~");
}
fn decode_function(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_function~");
}
fn decode_table(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_table~");
}
fn decode_memory(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_memory~");
}
fn decode_global(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_global~");
}
fn decode_export(cursor: &mut Cursor<Vec<u8>>) {
    println!("decode_export~");
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