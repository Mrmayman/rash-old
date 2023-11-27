use crate::{
    interpreter::{Instruction, Value},
    project::{
        base::{get_block, BlockResult},
        state::ParseState,
    },
};

impl<'a> ParseState<'a> {
    pub fn c_motion_go_to(
        &mut self,
        current_block: &serde_json::Value,
        sprite: &serde_json::Value,
    ) -> BlockResult {
        println!("{current_block}");

        let mut input_1: Value;
        let mut input_2: Value;

        let input_1_data = current_block["inputs"]["X"].as_array().unwrap();
        if input_1_data.len() == 3 {
            let input_block = get_block(input_1_data[1].as_str().unwrap(), sprite).unwrap();
            match self.compile_block(&input_block, sprite) {
                BlockResult::Nothing => {
                    eprintln!(
                        "[unimplemented block] {} (inside expression: motion_gotoxy)",
                        input_block["opcode"].as_str().unwrap()
                    );
                    input_1 = Value::Number(0.0);
                }
                BlockResult::AllocatedMemory(n) => {
                    input_1 = Value::Pointer(self.register_get_variable_id(n));
                    self.register_free(n);
                }
            }
        } else {
            // todo!()
        }

        // self.instructions.push(Instruction::MotionGoTo((), ()));
        BlockResult::Nothing
    }

    pub fn c_motion_change_x(
        &mut self,
        current_block: &serde_json::Value,
        sprite: &serde_json::Value,
    ) -> BlockResult {
        // self.instructions.push(Instruction::MotionGoTo((), ()));
        BlockResult::Nothing
    }

    pub fn c_motion_change_y(
        &mut self,
        current_block: &serde_json::Value,
        sprite: &serde_json::Value,
    ) -> BlockResult {
        // self.instructions.push(Instruction::MotionGoTo((), ()));
        BlockResult::Nothing
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
