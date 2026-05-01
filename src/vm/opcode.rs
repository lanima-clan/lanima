use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpKind {
    Const,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Gt,
    Call,
    Return,
    ReturnValue,
    JumpNotTruthy,
    Jump,
    GetLocal,
    CurrentFunc,
    Pop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub kind: OpKind,
    pub operands: Rc<[i32]>,
}

pub fn make(kind: OpKind, operands: &[i32]) -> Op {
    Op {
        kind,
        operands: operands.into(),
    }
}

pub fn fmt_instructions(instructions: &[Op]) -> String {
    let mut l = vec![];

    for ins in instructions {
        let operand_str = ins
            .operands
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        l.push(format!(
            "{:#?}{}",
            ins.kind,
            if ins.operands.is_empty() {
                "".to_owned()
            } else {
                " ".to_owned() + &operand_str
            }
        ));
    }

    l.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::vm::opcode::{OpKind, fmt_instructions, make};

    #[test]
    fn test_fmt_instructions() {
        let instructions = &[
            make(OpKind::Const, &[0]),
            make(OpKind::Pop, &[]),
            make(OpKind::Add, &[]),
            make(OpKind::Sub, &[]),
            make(OpKind::Div, &[]),
            make(OpKind::Mul, &[]),
            make(OpKind::ReturnValue, &[]),
            make(OpKind::Return, &[]),
            make(OpKind::Jump, &[]),
            make(OpKind::JumpNotTruthy, &[]),
            make(OpKind::GetLocal, &[]),
            make(OpKind::Eq, &[]),
            make(OpKind::NotEq, &[]),
            make(OpKind::Gt, &[]),
            make(OpKind::CurrentFunc, &[]),
        ];

        let expected = [
            "Const 0",
            "Pop",
            "Add",
            "Sub",
            "Div",
            "Mul",
            "ReturnValue",
            "Return",
            "Jump",
            "JumpNotTruthy",
            "GetLocal",
            "Eq",
            "NotEq",
            "Gt",
            "CurrentFunc"
        ].join("\n");

        assert_eq!(fmt_instructions(instructions), expected);
    }
}
