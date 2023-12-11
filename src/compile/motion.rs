use crate::{
    interpreter::{Instruction, Value},
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    pub fn c_motion_go_to(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let x = self.register_malloc();
        let y = self.register_malloc();

        self.register_set_to_input(current_block, x, "X");
        self.register_set_to_input(current_block, y, "Y");

        self.instructions.push(Instruction::MotionSetXY(
            Value::Pointer(self.register_get_variable_id(x)),
            Value::Pointer(self.register_get_variable_id(y)),
        ));

        self.register_free(x);
        self.register_free(y);
        BlockResult::Nothing
    }

    pub fn c_motion_set_x(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let x = self.register_malloc();
        self.register_set_to_input(current_block, x, "X");
        self.instructions
            .push(Instruction::MotionSetX(Value::Pointer(x)));
        self.register_free(x);
        BlockResult::Nothing
    }

    pub fn c_motion_set_y(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let y = self.register_malloc();
        self.register_set_to_input(current_block, y, "Y");
        self.instructions
            .push(Instruction::MotionSetY(Value::Pointer(y)));
        self.register_free(y);
        BlockResult::Nothing
    }

    pub fn c_motion_change_x(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let x = self.register_malloc();
        self.register_set_to_input(current_block, x, "DX");
        self.instructions
            .push(Instruction::MotionChangeX(Value::Pointer(x)));
        self.register_free(x);
        BlockResult::Nothing
    }

    pub fn c_motion_change_y(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let y = self.register_malloc();
        self.register_set_to_input(current_block, y, "DY");
        self.instructions
            .push(Instruction::MotionChangeY(Value::Pointer(y)));
        self.register_free(y);
        BlockResult::Nothing
    }

    pub fn c_motion_get_x(&mut self) -> BlockResult {
        let register = self.register_malloc();
        self.instructions
            .push(Instruction::MotionGetX(Value::Pointer(
                self.register_get_variable_id(register),
            )));
        BlockResult::AllocatedMemory(register)
    }

    pub fn c_motion_get_y(&mut self) -> BlockResult {
        let register = self.register_malloc();
        self.instructions
            .push(Instruction::MotionGetY(Value::Pointer(
                self.register_get_variable_id(register),
            )));
        BlockResult::AllocatedMemory(register)
    }
}

/*fn get_input() {
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
            self.register_free(*n);
        }
    }
}
*/
