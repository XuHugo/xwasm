
use rusty_leveldb::{DB, DBIterator, LdbIterator, Options};

let opt = rusty_leveldb::in_memory();
let mut db =  DB::open("contract", opt).unwrap();

//db.put(b"Hello", b"world").unwrap();

//let g = db.get(b"Hello").unwrap();

#[macro_use]
use lazy_static;

lazy_static!{
    pub static KVDB = DB::open("contract", rusty_leveldb::in_memory()).unwrap();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
