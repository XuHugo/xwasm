


const MAGIC_NUMBER: i32 = 0x6D736100;
const VERSION: i32 = 0x00000001;

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
pub enum ImportTag {
    ImportTagFunc,
    ImportTagTable,
    ImportTagMem,
    ImportTagGlobal,
}

pub enum ExportTag {
    ExportTagFunc,
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

#[derive(Debug)]
pub struct Module {
    pub magic: u32,
    pub version: u32,
    pub custom: CustomSection,
    pub types:      TypeSection,
    pub import:  ImportSection,
    pub func:    FunctionSection,
    pub table:   TableSection,
    pub memory:  MemorySection,
    pub global:  GlobalSection,
    pub export:  ExportSection,
    pub start:   StartSection,
    pub element: ElementSection,
    pub code:    CodeSection,
    pub data:    DataSection,
}

#[derive(Debug)]
pub struct CustomSection<'a> {
    pub name:     Name,
    pub contents: &'a [u8],
}

#[derive(Debug, Default)]
pub struct TypeSection{
    pub types:Vec<Rc<FunctionType>>
}

#[derive(Debug, Default)]
pub struct ImportSection{
    pub imports: Vec<Import>,
}

#[derive(Debug)]
pub struct Import {
    pub mod_name:    String,
    pub item_name:   String,
    pub description: ImportDescription,
}

#[derive(Debug)]
pub struct  ImportDescription {
    pub tag: ImportTag,
    pub func: TypeIndex,
    pub table: TableType,
    pub mem: MemoryType,
    pub global: Global,
}


#[derive(Debug, Default)]
pub struct FunctionSection {
    pub types: Vec<TypeIndex>,
}

#[derive(Debug, Default)]
pub struct TableSection {
    pub table_type: Option<TableType>,
}

#[derive(Debug, Default)]
pub struct MemorySection {
    pub memory_type: Option<MemoryType>,
}

#[derive(Debug, Default)]
pub struct GlobalSection {
    pub globals: Vec<Global>,
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
pub struct StartSection {
    pub start:
}

#[derive(Debug, Default)]
pub struct ElementSection {
    pub elements: Vec<Element>,
}

#[derive(Debug, Default)]
pub struct CodeSection {
    pub impls: Vec<Code>,
}

#[derive(Debug, Default)]
pub struct DataSection {
    pub sections: Vec<Data>,
}
