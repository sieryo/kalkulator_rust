use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

#[derive(Default, Debug)]
struct CalcState {
    preview_value: String,
    calculate_value: String,
    current_value: String,
    reset_value: bool,
    can_delete_value: bool,
    reset_input: bool,
}

fn add_operator(operator: &str, state: &mut CalcState) -> () {
    state.can_delete_value = false;
    let calculate_operator = if operator == "x" {
        "*".to_string()
    } else {
        operator.to_string()
    };
    state.calculate_value = format!(
        "{}{}{}",
        state.calculate_value, state.current_value, calculate_operator
    );
    if state.preview_value.contains("=") {
        state.preview_value = format!("{} {}", state.current_value, operator);
    } else {
        state.preview_value = format!(
            "{}{} {} ",
            state.preview_value, state.current_value, operator
        );
    }
    state.reset_value = true;
    state.reset_input = false;
}

fn reset_input(current_value: &str, state: &mut CalcState) -> () {
    state.calculate_value = "".to_string();
    state.current_value = current_value.to_string();
    state.preview_value = "".to_string();
    state.reset_input = false;

    println!("FUNGSI BEKERJA!");
}
fn calculate_result(state: &mut CalcState) -> () {
    let results = exmex::eval_str::<f32>(&state.calculate_value).unwrap();
    state.preview_value = format!("{}=", state.preview_value);
    state.current_value = results.to_string();
    state.calculate_value = "".to_string();
    state.reset_input = true;
}
fn main() {
    let app = MainWindow::new().unwrap();
    let weak = app.as_weak();
    let state = Rc::new(RefCell::new(CalcState::default()));

    app.global::<CalcLogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();
        let mut state = state.borrow_mut();

        if let Ok(val) = value.parse::<f32>() {
            state.can_delete_value = true;
            if state.reset_input {
                println!("MEMASUKI IF STATEMENT INI");
                reset_input(&value, &mut state);
                state.reset_value = false;
            } else if state.reset_value {
                state.current_value = format!("{}", &val.to_string());
                state.reset_value = false;
                state.reset_input = false;
            } else {
                state.current_value = format!("{}{}", state.current_value, &val.to_string());
            }
        } else if value == "x" {
            add_operator(&value, &mut state);
        } else if value == "." {
            state.calculate_value = format!("{}{}", state.calculate_value, &value.to_string());
            state.preview_value = format!("{}.", state.preview_value);
            state.current_value = format!("{}.", state.preview_value);
        } else if value == "÷" {
            add_operator(&value, &mut state);
        } else if value == "=" {
            add_operator("", &mut state);
            calculate_result(&mut state);
        } else if value == "-" {
            add_operator(&value, &mut state);
        } else if value == "+" {
            add_operator(&value, &mut state);
        } else if value == "CE" {
            if state.can_delete_value {
                state.current_value = "0".to_string();
                state.reset_value = true;
            } else {
                return;
            }
        } else if value == "C" {
            reset_input("0", &mut state);
            state.reset_value = true;
        } else if value == "%" {
            state.calculate_value = format!("{}{}", state.calculate_value, &value.to_string());
            state.preview_value = format!("{} % ", state.preview_value);
            state.current_value = "0".to_string();
        } else if value == "Back" {
            if state.can_delete_value {
                if state.current_value.len() == 1 {
                    state.current_value = "0".to_string();
                    app.set_input_value(state.current_value.clone().into());
                    state.reset_value = true;
                    return;
                }
                let len_value = state.current_value.len() - 1;
                state.current_value.remove(len_value);
            } else {
                return;
            }
        }

        app.set_value(state.preview_value.clone().into());
        app.set_input_value(state.current_value.clone().into());

        println!(
            "CALCULATE VALUE (TIDAK AKAN ADA DI UI) : {:?}",
            state.calculate_value
        );
        println!(
            "PREVIEW VALUE (DIATAS CURRENT VALUE) : {:?}",
            state.preview_value
        );
        println!(
            "CURRENT VALUE (TEXT PALING BESAR) : {:?}",
            state.current_value
        );
        println!("KEADAAN RESET INPUT : {}", state.reset_input);
    });

    app.run().unwrap();
}
