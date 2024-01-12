use crate::{
    interpreter::{Instruction, Value},
    thread_compiler::thread_compiler_main::ThreadCompiler,
};

impl<'a> ThreadCompiler<'a> {
    pub fn c_sensing_timer(&mut self) -> Option<usize> {
        let register = self.register_malloc();
        self.instructions
            .push(Instruction::SensingTimer(Value::Pointer(
                self.register_get_variable_id(register),
            )));
        Some(register)
    }
}
