/// Represents an error value returned
/// from a FFI function
pub enum Error {
    None = 0,
    WrongType = -1,
    NotDefined = -2,
    InvalidString = -3,
}

/// Represents the return value of a FFI function
/// includes the value and the error as an integer
#[repr(C)]
pub struct FFIReturnValue<T> {
    pub value: T,
    pub error: i32,
}

#[repr(C)]
pub struct FFIArrayReturnValue<T> {
    pub value: T,
    pub length: i32,
    pub error: i32,
}

/// Represents a YAML type for FFI
pub enum YamlType {
    Integer,
    Real,
    String,
    Boolean,
    Array,
    Hash,
    Null,
}
