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
        let input = self.input_get_number(current_block, "SIZE");
        self.instructions.push(Instruction::PenSetRadius(input));
        BlockResult::Nothing
    }
}
