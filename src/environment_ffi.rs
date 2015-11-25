use ast::AST;
use environment::{IEnvironment, Environment};
use std::mem::transmute;
use std::ffi::{CStr, CString};
use ffi_types::{FFIReturnValue, Error};
use libc::c_char;

#[no_mangle]
pub extern "C" fn environment_create() -> *mut Environment {
    let environment = unsafe { transmute(box Environment::new()) };
    environment
}

#[no_mangle]
pub extern "C" fn environment_set_integer(env: *mut Environment, name: *const c_char, value: i32) {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    environment.set(key, AST::Number(value));
}

#[no_mangle]
pub extern "C" fn environment_set_string(env: *mut Environment,
                                         name: *const c_char,
                                         value: *const c_char) {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let val: String = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    environment.set(key, AST::String(val));
}

#[no_mangle]
pub extern "C" fn environment_set_decimal(env: *mut Environment, name: *const c_char, value: f64) {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    environment.set(key, AST::Decimal(value));
}

#[no_mangle]
pub extern "C" fn environment_get_integer(env: *mut Environment,
                                          name: *const c_char)
                                          -> FFIReturnValue<i32> {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    match environment.get(key) {
        Some(&AST::Number(val)) => FFIReturnValue {
            value: val,
            error: Error::None as i32,
        },
        None => FFIReturnValue {
            value: 0,
            error: Error::NotDefined as i32,
        },
        _ => FFIReturnValue {
            value: 0,
            error: Error::WrongType as i32,
        },
    }
}

#[no_mangle]
pub extern "C" fn environment_get_string(env: *mut Environment,
                                         name: *const c_char)
                                         -> FFIReturnValue<*const c_char> {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    match environment.get(key) {
        Some(&AST::String(ref val)) => FFIReturnValue {
            value: CString::new(val.as_str()).unwrap().as_ptr(),
            error: Error::None as i32,
        },
        None => FFIReturnValue {
            value: CString::new("").unwrap().as_ptr(),
            error: Error::NotDefined as i32,
        },
        _ => FFIReturnValue {
            value: CString::new("").unwrap().as_ptr(),
            error: Error::WrongType as i32,
        },
    }
}

#[no_mangle]
pub extern "C" fn environment_get_decimal(env: *mut Environment,
                                          name: *const c_char)
                                          -> FFIReturnValue<f64> {
    let environment = unsafe { &mut *env };
    let key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    match environment.get(key) {
        Some(&AST::Decimal(val)) => FFIReturnValue {
            value: val,
            error: Error::None as i32,
        },
        None => FFIReturnValue {
            value: 0.0,
            error: Error::NotDefined as i32,
        },
        _ => FFIReturnValue {
            value: 0.0,
            error: Error::WrongType as i32,
        },
    }
}

#[no_mangle]
pub extern "C" fn environment_destroy(env: *mut Environment) {
    let environment: Box<Environment> = unsafe { transmute(env) };
}