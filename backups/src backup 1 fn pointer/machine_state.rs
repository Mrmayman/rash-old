#[derive(Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Pointer(usize),
}

pub struct MachineState {
    pub instruction_counter: usize,
    pub data_counter: usize,
    pub killed: bool,
    pub paused: bool,
    pub memory: Vec<Value>,
    data: Vec<Value>,
}

impl MachineState {
    pub fn new(data: Vec<Value>) -> MachineState {
        MachineState {
            instruction_counter: 0,
            data_counter: 0,
            killed: false,
            paused: false,
            memory: vec![],
            data,
        }
    }

    pub fn malloc(&mut self, size: i64) {
        // Fill the memory vector with empty placeholder elements
        for _ in 0..size {
            self.memory.push(Value::Number(0.0)); // You can use any appropriate placeholder value
        }
    }

    pub fn get_data(&self, i: usize) -> &Value {
        &self.data[self.data_counter + i]
    }

    pub fn get_number(&self, n: &Value) -> f64 {
        match n {
            Value::Pointer(ptr) => self.get_number(&self.memory[*ptr]),
            Value::Number(num) => *num,
            Value::String(str) => match str.parse::<f64>() {
                Ok(parsed) => parsed,
                Err(_) => 0.0,
            },
            Value::Boolean(val) => {
                if *val {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }

    pub fn get_bool(&self, n: &Value) -> bool {
        match n {
            Value::Pointer(ptr) => {
                self.get_bool(&self.memory[*ptr])
            }
            Value::Boolean(boolean) => {
                *boolean
            }
            Value::Number(number) => {
                todo!()
            }
            Value::String(stringvalue) => {
                todo!()
            }
            _ => {
                panic!("[error@bytecode.var_set] Pointer to variable is not actually a pointer");
            }
        }
    }

    pub fn set_mem(&mut self, target: &Value, value: &Value) {
        match target {
            Value::Pointer(ptr) => {
                self.memory[*ptr] = value.clone();
            }
            _ => {
                panic!("[error@bytecode.var_set] Pointer to variable is not actually a pointer");
            }
        }
    }
}
