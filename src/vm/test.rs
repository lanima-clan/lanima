#[cfg(test)]
mod tests {
    use std::time::Instant;

    use bigdecimal::BigDecimal;

    use crate::{
        gc,
        object::{Object, func::Func},
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
    fn test_simplest_function() {
        let instructions = vec![
            make(OpKind::Const, &[0]),
            make(OpKind::Call, &[0]),
            make(OpKind::Pop, &[]),
        ];

        let func_instructions = vec![make(OpKind::Return, &[])];

        let constants = vec![Object::Func(gc!(Func {
            instructions: func_instructions.into(),
            constants: vec![]
        }))];

        test_expected(&instructions, constants, Object::Null);
    }

    #[test]
    fn test_print_function() {
        let instructions = vec![
            make(OpKind::GetBuiltin, &[0]),
            make(OpKind::Const, &[0]),
            make(OpKind::Call, &[1]),
            make(OpKind::Pop, &[]),
        ];

        let constants = vec![Object::I64(42)];

        test_expected(&instructions, constants, Object::Null);
    }

    #[test]
    fn test_fib_function() {
        let instructions = vec![
            make(OpKind::Const, &[0]), // func fib
            make(OpKind::Const, &[1]), // n
            make(OpKind::Call, &[1]),
            make(OpKind::Pop, &[]),
        ];

        let func_instructions = vec![
            // 3 > n
            make(OpKind::Const, &[0]),
            make(OpKind::GetLocal, &[0]),
            make(OpKind::Gt, &[]),
            make(OpKind::JumpNotTruthy, &[7]),
            // 3 > n 的情况
            make(OpKind::Const, &[1]), // 3 > n: 1
            make(OpKind::ReturnValue, &[]),
            // fib(n - 1) + fib(n - 2)
            make(OpKind::CurrentFunc, &[]),
            make(OpKind::GetLocal, &[1]),
            make(OpKind::Const, &[1]),
            make(OpKind::Sub, &[]),
            make(OpKind::Call, &[1]),
            make(OpKind::CurrentFunc, &[]),
            make(OpKind::GetLocal, &[0]),
            make(OpKind::Const, &[2]),
            make(OpKind::Sub, &[]),
            make(OpKind::Call, &[1]),
            make(OpKind::Add, &[]),
            make(OpKind::ReturnValue, &[]),
        ];

        let constants = vec![
            Object::Func(gc!(Func {
                instructions: func_instructions.into(),
                constants: vec![Object::I64(3), Object::I64(1), Object::I64(2),],
            })),
            Object::I64(35),
            Object::I64(3),
        ];

        let timer = Instant::now();
        test_expected(&instructions, constants, Object::I64(9227465));
        println!("{:#?}", timer.elapsed());
    }

    #[test]
    fn test_operators() {
        let constants = vec![Object::I64(10), Object::I64(5)];

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
            (vec![make(OpKind::Null, &[])], Object::Null),
            (
                vec![
                    make(OpKind::Const, &[0]),
                    make(OpKind::Const, &[1]),
                    make(OpKind::Null, &[]),
                    make(OpKind::BuildArray, &[3]),
                    make(OpKind::Pop, &[]),
                ],
                Object::Vec(gc!(vec![Object::I64(10), Object::I64(5), Object::Null])),
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
