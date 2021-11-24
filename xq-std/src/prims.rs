
#[cfg_attr(target_arch = "wasm32", link(wasm_import_module = "xq"))]
extern "C" {
    pub(crate) fn accept() -> u32;
    pub(crate) fn simple_transfer(addr_bytes: *const u8, amount: u64) -> u32;
    pub(crate) fn send(
        addr_bytes: *const u8,
        receive_name: *const u8,
        receive_name_len: u32,
        amount: u64,
        parameter: *const u8,
        parameter_len: u32,
    ) -> u32;
    pub(crate) fn combine_and(l:u32, r:u32) -> u32;
    pub(crate) fn combine_or(l:u32, r:u32) -> u32;
    pub(crate) fn get_parameter_size() -> u32;
    pub(crate) fn get_parameter_section(param_bytes: *mut u8, length: u32, offset: u32) -> u32;
    pub(crate) fn log_event(start: *const u8, length: u32) -> i32;
    pub(crate) fn load_state(start: *mut u8, length: u32, offset: u32) -> u32;
    pub(crate) fn write_state(start: *const u8, length: u32, offset: u32) -> u32;
    pub(crate) fn resize_state(new_size: u32) -> u32;
    pub(crate) fn state_size() -> u32;

    pub(crate) fn get_init_origin(start: *mut u8);
    pub(crate) fn get_receive_invoker(start: *mut u8);
    pub(crate) fn get_receive_self_address(start: *mut u8);
    pub(crate) fn get_receive_sender(start: *mut u8);
    pub(crate) fn get_receive_owner(start: *mut u8);

    pub(crate) fn get_slot_time() -> u64;

    pub(crate) fn report_error(
        msg_start: *const u8,
        msg_length: u32,
        filename_start: *const u8,
        filename_length: u32,
        line: u32,
        column: u32,
    );

    pub(crate) fn wirte_return(start: *const u8, length: u32) -> u32;
}