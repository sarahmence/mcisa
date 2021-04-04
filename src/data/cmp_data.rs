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
    arg_0: Register,

    /// The second `Register` to compare
    arg_1: Register 
}

// implementation
impl CMPData { 
    /// Creates a new `CMPData` instance
    ///
    /// # Arguments
    ///
    /// * `arg0` - The first `Register` to compare
    /// * `arg1` - The second `Register` to compare
    ///
    /// # Returns
    ///
    /// A new `CMPData` instance with the specified data
    pub fn new(arg0: Register, arg1: Register) -> Self {
        CMPData {
            arg_0: arg0,
            arg_1: arg1 
        }
    }

    /// Gets the first `Register` being compared
    ///
    /// # Returns
    ///
    /// The first `Register` being compared
    pub fn arg0(&self) -> Register {
        self.arg_0 
    }

    /// Gets the second `Register` being compared
    ///
    /// # Returns
    ///
    /// The second `Register` being compared
    pub fn arg1(&self) -> Register {
        self.arg_1 
    }
}

// end of file
