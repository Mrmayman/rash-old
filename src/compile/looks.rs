use crate::{
    interpreter::{Instruction, Value},
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    pub fn c_looks_set_size(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let value = self.input_get_number(current_block, "SIZE");
        self.instructions.push(Instruction::LooksSetSize(value));
        BlockResult::Nothing
    }

    pub fn c_looks_switch_costume(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let sub_block = self
            .get_block(
                current_block["inputs"]["COSTUME"].as_array().unwrap()[1]
                    .as_str()
                    .unwrap(),
            )
            .unwrap();

        let costume_value: Value;

        if sub_block["opcode"].as_str().unwrap() == "looks_costume" {
            let costume = sub_block["fields"]["COSTUME"].as_array().unwrap()[0]
                .as_str()
                .unwrap()
                .to_string();
            costume_value = Value::String(costume);
        } else {
            match self.compile_block(&sub_block) {
                BlockResult::Nothing => costume_value = Value::Number(0.0),
                BlockResult::AllocatedMemory(n) => {
                    costume_value = Value::Pointer(self.register_get_variable_id(n));
                    self.register_free(n)
                }
            }
        }

        self.instructions
            .push(Instruction::LooksSetCostume(costume_value));
        BlockResult::Nothing
    }

    pub fn c_looks_get_costume(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let register = self.register_malloc();
        if current_block["fields"]["NUMBER_NAME"].as_array().unwrap()[0]
            .as_str()
            .unwrap()
            == "number"
        {
            self.instructions
                .push(Instruction::LooksGetCostumeNumber(Value::Pointer(
                    self.register_get_variable_id(register),
                )))
        } else {
            todo!()
        }
        BlockResult::AllocatedMemory(register)
    }

    pub fn c_looks_hide(&mut self) -> BlockResult {
        self.instructions.push(Instruction::LooksHide);
        BlockResult::Nothing
    }

    pub fn c_looks_show(&mut self) -> BlockResult {
        self.instructions.push(Instruction::LooksShow);
        BlockResult::Nothing
    }
}
