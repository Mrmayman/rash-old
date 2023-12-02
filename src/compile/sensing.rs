use crate::{
    interpreter::{Instruction, Value},
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    pub fn c_sensing_timer(&mut self) -> BlockResult {
        let register = self.register_malloc();
        self.instructions
            .push(Instruction::SensingTimer(Value::Pointer(
                self.register_get_variable_id(register),
            )));
        BlockResult::AllocatedMemory(register)
    }
}
