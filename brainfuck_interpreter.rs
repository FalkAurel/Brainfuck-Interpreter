use Operations::*;

/*
> increment the data pointer (to point to the next cell to the right).
< decrement the data pointer (to point to the next cell to the left).
+ increment (increase by one, truncate overflow: 255 + 1 = 0) the byte at the data pointer.
- decrement (decrease by one, treat as unsigned byte: 0 - 1 = 255 ) the byte at the data pointer.
. output the byte at the data pointer.
, accept one byte of input, storing its value in the byte at the data pointer.
[ if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
] if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

The interpreter will take the program code, a string with the sequence of machine instructions,
the program input, a string, possibly empty, that will be interpreted as an array of bytes using each character's ASCII code and will be consumed by the , instruction
*/

enum Operations {
    Add,       // +
    Subtract,  // -
    Increment, // >
    Decrement, // <
    Jump,      // [ ]
    Output,    // .
    Input      // ,
}

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

fn main() {
    let input: Vec<u8> = vec![8, 9]; // supply the underlying data.
    let code: &[u8] = ",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.".as_bytes(); // should multiply 8 * 9 => 72
    let mut sp: usize = 0;

    let mut stack: Vec<u8> = vec![0; 300]; // Ensuring enough space for the operations, namely jumping
    let mut output: Vec<u8> = Vec::new();

    let mut stack_pointer: usize = 0;
    let mut current_pointer: usize = 0;

    while let Some(element) = code.get(sp) {
        match decode(*element) {
            Add => add(stack_pointer, &mut stack),
            Subtract => subtract(stack_pointer, &mut stack),
            Input => {
                if stack_pointer >= stack.len() {
                    stack.push(input[current_pointer]);
                } else {
                    stack[stack_pointer] = input[current_pointer];
                }
                current_pointer = increment(current_pointer, input.len());
            }
            Output => output.push(stack[stack_pointer]),
            Increment => stack_pointer = increment(stack_pointer, stack.len()),
            Decrement => stack_pointer = decrement(stack_pointer, stack.len()),
            Jump => sp = jump(*element, stack[stack_pointer], sp, code),
        }

        sp += 1;
    }

    println!("{:?}", output);
}
