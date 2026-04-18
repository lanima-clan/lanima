pub mod err;
pub mod test;

pub type VmResult<T> = Result<T, String>;

pub const STACK_SIZE: usize = 2048;
pub const GLOBALS_SIZE: usize = 65535;

use crate::{
    object::{Object, func::Func},
    vm::opcode::{Op, OpKind},
};

pub mod opcode;

#[derive(Debug, Clone)]
pub struct Frame {
    pub func: Func,
    pub ip: isize,
    pub locals: Vec<Object>,
}

#[derive(Debug, Clone)]
pub struct Vm {
    constants: Vec<Object>,

    stack: Vec<Object>,
    sp: usize,

    frames: Vec<Frame>,
    frame_index: usize,
}

impl Vm {
    pub fn new(instructions: &[Op], constants: Vec<Object>) -> Self {
        let main_func = Func {
            instructions: instructions.into(),
        };

        Self {
            constants,

            stack: vec![Object::Null; STACK_SIZE],
            sp: 0,

            frames: vec![Frame {
                func: main_func,
                ip: -1,
                locals: vec![],
            }],
            frame_index: 1,
        }
    }
}

impl Vm {
    pub fn run(&mut self) -> VmResult<()> {
        while if self.current_frame_ref().ip >= 0 {
            self.current_frame_ref().ip as usize
        } else {
            0
        } < self.current_frame_ref().func.instructions.len() - 1
        {
            self.current_frame().ip += 1;

            let ip = self.current_frame_ref().ip as usize;
            let op = &self.current_frame_ref().func.instructions[ip];

            match op.kind {
                OpKind::Const => {
                    self.push(self.constants[op.operands[0] as usize].clone())?;
                }

                OpKind::Pop => {
                    if self.sp != 0 {
                        self.sp -= 1;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Vm {
    pub fn stack_top(&self) -> Option<Object> {
        if self.sp == 0 {
            None
        } else {
            Some(self.stack[self.sp - 1].clone())
        }
    }

    #[inline(always)]
    pub fn last_popped(&self) -> Option<Object> {
        self.stack.get(self.sp).cloned()
    }

    #[inline(always)]
    pub fn push(&mut self, obj: Object) -> VmResult<()> {
        if self.sp >= STACK_SIZE {
            return Err("Stack overflow".to_string());
        }

        self.stack[self.sp] = obj;

        self.sp += 1;

        Ok(())
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<Object> {
        if self.sp == 0 {
            return None;
        }

        let result = &self.stack[self.sp - 1];

        self.sp -= 1;

        Some(result.clone())
    }

    #[inline(always)]
    pub fn current_frame(&mut self) -> &mut Frame {
        &mut self.frames[self.frame_index - 1]
    }

    #[inline(always)]
    pub fn current_frame_ref(&self) -> &Frame {
        &self.frames[self.frame_index - 1]
    }
}
