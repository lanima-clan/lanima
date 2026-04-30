use std::cmp::Ordering;

use bigdecimal::BigDecimal;

use crate::{gc, object::Object};

// 基本定义:
// 顺序枚举 加减乘除比较 Trait

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjOrdering {
    /// An ordering where a compared value is less than another.
    Less = -1,
    /// An ordering where a compared value is equal to another.
    Equal = 0,
    /// An ordering where a compared value is greater than another.
    Greater = 1,
}

impl Into<ObjOrdering> for Ordering {
    fn into(self) -> ObjOrdering {
        match self {
            Self::Less => ObjOrdering::Less,
            Self::Equal => ObjOrdering::Equal,
            Self::Greater => ObjOrdering::Greater,
        }
    }
}

pub trait ObjAdd {
    type Output;

    fn obj_add(&self, right_obj: &Object) -> Self::Output;
}

pub trait ObjSub {
    type Output;

    fn obj_sub(&self, right_obj: &Object) -> Self::Output;
}

pub trait ObjMul {
    type Output;

    fn obj_mul(&self, right_obj: &Object) -> Self::Output;
}

pub trait ObjDiv {
    type Output;

    fn obj_div(&self, right_obj: &Object) -> Self::Output;
}

pub trait ObjCmp {
    type Output: Into<Option<ObjOrdering>>;

    fn obj_cmp(&self, right_obj: &Object) -> Self::Output;
}

// 为各种类型实现 加减乘除比较

impl ObjAdd for Object {
    type Output = Option<Object>;

    fn obj_add(&self, right_obj: &Object) -> Self::Output {
        match (self, right_obj) {
            (Object::I64(l), Object::I64(r)) => Some(Object::I64(l + r)),
            _ => None,
        }
    }
}

impl ObjSub for Object {
    type Output = Option<Object>;

    fn obj_sub(&self, right_obj: &Object) -> Self::Output {
        match (self, right_obj) {
            (Object::I64(l), Object::I64(r)) => Some(Object::I64(l - r)),
            _ => None,
        }
    }
}

impl ObjMul for Object {
    type Output = Option<Object>;

    fn obj_mul(&self, right_obj: &Object) -> Self::Output {
        match (self, right_obj) {
            (Object::I64(l), Object::I64(r)) => Some(Object::I64(l * r)),
            _ => None,
        }
    }
}

impl ObjDiv for Object {
    type Output = Option<Object>;

    fn obj_div(&self, right_obj: &Object) -> Self::Output {
        match (self, right_obj) {
            (Object::I64(l), Object::I64(r)) => {
                if *r != 0 {
                    Some(Object::Decimal(gc!(
                        BigDecimal::from(l) / BigDecimal::from(r)
                    )))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl ObjCmp for Object {
    type Output = Option<ObjOrdering>;

    fn obj_cmp(&self, right_obj: &Object) -> Self::Output {
        match (self, right_obj) {
            (Object::I64(l), Object::I64(r)) => Some(l.cmp(r).into()),
            (Object::Func(l), Object::Func(r)) => {
                if l == r {
                    Some(ObjOrdering::Equal)
                } else {
                    None
                }
            }
            (Object::Vec(l), Object::Vec(r)) => {
                if l == r {
                    Some(ObjOrdering::Equal)
                } else {
                    None
                }
            }
            (Object::Null, Object::Null) => Some(ObjOrdering::Equal),

            _ => None,
        }
    }
}

pub trait AllOperator: ObjAdd + ObjSub + ObjMul + ObjDiv + ObjCmp {}

impl AllOperator for Object {}