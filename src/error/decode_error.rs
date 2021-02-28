/*
 * decode_error.rs
 * Defines an error generated when opcode decoding fails
 * Created by Sarah Davis
 * Created on 2/27/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use super::super::opcode::Opcode;
use std::fmt;

/// An error resulting from opcode-to-instruction translation failure
#[derive(Clone, Debug)]
pub struct DecodeError {
    /// The `Opcode` that failed to decode
    opcode: Opcode,

    /// The reason for the failure
    err_reason: Option<String>
}

// implementation
impl DecodeError {
    /// Creates a new `DecodeError` instance
    ///
    /// # Arguments
    /// 
    /// * `op` - The `Opcode` that triggered the error
    ///
    /// # Returns
    ///
    /// A new `DecodeError` instance
    pub fn new(op: Opcode) -> Self {
        DecodeError {
            opcode: op, 
            err_reason: None
        }
    }

    /// Creates a new `DecodeError` instance with a reason
    ///
    /// # Arguments
    ///
    /// * `op` - The `Opcode` that triggered the error
    /// * `reason` - The reason that the error was triggered
    ///
    /// # Returns
    ///
    /// A new `DecodeError` instance
    pub fn with_reason(op: Opcode, reason: &str) -> Self {
        DecodeError {
            opcode: op,
            err_reason: Some(reason.to_owned())
        }
    }

    /// Gets the `Opcode` that triggered the error
    ///
    /// # Returns
    ///
    /// The opcode that triggered the error
    pub fn op(&self) -> Opcode {
        self.opcode 
    }

    /// Gets the reason for the error
    ///
    /// # Returns
    ///
    /// The reason for the error, wrapped in an `Option`
    pub fn reason(&self) -> Option<String> {
        self.err_reason.clone()
    }

    /// Determines whether the error has a known reason
    ///
    /// # Returns
    ///
    /// Whether there is a reason behind the error
    pub fn has_reason(&self) -> bool {
        self.err_reason.is_some()
    }
}

// Display implementation
impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.err_reason.clone() {
            Some(r) => {
                write!(f, "Couldn't decode opcode {}! Reason: {}", self.opcode, r)
            },
            None => {
                write!(f, "Couldn't decode opcode {}!", self.opcode)
            }
        }
    }
}

// start of unit tests
#[cfg(test)]
mod tests {
    // bring struct into scope
    use super::*;

    // tests the has_reason method
    #[test]
    fn test_has_reason() {
        let err1 = DecodeError::new(0x00000000.into());
        let err2 = DecodeError::with_reason(0x00000000.into(), "Example reason");
        assert_eq!(err1.has_reason(), false);
        assert!(err2.has_reason());
    }

    // tests formatting
    #[test]
    fn test_display_formatting() {
        let err1 = DecodeError::new(0x00000000.into());
        let err2 = DecodeError::with_reason(0x00000000.into(), "Example reason");
        assert_eq!(format!("{}", err1), "Couldn't decode opcode 0x00000000!");
        assert_eq!(format!("{}", err2), "Couldn't decode opcode 0x00000000! Reason: Example reason");
    }
}

// end of file
