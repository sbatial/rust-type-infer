use std::{num::ParseFloatError, str::FromStr};

use rand::{thread_rng, Rng};

/// .
///
/// # Panics
///
/// Panics if .
fn main() {
    let num = wrap_parsing();

    if num.is_ok() {
        println!("{}", num.unwrap());
    } else {
        println!("Input could not be parsed properly!");
    }

    let num = wrap_parsing();

    if num.is_ok() {
        println!("{}", num.unwrap());
    } else {
        println!("Input could not be parsed properly!");
    }
}

/// .
///
/// # Errors
///
/// This function will return an error if .
fn wrap_parsing() -> Result<f32, ParseFloatError> {
    let mut rng = thread_rng();
    let u_in;

    if rng.gen::<usize>() % 100 > 50 {
        u_in = get_user_input();
    } else {
        u_in = get_faulty_user_input();
    }

    let u_num = parse_user_input::<f32>(u_in);
    return u_num;
}

/// .
fn get_user_input() -> String {
    return String::from("32.0");
}

fn get_faulty_user_input() -> String {
    // return String::from("32..0");
    return String::from("32..0");
}

/// .
///
/// # Errors
///
/// This function will return an error if .
fn parse_user_input<T: FromStr>(input: String) -> Result<T, <T as FromStr>::Err> {
    return input.parse::<T>();
}
