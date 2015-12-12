#![feature(convert)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(custom_attribute)]

#![feature(plugin)]
#![plugin(clippy)]

extern crate yaml_rust;
extern crate libc;

#[macro_use]
extern crate lazy_static;

mod errors;
mod helpers;
mod ast;
mod parser;
mod token_builder;
mod lexer;
pub mod environment;
mod appliers;
mod evaluator;
pub mod yaml;
mod ffi_types;

pub mod environment_ffi;
pub mod yaml_ffi;
