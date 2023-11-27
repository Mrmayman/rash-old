use crate::{
    interpreter::DEBUG_OUTPUT,
    machine_state::{MachineState, Value},
};

pub fn memory_dump(state: &mut MachineState) {
    println!("Started Memory Dump {{");
    for i in &state.memory {
        match i {
            Value::Pointer(n) => {
                println!("pointer: {}", n)
            }
            Value::Number(n) => {
                println!("number: {}", n)
            }
            Value::Boolean(n) => {
                println!("bool: {}", n)
            }
            Value::String(n) => {
                println!("string: {}", n)
            }
        }
    }
    println!("}} Finished Memory Dump");
}

pub fn kill_thread(state: &mut MachineState) {
    state.killed = true;
}

pub fn var_set(state: &mut MachineState) {
    let value = state.get_data(1).clone();
    let var = state.get_data(0).clone();
    state.set_mem(&var, &value);
    state.data_counter += 2;
}

pub fn flow_if(state: &mut MachineState) {
    let condition = state.get_bool(state.get_data(0));
    if condition {
        state.instruction_counter = state.get_number(state.get_data(1)) as usize - 2;
        state.data_counter = state.get_number(state.get_data(2)) as usize;
    } else {
        state.data_counter += 3;
    }
    if DEBUG_OUTPUT {
        println!(
            "Jumped to {}, {}",
            state.instruction_counter, state.data_counter
        );
    }
}
