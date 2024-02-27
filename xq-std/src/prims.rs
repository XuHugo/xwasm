
#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "xq"))]
extern "C" {
    pub(crate) fn get_invoker(value: *mut u8);
    pub(crate) fn get_owner(value: *mut u8);

    pub(crate) fn get_contract_address(value: *mut u8);
    pub(crate) fn get_contract_balance() -> u64;

    pub(crate) fn get_parameter(value: *mut u8) -> u32;
    pub(crate) fn transfer(address: *const u8, amount: u64) -> u32;
    pub(crate) fn call(
        address: *const u8,
        amount: u64,
        func_name: *const u8,
        func_name_len: u32,
        parameter: *const u8,
        parameter_len: u32,
    ) -> u32;

    pub(crate) fn get_store(key: *mut u8, length: u32, value: *mut u8) -> u32;
    pub(crate) fn set_store(key: *const u8, klength: u32, value: *mut u8, vlength: u32) -> u32;

    pub(crate) fn get_block_time() -> u64;
    pub(crate) fn get_block_hash(value: *mut u8) -> u64;
    pub(crate) fn get_block_height() -> u64;

    pub(crate) fn set_event(start: *const u8, length: u32) -> i32;
    pub(crate) fn set_return_data(start: *const u8, length: u32) -> u32;
}