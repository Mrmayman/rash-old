use crate::{interpreter::Instruction, thread_compiler::thread_compiler_main::ThreadCompiler};

impl<'a> ThreadCompiler<'a> {
    pub fn c_pen_clear(&mut self) -> Option<usize> {
        self.instructions.push(Instruction::PenClear);
        None
    }

    pub fn c_pen_stamp(&mut self) -> Option<usize> {
        self.instructions.push(Instruction::PenStamp);
        None
    }

    pub fn c_pen_up(&mut self) -> Option<usize> {
        self.instructions.push(Instruction::PenUp);
        None
    }

    pub fn c_pen_down(&mut self) -> Option<usize> {
        self.instructions.push(Instruction::PenDown);
        None
    }

    pub fn c_pen_set_size(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        let size = self.register_malloc();
        self.register_set_to_input(current_block, size, "SIZE");
        self.instructions.push(Instruction::PenSetRadius(
            crate::interpreter::Value::Pointer(self.register_get_variable_id(size)),
        ));
        self.register_free(size);
        None
    }
}
