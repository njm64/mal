use std::io;
use std::io::Write;

mod printer;
mod reader;
mod types;

use types::*;

fn read(s: String) -> MalRet {
    reader::read_str(s.as_str())
}

fn eval(v: &MalValue) -> MalRet {
    return Ok(v.clone());
}

fn print(v: &MalValue) -> String {
    printer::pr_str(&v, true)
}

fn rep(s: String) -> MalResult<String> {
    let input_value = read(s)?;
    let output_value = eval(&input_value)?;
    Ok(print(&output_value))
}

fn main() {
    loop {
        print!("user> ");
        let _ = io::stdout().flush();
        let mut line = String::new();

        if let Err(error) = io::stdin().read_line(&mut line) {
            println!("Error: {}", error);
            continue;
        }

        match rep(line) {
            Ok(output) => {
                println!("{}", output);
            }
            Err(err) => {
                println!("Error: {}", err.s);
            }
        }
    }
}
