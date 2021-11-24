

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::types::LogError;
use xq_common::*;

pub trait HasParameter: Read{
    fn size(&self) -> u32;
}

pub trait HasChainMetadata {
    fn slot_time(&self) -> SlotTime;
}

pub trait HasCommonData {
    type MetadataType: HasChainMetadata;
    type ParamType: HasParameter + Read;

    fn metadata(&self) -> &Self::MetadataType;
    fn parameter_cursor(&self) -> Self::ParamType;
}

pub trait HasInitContext<Error:Default =()>:HasCommonData{
    type InitData;
    fn open(data: Self::InitData)-> Self;
    fn init_origin(&self) -> AccountAddress;
}

pub trait HasReceiveContext<Error: Default = ()>:HasCommonData{
    type ReceiveData;

    fn open(data: Self::ReceiveData) -> Self;
    fn invoker(&self) -> AccountAddress;
    fn self_address(&self) -> ContractAddress;
    fn self_balance(&self) -> Amount;
    fn sender(&self) -> Address;
    fn owner(&self) -> AccountAddress;
}

pub trait HasContractState<Error: Default = ()>
where
    Self: Read,
    Self: Write<Err = Error>,
    Self: Seek<Err = Error>,{
    type ContractStateData;

    fn open(_: Self::ContractStateData)-> Self;
    fn size(&self) -> u32;
    fn truncate(&mut self, new_size:u32);
    fn reserve(&mut self, len: u32)->bool;
}

pub  trait HasLogger {
    fn init() -> Self;
    fn log_raw(&mut self, event: &[u8]) -> Result<(), LogError>;
    #[inline(always)]
    fn log<S: Serial>(&mut self, event: &S) -> Result<(), LogError>{
        let mut out = Vec::new();
        if event.serial(&mut out).is_err(){
            crate::trap();
        }
        self.log_raw(&out)
    }
}

pub trait HasActions {
    fn accept() -> Self;
    fn simple_transfer(acc: &AccountAddress, amount: Amount) -> Self;
    fn send_raw(
        ca: &ContractAddress,
        receive_name: ReceiveName,
        amount: Amount,
        parameter: &[u8],
    ) -> Self;
    fn and_then(self, then:Self) -> Self;
    fn or_else(self, el: Self) -> Self;
}

pub trait UnwrapAbort{
    type Unwrap;

    fn unwrap_abort(self) -> Self::Unwrap;
}

pub trait ExpectErrReport {
    type Unwrap;
    fn expect_err_report(self, msg: &str) -> Self::Unwrap;
}

pub trait SerialCtx{
    fn serial_ctx<W: Write>(
        &self,
        size_length: schema::SizeLength,
        out: &mut W,
    ) -> Result<(), W::Err>;
}

pub trait DeserialCtx: Sized {
    fn deserial_ctx<R: Read>(
        size_leng: schema::SizeLength,
        ensure_ordered: bool,
        source: &mut R,
    ) -> ParseResult<Self>;
}


