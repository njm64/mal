use crate::types::*;
use std::rc::Rc;

struct Reader<'a> {
    data: Vec<&'a str>,
    offset: usize,
}

impl Reader<'_> {
    fn new(data: Vec<&str>) -> Reader {
        Reader { data, offset: 0 }
    }

    fn peek<'a>(&'a mut self) -> MalResult<&'a str> {
        if self.offset >= self.data.len() {
            return Err(MalError::new("EOF"));
        }
        Ok(self.data[self.offset])
    }

    fn next<'a>(&'a mut self) -> MalResult<&'a str> {
        if self.offset >= self.data.len() {
            return Err(MalError::new("EOF"));
        }
        let r = self.data[self.offset];
        self.offset += 1;
        return Ok(r);
    }
}

fn is_special_char(c: char) -> bool {
    return "[]{}()'`~^@".find(c) != None;
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace() || c == ','
}

fn strip_whitespace<'a>(s: &'a str) -> &'a str {
    let mut it = s.char_indices().skip_while(|(_, c)| is_whitespace(*c));
    match it.next() {
        Some((i, _)) => &s[i..],
        None => "",
    }
}

fn next_token<'a>(s: &'a str) -> MalResult<&'a str> {
    let mut it = s.char_indices();

    // Match the first char

    let (_, c) = match it.next() {
        Some(r) => r,
        None => return Ok(""),
    };

    // If it's a tilde, check the following character

    if c == '~' {
        return match it.next() {
            Some((_, '@')) => Ok("~@"),
            _ => Ok("~"),
        };
    }

    // Single special char

    if is_special_char(c) {
        return Ok(&s[..c.len_utf8()]);
    }

    // Quoted string

    if c == '"' {
        let mut esc = false;
        while let Some((i, c)) = it.next() {
            if esc {
                esc = false;
            } else if c == '\\' {
                esc = true;
            } else if c == '"' {
                return Ok(&s[..i + 1]);
            }
        }
        return Err(MalError::new("EOF"));
    }

    // Comment

    if c == ';' {
        return Ok(s);
    }

    // Otherwise, match up until the next whitespace or special char

    while let Some((i, c)) = it.next() {
        if is_whitespace(c) || is_special_char(c) {
            return Ok(&s[..i]);
        }
    }

    // If we didn't find one, the rest of the string is all non-special chars

    return Ok(s);
}

fn tokenize(input: &str) -> MalResult<Vec<&str>> {
    let mut v = Vec::new();
    let mut s = input;
    loop {
        s = strip_whitespace(s);
        match next_token(s) {
            Ok("") => return Ok(v),
            Ok(token) => {
                v.push(token);
                s = &s[token.len()..];
            }
            Err(e) => return Err(e),
        };
    }
}

pub fn read_str(s: &str) -> MalRet {
    let tokens = tokenize(s)?;
    let mut r = Reader::new(tokens);
    read_form(&mut r)
}

fn read_form(r: &mut Reader) -> MalRet {
    match r.peek()? {
        "(" => read_list(r),
        "[" => read_vec(r),
        _ => read_atom(r),
    }
}

fn read_list(r: &mut Reader) -> MalRet {
    let mut vec = Vec::new();
    r.next()?; // Skip the opening bracket
    while r.peek()? != ")" {
        vec.push(read_form(r)?);
    }
    r.next()?; // Skip the closing bracket
    return Ok(MalValue::List(Rc::new(vec)));
}

fn read_vec(r: &mut Reader) -> MalRet {
    let mut vec = Vec::new();
    r.next()?; // Skip the opening bracket
    while r.peek()? != "]" {
        vec.push(read_form(r)?);
    }
    r.next()?; // Skip the closing bracket
    return Ok(MalValue::Vector(Rc::new(vec)));
}

fn read_atom(r: &mut Reader) -> MalRet {
    let token = r.next()?;

    match token {
        "nil" => return Ok(MalValue::Nil),
        "true" => return Ok(MalValue::True),
        "false" => return Ok(MalValue::False),
        _ => (),
    }

    match token.chars().next() {
        Some('"') => {
            return Ok(MalValue::String(parse_string(token)?));
        }

        Some('0'..='9') | Some('-') => {
            if let Ok(i) = token.parse::<i32>() {
                return Ok(MalValue::Int(i));
            }
        }

        _ => (),
    }

    if token.chars().find(|c| is_special_char(*c)) == None {
        return Ok(MalValue::Symbol(token.to_string()));
    }

    return Err(MalError::new("Invalid token"));
}

fn parse_string(s: &str) -> MalResult<String> {
    let mut r = String::new();

    let mut it = s.chars();
    if it.next() != Some('"') {
        return Err(MalError::new("Expected opening quote"));
    }

    let mut esc = false;
    for c in it {
        if esc {
            r.push(match c {
                'n' => '\n',
                '\\' => '\\',
                '"' => '"',
                _ => return Err(MalError::new("Invalid char")),
            });
            esc = false;
        } else if c == '\\' {
            esc = true;
        } else if c == '"' {
            return Ok(r);
        } else {
            r.push(c);
        }
    }

    return Err(MalError::new("EOF"));
}
