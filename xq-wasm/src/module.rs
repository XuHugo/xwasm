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
    ImportTagFunc=0,
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

#[derive(Default, Debug)]
pub struct Module {
    pub magic:    [u8; 4],
    pub version:  [u8; 4],
    pub custom:   Option<CustomSection>,
    pub types:    Option<TypeSection>,
    pub import:   Option<ImportSection>,
    pub function: Option<FunctionSection>,
    pub table:    Option<TableSection>,
    pub memory:   Option<MemorySection>,
    pub global:   Option<GlobalSection>,
    pub export:   Option<ExportSection>,
    pub start:    Option<StartSection>,
    pub element:  Option<Vec<ElementSection>>,
    pub code:     Option<Vec<CodeSection>>,
    pub data:     Option<Vec<DataSection>>,
}

impl  Module {
    pub fn set_magic(&mut self, magic:[u8; 4]){
        self.magic=magic;
    }
    pub fn set_version(&mut self, version:[u8; 4]){
        self.version=version;
    }
    pub fn set_custom(&mut self, custom:Option<CustomSection>){
        self.custom=custom;
    }
    pub fn set_types(&mut self, types:Option<TypeSection>){
        self.types=types;
    }
    pub fn set_import(&mut self, import:Option<ImportSection>){
        self.import=import;
    }
    pub fn set_function(&mut self, function:Option<FunctionSection>){
        self.function=function;
    }
    pub fn set_table(&mut self, table:Option<TableSection>){
        self.table=table;
    }
    pub fn set_memory(&mut self, memory:Option<MemorySection>){
        self.memory=memory;
    }
    pub fn set_global(&mut self, global:Option<GlobalSection>){
        self.global=global;
    }
    pub fn set_export(&mut self, export:Option<ExportSection>){
        self.export=export;
    }
    pub fn set_start(&mut self, start:Option<StartSection>){
        self.start=start;
    }
    pub fn set_element(&mut self, element:Option<Vec<ElementSection>>){
        self.element=element;
    }
    pub fn set_code(&mut self, code:Option<Vec<CodeSection>>){
        self.code=code;
    }
    pub fn set_data(&mut self, data:Option<Vec<DataSection>>){
        self.data=data;
    }
}

#[derive(Debug)]
pub struct CustomSection {
    pub name:     String,
    pub contents: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct  TypeSection{
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
pub enum ImportDes{
    Func(TypeIndex),
    Table(TableType),
    Mem(MemoryType),
    Global(GlobalType),
}

#[derive(Debug)]
pub struct  ImportDescription {
    pub tag: ImportTag,
    pub importdes: ImportDes,
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
#[derive(Debug)]
pub struct GlobalSection {
    pub globals: Vec<Global>
}

#[derive(Debug)]
pub struct Global {
    pub types: GlobalType,
    pub expr: Expr,
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
#[derive(Debug)]
pub struct ElementSection {
    pub table: TableIndex,
    pub offset: Expr,
    pub init: Vec<FuncIndex>,
}
#[derive(Debug)]
pub struct CodeSection {
    pub locals: Vec<Locals>,
    pub expr: Expr,
}
#[derive(Debug)]
pub struct Locals {
    pub n:u32,
    pub types: ValueType,
}

#[derive(Debug)]
pub struct DataSection {
    pub mem: MemIndex,
    pub offset: Expr,
    pub init: Vec<u8>,
}
