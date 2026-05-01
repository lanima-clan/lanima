pub mod func;
pub mod object_operators;

pub type GcObject = Rc<RefCell<Object>>;
pub type Gc<T> = Rc<RefCell<T>>;

use std::{cell::RefCell, rc::Rc};

use bigdecimal::BigDecimal;
use num_traits::Zero;

use crate::object::func::Func;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Object {
    I64(i64),
    Bool(bool),
    Decimal(Gc<BigDecimal>),
    Func(Gc<Func>),
    Vec(Gc<Vec<Object>>),
    Null,
}

impl Object {
    pub fn type_name(&self) -> String {
        match self {
            Object::I64(_) => "i64".to_owned(),
            Object::Decimal(_) => "float".to_owned(),
            Object::Func(_) => "Function".to_owned(),
            Object::Vec(_) => "Vec".to_owned(),
            Object::Bool(_) => "bool".to_owned(),
            Object::Null => "null".to_owned(),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Bool(it) => *it,
            Object::I64(it) => *it != 0,
            Object::Decimal(it) => !it.borrow().is_zero(),
            Object::Func(func) => {
                !func.borrow().instructions.is_empty() && !func.borrow().constants.is_empty()
            }
            Object::Vec(v) => !v.borrow().is_empty(),
            Object::Null => false,
        }
    }
}

#[macro_export]
macro_rules! gc {
    ($o:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($o))
    };
}

pub const TRUE_OBJ: Object = Object::Bool(true);
pub const FALSE_OBJ: Object = Object::Bool(false);

pub fn native_bool_to_obj(native_bool: bool) -> Object {
    if native_bool {
        TRUE_OBJ
    } else {
        FALSE_OBJ
    }
}