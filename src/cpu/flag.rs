/*
 * flag.rs
 * Enumerates Minecraft CPU flags
 * Created by Sarah Davis
 * Created on 1/6/2021
 *
 * Copyright (C) 2021 Sarah Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// usage statements
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A CPU flag modified by the `CMP`,
/// `BLK` `ENT`, and `CHK` instructions, 
/// and used by branching instructions
#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum Flag {
    /// Modified by `CHK`. Set if the 
    /// argument register is zero,
    /// unset otherwise
    Zero,

    /// Modified by `CHK`. Set if the
    /// argument register is negative,
    /// unset otherwise
    Negative, 

    /// Modified by `CMP`. Set if the 
    /// argument registers are equal,
    /// unset otherwise
    Equal,

    /// Modified by `CMP`. Set if the
    /// first argument register is
    /// greater than the second,
    /// unset otherwise
    GreaterThan,

    /// Modified by `CMP`. Set if the
    /// first argument register is
    /// less than the second,
    /// unset otherwise
    LessThan,

    /// Modified by `BLK`. Set if the
    /// block was found, unset otherwise
    Block,

    /// Modified by `ENT`. Set if the
    /// entity was found, unset otherwise
    Entity 
}

// Implementation
impl Flag {
    /// Gets the number of CPU flags
    ///
    /// # Returns 
    ///
    /// The number of CPU flags
    pub fn count() -> usize {
        Self::iter().count()
    }
}

// Display implementation
impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // get the name of the flag 
        let flag_name = match self {
            Flag::Zero => "flag_Zero",
            Flag::Negative => "flag_Negative",
            Flag::Equal => "flag_Equal",
            Flag::GreaterThan => "flag_GreaterThan",
            Flag::LessThan => "flag_LessThan",
            Flag::Block => "flag_Block",
            Flag::Entity => "flag_Entity"
        };

        // and write it
        write!(f, "{}", flag_name)
    }
}

// start of unit tests
#[cfg(test)]
mod tests {
    // load the enum
    use super::*;

    // this test checks the count method
    #[test]
    fn test_count() {
        // NOTE: Remember to update this if flags are added or removed
        assert_eq!(Flag::count(), 7);
    }
}

// end of file
