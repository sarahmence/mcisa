/*
 * add_data.rs
 * Defines the metadata structure for the ADD instruction
 * Created by Sarah Davis
 * Created on 3/12/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use super::super::cpu::Register;

/// Metadata for the `ADD` instruction
#[derive(Copy, Clone, Debug)]
pub struct ADDData {
    /// The first addend
    arg_1: Register,

    /// The second addend
    arg_2: Register,

    /// The `Register` to store the sum in
    sum_dest: Register,
}

// implementation
impl ADDData {
    /// Creates a new `ADDData` instance
    /// 
    /// # Arguments
    /// 
    /// * `arg1` - The first addend `Register`
    /// * `arg2` - The second addend `Register`
    /// * `dest` - The `Register` that the sum is stored in
    ///
    /// # Returns
    ///
    /// A new `ADDData` instance with the specified data
    pub fn new(arg1: Register, arg2: Register, dest: Register) -> Self {
        ADDData {
            arg_1: arg1,
            arg_2: arg2,
            sum_dest: dest 
        }
    }

    /// Gets the first addend `Register`
    ///
    /// # Returns
    ///
    /// The first addend register
    pub fn arg1(&self) -> Register {
        self.arg_1 
    }

    /// Gets the second addend `Register`
    ///
    /// # Returns 
    ///
    /// The second addend register
    pub fn arg2(&self) -> Register {
        self.arg_2 
    }

    /// Gets the destination `Register`
    ///
    /// # Returns
    ///
    /// The destination register for the sum
    pub fn dest(&self) -> Register {
        self.sum_dest 
    }
}

// end of file
