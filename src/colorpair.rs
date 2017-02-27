use std::ops::BitOr;
use super::chtype;
use attributes::{Attribute, Attributes};

pub struct ColorPair(pub chtype);

/// Implement the | operator for setting a color pair on an Attributes object
///
/// # Example
///
/// ```
/// use pancurses::{Attribute, Attributes, ColorPair};
///
/// let mut attributes = Attributes::new();
/// assert!(attributes.color_pair() == 0);
/// attributes = attributes | ColorPair(1);
/// assert!(attributes.color_pair() == 1);
/// ```
impl BitOr<ColorPair> for Attributes {
    type Output = Attributes;

    fn bitor(mut self, rhs: ColorPair) -> Attributes {
        self.set_color_pair(rhs.0);
        self
    }
}

/// Implement the | operator for combining a ColorPair and an Attribute to produce Attributes
///
/// # Example
///
/// ```
/// use pancurses::{Attribute, Attributes, ColorPair};
///
/// let attributes = ColorPair(2) | Attribute::Blink;
/// assert!(attributes.color_pair() == 2);
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
/// use pancurses::{Attribute, Attributes, ColorPair};
///
/// let attributes = Attribute::Blink | ColorPair(2);
/// assert!(attributes.color_pair() == 2);
/// assert!(!attributes.is_bold());
/// assert!(attributes.is_blink());
/// ```
impl BitOr<ColorPair> for Attribute {
    type Output = Attributes;

    fn bitor(self, rhs: ColorPair) -> Attributes {
        Attributes::new() | self | rhs
    }
}

impl From<ColorPair> for chtype {
    fn from(color_pair: ColorPair) -> chtype {
        color_pair.0
    }
}
