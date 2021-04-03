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
#[derive(Clone, Debug)]
pub struct JMPData {
    /// The label to jump unconditionally to
    label: String
}

// implementation
impl JMPData { 
    /// Creates a new `JMPData` instance
    ///
    /// # Arguments
    ///
    /// * `lbl` - The label to jump unconditionally to
    ///
    /// # Returns
    ///
    /// A new `JMPData` instance with the specified data
    pub fn new(lbl: &str) -> Self {
        JMPData {
            label: String::from(lbl)
        }
    }

    /// Gets the label to jump unconditionally to
    ///
    /// # Returns
    ///
    /// A clone of the label to jump to
    pub fn jmp_label(&self) -> String {
        self.label.clone()
    }
}

// end of file
