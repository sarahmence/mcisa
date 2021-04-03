/*
 * chk_data.rs
 * Defines the metadata structure for the CHK instruction
 * Created by Sarah Davis
 * Created on 4/3/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statement
use super::super::cpu::Register;

/// Contains metadata for the `CHK` instruction
#[derive(Copy, Clone, Debug)]
pub struct CHKData {
    /// The `Register` to check
    reg: Register 
}

// implementation
impl CHKData { 
    /// Creates a new `CHKData` instance
    ///
    /// # Arguments
    ///
    /// * `r` - The `Register` to check
    ///
    /// # Returns
    ///
    /// A new `CHKData` instance with the specified data
    pub fn new(r: Register) -> Self {
        CHKData {
            reg: r
        }
    }

    /// Gets the `Register` to check 
    ///
    /// # Returns
    ///
    /// The `Register` to check
    pub fn chk_reg(&self) -> Register {
        self.reg 
    }
}

// end of file
