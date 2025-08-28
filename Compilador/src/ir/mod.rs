#[derive(Debug, Clone)]
pub enum IRValue {
    Const(i64),
    Local(String),
    Global(String),
    Temp(String),
}

#[derive(Debug, Clone)]
pub enum IROp {
    Add(IRValue, IRValue, IRValue),      // result = left + right
    Sub(IRValue, IRValue, IRValue),      // result = left - right
    Mul(IRValue, IRValue, IRValue),      // result = left * right
    Div(IRValue, IRValue, IRValue),      // result = left / right
    CmpEq(IRValue, IRValue, IRValue),    // result = left == right
    CmpLt(IRValue, IRValue, IRValue),    // result = left < right
    Assign(IRValue, IRValue),            // target = source
    Call(String, Vec<IRValue>, Option<IRValue>), // call func(args) -> result
    Label(String),                       // label:
    Jump(String),                        // jmp label
    JumpIfZero(IRValue, String),         // jz value, label
    JumpIfNotZero(IRValue, String),      // jnz value, label
    Return(Option<IRValue>),             // return value
    Print(IRValue),                      // print value
    Alloc(String, usize),                // alloc array
    ArraySet(IRValue, IRValue, IRValue), // array[index] = value
    ArrayGet(IRValue, IRValue, IRValue), // value = array[index]
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<String>,
    pub instructions: Vec<IROp>,
    pub locals: std::collections::HashMap<String, IRValue>,
}

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
    pub globals: std::collections::HashMap<String, IRValue>,
}
