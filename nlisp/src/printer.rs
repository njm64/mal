use super::types::*;

pub fn pr_str(value: &MalValue, print_readably: bool) -> String {
    match value {
        MalValue::Nil => String::from("nil"),
        MalValue::True => String::from("true"),
        MalValue::False => String::from("false"),
        MalValue::Int(i) => i.to_string(),
        MalValue::Symbol(s) => s.clone(),
        MalValue::List(list) => pr_list(&list, print_readably),
        MalValue::Vector(vec) => pr_vector(&vec, print_readably),
        MalValue::String(s) => pr_string(s.as_str(), print_readably),
        MalValue::Function(_) => String::from("<function>"),
    }
}

fn pr_list(list: &Vec<MalValue>, print_readably: bool) -> String {
    let mut s = String::from("(");

    let mut delim = "";
    for value in list.iter() {
        s.push_str(delim);
        s.push_str(pr_str(value, print_readably).as_str());
        delim = " ";
    }

    s.push_str(")");
    return s;
}

fn pr_vector(list: &Vec<MalValue>, print_readably: bool) -> String {
    let mut s = String::from("[");

    let mut delim = "";
    for value in list.iter() {
        s.push_str(delim);
        s.push_str(pr_str(value, print_readably).as_str());
        delim = " ";
    }

    s.push_str("]");
    return s;
}

fn pr_string(s: &str, print_readably: bool) -> String {
    let mut r = String::new();

    if !print_readably {
        r.push_str(s);
        return r;
    }

    r.push('"');
    for c in s.chars() {
        match c {
            '\n' => r.push_str("\\n"),
            '\\' => r.push_str("\\\\"),
            '"' => r.push_str("\\\""),
            _ => r.push(c),
        }
    }
    r.push('"');
    return r;
}
