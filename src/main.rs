#![allow(non_snake_case)]

extern crate log;
extern crate pretty_env_logger;

mod ram;
mod cpu;
mod data;
mod system;

use ram::RAM;
use data::{Datum, Value, Address, Instruction};
use log::{info};

/// Return a prepared RAM:
/// 0   load value      4
/// 1   add value       5
/// 2   store value     6
/// 3   jump            0
/// 4   5               
/// 5   6               
/// 6                   
fn get_ram() -> RAM {
    let mut ram = RAM::new();
    // add at addr 0u8        an instruction   new instruction opcode 0u8    operand address at 4u8
    ram.set(Address(0u8), Datum::DataInstruction(Instruction::new(0u8, Value::Address(Address(4u8)))))
        .set(Address(1u8), Datum::DataInstruction(Instruction::new(1u8, Value::Address(Address(5u8)))))
        .set(Address(2u8), Datum::DataInstruction(Instruction::new(2u8, Value::Address(Address(6u8)))))
        .set(Address(3u8), Datum::DataInstruction(Instruction::new(3u8, Value::Address(Address(0u8)))))
        .set(Address(4u8), Datum::DataValue(Value::Integer(5)))
        .set(Address(5u8), Datum::DataValue(Value::Integer(6)))
        .set(Address(6u8), Datum::DataValue(Value::None));

    ram
}

fn main() {
    pretty_env_logger::init();

    let ram = get_ram();
    let mut system = system::System::build(ram);
    system.CPU.start_cycle(100u64);
    info!("ending program");
}
