//! Compile a Python AST or source code into bytecode consumable by RustPython or
//! (eventually) CPython.
#![doc(html_logo_url = "https://raw.githubusercontent.com/RustPython/RustPython/master/logo.png")]
#![doc(html_root_url = "https://docs.rs/rustpython-compiler/")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
extern crate log;

pub mod compile;
pub mod error;
pub mod mode;
pub mod symboltable;
