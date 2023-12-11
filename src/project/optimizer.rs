use super::state::ParseState;

impl<'a> ParseState<'a> {
    pub fn optimize(&mut self) {
        self.optimize_flatten_places()
    }

    fn optimize_flatten_places(&mut self) {
        // TODO
    }
}
