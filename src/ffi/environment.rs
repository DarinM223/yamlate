use crate::ast::Lit;
use crate::environment::{ASTEnvironment, Environment};
use crate::ffi::types::{Error, FFIReturnValue};
use libc::c_char;
use std::ffi::{CStr, CString};

#[unsafe(no_mangle)]
pub extern "C" fn environment_create() -> *mut ASTEnvironment {
    Box::into_raw(Box::new(ASTEnvironment::new()))
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn environment_set_integer(
    env: *mut ASTEnvironment,
    name: *const c_char,
    value: i32,
) {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    environment.set(key.as_str(), Lit::Number(value));
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn environment_set_string(
    env: *mut ASTEnvironment,
    name: *const c_char,
    value: *const c_char,
) {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let val: String = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    environment.set(key.as_str(), Lit::Str(val));
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn environment_set_decimal(
    env: *mut ASTEnvironment,
    name: *const c_char,
    value: f64,
) {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    environment.set(key.as_str(), Lit::Decimal(value));
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn environment_get_integer(
    env: *mut ASTEnvironment,
    name: *const c_char,
) -> FFIReturnValue<i32> {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    match environment.get(key.as_str()) {
        Some(Lit::Number(val)) => FFIReturnValue {
            value: val,
            error: Error::None as i32,
        },
        Some(_) => FFIReturnValue {
            value: 0,
            error: Error::WrongType as i32,
        },
        None => FFIReturnValue {
            value: 0,
            error: Error::NotDefined as i32,
        },
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn environment_get_string(
    env: *mut ASTEnvironment,
    name: *const c_char,
) -> FFIReturnValue<*const c_char> {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    match environment.get(key.as_str()) {
        Some(Lit::Str(ref val)) => {
            let c_str = CString::new(val.clone().as_str()).unwrap().into_raw();

            FFIReturnValue {
                value: c_str as *const c_char,
                error: Error::None as i32,
            }
        }
        Some(_) => FFIReturnValue {
            value: CString::new("").unwrap().into_raw() as *const c_char,
            error: Error::WrongType as i32,
        },
        None => FFIReturnValue {
            value: CString::new("").unwrap().into_raw() as *const c_char,
            error: Error::NotDefined as i32,
        },
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn environment_get_decimal(
    env: *mut ASTEnvironment,
    name: *const c_char,
) -> FFIReturnValue<f64> {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    match environment.get(key.as_str()) {
        Some(Lit::Decimal(val)) => FFIReturnValue {
            value: val,
            error: Error::None as i32,
        },
        Some(_) => FFIReturnValue {
            value: 0.0,
            error: Error::WrongType as i32,
        },
        None => FFIReturnValue {
            value: 0.0,
            error: Error::NotDefined as i32,
        },
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn environment_destroy(env: *mut ASTEnvironment) {
    drop(unsafe { Box::from_raw(env) })
}
