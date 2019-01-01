use std::env;
use std::fs;

// Processes the contents of an output file to produce a vector
// of integers that were the program counters used during the test suite.
fn process_bytecode(contents: String) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let split = contents.split("PC: ");
    for s in split {
        let pc = s.parse::<i32>().unwrap();
        result.push(pc);
    }
    return result;
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
    println!("{}", bytecode_contents);
    println!("{}", pc_contents);
    /*
    let pc_arr = process(pc_contents);
    println!("{}", pc_arr.len()); 
    */
}
