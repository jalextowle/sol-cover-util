use std::collections::HashMap;

pub struct IterableMapping<T, K> {
    pub map: HashMap<T, K>,
    pub total: i32 
}

impl<T: std::cmp::Eq + std::hash::Hash + Copy, K> IterableMapping<T, K> {
    pub fn put(mut self, key: T, value: K) -> IterableMapping<T, K> {
        match self.at(key) {
            None => {
                self.map.insert(key, value);
                self.total += 1;
            }
            Some(_) => ()
        };
        self
    } 
    pub fn at(&self, key: T) -> Option<&K> {
        self.map.get(&key)
    }
}

fn to_hex(x: char) -> i32 {
    let result;
    match x as i32 {
        48 ... 57  => result = x as i32 - 48,
        65 ... 90  => result = x as i32 - 55,
        97 ... 122 => result = x as i32 - 87,
        _ => panic!("Not a valid hex number")
    };
    result
}

pub fn process_bytecode(bytecode: String) -> IterableMapping<i32, bool> {
    let mut result: IterableMapping<i32, bool> = IterableMapping { map: HashMap::new(), total: 0 };
    let flip = |x: usize, m: IterableMapping<i32, bool>| m.put(x as i32, true);
    let mut i = 0;
    for c in bytecode.trim().chars() {
        println!("Char: {}", c);
    }
    let byte_chars: Vec<char> = bytecode.trim().chars().collect();
    while 2 * i < byte_chars.len() { 
        let opcode = to_hex(byte_chars[2 * i]) * 16 + to_hex(byte_chars[ 2 * i + 1]);
        // Flip all the numbers that represent valid ethereum opcodes
        match opcode {
            0x00 ... 0x0b => result = flip(i, result),
            0x10 ... 0x1a => result = flip(i, result),
            0x20          => result = flip(i, result), 
            0x30 ... 0x3e => result = flip(i, result),
            0x40 ... 0x45 => result = flip(i, result),
            0x50 ... 0x5b => result = flip(i, result),
            op @0x60 ... 0x7f => {
                result = flip(i, result);
                i += 2 * ((op as usize) - 0x5f);
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
pub fn process_pc(contents: String) -> Vec<i32> {
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

#[cfg(test)]
mod bytecode_tests {
    use super::*;

    #[test]
    fn no_push1() {
        let test_string = "010002040a";
        let test_result = process_bytecode(String::from(test_string));
        assert_eq!(test_result.total, 5);
        for i in 0 .. 5 {
            assert_eq!(test_result.at(i), Some(&true));
        }
    }
}

#[cfg(test)]
mod pc_tests { 
    use super::*;

    #[test]
    fn in_order1() {
        let test_string = "PC: 0\nPC: 1\nPC: 2\nPC: 3\nPC: 4"; 
        let test_result = process_pc(String::from(test_string));
        assert_eq!(test_result.len(), 5);
        for i in 0 .. 5 {
            assert_eq!(test_result[i], i as i32); 
        }
    } 
}
