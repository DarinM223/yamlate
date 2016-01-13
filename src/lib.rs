#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(convert)]
#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(sorty)]
#![warn(unsorted_declarations)]

#[cfg_attr(feature="clippy", feature(plugin))]
#[cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate num;
extern crate yaml_rust;

mod ast;
mod errors;
mod evaluator;
mod helpers;
mod lexer;
mod parser;

pub mod environment;
pub mod ffi;
pub mod yaml;
