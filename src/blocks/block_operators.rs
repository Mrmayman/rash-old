use crate::{
    ansi_codes,
    interpreter::{Instruction, Value},
    bc_compiler::bc_comp_main::ThreadCompiler,
};

impl<'a> ThreadCompiler<'a> {
    fn get_operator_registers(&mut self, current_block: &serde_json::Value) -> (usize, usize) {
        let register1 = self.register_malloc();
        let register2 = self.register_malloc();

        self.register_set_to_input(current_block, register1, "NUM1");
        self.register_set_to_input(current_block, register2, "NUM2");
        (register1, register2)
    }

    pub fn c_operators_add(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorAdd(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        Some(register1)
    }

    pub fn c_operators_subtract(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorSubtract(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        Some(register1)
    }

    pub fn c_operators_multiply(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorMultiply(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        Some(register1)
    }

    pub fn c_operators_divide(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorDivide(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        Some(register1)
    }

    pub fn c_operators_mod(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let (register1, register2) = self.get_operator_registers(current_block);

        self.instructions.push(Instruction::OperatorModulo(
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register1)),
            Value::Pointer(self.register_get_variable_id(register2)),
        ));

        self.register_free(register2);
        Some(register1)
    }

    pub fn c_operators_greater(&mut self, current_block: &serde_json::Value) -> Option<usize> {
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
        Some(register1)
    }

    pub fn c_operators_lesser(&mut self, current_block: &serde_json::Value) -> Option<usize> {
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
        Some(register1)
    }

    pub fn c_operators_equals(&mut self, current_block: &serde_json::Value) -> Option<usize> {
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
        Some(register1)
    }

    pub fn c_operators_mathop(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let register = self.register_malloc();
        let num_register = self.register_malloc();
        self.register_set_to_input(current_block, num_register, "NUM");
        let operator = current_block["fields"]["OPERATOR"].as_array().unwrap()[0]
            .as_str()
            .unwrap();
        match operator {
            "e ^" => self.instructions.push(Instruction::OperatorERaised(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "10 ^" => self.instructions.push(Instruction::OperatorPower(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Number(10.0),
                Value::Pointer(num_register),
            )),
            "sin" => self.instructions.push(Instruction::OperatorSin(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "cos" => self.instructions.push(Instruction::OperatorCos(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "tan" => self.instructions.push(Instruction::OperatorTan(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "abs" => self.instructions.push(Instruction::OperatorAbs(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "sqrt" => self.instructions.push(Instruction::OperatorSqrt(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "asin" => self.instructions.push(Instruction::OperatorASin(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "acos" => self.instructions.push(Instruction::OperatorACos(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "ln" => self.instructions.push(Instruction::OperatorLn(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "log" => self.instructions.push(Instruction::OperatorLog(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "floor" => self.instructions.push(Instruction::OperatorFloor(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
            )),
            "ceiling" => self.instructions.push(Instruction::OperatorCeiling(
                Value::Pointer(self.register_get_variable_id(register)),
                Value::Pointer(num_register),
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
        self.register_free(num_register);
        Some(register)
    }
}
