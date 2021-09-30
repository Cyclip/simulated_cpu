//! Main memory
//! 
//! opcodes:
//! | opcode | instruction |
//! |--------|-------------|
//! | 0u8    | LOAD VAL    |
//! | 1u8    | ADD VAL     |
//! | 2u8    | STORE VAL   |
//! | 3u8    | STOP        |

// fixed capacity but not memory size for simplicity
pub struct RAM {
    pub data: Vec<Datum>,
    pub size: usize,
}

pub enum Datum {
    DataValue(Value),
    DataInstruction(Instruction),
}

pub enum Value {
    Integer(usize),
    String(String),
    Address(Address),
    None,
}

pub struct Address(pub u8);

pub struct Instruction {
    pub opcode: u8,
    pub operand: Value,
}

impl RAM {
    /// Create new growable RAM
    pub fn new() -> RAM {
        RAM {
            data: Vec::new(),
            size: 0,
        }
    }

    /// Append data onto RAM and return itself
    pub fn append(&mut self, address: Address, data: Datum) -> &mut Self {
        self.data.push(data);
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