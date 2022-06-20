

pub trait Get<T> {
    fn get(&mut self) -> ParseResult<T>;
}

impl <R: Read, T: Deserial> Get<T> for R {
    #[inline(always)]
    fn get(&mut self) -> ParseResult<T> {T::deserial(self)}
}