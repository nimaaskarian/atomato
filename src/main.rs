use clap::Parser;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;
use std::{fs, io, path};
mod machine;
use clap::{Command, CommandFactory};
use clap_complete::Shell;
use clap_complete::{generate, Generator};
use machine::Machine;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// A simple compiler (to-C compiler) for a simple DSL for finite-state-automatons
struct Args {
    /// Output C code in plain C mode. editline mode depends on editline/readline.h and unistd.h,
    /// plain C only depends on string.h and stdio.h
    #[arg(short = 'e', long)]
    plain_c: bool,

    /// Path to atomato file. - means stdin
    #[arg(default_value = "-")]
    path: path::PathBuf,

    /// make a Makefile and a .c file (inside the `path`'s directory)
    #[arg(short = 'm', long)]
    makefile: bool,

    /// don't make a .c file when creating the makefile
    #[arg(long, requires="makefile")]
    no_c_file: bool,

    /// Generate completion for a certain shell
    #[arg(short = 'c', long)]
    completion: Option<Shell>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    if let Some(generator) = args.completion {
        print_completions(generator, &mut Args::command());
        return Ok(());
    }
    let content = if args.path.to_str().unwrap() == "-" {
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
        lines.join("\n")
    } else {
        fs::read_to_string(&args.path)
            .expect("couldn't read the file. premission issue or doesn't exist.")
    };
    let machine = Machine::from_str(&content);
    match machine {
        Ok(mut machine) => {
            let c_code_function = if args.plain_c {
                Machine::to_c
            } else {
                Machine::to_c_editline
            };
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
            )?;
            if args.makefile {
                if let Some(stem) = args.path.file_stem() {
                    let filename = args.path.file_name().unwrap();
                    let dir = args.path.parent().unwrap();
                    let c_file = format!("{}.c", stem.to_str().unwrap());
                    if !args.no_c_file {
                        if let Ok(mut file) = File::create(dir.join(&c_file)) {
                            writeln!(file, "{}", c_code_function(&machine));
                        }
                    }

                    if let Ok(mut file) = File::create(dir.join("Makefile")) {
                        writeln!(file, "{}", gen_makefile(filename, stem, c_file, args.plain_c));
                    }
                }
            } else {
                println!("{}", c_code_function(&machine));
            }
        }
        Err(err) => {
            writeln!(io::stderr(), "Machine syntax error: {:?}", err.message())?;
        }
    }
    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn gen_makefile(filename: &OsStr, stem: &OsStr, c_file: String, plain_c: bool) -> String {
    let stem = stem.to_str().unwrap();
    let filename = filename.to_str().unwrap();
    let mut cflags = "";
    if !plain_c {
        cflags = "CFLAGS := -leditline";
    }
    format!(
        "{cflags}
run: {stem}
\t./{stem}
{stem}: {c_file}
\t$(CC) $(CFLAGS) $< -o $@

{c_file}: {filename}
\tatomato $< > $@
clean:
\trm {stem} {c_file}
"
    )
}
