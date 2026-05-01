use crate::{object::Object, vm::{Frame, Vm, VmResult}};

impl Vm {
    #[inline(always)]
    pub fn call_func(&mut self, arg_count: i32) -> VmResult<()> {
        let base_ptr = self.sp - arg_count as usize - 1;
        let func = &self.stack[base_ptr];

        let Object::Func(func) = func else {
            return Err("calling on uncallable".to_owned());
        };

        let new_frame = Frame {
            func: func.borrow().clone(),
            ip: -1,
            locals: self.stack[base_ptr + 1..self.sp].to_vec(),
            base_ptr
        };

        self.push_frame(new_frame)?;

        Ok(())
    }
}
