use std::collections::hash_map::HashMap;
use std::io::{stdout, Write};

fn main() {
    let code = ">  ++++ (4 digits)
[<+>>>>>>>>++++++++++<<<<<<<-]>+++++[<+++++++++>-]+>>>>>>+[<<+++[>>[-<]<[>]<-]>>
[>+>]<[<]>]>[[->>>>+<<<<]>>>+++>-]<[<<<<]<<<<<<<<+[->>>>>>>>>>>>[<+[->>>>+<<<<]>
>>>>]<<<<[>>>>>[<<<<+>>>>-]<<<<<-[<<++++++++++>>-]>>>[<<[<+<<+>>>-]<[>+<-]<++<<+
>>>>>>-]<<[-]<<-<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>+<<<-[>>+<<-]<]<<<<+>>>>>>>
>[-]>[<<<+>>>-]<<++++++++++<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>+>[<<+<+>>>-]<<<
<+<+>>[-[-[-[-[-[-[-[-[-<->[-<+<->>]]]]]]]]]]<[+++++[<<<++++++++<++++++++>>>>-]<
<<<+<->>>>[>+<<<+++++++++<->>>-]<<<<<[>>+<<-]+<[->-<]>[>>.<<<<[+.[-]]>>-]>[>>.<<
-]>[-]>[-]>>>[>>[<<<<<<<<+>>>>>>>>-]<<-]]>>[-]<<<[-]<<<<<<<<]++++++++++.";

    let mut operations: Vec<Operation> = Vec::new();

    let mut loops_locations: HashMap<usize, usize> = HashMap::new();
    let mut loops_openings: Vec<usize> = Vec::new();

    let mut i = 0;
    for char in code.chars() {
        let op = match char {
            '>' => Operation::IncrementDataPointer,
            '<' => Operation::DecrementDataPointer,
            '+' => Operation::IncrementCellValue,
            '-' => Operation::DecrementCellValue,
            '.' => Operation::OutputCell,
            ',' => Operation::InputCell,
            '[' => {
                loops_locations.insert(i, code.len());
                loops_openings.push(i);
                Operation::OpenLoop
            }
            ']' => {
                loops_locations.insert(loops_openings.pop().unwrap(), i);
                Operation::CloseLoop
            }
            _ => continue,
        };
        operations.push(op);
        i += 1;
    }
    assert!(loops_openings.is_empty());

    let mut loops_locations_reversed: HashMap<usize, usize> = HashMap::new();

    for (k, v) in loops_locations.iter() {
        loops_locations_reversed.insert(*v, *k);
    }

    let mut cells = Cells::new();

    let mut current_index = 0;

    loop {
        if current_index >= operations.len() {
            break;
        }
        let op = &operations[current_index];
        match op {
            Operation::IncrementDataPointer => cells.increment_index(),
            Operation::DecrementDataPointer => cells.decrement_index(),

            Operation::IncrementCellValue => cells.increment_value(),
            Operation::DecrementCellValue => cells.decrement_value(),

            Operation::OutputCell => cells.print(),
            Operation::InputCell => cells.read(),

            Operation::OpenLoop => {
                if cells.is_empty() {
                    current_index = *loops_locations.get(&current_index).unwrap();
                }
            }
            Operation::CloseLoop => {
                if !cells.is_empty() {
                    current_index = *loops_locations_reversed.get(&current_index).unwrap();
                }
            }
        }
        current_index += 1;
    }
    println!();
}

enum Operation {
    IncrementDataPointer,
    DecrementDataPointer,

    IncrementCellValue,
    DecrementCellValue,

    OutputCell,
    InputCell,

    OpenLoop,
    CloseLoop,
}

struct Cells {
    pub tape: Vec<u8>,
    index: usize,
    value: u8,
}

impl Cells {
    pub fn new() -> Cells {
        Cells {
            tape: vec![0],
            index: 0,
            value: 0u8,
        }
    }

    pub fn increment_index(&mut self) {
        self.index += 1;
        if self.index >= self.tape.len() {
            self.tape.push(0);
        }
        self.update_value();
    }

    pub fn decrement_index(&mut self) {
        if self.index == 0 {
            panic!("Invalid decrement operation")
        }

        self.index -= 1;
        self.update_value();
    }

    pub fn increment_value(&mut self) {
        if self.value == 255 {
            self.tape[self.index] = 0;
        } else {
            self.tape[self.index] += 1;
        }
        self.update_value();
    }

    pub fn decrement_value(&mut self) {
        if self.value == 0 {
            self.tape[self.index] = 255;
        } else {
            self.tape[self.index] -= 1;
        }
        self.update_value();
    }

    pub fn print(&self) {
        let value = &[self.value];
        let char = std::str::from_utf8(value).unwrap();
        print!("{}", char);
        stdout().flush().unwrap();
    }

    pub fn read(&mut self) {
        self.tape[self.index] = 0;
        self.update_value();
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    fn update_value(&mut self) {
        self.value = self.tape[self.index];
    }
}
