pub mod func;

pub type GcObject = Rc<RefCell<Object>>;
pub type Gc<T> = Rc<RefCell<T>>;

use std::{cell::RefCell, rc::Rc};

use crate::object::func::Func;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Object {
    I64(i64),
    Func(Func),
    Vec(Gc<Vec<Object>>),
    Null
}

#[macro_export]
macro_rules! gc_obj {
    ($o:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($o))
    };
}