/*
 * mul_data.rs
 * Defines the metadata structure for the MUL instruction
 * Created by Sarah Davis
 * Created on 3/12/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use super::super::cpu::Register;

/// Metadata for the `MUL` instruction
#[derive(Copy, Clone, Debug)]
pub struct MULData {
    /// The first `Register` being multiplied
    arg_1: Register,

    /// The second `Register` being multiplied
    arg_2: Register,

    /// The `Register` to store the product in
    prod_dest: Register,
}

// implementation
impl MULData {
    /// Creates a new `MULData` instance
    /// 
    /// # Arguments
    /// 
    /// * `arg1` - The first `Register` being multiplied
    /// * `arg2` - The second `Register` being multiplied
    /// * `dest` - The `Register` that the product is stored in
    ///
    /// # Returns
    ///
    /// A new `MULData` instance with the specified data
    pub fn new(arg1: Register, arg2: Register, dest: Register) -> Self {
        MULData {
            arg_1: arg1,
            arg_2: arg2,
            prod_dest: dest 
        }
    }

    /// Gets the first `Register` being multiplied
    ///
    /// # Returns
    ///
    /// The first argument register
    pub fn arg1(&self) -> Register {
        self.arg_1 
    }

    /// Gets the second `Register` being multiplied
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
    /// The destination register for the product
    pub fn dest(&self) -> Register {
        self.prod_dest 
    }
}

// end of file
