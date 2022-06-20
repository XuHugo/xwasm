




pub fn to_bytes<S: Serial>(x: &S) -> Vec<u8> {
    let mut out = Vec::new();
    let mut cursor = Cursor::new(&mut out);
    x.serial(&mut cursor).expect("writing to a vector should succeed.");
    out
}

pub fn from_bytes<S: Deserial>(source: &[u8]) -> ParseResult<S> {
    let mut cursor = Cursor::new(source);
    cursor.get()
}