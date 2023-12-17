use crate::{
    ansi_codes,
    interpreter::{Instruction, Value},
    project::state::ParseState,
};

impl<'a> ParseState<'a> {
    pub fn c_control_forever(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        self.forever_nest += 1;
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "forever{}",
            self.forever_nest
        )));
        if !current_block["inputs"].as_object().unwrap().is_empty() {
            self.compile_substack(current_block);
        }
        self.instructions.push(Instruction::ThreadPause);
        self.instructions.push(Instruction::ThreadPause);
        self.instructions.push(Instruction::FlowIfJumpToPlace(
            Value::Boolean(true),
            format!("forever{}", self.forever_nest),
        ));
        self.forever_nest -= 1;

        None
    }

    pub fn c_control_if(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        // If no condition and blocks. Example:
        /* if() {} */
        if current_block["inputs"].as_object().unwrap().is_empty() {
            return None;
        }
        // If no blocks. Example:
        /* if (condition) {} */
        if current_block["inputs"]["SUBSTACK"] == serde_json::Value::Null {
            return None;
        }
        // If no condition. Example:
        /* if() {
         *     // Code here.
         * }*/
        if current_block["inputs"]["CONDITION"] == serde_json::Value::Null {
            return None;
        }

        let condition = self
            .get_block(
                current_block["inputs"]["CONDITION"].as_array().unwrap()[1]
                    .as_str()
                    .unwrap(),
            )
            .unwrap();
        let result = self.compile_block(&condition);

        match &result {
            None => {
                eprintln!(
                    "{}[unimplemented block]{} {} (inside expression: control_if)",
                    ansi_codes::RED,
                    ansi_codes::RESET,
                    condition["opcode"].as_str().unwrap()
                )
            }
            Some(n) => {
                // self.instructions.push(Ins);
                self.instructions.push(Instruction::FlowIfNotJumpToPlace(
                    Value::Pointer(self.register_get_variable_id(*n)),
                    format!("if{}", self.if_jump_number),
                ));
                self.compile_substack(current_block);
                self.instructions.push(Instruction::FlowDefinePlace(format!(
                    "if{}",
                    self.if_jump_number
                )));
                self.register_free(*n);
                self.if_jump_number += 1;
            }
        }

        None
    }

    pub fn c_control_repeat(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        println!("{}", current_block);
        let num_iters = self.register_malloc();
        let temp_result = self.register_malloc();

        self.register_set_to_input(current_block, num_iters, "TIMES");
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "repeat_start{}",
            self.if_jump_number
        )));

        self.instructions.push(Instruction::OperatorLesser(
            Value::Pointer(self.register_get_variable_id(temp_result)),
            Value::Pointer(self.register_get_variable_id(num_iters)),
            Value::Number(1.0),
        ));
        self.instructions.push(Instruction::FlowIfJumpToPlace(
            Value::Pointer(self.register_get_variable_id(temp_result)),
            format!("repeat_end{}", self.if_jump_number),
        ));

        // The actual code in the loop.
        self.compile_substack(current_block);

        self.instructions.push(Instruction::ThreadPause);
        self.instructions.push(Instruction::ThreadPause);

        self.instructions.push(Instruction::OperatorSubtract(
            Value::Pointer(self.register_get_variable_id(num_iters)),
            Value::Pointer(self.register_get_variable_id(num_iters)),
            Value::Number(1.0),
        ));
        self.instructions.push(Instruction::FlowIfJumpToPlace(
            Value::Boolean(true),
            format!("repeat_start{}", self.if_jump_number),
        ));
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "repeat_end{}",
            self.if_jump_number
        )));
        self.if_jump_number += 1;

        self.register_free(num_iters);
        self.register_free(temp_result);

        None
    }
}
