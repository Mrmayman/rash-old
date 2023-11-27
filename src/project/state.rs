use std::{collections::HashMap, thread::current};

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
            "control_if" => self.c_control_if(current_block, sprite),
            "motion_gotoxy" => self.c_motion_go_to(current_block, sprite),
            "motion_changexby" => self.c_motion_change_x(current_block, sprite),
            "motion_changeyby" => self.c_motion_change_y(current_block, sprite),
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
