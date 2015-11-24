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
mod yaml_wrapper;
mod ffi_types;

pub mod environment_ffi;

