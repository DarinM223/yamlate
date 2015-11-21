use ast::AST;
use environment::{IEnvironment, Environment};
use std::mem::transmute;
use std::ffi::{CStr, CString};
use libc::c_char;

// Error code for no error in result
const ERROR_NONE: i32 = 0;
// Error code for error based on wrong type
const ERROR_WRONGTYPE: i32 = -1;
// Error code for error based on variable not defined
const ERROR_NOTDEFINED: i32 = -2;

#[repr(C)]
pub struct FFIReturnValue<T> {
    value: T,
    error: i32,
}

#[no_mangle]
pub extern "C" fn environment_create() -> *mut Environment {
    let environment = unsafe { transmute(box Environment::new()) };
    environment
}

#[no_mangle]
pub extern "C" fn environment_set_integer(env: *mut Environment, name: *const c_char, value: i32) {
    let mut environment = unsafe { &mut *env };
    let mut key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    environment.set(key, AST::Number(value));
}

#[no_mangle]
pub extern "C" fn environment_set_string(env: *mut Environment,
                                         name: *const c_char,
                                         value: *const c_char) {
    let mut environment = unsafe { &mut *env };
    let mut key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let mut val: String = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

    environment.set(key, AST::String(val));
}

#[no_mangle]
pub extern "C" fn environment_get_integer(env: *mut Environment,
                                          name: *const c_char)
                                          -> FFIReturnValue<i32> {
    let mut environment = unsafe { &mut *env };
    let mut key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    match environment.get(key) {
        Some(&AST::Number(val)) => FFIReturnValue {
            value: val,
            error: ERROR_NONE,
        },
        None => FFIReturnValue {
            value: 0,
            error: ERROR_NOTDEFINED,
        },
        _ => FFIReturnValue {
            value: 0,
            error: ERROR_WRONGTYPE,
        },
    }
}

#[no_mangle]
pub extern "C" fn environment_get_string(env: *mut Environment,
                                         name: *const c_char)
                                         -> FFIReturnValue<*const c_char> {
    let mut environment = unsafe { &mut *env };
    let mut key: String = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    match environment.get(key) {
        Some(&AST::String(ref val)) => FFIReturnValue {
            value: CString::new(val.as_str()).unwrap().as_ptr(),
            error: ERROR_NONE,
        },
        None => FFIReturnValue {
            value: CString::new("").unwrap().as_ptr(),
            error: ERROR_NOTDEFINED,
        },
        _ => FFIReturnValue {
            value: CString::new("").unwrap().as_ptr(),
            error: ERROR_WRONGTYPE,
        },
    }
}

// TODO: causes segmentation fault
#[no_mangle]
pub extern "C" fn environment_push(env: *mut Environment) {
    let mut environment = unsafe { &mut *env };
    environment.push();
}

// TODO: causes segmentation fault
#[no_mangle]
pub extern "C" fn environment_pop(env: *mut Environment) {
    let mut environment = unsafe { &mut *env };
    environment.pop();
}

#[no_mangle]
pub extern "C" fn environment_destroy(env: *mut Environment) {
    let environment: Box<Environment> = unsafe { transmute(env) };
}
