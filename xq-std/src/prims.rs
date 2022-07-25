
#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "xq"))]
extern "C" {
    pub(crate) fn get_invoker(start: *mut u8);
    pub(crate) fn get_sender(start: *mut u8);
    pub(crate) fn get_owner(start: *mut u8);

    pub(crate) fn get_self_address(start: *mut u8);
    pub(crate) fn get_self_balance() -> u64;

    pub(crate) fn get_parameter(param_bytes: *mut u8, length: u32) -> u32;
    pub(crate) fn _transfer(address: *const u8, amount: u64) -> u32;
    pub(crate) fn _call(
        address: *const u8,
        amount: u64,
        parameter: *const u8,
        parameter_len: u32,
    ) -> u32;

    pub(crate) fn get_store(start: *mut u8, length: u32) -> u32;
    pub(crate) fn set_store(kstart: *const u8, klength: u32, vstart: *mut u8, vlength: u32) -> u32;

    pub(crate) fn get_block_time() -> u64;
    pub(crate) fn get_tx_hash(hash_bytes: *mut u8) -> u64;
    pub(crate) fn get_block_number() -> u64;

    pub(crate) fn set_event(start: *const u8, length: u32) -> i32;
    pub(crate) fn set_return_data(start: *const u8, length: u32) -> u32;
    pub(crate) fn set_error(start: *const u8, length: u32) -> i32;
}