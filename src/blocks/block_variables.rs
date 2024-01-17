use crate::{
    interpreter::{Instruction, Value},
    bc_compiler::bc_comp_main::ThreadCompiler,
};

impl<'a> ThreadCompiler<'a> {
    pub fn c_variables_set(&mut self, current_block: &serde_json::Value) -> Option<usize> {
        // Get the internal name of the variable.
        let var_name = current_block["fields"]["VARIABLE"].as_array().unwrap()[1]
            .as_str()
            .unwrap();
        // Get the id of the variable in the Rash VM.
        let id: usize = self.variables.get_id(var_name).unwrap();

        let register = self.register_malloc();
        self.register_set_to_input(current_block, register, "VALUE");
        self.instructions.push(Instruction::MemoryStore(
            Value::Pointer(id),
            Value::Pointer(self.register_get_variable_id(register)),
        ));
        self.register_free(register);

        None
    }
}
