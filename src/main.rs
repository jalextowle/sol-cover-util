use std::env;
use std::collections::HashMap;
use std::fs;

struct IterableMapping<T, K> {
    map: HashMap<T, K>,
    total: i32 
}

impl<T: std::cmp::Eq + std::hash::Hash + Copy, K> IterableMapping<T, K> {
    fn put(mut self, key: T, value: K) -> IterableMapping<T, K> {
        match self.at(key) {
            None => {
                self.map.insert(key, value);
                self.total += 1;
            }
            Some(_) => ()
        };
        self
    } 
    fn at(&self, key: T) -> Option<&K> {
        self.map.get(&key)
    }
}

fn process_bytecode(bytecode: Vec<u8>) -> IterableMapping<i32, bool> {
    let mut result: IterableMapping<i32, bool> = IterableMapping { map: HashMap::new(), total: 0 };
    let flip = |x: usize, m: IterableMapping<i32, bool>| m.put(x as i32, true);
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
                result = flip(i, result);
                i += (op as usize) - 0x5f;
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
        let s = s.trim();
        match s {
            "" => continue,
            _ => {
                let pc = s.parse::<i32>().unwrap(); 
                result.push(pc)
            }
        };
    }
    return result;
}

fn coverage(mut byte_set: IterableMapping<i32, bool>, pc_set: Vec<i32>) -> f32 {
    let mut total: i32 = 0;
    for pc in pc_set {
        match byte_set.at(pc) {
            None => continue,
            Some(false) => continue, 
            Some(true) => { 
                total += 1;
                // I am breaking the abstraction barrier so that the total index will not be
                // incremented TODO I think it already wouldn't increment
                byte_set.map.insert(pc, false);
            }
        };
    }
    total as f32 / byte_set.total as f32
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
    println!("Code Coverage: {}", coverage(byte_set, pc_set));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let bytecode_contents = fs::read("test/sanity/bytecode.bytes").
            expect("Error: There was an issue reading the bytecode input file");
        let pc_contents = fs::read_to_string("test/sanity/output.txt").
            expect("Error: There was an issue reading the PC input file");
        let byte_set = process_bytecode(bytecode_contents);
        let pc_set = process_pc(pc_contents);
        assert_eq!(coverage(byte_set, pc_set), 1.0);
    }
}
