pub mod object_operators;
pub mod func;

pub type GcObject = Rc<RefCell<Object>>;
pub type Gc<T> = Rc<RefCell<T>>;

use std::{cell::RefCell, rc::Rc};

use bigdecimal::BigDecimal;

use crate::object::func::Func;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Object {
    I64(i64),
    Decimal(Gc<BigDecimal>),
    Func(Func),
    Vec(Gc<Vec<Object>>),
    Null
}

impl Object {
    pub fn type_name(&self) -> String {
        match self {
            Object::I64(_) => "i64".to_owned(),
            Object::Decimal(_) => "float".to_owned(),
            Object::Func(_) => "Function".to_owned(),
            Object::Vec(_) => "Vec".to_owned(),
            Object::Null => "null".to_owned(),
        }
    }
}

#[macro_export]
macro_rules! gc {
    ($o:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($o))
    };
}