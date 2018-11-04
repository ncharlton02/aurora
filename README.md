# Aurora (WIP)
[![Build Status](https://travis-ci.com/DevOrc/aurora.svg?branch=master)](https://travis-ci.com/DevOrc/aurora)

A lua intepreter written in rust. There is currently little documentation
due to the early stage of this project. 

## How to run
Aurora can run in two modes, file and console. File mode will run a file in the assets folder whereas console mode will run an interactive interpereter in the terminal. 

To run file mode run:
```cmd
$ cargo run -- file -f=basic
```

To run console mode run:
```
$ cargo run -- console
```

To see the tokens and the raw AST use the verbose flag:
```cmd
$ cargo run -- -v file -f=basic
```

## Features
Aurora is currently a WIP. Lots of the lua language
isn't supported. See below for the currently supported and planned features.

### Implemented Features
- Comments
- If statements
- Print statements
- Basic arithmetic
- Functions
- While loops
- Local variables
- Basic error messages with line numbers
  

### Planned features
- Tables
- Rust/Lua interops
- For loops
- A std library
- Modules
- Order of Operations
- Library to run files 
- Basic concurrency

### Known Problems
The expression parser is very buggy so I don't recomend writting complicated expressions
like 

```lua
local x = (5 - ((8 - 9) / 3.14) - (9 - 3) * 7)
```

### Want to contribute?
Please do, there is a lot of areas that need work! 
A good place to start would be testing, examples, or the expression parser. 
If you have any questions, don't be afraid to ask. 