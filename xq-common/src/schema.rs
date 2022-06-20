

pub trait SchemaType{
    fn get_type() -> crate::schema::Type;
}

#[derive(Debug, Clone)]
pub struct Module{
    pub contracts: BTreeMap<String, Contract>,
}

#[derive(Debug, Clone)]
pub struct Contract {
    pub state: Option<Type>,
    pub event: Option<Type>,
    pub init: Option<Type>,
    pub invoke: BTreeMap<String, Type>,
}

impl Contract {
    pub fn empty() -> Contract{
        Contract{
            state: None,
            event: None,
            init: None,
            invoke: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fields {
    Named(Vec<(String, Type)>),
    Unnamed(Vec<Type>),
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SizeLength{
    U8,
    U16,
    U32,
    U64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    Amount,
    AccountAddress,
    ContractAddress,
    Timestamp,
    Duration,
    Pair(Box<Type>, Box<Type>),
    List(SizeLength, Box<Type>),
    Set(SizeLength, Box<Type>),
    Map(SizeLength, Box<Type>, Box<Type>),
    Array(u32, Box<Type>),
    Struct(Fields),
    Enum(Vec<(String, Fields)>),
    String(SizeLength),
    ContractName(SizeLength),
    ReceiveName(SizeLength),
}

impl Type {
    pub fn set_size_length(self, size_len: SizeLength) -> Type{
        match self{
            Type::List(_, ty) => Type::List(size_len, ty),
            Type::Set(_, ty) => Type::Set(size_len, ty),
            Type::Map(_,key_ty, val_ty) => Type::Map(size_len,key_ty,val_ty),
            Type::String(_) => Type::String(size_len),
            t => t,
        }
    }
}

impl SchemaType for () {
    fn get_type() -> Type {
        Type::Unit
    }
}

impl SchemaType for bool {
    fn get_type() -> Type {
        Type::Bool
    }
}

impl SchemaType for u8 {
    fn get_type() -> Type { Type::U8 }
}
impl SchemaType for u16 {
    fn get_type() -> Type { Type::U16 }
}
impl SchemaType for u32 {
    fn get_type() -> Type { Type::U32 }
}
impl SchemaType for u64 {
    fn get_type() -> Type { Type::U64 }
}
impl SchemaType for u128 {
    fn get_type() -> Type { Type::U128 }
}
impl SchemaType for i8 {
    fn get_type() -> Type { Type::I8 }
}
impl SchemaType for i16 {
    fn get_type() -> Type { Type::I16 }
}
impl SchemaType for i32 {
    fn get_type() -> Type { Type::I32 }
}
impl SchemaType for i64 {
    fn get_type() -> Type { Type::I64 }
}
impl SchemaType for i128 {
    fn get_type() -> Type { Type::I128 }
}
impl SchemaType for Amount {
    fn get_type() -> Type { Type::Amount }
}
impl SchemaType for AccountAddress {
    fn get_type() -> Type { Type::AccountAddress }
}
impl SchemaType for ContractAddress {
    fn get_type() -> Type { Type::ContractAddress }
}
impl SchemaType for Timestamp {
    fn get_type() -> Type { Type::Timestamp }
}
impl SchemaType for Duration {
    fn get_type() -> Type { Type::Duration }
}
impl<T:SchemaType> SchemaType for Option<T> {
    fn get_type() -> Type {
        Type::Enum(Vec::from([
            (String::from("None"), Fields::None),
            (String::from("Some"), Fields::Unnamed(Vec::from([T::get_type()]))),
        ]))
    }
}

impl <L:SchemaType, R:SchemaType> SchemaType for (L, R){
    fn get_type() -> Type {
        Type::Pair(Box::new(L::get_type()), Box::new(R::get_type()))
    }
}

impl <T:SchemaType> SchemaType for Vec<T>{
    fn get_type() -> Type {
        Type::List(SizeLength::U32, Box::new(T::get_type()))
    }
}

impl <K:SchemaType, V:SchemaType> SchemaType for BTreeMap<K, V> {
    fn get_type() -> Type {
        Type::Map(SizeLength::U32, Box::new(K::get_type()), Box::new(V::get_type()))
    }
}

macro_rules! schema_type_array_x {
    ($x: expr) => {
        impl <A: SchemaType> SchemaType for [A; $x]{
            fn get_type() -> Type {Type::Array($x, Box::new(A::get_type()))}
        }
    }
}

