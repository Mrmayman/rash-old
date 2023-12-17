use crate::{
    interpreter::{Instruction, Value},
    project::state::ParseState,
};

impl<'a> ParseState<'a> {
    pub fn c_looks_set_size(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let register = self.register_malloc();
        self.register_set_to_input(current_block, register, "SIZE");
        self.instructions
            .push(Instruction::LooksSetSize(Value::Pointer(
                self.register_get_variable_id(register),
            )));
        None
    }

    pub fn c_looks_switch_costume(&mut self, current_block: &serde_json::Value) -> Option<usize> {
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
                None => costume_value = Value::Number(0.0),
                Some(n) => {
                    costume_value = Value::Pointer(self.register_get_variable_id(n));
                    self.register_free(n)
                }
            }
        }

        self.instructions
            .push(Instruction::LooksSetCostume(costume_value));
        None
    }

    pub fn c_looks_get_costume(&mut self, current_block: &serde_json::Value) -> Option<usize> {
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
        Some(register)
    }

    pub fn c_looks_hide(&mut self) -> Option<usize> {
        self.instructions.push(Instruction::LooksHide);
        None
    }

    pub fn c_looks_show(&mut self) -> Option<usize> {
        self.instructions.push(Instruction::LooksShow);
        None
    }
}
