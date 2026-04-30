#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    use crate::{
        gc,
        object::Object,
        vm::{
            Vm,
            opcode::{Op, OpKind, make},
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
        assert_eq!(vm.last_popped().unwrap(), expected_obj)
    }

    #[test]
    fn test_add_sub_mul_div() {
        let constants = vec![
            Object::I64(10),
            Object::I64(5),
        ];

        let test_cases = vec![
            (
                vec![
                    make(OpKind::Const, &[0]),
                    make(OpKind::Const, &[1]),
                    make(OpKind::Add, &[]),
                    make(OpKind::Pop, &[]),
                ],
                Object::I64(15),
            ),
            (
                vec![
                    make(OpKind::Const, &[0]),
                    make(OpKind::Const, &[1]),
                    make(OpKind::Sub, &[]),
                    make(OpKind::Pop, &[]),
                ],
                Object::I64(5),
            ),
            (
                vec![
                    make(OpKind::Const, &[0]),
                    make(OpKind::Const, &[1]),
                    make(OpKind::Mul, &[]),
                    make(OpKind::Pop, &[]),
                ],
                Object::I64(50),
            ),
            (
                vec![
                    make(OpKind::Const, &[0]),
                    make(OpKind::Const, &[1]),
                    make(OpKind::Div, &[]),
                    make(OpKind::Pop, &[]),
                ],
                Object::Decimal(gc!(BigDecimal::from(2))),
            ),
        ];

        for (ins, expected) in test_cases {
            test_expected(&ins, constants.clone(), expected);
        }
    }

    fn test_expected(instructions: &[Op], constants: Vec<Object>, expected: Object) {
        let mut vm = Vm::new(&instructions, constants);
        vm.run().unwrap();

        assert!(vm.last_popped().is_some());
        assert_eq!(vm.last_popped().unwrap(), expected);
    }
}
