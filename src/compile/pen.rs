use crate::{
    interpreter::Instruction,
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    pub fn c_pen_clear(&mut self) -> BlockResult {
        self.instructions.push(Instruction::PenClear);
        BlockResult::Nothing
    }

    pub fn c_pen_stamp(&mut self) -> BlockResult {
        self.instructions.push(Instruction::PenStamp);
        BlockResult::Nothing
    }

    pub fn c_pen_up(&mut self) -> BlockResult {
        self.instructions.push(Instruction::PenUp);
        BlockResult::Nothing
    }

    pub fn c_pen_down(&mut self) -> BlockResult {
        self.instructions.push(Instruction::PenDown);
        BlockResult::Nothing
    }

    pub fn c_pen_set_size(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let size = self.register_malloc();
        self.register_set_to_input(current_block, size, "SIZE");
        self.instructions.push(Instruction::PenSetRadius(
            crate::interpreter::Value::Pointer(size),
        ));
        self.register_free(size);
        BlockResult::Nothing
    }
}
