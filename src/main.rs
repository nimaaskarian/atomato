use std::ffi::OsString;
use std::fs;
use std::env;
use std::str::FromStr;
mod machine;
use machine::Machine;

fn main() {
    let args: Vec<OsString> = env::args_os().collect();
    let machine = Machine::from_str(
        fs::read_to_string(args[1].to_str().unwrap())
            .unwrap()
            .as_str(),
    );
    
    println!("{}", machine.unwrap().to_c());
}
