/*
 * jmp_data.rs
 * Defines the metadata structure for the JMP instruction
 * Created by Sarah Davis
 * Created on 4/3/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// no usage statements

/// Contains metadata for the `JMP` instruction
#[derive(Copy, Clone, Debug)]
pub struct JMPData {
    /// The ID of the label to jump unconditionally to
    label_id: u32
}

// implementation
impl JMPData { 
    /// Creates a new `JMPData` instance
    ///
    /// # Arguments
    ///
    /// * `lbl_id` - The ID of the label to jump unconditionally to
    ///
    /// # Returns
    ///
    /// A new `JMPData` instance with the specified data
    pub fn new(lbl_id: u32) -> Self {
        JMPData {
            label_id: lbl_id 
        }
    }

    /// Gets the label ID to jump unconditionally to
    ///
    /// # Returns
    ///
    /// The label ID to jump to
    pub fn jmp_label_id(&self) -> u32 {
        self.label_id 
    }
}

// end of file
