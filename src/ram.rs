//! Main memory
//! 
//! opcodes:
//! | opcode | instruction |
//! |--------|-------------|
//! | 0u8    | LOAD VAL    |
//! | 1u8    | ADD VAL     |
//! | 2u8    | STORE VAL   |
//! | 3u8    | JUMP        |

use crate::data::{Datum, Value, Address, Instruction};
use std::collections::HashMap;

// fixed capacity but not memory size for simplicity
pub struct RAM {
    pub data: HashMap<Address, Datum>,
    pub size: usize,
}

impl RAM {
    /// Create new growable RAM
    pub fn new() -> RAM {
        RAM {
            data: HashMap::new(),
            size: 0,
        }
    }

    /// Set data onto RAM and return itself
    pub fn set(&mut self, address: Address, data: Datum) -> &mut Self {
        self.data.insert(address, data);
        self.size += 1;
        self
    }
}

impl Instruction {
    pub fn new(opcode: u8, operand: Value) -> Instruction {
        Instruction {
            opcode,
            operand,
        }
    }
}