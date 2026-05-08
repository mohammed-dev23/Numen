use std::{
    fmt::{self},
    ops::Neg,
    rc::Rc,
    u16,
};

#[repr(u16)]
pub enum OpCode {
    OpR,
    OpC,
    OpPop,
    OpDefGlobal,
    OpGetGlobal,
    OpSetGlobal,
    OpSetLocal,
    OpGetLocal,
    OpDefFixed,
    OpSetLocalFixed,
    OpLoop,

    //values for UnaryOp
    OpNegate,
    OpSqrt,
    OpAbs,
    OpFloor,
    OpCeil,
    OpSin,
    OpCos,
    OpTan,
    OpNot,

    //values for BinaryOp
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpMod,
    OpPow,
    OpDivideDivide,

    //values for ComparisonOp
    OpEqEq,
    OpEq,
    OpNotEq,
    OpLt,
    OpGt,
    OpGte,
    OpLte,

    //values for  LogicalOp
    OpAnd,
    OpOr,

    //statements
    OpPrint,

    //controlflow OpCodes
    OpJumpIfFalse,
    OpJump,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u16>,
    pub constant: ValueArray,
    pub line: Vec<usize>,
}

#[derive(Debug, Clone)]
pub enum Values {
    Float(f64),
    Int(i64),
    Bool(bool),
    Str(Rc<str>),
    None,
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(fnum) => write!(f, "{}", fnum),
            _ => Err(fmt::Error::default()),
        }
    }
}

impl Neg for Values {
    type Output = Values;

    fn neg(self) -> Self::Output {
        match self {
            Values::Float(d) => Values::Float(-d),
            Values::Int(d) => Values::Int(-d),
            Values::Bool(b) => Values::Bool(b),
            _ => Values::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueArray {
    pub values: Vec<Values>,
}

impl ValueArray {
    pub fn new_value() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write_value(&mut self, values: Values) {
        self.values.push(values);
    }
}

#[allow(warnings)]
impl Values {
    pub fn bool_val(bool: bool) -> Self {
        Self::Bool(bool)
    }

    pub fn int_val(int: i64) -> Self {
        Self::Int(int)
    }

    pub fn float_val(float: f64) -> Self {
        Self::Float(float)
    }

    pub fn none() -> Self {
        Self::None
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Values::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Values::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Values::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn is_int(&self) -> bool {
        self.as_int().is_some()
    }

    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    pub fn is_false(&self) -> bool {
        match self {
            Values::Bool(b) => !b,
            Values::None => true,
            Values::Int(0) => true,
            Values::Float(f) => *f == 0.0,
            _ => false,
        }
    }
}
