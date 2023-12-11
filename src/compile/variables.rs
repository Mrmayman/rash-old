use crate::{
    interpreter::{Instruction, Value},
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    pub fn c_variables_set(&mut self, current_block: &serde_json::Value) -> BlockResult {
        // Get the internal name of the variable.
        let var_name = current_block["fields"]["VARIABLE"].as_array().unwrap()[1]
            .as_str()
            .unwrap();
        // Get the id of the variable in the Rash VM.
        let id: usize = *self.variables.get(var_name).unwrap();

        let register = self.register_malloc();
        self.register_set_to_input(current_block, register, "VALUE");
        self.instructions.push(Instruction::MemoryStore(
            Value::Pointer(id),
            Value::Pointer(self.register_get_variable_id(register)),
        ));
        self.register_free(register);

        // Deal with 2 different cases:
        // 1) literal: set var to "1"
        // 2) expression/function: set var to (1 + 1)
        /*match &current_block["inputs"]["VALUE"].as_array().unwrap()[1] {
            // If we are setting the variable to the output of another block (expression/function).
            serde_json::Value::String(s) => {
                let block = self.get_block(s).unwrap();
                let result = self.compile_block(&block);
                match &result {
                    crate::project::base::BlockResult::Nothing => {
                        eprintln!(
                            "{}[unimplemented block]{} {} (inside expression: data_setvariableto)",
                            ansi_codes::RED,
                            ansi_codes::RESET,
                            block["opcode"].as_str().unwrap()
                        )
                    }
                    crate::project::base::BlockResult::AllocatedMemory(n) => {
                        self.instructions.push(Instruction::MemoryStore(
                            Value::Pointer(id),
                            Value::Pointer(self.register_get_variable_id(*n)),
                        ));
                        self.register_free(*n);
                    }
                }
            }
            // If we are setting the variable to a literal.
            serde_json::Value::Array(a) => match &a[1] {
                // Setting the variable to a number.
                serde_json::Value::Number(n) => {
                    self.instructions.push(Instruction::MemoryStore(
                        Value::Pointer(id),
                        Value::Number(n.as_f64().unwrap()),
                    ));
                }
                // Setting the variable to a string.
                serde_json::Value::String(s) => {
                    self.instructions.push(Instruction::MemoryStore(
                        Value::Pointer(id),
                        Value::String(s.to_string()),
                    ));
                }
                // I don't think variables can get set to booleans. Not sure.
                _ => panic!("Setting variable to invalid data type"),
            },
            _ => panic!("Invalid data in block.variables.inputs.VALUE"),
        }*/
        BlockResult::Nothing
    }
}
