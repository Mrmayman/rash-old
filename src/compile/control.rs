use crate::{
    interpreter::{Instruction, Value},
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    pub fn c_control_forever(&mut self, current_block: &serde_json::Value) -> BlockResult {
        self.forever_nest += 1;
        self.instructions.push(Instruction::ThreadPause);
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "forever{}",
            self.forever_nest
        )));
        if !current_block["inputs"].as_object().unwrap().is_empty() {
            self.compile_substack(current_block);
        }
        self.instructions.push(Instruction::ThreadPause);
        self.instructions.push(Instruction::FlowIfJumpToPlace(
            Value::Boolean(true),
            format!("forever{}", self.forever_nest),
        ));
        self.forever_nest -= 1;

        BlockResult::Nothing
    }

    pub fn c_control_if(&mut self, current_block: &serde_json::Value) -> BlockResult {
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
        let condition = self
            .get_block(
                current_block["inputs"]["CONDITION"].as_array().unwrap()[1]
                    .as_str()
                    .unwrap(),
            )
            .unwrap();
        let result = self.compile_block(&condition);

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
                self.compile_substack(current_block);
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
}
