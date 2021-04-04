/*
 * pop_data.rs
 * Defines the metadata structure for the POP instruction
 * Created by Sarah Davis
 * Created on 3/24/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use super::super::cpu::Register;

/// Contains metadata for the `POP` instruction
#[derive(Copy, Clone, Debug)]
pub struct POPData {
    /// The `Register` being popped into from the stack
    reg: Register 
}

// implementation
impl POPData { 
    /// Creates a new `POPData` instance
    ///
    /// # Arguments
    ///
    /// * `r` - The `Register` to pop into from the stack
    ///
    /// # Returns
    ///
    /// A new `POPData` instance with the specified data
    pub fn new(r: Register) -> Self {
        POPData {
            reg: r
        }
    }

    /// Gets the `Register` being poped
    ///
    /// # Returns
    ///
    /// The `Register` being popped into from the stack
    pub fn arg0(&self) -> Register {
        self.reg 
    }
}

// end of file
