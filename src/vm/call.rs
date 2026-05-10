use crate::{
    object::Object,
    vm::{Frame, Vm, VmResult},
};

impl Vm {
    #[inline(always)]
    pub fn call_func(&mut self, arg_count: i32) -> VmResult<()> {
        let base_ptr = self.sp - arg_count as usize - 1;
        let func = &self.stack[base_ptr];

        if let Object::Func(func) = func {
            let new_frame = Frame {
                func: func.borrow().clone(),
                ip: -1,
                locals: self.stack[base_ptr + 1..self.sp].to_vec(),
                base_ptr,
            };

            self.push_frame(new_frame)?;
        } else if let Object::NativeFunc(func) = func {
            let result = (func.0)(self, self.stack[base_ptr + 1..self.sp].to_vec())?;

            self.sp = base_ptr;

            self.push(result.unwrap_or(Object::Null))?;
        } else {
            return Err(format!("calling on uncallable {}", func.type_name()));
        };

        Ok(())
    }
}
