use std::collections::HashMap;
use std::error::Error;
use std::fmt;
// use std::Vec;

#[derive(Debug)]
pub struct SyntaxError {
    msg: String
}

impl SyntaxError {
    fn new(msg: &str, lineno: usize) -> SyntaxError {
        SyntaxError { msg: format!("{} at line {}", msg, lineno) }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error for SyntaxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}


pub fn parse_script(text: &str) -> Result<HashMap<u32, String>, SyntaxError> {
    let mut result = HashMap::new();
    for (mut lineno, line) in text.lines().enumerate() {
        lineno += 1;
        let items: Vec<_> = line.splitn(2, " ").collect();
        let line_idx = items.get(0).ok_or(SyntaxError::new("invalid line", lineno))?;
        let text = items.get(1).ok_or(SyntaxError::new("invalid line", lineno))?;
        result.insert(line_idx.parse::<u32>().map_err(|_| SyntaxError::new(
            &format!("could not parse line index as string: {:?}", line_idx), lineno))?, text.to_string());
    }
    Ok(result)
}


pub fn tokenize(line: &str) -> Vec<String> {
    line.split(" ").map(&str::to_string).collect()
}


pub fn interpret_program(program: HashMap<u32, String>) {

    let mut ip: u32 = match program.keys().min() {
        Some(n) => *n,
        None => return
    };

    loop {
        let cmd = program.get(&ip).unwrap();
        let items = tokenize(cmd);
        match items.get(0).map(String::as_str) {
            Some("GOTO") => {
                let target_str = items.get(1).expect(&format!("invalid GOTO with no target at {}", ip));
                let target = target_str.parse::<u32>().expect(&format!("Invalid jump target {} at {}", target_str, ip));
                match program.get(&target) {
                    Some(_) => { ip = target; },
                    None => panic!("No such line {} at line {} GOTO", target, ip)
                }
            },
            Some("PRINT") => {
                let msg = items.get(1).expect(&format!("invalid PRINT with no msg at {}", ip));
                println!("{}", msg)
            },
            Some(unknown) => panic!("Invalid operation {}", unknown),
            None => panic!("No such line {}", ip)
        }
    }


}