/*
 * push_data.rs
 * Defines the metadata structure for the PUSH instruction
 * Created by Sarah Davis
 * Created on 3/24/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use super::super::cpu::Register;

/// Contains metadata for the `PUSH` instruction
#[derive(Copy, Clone, Debug)]
pub struct PUSHData {
    /// The `Register` being pushed onto the stack
    reg: Register 
}

// implementation
impl PUSHData { 
    /// Creates a new `PUSHData` instance
    ///
    /// # Arguments
    ///
    /// * `r` - The `Register` to push onto the stack
    ///
    /// # Returns
    ///
    /// A new `PUSHData` instance with the specified data
    pub fn new(r: Register) -> Self {
        PUSHData {
            reg: r
        }
    }

    /// Gets the `Register` being pushed
    ///
    /// # Returns
    ///
    /// The `Register` being pushed onto the stack
    pub fn arg0(&self) -> Register {
        self.reg 
    }
}

// end of file
