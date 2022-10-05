//! # Art
//! 
//! A library for modeling artistic concept.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The second colors according to RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to crate
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) ->SecondaryColor {
        SecondaryColor::Green
    }
}