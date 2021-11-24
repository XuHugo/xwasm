


#[derive(Debug)]
pub struct UnparsedSection<'a>{
    pub section_id: SectionId,
    pub bytes: &'a[u8],
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
pub enum SectionId{
    Custom = 0,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

#[derive(Debug)]
pub struct Skeleton<'a>{
    pub custom:  Vec<UnparsedSection<'a>>,
    pub types:   Option<UnparsedSection<'a>>,
    pub import:  Option<UnparsedSection<'a>>,
    pub func:    Option<UnparsedSection<'a>>,
    pub table:   Option<UnparsedSection<'a>>,
    pub memory:  Option<UnparsedSection<'a>>,
    pub global:  Option<UnparsedSection<'a>>,
    pub export:  Option<UnparsedSection<'a>>,
    pub start:   Option<UnparsedSection<'a>>,
    pub element: Option<UnparsedSection<'a>>,
    pub code:    Option<UnparsedSection<'a>>,
    pub data:    Option<UnparsedSection<'a>>,
}

pub type ParseResult<A> = anyhow::Result<A>;

pub trait Parseable<'a, Ctx>: Sized {
    fn parse(ctx: Ctx, cursor: &mut Cursor<&'a [u8]>) -> ParseResult<Self>;
}



