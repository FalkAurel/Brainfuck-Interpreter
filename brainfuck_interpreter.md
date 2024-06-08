# Brainfuck Interpreter in Rust

This project is a simple Brainfuck interpreter implemented in Rust. Brainfuck is a minimalistic programming language known for its simple syntax and challenging code structures.

## Brainfuck Operations

The interpreter supports the following Brainfuck operations:

- `>`: Increment the data pointer (to point to the next cell to the right).
- `<`: Decrement the data pointer (to point to the previous cell to the left).
- `+`: Increment the byte at the data pointer (wrapping overflow: 255 + 1 = 0).
- `-`: Decrement the byte at the data pointer (treat as unsigned byte: 0 - 1 = 255).
- `.`: Output the byte at the data pointer.
- `,`: Accept one byte of input, storing its value in the byte at the data pointer.
- `[`: If the byte at the data pointer is zero, jump forward to the command after the matching `]`.
- `]`: If the byte at the data pointer is non-zero, jump back to the command after the matching `[`.

## Implementation

### Enum for Operations

```rust
use Operations::*;

#[derive(Debug)]
enum Operations {
    Add,       // +
    Subtract,  // -
    Increment, // >
    Decrement, // <
    Jump,      // [ ]
    Output,    // .
    Input      // ,
}
```

### Decode Function
```rust
fn decode(instruction: u8) -> Operations {
    match instruction {
        b'+' => Add,
        b'-' => Subtract,
        b'>' => Increment,
        b'<' => Decrement,
        b'[' | b']' => Jump,
        b'.' => Output,
        b',' => Input,
        _ => std::process::exit(-1) // Should never occur
    }
}
```

### Operation Functions

```rust
fn add(stack_pointer: usize, stack: &mut Vec<u8>) {
    stack[stack_pointer] = stack[stack_pointer].wrapping_add(1);
}

fn subtract(stack_pointer: usize, stack: &mut Vec<u8>) {
    stack[stack_pointer] = stack[stack_pointer].wrapping_sub(1);
}

fn increment(current: usize, length: usize) -> usize {
    (current + 1) % length
}

fn decrement(current: usize, length: usize) -> usize {
    if current == 0 {
        length - 1
    } else {
        current - 1
    }
}
```

### Jump Function

```rust
fn jump(indicator: u8, value: u8, current_index: usize, code: &[u8]) -> usize {
    fn move_ptr(pattern: u8, code: &[u8], mut index: usize, increment: bool) -> usize {
        let mut balance = 1;
        while balance != 0 {
            index = if increment { index + 1 } else { index - 1 };
            if code[index] == pattern {
                balance -= 1;
            } else if code[index] == if increment { b'[' } else { b']' } {
                balance += 1;
            }
        }
        index
    }

    match indicator {
        b'[' if value == 0 => move_ptr(b']', code, current_index, true),
        b']' if value != 0 => move_ptr(b'[', code, current_index, false),
        _ => current_index,
    }
}
```
