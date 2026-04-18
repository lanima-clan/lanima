#[cfg(test)]
mod tests {
    use crate::{
        object::Object,
        vm::{
            Vm,
            opcode::{OpKind, make},
        },
    };

    #[test]
    fn simple_program() {
        let instructions = vec![make(OpKind::Const, &[0]), make(OpKind::Pop, &[])];

        let expected_obj = Object::I64(42);

        let constants = vec![expected_obj.clone()];

        let mut vm = Vm::new(&instructions, constants);
        vm.run().unwrap();

        assert!(vm.last_popped().is_some());
        assert_eq!(
            vm.last_popped().unwrap(),
            expected_obj
        )
    }
}
