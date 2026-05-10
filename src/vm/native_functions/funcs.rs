use crate::{object::{Object, func::NativeFuncResult}, vm::Vm};

pub fn builtin_print(_vm: &mut Vm, args: Vec<Object>) -> NativeFuncResult {
    let o = &args[0];

    println!("{o}");

    Ok(None)
}