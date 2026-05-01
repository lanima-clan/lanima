use std::rc::Rc;

use crate::{object::Object, vm::opcode::Op};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    pub instructions: Rc<[Op]>,
    pub constants: Vec<Object>
}