a simple finite state automaton generator with yet another DSL.

## Usage
use `atomato --help` to see usage. the general usage is `atomato foo` which
outputs the C code for the file named `foo` according to [atomato
syntax](#Syntax) in stdout. you can also use `--makefile` option to create a gnu
makefile. the normal C code depends on `editline` library. if you want to
disable that, you can use the `--plain-c` option.

## Syntax
```atomato
s0,1 > s1,1
s1,1 > s0,0
```

the above code generates a machine, that when `s0` is present, with input `1` it
prints `1` and goes to state `s1`.

when `s1`, given input `1`, it prints `0` and goes to state `s0`.

initial state is the first input state defined. which is `s0` here.

```atomato
s1,1 > s0,0
s0,1 > s1,1
```

if you change the code to something like above, the initial state would be `s1`.


## Examples
there are some examples of different machines and their outputed C files inside the [examples folder](https://github.com/nimaaskarian/atomato/tree/master/examples).
you can check them out
