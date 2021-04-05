/*
 * jns_data.rs
 * Defines the metadata structure for the JNS instruction
 * Created by Sarah Davis
 * Created on 4/4/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statement
use super::super::cpu::Flag;

/// Contains metadata for the `JNS` instruction
#[derive(Copy, Clone, Debug)]
pub struct JNSData {
    /// The condition `Flag` 
    cond_flg: Flag, 

    /// The ID of the label to jump to if `cond_flag` is unset
    label_id: u32
}

// implementation
impl JNSData { 
    /// Creates a new `JNSData` instance
    ///
    /// # Arguments:
    ///
    /// * `flag` - The condition `Flag`
    /// * `lbl_id` - The ID of the label to jump to if `flag` is unset
    ///
    /// # Returns
    ///
    /// A new `JNSData` instance with the specified data
    pub fn new(flag: Flag, lbl_id: u32) -> Self {
        JNSData {
            cond_flg: flag, 
            label_id: lbl_id 
        }
    }

    /// Gets the condition flag
    ///
    /// # Returns 
    ///
    /// The condition flag
    pub fn cond_flag(&self) -> Flag {
        self.cond_flg 
    }

    /// Gets the label ID to jump to if the condition flag is unset
    ///
    /// # Returns
    ///
    /// The label ID to jump to
    pub fn jmp_label_id(&self) -> u32 {
        self.label_id 
    }
}

// end of file
