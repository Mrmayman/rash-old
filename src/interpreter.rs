use std::collections::HashMap;

#[derive(Clone)]
pub enum Instruction {
    MemoryStore(Value, Value),
    MemoryDump,
    ThreadKill,
    ThreadPause,
    OperatorModulo(Value, Value, Value),
    OperatorAdd(Value, Value, Value),
    OperatorSubtract(Value, Value, Value),
    OperatorMultiply(Value, Value, Value),
    OperatorDivide(Value, Value, Value),
    OperatorLesser(Value, Value, Value),
    OperatorGreater(Value, Value, Value),
    OperatorEquals(Value, Value, Value),
    FlowIfJump(Value, Value),
    FlowIfJumpToPlace(Value, String),
    FlowDefinePlace(String),
    FlowIfNotJump(Value, Value),
    FlowIfNotJumpToPlace(Value, String),
    MotionChangeX(Value),
    MotionChangeY(Value),
    MotionSetX(Value),
    MotionSetY(Value),
    LooksSetSize(Value),
}

impl Instruction {
    pub fn print(&self, variables: Option<&HashMap<String, usize>>) -> String {
        match &self {
            Instruction::MemoryStore(at, n) => {
                format!("{} = {};", get_var(variables, at), n.print(variables))
            }
            Instruction::MemoryDump => format!("std::dumpMemory();"),
            Instruction::ThreadKill => format!("return;"),
            Instruction::ThreadPause => format!("std::renderFrame()"),
            Instruction::OperatorModulo(l, a, b) => {
                format!(
                    "{} = {} % {}",
                    get_var(variables, l),
                    get_var(variables, a),
                    get_var(variables, b)
                )
            }
            Instruction::OperatorAdd(l, a, b) => {
                format!(
                    "{} = {} + {}",
                    get_var(variables, l),
                    get_var(variables, a),
                    get_var(variables, b)
                )
            }
            Instruction::OperatorSubtract(l, a, b) => {
                format!(
                    "{} = {} - {}",
                    get_var(variables, l),
                    get_var(variables, a),
                    get_var(variables, b)
                )
            }
            Instruction::OperatorMultiply(l, a, b) => {
                format!(
                    "{} = {} * {}",
                    get_var(variables, l),
                    get_var(variables, a),
                    get_var(variables, b)
                )
            }
            Instruction::OperatorDivide(l, a, b) => {
                format!(
                    "{} = {} / {}",
                    get_var(variables, l),
                    get_var(variables, a),
                    get_var(variables, b)
                )
            }
            Instruction::OperatorLesser(l, a, b) => {
                format!(
                    "{} = {} < {}",
                    get_var(variables, l),
                    get_var(variables, a),
                    get_var(variables, b)
                )
            }
            Instruction::OperatorGreater(l, a, b) => {
                format!(
                    "{} = {} > {}",
                    get_var(variables, l),
                    get_var(variables, a),
                    get_var(variables, b)
                )
            }
            Instruction::OperatorEquals(l, a, b) => {
                format!(
                    "{} = {} == {}",
                    get_var(variables, l),
                    get_var(variables, a),
                    get_var(variables, b)
                )
            }
            Instruction::FlowIfJump(c, l) => format!("if {c} jump to {l}"),
            Instruction::FlowIfJumpToPlace(c, l) => format!("if {c} goto {l}"),
            Instruction::FlowDefinePlace(p) => format!("{p}:"),
            Instruction::FlowIfNotJump(c, l) => format!("if !{c} jump to {l}"),
            Instruction::FlowIfNotJumpToPlace(c, l) => format!("if !{c} goto {l}"),
            Instruction::MotionChangeX(n) => format!("change x by {n}"),
            Instruction::MotionChangeY(n) => format!("change y by {n}"),
            Instruction::MotionSetX(x) => format!("set x to {x}"),
            Instruction::MotionSetY(y) => format!("set y to {y}"),
            Instruction::LooksSetSize(size) => format!("set size to {size}"),
        }
    }
}

fn find_key_by_value(map: &HashMap<String, usize>, target_value: usize) -> Option<&String> {
    for (key, &value) in map.iter() {
        if value == target_value {
            return Some(key);
        }
    }
    None
}

pub fn get_var(variables: Option<&HashMap<String, usize>>, item: &Value) -> String {
    match variables {
        Some(variables) => {
            let pointer: usize;
            match &item {
                Value::Pointer(n) => pointer = *n,
                _ => panic!(),
            }
            match find_key_by_value(&variables, pointer) {
                Some(key) => key.clone(),
                None => panic!(),
            }
        }
        None => match &item {
            Value::Pointer(n) => return n.to_string(),
            _ => panic!(),
        },
    }
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Pointer(usize),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "\"{}\"", n),
            Value::Pointer(n) => write!(f, "*{}", n),
        }
    }
}

impl Value {
    pub fn print(&self, variables: Option<&HashMap<String, usize>>) -> String {
        match &self {
            Value::Number(n) => format!("{}", n),
            Value::Boolean(n) => format!("{}", n),
            Value::String(n) => format!("\"{}\"", n),
            Value::Pointer(n) => format!("{}", get_var(variables, &Value::Pointer(*n))),
        }
    }

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
            Value::Pointer(n) => memory[*n].get_number(memory),
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
            Value::Number(n) => *n == 1.0,
            Value::String(n) => {
                if n == "true" {
                    return true;
                }
                match n.parse::<f64>() {
                    Ok(number) => {
                        if number == 1.0 {
                            return true;
                        }
                    }
                    Err(_) => {}
                }
                return false;
            }
            Value::Pointer(n) => memory[*n].get_bool(memory),
        }
    }
}
