use super::thread_compiler_main::ThreadCompiler;

impl<'a> ThreadCompiler<'a> {
    pub fn optimize(&mut self) {
        self.optimize_flatten_places()
    }

    fn optimize_flatten_places(&mut self) {
        // TODO: Implement compiler optimization
    }
}
