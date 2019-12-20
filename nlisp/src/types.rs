use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub enum MalValue {
    Nil,
    True,
    False,
    String(String),
    Symbol(String),
    Int(i32),
    List(Rc<Vec<MalValue>>),
    Vector(Rc<Vec<MalValue>>),
    Function(fn(args: &[MalValue]) -> MalRet),
}

impl MalValue {

    pub fn as_int(self: &MalValue) -> MalResult<i32> {
        if let MalValue::Int(i) = self {
            Ok(*i)
        } else {
            Err(MalError::new("Expected integer"))
        }
    }
}



pub type MalEnv = HashMap<String, MalValue>;

pub struct MalError {
    pub s: String,
}

impl MalError {
    pub fn new(s: &str) -> MalError {
        MalError { s: s.to_string() }
    }
}

pub type MalResult<T> = Result<T, MalError>;
pub type MalRet = Result<MalValue, MalError>;
