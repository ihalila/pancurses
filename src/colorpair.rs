use std::ops::BitOr;
use super::{chtype, COLOR_PAIR};
use attributes::{Attribute, Attributes};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColorPair(pub u8);

impl From<ColorPair> for chtype {
    fn from(color_pair: ColorPair) -> chtype {
        COLOR_PAIR(chtype::from(color_pair.0))
    }
}

/// Implement the | operator for setting a color pair on an Attributes object
///
/// # Example
///
/// ```
/// use pancurses::{Attribute, Attributes};
/// use pancurses::colorpair::ColorPair;
///
/// let mut attributes = Attributes::new();
/// assert!(attributes.color_pair().0 == 0);
/// attributes = attributes | ColorPair(1);
/// assert!(attributes.color_pair().0 == 1);
/// ```
impl BitOr<ColorPair> for Attributes {
    type Output = Attributes;

    fn bitor(mut self, rhs: ColorPair) -> Attributes {
        self.set_color_pair(rhs);
        self
    }
}

/// Implement the | operator for combining a ColorPair and an Attribute to produce Attributes
///
/// # Example
///
/// ```
/// use pancurses::Attribute;
/// use pancurses::colorpair::ColorPair;
///
/// let attributes = ColorPair(5) | Attribute::Blink;
/// assert!(attributes.color_pair().0 == 5);
/// assert!(!attributes.is_bold());
/// assert!(attributes.is_blink());
/// ```
impl BitOr<Attribute> for ColorPair {
    type Output = Attributes;

    fn bitor(self, rhs: Attribute) -> Attributes {
        Attributes::new() | self | rhs
    }
}

/// Implement the | operator for combining an Attribute and a  ColorPair to produce Attributes
///
/// # Example
///
/// ```
/// use pancurses::Attribute;
/// use pancurses::colorpair::ColorPair;
///
/// let attributes = Attribute::Blink | ColorPair(2);
/// assert!(attributes.color_pair().0 == 2);
/// assert!(!attributes.is_bold());
/// assert!(attributes.is_blink());
/// ```
impl BitOr<ColorPair> for Attribute {
    type Output = Attributes;

    fn bitor(self, rhs: ColorPair) -> Attributes {
        Attributes::new() | self | rhs
    }
}
