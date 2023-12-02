use crate::{
    ansi_codes,
    interpreter::{Instruction, Value},
    project::{base::BlockResult, state::ParseState},
};

impl<'a> ParseState<'a> {
    fn get_operator_registers(&mut self, current_block: &serde_json::Value) -> (usize, usize) {
        let register1 = self.register_malloc();
        let register2 = self.register_malloc();

        self.register_set_to_input(current_block, register1, "NUM1");
        self.register_set_to_input(current_block, register2, "NUM2");
        (register1, register2)
    }

    pub fn c_operators_add(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorAdd(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        BlockResult::AllocatedMemory(register1)
    }

    pub fn c_operators_subtract(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorSubtract(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        BlockResult::AllocatedMemory(register1)
    }

    pub fn c_operators_multiply(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorMultiply(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        BlockResult::AllocatedMemory(register1)
    }

    pub fn c_operators_divide(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorDivide(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        BlockResult::AllocatedMemory(register1)
    }

    pub fn c_operators_mod(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorModulo(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        BlockResult::AllocatedMemory(register1)
    }

    pub fn c_operators_greater(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let register1 = self.register_malloc();
        let register2 = self.register_malloc();

        self.register_set_to_input(current_block, register1, "OPERAND1");
        self.register_set_to_input(current_block, register2, "OPERAND2");

        self.instructions.push(Instruction::OperatorGreater(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        BlockResult::AllocatedMemory(register1)
    }

    pub fn c_operators_lesser(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let register1 = self.register_malloc();
        let register2 = self.register_malloc();

        self.register_set_to_input(current_block, register1, "OPERAND1");
        self.register_set_to_input(current_block, register2, "OPERAND2");

        self.instructions.push(Instruction::OperatorLesser(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        BlockResult::AllocatedMemory(register1)
    }

    pub fn c_operators_equals(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let register1 = self.register_malloc();
        let register2 = self.register_malloc();

        self.register_set_to_input(current_block, register1, "OPERAND1");
        self.register_set_to_input(current_block, register2, "OPERAND2");

        self.instructions.push(Instruction::OperatorEquals(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        BlockResult::AllocatedMemory(register1)
    }

    pub fn c_operators_mathop(&mut self, current_block: &serde_json::Value) -> BlockResult {
        let register = self.register_malloc();
        let input = self.input_get_number(current_block, "NUM");
        let operator = current_block["fields"]["OPERATOR"].as_array().unwrap()[0]
            .as_str()
            .unwrap();
        match operator {
            "e ^" => self.instructions.push(Instruction::OperatorERaised(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "10 ^" => self.instructions.push(Instruction::OperatorPower(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Number(10.0),
                input,
            )),
            "sin" => self.instructions.push(Instruction::OperatorSin(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "cos" => self.instructions.push(Instruction::OperatorCos(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "tan" => self.instructions.push(Instruction::OperatorTan(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "abs" => self.instructions.push(Instruction::OperatorAbs(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "sqrt" => self.instructions.push(Instruction::OperatorSqrt(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "asin" => self.instructions.push(Instruction::OperatorASin(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "acos" => self.instructions.push(Instruction::OperatorACos(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "ln" => self.instructions.push(Instruction::OperatorLn(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "log" => self.instructions.push(Instruction::OperatorLog(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "floor" => self.instructions.push(Instruction::OperatorFloor(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            "ceiling" => self.instructions.push(Instruction::OperatorCeiling(
                Value::Pointer(self.register_get_variable_id(register)),
                input,
            )),
            _ => {
                eprintln!(
                    "{}[unimplemented mathop]{} {}",
                    ansi_codes::RED,
                    ansi_codes::RESET,
                    operator
                )
            }
        }
        BlockResult::AllocatedMemory(register)
    }
}
