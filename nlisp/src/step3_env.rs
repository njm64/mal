use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::rc::Rc;

mod printer;
mod reader;
mod types;
mod env;

use types::*;
use env::MalEnv;

fn read(s: String) -> MalRet {
    reader::read_str(s.as_str())
}

fn arith(args: &[MalValue], f: &dyn Fn(i32, i32) -> i32) -> MalRet {
    if args.len() < 1 {
        return Err(MalError::new("Not enough arguments"));
    }

    let mut r = args[0].as_int()?;
    for arg in args[1..].iter() {
        r = f(r, arg.as_int()?)
    }
    Ok(MalValue::Int(r))
}

fn add(args: &[MalValue]) -> MalRet {
    arith(args, &|a, b| a + b)
}

fn sub(args: &[MalValue]) -> MalRet {
    arith(args, &|a, b| a - b)
}

fn mul(args: &[MalValue]) -> MalRet {
    arith(args, &|a, b| a * b)
}

fn div(args: &[MalValue]) -> MalRet {
    arith(args, &|a, b| a / b)
}

fn make_env() -> MalEnv {
    let mut e = MalEnv::new(None);
    e.set("+", MalValue::Function(add));
    e.set("-", MalValue::Function(sub));
    e.set("*", MalValue::Function(mul));
    e.set("/", MalValue::Function(div));
    return e;
}

fn eval_ast(v: &MalValue, env: &MalEnv) -> MalRet {
    match v {
        MalValue::Symbol(s) => Ok(env.get(s).ok_or(MalError::new("Unknown symbol"))?.clone()),

        MalValue::List(l) => {
            let mut r = Vec::new();
            for item in l.iter() {
                r.push(eval(item, env)?);
            }
            Ok(MalValue::List(Rc::new(r)))
        }

        x => Ok(x.clone()),
    }
}

fn def(env: &MalEnv, args: &[MalValue]) -> MalRet {

    if args.len() != 2 {
        return Err(MalError::new("Expected 2 arguments to def!"));
    }

    let symbol = args[0].as_symbol()?;
    let value = eval(&args[1], env)?;
    env.set(symbol.as_str(), value);
    return Ok(MalValue::Nil)
}

fn eval(v: &MalValue, env: &MalEnv) -> MalRet {

    // If it's not a list, just call eval_ast on it

    let list = match v {
        MalValue::List(l) => l,
        _ => return eval_ast(v, &env)
    };

    // If it's an empty list, just return it

    if list.len() == 0 {
        return Ok(v.clone());
    }

    // If it's a symbol, check for a special form
    
    if let MalValue::Symbol(symbol) = &list[0] {
        match symbol.as_str() {
            "def!" => return def(env, &list[1..]),
            _ => ()
        }
    }
    
    // Evaluate the list. It should always evaluate to a list 
    // of the same length.
    
    let evaluated_list = eval_ast(v, &env)?.as_list()?;
    let function = evaluated_list[0].as_function()?;
    return function(&evaluated_list[1..]);
}

fn print(v: &MalValue) -> String {
    printer::pr_str(&v, true)
}

fn rep(s: String, env: &MalEnv) -> MalResult<String> {
    let input = read(s)?;
    let output = eval(&input, env)?;
    Ok(print(&output))
}

fn main() {
    let env = make_env();

    loop {
        print!("user> ");
        let _ = io::stdout().flush();
        let mut line = String::new();

        if let Err(error) = io::stdin().read_line(&mut line) {
            println!("Error: {}", error);
            continue;
        }

        match rep(line, &env) {
            Ok(output) => {
                println!("{}", output);
            }
            Err(err) => {
                println!("Error: {}", err.s);
            }
        }
    }
}
