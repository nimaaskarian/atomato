use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::mem;
use std::str::FromStr;

#[derive(Default, Debug, PartialEq)]
pub struct Machine {
    inputs: Vec<String>,
    outputs: Vec<String>,
    states: Vec<String>,
    ostates: Vec<String>,
    i_count_map: HashMap<String, usize>,
    o_count_map: HashMap<String, usize>,
    s_count_map: HashMap<String, usize>,
    os_count_map: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct MachineError {
    message: String,
    line: usize,
    offset: usize,
}

impl MachineError {
    pub fn message(&self) -> String {
        format!(
            "{} at line {} (column {})",
            self.message, self.line, self.offset
        )
    }
}

enum FromStrState {
    State,
    Input,
    Ostate,
    Output,
}

impl Machine {
    pub fn to_c(&self) -> String {
        format!(
            "#include <stdio.h>
#include <string.h>

static char * states[] = {{
 {}, NULL
}};

static char * inputs[] = {{
 {}, NULL
}};

static char * ostates[] = {{
 {}, NULL
}};

static char * outputs[] = {{
 {}, NULL
}};

void process_input(const char *input, size_t len) {{
  char * state = \"{}\";
  size_t strindex = 0;
  while(strindex < len) {{
    for (int i = 0; inputs[i] != NULL; i++) {{
      size_t step = strlen(inputs[i]);
      if (strncmp(inputs[i], input, step) == 0 &&
        strcmp(state, states[i]) == 0) {{
        fprintf(stderr, \"%s -> %s\\n\", state, ostates[i]);
        state = ostates[i];
        printf(\"%s\", outputs[i]);
        strindex+=step;
        input+=step;
        break;
      }}
    }}
  }}
}}

int main(int argc, char *argv[]) {{
  char line[256];          
  while (fgets(line, sizeof(line), stdin) != NULL) {{
    size_t size = strcspn(line, \"\\n\");
    line[size] = 0;
    process_input(line, size);
    puts(\"\");
  }}

  return 0;
}}",
            arrayize(&self.states),
            arrayize(&self.inputs),
            arrayize(&self.ostates),
            arrayize(&self.outputs),
            self.states[0]
        )
    }
    pub fn to_c_editline(&self) -> String {
        format!(
            "#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <editline/readline.h>

static char * states[] = {{
 {}, NULL
}};

static char * inputs[] = {{
 {}, NULL
}};

static char * ostates[] = {{
 {}, NULL
}};

static char * outputs[] = {{
 {}, NULL
}};

void process_input(const char *input, size_t len) {{
  char * state = \"{}\";
  size_t strindex = 0;
  while(strindex < len) {{
    for (int i = 0; inputs[i] != NULL; i++) {{
      size_t step = strlen(inputs[i]);
      if (strncmp(inputs[i], input, step) == 0 &&
        strcmp(state, states[i]) == 0) {{
        fprintf(stderr, \"%s -> %s\\n\", state, ostates[i]);
        state = ostates[i];
        printf(\"%s\", outputs[i]);
        strindex+=step;
        input+=step;
        break;
      }}
    }}
  }}
}}

int main(int argc, char *argv[]) {{
  if (isatty(STDIN_FILENO)) {{
    while (1) {{
        char * line = readline(\"> \");
        process_input(line, strlen(line));
        puts(\"\");
    }}
  }} else {{
    char line[256];          
    while (fgets(line, sizeof(line), stdin) != NULL) {{
      size_t size = strcspn(line, \"\\n\");
      line[size] = 0;
      process_input(line, size);
      puts(\"\");
    }}
  }}
  return 0;
}}",
            arrayize(&self.states),
            arrayize(&self.inputs),
            arrayize(&self.ostates),
            arrayize(&self.outputs),
            self.states[0]
        )
    }

    fn update_count_maps(&mut self) {
        if self.i_count_map.is_empty() {
            self.i_count_map = count_map(&self.inputs);
            self.o_count_map = count_map(&self.outputs);
            self.s_count_map = count_map(&self.states);
            self.os_count_map = count_map(&self.ostates);
        }
    }

    pub fn is_complete(&mut self) -> bool {
        self.update_count_maps();
        let input_count = self.i_count_map.len();
        for (s, count) in self.s_count_map.iter() {
            if *count != input_count {
                dbg!(s, count, input_count);
                return false;
            }
        }
        true
    }
}

fn count_map<T>(arr: &[T]) -> HashMap<T, usize>
where
    T: Eq,
    T: Hash,
    T: Clone,
{
    let mut counts = HashMap::with_capacity(arr.len());
    for item in arr {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
}

fn arrayize(arr: &[String]) -> String {
    let quoted: Vec<String> = arr.iter().map(|s| format!("\"{}\"", s)).collect();
    quoted.join(", ")
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.states.len() {
            writeln!(
                f,
                "{}, {} > {}, {}",
                self.states[i], self.inputs[i], self.ostates[i], self.outputs[i]
            )?
        }
        Ok(())
    }
}

impl FromStr for Machine {
    type Err = MachineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = String::new();
        let mut machine = Machine::default();
        let mut temp_machine = Machine::default();
        let mut state = FromStrState::State;
        let mut offset: usize = 0;
        let mut line: usize = 1;
        for ch in s.chars() {
            match ch {
                ' ' | '\t' | '\r' => {}
                '\n' => match state {
                    FromStrState::Output => {
                        for _ in 1..temp_machine.inputs.len() {
                            temp_machine.ostates.push(temp_machine.ostates[0].clone())
                        }
                        machine.states.push(temp_machine.states.pop().unwrap());
                        machine.outputs.push(mem::take(&mut buf));
                        machine.ostates.push(temp_machine.ostates.pop().unwrap());
                        machine.inputs.append(&mut temp_machine.inputs);
                        state = FromStrState::State;
                        offset = 0;
                        line += 1;
                    }
                    _ => {
                        return Err(MachineError {
                            message: "Didn't expect newline".to_string(),
                            line,
                            offset,
                        })
                    }
                },
                ',' => match state {
                    FromStrState::State => {
                        temp_machine.states.push(mem::take(&mut buf));
                        state = FromStrState::Input;
                    }
                    FromStrState::Input => {
                        temp_machine.inputs.push(mem::take(&mut buf));
                        temp_machine
                            .states
                            .push(temp_machine.states.last().unwrap().clone());
                    }
                    FromStrState::Ostate => {
                        temp_machine.ostates.push(mem::take(&mut buf));
                        for _ in 1..temp_machine.inputs.len() {
                            temp_machine
                                .ostates
                                .push(temp_machine.ostates.last().unwrap().clone())
                        }
                        state = FromStrState::Output;
                    }
                    FromStrState::Output => {
                        return Err(MachineError {
                            message: "Expected newline. Got ,".to_string(),
                            line,
                            offset,
                        })
                    }
                },
                '>' => match state {
                    FromStrState::Input => {
                        temp_machine.inputs.push(mem::take(&mut buf));
                        state = FromStrState::Ostate;
                    }
                    _ => {
                        return Err(MachineError {
                            message: "Didn't expect '>'".to_string(),
                            line,
                            offset,
                        })
                    }
                },
                ch => {
                    buf.push(ch);
                }
            }
            offset += 1;
        }
        Ok(machine)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const BINARY_ADDITION: &str = "s0, 00 > s0, 0
s0, 01 > s0, 1
s0, 10 > s0, 1
s0, 11 > s1, 0
s1, 10 > s1, 0
s1, 01 > s1, 0
s1, 00 > s0, 1
s1, 11 > s1, 1
";
    #[test]
    fn test_simple() {
        let machine = Machine::from_str(BINARY_ADDITION);
        assert!(machine.is_ok());
        let machine = machine.unwrap();
        assert_eq!(format!("{machine}"), BINARY_ADDITION)
    }

    #[test]
    fn test_multi_input() {
        let binary_multiinput = "s0, 00 > s0, 0
s0, 01, 10 > s0, 1
s0, 11 > s1, 0
s1, 01, 10 > s1, 0
s1, 00 > s0, 1
s1, 11 > s1, 1
";
        let machine = Machine::from_str(binary_multiinput);
        assert!(machine.is_ok());
        let machine = machine.unwrap();
        assert_eq!(format!("{machine}"), BINARY_ADDITION)
    }
}
