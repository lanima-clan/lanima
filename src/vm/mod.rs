pub mod call;
pub mod err;
pub mod native_functions;
pub mod test;

const NO_OBJECT_HERE: &str = "no object here";

pub type VmResult<T> = Result<T, String>;

pub const STACK_SIZE: usize = 2048;
pub const GLOBALS_SIZE: usize = 65535;

use crate::{
    gc,
    object::{
        Object,
        func::Func,
        object_operators::{ObjAdd, ObjDiv, ObjEq, ObjGt, ObjMul, ObjNotEq, ObjSub},
    },
    vm::{
        native_functions::{BUILTIN_INDEX_TO_NAME, BUILTIN_MAP},
        opcode::{Op, OpKind},
    },
};

pub mod opcode;

#[derive(Debug, Clone)]
pub struct Frame {
    pub func: Func,
    pub ip: isize,
    pub locals: Vec<Object>,
    pub base_ptr: usize,
}

#[derive(Debug, Clone)]
pub struct Vm {
    stack: Vec<Object>,
    sp: usize,

    frames: Vec<Frame>,
    frame_index: usize,
}

impl Vm {
    pub fn new(instructions: &[Op], constants: Vec<Object>) -> Self {
        let main_func = Func {
            instructions: instructions.into(),
            constants,
        };

        Self {
            stack: vec![Object::Null; STACK_SIZE],
            sp: 0,

            frames: vec![Frame {
                func: main_func,
                ip: -1,
                locals: vec![],
                base_ptr: 0,
            }],
            frame_index: 1,
        }
    }
}

impl Vm {
    fn obj_operator(&mut self, op: OpKind) -> VmResult<()> {
        if ![
            OpKind::Add,
            OpKind::Div,
            OpKind::Sub,
            OpKind::Mul,
            OpKind::Eq,
            OpKind::NotEq,
            OpKind::Gt,
        ]
        .contains(&op)
        {
            return Ok(());
        }

        let right_obj = self
            .pop()
            .map_or_else(|| VmResult::Err(NO_OBJECT_HERE.to_owned()), |it| Ok(it))?;

        let left_obj = self
            .pop()
            .map_or_else(|| VmResult::Err(NO_OBJECT_HERE.to_owned()), |it| Ok(it))?;

        let op_functions = [
            Object::obj_add,
            Object::obj_sub,
            Object::obj_mul,
            Object::obj_div,
            Object::obj_eq,
            Object::obj_not_eq,
            Object::obj_gt,
        ];

        let op_func = op_functions[op as usize - 1];
        let o = op_func(&left_obj, &right_obj).map_or_else(
            || {
                Err(format!(
                    "invaild operator to {} and {}",
                    left_obj.type_name(),
                    right_obj.type_name(),
                ))
            },
            |it| Ok(it),
        )?;

        self.push(o)?;

        Ok(())
    }

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
            let constants = &self.current_frame_ref().func.constants;

            match op.kind {
                OpKind::Const => {
                    self.push(constants[op.operands[0] as usize].clone())?;
                }

                OpKind::CurrentFunc => {
                    self.push(Object::Func(gc!(self.current_frame_ref().func.clone())))?
                }

                OpKind::Call => self.call_func(op.operands[0])?,

                OpKind::ReturnValue => {
                    let ret_value = self
                        .pop()
                        .map_or_else(|| Err(NO_OBJECT_HERE), |it| Ok(it))?;

                    let frame = self
                        .pop_frame()
                        .map_or_else(|| Err("return outside function"), |it| Ok(it))?;

                    self.sp = frame.base_ptr;

                    self.push(ret_value)?;
                }

                OpKind::Return => {
                    let frame = self
                        .pop_frame()
                        .map_or_else(|| Err("return outside function"), |it| Ok(it))?;

                    self.sp = frame.base_ptr;

                    self.push(Object::Null)?;
                }

                OpKind::Jump => {
                    self.current_frame().ip = op.operands[0] as isize;
                }

                OpKind::JumpNotTruthy => {
                    let target_ip = op.operands[0] as isize;

                    let cond_obj = self
                        .pop()
                        .map_or_else(|| VmResult::Err(NO_OBJECT_HERE.to_owned()), |it| Ok(it))?;

                    if !cond_obj.is_truthy() {
                        self.current_frame().ip = target_ip;
                    }
                }

                OpKind::GetLocal => {
                    self.push(self.current_frame_ref().locals[op.operands[0] as usize].clone())?
                }

                OpKind::GetBuiltin => {
                    self.push(BUILTIN_MAP[BUILTIN_INDEX_TO_NAME[op.operands[0] as usize]].clone())?
                }

                OpKind::Add => self.obj_operator(op.kind)?,
                OpKind::Sub => self.obj_operator(op.kind)?,
                OpKind::Mul => self.obj_operator(op.kind)?,
                OpKind::Div => self.obj_operator(op.kind)?,
                OpKind::Eq => self.obj_operator(op.kind)?,
                OpKind::NotEq => self.obj_operator(op.kind)?,
                OpKind::Gt => self.obj_operator(op.kind)?,

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
    pub fn push_frame(&mut self, frame: Frame) -> VmResult<()> {
        if self.frames.len() <= self.frame_index {
            self.frames.push(frame);
        } else {
            self.frames[self.frame_index] = frame;
        }

        self.frame_index += 1;

        Ok(())
    }

    #[inline(always)]
    pub fn pop_frame(&mut self) -> Option<Frame> {
        // 你不能把主函数 pop 出来
        if self.frame_index <= 1 {
            return None;
        }

        let result = &self.frames[self.frame_index - 1];

        self.frame_index -= 1;

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
