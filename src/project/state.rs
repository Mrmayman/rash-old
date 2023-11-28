use std::collections::HashMap;

use crate::{
    interpreter::{Instruction, Value},
    project::base::BlockResult,
};

pub struct ParseState<'a> {
    pub variables: &'a mut HashMap<String, usize>,
    pub variable_data: &'a mut Vec<Value>,
    pub instructions: &'a mut Vec<Instruction>,
    pub forever_nest: i64,
    pub if_jump_number: i64,
    pub temp_variables: Vec<bool>,
    pub thread_number: usize,
    pub sprite: &'a serde_json::Value,
    pub costume_names: &'a Vec<String>,
}

impl<'a> ParseState<'a> {
    pub fn new(
        variables: &'a mut HashMap<String, usize>,
        variable_data: &'a mut Vec<Value>,
        instructions: &'a mut Vec<Instruction>,
        thread_number: usize,
        sprite: &'a serde_json::Value,
        costume_names: &'a Vec<String>,
    ) -> ParseState<'a> {
        ParseState {
            variables,
            variable_data,
            instructions,
            forever_nest: 0,
            if_jump_number: 0,
            temp_variables: vec![],
            thread_number,
            sprite,
            costume_names,
        }
    }
    pub fn compile_block(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let opcode = current_block["opcode"].as_str().unwrap();
        match opcode {
            "data_setvariableto" => self.c_variables_set(current_block),
            "operator_add" => self.c_operators_add(current_block),
            "operator_subtract" => self.c_operators_subtract(current_block),
            "operator_multiply" => self.c_operators_multiply(current_block),
            "operator_divide" => self.c_operators_divide(current_block),
            "operator_mod" => self.c_operators_mod(current_block),
            "operator_gt" => self.c_operators_greater(current_block),
            "operator_lt" => self.c_operators_lesser(current_block),
            "operator_equals" => self.c_operators_equals(current_block),
            "control_forever" => self.c_control_forever(current_block),
            "control_if" => self.c_control_if(current_block),
            "motion_gotoxy" => self.c_motion_go_to(current_block),
            "motion_changexby" => self.c_motion_change_x(current_block),
            "motion_changeyby" => self.c_motion_change_y(current_block),
            "motion_setx" => self.c_motion_set_x(current_block),
            "motion_sety" => self.c_motion_set_y(current_block),
            "looks_setsizeto" => self.c_looks_set_size(current_block),
            // "looks_switchcostumeto" => self.c_looks_switch_costume(current_block),
            _ => {
                eprintln!("[unimplemented block] {opcode}");
                BlockResult::Nothing
            }
        }
    }

    pub fn compile_substack(&mut self, current_block: &serde_json::Value) {
        let block_input = &current_block["inputs"]["SUBSTACK"];
        let block_id = block_input.as_array().unwrap()[1].as_str().unwrap();
        let mut block = self.get_block(block_id).unwrap();

        self.compile_block(&block);

        while block["next"] != serde_json::Value::Null {
            let next = block["next"].as_str().unwrap();
            block = self.get_block(next).unwrap();
            self.compile_block(&block);
        }
    }

    pub fn input_get_number(&mut self, current_block: &serde_json::Value, name: &str) -> Value {
        let input_1: Value;
        let input_1_data = current_block["inputs"][name].as_array().unwrap();

        // Deal with 2 different cases:
        // 1) Expression/function: set var to (1 + 1)
        // 2) Literal: set var to "1"

        if input_1_data.len() == 3 {
            // Case 1) expression/function: set var to (1 + 1)

            // Deal with 2 further cases:
            // 1.1) Block output: set var to (1 + 1)
            // 1.2) Variable output: set var to other_var

            match &input_1_data[1] {
                // Case 1.1) block output: set var to (1 + 1)
                serde_json::Value::String(n) => self.input_number_deal_with_block_output(n),
                // Case 1.2) Variable output: set var to other_var
                serde_json::Value::Array(n) => {
                    let ptr = self.variables.get(&n[2].as_str().unwrap().to_string());
                    Value::Pointer(*ptr.unwrap())
                }
                _ => panic!(),
            }
        } else {
            // Case 2) Literal: set var to "1"

            let num = input_1_data[1].as_array().unwrap()[1]
                .as_str()
                .unwrap()
                .to_string();
            match num.parse::<f64>() {
                Ok(n) => Value::Number(n),
                Err(_) => Value::Number(0.0),
            }
        }
    }

    fn input_number_deal_with_block_output(&mut self, block_id: &String) -> Value {
        // Get the input block.
        // set var to (1 + 1): Here the input block would be the plus operator.
        let input_block = self.get_block(&block_id.as_str()).unwrap();

        // Try to compile the block
        match self.compile_block(&input_block) {
            // If the block is not an operator (error).
            BlockResult::Nothing => {
                eprintln!(
                    "[unimplemented block] {}",
                    input_block["opcode"].as_str().unwrap()
                );
                Value::Number(0.0)
            }
            // Otherwise, deal with it.
            BlockResult::AllocatedMemory(n) => {
                self.register_free(n);
                Value::Pointer(self.register_get_variable_id(n))
            }
        }
    }

    pub fn get_block(&self, next: &str) -> Option<serde_json::Value> {
        for (block_id, block_data) in self.sprite["blocks"].as_object().unwrap() {
            if block_id == next {
                return Some(block_data.clone());
            }
        }
        None
    }
}
