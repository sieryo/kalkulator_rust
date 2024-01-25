use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

#[derive(Default, Debug)]
struct CalcState {
    preview_value: String,
    calculate_value: String,
    current_value: String,
    reset_value: bool,
}

fn add_operator(operator: &str, state: &mut CalcState) -> () {
    let calculate_operator = if operator == "x" {
        "*".to_string()
    } else {
        operator.to_string()
    };
    state.calculate_value = format!("{}{}{}", state.calculate_value, state.current_value, calculate_operator);
    state.preview_value = format!("{}{} {} ", state.preview_value, state.current_value, operator);
    state.reset_value = true;
}

fn main() {
    let a = "DHAS x".chars().last().unwrap();

    println!("{}", a);
    let app = MainWindow::new().unwrap();
    let weak = app.as_weak();
    let state = Rc::new(RefCell::new(CalcState::default()));

    app.global::<CalcLogic>().on_button_pressed(move |value| {
        let app = weak.unwrap();
        let mut state = state.borrow_mut();

        if state.preview_value == "0" && value != "C" && value != "Back" {
            state.preview_value = value.clone().into();
            state.calculate_value = value.clone().into();
            state.current_value = value.clone().into();
            app.set_value(state.preview_value.clone().into());
            return;
        }

        if let Ok(val) = value.parse::<f32>() {
            if state.reset_value {
                state.current_value = format!("{}", &val.to_string());
                state.reset_value = false;
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
            let results = exmex::eval_str::<f32>(&state.calculate_value).unwrap();
            state.preview_value = format!("{} = ", state.preview_value);
            state.current_value = results.to_string();
        } else if value == "-" {
            add_operator(&value, &mut state);
        } else if value == "+" {
            add_operator(&value, &mut state);
        } else if value == "CE" {
            state.current_value = "0".to_string();
        } else if value == "C" {
            state.calculate_value = "0".to_string();
            state.current_value = "0".to_string();
            state.preview_value = "0".to_string();
        } else if value == "%" {
            state.calculate_value = format!("{}{}", state.calculate_value, &value.to_string());
            state.preview_value = format!("{} % ", state.preview_value);
            state.current_value = "0".to_string();
        } else if value == "Back" {
            if state.preview_value.len() == 0 {
                return;
            }
            let len_value = state.preview_value.len() - 1;
            state.preview_value.remove(len_value);
            state.calculate_value.remove(len_value);
            state.current_value.remove(len_value);
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
    });

    app.run().unwrap();
}
