use std::rc::Rc;

use crate::vm::opcode::Op;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    pub instructions: Rc<[Op]>
}