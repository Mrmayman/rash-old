use crate::{
    interpreter::{Instruction, Value},
    thread_compiler::thread_compiler_main::ThreadCompiler,
};

impl<'a> ThreadCompiler<'a> {
    pub fn c_control_forever(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        self.jump_counter += 1;
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "forever{}",
            self.jump_counter
        )));
        if !current_block["inputs"].as_object().unwrap().is_empty() {
            self.compile_substack(current_block);
        }
        self.pause();
        self.instructions.push(Instruction::FlowIfJumpToPlace(
            Value::Boolean(true),
            format!("forever{}", self.jump_counter),
        ));
        self.jump_counter -= 1;

        None
    }

    pub fn c_control_if(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        // If no condition and blocks. if() {}
        if current_block["inputs"].as_object().unwrap().is_empty() {
            return None;
        }
        // If no blocks. if(condition) {}
        if current_block["inputs"]["SUBSTACK"] == serde_json::Value::Null {
            return None;
        }
        // If no condition. if() {}
        if current_block["inputs"]["CONDITION"] == serde_json::Value::Null {
            return None;
        }

        let result = self.get_input_bool(current_block);
        result?;

        self.instructions.push(Instruction::FlowIfNotJumpToPlace(
            Value::Pointer(self.register_get_variable_id(result.unwrap())),
            format!("if{}", self.if_jump_number),
        ));
        self.compile_substack(current_block);
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "if{}",
            self.if_jump_number
        )));
        self.register_free(result.unwrap());
        self.if_jump_number += 1;

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
        self.pause();

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

    pub fn c_control_repeat_until(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "repeat_until_start{}",
            self.jump_counter
        )));

        let condition = self.get_input_bool(current_block);
        condition?;

        self.instructions.push(Instruction::FlowIfJumpToPlace(
            Value::Pointer(self.register_get_variable_id(condition.unwrap())),
            format!("repeat_until_end{}", self.jump_counter),
        ));
        self.compile_substack(current_block);
        self.pause();

        self.instructions.push(Instruction::FlowIfJumpToPlace(
            Value::Boolean(true),
            format!("repeat_until_start{}", self.jump_counter),
        ));
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "repeat_until_end{}",
            self.jump_counter
        )));
        self.jump_counter += 1;

        self.register_free(condition.unwrap());

        None
    }
}
