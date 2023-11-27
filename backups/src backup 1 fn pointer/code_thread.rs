use crate::machine_state::{MachineState, Value};

// In Rash, each instruction is represented as a function pointer.
// Basically a giant list of functions that are looped through.
// Each instruction has its own function.

pub struct CodeThread {
    pub state: MachineState,
    // This Vec (resizeable list) stores the function pointers.
    instructions: Vec<fn(&mut MachineState) -> ()>,
}

impl CodeThread {
    pub fn new(instructions: Vec<fn(&mut MachineState) -> ()>, data: Vec<Value>) -> CodeThread {
        CodeThread {
            state: MachineState::new(data),
            instructions,
        }
    }

    pub fn run(&mut self) {
        loop {
            // Run an instruction.
            // Runs the function pointer (Notice the brackets at the end?).
            self.instructions[self.state.instruction_counter as usize](&mut self.state);
            // Go to next instruction.
            self.state.instruction_counter += 1;

            // If ready to draw frame to screen, stop.
            if self.state.paused {
                self.state.paused = false;
                break;
            }
            // If code has finished running, stop.
            if self.state.killed {
                break;
            }
        }
    }
}
