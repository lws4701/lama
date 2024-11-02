use std::vec::Vec;
use std::fmt::Display;
use std::num::ParseIntError;

pub struct CPURegister {
    name: String,
    number: u32,
    value: u64,
    reset_value: u64
}

pub struct CPUProcessor {
    name: String,
    number: u32,
    pub(crate) registers: Vec<CPURegister>
}

impl CPURegister {
    pub fn new(name: String, number: u32, initial_value: u64, reset_value: u64) -> CPURegister {
        CPURegister {
            name,
            number,
            value: initial_value,
            reset_value
        }
    }

    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }

    pub fn get_32bit_value(&self) -> u64 {
        self.value << 32
    }
    pub fn set_32bit_value(&mut self, value: u64) {
        self.value = value >> 32
    }

    pub fn reset_register(&mut self) {
        self.value = self.reset_value
    }
}

impl CPUProcessor {
    pub fn new(name: String, number: u32) -> CPUProcessor {
        let mut processor: CPUProcessor = CPUProcessor {
            name,
            number,
            registers: Vec::new() // The number of registers is limited, so it's okay we do
            // a linear search for the appropriate register
        };

        /*
         * General-Purpose Registers
         *
         * 64 bit views of each register should be accessible as X*, 32-bit views as W*
         * To access a specific view of the Zero Register or Stack Pointer (ZR, SP), append X or W
         * to the beginning of the register name. This should tell the emulator to access the
         * appropriate get_value function.
         */
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R0"), 0, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R1"), 1, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R2"), 2, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R3"), 3, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R4"), 4, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R5"), 5, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R6"), 6, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R7"), 7, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R8"), 8, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R9"), 9, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R10"), 10, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R11"), 11, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R12"), 12, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R13"), 13, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R14"), 14, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R15"), 15, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R16"), 16, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R17"), 17, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R18"), 18, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R19"), 19, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R20"), 20, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R21"), 21, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("22"), 22, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R23"), 23, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R24"), 24, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R25"), 25, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R26"), 26, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R27"), 27, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R28"), 28, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R29"), 29, 0, 0));
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("R30"), 30, 0, 0));
        // Zero Register
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("ZR"), 31, 0, 0));
        // Stack Pointer
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("SP"), 32, 0, 0));

        /*
         * Non General-Purpose Registers
         * TODO: Implement extension registers (vector registers, 128-bit registers, floating point
         *       registers)
         */
        // Program Counter - this should never be explicitly accessible
        processor.registers.insert(processor.registers.len(),
                                   CPURegister::new(String::from("PC"), 33, 0, 0));
        processor
    }

    fn read_register_from_name(&self, register_name: String) -> Result<u64, ParseIntError> {
        let is_32bit_view: bool = register_name.contains("W");
        if register_name.contains("ZR") {
            Ok(self.read_register_from_number(31, is_32bit_view))
        } else if register_name.contains("SP") {
            Ok(self.read_register_from_number(32, is_32bit_view))
        } else if register_name.contains("PC") {
            Ok(self.read_register_from_number(33, is_32bit_view))
        } else {
            // This is a general-purpose register
            return match register_name
                .replace(&['W', 'Z', 'X'][..], "")
                .parse() {
                Ok(register_number) => Ok(self.read_register_from_number(register_number, is_32bit_view)),
                Err(e) => Err(e)
            };
        }
    }

    pub fn read_register_from_number(&self, register_number: u32, is_32bit_view: bool) -> u64 {
        if is_32bit_view {
            self.registers.get(register_number as usize).unwrap().get_32bit_value()
        } else {
            self.registers.get(register_number as usize).unwrap().get_value()
        }
    }
}
impl Display for CPURegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Register {} ({}): {:#16x}", self.number, self.name, self.value)
    }
}

impl Display for CPUProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "---CPU {} ({})---\n\t---Registers---\n", self.number, self.name)?;
        for register in self.registers.iter() {
            write!(f, "\t{}\n", register)?;
        }
        write!(f, "\t------")?;
        Ok(())
    }
}