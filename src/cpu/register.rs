/*
 * register.rs
 * Enumerates Minecraft CPU registers
 * Created by Sarah Davis
 * Created on 1/6/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use std::convert::TryFrom;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


/// A register in the Minecraft CPU
///
/// Implemented as a scoreboard objective
#[derive(Copy, Clone, Debug, PartialEq, EnumIter)]
pub enum Register {
    /// Register 0 (`r0`)
    R0,

    /// Register 1 (`r1`)
    R1,

    /// Register 2 (`r2`)
    R2,

    /// Register 3 (`r3`)
    R3,

    /// Register 4 (`r4`)
    R4,

    /// Register 5 (`r5`)
    R5,

    /// Register 6 (`r6`)
    R6,

    /// Register 7 (`r7`)
    R7,

    /// Register 8 (`r8`)
    R8,

    /// Register 9 (`r9`)
    R9,

    /// Register 10 (`r10`)
    R10,

    /// Register 11 (`r11`)
    R11,

    /// Register 12 (`r12`)
    R12,

    /// Register 13 (`r13`)
    R13,

    /// Register 14 (`r14`)
    R14,

    /// Register 15 (`r15`)
    R15,

    /// The stack register
    ///
    /// Always contains the value
    /// on top of the CPU stack,
    /// and cannot be read from
    /// or written to from
    /// userspace code
    SR
}

// implementation
impl Register {
    /// Gets the total number of registers
    ///
    /// # Returns
    ///
    /// The total number of registers
    pub fn count() -> usize {
        Self::iter().count()
    }
}

// TryFrom implementation
impl TryFrom<u8> for Register {
    // create an error type
    type Error = ();

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        // convert the value to a Register
        match val {
            0x0 => Ok(Register::R0),
            0x1 => Ok(Register::R1),
            0x2 => Ok(Register::R2),
            0x3 => Ok(Register::R3),
            0x4 => Ok(Register::R4),
            0x5 => Ok(Register::R5),
            0x6 => Ok(Register::R6),
            0x7 => Ok(Register::R7),
            0x8 => Ok(Register::R8),
            0x9 => Ok(Register::R9),
            0xA => Ok(Register::R10),
            0xB => Ok(Register::R11),
            0xC => Ok(Register::R12),
            0xD => Ok(Register::R13),
            0xE => Ok(Register::R14),
            0xF => Ok(Register::R15),
            0x10 => Err(()),
            _ => Err(())
        }
    }
}

// Display implementation 
impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // get the name of the register
        let reg_name = match self {
            Register::R0 => "r0",
            Register::R1 => "r1",
            Register::R2 => "r2",
            Register::R3 => "r3",
            Register::R4 => "r4",
            Register::R5 => "r5",
            Register::R6 => "r6",
            Register::R7 => "r7",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
            Register::SR => "sr"
        };

        // and write it
        write!(f, "{}", reg_name)
    }
}

// From implementation for u8
impl TryFrom<Register> for u8 {
    // create an error type
    type Error = ();

    fn try_from(reg: Register) -> Result<u8, Self::Error> {
        match reg {
            Register::R0 => Ok(0x0),
            Register::R1 => Ok(0x1),
            Register::R2 => Ok(0x2),
            Register::R3 => Ok(0x3),
            Register::R4 => Ok(0x4),
            Register::R5 => Ok(0x5),
            Register::R6 => Ok(0x6),
            Register::R7 => Ok(0x7),
            Register::R8 => Ok(0x8),
            Register::R9 => Ok(0x9),
            Register::R10 => Ok(0xA),
            Register::R11 => Ok(0xB),
            Register::R12 => Ok(0xC),
            Register::R13 => Ok(0xD),
            Register::R14 => Ok(0xE),
            Register::R15 => Ok(0xF),
            Register::SR => Err(())
        }
    }
}

// start of unit tests
#[cfg(test)]
mod tests {
    // load the enum
    use super::*;

    // this test checks converting from a u8 to a Register
    #[test]
    fn test_convert_from_u8() {
        // test valid conversions
        assert_eq!(Register::try_from(0x0).unwrap(), Register::R0);
        assert_eq!(Register::try_from(0x1).unwrap(), Register::R1);
        assert_eq!(Register::try_from(0x2).unwrap(), Register::R2);
        assert_eq!(Register::try_from(0x3).unwrap(), Register::R3);
        assert_eq!(Register::try_from(0x4).unwrap(), Register::R4);
        assert_eq!(Register::try_from(0x5).unwrap(), Register::R5);
        assert_eq!(Register::try_from(0x6).unwrap(), Register::R6);
        assert_eq!(Register::try_from(0x7).unwrap(), Register::R7);
        assert_eq!(Register::try_from(0x8).unwrap(), Register::R8);
        assert_eq!(Register::try_from(0x9).unwrap(), Register::R9);
        assert_eq!(Register::try_from(0xA).unwrap(), Register::R10);
        assert_eq!(Register::try_from(0xB).unwrap(), Register::R11);
        assert_eq!(Register::try_from(0xC).unwrap(), Register::R12);
        assert_eq!(Register::try_from(0xD).unwrap(), Register::R13);
        assert_eq!(Register::try_from(0xE).unwrap(), Register::R14);
        assert_eq!(Register::try_from(0xF).unwrap(), Register::R15);

        // then test invalid conversions
        assert_eq!(Register::try_from(0x10), Err(())); // 0x10 is reserved for the stack pointer
        assert_eq!(Register::try_from(0x11), Err(()));
    }

    // this test checks converting from a Register to a u8
    #[test]
    fn test_convert_to_u8() {
        // test valid conversions
        assert_eq!(u8::try_from(Register::R0).unwrap(), 0x0);
        assert_eq!(u8::try_from(Register::R1).unwrap(), 0x1);
        assert_eq!(u8::try_from(Register::R2).unwrap(), 0x2);
        assert_eq!(u8::try_from(Register::R3).unwrap(), 0x3);
        assert_eq!(u8::try_from(Register::R4).unwrap(), 0x4);
        assert_eq!(u8::try_from(Register::R5).unwrap(), 0x5);
        assert_eq!(u8::try_from(Register::R6).unwrap(), 0x6);
        assert_eq!(u8::try_from(Register::R7).unwrap(), 0x7);
        assert_eq!(u8::try_from(Register::R8).unwrap(), 0x8);
        assert_eq!(u8::try_from(Register::R9).unwrap(), 0x9);
        assert_eq!(u8::try_from(Register::R10).unwrap(), 0xA);
        assert_eq!(u8::try_from(Register::R11).unwrap(), 0xB);
        assert_eq!(u8::try_from(Register::R12).unwrap(), 0xC);
        assert_eq!(u8::try_from(Register::R13).unwrap(), 0xD);
        assert_eq!(u8::try_from(Register::R14).unwrap(), 0xE);
        assert_eq!(u8::try_from(Register::R15).unwrap(), 0xF);

        // then test invalid conversions
        assert_eq!(u8::try_from(Register::SR), Err(()));
    }

    // this test checks getting the number of available registers
    #[test]
    fn test_count() {
        // NOTE: Update this if the number of registers changes
        assert_eq!(Register::count(), 17); 
    }
}

// end of file
