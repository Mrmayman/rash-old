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
}
