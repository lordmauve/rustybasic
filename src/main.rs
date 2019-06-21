use std::io::BufReader;
use std::error::Error;
use basic::{parse_script, interpret_program, SyntaxError};


fn main() -> Result<(), SyntaxError> {
    println!("Hello, world!");

    //println!("cwd: {:?}", std::env::current_dir().unwrap());
    let file_as_string = std::fs::read_to_string("src/example.bas").unwrap();

    let program = parse_script(&file_as_string)?;
    interpret_program(program);

    // let f = std::fs::File::open("example.bas").unwrap();
    // let mut file = BufReader::new(&f);
    // for line in file.lines() {
    //     let l = line.unwrap();
        
    // }
    Ok((
        
    ))
}

