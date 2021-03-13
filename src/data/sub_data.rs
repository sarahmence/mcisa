/*
 * sub_data.rs
 * Defines the metadata structure for the SUB instruction
 * Created by Sarah Davis
 * Created on 3/12/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use super::super::cpu::Register;

/// Metadata for the `SUB` instruction
#[derive(Copy, Clone, Debug)]
pub struct SUBData {
    /// The `Register` to subtract from
    arg_1: Register,

    /// The `Register` being subtracted
    arg_2: Register,

    /// The `Register` to store the difference in
    diff_dest: Register,
}

// implementation
impl SUBData {
    /// Creates a new `SUBData` instance
    /// 
    /// # Arguments
    /// 
    /// * `arg1` - The `Register` to subtract from
    /// * `arg2` - The `Register` being subtracted
    /// * `dest` - The `Register` that the difference is stored in
    ///
    /// # Returns
    ///
    /// A new `SUBData` instance with the specified data
    pub fn new(arg1: Register, arg2: Register, dest: Register) -> Self {
        SUBData {
            arg_1: arg1,
            arg_2: arg2,
            diff_dest: dest 
        }
    }

    /// Gets the `Register` being subtracted from
    ///
    /// # Returns
    ///
    /// The first argument register
    pub fn arg1(&self) -> Register {
        self.arg_1 
    }

    /// Gets the `Register` being subtracted
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
    /// The destination register for the difference
    pub fn dest(&self) -> Register {
        self.diff_dest 
    }
}

// end of file
