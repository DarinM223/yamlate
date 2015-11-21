#![feature(convert)]
#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate yaml_rust;
extern crate libc;

#[macro_use]
extern crate lazy_static;

mod helpers;
mod ast;
mod parser;
mod lexer;
mod environment;
mod evaluator;

use ast::AST;
use environment::{IEnvironment, Environment};
use std::mem::transmute;
use std::ffi::CStr;
use libc::c_char;

#[no_mangle]
pub extern fn environment_create() -> *mut Environment {
    let _environment = unsafe { transmute(box Environment::new()) };
    _environment
}

#[no_mangle]
pub extern fn environment_set_integer(env: *mut Environment, key: *const c_char, value: i32) {
    let mut _environment = unsafe { &mut *env };
    let mut _key: String = unsafe { CStr::from_ptr(key).to_string_lossy().into_owned() };

    _environment.set(_key, AST::Number(value));
}

#[no_mangle]
pub extern fn environment_get_integer(env: *mut Environment, key: *const c_char) -> i32 {
    let mut _environment = unsafe { &mut *env };
    let mut _key: String = unsafe { CStr::from_ptr(key).to_string_lossy().into_owned() };

    match _environment.get(_key) {
        Some(&AST::Number(val)) => val,
        _ => -1,
    }
}

#[no_mangle]
pub extern fn environment_destroy(env: *mut Environment) {
    let _environment: Box<Environment> = unsafe { transmute(env) };
}
