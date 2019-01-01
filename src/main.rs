mod lib;
use lib::{ IterableMapping, process_bytecode, process_pc };
use std::env;
use std::fs;

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
    let bytecode_contents = fs::read_to_string(bytecode_input).
        expect("Error: There was an issue reading the bytecode input file");
    let pc_contents = fs::read_to_string(pc_input).
        expect("Error: There was an issue reading the PC input file");
    let byte_set = process_bytecode(bytecode_contents);
    let pc_set = process_pc(pc_contents);
    println!("Code Coverage: {}", coverage(byte_set, pc_set));
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let bytecode_contents = fs::read_to_string("test/sanity/bytecode.txt").
            expect("Error: There was an issue reading the bytecode input file");
        let pc_contents = fs::read_to_string("test/sanity/output.txt").
            expect("Error: There was an issue reading the PC input file");
        let byte_set = process_bytecode(bytecode_contents);
        let pc_set = process_pc(pc_contents);
        assert_eq!(coverage(byte_set, pc_set), 1.0);
    }
}
