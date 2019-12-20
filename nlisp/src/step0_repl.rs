use std::io;
use std::io::Write;

fn read(s: String) -> String {
    return s;
}

fn eval(s: String) -> String {
    return s;
}

fn print(s: String) {
    println!("{}", s);
}

fn rep(s: String) {
    print(eval(read(s)));
}

fn main() {
    loop {
        print!("user> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => rep(line),
            Err(error) => println!("Error: {}", error),
        }
    }
}
