/*
 * div_data.rs
 * Defines the metadata structure for the DIV instruction
 * Created by Sarah Davis
 * Created on 3/24/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use super::super::cpu::Register;

/// Metadata for the `DIV` instruction
#[derive(Copy, Clone, Debug)]
pub struct DIVData {
    /// The `Register` being divided
    arg_1: Register,

    /// The `Register` being divided by
    arg_2: Register,

    /// The `Register` to store the quotient in
    quo_dest: Register,
}

// implementation
impl DIVData {
    /// Creates a new `DIVData` instance
    /// 
    /// # Arguments
    /// 
    /// * `arg1` - The `Register` being divided
    /// * `arg2` - The `Register` being divided by
    /// * `dest` - The `Register` that the quotient is stored in
    ///
    /// # Returns
    ///
    /// A new `DIVData` instance with the specified data
    pub fn new(arg1: Register, arg2: Register, dest: Register) -> Self {
        DIVData {
            arg_1: arg1,
            arg_2: arg2,
            quo_dest: dest 
        }
    }

    /// Gets the `Register` being divided
    ///
    /// # Returns
    ///
    /// The first argument register
    pub fn arg1(&self) -> Register {
        self.arg_1 
    }

    /// Gets the `Register` being divided by
    ///
    /// # Returns 
    ///
    /// The second argument register
    pub fn arg2(&self) -> Register {
        self.arg_2 
    }

    /// Gets the destination `Register`
    ///
    /// # Returns
    ///
    /// The destination register for the quotient
    pub fn dest(&self) -> Register {
        self.quo_dest 
    }
}

// end of file
