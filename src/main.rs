use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::io::Write;
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
    match machine {
        Ok(mut machine) => {
            writeln!(
                io::stderr(),
                "{machine}
Machine is {}
",
                if machine.is_complete() {
                    "complete"
                } else {
                    "incomplete"
                }
            );
            println!("{}", machine.to_c_editline());
        }
        Err(err) => {
            writeln!(io::stderr(), "Machine syntax error: {:?}", err.message());
        }
    }
}
