use std::rc::Rc;

use crate::{
    object::Object,
    vm::{Vm, opcode::Op},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    pub instructions: Rc<[Op]>,
    pub constants: Vec<Object>,
}

pub type NativeFuncResult = crate::vm::VmResult<Option<crate::object::Object>>;

#[derive(Debug, Clone)]
pub struct NativeFunc(pub fn(&mut Vm, Vec<Object>) -> NativeFuncResult);

impl PartialEq for NativeFunc {
    fn eq(&self, other: &Self) -> bool {
        // 使用 ptr::fn_addr_eq 进行可靠的函数指针比较
        std::ptr::fn_addr_eq(self.0, other.0)
    }
}

impl Eq for NativeFunc {}