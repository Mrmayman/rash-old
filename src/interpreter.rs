use std::{collections::HashMap, fmt::format};

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
    MotionSetXY(Value, Value),
    MotionSetX(Value),
    MotionSetY(Value),
    MotionGetX(Value),
    MotionGetY(Value),
    LooksSetSize(Value),
    LooksSetCostume(Value),
    LooksGetCostumeNumber(Value),
    PenClear,
    PenStamp,
}

impl Instruction {
    pub fn print(&self, variables: Option<&HashMap<String, usize>>) -> String {
        match &self {
            Instruction::MemoryStore(at, n) => {
                format!("{} = {};", get_var(variables, at), n.print(variables))
            }
            Instruction::MemoryDump => "dump_memory();".to_owned(),
            Instruction::ThreadKill => "return;".to_owned(),
            Instruction::ThreadPause => "render_frame()".to_owned(),
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
            Instruction::FlowIfJump(condition, l) => {
                format!(
                    "if {} jump to {}",
                    condition.print(variables),
                    l.print(variables)
                )
            }
            Instruction::FlowIfJumpToPlace(condition, l) => {
                format!("if {} goto {l}", condition.print(variables))
            }
            Instruction::FlowDefinePlace(place) => format!("{place}:"),
            Instruction::FlowIfNotJump(condition, location) => {
                format!(
                    "if !{} jump to {}",
                    condition.print(variables),
                    location.print(variables)
                )
            }
            Instruction::FlowIfNotJumpToPlace(condition, location) => {
                format!("if !{} goto {location}", condition.print(variables))
            }
            Instruction::MotionChangeX(x) => format!("change x by {}", x.print(variables)),
            Instruction::MotionChangeY(y) => format!("change y by {}", y.print(variables)),
            Instruction::MotionSetX(x) => format!("set x to {}", x.print(variables)),
            Instruction::MotionSetY(y) => format!("set y to {}", y.print(variables)),
            Instruction::MotionSetXY(x, y) => {
                format!("go to x: {}, y: {}", x.print(variables), y.print(variables))
            }
            Instruction::LooksSetSize(size) => format!("set size to {}", size.print(variables)),
            Instruction::LooksSetCostume(costume) => {
                format!("set costume to {}", costume.print(variables))
            }
            Instruction::LooksGetCostumeNumber(location) => {
                format!("{} = get_costume_number()", location.print(variables))
            }
            Instruction::MotionGetX(location) => format!("{} = get_x()", location.print(variables)),
            Instruction::MotionGetY(location) => format!("{} = get_y()", location.print(variables)),
            Instruction::PenClear => "pen_clear()".to_owned(),
            Instruction::PenStamp => "pen_stamp()".to_owned(),
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
            let pointer: usize = item.get_pointer();
            match find_key_by_value(variables, pointer) {
                Some(key) => key.clone(),
                None => panic!(),
            }
        }
        None => match &item {
            Value::Pointer(n) => "*".to_owned() + &n.to_string(),
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

/*impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(n) => write!(f, "{}", n),
            Value::String(n) => write!(f, "\"{}\"", n),
            Value::Pointer(n) => write!(f, "*{}", n),
        }
    }
}*/

impl Value {
    pub fn print(&self, variables: Option<&HashMap<String, usize>>) -> String {
        match &self {
            Value::Number(n) => format!("{}", n),
            Value::Boolean(n) => format!("{}", n),
            Value::String(n) => format!("\"{}\"", n),
            Value::Pointer(n) => format!("*{}", get_var(variables, &Value::Pointer(*n))),
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
                if let Ok(number) = n.parse::<f64>() {
                    if number == 1.0 {
                        return true;
                    }
                }
                false
            }
            Value::Pointer(n) => memory[*n].get_bool(memory),
        }
    }

    pub fn get_string(&self, memory: &[Value]) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Boolean(n) => {
                if *n {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Value::String(n) => n.clone(),
            Value::Pointer(n) => memory[*n].get_string(memory),
        }
    }
}
