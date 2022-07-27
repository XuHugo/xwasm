use crate::types::*;
use crate::instruction::*;


const MAGIC_NUMBER: u32 = 0x6D736100;
const VERSION: u32 = 0x00000001;

pub enum SectionID {
    SecCustomID = 0,
    SecTypeID,
    SecImportID,
    SecFuncID,
    SecTableID,
    SecMemID,
    SecGlobalID,
    SecExportID,
    SecStartID,
    SecElemID,
    SecCodeID,
    SecDataID,
}
#[derive(Debug)]
pub enum ImportTag {
    ImportTagFunc,
    ImportTagTable,
    ImportTagMem,
    ImportTagGlobal,
}
#[derive(Debug)]
pub enum ExportTag {
    ExportTagFunc=0,
    ExportTagTable,
    ExportTagMem,
    ExportTagGlobal,
}

pub type TypeIndex = u32;
pub type FuncIndex = u32;
pub type TableIndex = u32;
pub type MemIndex = u32;
pub type GlobalIndex = u32;
pub type LocalIndex = u32;
pub type LabelIndex = u32;

pub struct Module {
    pub magic:    u32,
    pub version:  u32,
    pub custom:   Option<Vec<CustomSection>>,
    pub types:    Option<Vec<TypeSection>>,
    pub import:   Option<Vec<ImportSection>>,
    pub function: Option<Vec<FunctionSection>>,
    pub table:    Option<Vec<TableSection>>,
    pub memory:   Option<Vec<MemorySection>>,
    pub global:   Option<Vec<GlobalSection>>,
    pub export:   Option<Vec<ExportSection>>,
    pub start:    Option<StartSection>,
    pub element:  Option<Vec<ElementSection>>,
    pub code:     Option<Vec<CodeSection>>,
    pub data:     Option<Vec<DataSection>>,
}

#[derive(Debug)]
pub struct CustomSection {
    pub name:     String,
    pub contents: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct TypeSection{
    pub types:Vec<FunctionType>
}

#[derive(Debug, Default)]
pub struct ImportSection{
    pub imports: Vec<Import>,
}

#[derive(Debug)]
pub struct Import {
    pub module:    String,
    pub name:   String,
    pub description: ImportDescription,
}

#[derive(Debug)]
pub struct  ImportDescription {
    pub tag: ImportTag,
    pub func: TypeIndex,
    pub table: TableType,
    pub mem: MemoryType,
    pub global: GlobalType,
}


#[derive(Debug, Default)]
pub struct FunctionSection {
    pub types: Vec<TypeIndex>,
}

#[derive(Debug, Default)]
pub struct TableSection {
    pub table_type: Vec<TableType>,
}

#[derive(Debug, Default)]
pub struct MemorySection {
    pub memory_type: Vec<MemoryType>,
}

pub struct GlobalSection {
    pub globals: Vec<Global>
}

pub struct Global {
    pub types: GlobalType,
    pub expr: Box<dyn Expr>,
}

#[derive(Debug, Default)]
pub struct ExportSection {
    pub exports: Vec<Export>,
}

#[derive(Debug)]
pub struct Export {
    pub name:        String,
    pub description: ExportDescription,
}

#[derive(Debug)]
pub struct  ExportDescription {
    pub tag: ExportTag,
    pub index: u32,
}


#[derive(Debug, Default)]
pub struct StartSection(pub FuncIndex);

pub struct ElementSection {
    pub table: TableIndex,
    pub offset: Box<dyn Expr>,
    pub init: Vec<FuncIndex>,
}

pub struct CodeSection {
    pub locals: Vec<Locals>,
    pub expr: Box<dyn Expr>,
}

pub struct Locals {
    pub n:u32,
    pub types: ValueType,
}


pub struct DataSection {
    pub mem: MemIndex,
    pub offset: Box<dyn Expr>,
    pub init: Vec<u8>,
}
