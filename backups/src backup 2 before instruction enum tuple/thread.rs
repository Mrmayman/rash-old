use crate::interpreter::{Instruction, Value};
use std::ops::{Add, Div, Mul, Sub};

pub struct Thread {
    instructions: Box<[Value]>,
    pub killed: bool,
    counter: usize,
}

impl Thread {
    pub fn new(instructions: Box<[Value]>) -> Thread {
        Thread {
            instructions,
            killed: false,
            counter: 0,
        }
    }

    pub fn run(&mut self, memory: &mut [Value]) {
        loop {
            let should_break: bool = self.run_bytecode(memory);
            if should_break {
                break;
            }
            self.counter += 1;
        }
    }

    fn get_data(&self, n: usize) -> &Value {
        &self.instructions[self.counter + n]
    }

    fn perform_arithmetic(&mut self, memory: &mut [Value], operation: fn(f64, f64) -> f64) {
        let location = self.get_data(1).get_pointer();
        let a = self.get_data(2).get_number(memory);
        let b = self.get_data(3).get_number(memory);
        memory[location] = Value::Number(operation(a, b));
        self.counter += 3;
    }

    fn run_bytecode(&mut self, memory: &mut [Value]) -> bool {
        match self.instructions[self.counter].get_bytecode() {
            Instruction::MemoryDump => {
                dump_memory(memory);
            }
            Instruction::MemoryStore => {
                let location = self.get_data(1).get_pointer();
                let value = self.get_data(2).clone();
                memory[location] = value;
                self.counter += 2;
            }
            Instruction::ThreadPause => {
                self.counter += 1;
                return true;
            }
            Instruction::ThreadKill => {
                self.killed = true;
                return true;
            }
            Instruction::OperatorModulo => {
                let location = self.get_data(1).get_pointer();
                let a = self.get_data(2).get_number(memory);
                let b = self.get_data(3).get_number(memory);
                memory[location] = Value::Number(a.rem_euclid(b));
                self.counter += 3;
            }
            Instruction::OperatorAdd => self.perform_arithmetic(memory, f64::add),
            Instruction::OperatorSubtract => self.perform_arithmetic(memory, f64::sub),
            Instruction::OperatorMultiply => self.perform_arithmetic(memory, f64::mul),
            Instruction::OperatorDivide => self.perform_arithmetic(memory, f64::div),
            Instruction::OperatorLesser => {
                let location = self.get_data(1).get_pointer();
                let a = self.get_data(2).get_number(memory);
                let b = self.get_data(3).get_number(memory);
                memory[location] = Value::Boolean(a < b);
                self.counter += 3;
            }
            Instruction::OperatorGreater => {
                let location = self.get_data(1).get_pointer();
                let a = self.get_data(2).get_number(memory);
                let b = self.get_data(3).get_number(memory);
                memory[location] = Value::Boolean(a > b);
                self.counter += 3;
            }
            Instruction::FlowIfJump => {
                let condition = self.get_data(1).get_bool(memory);
                if condition {
                    let location = self.get_data(2).get_number(memory) as usize;
                    self.counter = location;
                } else {
                    self.counter += 2;
                }
            }
        }
        false
    }
}

fn dump_memory(memory: &[Value]) {
    for val in memory {
        match val {
            Value::Pointer(n) => {
                println!("pointer: {}", n)
            }
            Value::Number(n) => {
                println!("number: {}", n)
            }
            Value::Boolean(n) => {
                println!("bool: {}", n)
            }
            Value::String(n) => {
                println!("string: {}", n)
            }
            Value::Bytecode(_) => {
                println!("bytecode (unprintable)");
            }
        }
    }
}
