use crate::{sprite::Sprite, thread::Thread};

#[derive(Clone)]
pub enum Instruction {
    MemoryStore,
    MemoryDump,
    ThreadKill,
    ThreadPause,
    OperatorModulo,
    OperatorAdd,
    OperatorSubtract,
    OperatorMultiply,
    OperatorDivide,
    OperatorLesser,
    OperatorGreater,
    FlowIfJump,
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Pointer(usize),
    Bytecode(Instruction),
}

impl Value {
    pub fn get_number(&self, memory: &[Value]) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Boolean(n) => {
                if *n {
                    1.0
                } else {
                    0.0
                }
            }
            Value::String(n) => n.parse().unwrap_or(0.0),
            Value::Bytecode(n) => {
                panic!()
            }
            Value::Pointer(n) => memory[*n].get_number(memory),
        }
    }

    pub fn get_bytecode(&self) -> Instruction {
        match self {
            Value::Bytecode(n) => n.clone(),
            _ => {
                panic!()
            }
        }
    }

    pub fn get_pointer(&self) -> usize {
        match self {
            Value::Pointer(n) => *n,
            _ => {
                panic!()
            }
        }
    }

    pub fn get_bool(&self, memory: &[Value]) -> bool {
        match self {
            Value::Boolean(n) => *n,
            Value::Number(_) => todo!(),
            Value::String(_) => todo!(),
            Value::Pointer(n) => memory[*n].get_bool(memory),
            _ => panic!(),
        }
    }
}
