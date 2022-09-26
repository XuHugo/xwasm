
pub mod types;
pub mod module;
pub mod instruction;
pub mod parse;
pub mod opcodes;
pub mod vm;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
