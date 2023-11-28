use crate::{
    interpreter::{Instruction, Value},
    sprite::GraphicalProperties,
};

pub struct Thread {
    instructions: Box<[Instruction]>,
    pub killed: bool,
    counter: usize,
}

impl Thread {
    pub fn new(instructions: Box<[Instruction]>) -> Thread {
        Thread {
            instructions,
            killed: false,
            counter: 0,
        }
    }

    pub fn run(&mut self, memory: &mut [Value], properties: &mut GraphicalProperties) {
        loop {
            let should_break: bool = self.run_bytecode(memory, properties);
            self.counter += 1;
            if should_break {
                break;
            }
        }
    }

    fn get_place(&self, string: &String) -> Option<usize> {
        for (index, item) in self.instructions.iter().enumerate() {
            match item {
                Instruction::FlowDefinePlace(n) => {
                    if n == string {
                        return Some(index);
                    }
                }
                // Handle other variants if needed
                _ => {}
            }
        }
        None
    }

    fn run_bytecode(&mut self, memory: &mut [Value], properties: &mut GraphicalProperties) -> bool {
        // println!("{}", self.instructions[self.counter].print(None));
        match &self.instructions[self.counter] {
            Instruction::MemoryDump => {
                println!("[memory dump] {{");
                dump_memory(memory);
                println!("}}")
            }
            Instruction::MemoryStore(location, value) => {
                let location = location.get_pointer();
                let value_read: Value;
                if let Value::Pointer(n) = value {
                    value_read = memory[*n].clone();
                } else {
                    value_read = value.clone();
                }
                memory[location] = value_read;
            }
            Instruction::ThreadPause => {
                return true;
            }
            Instruction::ThreadKill => {
                self.killed = true;
                return true;
            }
            Instruction::OperatorModulo(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a.rem_euclid(b));
            }
            Instruction::OperatorAdd(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a + b);
            }
            Instruction::OperatorSubtract(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a - b);
            }
            Instruction::OperatorMultiply(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a * b);
            }
            Instruction::OperatorDivide(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Number(a / b);
            }
            Instruction::OperatorLesser(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Boolean(a < b);
            }
            Instruction::OperatorGreater(location, a, b) => {
                let location = location.get_pointer();
                let a = a.get_number(memory);
                let b = b.get_number(memory);
                memory[location] = Value::Boolean(a > b);
            }
            Instruction::OperatorEquals(location, a, b) => {
                let a = if let Value::Pointer(n) = a {
                    &memory[*n]
                } else {
                    a
                };
                let b = if let Value::Pointer(n) = b {
                    &memory[*n]
                } else {
                    b
                };
                let location = location.get_pointer();
                memory[location] = Value::Boolean(match &(a, b) {
                    (Value::Number(n1), Value::Number(n2)) => *n1 == *n2,
                    (Value::Boolean(n1), Value::Boolean(n2)) => *n1 == *n2,
                    (Value::String(n1), Value::String(n2)) => *n1 == *n2,
                    (Value::Number(_), Value::Boolean(n2)) => a.get_bool(memory) == *n2,
                    (Value::Boolean(n1), Value::Number(_)) => *n1 == b.get_bool(memory),
                    (Value::String(_), Value::Number(n2)) => a.get_number(memory) == *n2,
                    (Value::Number(n1), Value::String(_)) => *n1 == b.get_number(memory),
                    (Value::Boolean(n1), Value::String(_)) => *n1 == b.get_bool(memory),
                    (Value::String(_), Value::Boolean(n2)) => a.get_bool(memory) == *n2,
                    _ => panic!(
                        "Unsupported types for equality comparison at {}",
                        self.counter
                    ),
                })
            }
            Instruction::FlowIfJump(condition, location) => {
                if condition.get_bool(memory) {
                    let location = location.get_number(memory) as usize;
                    self.counter = location;
                }
            }
            Instruction::FlowIfJumpToPlace(condition, place) => {
                if condition.get_bool(memory) {
                    let location = self
                        .get_place(place)
                        .expect("Could not find jump point in program");
                    self.counter = location;
                }
            }
            Instruction::FlowIfNotJump(condition, location) => {
                if !condition.get_bool(memory) {
                    let location = location.get_number(memory) as usize;
                    self.counter = location;
                }
            }
            Instruction::FlowIfNotJumpToPlace(condition, place) => {
                if !condition.get_bool(memory) {
                    let location = self
                        .get_place(place)
                        .expect(format!("Could not find jump point in program {}", place).as_str());
                    self.counter = location;
                }
            }
            Instruction::FlowDefinePlace(_) => {}
            Instruction::MotionChangeX(n) => {
                properties.x += n.get_number(memory);
            }
            Instruction::MotionChangeY(n) => properties.y += n.get_number(memory),
            Instruction::MotionSetX(x) => properties.x = x.get_number(memory),
            Instruction::MotionSetY(y) => properties.y = y.get_number(memory),
            Instruction::LooksSetSize(size) => properties.size = size.get_number(memory) as f32,
        }
        false
    }
}

fn dump_memory(memory: &[Value]) {
    for val in memory {
        match val {
            Value::Pointer(n) => {
                println!("    pointer: {}", n)
            }
            Value::Number(n) => {
                println!("    number: {}", n)
            }
            Value::Boolean(n) => {
                println!("    bool: {}", n)
            }
            Value::String(n) => {
                println!("    string: {}", n)
            }
        }
    }
}
