use std::{
    fmt::{self},
    ops::Neg,
    rc::Rc,
    u8,
};

#[repr(u8)]
pub enum OpCode {
    OpR,
    OpC,
    OpPop,
    OpDefGlobal,
    OpGetGlobal,
    OpSetGlobal,
    OpSetLocal,
    OpGetLocal,

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

    //statements
    OpPrint,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
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
