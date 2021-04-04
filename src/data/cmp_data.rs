/*
 * cmp_data.rs
 * Defines the metadata structure for the CMP instruction
 * Created by Sarah Davis
 * Created on 4/4/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statement
use super::super::cpu::Register;

/// Contains metadata for the `CMP` instruction
#[derive(Copy, Clone, Debug)]
pub struct CMPData {
    /// The first `Register` to compare
    reg1: Register,

    /// The second `Register` to compare
    reg2: Register 
}

// implementation
impl CMPData { 
    /// Creates a new `CMPData` instance
    ///
    /// # Arguments
    ///
    /// * `r1` - The first `Register` to compare
    /// * `r2` - The second `Register` to compare
    ///
    /// # Returns
    ///
    /// A new `CMPData` instance with the specified data
    pub fn new(r1: Register, r2: Register) -> Self {
        CMPData {
            reg1: r1,
            reg2: r2
        }
    }

    /// Gets the first `Register` being compared
    ///
    /// # Returns
    ///
    /// The first `Register` being compared
    pub fn arg1(&self) -> Register {
        self.reg1 
    }

    /// Gets the second `Register` being compared
    ///
    /// # Returns
    ///
    /// The second `Register` being compared
    pub fn arg2(&self) -> Register {
        self.reg2 
    }
}

// end of file
