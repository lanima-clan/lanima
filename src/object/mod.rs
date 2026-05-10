pub mod func;
pub mod object_operators;

pub type GcObject = Rc<RefCell<Object>>;
pub type Gc<T> = Rc<RefCell<T>>;

use std::{cell::RefCell, fmt::Display, rc::Rc};

use bigdecimal::BigDecimal;
use num_traits::Zero;

use crate::object::func::{Func, NativeFunc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Object {
    I64(i64),
    Bool(bool),
    String(Gc<String>),
    Decimal(Gc<BigDecimal>),
    Func(Gc<Func>),
    NativeFunc(NativeFunc),
    Vec(Gc<Vec<Object>>),
    Null,
}

impl Object {
    pub fn type_name(&self) -> String {
        match self {
            Object::I64(_) => "i64".to_owned(),
            Object::Decimal(_) => "float".to_owned(),
            Object::String(_) => "String".to_owned(),
            Object::Func(_) => "Function".to_owned(),
            Object::NativeFunc(_) => "NativeFunction".to_owned(),
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
            Object::String(it) => !it.borrow().is_empty(),
            Object::Null => false,
            Object::NativeFunc(_it) => true,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(it) => write!(f, "{}", it.borrow()),
            Object::Bool(it) => write!(f, "{it}"),
            Object::I64(it) => write!(f, "{it}"),
            Object::Decimal(it) => write!(f, "{}", it.borrow()),
            Object::Func(func) => write!(
                f,
                "<function instructions: {:?} constants: {:?}>",
                func.borrow().instructions,
                func.borrow().constants
            ),
            Object::Vec(v) => write!(
                f,
                "[{}]",
                v.borrow()
                    .iter()
                    .map(|it| it.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Object::Null => write!(f, "null"),
            Object::NativeFunc(it) => write!(f, "<nativefunction ptr: {:?}>", it.0),
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
    if native_bool { TRUE_OBJ } else { FALSE_OBJ }
}

unsafe impl Send for Object {}
unsafe impl Sync for Object {}
