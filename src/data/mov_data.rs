/*
 * mov_data.rs
 * Defines the metadata structure for the MOV instruction
 * Created by Sarah Davis
 * Created on 3/8/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statement
use super::super::cpu::Register;

/// Metadata for the `MOV` instruction
#[derive(Copy, Clone, Debug)]
pub struct MOVData {
    /// The source data (if a literal)
    src_lit: Option<u32>,

    /// The source data (if a register)
    src_reg: Option<Register>,

    /// The destination register
    dst: Register,
}

// implementation
impl MOVData {
    /// Creates a new `MOVData` instance with a literal
    /// as the source
    ///
    /// # Arguments
    ///
    /// * `src` - The source literal
    /// * `dest` - The destination register
    ///
    /// # Returns
    ///
    /// A new `MOVData` instance with the given data
    pub fn from_literal(src: u32, dest: Register) -> Self {
        MOVData::new(Some(src), None, dest)
    }

    /// Creates a new `MOVData` instanct
    /// with a register
    /// as the source
    ///
    /// # Arguments
    ///
    /// * `src` - The source register
    /// * `dest` - The destination register
    ///
    /// # Returns
    ///
    /// A new `MOVData` instance with the given data
    pub fn from_register(src: Register, dest: Register) -> Self {
        MOVData::new(None, Some(src), dest)
    }

    /// Returns whether the source is a literal
    ///
    /// # Returns
    ///
    /// Whether the associated `MOV` instruction has a literal as its source
    pub fn has_literal_source(&self) -> bool {
        self.src_lit.is_some()
    }

    /// Returns whether the source is a register
    ///
    /// # Returns
    ///
    /// Whether the associated `MOV` instruction has a register as its source
    pub fn has_register_source(&self) -> bool {
        self.src_reg.is_some()
    }

    /// Gets the source literal
    ///
    /// # Returns
    ///
    /// The source literal, wrapped in an `Option`
    pub fn src_literal(&self) -> Option<u32> {
        self.src_lit
    }

    /// Gets the source literal, unwrapped
    ///
    /// # Returns
    ///
    /// The source literal
    ///
    /// # Panics
    ///
    /// This method will panic if called on an instance that does
    /// not have a source literal.
    pub fn unwrap_src_literal(&self) -> u32 {
        self.src_lit.unwrap()
    }

    /// Gets the source register
    ///
    /// # Returns
    ///
    /// The source register, wrapped in an `Option`
    pub fn src_register(&self) -> Option<Register> {
        self.src_reg
    }

    /// Gets the source register, unwrapped
    ///
    /// # Returns
    ///
    /// The source register
    ///
    /// # Panics
    ///
    /// This method will panic if called on an instance that does
    /// not have a source register.
    pub fn unwrap_src_register(&self) -> Register {
        self.src_reg.unwrap()
    }

    /// Gets the destination register
    ///
    /// # Returns
    ///
    /// The destination register
    pub fn dest(&self) -> Register {
        self.dst
    }

    /// [Internal use only]
    /// Creates a new `MOVData` instance
    ///
    /// # Arguments
    ///
    /// * `slit` - The source literal
    /// * `sreg` - The source register
    /// * `dest` - The destination register
    ///
    /// # Returns
    ///
    /// A new `MOVData` instance with the given data
    fn new(slit: Option<u32>, sreg: Option<Register>, dest: Register) -> Self {
        MOVData {
            src_lit: slit,
            src_reg: sreg,
            dst: dest,
        }
    }
}

// start of unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Tests the has_literal_source method
    #[test]
    fn test_has_literal_source() {
        let d1 = MOVData::from_literal(0x32, Register::R0);
        let d2 = MOVData::from_register(Register::R0, Register::R1);
        assert!(d1.has_literal_source());
        assert_eq!(d2.has_literal_source(), false);
    }

    // Tests the has_register_source method
    #[test]
    fn test_has_register_source() {
        let d1 = MOVData::from_register(Register::R0, Register::R1);
        let d2 = MOVData::from_literal(0x32, Register::R0);
        assert!(d1.has_register_source());
        assert_eq!(d2.has_register_source(), false);
    }
}

// end of file
