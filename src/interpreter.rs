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
    OperatorPower(Value, Value, Value),
    OperatorERaised(Value, Value),
    OperatorSin(Value, Value),
    OperatorCos(Value, Value),
    OperatorTan(Value, Value),
    OperatorAbs(Value, Value),
    OperatorASin(Value, Value),
    OperatorACos(Value, Value),
    OperatorATan(Value, Value),
    OperatorSqrt(Value, Value),
    OperatorLn(Value, Value),
    OperatorLog(Value, Value),
    OperatorFloor(Value, Value),
    OperatorCeiling(Value, Value),
    OperatorGreater(Value, Value, Value),
    OperatorEquals(Value, Value, Value),
    SensingTimer(Value),
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
    LooksHide,
    LooksShow,
    PenClear,
    PenStamp,
    PenUp,
    PenDown,
    PenSetRadius(Value),
}

impl Instruction {
    pub fn print(&self, variables: Option<&HashMap<String, usize>>) -> String {
        match &self {
            Instruction::MemoryStore(at, n) => {
                format!("{} = {};", at.print(variables), n.print(variables))
            }
            Instruction::MemoryDump => "dump_memory();".to_owned(),
            Instruction::ThreadKill => "return;".to_owned(),
            Instruction::ThreadPause => "render_frame()".to_owned(),
            Instruction::OperatorModulo(l, a, b) => {
                format!(
                    "{} = {} % {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::OperatorAdd(l, a, b) => {
                format!(
                    "{} = {} + {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::OperatorSubtract(l, a, b) => {
                format!(
                    "{} = {} - {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::OperatorMultiply(l, a, b) => {
                format!(
                    "{} = {} * {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::OperatorDivide(l, a, b) => {
                format!(
                    "{} = {} / {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::OperatorPower(l, a, b) => {
                format!(
                    "{} = {} ^ {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::OperatorERaised(l, n) => {
                format!("{} = e ^ {}", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorSin(l, n) => {
                format!("{} = sin({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorCos(l, n) => {
                format!("{} = cos({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorTan(l, n) => {
                format!("{} = tan({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorAbs(l, n) => {
                format!("{} = abs({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorASin(l, n) => {
                format!("{} = asin({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorACos(l, n) => {
                format!("{} = acos({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorATan(l, n) => {
                format!("{} = atan({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorSqrt(l, n) => {
                format!("{} = sqrt({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorLn(l, n) => {
                format!("{} = ln({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorLog(l, n) => {
                format!("{} = log({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorFloor(l, n) => {
                format!("{} = floor({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorCeiling(l, n) => {
                format!("{} = ceiling({})", l.print(variables), n.print(variables),)
            }
            Instruction::OperatorLesser(l, a, b) => {
                format!(
                    "{} = {} < {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::OperatorGreater(l, a, b) => {
                format!(
                    "{} = {} > {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::OperatorEquals(l, a, b) => {
                format!(
                    "{} = {} == {}",
                    l.print(variables),
                    a.print(variables),
                    b.print(variables)
                )
            }
            Instruction::SensingTimer(location) => {
                format!("{} = timer()", location.print(variables))
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
            Instruction::PenUp => "pen_up()".to_owned(),
            Instruction::PenDown => "pen_down()".to_owned(),
            Instruction::PenSetRadius(value) => format!("pen_set_size({})", value.print(variables)),
            Instruction::LooksHide => "looks_hide()".to_owned(),
            Instruction::LooksShow => "looks_show()".to_owned(),
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
