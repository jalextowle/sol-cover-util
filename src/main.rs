use std::env;
use std::collections::HashMap;
use std::fs;

struct IterableMapping<T, K> {
    map: HashMap<T, K>,
    keys: Vec<T>
}

fn process_bytecode(bytecode: Vec<u8>) -> IterableMapping<i32, bool> {
    let mut result: IterableMapping<i32, bool> = IterableMapping { map: HashMap::new(), keys: Vec::new() };
    let flip = |x: usize, mut m: IterableMapping<i32, bool>| { 
        m.map.insert(x as i32, true);
        m
    };
    let mut i = 0;
    while i < bytecode.len() {
        // Flip all the numbers that represent valid ethereum opcodes
        match bytecode[i] {
            0x00 ... 0x0b => result = flip(i, result),
            0x10 ... 0x1a => result = flip(i, result),
            0x20          => result = flip(i, result),
            0x30 ... 0x3e => result = flip(i, result),
            0x40 ... 0x45 => result = flip(i, result),
            0x50 ... 0x5b => result = flip(i, result),
            op @0x60 ... 0x7f => {
                i += (op as usize) - 0x5f;
                result = flip(i, result);
            }
            0x80 ... 0x8f => result = flip(i, result), 
            0x90 ... 0x9f => result = flip(i, result),
            0xa0 ... 0xa4 => result = flip(i, result),
            0xf0 ... 0xf4 => result = flip(i, result),
            0xfd ... 0xff => result = flip(i, result),
            _ => continue 
        };
        i += 1;
    }
    result
}

// Processes the contents of an output file to produce a vector
// of integers that were the program counters used during the test suite.
fn process_pc(contents: String) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let split = contents.split("PC: ");
    for s in split {
        let pc = s.parse::<i32>().unwrap();
        result.push(pc);
    }
    return result;
}

fn coverage(mut byte_set: IterableMapping<i32, bool>, pc_set: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for pc in pc_set {
        match byte_set.map.get(&pc) {
            // FIXME
            None => panic!("Line 40 in main.rs"),
            Some(false) => continue, 
            Some(true) => { 
                total += 1;
                byte_set.map.insert(pc, false);
            }
        }
    }
    total / byte_set.keys.len() as i32
}

// Takes in one commandline argument that specifies an input file. This input file 
// is expected to be list of program counters prefixed with `PC:` and separated with
// newlines.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Error: Wrong number of arguments provided");
    }
    let ref bytecode_input = args[1];
    let ref pc_input = args[2];
    let bytecode_contents = fs::read(bytecode_input).
        expect("Error: There was an issue reading the bytecode input file");
    let pc_contents = fs::read_to_string(pc_input).
        expect("Error: There was an issue reading the PC input file");
    let byte_set = process_bytecode(bytecode_contents);
    let pc_set = process_pc(pc_contents);
    
}
