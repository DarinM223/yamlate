/// Represents an error value returned
/// from a FFI function
pub enum Error {
    None = 0,
    WrongType = -1,
    NotDefined = -2,
}

/// Represents the return value of a FFI function
/// includes the value and the error as an integer
#[repr(C)]
pub struct FFIReturnValue<T> {
    pub value: T,
    pub error: i32,
}