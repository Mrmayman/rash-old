use std::collections::HashMap;

use crate::{
    ansi_codes,
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
}

impl<'a> ParseState<'a> {
    pub fn new(
        variables: &'a mut HashMap<String, usize>,
        variable_data: &'a mut Vec<Value>,
        instructions: &'a mut Vec<Instruction>,
        thread_number: usize,
        sprite: &'a serde_json::Value,
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
            "operator_mathop" => self.c_operators_mathop(current_block),
            "control_forever" => self.c_control_forever(current_block),
            "control_if" => self.c_control_if(current_block),
            "control_repeat" => self.c_control_repeat(current_block),
            "motion_gotoxy" => self.c_motion_go_to(current_block),
            "motion_changexby" => self.c_motion_change_x(current_block),
            "motion_changeyby" => self.c_motion_change_y(current_block),
            "motion_setx" => self.c_motion_set_x(current_block),
            "motion_sety" => self.c_motion_set_y(current_block),
            "motion_xposition" => self.c_motion_get_x(),
            "motion_yposition" => self.c_motion_get_y(),
            "looks_setsizeto" => self.c_looks_set_size(current_block),
            "looks_switchcostumeto" => self.c_looks_switch_costume(current_block),
            "looks_costumenumbername" => self.c_looks_get_costume(current_block),
            "looks_show" => self.c_looks_show(),
            "looks_hide" => self.c_looks_hide(),
            "pen_clear" => self.c_pen_clear(),
            "pen_stamp" => self.c_pen_stamp(),
            "pen_penUp" => self.c_pen_up(),
            "pen_penDown" => self.c_pen_down(),
            "pen_setPenSizeTo" => self.c_pen_set_size(current_block),
            "sensing_timer" => self.c_sensing_timer(),
            _ => {
                eprintln!(
                    "{}[unimplemented block]{} {opcode}",
                    ansi_codes::RED,
                    ansi_codes::RESET
                );
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

    pub fn get_block(&self, next: &str) -> Option<serde_json::Value> {
        for (block_id, block_data) in self.sprite["blocks"].as_object().unwrap() {
            if block_id == next {
                return Some(block_data.clone());
            }
        }
        None
    }

    pub fn dump(&self) {
        println!(
            "{}[variable dump]{} {{",
            ansi_codes::GREEN,
            ansi_codes::RESET
        );
        for (variable, i) in self.variables.iter() {
            println!(
                "    {}{i}: {}{variable}{} ({})",
                ansi_codes::YELLOW,
                ansi_codes::WHITE,
                ansi_codes::RESET,
                self.variable_data[*i].print(Some(self.variables))
            );
        }
        println!("}}");
        println!(
            "{}[memory leak dump]{} {{",
            ansi_codes::GREEN,
            ansi_codes::RESET
        );
        for (index, variable) in self.temp_variables.iter().enumerate() {
            if *variable {
                println!("    thread{}tempvar{}", self.thread_number, index,);
            }
        }
        println!("}}");
        println!(
            "{}[instruction dump]{} {{",
            ansi_codes::GREEN,
            ansi_codes::RESET
        );
        for instruction in self.instructions.iter() {
            println!(
                "    {}{}{}",
                ansi_codes::WHITE,
                instruction.print(Some(self.variables)),
                ansi_codes::RESET
            );
        }
        println!("}}");
    }

    pub fn finish(&mut self) {
        self.instructions.push(Instruction::ThreadKill)
    }
}
