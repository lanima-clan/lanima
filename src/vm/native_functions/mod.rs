pub mod funcs;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{object::{Object, func::NativeFunc}, vm::native_functions::funcs::builtin_print};

pub const BUILTIN_INDEX_TO_NAME: [&str; 1] = [
    "print"
];

pub static BUILTIN_MAP: Lazy<HashMap<&str, Object>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("print", Object::NativeFunc(NativeFunc(builtin_print)));

    m
});