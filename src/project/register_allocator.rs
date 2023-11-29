use crate::interpreter::{Instruction, Value};

use super::{base::BlockResult, state::ParseState};

impl<'a> ParseState<'a> {
    pub fn register_malloc(&mut self) -> usize {
        let index = {
            if let Some(index) = self.temp_variables.iter().position(|&x| !x) {
                // There is an unallocated register, so allocate to it.
                self.temp_variables[index] = true;
                index
            } else {
                // No unallocated register found, creating new one.
                self.temp_variables.push(true);
                // Return the index of the new register.
                self.temp_variables.len() - 1
            }
        };
        let var_number = self.variables.len();

        let temp_var_name = format!("thread{}tempvar{}", self.thread_number, index);
        // If temp_var_name doesn't exist, add it.
        self.variables.entry(temp_var_name).or_insert(var_number);
        self.variable_data.push(Value::Number(0.0));
        index
    }

    pub fn register_free(&mut self, index: usize) {
        self.temp_variables[index] = false;
    }

    pub fn register_get_variable_id(&self, index: usize) -> usize {
        *self
            .variables
            .get(&format!("thread{}tempvar{}", self.thread_number, index))
            .unwrap()
    }

    pub fn register_set_to_input(
        &mut self,
        current_block: &serde_json::Value,
        register: usize,
        input: &str,
    ) {
        match &current_block["inputs"][input].as_array().unwrap()[1] {
            serde_json::Value::String(n) => {
                let block = self.get_block(n.as_str()).unwrap();
                match self.compile_block(&block) {
                    BlockResult::Nothing => {
                        self.instructions.push(Instruction::MemoryStore(
                            Value::Pointer(self.register_get_variable_id(register)),
                            Value::Number(0.0),
                        ));
                        eprintln!(
                            "[unimplemented block] {} (inside expression)",
                            block["opcode"].as_str().unwrap()
                        );
                    }
                    BlockResult::AllocatedMemory(n) => {
                        self.instructions.push(Instruction::MemoryStore(
                            Value::Pointer(self.register_get_variable_id(register)),
                            Value::Pointer(self.register_get_variable_id(n)),
                        ));
                        self.register_free(n);
                    }
                }
            }
            serde_json::Value::Array(n) => {
                if n.len() == 2 {
                    self.instructions.push(Instruction::MemoryStore(
                        Value::Pointer(self.register_get_variable_id(register)),
                        {
                            match n[1].as_str().unwrap().parse::<f64>() {
                                Ok(n) => Value::Number(n),
                                Err(_) => Value::Number(0.0),
                            }
                        },
                    ));
                } else {
                    self.instructions.push(Instruction::MemoryStore(
                        Value::Pointer(self.register_get_variable_id(register)),
                        Value::Pointer(
                            *self
                                .variables
                                .get(&n[2].as_str().unwrap().to_string())
                                .unwrap(),
                        ),
                    ))
                }
            }
            _ => panic!(),
        }
    }
}
