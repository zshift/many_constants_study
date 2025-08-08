# Many Constants Debugging

This repo was inspired by [this post on reddit](https://www.reddit.com/r/rust/comments/1mk8nvt/handling_80000_constants_in_a_project/).

There's unusual behavior that lines up between rust-analyzer and compiled rust binaries when there are greater than ~8478 constants.
Rust-analyzer and a compiled binary that iterates over a const array of 8478 constants both work correctly, albeit rust-analyzer takes a couple of seconds to finish indexing the constants on VSCode. When I bump this to 8479 constants, rust-analyzer appears to hang indefinitely, and running the compiled code will throw a stack-overflow exception.

When run at 8478
```bash
> cargo run
Holy constants, Batman!
CONST_0 = 0
CONST_1 = 1
CONST_2 = 2
CONST_3 = 3
CONST_4 = 4
...
CONST_8477 = 8477
```

When run at 8479
```bash
> cargo run
Holy constants, Batman!

thread 'main' has overflowed its stack
error: process didn't exit successfully: `target\debug\many_constants_study.exe` (exit code: 0xc00000fd, STATUS_STACK_OVERFLOW)
```

This was run on a system with the following specifications:

OS: Windows 11 Pro 24H2

CPU: 9800X3D

RAM: 64GB

Rust version: 1.88.0 stable
