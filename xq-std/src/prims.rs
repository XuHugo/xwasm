
#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "xq"))]
extern "C" {
    pub(crate) fn get_invoker(start: *mut u8);
    pub(crate) fn get_self_address(start: *mut u8);
    pub(crate) fn get_self_balance() -> u64;
    pub(crate) fn get_sender(start: *mut u8);
    pub(crate) fn get_owner(start: *mut u8);

    pub(crate) fn get_parameter(param_bytes: *mut u8, length: u32) -> u32;

    pub(crate) fn transfer(address: *const u8, amount: u64) -> u32;
    pub(crate) fn send(
        address: *const u8,
        receive_name: *const u8,
        receive_name_len: u32,
        amount: u64,
        parameter: *const u8,
        parameter_len: u32,
    ) -> u32;

    pub(crate) fn state_get(start: *mut u8, length: u32, offset: u32) -> u32;
    pub(crate) fn state_set(start: *const u8, length: u32, offset: u32) -> u32;

    pub(crate) fn store_get(start: *mut u8, length: u32) -> u32;
    pub(crate) fn store_set(kstart: *const u8, klength: u32, vstart: *mut u8, vlength: u32) -> u32;

    pub(crate) fn get_block_time() -> u64;
    pub(crate) fn get_tx_hash() -> u64;
    pub(crate) fn get_block_number() -> u64;

    pub(crate) fn event(start: *const u8, length: u32) -> i32;
    pub(crate) fn return_data_set(start: *const u8, length: u32) -> u32;
    pub(crate) fn error_set(start: *const u8, length: u32) -> i32;
}