use std::collections::HashMap;

use crate::{
    interpreter::{Instruction, Value},
    project::base::{get_block, BlockResult},
};

pub struct ParseState<'a> {
    pub variables: &'a mut HashMap<String, usize>,
    pub variable_data: &'a mut Vec<Value>,
    pub instructions: &'a mut Vec<Instruction>,
    pub forever_nest: i64,
    pub if_jump_number: i64,
    pub temp_variables: Vec<bool>,
    pub thread_number: usize,
}

impl<'a> ParseState<'a> {
    pub fn new(
        variables: &'a mut HashMap<String, usize>,
        variable_data: &'a mut Vec<Value>,
        instructions: &'a mut Vec<Instruction>,
        thread_number: usize,
    ) -> ParseState<'a> {
        ParseState {
            variables,
            variable_data,
            instructions,
            forever_nest: 0,
            if_jump_number: 0,
            temp_variables: vec![],
            thread_number,
        }
    }
    pub fn compile_block(
        &mut self,
        current_block: &serde_json::Value,
        sprite: &serde_json::Value,
    ) -> BlockResult {
        let opcode = current_block["opcode"].as_str().unwrap();
        match opcode {
            "data_setvariableto" => self.c_variables_set(current_block, sprite),
            "operator_add" => self.c_operators_add(current_block, sprite),
            "operator_subtract" => self.c_operators_subtract(current_block, sprite),
            "operator_multiply" => self.c_operators_multiply(current_block, sprite),
            "operator_divide" => self.c_operators_divide(current_block, sprite),
            "operator_mod" => self.c_operators_mod(current_block, sprite),
            "operator_gt" => self.c_operators_greater(current_block, sprite),
            "operator_lt" => self.c_operators_lesser(current_block, sprite),
            "operator_equals" => self.c_operators_equals(current_block, sprite),
            "control_forever" => self.c_control_forever(current_block, sprite),
            "control_if" => {
                // If no condition and blocks. Example:
                /* if() {} */
                if current_block["inputs"].as_object().unwrap().is_empty() {
                    return BlockResult::Nothing;
                }
                // If no blocks. Example:
                /* if (condition) {} */
                if current_block["inputs"]["SUBSTACK"] == serde_json::Value::Null {
                    return BlockResult::Nothing;
                }
                // If no condition. Example:
                /* if() {
                    // Code here.
                } */
                if current_block["inputs"]["CONDITION"] == serde_json::Value::Null {
                    return BlockResult::Nothing;
                }
                let condition = get_block(
                    current_block["inputs"]["CONDITION"].as_array().unwrap()[1]
                        .as_str()
                        .unwrap(),
                    sprite,
                )
                .unwrap();
                let result = self.compile_block(&condition, sprite);

                match &result {
                    crate::project::base::BlockResult::Nothing => {
                        eprintln!(
                            "[unimplemented block] {} (inside expression: control_if)",
                            condition["opcode"].as_str().unwrap()
                        )
                    }
                    crate::project::base::BlockResult::AllocatedMemory(n) => {
                        // self.instructions.push(Ins);
                        self.instructions.push(Instruction::FlowIfNotJumpToPlace(
                            Value::Pointer(self.register_get_variable_id(*n)),
                            format!("if{}", self.if_jump_number),
                        ));
                        self.compile_substack(current_block, sprite);
                        self.instructions.push(Instruction::FlowDefinePlace(format!(
                            "if{}",
                            self.if_jump_number
                        )));
                        self.register_free(*n);
                        self.if_jump_number += 1;
                    }
                }

                BlockResult::Nothing
            }
            _ => {
                eprintln!("[unimplemented block] {opcode}");
                BlockResult::Nothing
            }
        }
    }

    pub fn compile_substack(
        &mut self,
        current_block: &serde_json::Value,
        sprite: &serde_json::Value,
    ) {
        let block_input = &current_block["inputs"]["SUBSTACK"];
        let block_id = block_input.as_array().unwrap()[1].as_str().unwrap();
        let mut block = get_block(block_id, sprite).unwrap();

        self.compile_block(&block, &sprite);

        while block["next"] != serde_json::Value::Null {
            let next = block["next"].as_str().unwrap();
            block = get_block(next, &sprite).unwrap();
            self.compile_block(&block, &sprite);
        }
    }
}
