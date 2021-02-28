/*
 * opcode.rs
 * Defines a struct that represents a Minecraft CPU opcode
 * Created by Sarah Davis
 * Created on 2/27/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */


// usage statements
use std::convert::From;
use std::fmt;
use std::cmp::PartialEq;

/// An assembly opcode
#[derive(Copy, Clone)]
pub struct Opcode(u32);

// implementation
impl Opcode {
    /// Gets the numeric value of the `Opcode`
    ///
    /// # Returns
    ///
    /// The `u32` wrapped by the `Opcode`
    pub fn value(&self) -> u32 {
        self.0 
    }
}

// PartialEq implementation
impl PartialEq for Opcode {
    fn eq(&self, other: &Opcode) -> bool {
        self.0 == other.0 
    }
}

// Display implementation
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:#010x}", self.0))
    }
}

// Debug implementation
impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode ({})", self)
    }
}

// From implementation
impl From<u32> for Opcode {
    fn from(data: u32) -> Opcode {
        Opcode(data)
    }
}

// From implementation for u32
impl From<Opcode> for u32 {
    fn from(op: Opcode) -> u32 {
        op.value()
    }
}

// start of unit tests
#[cfg(test)]
mod tests {
    // load the struct
    use super::*;

    // this test checks equality between Opcodes
    #[test]
    fn test_equality() {
        // create test opcodes
        let op1 = Opcode(0xdeadbeef);
        let op2 = Opcode(0x12345678);

        // test equality
        assert_eq!(op1, op1);
        assert_eq!(op2, op2);

        // and test inequality
        assert_ne!(op1, op2);
        assert_ne!(op2, op1);
    }

    // this test checks the value method
    #[test]
    fn test_value() {
        assert_eq!(Opcode(0xdeadbeef).value(), 0xdeadbeef);
    }

    // this test checks display formatting
    #[test]
    fn test_display_formatting() {
        assert_eq!(format!("{}", Opcode(0x12345678)), "0x12345678");
        assert_eq!(format!("{}", Opcode(0x2345678)), "0x02345678");
        assert_eq!(format!("{}", Opcode(0x345678)), "0x00345678");
        assert_eq!(format!("{}", Opcode(0x45678)), "0x00045678");
        assert_eq!(format!("{}", Opcode(0x5678)), "0x00005678");
        assert_eq!(format!("{}", Opcode(0x678)), "0x00000678");
        assert_eq!(format!("{}", Opcode(0x78)), "0x00000078");
        assert_eq!(format!("{}", Opcode(0x8)), "0x00000008");
        assert_eq!(format!("{}", Opcode(0)), "0x00000000");
    }

    // this test checks converting from a u32 into an Opcode
    #[test]
    fn test_u32_to_opcode_conversion() {
        assert_eq!(Opcode::from(0xdeadbeef), Opcode(0xdeadbeef));
    }

    // this test checks converting from an Opcode into a u32
    #[test]
    fn test_opcode_to_u32_conversion() {
        assert_eq!(u32::from(Opcode(0xdeadbeef)), 0xdeadbeef);
    }
}

// end of file
