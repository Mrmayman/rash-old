use crate::{
    interpreter::DEBUG_OUTPUT,
    machine_state::{MachineState, Value},
};

pub fn modulo(state: &mut MachineState) {
    let result_ptr = state.get_data(0).clone();
    let op_a = state.get_number(state.get_data(1));
    let op_b = state.get_number(state.get_data(2));
    if DEBUG_OUTPUT {
        println!("{} % {} = {}", op_a, op_b, op_a % op_b);
    }
    let mut result = op_a % op_b;
    if result < 0.0 {
        result += op_b;
    }
    state.set_mem(&result_ptr, &Value::Number(result));
    state.data_counter += 3;
}

pub fn mult(state: &mut MachineState) {
    let result_ptr = state.get_data(0).clone();
    let op_a = state.get_number(state.get_data(1));
    let op_b = state.get_number(state.get_data(2));
    if DEBUG_OUTPUT {
        println!("{} * {} = {}", op_a, op_b, op_a * op_b);
    }
    state.set_mem(&result_ptr, &Value::Number(op_a * op_b));
    state.data_counter += 3;
}

pub fn div(state: &mut MachineState) {
    let result_ptr = state.get_data(0).clone();
    let op_a = state.get_number(state.get_data(1));
    let op_b = state.get_number(state.get_data(2));
    if DEBUG_OUTPUT {
        println!("{} / {} = {}", op_a, op_b, op_a / op_b);
    }
    state.set_mem(&result_ptr, &Value::Number(op_a / op_b));
    state.data_counter += 3;
}

pub fn add(state: &mut MachineState) {
    let result_ptr = state.get_data(0).clone();
    let op_a = state.get_number(state.get_data(1));
    let op_b = state.get_number(state.get_data(2));
    if DEBUG_OUTPUT {
        println!("{} + {} = {}", op_a, op_b, op_a + op_b);
    }
    state.set_mem(&result_ptr, &Value::Number(op_a + op_b));
    state.data_counter += 3;
}

pub fn sub(state: &mut MachineState) {
    let result_ptr = state.get_data(0).clone();
    let op_a = state.get_number(state.get_data(1));
    let op_b = state.get_number(state.get_data(2));
    if DEBUG_OUTPUT {
        println!("{} - {} = {}", op_a, op_b, op_a - op_b);
    }
    state.set_mem(&result_ptr, &Value::Number(op_a - op_b));
    state.data_counter += 3;
}

pub fn lesser(state: &mut MachineState) {
    let result_ptr = state.get_data(0).clone();
    let op_a = state.get_number(state.get_data(1));
    let op_b = state.get_number(state.get_data(2));
    if DEBUG_OUTPUT {
        println!("{} < {} = {}", op_a, op_b, op_a < op_b);
    }
    state.set_mem(&result_ptr, &Value::Boolean(op_a < op_b));
    state.data_counter += 3;
}

pub fn greater(state: &mut MachineState) {
    let result_ptr = state.get_data(0).clone();
    let op_a = state.get_number(state.get_data(1));
    let op_b = state.get_number(state.get_data(2));
    if DEBUG_OUTPUT {
        println!("{} > {} = {}", op_a, op_b, op_a > op_b);
    }
    state.set_mem(&result_ptr, &Value::Boolean(op_a > op_b));
    state.data_counter += 3;
}
