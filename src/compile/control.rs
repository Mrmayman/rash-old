use crate::{
    interpreter::{Instruction, Value},
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    pub fn c_control_forever(
        &mut self,
        current_block: &serde_json::Value,
        sprite: &serde_json::Value,
    ) -> BlockResult {
        self.forever_nest += 1;
        self.instructions.push(Instruction::ThreadPause);
        self.instructions.push(Instruction::FlowDefinePlace(format!(
            "forever{}",
            self.forever_nest
        )));
        if !current_block["inputs"].as_object().unwrap().is_empty() {
            self.compile_substack(current_block, sprite);
        }
        self.instructions.push(Instruction::ThreadPause);
        self.instructions.push(Instruction::FlowIfJumpToPlace(
            Value::Boolean(true),
            format!("forever{}", self.forever_nest),
        ));
        self.forever_nest -= 1;

        BlockResult::Nothing
    }
}
