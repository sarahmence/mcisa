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
use std::convert::TryFrom;
use std::convert::From;
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
    Entity,
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
            Flag::Entity => "flag_Entity",
        };

        // and write it
        write!(f, "{}", flag_name)
    }
}

// TryFrom implementation
impl TryFrom<u8> for Flag {
    // create an error type
    type Error = ();

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x0 => Ok(Flag::Zero),
            0x1 => Ok(Flag::Negative),
            0x2 => Ok(Flag::Equal),
            0x3 => Ok(Flag::GreaterThan),
            0x4 => Ok(Flag::LessThan),
            0x5 => Ok(Flag::Block),
            0x6 => Ok(Flag::Entity),
            _ => Err(())
        }
    }
}

// From implementation for u8
impl From<Flag> for u8 {
    fn from(f: Flag) -> u8 {
        match f {
            Flag::Zero => 0x0,
            Flag::Negative => 0x1,
            Flag::Equal => 0x2,
            Flag::GreaterThan => 0x3,
            Flag::LessThan => 0x4,
            Flag::Block => 0x5,
            Flag::Entity => 0x6
        }
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

    // this test checks conversion from a u8
    #[test]
    fn test_convert_from_u8() {
        assert_eq!(Flag::try_from(0x0).unwrap(), Flag::Zero);
        assert_eq!(Flag::try_from(0x1).unwrap(), Flag::Negative);
        assert_eq!(Flag::try_from(0x2).unwrap(), Flag::Equal);
        assert_eq!(Flag::try_from(0x3).unwrap(), Flag::GreaterThan);
        assert_eq!(Flag::try_from(0x4).unwrap(), Flag::LessThan);
        assert_eq!(Flag::try_from(0x5).unwrap(), Flag::Block);
        assert_eq!(Flag::try_from(0x6).unwrap(), Flag::Entity);
        assert!(Flag::try_from(0x7).is_err());
    }

    // this test checks conversion to a u8
    #[test]
    fn test_convert_to_u8() {
        assert_eq!(u8::from(Flag::Zero), 0x0);
        assert_eq!(u8::from(Flag::Negative), 0x1);
        assert_eq!(u8::from(Flag::Equal), 0x2);
        assert_eq!(u8::from(Flag::GreaterThan), 0x3);
        assert_eq!(u8::from(Flag::LessThan), 0x4);
        assert_eq!(u8::from(Flag::Block), 0x5);
        assert_eq!(u8::from(Flag::Entity), 0x6)
    }
}

// end of file
