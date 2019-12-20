use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::rc::Rc;

mod printer;
mod reader;
mod types;

use types::*;

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
    let mut m = HashMap::new();
    m.insert("+".to_string(), MalValue::Function(add));
    m.insert("-".to_string(), MalValue::Function(sub));
    m.insert("*".to_string(), MalValue::Function(mul));
    m.insert("/".to_string(), MalValue::Function(div));
    return m;
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

fn eval(v: &MalValue, env: &MalEnv) -> MalRet {
    if let MalValue::List(l) = v {
        if l.len() == 0 {
            return Ok(v.clone());
        } else {
            let v2 = eval_ast(v, &env)?;

            if let MalValue::List(l2) = v2 {
                if let MalValue::Function(f) = l2[0] {
                    return f(&l2[1..]);
                }
            }

            return Err(MalError::new("Uh oh"));
        }
    } else {
        return eval_ast(v, &env);
    }
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
