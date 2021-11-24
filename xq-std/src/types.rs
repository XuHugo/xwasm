
#[derive(Default)]
pub struct ContractState{
    pub(crate)  current_position: u32,
}

#[derive(Default)]
pub struct Parameter {
    pub(crate) current_position: u32,
}

#[derive(Default)]
pub struct AttributesCursor{
    pub(crate) current_position: u32,
    pub(crate) remaining_items: u16,
}

#[derive(Default)]
pub struct Logger{
    pub(crate) _private:(),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum LogError{
    Full,
    Malformed,
}

#[must_use]
pub struct Action{
    pub(crate) _private: u32,
}

impl Action {
    pub fn tag(&self) -> u32{self._private}
}

#[derive(Eq, ParitalEq, Debug)]
#[repr(transparent)]
pub struct Reject{
    pub error_code: crate::num::NonZeroI32,
}

impl Default for Reject{
    #[inline(always)]
    fn default() -> Self{
        Self{
            error_code: unsafe {crate::num::NonZeroI32::new_unchecked(i32::MIN)},
        }
    }
}

impl Reject{
    pub fn new(x: i32) -> Option<Self>{
        if x < 0 {
            let error_code = unsafe {crate::num::NonZeroI32::new_unchecked(x)};
            Some(Reject{
                error_code,
            })
        }else{
            None
        }
    }
}

#[macro_export]
macro_rules! bail{
    () => {{
       return Err(Default::default());
    }};
    ($arg:expr) => {{
        return Err($arg);
    }};
}

#[macro_export]
macro_rules! ensure{
    ($p:expr) => {
      if !$p {
          $crate::bail!();
      }
    };
    ($p:expr, $arg:expr) => {{
        if !$p{
            $crate::bail!($arg);
        }
    }};
}

#[macro_export]
macro_rules! ensure_eq {
    ($l:expr, $r:expr) => {
        $crate::ensure!($l == $r)
    };
    ($l:expr, $r:expr, $arg:expr) => {
        $crate::ensure!($l == $r, $arg)
    };
}

#[macro_export]
macro_rules! ensure_en {
    ($l:expr, $r:expr) => {
        $crate::ensure!($l != $r)
    };
    ($l:expr, $r:expr, $arg:expr) => {
        $crate::ensure!($l != $r, $arg)
    };
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! fail{
    () => {
        {
            $crate::test_infrastructure::report_error("", file!(), line!(), colume!());
            panic!()
        }
    };
    ($($arg:tt), +) => {
        {
            let msg = format!($($arg),+);
            $crate::test_infrastructure::report_error(&msg, file!(), line!(), column!());
            panic!(msg)
        }
    };
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! fail{
    () => {
        {
            $crate::test_infrastructure::report_error("", file!(), line!(), colume!());
            panic!()
        }
    };
    ($($arg:tt), +) => {
        {
            let msg = &$crate::alloc::format!($($arg), +);
            $crate::test_infrastructure::report_error("", file!(), line!(), colume!());
            panic!(msg)
        }
    };
}

#[macro_export]
macro_rules! claim {
    ($cond:expr) => {
        if !$cond{
            $crate::fail!()
        }
    };
    ($cond:expr) => {
        if !$cond{
            $crate::fail!()
        }
    };
    ($cond:expr, $($arg:tt),+) => {
        if !$cond{
            $crate::fail!()
        }
    };
}

#[macro_export]
macro_rules! claim_eq {
    ($l:expr, $r:expr) => {
        $crate::claim!($l == $r, "left and right are not equal\nleft: {:?}\nright:{:?}", $left, $right)
    };
    ($l:expr, $r:expr,) => {
      $crate::claim_eq!($l, $r)
    };
    ($l:expr, $r:expr, $($arg:tt),+) => {
      $crate::claim!($l == $r, $($arg),+)
    };
}

#[macro_export]
macro_rules! claim_ne {
    ($left:expr, $right:expr) => {
        $crate::claim!($left != $right)
    };
    ($left:expr, $right:expr,) => {
        $crate::claim!($left != $right)
    };
    ($left:expr, $right:expr, $($arg:tt),+) => {
        $crate::claim!($left != $right, $($arg),+)
    };
}

pub type ReceiveResult<A> = Result<A, Reject>;

pub type InitResult<S> = Result<S, Reject>;

#[derive(Default)]
pub struct ExternContext<T: sealed::ContextType>{
    marker: crate::marker::PhantomData<T>,
}

pub struct ChainMetaExtern {}

#[derive(Default)]
pub struct InitContextExtern;

#[derive(Default)]
pub struct ReceiveContextExtern;

pub(crate) mod sealed {
    use super::*;

    pub trait ContextType{}
    impl ContextType for InitContextExtern {}
    impl ContextType for ReceiveContextExtern {}
}

