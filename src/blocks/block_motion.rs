use crate::{
    interpreter::{Instruction, Value},
    bc_compiler::bc_comp_main::ThreadCompiler,
};

impl<'a> ThreadCompiler<'a> {
    pub fn c_motion_go_to(&mut self, current_block: &serde_json::Value) -> Option<usize> {
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
        None
    }

    pub fn c_motion_set_x(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let x = self.register_malloc();
        self.register_set_to_input(current_block, x, "X");
        self.instructions
            .push(Instruction::MotionSetX(Value::Pointer(
                self.register_get_variable_id(x),
            )));
        self.register_free(x);
        None
    }

    pub fn c_motion_set_y(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let y = self.register_malloc();
        self.register_set_to_input(current_block, y, "Y");
        self.instructions
            .push(Instruction::MotionSetY(Value::Pointer(
                self.register_get_variable_id(y),
            )));
        self.register_free(y);
        None
    }

    pub fn c_motion_change_x(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let x = self.register_malloc();
        self.register_set_to_input(current_block, x, "DX");
        self.instructions
            .push(Instruction::MotionChangeX(Value::Pointer(
                self.register_get_variable_id(x),
            )));
        self.register_free(x);
        None
    }

    pub fn c_motion_change_y(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let y = self.register_malloc();
        self.register_set_to_input(current_block, y, "DY");
        self.instructions
            .push(Instruction::MotionChangeY(Value::Pointer(
                self.register_get_variable_id(y),
            )));
        self.register_free(y);
        None
    }

    pub fn c_motion_get_x(&mut self) -> Option<usize> {
        let register = self.register_malloc();
        self.instructions
            .push(Instruction::MotionGetX(Value::Pointer(
                self.register_get_variable_id(register),
            )));
        Some(register)
    }

    pub fn c_motion_get_y(&mut self) -> Option<usize> {
        let register = self.register_malloc();
        self.instructions
            .push(Instruction::MotionGetY(Value::Pointer(
                self.register_get_variable_id(register),
            )));
        Some(register)
    }
}
