use crate::{
    interpreter::Instruction,
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    pub fn c_looks_set_size(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let value = self.input_get_number(current_block, "SIZE");
        self.instructions.push(Instruction::LooksSetSize(value));
        BlockResult::Nothing
    }

    /*pub fn c_looks_switch_costume(&mut self, current_block: &serde_json::Value) -> BlockResult {
        println!("{current_block}");
        println!(
            "{}",
            self.get_block(
                current_block["inputs"]["COSTUME"].as_array().unwrap()[1]
                    .as_str()
                    .unwrap()
            )
            .unwrap()
        );
        // if self.costume_names.contains(x)
        // todo!();
        BlockResult::Nothing
    }*/
}
