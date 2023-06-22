use std::{str::FromStr, num::ParseFloatError};

fn main() {
    println!("{}", wrap_parsing().unwrap());
}

fn wrap_parsing() -> Result<f32, ParseFloatError> {
    let u_in = get_user_input();
    let u_num = parse_user_input::<f32>(u_in);
    return u_num;
}

fn get_user_input() -> String {
    return String::from("32.0");
}

fn parse_user_input<T: FromStr>(input: String) -> Result<T, <T as FromStr>::Err> {
    return input.parse::<T>();
}
