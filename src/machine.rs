use std::mem;
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct Machine {
    inputs: Vec<String>,
    outputs: Vec<String>,
    states: Vec<String>,
    ostates: Vec<String>,
}

#[derive(Debug)]
pub struct MachineError {
    message: String,
    line: usize,
    offset: usize,
}

enum FromStrState {
    State,
    Input,
    Ostate,
    Output,
}

impl Machine {
    pub fn to_c(&self) -> String {
format!("#include <stdio.h>
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
  char * state = {};
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
}}", arrayize(&self.states), arrayize(&self.inputs), arrayize(&self.ostates), arrayize(&self.outputs), self.states[0])
    }
}

fn arrayize(arr: &Vec<String>) -> String {
    let quoted: Vec<String> = arr.iter().map(|s| format!("\"{}\"",s)).collect();
    return quoted.join(", ");
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
                        machine.states.push(temp_machine.states.pop().unwrap());
                        machine.outputs.push(mem::take(&mut buf));
                        machine.ostates.push(temp_machine.ostates.pop().unwrap());
                        for _ in 1..temp_machine.inputs.len() {
                            machine.states.push(machine.states[0].clone());
                            machine.outputs.push(machine.outputs[0].clone());
                            machine.ostates.push(machine.ostates[0].clone());
                        }
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
                    }
                    FromStrState::Ostate => {
                        temp_machine.ostates.push(mem::take(&mut buf));
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
        return Ok(machine);
    }
}
